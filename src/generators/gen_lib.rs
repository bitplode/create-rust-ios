use std::path::PathBuf;

use crate::configuration::Config;
use super::code_package::CodePackage;

pub fn generate(config: &Config) -> CodePackage {
  let code = format!("use std::os::raw::{{c_char}};
use std::ffi::{{CString, CStr}};

/// # Safety
///
/// This function should not be called too early!
#[no_mangle]
pub unsafe extern fn rust_greet(to: *const c_char) -> *mut c_char {{
  let c_str = CStr::from_ptr(to);
  let recipient = match c_str.to_str() {{
    Err(_) => \"there\",
    Ok(string) => string,
  }};
  CString::new(\"Greetings \".to_owned() + recipient).unwrap().into_raw()
}}

/// # Safety
///
/// This function should not be called too early!
#[no_mangle]
pub unsafe extern fn rust_greet_free(s: *mut c_char) {{
  {{
    if s.is_null() {{ return }}
    CString::from_raw(s)
  }};
}}
");

  let pp = PathBuf::from(&config.output_dir)
    .join(&config.rust_dir)
    .join(PathBuf::from("src"));
  CodePackage { dir_rel_path: pp.to_str().unwrap().to_string(), file_name: "lib.rs".to_string(), code }
}
