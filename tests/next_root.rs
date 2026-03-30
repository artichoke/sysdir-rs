#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "watchos"
))]
fn assert_tests_pass_with_next_root(next_root: impl AsRef<std::ffi::OsStr>, context: &str) {
    use std::process::Command;

    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    for args in [["test", "--lib"], ["test", "--doc"]] {
        let output = Command::new(&cargo)
            .args(args)
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .env("NEXT_ROOT", next_root.as_ref())
            .output()
            .expect("should run cargo test in a subprocess");

        assert!(
            output.status.success(),
            "cargo {} failed with {context}\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
            args.join(" "),
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );
    }
}

#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "watchos"
))]
#[test]
fn lib_tests_and_doctests_pass_with_next_root_set() {
    assert_tests_pass_with_next_root("/tmp/sysdir-next-root-test", "NEXT_ROOT set");
}

#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "watchos"
))]
#[test]
fn lib_tests_and_doctests_pass_with_non_utf8_next_root_set() {
    use std::{ffi::OsString, os::unix::ffi::OsStringExt};

    let next_root = OsString::from_vec(b"/tmp/sysdir-next-root-\xff-test".to_vec());
    assert_tests_pass_with_next_root(next_root, "NEXT_ROOT set to non-UTF-8 bytes");
}
