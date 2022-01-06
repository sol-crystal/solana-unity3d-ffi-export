use std::ffi::CString;
use std::hash::Hash;
use std::os::raw::c_char;
use std::ptr::null_mut;
use bip39::Seed;

use libc::size_t;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signature, Signer};
use solana_sdk::signer::keypair::{keypair_from_seed, keypair_from_seed_phrase_and_passphrase};

use crate::c_sharp_string::str_from_c_char_ptr;
use crate::common_types::{ResultExport, to_raw_parts, VecRawParts};

#[no_mangle]
extern "C" fn new_keypair() -> *mut Keypair {
    Box::into_raw(Box::new(Keypair::new()))
}

#[no_mangle]
extern "C" fn from_bytes(bytes: *const u8, len: size_t) -> ResultExport<*mut Keypair>{

    let bytes = unsafe {
        assert!(!bytes.is_null());
        std::slice::from_raw_parts(bytes  , len as usize)
    };

    match Keypair::from_bytes(bytes){
        Ok(keypair) => ResultExport{
            is_error: 0,
            result: Box::into_raw(Box::new(keypair)),
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
extern "C" fn to_bytes(ptr : *mut Keypair)  -> VecRawParts {
    let keypair = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    to_raw_parts(keypair.to_bytes().to_vec())

}

#[no_mangle]
extern "C" fn from_base58_string(bs58: *const c_char) -> *mut Keypair {
    Box::into_raw(Box::new(Keypair::from_base58_string(str_from_c_char_ptr(bs58).unwrap())))
}

#[no_mangle]
extern "C" fn to_base58_string(ptr : *mut Keypair) -> *mut c_char {
    let keypair = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    CString::new(keypair.to_base58_string()).unwrap().into_raw()
}

#[no_mangle]
extern "C" fn secret(ptr : *mut Keypair) -> VecRawParts {
    let keypair = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    to_raw_parts(keypair.secret().to_bytes().to_vec())
}

#[no_mangle]
extern "C" fn pubkey(ptr : *mut Keypair) -> *mut Pubkey {
    let keypair = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    Box::into_raw(Box::new(keypair.pubkey()))
}

#[no_mangle]
extern "C" fn sign_message(ptr : *mut Keypair, message: *const u8, len: size_t) -> *mut Signature {
    let keypair = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let message = unsafe {
        assert!(!message.is_null());
        std::slice::from_raw_parts(message  , len as usize)
    };

    Box::into_raw(Box::new(keypair.sign_message(message)))

}

#[no_mangle]
extern "C" fn is_interactive(ptr : *mut Keypair) -> u8 {
    let keypair = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    keypair.is_interactive() as u8

}
#[no_mangle]
extern "C" fn  keypair_from_seed_entropy(seed: *mut Seed) -> ResultExport<*mut Keypair> {
    let seed = unsafe {
        assert!(!seed.is_null());
        &mut *seed
    };

    match   keypair_from_seed(seed.as_bytes()){
        Ok(keypair) => ResultExport{
            is_error: 0,
            result: Box::into_raw(Box::new(keypair)),
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
extern "C" fn from_seed_phrase_and_passphrase(seed_phrase: *const c_char, passphrase: *const c_char) -> ResultExport<*mut Keypair>{

   match keypair_from_seed_phrase_and_passphrase(
       str_from_c_char_ptr(seed_phrase).unwrap(), str_from_c_char_ptr(passphrase).unwrap())
   {
       Ok(keypair) => ResultExport{
           is_error: 0,
           result: Box::into_raw(Box::new(keypair)),
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
pub extern "C" fn free_keypair(ptr: *mut Keypair) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        Box::from_raw(ptr);
    }
}

//https://stackoverflow.com/questions/66775719/how-should-i-free-a-c-sharp-byte-allocated-in-rust