{ makeRustPlatform, rust-bin }:
let
  toolchain = rust-bin.stable.latest.default;
  rustPlatform = makeRustPlatform {
    cargo = toolchain;
    rustc = toolchain;
  };
in
rustPlatform.buildRustPackage {
  pname = "mcpack-builder";
  version = "0.0.1";

  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;
}
