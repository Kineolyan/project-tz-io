# TZ-IO multi-process runtime

The `tzio-multi` core runs every node of a TZ-IO program in its own process, called a
*sub-kernel*. The host (this Java module) owns the slots connecting the nodes, and talks
to each sub-kernel through its standard streams:

- `stdin`: host → node control messages
- `stdout`: node → host control messages
- `stderr`: node debug state, only recorded when tracing is enabled

All control messages are plain ASCII text, one message per line (`\n`), words separated
by a single space. A sub-kernel must flush `stdout` after each message.

## 1. Kernel configuration

The host reads the kernel configuration from the file given by the system property
`tzio.multi.config`, or else by the environment variable `TZIO_MULTI_CONFIG`.

```
# Comments start with '#'. Blank lines are ignored.
# Node to kernel:    <node name> => <kernel id>
double => py
main   => py
# Kernel command:    <kernel id> <= <command>
py <= python3 kernels/python/tzk.py
```

- Commands are run through `sh -c`, from the directory holding the configuration file.
- Every node of the program must be mapped to a declared kernel, or the program fails
  at start.

## 2. Bytecode (version 1)

The node program is sent as bytes, encoded in base64. All integers are big-endian.

### Header

| Field       | Type | Description                       |
|-------------|------|-----------------------------------|
| `version`   | u8   | Always `1` for this version       |
| `inputs`    | u16  | Number of node inputs             |
| `outputs`   | u16  | Number of node outputs            |
| `memory`    | u16  | Size of the node memory           |
| `ops`       | u16  | Number of operations that follow  |

### Operands

A reference operand is an `i16`:

| Value             | Meaning                                                   |
|-------------------|-----------------------------------------------------------|
| `0` .. `1023`     | Slot: input index as a source, output index as a target   |
| `-1`              | `ACC`                                                     |
| `-2` .. `-128`    | Reserved for other registers                              |
| `-32768`          | `NIL`: reads 0, discards writes                           |

When the opcode carries the flag `0x80` (only `MOV`, `ADD`, `SUB`, `JRO`), the source
operand is replaced by an immediate `i32` value.

### Operations

| Opcode | Name | Arguments            | Semantics                                         |
|--------|------|----------------------|---------------------------------------------------|
| `0x01` | MOV  | `src`, `dst`         | `dst <- src`                                      |
| `0x02` | ADD  | `src`                | `ACC <- ACC + src`                                |
| `0x03` | SUB  | `src`                | `ACC <- ACC - src`                                |
| `0x04` | NEG  |                      | `ACC <- -ACC`                                     |
| `0x05` | SAV  | `u16` memory index   | `mem[i] <- ACC`                                   |
| `0x06` | SWP  | `u16` memory index   | swaps `ACC` and `mem[i]`                          |
| `0x11` | JMP  | `u16` target         | `pc <- target`                                    |
| `0x12` | JEZ  | `u16` target         | `pc <- target` if `ACC == 0`                      |
| `0x13` | JNZ  | `u16` target         | `pc <- target` if `ACC != 0`                      |
| `0x14` | JLZ  | `u16` target         | `pc <- target` if `ACC < 0`                       |
| `0x15` | JGZ  | `u16` target         | `pc <- target` if `ACC > 0`                       |
| `0x16` | JRO  | `src`                | `pc <- (pc + src) mod ops` (always positive)      |

- Memory indexes are 0-based.
- Jump targets are absolute operation indexes. Labels do not exist in the bytecode: the
  host resolves them. A label at the end of the program targets operation `0`.
- After the last operation, execution continues with operation `0`.
- `JRO` with an offset of `0` stays on the same operation.

### Blocking

An operation reading a slot without a value, or writing to an occupied output, does not
execute: the node stays on that operation until the host provides what it needs. A `MOV`
from a slot to a slot requires both conditions before consuming anything.

## 3. Control protocol

### Host → node (`stdin`)

| Message                    | Description                                                |
|----------------------------|------------------------------------------------------------|
| `LOAD <base64>`            | Program of the node. The end of the line ends the bytes    |
| `START`                    | Starts the execution                                       |
| `SYNC <values> <frees>`    | Resources made available since the last `SYNC`             |
| `STOP`                     | Stops the node. The node must exit                         |

In `SYNC`:

- `<values>` lists new input values as `<input>:<value>` separated by `,`, or `-`.
- `<frees>` lists outputs that were consumed and can be written again, separated by `,`,
  or `-`.

Examples: `SYNC 0:42,3:-5 -`, `SYNC - 1`, `SYNC 1:7 0,2`.

The host sends at most one `SYNC` per node and per host tick, and only when there is
something new. An input receives a new value only after the node consumed the previous
one. At start, every output of a node is free and no input has a value.

### Node → host (`stdout`)

| Message                      | Description                                           |
|------------------------------|-------------------------------------------------------|
| `READY`                      | Answer to `LOAD`, once the program is decoded         |
| `IO <consumed> <produced>`   | I/O performed by one node tick                        |

In `IO`:

- `<consumed>` is the input index whose value was consumed, or `-`.
- `<produced>` is `<output>:<value>` for a value written to an output, or `-`.

A tick reads at most one input and writes at most one output. `IO` is only sent for
ticks doing I/O. Examples: `IO 0 1:42` for `MOV <0, >1`, `IO 0 -` for `ADD <0`,
`IO - 1:7` for `MOV ACC, >1`.

A written output stays occupied until the host sends it back in the `<frees>` of a
`SYNC`.

### Node → host (`stderr`)

Each node tick writes a state line:

```
@S <tick> <pc> <acc> <status> <mem0> <mem1> ...
```

`<status>` is `RUN` after an executed operation, or `WAIT_IN`/`WAIT_OUT` when the
operation is blocked on an input or an output. A blocked node writes its state once,
then waits for a message. Any line not starting with `@S ` is a free log line.

The host records `stderr` only when the system property `tzio.multi.trace` is `true`.
Otherwise, `stderr` is discarded.

## 4. Host execution

1. Start every sub-kernel, send `LOAD`, wait for all `READY`, then send `START`.
2. Each host tick:
   1. read the pending `IO` messages of every node, skipping the nodes without any;
   2. update the slots: a consumed value frees the slot for its writer, a produced value
      fills the slot for its reader;
   3. feed the external inputs and collect the external outputs;
   4. send one `SYNC` to every node with new values or freed outputs.
3. If any sub-kernel dies, all others are killed and the program fails.

Each slot must have at most one writer and one reader.

`runOn(inputs, cycles)` counts host ticks. A host tick without any message from the
nodes waits up to `tzio.multi.tickWaitMs` milliseconds (default `20`) for one.

## 5. Testing a kernel

The `tzio-kernel-tck` module checks that a kernel follows this protocol. It runs a single
node per test, sending messages on its `stdin` and expecting messages on its `stdout`.
It covers the control protocol, the I/O, every instruction and the `stderr` trace.

```
./gradlew :tzio-kernel-tck:test -Dtzio.kernel.command="<command starting the kernel>"
```

Without the property, the reference Python kernel is tested.
