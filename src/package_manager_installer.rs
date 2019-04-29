use std::process::{Command, Stdio};

pub fn install_package_manager(verbose: bool, _yes: bool) -> () {
  let installed = check_package_manager(verbose);

  if installed {
    return ();
  }

  let package_manager_install_output = if cfg!(target_os = "windows") {
    // TODO Windows needs chocolaty
    Command::new("cmd")
      .args(&["/C", "echo hello"])
      .output()
      .expect("failed to execute process")
  } else if cfg!(target_os = "macos") {
    // TODO Convert this to native download homebrew, run ruby native
    Command::new("sh")
      .arg("-c")
      .arg("ruby -e")
      .arg(r#""$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)""#) //Raw string literal
      .output()
      .expect("failed to execute process")
  } else {
    // TODO If linux use apt-get
    Command::new("sh")
      .arg("-c")
      .arg("echo hello")
      .output()
      .expect("failed to execute process")
  };
  let package_manager_install_output =
    String::from_utf8_lossy(&package_manager_install_output.stdout);

  if verbose {
    println!("{}", package_manager_install_output);
  } else {
    println!("Package installer has been installed!")
  }
  return ();
}

fn check_package_manager(verbose: bool) -> bool {
  let check = if cfg!(target_os = "windows") {
    // TODO
    Command::new("sh")
      .arg("-c")
      .arg("brew")
      .stdout(Stdio::null())
      .stderr(Stdio::null())
      .spawn()
      .expect("Package manager check failed")
  } else if cfg!(target_os = "macos") {
    Command::new("sh")
      .arg("-c")
      .arg("brew")
      .stdout(Stdio::null())
      .stderr(Stdio::null())
      .spawn()
      .expect("Package manager check failed")
  } else {
    // TODO
    Command::new("sh")
      .arg("-c")
      .arg("brew")
      .stdout(Stdio::null())
      .stderr(Stdio::null())
      .spawn()
      .expect("Package manager check failed")
  };

  match check
    .wait_with_output()
    .expect("Failed to wait for check")
    .status
    .code()
  {
    Some(code) => {
      if code == 1 {
        if verbose {
          println!("Exit Code: {}", code);
          println!("Package manager is already present skipping");
        }
        return true;
      } else {
        if verbose {
          println!("Exit Code: {}", code);
        }
        return false;
      }
    }
    None => {
      if verbose {
        println!("Process terminated by signal");
      }
      panic!("could not run sh brew command");
    }
  }
}
