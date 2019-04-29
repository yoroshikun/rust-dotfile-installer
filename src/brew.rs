use std::fs::File;
use std::process::{Command, Stdio};

// TODO proper return types
pub fn brew_install(verbose: bool, formula_array: Vec<String>) -> bool {
  // TODO runs brew installation of package

  for formula in formula_array.iter() {
    let brew_install_output = if verbose {
      Command::new("brew")
        .arg("install")
        .arg(&formula)
        .output()
        .expect("Installing formula failed")
    } else {
      Command::new("brew")
        .arg("install")
        .arg(&formula)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .expect("Installing formula failed")
    };

    if verbose {
      println!("formula: {}", formula);
      // TODO Mimic usual output
      println!(
        "brew_formula_install: Standard Out: {}",
        String::from_utf8_lossy(&brew_install_output.stdout)
      );
      println!(
        "brew_formula_install: Standard Err: {}",
        String::from_utf8_lossy(&brew_install_output.stderr)
      )
    } else {
      if brew_install_output.status.success() {
        println!("installed {} sucessfully", formula)
      } else {
        println!("failed to install {}", formula);
        println!("run the installer with -v to debug {}", formula)
      }
    }
  }
  println!("Installed all brew formulas");
  return true;
}
// TODO proper return types
fn brew_tap() -> () {
  // TODO runs brew tap
}
// TODO proper return types
fn brew_cask_install() -> () {
  // TODO runs brew cask installations
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
  title: Option<String>,
  pub brew_formulas: Vec<String>,
  pub brew_cask_packages: Vec<String>,
  asdf_config: Option<Asdf>,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Asdf {
  plugins: Option<Vec<String>>,
  node_versions: Option<Vec<String>>,
}

pub fn config_readfile(verbose: bool) -> Result<Config, Box<std::error::Error>> {
  // TODO Reads the brew package list and parses it
  let f = File::open("./config/config.yaml").expect("Could not read file");
  let s: Config = serde_yaml::from_reader(f)?;
  if verbose {
    println!("Loaded config: {:?}", s);
  }
  Ok(s)
}

fn brew_cask_readfile() -> () {
  // TODO Reads the brew package list and parses it
}
