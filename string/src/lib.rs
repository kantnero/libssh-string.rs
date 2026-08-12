/*
* Porting of the libssh string library from C to Rust.
*/

#![allow(unsed, dead_code)]
use std::ffi::{c_char, c_void, CStr};
use std::ptr::{copy_nonoverlapping, null_mut};
use std::alloc::{alloc_zeroed, dealloc, handle_alloc_error, Layout};

#[repr(C)]
#[derive(Debug)]
struct SshStringStruct {
    data: *mut u8,
    size: usize,
}

type ssh_string_struct = SshStringStruct;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ssh_string_new(size: usize) -> *mut ssh_string_struct
{
    if size == 0 {
        /* 0 crashed layout so we do something different when size is 0*/
       // let s: ssh_string_struct = ssh_string_zero(size);
//        return s;
    }

    let layout = Layout::array::<u8>(size).unwrap();
    let data = alloc_zeroed(layout);
    if data.is_null() {
        handle_alloc_error(layout);
    }

    let s: Box<SshStringStruct> = Box::new(
        SshStringStruct {
            data,
            size
        }
    );

    let ptr = Box::into_raw(s);
    ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SAFE_FREE(ptr: *mut ssh_string_struct) {
    if ptr.is_null() {
        return
    }

    let s = Box::from_raw(ptr);
    let layout = Layout::array::<u8>(s.size).unwrap();
    dealloc(s.data, layout);
}

#[unsafe(no_mangle)]
pub extern "C" fn ssh_string_fill(s: *mut ssh_string_struct,
                                  data: *const c_void,
                                  len: usize) -> isize
{
    if data.is_null() || s.is_null() ||
       len == 0 || len > ssh_string_len(s) {
        return -1;
    }
    unsafe {
        let data = data as *const u8; // shadowing cause of type mismatch
        copy_nonoverlapping(data, (*s).data, len);
    }
    return 0;

}

#[unsafe(no_mangle)]
pub extern "C" fn ssh_string_from_char(what: *const c_char) -> *mut ssh_string_struct {
    if what.is_null() {
        return null_mut();
    }
    let len: usize = unsafe { CStr::from_ptr(what) }
        .to_bytes()
        .len();
    let ptr: *mut ssh_string_struct = unsafe { ssh_string_new(len) };

    if ptr.is_null() {
        return null_mut();
    }

    unsafe {
        let what = what as *const u8;
        // Instead of this call ssh_string_fill filling the s->data with what
        copy_nonoverlapping(what, (*ptr).data, len);
    }
    ptr
}

#[unsafe(no_mangle)]
pub extern "C" fn ssh_string_from_data(data: *const c_void, len: usize) -> *mut ssh_string_struct {
    if data.is_null() && len > 0 {
        return null_mut();
    }

    let ptr: *mut ssh_string_struct = unsafe { ssh_string_new(len) };

    if ptr.is_null() {
        return null_mut();
    }

    if len > 0 {
        let rc: isize = ssh_string_fill(ptr, data, len);
        if rc != 0 {
            unsafe {
                ssh_string_free(ptr);
            }
            return null_mut();
        }
    }
    ptr

}


/* Find rust equivalent of htnol() for converting network bytes to host bytes */
#[unsafe(no_mangle)]
pub extern "C" fn ssh_string_len(s: *mut ssh_string_struct) -> usize {
    if s.is_null() {
        return 0;
    }

    let size: usize = unsafe { (*s).size };
    if size > 0 {
        return size;
    }
    return 0;
}

#[unsafe(no_mangle)]
pub extern "C" fn ssh_string_get_char(s: *mut ssh_string_struct) -> *const u8 {
    if s.is_null() {
        return null_mut();
    }

    let c = unsafe { (*s).data }; // slice the string to get first byte
    return c;
}

#[unsafe(no_mangle)]
pub extern "C" fn ssh_string_to_char(s: *mut ssh_string_struct) -> *mut u8 {
    if s.is_null() {
        return null_mut();
    }

    let new = unsafe { (*s).data };
    return new;
}
/*
#[unsafe(no_mangle)]
pub extern "C" fn ssh_string_free_char(s: *mut u8) {
    SAFE_FREE(s); // use a macro;
}
*/
#[unsafe(no_mangle)]
pub extern "C" fn ssh_string_copy(s: *mut ssh_string_struct) -> *mut ssh_string_struct {

    if s.is_null() {
        return null_mut();
    }

    let size: usize = unsafe {(*s).size};
    let new: *mut ssh_string_struct = unsafe { ssh_string_new(size) };

    if new.is_null() {
        return null_mut();
    }
    unsafe {
        copy_nonoverlapping((*s).data, (*new).data, size);
    }
    new
}

#[unsafe(no_mangle)]
pub extern "C" fn ssh_string_cmp(s1: *const ssh_string_struct,
                                 s2: *const ssh_string_struct) -> isize 
{
    if s1.is_null() || s2.is_null() {
        return 1;
    }

    /*TODO: can not use assert to compare string it panics. */
    assert!(s1 == s2);

    return 0;
}
/* 
#[unsafe(no_mangle)]
pub extern "C" fn ssh_string_burn(s: *mut ssh_string_struct) {
    if s.is_null() || (*s).size == 0 {
        return;
    }

    ssh_burn((*s).data, ssh_string_len(s));
}
*/
#[unsafe(no_mangle)]
pub extern "C" fn ssh_string_data(s: *mut ssh_string_struct) -> *mut u8 {
    if s.is_null() {
        return null_mut();
    }

    unsafe {
        (*s).data
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ssh_string_free(s: *mut ssh_string_struct) {
    if s.is_null() {
        return;
    }

    SAFE_FREE(s);
}

fn main() {
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn new() {
    }

    fn fill() {
        let s: *mut ssh_string_struct = ssh_string_new(11);
        let data: CStr = "hello";
        let rc = ssh_string_fill(s, data, Cstr::from_ptr(data).to_bytes().len());
        assert_eq!((*s).data, data as *mut u8);
    }

}
