use std::{fmt::Display, fs, path::PathBuf};

pub struct CodePackageError {
  message: String
}

impl Display for CodePackageError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("Error: {}", self.message))
  }
}

pub struct CodePackage {
  pub dir_rel_path: String,
  pub file_name: String,
  pub code: String,
}

impl Display for CodePackage {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("{} - in dir: {}\nCode:\n{}", self.file_name, self.dir_rel_path, self.code))
  }
}

impl CodePackage {
  pub fn save(&self) -> Result<(), CodePackageError> {
    let pp = shellexpand::tilde(&self.dir_rel_path).into_owned();
    if let Err(err) = fs::create_dir_all(&pp) {
      return Err(CodePackageError { message: format!("Failed to create directory at {pp} - {err}") });
    }

    let dir_path = PathBuf::from(pp);
    let file_path = dir_path.join(&self.file_name);

    println!("file_path = {file_path:?}");

    match std::fs::write(&file_path, &self.code) {
      Ok(_) => Ok(()),
      Err(err) => Err(CodePackageError {
        message: format!(
          "Failed to create user keys file at {:?} due to error: {:?}",
          &file_path, err
        )
      }),
    }
  }
}
