// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

mod utils;

use predicates::prelude::predicate;

#[test]
fn rng() {
    {
        let output = utils::command::command()
            .arg("-r")
            .arg("xoshiro256++")
            .arg("1KiB")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(output.stdout.len(), 1024);
    }
    {
        let output = utils::command::command()
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
        let output = utils::command::command()
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
            [0x73, 0x1c, 0xad, 0xec, 0x05, 0x5c, 0x66, 0x24]
        );
    }
    {
        let output = utils::command::command()
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
            [0x1e, 0x21, 0x9d, 0xec, 0xd1, 0x3d, 0x23, 0xdb]
        );
    }
    {
        let output = utils::command::command()
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
            [0xcf, 0x6e, 0xc4, 0x45, 0xd8, 0xa0, 0xa6, 0x88]
        );
    }
    #[cfg(feature = "hc")]
    {
        let output = utils::command::command()
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
            [0x41, 0xbf, 0x56, 0xac, 0x65, 0x23, 0x21, 0x97]
        );
    }
    #[cfg(feature = "isaac")]
    {
        let output = utils::command::command()
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
            [0x40, 0x82, 0x37, 0x31, 0x14, 0x92, 0xf4, 0x2b]
        );
    }
    #[cfg(feature = "isaac")]
    {
        let output = utils::command::command()
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
            [0x90, 0xb3, 0x17, 0xb0, 0x94, 0xbf, 0xf7, 0xbb]
        );
    }
    #[cfg(feature = "mt")]
    {
        let output = utils::command::command()
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
            [0xec, 0xcf, 0x0f, 0x03, 0x5b, 0x29, 0x42, 0x37]
        );
    }
    #[cfg(feature = "mt")]
    {
        let output = utils::command::command()
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
            [0xe0, 0x64, 0x75, 0x02, 0x9d, 0x4d, 0x2e, 0xb8]
        );
    }
    #[cfg(feature = "pcg")]
    {
        let output = utils::command::command()
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
            [0xd5, 0x8b, 0xca, 0x10, 0x9d, 0x34, 0x5c, 0x1f]
        );
    }
    #[cfg(feature = "pcg")]
    {
        let output = utils::command::command()
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
            [0x3c, 0xa3, 0xca, 0x32, 0x8e, 0x77, 0xf2, 0xe3]
        );
    }
    #[cfg(feature = "pcg")]
    {
        let output = utils::command::command()
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
            [0x5f, 0xf5, 0xef, 0x57, 0x74, 0x99, 0xf5, 0x9d]
        );
    }
    #[cfg(feature = "pcg")]
    {
        let output = utils::command::command()
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
            [0x39, 0x35, 0x4a, 0x40, 0x30, 0xc0, 0xc3, 0xcd]
        );
    }
    #[cfg(feature = "sfc")]
    {
        let output = utils::command::command()
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
            [0x4f, 0x55, 0xf5, 0x36, 0x95, 0x62, 0x21, 0xc8]
        );
    }
    #[cfg(feature = "sfc")]
    {
        let output = utils::command::command()
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
            [0x95, 0x2e, 0x5a, 0x22, 0xd5, 0x01, 0x96, 0x6c]
        );
    }
    {
        let output = utils::command::command()
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
            [0x07, 0x92, 0x77, 0xba, 0xdc, 0x86, 0xe1, 0x5d]
        );
    }
    #[cfg(feature = "xorshift")]
    {
        let output = utils::command::command()
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
            [0x26, 0x99, 0x59, 0x32, 0xbc, 0x45, 0xb1, 0x30]
        );
    }
    {
        let output = utils::command::command()
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
            [0x1d, 0xfa, 0xdd, 0x6a, 0x61, 0x7a, 0xa3, 0x34]
        );
    }
    {
        let output = utils::command::command()
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
            [0x61, 0x52, 0xbc, 0xca, 0xbe, 0x7c, 0x2c, 0xe6]
        );
    }
    {
        let output = utils::command::command()
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
            [0x52, 0xf6, 0xfc, 0x83, 0xa7, 0x98, 0x3b, 0x72]
        );
    }
    {
        let output = utils::command::command()
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
            [0x7e, 0x76, 0x1c, 0xa7, 0xd6, 0x8e, 0x30, 0x8f]
        );
    }
    {
        let output = utils::command::command()
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
            [0x3a, 0xa1, 0x55, 0x82, 0x63, 0x66, 0x5a, 0x52]
        );
    }
    {
        let output = utils::command::command()
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
            [0xd1, 0xa3, 0xd1, 0xce, 0x5e, 0xb7, 0x84, 0xd0]
        );
    }
    {
        let output = utils::command::command()
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
            [0xee, 0x7a, 0x49, 0x23, 0x79, 0xb4, 0x27, 0x36]
        );
    }
    {
        let output = utils::command::command()
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
            [0xba, 0x59, 0x5a, 0x52, 0x0b, 0xac, 0x64, 0xb5]
        );
    }
    {
        let output = utils::command::command()
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
            [0xe1, 0x5a, 0x99, 0x9e, 0x13, 0x88, 0x1e, 0xf5]
        );
    }
    {
        let output = utils::command::command()
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
            [0x4b, 0x21, 0x72, 0x2b, 0x8a, 0xd3, 0xb0, 0x67]
        );
    }
    {
        let output = utils::command::command()
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
            [0x42, 0x99, 0x50, 0x39, 0xb6, 0x52, 0x90, 0xea]
        );
    }
    {
        let output = utils::command::command()
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
            [0x7b, 0xd8, 0x5f, 0x5f, 0xef, 0xb3, 0x14, 0x37]
        );
    }
    {
        let output = utils::command::command()
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
            [0x9d, 0xb4, 0xde, 0x55, 0xd2, 0xeb, 0x11, 0x41]
        );
    }
    {
        let output = utils::command::command()
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
            [0x42, 0x99, 0x50, 0x39, 0xb6, 0x52, 0x90, 0xea]
        );
    }
}

#[test]
fn invalid_rng() {
    utils::command::command()
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
