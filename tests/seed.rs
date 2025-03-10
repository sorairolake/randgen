// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

mod utils;

use predicates::prelude::predicate;

#[test]
fn with_seed() {
    {
        let output = utils::command::command()
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0x1e, 0x21, 0x9d, 0xec, 0xd1, 0x3d, 0x23, 0xdb]
        );
    }
    {
        let output = utils::command::command()
            .arg("--seed")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0x1e, 0x21, 0x9d, 0xec, 0xd1, 0x3d, 0x23, 0xdb]
        );
    }
}

#[test]
fn validate_seed_ranges() {
    utils::command::command()
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
    utils::command::command()
        .arg("-s")
        .arg("n")
        .arg("8B")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("invalid digit found in string"));
}
