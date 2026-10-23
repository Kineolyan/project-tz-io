{
  description = "TZ-IO sub-kernel in C++";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
    in
    {
      # Kernel executable: nix build, then result/bin/tzk
      packages = forAllSystems (pkgs: {
        default = pkgs.stdenv.mkDerivation {
          pname = "tzk-cpp";
          version = "0.1.0";
          src = ./.;
          nativeBuildInputs = [ pkgs.cmake pkgs.ninja ];
        };
      });

      # Development tools: nix develop, or direnv with .envrc
      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          packages = [
            pkgs.cmake
            pkgs.ninja
            pkgs.clang-tools # clangd and clang-format
          ];
        };
      });
    };
}
