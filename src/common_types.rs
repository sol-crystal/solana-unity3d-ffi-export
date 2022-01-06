use std::ffi::c_void;
use std::os::raw::c_char;
use std::ptr::null_mut;

#[repr(C)]
pub struct ResultExport<T> {
    pub is_error: u8,
    pub result: T,
    pub error: *mut c_char,
}

#[repr(C)]
pub struct OptionExport<T> {
    pub has_value: u8,
    pub value: T,
}


#[repr(C)]
pub struct VecRawParts {
    pub data: *const c_void,
    pub length: u32,
    pub capacity: u32,
}

impl Default for VecRawParts {
    fn default() -> Self {
        Self{
            data: null_mut(),
            length: 0,
            capacity: 0
        }
    }
}

#[no_mangle]
extern "C" fn init_vec(byte_length: u32, out_raw_parts_ptr: *mut VecRawParts) {
    let mut vec = Vec::<u8>::new();
    vec.resize(byte_length as usize, 0);
    let raw = Vec::into_raw_parts(vec);
    let mut raw_parts = unsafe { out_raw_parts_ptr.as_mut().unwrap() };
    raw_parts.data = raw.0 as *const c_void;
    raw_parts.length = raw.1 as u32;
    raw_parts.capacity = raw.2 as u32;
}

pub fn from_raw_parts<T>(parts: &VecRawParts) -> Vec<T> {
    unsafe {
        if parts.capacity == 0 {
            return Vec::new();
        } else {
            let element_size = std::mem::size_of::<T>();
            Vec::<T>::from_raw_parts(
                parts.data as *mut T,
                parts.length as usize / element_size,
                parts.capacity as usize / element_size,
            )
        }
    }
}

#[no_mangle]
extern "C" fn free_vec(raw_parts: VecRawParts) {
    unsafe {
        Vec::from_raw_parts(
            raw_parts.data as *mut c_void,
            raw_parts.length as usize,
            raw_parts.capacity as usize,
        );
    }
}

pub fn to_raw_parts<T>(vec: Vec<T>) -> VecRawParts {
    //unsafe {
        if vec.capacity() == 0 {
            return VecRawParts{
                data: null_mut(),
                length: 0,
                capacity: 0
            };
        } else {
            let raw_parts = vec.into_raw_parts();
            VecRawParts{
                data: raw_parts.0 as *mut c_void,
                length: raw_parts.1 as u32,
                capacity: raw_parts.2 as u32
            }
        }
    //}
}
