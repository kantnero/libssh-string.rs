/*
* Porting of the libssh string library from C to Rust.
*/
#![allow(unused, dead_code)]
use std::ffi::{
    c_char,
    c_void,
    CStr,
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
    let layout = Layout::array::<u8>(size).unwrap(); // 1 bytes * size
//    println!("{:#?}", layout);
//    println!("{}", std::mem::size_of::<u8>());
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
        copy_nonoverlapping(data as *const u8, (*s).data, len);
        return 0;
    }

}

#[unsafe(no_mangle)]
pub extern "C" fn ssh_string_from_char(what: *const i8) -> *mut ssh_string_struct {
    if what.is_null() {
        return null_mut();
    }

    let len = unsafe {
        CStr::from_ptr(what)
            .to_bytes()
            .len()
    };
    let s = unsafe {
        ssh_string_new(len)
    };
    unsafe {
        copy_nonoverlapping(what as *const u8, (*s).data, len)
    }
    s
}

#[unsafe(no_mangle)]
pub extern "C" fn ssh_string_from_data(data: *const c_void, len: usize) -> *mut ssh_string_struct {
    if data.is_null() && len > 0 {
        return null_mut();
    }

    let s = ssh_string_new(len);
    if s.is_null() {
        return null_mut();
    }

    if len > 0 {
        let rc = ssh_string_fill(s, data, len);

        if rc != 0 {
//        ssh_string_free(s);
        return null_mut();
        }
    }
    s
}

#[unsafe(no_mangle)]
pub extern "C" fn ssh_string_len(s: *mut ssh_string_struct) -> usize {
    if s.is_null() {
        return 0;
    }

    let size = unsafe {
        (*s).size // convert to ntohl
    };
    if size > 0 {
        return size;
    }
    return 0;
}

#[unsafe(no_mangle)]
pub extern "C" fn ssh_string_get_char(s: *mut ssh_string_struct) -> *mut u8 {
    if s.is_null() {
        return null_mut()
    }

    unsafe {
        (*s).data
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ssh_string_to_char(s: *mut ssh_string_struct) ->*mut u8 {
    if s.is_null() {
        return null_mut();
    }
    let len = ssh_string_len(s);
    let layout = Layout::array::<i8>(len).unwrap();
    let new = unsafe {
        alloc(layout)
    };
    unsafe {
        copy_nonoverlapping((*s).data, new, len)
    };
    new
}

#[unsafe(no_mangle)]
pub extern "C" fn ssh_string_free_char(c: *mut c_char) {
    let len = unsafe {
        CStr::from_ptr(c)
            .to_bytes()
            .len()
    };
    let layout = Layout::array::<i8>(len).unwrap();

    unsafe {
        dealloc(c as *mut u8, layout);
    };
}
