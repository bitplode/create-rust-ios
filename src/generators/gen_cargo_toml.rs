use std::path::PathBuf;

use crate::configuration::Config;
use super::code_package::CodePackage;

pub fn generate(config: &Config) -> CodePackage {
  let rust_file_name = &config.rust_file_name;
  let code = format!("[package]
name = \"{rust_file_name}\"
version = \"0.1.0\"
authors = [\"You da author <you@da-author.app>\"]
edition = \"2018\"

[dependencies]

[lib]
name = \"{rust_file_name}\"
crate-type = [\"staticlib\", \"cdylib\"]
");

  let pp = PathBuf::from(&config.output_dir)
    .join(&config.rust_dir);
  CodePackage { dir_rel_path: pp.to_str().unwrap().to_string(), file_name: "Cargo.toml".to_string(), code }
}
