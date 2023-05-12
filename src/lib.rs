// src/lib.rs
//
// Copyright (c) 2023 Ryan Lopopolo <rjl@hyperbo.la>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE> or
// <http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT>
// or <http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(unknown_lints)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]
// Enable feature callouts in generated documentation:
// https://doc.rust-lang.org/beta/unstable-book/language-features/doc-cfg.html
//
// This approach is borrowed from tokio.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_alias))]

//! Enumeration of the filesystem paths for the various standard system
//! directories where apps, resources, etc. get installed.
//!
//! This crate exposes Rust bindings to the `sysdir(3)` library functions
//! provided by `libSystem.dylib` on macOS, iOS, tvOS, and watchOS.
//!
//! For more detailed documentation, refer to the [`sysdir(3)` man page](mod@man).
//!
//! # Platform Support
//!
//! The `sysdir` API first appeared in OS X 10.12, iOS 10, watchOS 3 and tvOS 10
//! replacing the deprecated `NSSystemDirectories(3)` API.
//!
//! # Linkage
//!
//! `sysdir(3)` is provided by `libSystem`, which is linked into every binary on
//! Apple platforms. This crate does not link to `CoreFoundation`, `Foundation`,
//! or any other system libraries and frameworks.
//!
//! # Examples
//!
//! ```
//! use core::ffi::{c_char, CStr};
//!
//! use sysdir::*;
//!
//! let mut path = [0; PATH_MAX as usize];
//!
//! let dir = sysdir_search_path_directory_t::SYSDIR_DIRECTORY_USER;
//! let domain_mask = SYSDIR_DOMAIN_MASK_LOCAL;
//!
//! unsafe {
//!     let mut state = sysdir_start_search_path_enumeration(dir, domain_mask);
//!     loop {
//!         let path = path.as_mut_ptr().cast::<c_char>();
//!         state = sysdir_get_next_search_path_enumeration(state, path);
//!         if state == 0 {
//!             break;
//!         }
//!         let path = CStr::from_ptr(path);
//!         let s = path.to_str().unwrap();
//!         assert_eq!(s, "/Users");
//!     }
//! }
//! ```

#![no_std]
#![doc(html_root_url = "https://docs.rs/sysdir/1.0.0")]

// Ensure code blocks in `README.md` compile
#[doc = include_str!("../README.md")]
mod readme {}

/// man page for `sysdir(3)`.
///
/// ```text
#[doc = include_str!("../sysdir.3.man")]
/// ```
#[cfg(any(doc, doctest))]
pub mod man {}

/// Raw bindings to `sysdir(3)`, provided by `libSystem`.
///
/// The `sysdir` API first appeared in OS X 10.12, iOS 10, watchOS 3 and tvOS 10
/// replacing the deprecated `NSSystemDirectories(3)` API.
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[allow(clippy::all)]
#[allow(clippy::pedantic)]
#[allow(clippy::restriction)]
mod sys;

pub use self::sys::*;

#[cfg(test)]
mod tests {
    use core::ffi::{c_char, CStr};

    use super::*;

    // EXAMPLES
    //
    // ```c
    // #include <limits.h>
    // #include <sysdir.h>
    //
    // char path[PATH_MAX];
    // sysdir_search_path_enumeration_state state = sysdir_start_search_path_enumeration(dir, domainMask);
    // while ( (state = sysdir_get_next_search_path_enumeration(state, path)) != 0 ) {
    //     // Handle directory path
    // }
    // ```
    #[test]
    fn example_and_linkage() {
        let mut count = 0_usize;
        let mut path = [0; PATH_MAX as usize];

        let dir = sysdir_search_path_directory_t::SYSDIR_DIRECTORY_USER;
        let domain_mask = SYSDIR_DOMAIN_MASK_LOCAL;

        unsafe {
            let mut state = sysdir_start_search_path_enumeration(dir, domain_mask);
            loop {
                let path = path.as_mut_ptr().cast::<c_char>();
                state = sysdir_get_next_search_path_enumeration(state, path);
                if state == 0 {
                    break;
                }
                let path = CStr::from_ptr(path);
                let s = path.to_str().unwrap();
                assert_eq!(s, "/Users");
                count += 1;
            }
        }

        assert_eq!(count, 1, "Should iterate once and find `/Users`");
    }
}
