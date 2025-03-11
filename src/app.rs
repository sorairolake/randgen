// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::io::{self, BufWriter, Write};

use anyhow::Context;
use clap::Parser;

use crate::{
    cli::{Format, Opt},
    rng::Rng,
};

// 2 KiB.
const CHUNK_SIZE: usize = 1 << 11;

/// Runs the program and returns the result.
pub fn run() -> anyhow::Result<()> {
    let opt = Opt::parse();

    if let Some(shell) = opt.generate_completion {
        Opt::print_completion(shell);
        return Ok(());
    }

    let rng = opt.random_number_generator;
    let mut rng = if let Some(seed) = opt.seed {
        Rng::seed_from_u64(&rng, seed)
    } else {
        Rng::try_from_os_rng(&rng).context("could not create a new instance of the RNG")?
    };

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    let mut remaining = opt
        .length
        .expect("the number of bytes to generate should be provided")
        .try_into()?;

    let mut buf = [u8::default(); CHUNK_SIZE];

    match opt.format {
        #[cfg(feature = "base64")]
        format @ (Format::Base64 | Format::Base64Url) => {
            let engine = match format {
                Format::Base64 => base64::engine::general_purpose::STANDARD,
                Format::Base64Url => base64::engine::general_purpose::URL_SAFE,
                _ => unreachable!(),
            };
            let mut writer = base64::write::EncoderWriter::new(writer, &engine);
            while remaining > 0 {
                let chunk_size = CHUNK_SIZE.min(remaining);
                rng.fill_bytes(&mut buf[..chunk_size]);
                writer
                    .write_all(&buf[..chunk_size])
                    .context("could not write base64 encoded random bytes to standard output")?;
                remaining -= chunk_size;
            }
            let mut writer = writer.finish().context(
                "could not write remaining base64 encoded random bytes to standard output",
            )?;
            writer
                .flush()
                .context("could not flush base64 encoded random bytes to standard output")?;
        }
        format => {
            while remaining > 0 {
                let chunk_size = CHUNK_SIZE.min(remaining);
                rng.fill_bytes(&mut buf[..chunk_size]);
                match format {
                    Format::Raw => writer
                        .write_all(&buf[..chunk_size])
                        .context("could not write raw random bytes to standard output")?,
                    #[cfg(feature = "hex")]
                    Format::Hex => {
                        let s = hex::encode(&buf[..chunk_size]);
                        write!(writer, "{s}").context(
                            "could not write hex encoded random bytes to standard output",
                        )?;
                    }
                    #[cfg(feature = "base64")]
                    _ => unreachable!(),
                }
                remaining -= chunk_size;
            }
            writer
                .flush()
                .context("could not flush random bytes to standard output")?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_size() {
        assert_eq!(CHUNK_SIZE, 2048);
    }
}
