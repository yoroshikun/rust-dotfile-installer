//TODO install rust compiler and cargo (add path)
use std::process::Command;

// TODO proper return types
fn rust_install(type: String) -> () {
  Command::new("sh")
    .arg("curl")
    .arg("-sf")
    .arg("-L")
    .arg("https://static.rust-lang.org/rustup.sh")
    .arg("|")
    .arg("sh")
    .output()
    .expect("Download and execution of rust installer failed");

  return Ok(())
}