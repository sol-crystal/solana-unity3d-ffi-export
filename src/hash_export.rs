use std::convert::TryFrom;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr::null_mut;
use std::slice;
use std::str::FromStr;

use borsh::BorshSerialize;
use libc::size_t;
use solana_program::hash::Hash;

use crate::c_sharp_string::str_from_c_char_ptr;
use crate::common_types::{ResultExport, to_raw_parts, VecRawParts};

pub const HASH_BYTES: usize = 32;
/// Maximum string length of a base58 encoded hash
const MAX_BASE58_LEN: usize = 44;

pub struct HashResult {
    pub hash: [u8; HASH_BYTES]
}

#[no_mangle]
extern "C"  fn hash_new(hash_slice:  *const u8, len: size_t) -> *mut Hash {
    let hash_slice = unsafe {
        assert!(!hash_slice.is_null());
        std::slice::from_raw_parts(hash_slice  , len as usize)
    };

    Box::into_raw(Box::new( Hash::new(hash_slice) ))

}
#[no_mangle]
extern "C" fn  hash_new_from_array(hash_array: *const u8, len: size_t) -> *mut Hash {
    let hash_slice = unsafe {
        assert!(!hash_array.is_null());
        std::slice::from_raw_parts(hash_array  , len as usize)
    };

    Box::into_raw(Box::new( Hash::new_from_array(<[u8; HASH_BYTES]>::try_from(hash_slice).unwrap())))
}

#[no_mangle]
extern "C" fn  hash_new_unique() -> *mut Hash {
    Box::into_raw(Box::new( Hash::new_unique() ))
}

#[no_mangle]
extern "C" fn  hash_from_string(s: *const c_char) -> ResultExport<*mut Hash> {

    match Hash::from_str(str_from_c_char_ptr(s).unwrap()) {
        Ok(hash) => ResultExport{
            is_error: 0 ,
            result: Box::into_raw(Box::new(hash)),
            error: null_mut()
        },
        Err(error) => ResultExport{
            is_error: 1,
            result: null_mut(),
            error: CString::new(error.to_string()).unwrap().into_raw()
        }
    }
}

#[no_mangle]
extern "C" fn  hash_to_bytes(ptr: *mut Hash) -> VecRawParts {
    let hash = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    to_raw_parts(hash.to_bytes().to_vec())
}

#[no_mangle]
extern "C" fn hash_to_string(ptr: *mut Hash) -> *mut c_char {
    let hash = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    CString::new(hash.to_string()).unwrap().into_raw()
}


#[no_mangle]
extern "C" fn free_hash(ptr: *mut Hash) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }

}