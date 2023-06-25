use serde::Deserialize;
use std::{path::PathBuf, str::FromStr};

#[derive(Deserialize, Debug)]
pub struct Config {
  #[serde(rename(deserialize = "project_name", serialize = "project_name"))]
  pub project_name: String,
  #[serde(rename(deserialize = "output_dir", serialize = "output_dir"))]
  pub output_dir: String,
  #[serde(rename(deserialize = "rust_file_name", serialize = "rust_file_name"))]
  pub rust_file_name: String,
  #[serde(rename(deserialize = "rust_dir", serialize = "rust_dir"))]
  pub rust_dir: String,
  #[serde(rename(deserialize = "ios_dir", serialize = "ios_dir"))]
  pub ios_dir: String,
}

impl Config {
  pub fn get_ios_lib_executable_name(&self) -> String {
    let rust_file_name = &self.rust_file_name;
    format!("lib{rust_file_name}.a")
  }

  pub fn get_bridge_h_file_name(&self) -> String {
    let rust_file_name = &self.rust_file_name;
    format!("{rust_file_name}.h")
  }

  pub fn get_ios_libs_directory_path(&self) -> String {
    let ios_dir = shellexpand::tilde(&self.ios_dir).into_owned();
    let libs_path_obj = PathBuf::from_str(&ios_dir).unwrap().join("libs");
    let libs_path = libs_path_obj.to_str().unwrap();
    libs_path.to_string()
  }

  pub fn get_ios_dir_name(&self) -> String {
    let ios_dir = PathBuf::from_str(&self.ios_dir).unwrap();
    let ios_dir_name = ios_dir.file_name().unwrap().to_str().unwrap();
    ios_dir_name.to_string()
  }
}

fn get_config_file(p: &PathBuf) -> Option<String> {
  if p.exists() {
    if let Ok(content) = std::fs::read_to_string(p) {
      return Some(content);
    }
  }

  if let Some(file_name) = p.file_name() {
    if let Some(dir) = p.parent() {
      if let Some(parent_dir) = dir.parent() {
        return get_config_file(&parent_dir.join(file_name));
      }
    }
  }

  None
}

pub fn get_config() -> Config {
  if let Ok(p) = std::env::current_dir() {
    if let Some(x) = get_config_file(&p.join("create-rust-ios.config.json")) {
      return serde_json::from_str(&x).unwrap();
    }
  }

  panic!("Cannot load the create-rust-ios.config.json file");
}
