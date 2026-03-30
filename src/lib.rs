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
//! Note that this crate is completely empty on non-Apple platforms.
//!
//! ## Linkage
//!
//! `sysdir(3)` is provided by `libSystem`, which is linked into every binary on
//! Apple platforms. This crate does not link to `CoreFoundation`, `Foundation`,
//! or any other system libraries and frameworks.
//!
//! ## Path Semantics
//!
//! These bindings expose raw `sysdir(3)` search-path strings from Darwin.
//! Returned values are not normalized filesystem paths:
//!
//! - user-domain results may contain a literal `~` instead of an expanded home
//!   directory
//! - if `NEXT_ROOT` is set and honored by the process, many local, network, and
//!   system-domain results are prefixed by that directory
//! - callers should not assume returned values are valid UTF-8 if `NEXT_ROOT`
//!   contains non-UTF-8 bytes
//!
//! Callers that intend to use these values with filesystem APIs should expand
//! `~`, account for `NEXT_ROOT`, and validate UTF-8 before opening or creating
//! files.
//!
//! # Examples
//!
#![cfg_attr(
    any(
        target_os = "macos",
        target_os = "ios",
        target_os = "tvos",
        target_os = "watchos"
    ),
    doc = "```"
)]
#![cfg_attr(
    not(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "tvos",
        target_os = "watchos"
    )),
    doc = "```compile_fail"
)]
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
//!         let bytes = path.to_bytes();
//!         // `sysdir(3)` may prefix local-domain results with `NEXT_ROOT`,
//!         // which can also introduce non-UTF-8 bytes.
//!         assert!(bytes.ends_with(b"/Users"));
//!     }
//! }
//! ```

#![no_std]
#![doc(html_root_url = "https://docs.rs/sysdir/1.3.2")]

#[cfg(test)]
extern crate std;

// Ensure code blocks in `README.md` compile
#[cfg(all(
    doctest,
    any(
        target_os = "macos",
        target_os = "ios",
        target_os = "tvos",
        target_os = "watchos"
    )
))]
#[doc = include_str!("../README.md")]
mod readme {}

/// man page for `sysdir(3)`.
///
/// ```text
#[doc = include_str!("../sysdir.3")]
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
#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "watchos"
))]
mod sys;

#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "watchos"
))]
pub use self::sys::*;

#[cfg(all(
    test,
    any(
        target_os = "macos",
        target_os = "ios",
        target_os = "tvos",
        target_os = "watchos"
    )
))]
mod tests {
    use core::ffi::{CStr, c_char};
    use std::os::unix::ffi::OsStrExt;
    use std::{borrow::Cow, env};

    use super::*;

    fn expected_local_users_directory() -> Cow<'static, [u8]> {
        match env::var_os("NEXT_ROOT") {
            Some(next_root) => {
                let next_root = next_root.as_os_str().as_bytes();
                let next_root = next_root
                    .iter()
                    .rposition(|&byte| byte != b'/')
                    .map_or(&[][..], |pos| &next_root[..=pos]);
                if next_root.is_empty() {
                    Cow::Borrowed(b"/Users")
                } else {
                    let mut path = std::vec::Vec::with_capacity(next_root.len() + b"/Users".len());
                    path.extend_from_slice(next_root);
                    path.extend_from_slice(b"/Users");
                    Cow::Owned(path)
                }
            }
            None => Cow::Borrowed(b"/Users"),
        }
    }

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
        let expected = expected_local_users_directory();

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
                let bytes = path.to_bytes();
                assert_eq!(bytes, expected.as_ref());
                count += 1;
            }
        }

        assert_eq!(count, 1, "Should iterate once and find `/Users`");
    }

    #[test]
    fn example_and_linkage_with_opaque_state_helpers() {
        let mut count = 0_usize;
        let mut path = [0; PATH_MAX as usize];
        let expected = expected_local_users_directory();

        let dir = sysdir_search_path_directory_t::SYSDIR_DIRECTORY_USER;
        let domain_mask = SYSDIR_DOMAIN_MASK_LOCAL;

        unsafe {
            let mut state = sysdir_start_search_path_enumeration(dir, domain_mask);
            loop {
                let path = path.as_mut_ptr().cast::<c_char>();
                state = sysdir_get_next_search_path_enumeration(state, path);
                if state.is_finished() {
                    break;
                }
                let path = CStr::from_ptr(path);
                let bytes = path.to_bytes();
                assert_eq!(bytes, expected.as_ref());
                count += 1;
            }
        }

        assert_eq!(count, 1, "Should iterate once and find `/Users`");
    }
}
