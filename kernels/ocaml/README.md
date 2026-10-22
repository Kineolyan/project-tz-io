# TZ-IO kernel in OCaml

Sub-kernel running a single TZ-IO node, written in OCaml 5.
See [the protocol](../../tzio-core/multi/PROTOCOL.md) for what it must implement.

## Tools

All tools come from the Nix flake: OCaml, dune, `ocaml-lsp`, `ocamlformat` and `utop`.
Enter the development shell with:

```sh
nix develop
```

or, with [direnv](https://direnv.net/), run `direnv allow` once to load it automatically.

The first run creates `flake.lock`, pinning the versions of the tools. Commit it.

Libraries are added to the `depends` of `dune-project` and to the `libraries` of `bin/dune`.
With Nix, they must also be added to the shell and the package in `flake.nix`.

## Build

```sh
dune build            # produces _build/default/bin/main.exe
dune exec tzk         # builds and runs the kernel
dune fmt              # formats the sources
```

Or build the kernel with Nix, without entering the shell:

```sh
nix build
./result/bin/tzk
```

## Test

From `tzio-core`, run the kernel test kit against the built kernel:

```sh
./gradlew :tzio-kernel-tck:test -Dtzio.kernel.command="$PWD/../kernels/ocaml/_build/default/bin/main.exe"
```

To run TZ-IO programs with this kernel, reference it in the kernel configuration:

```
ocaml <= ../kernels/ocaml/_build/default/bin/main.exe
```
