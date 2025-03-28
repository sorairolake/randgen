// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

mod app;
mod cli;
mod rng;

use std::{io, process::ExitCode};

fn main() -> ExitCode {
    sigpipe::reset();

    match app::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err:?}");
            if let Some(e) = err.downcast_ref::<io::Error>() {
                return sysexits::ExitCode::from(e.kind()).into();
            }
            if let Some(e) = err.downcast_ref::<getrandom::Error>() {
                return sysexits::ExitCode::from(io::Error::from(*e)).into();
            }
            ExitCode::FAILURE
        }
    }
}
