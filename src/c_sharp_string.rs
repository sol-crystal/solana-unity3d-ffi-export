use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::str::FromStr;


pub fn str_from_c_char_ptr<'a>(s: *const c_char) -> Result<&'a str, std::str::Utf8Error> {
    unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s)
    }
        .to_str()
}

pub fn string_from_c_char_ptr(s: *const c_char) -> Result<String, std::str::Utf8Error> {
    Ok(str_from_c_char_ptr(s)?.to_string())
}

pub fn string_from_csharp(s: *const c_char) -> String {
    let c_str = unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s)
    };
    let r_str = c_str.to_str().unwrap();
    r_str.into()
}

#[no_mangle]
extern "C" fn ffi_free_cstring(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe { CString::from_raw(ptr) };
}

#[no_mangle]
extern "C" fn free_cstring(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe { CString::from_raw(ptr) };
}
