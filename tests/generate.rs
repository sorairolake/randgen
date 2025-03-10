// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

mod utils;

use predicates::prelude::predicate;

#[test]
fn basic() {
    let output = utils::command::command().arg("1KiB").output().unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout.len(), 1024);
}

#[test]
fn missing_length() {
    utils::command::command()
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the following required arguments were not provided",
        ))
        .stderr(predicate::str::contains("<BYTES>"));
}

#[test]
fn length_is_zero() {
    let output = utils::command::command().arg("0").output().unwrap();
    assert!(output.status.success());
    assert!(output.stdout.is_empty());
}

#[test]
fn validate_length_with_unit() {
    let output = utils::command::command().arg("256 B").output().unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout.len(), 256);
}

#[test]
fn validate_length_without_unit() {
    let output = utils::command::command().arg("256").output().unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout.len(), 256);
}

#[test]
fn validate_length_with_byte_prefix() {
    {
        let output = utils::command::command().arg("2048 KiB").output().unwrap();
        assert!(output.status.success());
        assert_eq!(output.stdout.len(), 2_097_152);
    }
    {
        let output = utils::command::command().arg("2.00 MiB").output().unwrap();
        assert!(output.status.success());
        assert_eq!(output.stdout.len(), 2_097_152);
    }
    {
        let output = utils::command::command().arg("2MiB").output().unwrap();
        assert!(output.status.success());
        assert_eq!(output.stdout.len(), 2_097_152);
    }
    {
        let output = utils::command::command().arg("4kB").output().unwrap();
        assert!(output.status.success());
        assert_eq!(output.stdout.len(), 4000);
    }
}

#[test]
fn validate_length_with_invalid_unit() {
    utils::command::command()
        .arg("2048 A")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("the character 'A' is incorrect"));
    utils::command::command()
        .arg("2.00LiB")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("the character 'L' is incorrect"));
}

#[test]
fn validate_length_with_nan() {
    utils::command::command()
        .arg("n B")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the character 'n' is not a number",
        ));
    utils::command::command()
        .arg("n")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the character 'n' is not a number",
        ));
    utils::command::command()
        .arg("nKiB")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the character 'n' is not a number",
        ));
}
