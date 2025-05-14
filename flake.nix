{
  description = "Dev environment with aidoku-cli, rustup (nightly) and mitmproxy";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs { inherit system; };

      aidoku-cli = pkgs.buildGoModule {
        pname = "aidoku-cli";
        version = "0.5.9";

        src = pkgs.fetchFromGitHub {
          owner = "Aidoku";
          repo = "aidoku-cli";
          rev = "v0.5.9";
          fetchSubmodules = true;
          hash = "sha256-RJDvxUBbBoPtMchtZmSPZcBOcTgEUNaxQ0DFOzgdW/Y=";
        };

        vendorHash = "sha256-fEFBlbCOBP0axDRv2YfWrBS951EJeRcBe0soBIazeEg=";
        subPackages = [ "." ];
      };

    in {
      devShells.default = pkgs.mkShell {
        buildInputs = [
          aidoku-cli
          pkgs.rustup
          pkgs.mitmproxy
        ];

        shellInit = ''
          rustup install nightly
          rustup default nightly
        '';
      };
    }
  );
}
