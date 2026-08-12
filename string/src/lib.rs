/*
* Porting of the libssh string library from C to Rust.
*/
#![allow(unused, dead_code)]
use std::ffi::{
    c_char,
    c_void,
};
use std::ptr::{
    null_mut,
    copy_nonoverlapping
};
use std::alloc::{
    alloc,
    dealloc,
    handle_alloc_error,
    Layout
};

#[repr(C)]
#[derive(Debug)]
pub struct ssh_string_struct {
    data: *mut u8,
    size: usize,
    layout: Layout
}
impl Drop for ssh_string_struct {
    fn drop(&mut self) {
        unsafe {
            println!("{:p}", self.data);
            dealloc(self.data, self.layout);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ssh_string_new(size: usize) -> *mut ssh_string_struct {
    let layout = Layout::array::<u8>(size).unwrap();
    println!("{:#?}", layout);
    let data = unsafe {
        alloc(layout)
    };

    if data.is_null() {
        return null_mut()
    }

    let s = Box::new(
        ssh_string_struct {
            data,
            size, // Find rust equivalent of htonl
            layout
        }
    );
    println!("{:p}", s);
    Box::into_raw(s)

}


#[unsafe(no_mangle)]
pub extern "C" fn ssh_string_fill(s: *mut ssh_string_struct, data: *const c_void, len: usize) -> i32 {
    if s.is_null() || data.is_null() ||
       len == 0  {
        return -1;
    }

    unsafe {
        let data = data as *const u8;
        copy_nonoverlapping(data, (*s).data, len);
        return 0;
    }

}

#[unsafe(no_mangle)]
pub extern "C" fn ssh_string_from_char(what: *const c_char) -> *mut ssh_string_struct {
    if what.is_null() {
        return null_mut();
    }

    let s = unsafe {
        ssh_string_new(10)
    };
    s
}


