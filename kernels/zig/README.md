# TZ-IO kernel in Zig

Sub-kernel running a single TZ-IO node, written in Zig 0.17.
See [the protocol](../../tzio-core/multi/PROTOCOL.md) for what it must implement.

## Tools

All tools come from the Nix flake: Zig and its language server `zls`.
Enter the development shell with:

```sh
nix develop
```

or, with [direnv](https://direnv.net/), run `direnv allow` once to load it automatically.

The first run creates `flake.lock`, pinning the versions of the tools. Commit it.

Zig is pinned to 0.17 in `flake.nix`, as its API changes between versions.

## Build

```sh
zig build                       # produces zig-out/bin/tzk
zig build run                   # builds and runs the kernel
zig build test                  # runs the unit tests
zig build -Doptimize=ReleaseFast
```

Or build the kernel with Nix, without entering the shell:

```sh
nix build
./result/bin/tzk
```

## Test

From `tzio-core`, run the kernel test kit against the built kernel:

```sh
./gradlew :tzio-kernel-tck:test -Dtzio.kernel.command="$PWD/../kernels/zig/zig-out/bin/tzk"
```

To run TZ-IO programs with this kernel, reference it in the kernel configuration:

```
zig <= ../kernels/zig/zig-out/bin/tzk
```
