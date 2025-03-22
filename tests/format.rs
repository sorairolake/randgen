// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

mod utils;

use predicates::prelude::predicate;

#[test]
fn format() {
    {
        let output = utils::command::command()
            .arg("-f")
            .arg("raw")
            .arg("1KiB")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(output.stdout.len(), 1024);
    }
    {
        let output = utils::command::command()
            .arg("--format")
            .arg("raw")
            .arg("1KiB")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(output.stdout.len(), 1024);
    }
}

#[test]
fn raw() {
    let output = utils::command::command()
        .arg("-f")
        .arg("raw")
        .arg("-s")
        .arg("256")
        .arg("32B")
        .output()
        .unwrap();
    assert!(output.status.success());
    let expected = [
        0xd1, 0x7f, 0x36, 0xca, 0x35, 0x54, 0xd7, 0xf2, 0x4b, 0x74, 0x35, 0x34, 0xef, 0x2c, 0x09,
        0xff, 0xb0, 0x80, 0x24, 0x1c, 0x11, 0x4c, 0x45, 0x52, 0xfe, 0x2d, 0x7e, 0x91, 0x18, 0x11,
        0xe8, 0x74,
    ];
    assert_eq!(output.stdout, expected);
}

#[test]
fn raw_with_long_output() {
    let output = utils::command::command()
        .arg("-f")
        .arg("raw")
        .arg("-s")
        .arg("256")
        .arg("64B")
        .output()
        .unwrap();
    assert!(output.status.success());
    let expected = [
        0xd1, 0x7f, 0x36, 0xca, 0x35, 0x54, 0xd7, 0xf2, 0x4b, 0x74, 0x35, 0x34, 0xef, 0x2c, 0x09,
        0xff, 0xb0, 0x80, 0x24, 0x1c, 0x11, 0x4c, 0x45, 0x52, 0xfe, 0x2d, 0x7e, 0x91, 0x18, 0x11,
        0xe8, 0x74, 0x2c, 0x6f, 0xd8, 0xc5, 0xc9, 0x03, 0x6e, 0x88, 0x52, 0x01, 0x9a, 0xb4, 0xee,
        0x72, 0x9c, 0x2d, 0xf8, 0x26, 0x63, 0x66, 0x3f, 0x74, 0x25, 0x88, 0xc4, 0xe8, 0x3d, 0x9e,
        0xac, 0x91, 0x27, 0xe9,
    ];
    assert_eq!(output.stdout, expected);
}

#[cfg(feature = "base64")]
#[test]
fn base64() {
    let output = utils::command::command()
        .arg("-f")
        .arg("base64")
        .arg("-s")
        .arg("256")
        .arg("32B")
        .output()
        .unwrap();
    assert!(output.status.success());
    let expected = "0X82yjVU1/JLdDU07ywJ/7CAJBwRTEVS/i1+kRgR6HQ=";
    assert_eq!(output.stdout, expected.as_bytes());
}

#[cfg(feature = "base64")]
#[test]
fn base64_check_no_line_wrapping() {
    let output = utils::command::command()
        .arg("-f")
        .arg("base64")
        .arg("-s")
        .arg("256")
        .arg("64B")
        .output()
        .unwrap();
    assert!(output.status.success());
    // More than 76 columns.
    let expected =
        "0X82yjVU1/JLdDU07ywJ/7CAJBwRTEVS/i1+kRgR6HQsb9jFyQNuiFIBmrTucpwt+CZjZj90JYjE6D2erJEn6Q==";
    assert_eq!(output.stdout, expected.as_bytes());
}

#[cfg(feature = "base64")]
#[test]
fn base64url() {
    let output = utils::command::command()
        .arg("-f")
        .arg("base64url")
        .arg("-s")
        .arg("256")
        .arg("32B")
        .output()
        .unwrap();
    assert!(output.status.success());
    let expected = "0X82yjVU1_JLdDU07ywJ_7CAJBwRTEVS_i1-kRgR6HQ=";
    assert_eq!(output.stdout, expected.as_bytes());
}

#[cfg(feature = "base64")]
#[test]
fn base64url_check_no_line_wrapping() {
    let output = utils::command::command()
        .arg("-f")
        .arg("base64url")
        .arg("-s")
        .arg("256")
        .arg("64B")
        .output()
        .unwrap();
    assert!(output.status.success());
    // More than 76 columns.
    let expected =
        "0X82yjVU1_JLdDU07ywJ_7CAJBwRTEVS_i1-kRgR6HQsb9jFyQNuiFIBmrTucpwt-CZjZj90JYjE6D2erJEn6Q==";
    assert_eq!(output.stdout, expected.as_bytes());
}

#[cfg(feature = "hex")]
#[test]
fn hex() {
    let output = utils::command::command()
        .arg("-f")
        .arg("hex")
        .arg("-s")
        .arg("256")
        .arg("32B")
        .output()
        .unwrap();
    assert!(output.status.success());
    let expected = "d17f36ca3554d7f24b743534ef2c09ffb080241c114c4552fe2d7e911811e874";
    assert_eq!(output.stdout, expected.as_bytes());
}

#[cfg(feature = "hex")]
#[test]
fn hex_check_no_line_wrapping() {
    let output = utils::command::command()
        .arg("-f")
        .arg("hex")
        .arg("-s")
        .arg("256")
        .arg("64B")
        .output()
        .unwrap();
    assert!(output.status.success());
    // More than 60 columns.
    let expected = "d17f36ca3554d7f24b743534ef2c09ffb080241c114c4552fe2d7e911811e8742c6fd8c5c9036e\
                    8852019ab4ee729c2df82663663f742588c4e83d9eac9127e9";
    assert_eq!(output.stdout, expected.as_bytes());
}

#[test]
fn invalid_format() {
    utils::command::command()
        .arg("-f")
        .arg("a")
        .arg("32B")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'a' for '--format <FORMAT>'",
        ));
}
