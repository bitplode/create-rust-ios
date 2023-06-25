use std::{process::Command, path::PathBuf, str::FromStr};
use crate::generators::{gen_install_sh, gen_lib, gen_gitignore, gen_cargo_toml};

mod configuration;
mod errors;
mod generators;
mod injectors;

fn chmod_install_sh(install_sh_file_path: &str) {
  let output = if cfg!(target_os = "windows") {
    Command::new("cmd")
      .args(["/C", &format!("chmod u+x {install_sh_file_path}")])
      .output()
      .expect("failed to execute process")
  } else {
    Command::new("sh")
      .arg("-c")
      .arg(&format!("chmod u+x {install_sh_file_path}"))
      .output()
      .expect("failed to execute process")
  };

  let ret = output.stdout;
  let s = match std::str::from_utf8(&ret) {
    Ok(v) => v,
    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
  };
  println!("stdout: {s}");
}

fn main() {
  let config = configuration::get_config();

  let code_packages = vec![
    gen_lib::generate(&config),
    gen_gitignore::generate(&config),
    gen_cargo_toml::generate(&config),
    gen_install_sh::generate(&config),
  ];

  println!("Code packages:");
  for code_package in code_packages {
    println!("{code_package}");
    if let Err(err) = code_package.save() {
      println!("--> NOT SAVED - {err}");
    } else {
      println!("--> SAVED {}/{}", code_package.dir_rel_path, code_package.file_name);

      if code_package.file_name == "install.sh" {
        println!(">>> CHMODING");
        let pp = PathBuf::from_str(&code_package.dir_rel_path).unwrap().join(&code_package.file_name);
        chmod_install_sh(pp.to_str().unwrap());
      }
    }
  }

  if let Err(err) = injectors::inject_pbxproj::inject(&config) {
    println!("Error injecting into {}/project.pbxproj : {err}", config.ios_dir);
  }
}
