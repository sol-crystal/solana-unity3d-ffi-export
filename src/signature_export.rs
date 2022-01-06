use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr::null_mut;
use std::str::FromStr;
use libc::size_t;
use solana_sdk::signature::{Keypair, Signature, Signer};

use crate::c_sharp_string::str_from_c_char_ptr;
use crate::common_types::ResultExport;

/// Number of bytes in a signature
pub const SIGNATURE_BYTES: usize = 64;
pub const PUBLIC_KEY_LENGTH: usize = 32;

#[no_mangle]
extern "C" fn from_slice(slice: *const u8, len: size_t) -> *mut Signature{
    let slice = unsafe {
        assert!(!slice.is_null());

        std::slice::from_raw_parts(slice  , len as usize)
    };
    Box::into_raw(Box::new(Signature::new(slice)))
}


#[no_mangle]
extern "C" fn new_unique() -> *mut Signature{
    Box::into_raw(Box::new(Signature::new_unique()))
}


#[no_mangle]
extern "C" fn verify(ptr: *mut Signature, pubkey_bytes: *const u8, pubkey_bytes_len: size_t,
                     message_bytes: *const u8, message_bytes_len: size_t) -> u8 {
    let signature = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let pubkey_bytes = unsafe {
        assert!(!pubkey_bytes.is_null());
        std::slice::from_raw_parts(pubkey_bytes  , pubkey_bytes_len as usize)
    };

    let message_bytes = unsafe {
        assert!(!message_bytes.is_null());
        std::slice::from_raw_parts(message_bytes  , message_bytes_len as usize)
    };
    signature.verify(pubkey_bytes,message_bytes) as u8

}


#[no_mangle]
extern "C" fn signature_from_str(s: *const c_char) -> ResultExport<*mut Signature> {


    match Signature::from_str(str_from_c_char_ptr(s).unwrap()) {
        Ok(signature) => ResultExport {
            is_error: 0 ,
            result: Box::into_raw(Box::new(signature)),
            error: null_mut()
        },
        Err(error) => ResultExport {
            is_error: 1,
            result: null_mut(),
            error: CString::new(error.to_string()).unwrap().into_raw()
        }
    }
}


#[no_mangle]
extern "C" fn signature_to_str(ptr: *mut Signature) -> *mut c_char{
    let signature = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    CString::new(signature.to_string()).unwrap().into_raw()
}

#[no_mangle]
extern "C" fn free_signature(signature: *mut Signature){
    if signature.is_null() {
        return;
    }
    unsafe { Box::from_raw(signature); }
}