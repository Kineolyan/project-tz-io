# TZ-IO kernel in C++

Sub-kernel running a single TZ-IO node, written in C++26.
See [the protocol](../../tzio-core/multi/PROTOCOL.md) for what it must implement.

## Tools

All tools come from the Nix flake: GCC, CMake, Ninja and `clangd`/`clang-format`.
Enter the development shell with:

```sh
nix develop
```

or, with [direnv](https://direnv.net/), run `direnv allow` once to load it automatically.

The first run creates `flake.lock`, pinning the versions of the tools. Commit it.

## Build

```sh
cmake -S . -B build -G Ninja
cmake --build build
./build/tzk
```

Or build the kernel with Nix, without entering the shell:

```sh
nix build
./result/bin/tzk
```

## Test

From `tzio-core`, run the kernel test kit against the built kernel:

```sh
./gradlew :tzio-kernel-tck:test -Dtzio.kernel.command="$PWD/../kernels/cpp/build/tzk"
```

To run TZ-IO programs with this kernel, reference it in the kernel configuration:

```
cpp <= ../kernels/cpp/build/tzk
```
