// Note: you'll have to `cargo build` before running these integration tests.
// TODO: Figure out how to get this to build before every invocation of `cargo test`
#[cfg(test)]
mod integration_test {
    use assert_cmd::Command;
    // use assert_fs::{prelude::*, NamedTempFile};

    // Incidentally, this does test the fact that --source and --target have to exist...for now
    #[test]
    fn test_requires_two_arguments() {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

        let assert = cmd.assert();

        assert.failure();
    }
}
