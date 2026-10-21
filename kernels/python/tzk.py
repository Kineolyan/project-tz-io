#!/usr/bin/env python3
"""TZ-IO sub-kernel running a single node.

Reference implementation of the protocol described in tzio-core/multi/PROTOCOL.md.
It only depends on the Python standard library.
"""

import base64
import os
import select
import struct
import sys

BYTECODE_VERSION = 1

MOV, ADD, SUB, NEG, SAV, SWP = 0x01, 0x02, 0x03, 0x04, 0x05, 0x06
JMP, JEZ, JNZ, JLZ, JGZ, JRO = 0x11, 0x12, 0x13, 0x14, 0x15, 0x16
IMMEDIATE = 0x80

ACC = -1
NIL = -32768

JUMP_CONDITIONS = {
    JMP: lambda acc: True,
    JEZ: lambda acc: acc == 0,
    JNZ: lambda acc: acc != 0,
    JLZ: lambda acc: acc < 0,
    JGZ: lambda acc: acc > 0,
}

RUN, WAIT_IN, WAIT_OUT = "RUN", "WAIT_IN", "WAIT_OUT"


def to_int32(value):
    """Wraps a value as a 32-bit signed integer, like the Java runtime."""
    return (value + 0x80000000) % 0x100000000 - 0x80000000


class Stop(Exception):
    """Raised when the host stops the node."""


class ProtocolError(Exception):
    """Raised on an unexpected message from the host."""


class Immediate:
    """Immediate value used as a source operand."""

    __slots__ = ("value",)

    def __init__(self, value):
        self.value = value


class Program:
    """Decoded bytecode of a node."""

    def __init__(self, inputs, outputs, memory, ops):
        self.inputs = inputs
        self.outputs = outputs
        self.memory = memory
        self.ops = ops


def decode(data):
    """Decodes the bytecode into a Program, each operation being an (opcode, args) tuple."""
    if not data or data[0] != BYTECODE_VERSION:
        raise ProtocolError(f"unsupported bytecode version {data[0] if data else None}")
    inputs, outputs, memory, count = struct.unpack_from(">HHHH", data, 1)
    pos = 9

    def read(fmt):
        nonlocal pos
        (value,) = struct.unpack_from(fmt, data, pos)
        pos += struct.calcsize(fmt)
        return value

    def source(code):
        return Immediate(read(">i")) if code & IMMEDIATE else read(">h")

    ops = []
    for _ in range(count):
        code = read(">B")
        base = code & ~IMMEDIATE
        if base == MOV:
            src = source(code)
            ops.append((MOV, (src, read(">h"))))
        elif base in (ADD, SUB, JRO):
            ops.append((base, (source(code),)))
        elif base == NEG:
            ops.append((NEG, ()))
        elif base in (SAV, SWP) or base in JUMP_CONDITIONS:
            ops.append((base, (read(">H"),)))
        else:
            raise ProtocolError(f"unknown opcode {code:#x}")
    return Program(inputs, outputs, memory, ops)


class Host:
    """Line-based channel with the host, over stdin and stdout."""

    def __init__(self):
        self._buffer = b""
        self._lines = []

    def send(self, message):
        os.write(1, (message + "\n").encode("ascii"))

    def receive(self, block):
        """Returns the available lines, waiting for at least one if block is true."""
        while not self._lines:
            ready, _, _ = select.select([0], [], [], None if block else 0)
            if not ready:
                break
            chunk = os.read(0, 65536)
            if not chunk:
                raise Stop()
            self._buffer += chunk
            *lines, self._buffer = self._buffer.split(b"\n")
            self._lines.extend(line.decode("ascii") for line in lines)
        lines, self._lines = self._lines, []
        return lines

    def expect(self, keyword):
        """Waits for the next message, checking its keyword. Returns its argument."""
        lines = self.receive(block=True)
        # Keep the following messages for the next reception
        self._lines = lines[1:] + self._lines
        name, _, argument = lines[0].partition(" ")
        if name == "STOP":
            raise Stop()
        if name != keyword:
            raise ProtocolError(f"expected {keyword}, got {lines[0]!r}")
        return argument


class Node:
    """State of the executed node."""

    def __init__(self, program):
        self.program = program
        self.pc = 0
        self.acc = 0
        self.memory = [0] * program.memory
        self.inputs = [None] * program.inputs
        self.busy_outputs = [False] * program.outputs
        self.tick = 0

    def on_message(self, line):
        name, _, argument = line.partition(" ")
        if name == "STOP":
            raise Stop()
        if name != "SYNC":
            raise ProtocolError(f"unexpected message {line!r}")
        values, frees = argument.split(" ")
        if values != "-":
            for entry in values.split(","):
                index, value = entry.split(":")
                index = int(index)
                if self.inputs[index] is not None:
                    raise ProtocolError(f"input {index} already has a value")
                self.inputs[index] = int(value)
        if frees != "-":
            for index in frees.split(","):
                self.busy_outputs[int(index)] = False

    def can_read(self, src):
        return isinstance(src, Immediate) or src < 0 or self.inputs[src] is not None

    def can_write(self, dst):
        return dst < 0 or not self.busy_outputs[dst]

    def read(self, src):
        """Reads a source operand. Returns (value, consumed input or None)."""
        if isinstance(src, Immediate):
            return src.value, None
        if src == ACC:
            return self.acc, None
        if src == NIL:
            return 0, None
        value, self.inputs[src] = self.inputs[src], None
        return value, src

    def write(self, dst, value):
        """Writes into a target operand. Returns the produced output or None."""
        if dst == ACC:
            self.acc = value
        elif dst >= 0:
            self.busy_outputs[dst] = True
            return dst, value
        return None

    def step(self):
        """Executes one tick. Returns (status, consumed input, produced output)."""
        self.tick += 1
        code, args = self.program.ops[self.pc]
        consumed = produced = None
        next_pc = self.pc + 1

        if code == MOV:
            src, dst = args
            if not self.can_read(src):
                return WAIT_IN, None, None
            if not self.can_write(dst):
                return WAIT_OUT, None, None
            value, consumed = self.read(src)
            produced = self.write(dst, value)
        elif code in (ADD, SUB):
            if not self.can_read(args[0]):
                return WAIT_IN, None, None
            value, consumed = self.read(args[0])
            self.acc = to_int32(self.acc + (value if code == ADD else -value))
        elif code == NEG:
            self.acc = to_int32(-self.acc)
        elif code == SAV:
            self.memory[args[0]] = self.acc
        elif code == SWP:
            index = args[0]
            self.acc, self.memory[index] = self.memory[index], self.acc
        elif code == JRO:
            if not self.can_read(args[0]):
                return WAIT_IN, None, None
            offset, consumed = self.read(args[0])
            next_pc = self.pc + offset
        elif JUMP_CONDITIONS[code](self.acc):
            next_pc = args[0]

        self.pc = next_pc % len(self.program.ops)
        return RUN, consumed, produced

    def trace(self, status):
        memory = "".join(f" {value}" for value in self.memory)
        sys.stderr.write(f"@S {self.tick} {self.pc} {self.acc} {status}{memory}\n")
        sys.stderr.flush()


def run():
    host = Host()
    node = Node(decode(base64.b64decode(host.expect("LOAD"))))
    host.send("READY")
    host.expect("START")

    while not node.program.ops:
        # Nothing to execute
        for line in host.receive(block=True):
            node.on_message(line)

    while True:
        for line in host.receive(block=False):
            node.on_message(line)
        status, consumed, produced = node.step()
        node.trace(status)
        if status == RUN:
            if consumed is not None or produced is not None:
                consumed_part = "-" if consumed is None else str(consumed)
                produced_part = "-" if produced is None else f"{produced[0]}:{produced[1]}"
                host.send(f"IO {consumed_part} {produced_part}")
        else:
            # Blocked: wait for the host to provide the missing resource
            for line in host.receive(block=True):
                node.on_message(line)


def main():
    try:
        run()
    except Stop:
        return 0
    except (ProtocolError, ValueError, IndexError, KeyError) as error:
        sys.stderr.write(f"tzk: {error}\n")
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
