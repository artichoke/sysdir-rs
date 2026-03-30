#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "watchos"
))]
#[test]
fn lib_tests_and_doctests_pass_with_next_root_set() {
    use std::process::Command;

    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    for args in [["test", "--lib"], ["test", "--doc"]] {
        let output = Command::new(&cargo)
            .args(args)
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .env("NEXT_ROOT", "/tmp/sysdir-next-root-test")
            .output()
            .expect("should run cargo test in a subprocess");

        assert!(
            output.status.success(),
            "cargo {} failed with NEXT_ROOT set\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
            args.join(" "),
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );
    }
}
