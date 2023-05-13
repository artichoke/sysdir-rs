#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![allow(clippy::pedantic)]
#![allow(clippy::restriction)]

pub const PATH_MAX: u32 = 1024;

#[repr(u32)]
pub enum sysdir_search_path_directory_t {
    SYSDIR_DIRECTORY_USER,
}

pub const SYSDIR_DOMAIN_MASK_LOCAL: sysdir_search_path_domain_mask_t = 2;
pub type sysdir_search_path_domain_mask_t = ::core::ffi::c_uint;

pub unsafe fn sysdir_start_search_path_enumeration(
    dir: sysdir_search_path_directory_t,
    domainMask: sysdir_search_path_domain_mask_t,
) -> sysdir_search_path_enumeration_state {
    unimplemented!();
}

pub unsafe fn sysdir_get_next_search_path_enumeration(
    state: sysdir_search_path_enumeration_state,
    path: *mut ::core::ffi::c_char,
) -> sysdir_search_path_enumeration_state {
    unimplemented!();
}

pub type sysdir_search_path_enumeration_state = ::core::ffi::c_uint;
