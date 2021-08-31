use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

use icu_uniset::{UnicodeSet, UnicodeSetBuilder};

pub type ICU4XUniset = UnicodeSet;

unsafe fn ptr_to_str<'a>(raw: *const c_char) -> Option<&'a str> {
    if raw.is_null() {
        return None;
    }
    CStr::from_ptr(raw).to_str().ok()
}

#[no_mangle]
pub unsafe extern "C" fn icu4x_uniset_create_for_property(
    prop_name: *const c_char,
    prop_value: *const c_char,
) -> *mut ICU4XUniset {
    let prop_name = match ptr_to_str(prop_name) {
        Some(prop_name) => prop_name,
        None => return ptr::null_mut(),
    };
    let prop_value = ptr_to_str(prop_value);

    match crate::property::get_unicode_set(prop_name, prop_value) {
        Some(set) => Box::into_raw(Box::new(set)),
        None => ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn icu4x_uniset_complemented(
    uniset: *mut ICU4XUniset
) -> *mut ICU4XUniset {
    let set = Box::from_raw(uniset);
    let mut builder = UnicodeSetBuilder::new();
    builder.add_set(&set);
    builder.complement();
    Box::into_raw(Box::new(builder.build()))
}

#[no_mangle]
pub unsafe extern "C" fn icu4x_uniset_get_range_count(uniset: *const ICU4XUniset) -> usize {
    (&*uniset).get_range_count()
}

#[no_mangle]
pub unsafe extern "C" fn icu4x_uniset_get_range_start(
    uniset: *const ICU4XUniset,
    index: usize
) -> u32 {
    (&*uniset).get_nth_range(index).map(|range| *range.start()).unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn icu4x_uniset_get_range_end(
    uniset: *const ICU4XUniset,
    index: usize
) -> u32 {
    (&*uniset).get_nth_range(index).map(|range| *range.end()).unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn icu4x_uniset_destroy(uniset: *mut ICU4XUniset) {
    let _ = Box::from_raw(uniset);
}
