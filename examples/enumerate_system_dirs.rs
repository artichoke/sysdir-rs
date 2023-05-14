// examples/enumerate_system_dirs.rs
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
#![allow(clippy::enum_glob_use)]
#![allow(clippy::wildcard_imports)]
#![allow(unknown_lints)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]

//! Example demonstrating the sysdir well-known directory enumeration API.
//!
//! # Usage
//!
//! ```shell
//! cargo run --example enumerate_system_dirs
//! ```

use std::io::{self, Write as _};
use std::process;

fn main() {
    match platform::try_main() {
        Ok(()) => {}
        Err(err) => {
            let _ignore = writeln!(io::stderr(), "{err}");
            process::exit(1);
        }
    }
}

#[cfg(not(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "watchos"
)))]
mod platform {
    use std::error::Error;
    use std::fmt;

    #[derive(Debug, Clone, Copy)]
    struct PlatformNotSupported;

    impl fmt::Display for PlatformNotSupported {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("sysdir is not supported on this platform. sysdir is only available on macOS, iOS, tvOS, and watchOS")
        }
    }

    impl Error for PlatformNotSupported {}

    pub fn try_main() -> Result<(), Box<dyn Error>> {
        return Err(Box::new(PlatformNotSupported));
    }
}

#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "watchos"
))]
mod platform {
    use std::error::Error;
    use std::ffi::{c_char, CStr};
    use std::io::{self, Write as _};

    use sysdir::{sysdir_search_path_directory_t::*, *};

    pub fn try_main() -> Result<(), Box<dyn Error>> {
        let domains = [
            (SYSDIR_DOMAIN_MASK_USER, "user"),
            (SYSDIR_DOMAIN_MASK_LOCAL, "local"),
            (SYSDIR_DOMAIN_MASK_NETWORK, "network"),
            (SYSDIR_DOMAIN_MASK_SYSTEM, "system"),
        ];
        let search_paths = [
            (SYSDIR_DIRECTORY_APPLICATION, "Applications"),
            (SYSDIR_DIRECTORY_DEMO_APPLICATION, "Applications/Demos"),
            (
                SYSDIR_DIRECTORY_DEVELOPER_APPLICATION,
                "Developer/Applications",
            ),
            (SYSDIR_DIRECTORY_ADMIN_APPLICATION, "Applications/Utilities"),
            (SYSDIR_DIRECTORY_LIBRARY, "Library"),
            (SYSDIR_DIRECTORY_DEVELOPER, "Developer"),
            (SYSDIR_DIRECTORY_USER, "Users"),
            (SYSDIR_DIRECTORY_DOCUMENTATION, "Library/Documentation"),
            (SYSDIR_DIRECTORY_DOCUMENT, "Documents"),
            (SYSDIR_DIRECTORY_CORESERVICE, "Library/CoreServices"),
            (
                SYSDIR_DIRECTORY_AUTOSAVED_INFORMATION,
                "Library/Autosave Information",
            ),
            (SYSDIR_DIRECTORY_DESKTOP, "Desktop"),
            (SYSDIR_DIRECTORY_CACHES, "Library/Caches"),
            (
                SYSDIR_DIRECTORY_APPLICATION_SUPPORT,
                "Library/Application Support",
            ),
            (SYSDIR_DIRECTORY_DOWNLOADS, "Downloads"),
            (SYSDIR_DIRECTORY_INPUT_METHODS, "Library/Input Methods"),
            (SYSDIR_DIRECTORY_MOVIES, "Movies"),
            (SYSDIR_DIRECTORY_MUSIC, "Music"),
            (SYSDIR_DIRECTORY_PICTURES, "Pictures"),
            (
                SYSDIR_DIRECTORY_PRINTER_DESCRIPTION,
                "Library/Printers/PPDs",
            ),
            (SYSDIR_DIRECTORY_SHARED_PUBLIC, "Public"),
            (SYSDIR_DIRECTORY_PREFERENCE_PANES, "Library/PreferencePanes"),
            (
                SYSDIR_DIRECTORY_ALL_APPLICATIONS,
                "/Applications, ~/Applications, /Applications/Utilities, etc",
            ),
            (
                SYSDIR_DIRECTORY_ALL_LIBRARIES,
                "/Library, ~/Library, /Network/Library, etc",
            ),
        ];

        let mut idx = domains.len() * search_paths.len() - 1;

        for (domain_mask, domain_name) in domains {
            for (search_path, search_path_name) in search_paths {
                let mut out = io::stdout().lock();

                let mut path_count = 0;
                writeln!(out, "{search_path_name} in {domain_name} domain:\n")?;
                enumerate(search_path, domain_mask, |bytes| {
                    path_count += 1;

                    out.write_all(b"- ")?;
                    out.write_all(bytes)?;
                    out.write_all(b"\n")?;
                    Ok(())
                })?;

                if path_count == 0 {
                    writeln!(out, "N/A")?;
                }

                if idx > 0 {
                    writeln!(out)?;
                    idx -= 1;
                }
            }
        }
        Ok(())
    }

    fn enumerate(
        dir: sysdir_search_path_directory_t,
        domain_mask: sysdir_search_path_domain_mask_t,
        mut thunk: impl FnMut(&[u8]) -> Result<(), Box<dyn Error>>,
    ) -> Result<(), Box<dyn Error>> {
        let mut path = [0; PATH_MAX as usize];

        unsafe {
            let mut state = sysdir_start_search_path_enumeration(dir, domain_mask);
            loop {
                let path = path.as_mut_ptr().cast::<c_char>();
                state = sysdir_get_next_search_path_enumeration(state, path);
                if state == 0 {
                    break;
                }
                let path = CStr::from_ptr(path);
                thunk(path.to_bytes())?;
            }
        }
        Ok(())
    }
}
