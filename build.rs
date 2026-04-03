fn main() {
  if cfg!(feature = "nautilus") {
    println!("cargo:rustc-link-lib=nautilus-extension");
  } else if cfg!(feature = "thunar") {
    println!("cargo:rustc-link-lib=thunarx-3");
  } else {
    panic!("Escolha uma feature: --features nautilus  ou  --features thunar");
  }
}
