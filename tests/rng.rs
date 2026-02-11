// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

mod utils;

use predicates::prelude::predicate;

use crate::utils::command;

#[test]
fn rng() {
    {
        let output = command::command()
            .arg("-r")
            .arg("xoshiro256++")
            .arg("1KiB")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(output.stdout.len(), 1024);
    }
    {
        let output = command::command()
            .arg("--random-number-generator")
            .arg("xoshiro256++")
            .arg("1KiB")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(output.stdout.len(), 1024);
    }
}

#[test]
fn rng_with_seed() {
    {
        let output = command::command()
            .arg("-r")
            .arg("chacha8")
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0x73, 0x1C, 0xAD, 0xEC, 0x05, 0x5C, 0x66, 0x24]
        );
    }
    {
        let output = command::command()
            .arg("-r")
            .arg("chacha12")
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
            .arg("-r")
            .arg("chacha20")
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0xCF, 0x6E, 0xC4, 0x45, 0xD8, 0xA0, 0xA6, 0x88]
        );
    }
    #[cfg(feature = "hc")]
    {
        let output = command::command()
            .arg("-r")
            .arg("hc128")
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0x41, 0xBF, 0x56, 0xAC, 0x65, 0x23, 0x21, 0x97]
        );
    }
    #[cfg(feature = "isaac")]
    {
        let output = command::command()
            .arg("-r")
            .arg("isaac")
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0x40, 0x82, 0x37, 0x31, 0x14, 0x92, 0xF4, 0x2B]
        );
    }
    #[cfg(feature = "isaac")]
    {
        let output = command::command()
            .arg("-r")
            .arg("isaac64")
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0x90, 0xB3, 0x17, 0xB0, 0x94, 0xBF, 0xF7, 0xBB]
        );
    }
    #[cfg(feature = "mt")]
    {
        let output = command::command()
            .arg("-r")
            .arg("mt")
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0xEC, 0xCF, 0x0F, 0x03, 0x5B, 0x29, 0x42, 0x37]
        );
    }
    #[cfg(feature = "mt")]
    {
        let output = command::command()
            .arg("-r")
            .arg("mt64")
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0xE0, 0x64, 0x75, 0x02, 0x9D, 0x4D, 0x2E, 0xB8]
        );
    }
    #[cfg(feature = "pcg")]
    {
        let output = command::command()
            .arg("-r")
            .arg("pcg32")
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0xD5, 0x8B, 0xCA, 0x10, 0x9D, 0x34, 0x5C, 0x1F]
        );
    }
    #[cfg(feature = "pcg")]
    {
        let output = command::command()
            .arg("-r")
            .arg("pcg64")
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0x3C, 0xA3, 0xCA, 0x32, 0x8E, 0x77, 0xF2, 0xE3]
        );
    }
    #[cfg(feature = "pcg")]
    {
        let output = command::command()
            .arg("-r")
            .arg("pcg64dxsm")
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0x5F, 0xF5, 0xEF, 0x57, 0x74, 0x99, 0xF5, 0x9D]
        );
    }
    #[cfg(feature = "pcg")]
    {
        let output = command::command()
            .arg("-r")
            .arg("pcg64mcg")
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0x39, 0x35, 0x4A, 0x40, 0x30, 0xC0, 0xC3, 0xCD]
        );
    }
    #[cfg(feature = "sfc")]
    {
        let output = command::command()
            .arg("-r")
            .arg("sfc32")
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0x97, 0xE2, 0xA7, 0x5F, 0xF2, 0x8F, 0xEA, 0x39]
        );
    }
    #[cfg(feature = "sfc")]
    {
        let output = command::command()
            .arg("-r")
            .arg("sfc64")
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0xA9, 0x27, 0x03, 0xE6, 0xCD, 0x01, 0x03, 0xF3]
        );
    }
    {
        let output = command::command()
            .arg("-r")
            .arg("splitmix64")
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0x07, 0x92, 0x77, 0xBA, 0xDC, 0x86, 0xE1, 0x5D]
        );
    }
    #[cfg(feature = "xorshift")]
    {
        let output = command::command()
            .arg("-r")
            .arg("xorshift")
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0x26, 0x99, 0x59, 0x32, 0xBC, 0x45, 0xB1, 0x30]
        );
    }
    {
        let output = command::command()
            .arg("-r")
            .arg("xoroshiro64*")
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0x1D, 0xFA, 0xDD, 0x6A, 0x61, 0x7A, 0xA3, 0x34]
        );
    }
    {
        let output = command::command()
            .arg("-r")
            .arg("xoroshiro64**")
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0x61, 0x52, 0xBC, 0xCA, 0xBE, 0x7C, 0x2C, 0xE6]
        );
    }
    {
        let output = command::command()
            .arg("-r")
            .arg("xoroshiro128+")
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0x52, 0xF6, 0xFC, 0x83, 0xA7, 0x98, 0x3B, 0x72]
        );
    }
    {
        let output = command::command()
            .arg("-r")
            .arg("xoroshiro128++")
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0x7E, 0x76, 0x1C, 0xA7, 0xD6, 0x8E, 0x30, 0x8F]
        );
    }
    {
        let output = command::command()
            .arg("-r")
            .arg("xoroshiro128**")
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0x3A, 0xA1, 0x55, 0x82, 0x63, 0x66, 0x5A, 0x52]
        );
    }
    {
        let output = command::command()
            .arg("-r")
            .arg("xoshiro128+")
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0xD1, 0xA3, 0xD1, 0xCE, 0x5E, 0xB7, 0x84, 0xD0]
        );
    }
    {
        let output = command::command()
            .arg("-r")
            .arg("xoshiro128++")
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0xEE, 0x7A, 0x49, 0x23, 0x79, 0xB4, 0x27, 0x36]
        );
    }
    {
        let output = command::command()
            .arg("-r")
            .arg("xoshiro128**")
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0xBA, 0x59, 0x5A, 0x52, 0x0B, 0xAC, 0x64, 0xB5]
        );
    }
    {
        let output = command::command()
            .arg("-r")
            .arg("xoshiro256+")
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0xE1, 0x5A, 0x99, 0x9E, 0x13, 0x88, 0x1E, 0xF5]
        );
    }
    {
        let output = command::command()
            .arg("-r")
            .arg("xoshiro256++")
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0x4B, 0x21, 0x72, 0x2B, 0x8A, 0xD3, 0xB0, 0x67]
        );
    }
    {
        let output = command::command()
            .arg("-r")
            .arg("xoshiro256**")
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0x42, 0x99, 0x50, 0x39, 0xB6, 0x52, 0x90, 0xEA]
        );
    }
    {
        let output = command::command()
            .arg("-r")
            .arg("xoshiro512+")
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0x7B, 0xD8, 0x5F, 0x5F, 0xEF, 0xB3, 0x14, 0x37]
        );
    }
    {
        let output = command::command()
            .arg("-r")
            .arg("xoshiro512++")
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0x9D, 0xB4, 0xDE, 0x55, 0xD2, 0xEB, 0x11, 0x41]
        );
    }
    {
        let output = command::command()
            .arg("-r")
            .arg("xoshiro512**")
            .arg("-s")
            .arg("16")
            .arg("8B")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            [0x42, 0x99, 0x50, 0x39, 0xB6, 0x52, 0x90, 0xEA]
        );
    }
}

#[test]
fn invalid_rng() {
    command::command()
        .arg("-r")
        .arg("a")
        .arg("8B")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'a' for '--random-number-generator <RNG>'",
        ));
}
