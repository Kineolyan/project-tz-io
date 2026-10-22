# TZ-IO kernels

A kernel runs a single node of a TZ-IO program, in its own process, for the
[multi-process runtime](../tzio-core/multi/PROTOCOL.md).

| Kernel              | Language | State           |
|---------------------|----------|-----------------|
| [python](python)    | Python 3 | Reference       |
| [cpp](cpp)          | C++26    | Skeleton        |
| [zig](zig)          | Zig 0.17 | Skeleton        |
| [ocaml](ocaml)      | OCaml 5  | Skeleton        |

Each kernel directory has its own README, describing how to build and run it.

## Tools

The compiled kernels provide their tools with a [Nix flake](https://nixos.wiki/wiki/Flakes):
`nix develop` in a kernel directory opens a shell with everything needed. With
[direnv](https://direnv.net/), `direnv allow` loads it automatically when entering the
directory.

Nix only sees the files tracked by git: `git add` new files before running `nix build`.

## Testing

The `tzio-kernel-tck` module checks that a kernel follows the protocol. From `tzio-core`:

```sh
./gradlew :tzio-kernel-tck:test -Dtzio.kernel.command="<command starting the kernel>"
```
