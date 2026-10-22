{
  description = "TZ-IO sub-kernel in Zig";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
      # The Zig API changes between versions: keep it pinned
      zigOf = pkgs: pkgs.zig_0_17;
    in
    {
      # Kernel executable: nix build, then result/bin/tzk
      packages = forAllSystems (pkgs: {
        default = pkgs.stdenv.mkDerivation {
          pname = "tzk-zig";
          version = "0.1.0";
          src = ./.;
          nativeBuildInputs = [ (zigOf pkgs).hook ];
        };
      });

      # Development tools: nix develop, or direnv with .envrc
      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          packages = [
            (zigOf pkgs)
            pkgs.zls
          ];
        };
      });
    };
}
