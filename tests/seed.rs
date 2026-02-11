// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

mod utils;

use predicates::prelude::predicate;

use crate::utils::command;

#[test]
fn with_seed() {
    {
        let output = command::command()
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0x1E, 0x21, 0x9D, 0xEC, 0xD1, 0x3D, 0x23, 0xDB]
        );
    }
    {
        let output = command::command()
            .arg("--seed")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0x1E, 0x21, 0x9D, 0xEC, 0xD1, 0x3D, 0x23, 0xDB]
        );
    }
}

#[test]
fn validate_seed_ranges() {
    command::command()
        .arg("-s")
        .arg("18446744073709551616")
        .arg("8B")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "number too large to fit in target type",
        ));
}

#[test]
fn validate_seed_with_nan() {
    command::command()
        .arg("-s")
        .arg("n")
        .arg("8B")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("invalid digit found in string"));
}
