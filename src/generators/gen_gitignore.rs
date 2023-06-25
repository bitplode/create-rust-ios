use std::path::PathBuf;

use crate::configuration::Config;
use super::code_package::CodePackage;

pub fn generate(config: &Config) -> CodePackage {
  let code = format!("/target
**/*.rs.bk
Cargo.lock
");

  let pp = PathBuf::from(&config.output_dir)
    .join(&config.rust_dir);
  CodePackage { dir_rel_path: pp.to_str().unwrap().to_string(), file_name: ".gitignore".to_string(), code }
}
