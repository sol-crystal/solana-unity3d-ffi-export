#[allow(unused_variables)]
use std::convert::TryInto;
use std::ffi::{CString};
use std::os::raw::c_char;
use std::ptr::null_mut;
use std::str::FromStr;
use libc::size_t;
use solana_program::pubkey::{Pubkey};
use crate::c_sharp_string::str_from_c_char_ptr;
use crate::common_types::{ResultExport, to_raw_parts, VecRawParts};


/// Number of bytes in a pubkey
pub const PUBKEY_BYTES: usize = 32;
/// maximum length of derived `Pubkey` seed
pub const MAX_SEED_LEN: usize = 32;
/// Maximum number of seeds
pub const MAX_SEEDS: usize = 16;
/// Maximum string length of a base58 encoded pubkey
const MAX_BASE58_LEN: usize = 44;

#[repr(C)]
pub struct ProgramAddress {
    pub pubkey: *mut Pubkey,
    pub bump_seed: u8
}

#[repr(C)]
pub struct Seeds<T> {
    seed: *const T,
    length: size_t
}

#[no_mangle]
extern "C" fn pubkey_new_from_array(pubkey_array: *const u8, len: size_t) -> *mut Pubkey{
    let pubkey_array = unsafe {
        assert!(!pubkey_array.is_null());
        std::slice::from_raw_parts(pubkey_array  , len as usize)
    };

    let pubkey = Pubkey::new_from_array(pubkey_array.try_into().expect("slice with incorrect length"));
    Box::into_raw(Box::new(pubkey))
}

#[no_mangle]
extern "C" fn pubkey_new_unique()-> *mut Pubkey {
    let pubkey = Pubkey::new_unique();
    Box::into_raw(Box::new(pubkey))
}

#[no_mangle]
extern "C" fn pubkey_from_str(s: *const c_char) -> ResultExport<*mut Pubkey> {

    match Pubkey::from_str(str_from_c_char_ptr(s).unwrap()) {
        Ok(pubkey) => ResultExport{
            is_error: 0 ,
            result: Box::into_raw(Box::new(pubkey)),
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
extern "C" fn pubkey_create_with_seed(base: *const Pubkey, seed: *const c_char, owner: *const Pubkey)-> ResultExport<*mut Pubkey>{
    let base = unsafe {
        assert!(!base.is_null());
        &*base
    };

    let owner = unsafe {
        assert!(!owner.is_null());
        &*owner
    };

    let seed = str_from_c_char_ptr(seed).unwrap();

    let pubkey_result = Pubkey::create_with_seed(base, seed, owner);
    match pubkey_result {
        Ok(pubkey) => ResultExport{
            is_error: 0 ,
            result: Box::into_raw(Box::new(pubkey)),
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
extern "C" fn pubkey_find_program_address(seeds:  *const Seeds<u8>, len: size_t, program_id: *const Pubkey)-> ProgramAddress {

    let array_slice = unsafe { std::slice::from_raw_parts(seeds, len  as usize) };
    let mut vec_of_seed : Vec<&[u8]> = Vec::new();
    for seed_slice in array_slice {
        let slice = unsafe { std::slice::from_raw_parts(seed_slice.seed, seed_slice.length  as usize) };
        vec_of_seed.push(slice)
    };


    let program_id = unsafe {
        assert!(!program_id.is_null());
        &*program_id
    };


    let program_address = Pubkey::find_program_address(vec_of_seed.as_slice(),program_id);

    ProgramAddress {
        pubkey: Box::into_raw(Box::new(program_address.0)),
        bump_seed: program_address.1
    }
}

#[no_mangle]
extern "C" fn pubkey_try_find_program_address(seeds: *const Seeds<u8>, len: size_t, program_id: *mut Pubkey) -> ProgramAddress {

    let array_slice = unsafe { std::slice::from_raw_parts(seeds, len  as usize) };
    let mut vec_of_seed : Vec<&[u8]> = Vec::new();
    for seed_slice in array_slice {
        let slice = unsafe { std::slice::from_raw_parts(seed_slice.seed, seed_slice.length  as usize) };
        vec_of_seed.push(slice)
    };


    let program_id = unsafe {
        assert!(!program_id.is_null());
        &*program_id
    };


    let program_address_result = Pubkey::try_find_program_address(vec_of_seed.as_slice(),program_id);
    match program_address_result {
        None => ProgramAddress {
            pubkey: null_mut(),
            bump_seed: 0
        },
        Some(program_address) => ProgramAddress {
            pubkey: Box::into_raw(Box::new(program_address.0)),
            bump_seed: program_address.1
        }
    }
}

#[no_mangle]
extern "C" fn pubkey_create_program_address(seeds:  *const Seeds<u8>, len: size_t, program_id: *const Pubkey)-> ResultExport<*mut Pubkey> {
    let array_slice = unsafe { std::slice::from_raw_parts(seeds, len  as usize) };
    let mut vec_of_seed : Vec<&[u8]> = Vec::new();
    for seed_slice in array_slice {
        let slice = unsafe { std::slice::from_raw_parts(seed_slice.seed, seed_slice.length  as usize) };
        vec_of_seed.push(slice)
    };

    let program_id = unsafe {
        assert!(!program_id.is_null());
        &*program_id
    };

    match Pubkey::create_program_address(vec_of_seed.as_slice(),program_id) {
        Ok(pubkey) => ResultExport{
            is_error: 0 ,
            result: Box::into_raw(Box::new(pubkey)),
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
extern "C" fn pubkey_to_bytes(ptr : *mut Pubkey) -> VecRawParts {
    let pubkey = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    to_raw_parts(pubkey.to_bytes().to_vec())
}

#[no_mangle]
extern "C" fn pubkey_to_base58(ptr : *mut Pubkey) -> *mut c_char {
    let pubkey = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    CString::new(pubkey.to_string()).unwrap().into_raw()
}

#[no_mangle]
extern "C" fn pubkey_is_on_curve(ptr : *mut Pubkey) -> u8 {
    let pubkey = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    pubkey.is_on_curve() as u8
}

#[no_mangle]
extern "C" fn free_pubkey(ptr: *mut Pubkey){
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}
