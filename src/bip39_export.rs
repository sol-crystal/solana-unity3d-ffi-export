use std::error::Error;
use std::ffi::CString;
use std::ptr::null_mut;
use bip39::{Language, Mnemonic, MnemonicType, Seed};
use libc::size_t;
use std::os::raw::c_char;
use crate::c_sharp_string::str_from_c_char_ptr;
use crate::common_types::{ResultExport, to_raw_parts, VecRawParts};

pub fn language_from_str(lang: &str) -> Option<Language> {
    match lang {
        "English" => Some(Language::English),
        "ChineseSimplified" =>  Some(Language::ChineseSimplified),
        "ChineseTraditional" => Some(Language::ChineseTraditional),
        "French" => Some(Language::French),
        "Italian" => Some(Language::Italian),
        "Japanese" => Some(Language::Japanese),
        "Korean" => Some(Language::Korean),
        "Spanish" => Some(Language::Spanish),
        _ => None
    }
}

pub fn mnemonic_type_from_str(mtype: &str) -> Option<MnemonicType>{
    match mtype {
        "Words12" => Some(MnemonicType::Words12),
        "Words15" => Some(MnemonicType::Words15),
        "Words18" => Some(MnemonicType::Words18),
        "Words21" => Some(MnemonicType::Words21),
        "Words24" => Some(MnemonicType::Words24),
        _ => None
    }
}

#[no_mangle]
extern "C"  fn mnemonic_new(mtype: *const c_char, lang: *const c_char) -> *mut Mnemonic {

    let mnemonic_type = mnemonic_type_from_str(str_from_c_char_ptr(mtype).unwrap()).unwrap();
    let language = language_from_str(str_from_c_char_ptr(lang).unwrap()).unwrap();

    Box::into_raw(Box::new(Mnemonic::new(mnemonic_type, language)))
}


#[no_mangle]
extern "C"  fn from_entropy(entropy: *const u8, entropy_length: size_t , lang: *const c_char) -> ResultExport<*mut Mnemonic> {
    let entropy = unsafe{ std::slice::from_raw_parts(entropy, entropy_length as usize)};
    let lang = language_from_str(str_from_c_char_ptr(lang).unwrap()).unwrap();

    match Mnemonic::from_entropy(entropy, lang) {
        Ok(mnemonic) => ResultExport{
            is_error: 0,
            result: Box::into_raw(Box::new(mnemonic)),
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
extern "C"  fn from_phrase(phrase: *const c_char, lang: *const c_char) -> ResultExport<*mut Mnemonic>{
    let language = language_from_str(str_from_c_char_ptr(lang).unwrap()).unwrap();

    match Mnemonic::from_phrase(str_from_c_char_ptr(phrase).unwrap(), language) {
        Ok(mnemonic) => ResultExport{
            is_error: 0,
            result: Box::into_raw(Box::new(mnemonic)),
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
extern "C"  fn validate(phrase: *const c_char, lang: *const c_char) -> ResultExport<u8>{
    let lang = language_from_str(str_from_c_char_ptr(lang).unwrap()).unwrap();
    let phrase = str_from_c_char_ptr(phrase).unwrap();

    match Mnemonic::validate(phrase, lang) {
        Ok(..) => ResultExport{
            is_error: 0,
            result: true as u8,
            error: null_mut()
        },
        Err(error) => ResultExport{
            is_error: 1,
            result: false as u8,
            error: CString::new(error.to_string()).unwrap().into_raw()
        }
    }
}


#[no_mangle]
extern "C"  fn phrase(mnemonic: *mut Mnemonic) -> *mut c_char {
    let mnemonic = unsafe {
        assert!(!mnemonic.is_null());
        &mut *mnemonic
    };
    CString::new(mnemonic.phrase()).unwrap().into_raw()
}


#[no_mangle]
extern "C"  fn into_phrase(mnemonic: *mut Mnemonic) -> *mut c_char {
    let mnemonic = unsafe {
        assert!(!mnemonic.is_null());
        &mut *mnemonic
    };

    CString::new(mnemonic.clone().into_phrase()).unwrap().into_raw()
}


#[no_mangle]
extern "C"  fn entropy(mnemonic: *mut Mnemonic) -> VecRawParts {

    let mnemonic = unsafe {
        assert!(!mnemonic.is_null());
        &mut *mnemonic
    };
    to_raw_parts(mnemonic.entropy().to_vec())
}

// #[no_mangle]
// extern "C" fn language(mnemonic: *mut Mnemonic) -> *mut c_char {
//
//     let mnemonic = unsafe {
//         assert!(!mnemonic.is_null());
//         &mut *mnemonic
//     };
//
//     CString::new(mnemonic.language().as_ref()).unwrap().into_raw()
// }

#[no_mangle]
extern "C" fn seed_new(mnemonic: *mut Mnemonic, password: *const c_char) -> *mut Seed {

    let mnemonic = unsafe {
        assert!(!mnemonic.is_null());
        &mut *mnemonic
    };

    let password = str_from_c_char_ptr(password).unwrap();
    Box::into_raw(Box::new(Seed::new(mnemonic, password)))
}

#[no_mangle]
extern "C" fn as_bytes(seed: *mut Seed) -> VecRawParts{
    let seed = unsafe {
        assert!(!seed.is_null());
        &mut *seed
    };

    to_raw_parts(seed.as_bytes().to_vec())
}

#[no_mangle]
extern "C" fn free_mnemonic(mnemonic: *mut Mnemonic){
    if mnemonic.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(mnemonic);
    }
}

#[no_mangle]
extern "C" fn free_seed(seed: *mut Seed){
    if seed.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(seed);
    }
}