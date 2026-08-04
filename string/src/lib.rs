/*
* Porting of the libssh string library from C to Rust.
*/


use std::ffi::{c_char, c_void};
use std::ptr::{copy_nonoverlapping, null_mut};

#[repr(C)]
#[derive(Debug)]
struct ssh_string_struct {
    data: *mut u8,
    size: usize,
}

#[no_mangle]
pub extern "C"
fn ssh_string_new(size: usize) -> *mut ssh_string_struct
{
}

#[no_mangle]
pub extern "C"
fn ssh_string_fill(s: *mut ssh_string_struct,
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

#[no_mangle]
pub extern "C" fn ssh_string_from_char(what: *const c_char) -> *mut ssh_string_struct {
    if what.is_null() {
        return null_mut();
    }
    let len: usize = what.len();
    let ptr: *mut ssh_string_struct = ssh_string_new(len);

    if ptr.is_null() {
        return null_mut();
    }

    unsafe {
        copy_nonoverlapping(what, (*ptr).data, len);
    }
    ptr
}

#[no_mangle]
pub extern "C" fn ssh_string_data(data: *const c_void, len: usize) -> *mut ssh_string_struct {
    if data.is_null() && len > 0 {
        return null_mut();
    }

    let ptr: *mut ssh_string_struct = ssh_string_new(len);

    if ptr.is_null() {
        return null_mut();
    }

    if len > 0 {
        let rc: isize = ssh_string_fill(ptr, data, len);
        if rc != 0 {
            ssh_string_free(ptr);
            return null_mut();
        }
    }
    ptr

}


/*Rust equivalent for converting network bytes to host bytes */
#[no_mangle]
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

#[no_mangle]
pub extern "C" fn ssh_string_get_char(s: *mut ssh_string_struct) -> *const u8 {
    if s.is_null() {
        return null_mut();
    }

    let c = (*s).data[0..1]; // slice the string to get first byte
    return *c;
}

#[no_mangle]
pub extern "C" fn ssh_string_to_char(s: *mut ssh_string_struct) -> *mut u8 {
    if s.is_null() {
        return null_mut();
    }

    let new = (*s).data[0..1];

    return new;
}

#[no_mangle]
pub extern "C" fn ssh_string_free_char(s: *mut u8) {
    SAFE_FREE!(s); // use a macro;
}

#[no_mangle]
pub extern "C" fn ssh_string_copy(s: *mut ssh_string_struct) -> *mut ssh_string_struct {

    if s.is_null() {
        return null_mut();
    }

    let size: usize = unsafe {(*s).size};
    let new: *mut ssh_string_struct = ssh_string_new(size);

    if new.is_null() {
        return null_mut();
    }
    unsafe {
        copy_nonoverlapping((*new).data, (*s).data, size);
    }
    new
}

pub extern "C" fn ssh_string_cmp(s1: ssh_string_struct, s2: ssh_string_struct) -> isize {
    if s1.is_null() || s2.is_null() {
        return 1;
    }
    assert!(s1 == s2);

    return 0;
}

pub extern "C" fn ssh_string_burn(s: ssh_string_struct) -> *mut u8 {
    if s.is_null() || s.size == 0 {
        return null_mut();
    }

    ssh_burn(s.data, ssh_string_len(s));
}

pub extern "C" fn ssh_string_data() -> *mut u8 {
    if s.is_null() {
        return null_mut();
    }

    return s.data;
}

pub extern "C" fn ssh_string_free(s: ssh_string_struct) {
    SAFE_FREE(s);
}

fn main() {
}


#[cfg(test)]
mod test {
}
