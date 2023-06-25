use std::os::raw::{c_char};
use std::ffi::{CString, CStr};

/// # Safety
///
/// This function should not be called too early!
#[no_mangle]
pub unsafe extern fn rust_greet(to: *const c_char) -> *mut c_char {
  let c_str = CStr::from_ptr(to);
  let recipient = match c_str.to_str() {
    Err(_) => "there",
    Ok(string) => string,
  };
  CString::new("Greetings ".to_owned() + recipient).unwrap().into_raw()
}

/// # Safety
///
/// This function should not be called too early!
#[no_mangle]
pub unsafe extern fn rust_greet_free(s: *mut c_char) {
  {
    if s.is_null() { return }
    CString::from_raw(s)
  };
}
