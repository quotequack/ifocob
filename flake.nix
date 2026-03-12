{
    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    };

    outputs = { self, nixpkgs }: let
    pkgs = nixpkgs.legacyPackages."x86_64-linux";
    in {
        packages."x86_64-linux".default = pkgs.rustPlatform.buildRustPackage {
            name = "ifocob";
            src = ./.;
            buildInputs = [];
            nativeBuildInputs = [];
            cargoHash = "sha256-sxJSv4tfBdRM1aZB00Tq8yOPwjV8Xm3WZ9HUP46H3GA=";
            postInstall = ''
              mkdir -p $out/share/applications/
              install -Dm644 ifconvert.desktop $out/share/applications/ifconvert.desktop
              install -Dm644 ifocob.desktop $out/share/applications/ifocob.desktop
            '';
        };
        devShells."x86_64-linux".default = pkgs.mkShell {
            buildInputs = with pkgs; [
                cargo
                rustc 
                just
            ];
            nativeBuildInputs = [ pkgs.pkg-config ];
            env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
    };
}