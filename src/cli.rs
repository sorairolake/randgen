// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::io::{self, Write};

use byte_unit::Byte;
use clap::{CommandFactory, Parser, ValueEnum};
use clap_complete::Generator;

const LONG_VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    '\n',
    "Copyright (C) 2025 Shun Sakai\n",
    '\n',
    "This program is distributed under the terms of either the Apache License 2.0 or\n",
    "the MIT License.\n",
    '\n',
    "This is free software: you are free to change and redistribute it. There is NO\n",
    "WARRANTY, to the extent permitted by law.\n",
    '\n',
    "Report bugs to <https://github.com/sorairolake/randgen/issues>."
);

const AFTER_LONG_HELP: &str = concat!(
    "The generated bytes will be output to standard output.\n",
    '\n',
    "See `randgen(1)` for more details."
);

#[derive(Debug, Parser)]
#[command(
    version,
    long_version(LONG_VERSION),
    about,
    max_term_width(100),
    after_long_help(AFTER_LONG_HELP)
)]
pub struct Opt {
    /// Output in the specified output format.
    #[arg(
        short,
        long,
        value_enum,
        default_value_t,
        value_name("FORMAT"),
        ignore_case(true)
    )]
    pub format: Format,

    /// Random number generator to use.
    #[arg(
        short,
        long,
        value_enum,
        default_value_t,
        value_name("RNG"),
        ignore_case(true)
    )]
    pub random_number_generator: Rng,

    /// Random seed to use.
    ///
    /// If this option is not specified, the RNG seeded via random data from
    /// system sources such as the `getrandom` system call on Linux.
    #[arg(short, long, value_name("NUMBER"))]
    pub seed: Option<u64>,

    /// Print information showing the progress of the generation of random
    /// bytes.
    #[arg(short, long)]
    pub progress: bool,

    /// Generate shell completion.
    ///
    /// The completion is output to standard output.
    #[arg(long, value_enum, value_name("SHELL"))]
    pub generate_completion: Option<Shell>,

    /// Number of bytes to generate.
    ///
    /// [BYTES] can be suffixed with the symbol (B) and the byte prefix (such as
    /// Ki and M). If only a numeric value is specified for [BYTES], it is the
    /// same as specifying the symbol without the byte prefix.
    #[arg(value_name("BYTES"), required_unless_present("generate_completion"))]
    pub length: Option<Byte>,
}

impl Opt {
    /// Generates shell completion and print it.
    pub fn print_completion(generator: impl Generator) {
        clap_complete::generate(
            generator,
            &mut Self::command(),
            Self::command().get_name(),
            &mut io::stdout(),
        );
    }
}

#[derive(Clone, Debug, ValueEnum)]
#[allow(clippy::doc_markdown)]
#[value(rename_all = "lower")]
pub enum Shell {
    /// Bash.
    Bash,

    /// Elvish.
    Elvish,

    /// fish.
    Fish,

    /// Nushell.
    Nushell,

    #[allow(clippy::enum_variant_names)]
    /// PowerShell.
    PowerShell,

    /// Zsh.
    Zsh,
}

impl Generator for Shell {
    fn file_name(&self, name: &str) -> String {
        match self {
            Self::Bash => clap_complete::Shell::Bash.file_name(name),
            Self::Elvish => clap_complete::Shell::Elvish.file_name(name),
            Self::Fish => clap_complete::Shell::Fish.file_name(name),
            Self::Nushell => clap_complete_nushell::Nushell.file_name(name),
            Self::PowerShell => clap_complete::Shell::PowerShell.file_name(name),
            Self::Zsh => clap_complete::Shell::Zsh.file_name(name),
        }
    }

    fn generate(&self, cmd: &clap::Command, buf: &mut dyn Write) {
        match self {
            Self::Bash => clap_complete::Shell::Bash.generate(cmd, buf),
            Self::Elvish => clap_complete::Shell::Elvish.generate(cmd, buf),
            Self::Fish => clap_complete::Shell::Fish.generate(cmd, buf),
            Self::Nushell => clap_complete_nushell::Nushell.generate(cmd, buf),
            Self::PowerShell => clap_complete::Shell::PowerShell.generate(cmd, buf),
            Self::Zsh => clap_complete::Shell::Zsh.generate(cmd, buf),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, ValueEnum)]
#[value(rename_all = "lower")]
pub enum Format {
    /// Encode the generated bytes as raw bytes.
    #[default]
    Raw,

    /// Encode the generated bytes as base64.
    #[cfg(feature = "base64")]
    Base64,

    /// Encode the generated bytes as URL-safe base64.
    #[cfg(feature = "base64")]
    Base64Url,

    /// Encode the generated bytes as hex string.
    #[cfg(feature = "hex")]
    Hex,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, ValueEnum)]
#[allow(clippy::doc_markdown)]
#[value(rename_all = "lower")]
pub enum Rng {
    /// A CSPRNG that uses the ChaCha8 algorithm.
    ChaCha8,

    /// A CSPRNG that uses the ChaCha12 algorithm.
    #[default]
    ChaCha12,

    /// A CSPRNG that uses the ChaCha20 algorithm.
    ChaCha20,

    /// A CSPRNG that uses the HC-128 algorithm.
    #[cfg(feature = "hc")]
    Hc128,

    /// A PRNG that uses the ISAAC algorithm.
    #[cfg(feature = "isaac")]
    Isaac,

    /// A PRNG that uses the ISAAC-64 algorithm.
    #[cfg(feature = "isaac")]
    Isaac64,

    /// The MT19937 PRNG.
    #[cfg(feature = "mt")]
    Mt,

    /// The MT19937-64 PRNG.
    #[cfg(feature = "mt")]
    Mt64,

    /// The pcg32 PRNG.
    #[cfg(feature = "pcg")]
    Pcg32,

    /// The pcg64 PRNG.
    #[cfg(feature = "pcg")]
    Pcg64,

    /// The PCG64DXSM PRNG.
    #[cfg(feature = "pcg")]
    Pcg64Dxsm,

    /// The pcg64_fast PRNG.
    #[cfg(feature = "pcg")]
    Pcg64Mcg,

    /// The sfc32 PRNG.
    #[cfg(feature = "sfc")]
    Sfc32,

    /// The sfc64 PRNG.
    #[cfg(feature = "sfc")]
    Sfc64,

    /// The splitmix64 PRNG.
    SplitMix64,

    /// The Xorshift PRNG.
    #[cfg(feature = "xorshift")]
    XorShift,

    /// The xoroshiro64* PRNG.
    #[value(name = "xoroshiro64*")]
    Xoroshiro64Star,

    /// The xoroshiro64** PRNG.
    #[value(name = "xoroshiro64**")]
    Xoroshiro64StarStar,

    /// The xoroshiro128+ PRNG.
    #[value(name = "xoroshiro128+")]
    Xoroshiro128Plus,

    /// The xoroshiro128++ PRNG.
    #[value(name = "xoroshiro128++")]
    Xoroshiro128PlusPlus,

    /// The xoroshiro128** PRNG.
    #[value(name = "xoroshiro128**")]
    Xoroshiro128StarStar,

    /// The xoshiro128+ PRNG.
    #[value(name = "xoshiro128+")]
    Xoshiro128Plus,

    /// The xoshiro128++ PRNG.
    #[value(name = "xoshiro128++")]
    Xoshiro128PlusPlus,

    /// The xoshiro128** PRNG.
    #[value(name = "xoshiro128**")]
    Xoshiro128StarStar,

    /// The xoshiro256+ PRNG.
    #[value(name = "xoshiro256+")]
    Xoshiro256Plus,

    /// The xoshiro256++ PRNG.
    #[value(name = "xoshiro256++")]
    Xoshiro256PlusPlus,

    /// The xoshiro256** PRNG.
    #[value(name = "xoshiro256**")]
    Xoshiro256StarStar,

    /// The xoshiro512+ PRNG.
    #[value(name = "xoshiro512+")]
    Xoshiro512Plus,

    /// The xoshiro512++ PRNG.
    #[value(name = "xoshiro512++")]
    Xoshiro512PlusPlus,

    /// The xoshiro512** PRNG.
    #[value(name = "xoshiro512**")]
    Xoshiro512StarStar,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_app() {
        Opt::command().debug_assert();
    }

    #[test]
    fn file_name_shell() {
        assert_eq!(Shell::Bash.file_name("randgen"), "randgen.bash");
        assert_eq!(Shell::Elvish.file_name("randgen"), "randgen.elv");
        assert_eq!(Shell::Fish.file_name("randgen"), "randgen.fish");
        assert_eq!(Shell::Nushell.file_name("randgen"), "randgen.nu");
        assert_eq!(Shell::PowerShell.file_name("randgen"), "_randgen.ps1");
        assert_eq!(Shell::Zsh.file_name("randgen"), "_randgen");
    }

    #[test]
    fn default_format() {
        assert_eq!(Format::default(), Format::Raw);
    }

    #[test]
    fn default_rng() {
        assert_eq!(Rng::default(), Rng::ChaCha12);
    }
}
