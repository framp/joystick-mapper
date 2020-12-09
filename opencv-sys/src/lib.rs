#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

include!(concat!(env!("OUT_DIR"), "/opencv-sys.rs"));

impl Mats {
    pub fn new_of_len(length: usize) -> Self {
        let mut mats_: Vec<*mut ::std::os::raw::c_void> = Vec::with_capacity(length);
        unsafe { mats_.set_len(length) };
        let mut boxed = mats_.into_boxed_slice();
        let mats = Mats {
            mats: boxed.as_mut_ptr(),
            length: boxed.len() as i32,
        };
        mats
    }
}
