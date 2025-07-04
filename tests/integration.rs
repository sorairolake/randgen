// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

mod utils;

use predicates::prelude::predicate;

#[test]
fn generate_completion() {
    utils::command::command()
        .arg("--generate-completion")
        .arg("bash")
        .assert()
        .success()
        .stdout(predicate::ne(""));
    utils::command::command()
        .arg("--generate-completion")
        .arg("elvish")
        .assert()
        .success()
        .stdout(predicate::ne(""));
    utils::command::command()
        .arg("--generate-completion")
        .arg("fish")
        .assert()
        .success()
        .stdout(predicate::ne(""));
    utils::command::command()
        .arg("--generate-completion")
        .arg("nushell")
        .assert()
        .success()
        .stdout(predicate::ne(""));
    utils::command::command()
        .arg("--generate-completion")
        .arg("powershell")
        .assert()
        .success()
        .stdout(predicate::ne(""));
    utils::command::command()
        .arg("--generate-completion")
        .arg("zsh")
        .assert()
        .success()
        .stdout(predicate::ne(""));
}

#[test]
fn generate_completion_with_invalid_shell() {
    utils::command::command()
        .arg("--generate-completion")
        .arg("a")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'a' for '--generate-completion <SHELL>'",
        ));
}
