use std::process::Command;

pub fn install_package_manager(verbose) -> () {
  let output = if cfg!(target_os = "windows") {
    // TODO Windows needs chocolaty
    Command::new("cmd")
      .args(&["/C", "echo hello"])
      .output()
      .expect("failed to execute process")
  } else if cfg!(target_os = "macos") {
    // If Mac install brew and brew cask (first check if installed)
    let brew_check = Command::new("sh")
      .arg("-c")
      .arg("which brew")
      .status()
      .expect("failed to execute process");
    if !brew_check.success() {
      // TODO Convert this to native download homebrew, run ruby native
      Command::new("sh")
        .arg("-c")
        .arg("ruby -e")
        .arg(r#""$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)""#) //Raw string literal
        .output()
        .expect("failed to execute process")
    } else {
      Command::new("sh")
        .arg("-c")
        .arg("echo brew is already installed!!")
        .output()
        .expect("failed to execute process")
    }
  } else {
    // TODO If linux use apt-get
    Command::new("sh")
      .arg("-c")
      .arg("echo hello")
      .output()
      .expect("failed to execute process")
  };
  let hello = String::from_utf8_lossy(&output.stdout);

  println!("output: {}", hello);
  ()
}