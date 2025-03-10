// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use getrandom::Error;
use rand_chacha::{ChaCha8Rng, ChaCha12Rng, ChaCha20Rng};
use rand_core::{RngCore, SeedableRng};
use rand_xoshiro::{
    SplitMix64, Xoroshiro64Star, Xoroshiro64StarStar, Xoroshiro128Plus, Xoroshiro128PlusPlus,
    Xoroshiro128StarStar, Xoshiro128Plus, Xoshiro128PlusPlus, Xoshiro128StarStar, Xoshiro256Plus,
    Xoshiro256PlusPlus, Xoshiro256StarStar, Xoshiro512Plus, Xoshiro512PlusPlus, Xoshiro512StarStar,
};

use crate::cli;

#[derive(Clone, Debug)]
pub enum Rng {
    ChaCha8(ChaCha8Rng),
    ChaCha12(ChaCha12Rng),
    ChaCha20(ChaCha20Rng),
    #[cfg(feature = "hc")]
    Hc128(rand_hc::Hc128Rng),
    #[cfg(feature = "isaac")]
    Isaac(rand_isaac::IsaacRng),
    #[cfg(feature = "isaac")]
    Isaac64(rand_isaac::Isaac64Rng),
    #[cfg(feature = "mt")]
    Mt(rand_mt::Mt),
    #[cfg(feature = "mt")]
    Mt64(rand_mt::Mt64),
    #[cfg(feature = "pcg")]
    Pcg32(rand_pcg::Pcg32),
    #[cfg(feature = "pcg")]
    Pcg64(rand_pcg::Pcg64),
    #[cfg(feature = "pcg")]
    Pcg64Dxsm(rand_pcg::Pcg64Dxsm),
    #[cfg(feature = "pcg")]
    Pcg64Mcg(rand_pcg::Pcg64Mcg),
    #[cfg(feature = "sfc")]
    Sfc32(rand_sfc::Sfc32),
    #[cfg(feature = "sfc")]
    Sfc64(rand_sfc::Sfc64),
    SplitMix64(SplitMix64),
    #[cfg(feature = "xorshift")]
    XorShift(rand_xorshift::XorShiftRng),
    Xoroshiro64Star(Xoroshiro64Star),
    Xoroshiro64StarStar(Xoroshiro64StarStar),
    Xoroshiro128Plus(Xoroshiro128Plus),
    Xoroshiro128PlusPlus(Xoroshiro128PlusPlus),
    Xoroshiro128StarStar(Xoroshiro128StarStar),
    Xoshiro128Plus(Xoshiro128Plus),
    Xoshiro128PlusPlus(Xoshiro128PlusPlus),
    Xoshiro128StarStar(Xoshiro128StarStar),
    Xoshiro256Plus(Xoshiro256Plus),
    Xoshiro256PlusPlus(Xoshiro256PlusPlus),
    Xoshiro256StarStar(Xoshiro256StarStar),
    Xoshiro512Plus(Xoshiro512Plus),
    Xoshiro512PlusPlus(Xoshiro512PlusPlus),
    Xoshiro512StarStar(Xoshiro512StarStar),
}

impl Rng {
    pub fn fill_bytes(&mut self, dst: &mut [u8]) {
        match self {
            Self::ChaCha8(rng) => rng.fill_bytes(dst),
            Self::ChaCha12(rng) => rng.fill_bytes(dst),
            Self::ChaCha20(rng) => rng.fill_bytes(dst),
            #[cfg(feature = "hc")]
            Self::Hc128(rng) => rng.fill_bytes(dst),
            #[cfg(feature = "isaac")]
            Self::Isaac(rng) => rng.fill_bytes(dst),
            #[cfg(feature = "isaac")]
            Self::Isaac64(rng) => rng.fill_bytes(dst),
            #[cfg(feature = "mt")]
            Self::Mt(rng) => rng.fill_bytes(dst),
            #[cfg(feature = "mt")]
            Self::Mt64(rng) => rng.fill_bytes(dst),
            #[cfg(feature = "pcg")]
            Self::Pcg32(rng) => rng.fill_bytes(dst),
            #[cfg(feature = "pcg")]
            Self::Pcg64(rng) => rng.fill_bytes(dst),
            #[cfg(feature = "pcg")]
            Self::Pcg64Dxsm(rng) => rng.fill_bytes(dst),
            #[cfg(feature = "pcg")]
            Self::Pcg64Mcg(rng) => rng.fill_bytes(dst),
            #[cfg(feature = "sfc")]
            Self::Sfc32(rng) => rng.fill_bytes(dst),
            #[cfg(feature = "sfc")]
            Self::Sfc64(rng) => rng.fill_bytes(dst),
            Self::SplitMix64(rng) => rng.fill_bytes(dst),
            #[cfg(feature = "xorshift")]
            Self::XorShift(rng) => rng.fill_bytes(dst),
            Self::Xoroshiro64Star(rng) => rng.fill_bytes(dst),
            Self::Xoroshiro64StarStar(rng) => rng.fill_bytes(dst),
            Self::Xoroshiro128Plus(rng) => rng.fill_bytes(dst),
            Self::Xoroshiro128PlusPlus(rng) => rng.fill_bytes(dst),
            Self::Xoroshiro128StarStar(rng) => rng.fill_bytes(dst),
            Self::Xoshiro128Plus(rng) => rng.fill_bytes(dst),
            Self::Xoshiro128PlusPlus(rng) => rng.fill_bytes(dst),
            Self::Xoshiro128StarStar(rng) => rng.fill_bytes(dst),
            Self::Xoshiro256Plus(rng) => rng.fill_bytes(dst),
            Self::Xoshiro256PlusPlus(rng) => rng.fill_bytes(dst),
            Self::Xoshiro256StarStar(rng) => rng.fill_bytes(dst),
            Self::Xoshiro512Plus(rng) => rng.fill_bytes(dst),
            Self::Xoshiro512PlusPlus(rng) => rng.fill_bytes(dst),
            Self::Xoshiro512StarStar(rng) => rng.fill_bytes(dst),
        }
    }

    pub fn seed_from_u64(rng: &cli::Rng, state: u64) -> Self {
        match rng {
            cli::Rng::ChaCha8 => Self::ChaCha8(ChaCha8Rng::seed_from_u64(state)),
            cli::Rng::ChaCha12 => Self::ChaCha12(ChaCha12Rng::seed_from_u64(state)),
            cli::Rng::ChaCha20 => Self::ChaCha20(ChaCha20Rng::seed_from_u64(state)),
            #[cfg(feature = "hc")]
            cli::Rng::Hc128 => Self::Hc128(rand_hc::Hc128Rng::seed_from_u64(state)),
            #[cfg(feature = "isaac")]
            cli::Rng::Isaac => Self::Isaac(rand_isaac::IsaacRng::seed_from_u64(state)),
            #[cfg(feature = "isaac")]
            cli::Rng::Isaac64 => Self::Isaac64(rand_isaac::Isaac64Rng::seed_from_u64(state)),
            #[cfg(feature = "mt")]
            cli::Rng::Mt => Self::Mt(rand_mt::Mt::seed_from_u64(state)),
            #[cfg(feature = "mt")]
            cli::Rng::Mt64 => Self::Mt64(rand_mt::Mt64::seed_from_u64(state)),
            #[cfg(feature = "pcg")]
            cli::Rng::Pcg32 => Self::Pcg32(rand_pcg::Pcg32::seed_from_u64(state)),
            #[cfg(feature = "pcg")]
            cli::Rng::Pcg64 => Self::Pcg64(rand_pcg::Pcg64::seed_from_u64(state)),
            #[cfg(feature = "pcg")]
            cli::Rng::Pcg64Dxsm => Self::Pcg64Dxsm(rand_pcg::Pcg64Dxsm::seed_from_u64(state)),
            #[cfg(feature = "pcg")]
            cli::Rng::Pcg64Mcg => Self::Pcg64Mcg(rand_pcg::Pcg64Mcg::seed_from_u64(state)),
            #[cfg(feature = "sfc")]
            cli::Rng::Sfc32 => Self::Sfc32(rand_sfc::Sfc32::seed_from_u64(state)),
            #[cfg(feature = "sfc")]
            cli::Rng::Sfc64 => Self::Sfc64(rand_sfc::Sfc64::seed_from_u64(state)),
            cli::Rng::SplitMix64 => Self::SplitMix64(SplitMix64::seed_from_u64(state)),
            #[cfg(feature = "xorshift")]
            cli::Rng::XorShift => Self::XorShift(rand_xorshift::XorShiftRng::seed_from_u64(state)),
            cli::Rng::Xoroshiro64Star => {
                Self::Xoroshiro64Star(Xoroshiro64Star::seed_from_u64(state))
            }
            cli::Rng::Xoroshiro64StarStar => {
                Self::Xoroshiro64StarStar(Xoroshiro64StarStar::seed_from_u64(state))
            }
            cli::Rng::Xoroshiro128Plus => {
                Self::Xoroshiro128Plus(Xoroshiro128Plus::seed_from_u64(state))
            }
            cli::Rng::Xoroshiro128PlusPlus => {
                Self::Xoroshiro128PlusPlus(Xoroshiro128PlusPlus::seed_from_u64(state))
            }
            cli::Rng::Xoroshiro128StarStar => {
                Self::Xoroshiro128StarStar(Xoroshiro128StarStar::seed_from_u64(state))
            }
            cli::Rng::Xoshiro128Plus => Self::Xoshiro128Plus(Xoshiro128Plus::seed_from_u64(state)),
            cli::Rng::Xoshiro128PlusPlus => {
                Self::Xoshiro128PlusPlus(Xoshiro128PlusPlus::seed_from_u64(state))
            }
            cli::Rng::Xoshiro128StarStar => {
                Self::Xoshiro128StarStar(Xoshiro128StarStar::seed_from_u64(state))
            }
            cli::Rng::Xoshiro256Plus => Self::Xoshiro256Plus(Xoshiro256Plus::seed_from_u64(state)),
            cli::Rng::Xoshiro256PlusPlus => {
                Self::Xoshiro256PlusPlus(Xoshiro256PlusPlus::seed_from_u64(state))
            }
            cli::Rng::Xoshiro256StarStar => {
                Self::Xoshiro256StarStar(Xoshiro256StarStar::seed_from_u64(state))
            }
            cli::Rng::Xoshiro512Plus => Self::Xoshiro512Plus(Xoshiro512Plus::seed_from_u64(state)),
            cli::Rng::Xoshiro512PlusPlus => {
                Self::Xoshiro512PlusPlus(Xoshiro512PlusPlus::seed_from_u64(state))
            }
            cli::Rng::Xoshiro512StarStar => {
                Self::Xoshiro512StarStar(Xoshiro512StarStar::seed_from_u64(state))
            }
        }
    }

    pub fn try_from_os_rng(rng: &cli::Rng) -> Result<Self, Error> {
        match rng {
            cli::Rng::ChaCha8 => Ok(Self::ChaCha8(ChaCha8Rng::try_from_os_rng()?)),
            cli::Rng::ChaCha12 => Ok(Self::ChaCha12(ChaCha12Rng::try_from_os_rng()?)),
            cli::Rng::ChaCha20 => Ok(Self::ChaCha20(ChaCha20Rng::try_from_os_rng()?)),
            #[cfg(feature = "hc")]
            cli::Rng::Hc128 => Ok(Self::Hc128(rand_hc::Hc128Rng::try_from_os_rng()?)),
            #[cfg(feature = "isaac")]
            cli::Rng::Isaac => Ok(Self::Isaac(rand_isaac::IsaacRng::try_from_os_rng()?)),
            #[cfg(feature = "isaac")]
            cli::Rng::Isaac64 => Ok(Self::Isaac64(rand_isaac::Isaac64Rng::try_from_os_rng()?)),
            #[cfg(feature = "mt")]
            cli::Rng::Mt => Ok(Self::Mt(rand_mt::Mt::try_from_os_rng()?)),
            #[cfg(feature = "mt")]
            cli::Rng::Mt64 => Ok(Self::Mt64(rand_mt::Mt64::try_from_os_rng()?)),
            #[cfg(feature = "pcg")]
            cli::Rng::Pcg32 => Ok(Self::Pcg32(rand_pcg::Pcg32::try_from_os_rng()?)),
            #[cfg(feature = "pcg")]
            cli::Rng::Pcg64 => Ok(Self::Pcg64(rand_pcg::Pcg64::try_from_os_rng()?)),
            #[cfg(feature = "pcg")]
            cli::Rng::Pcg64Dxsm => Ok(Self::Pcg64Dxsm(rand_pcg::Pcg64Dxsm::try_from_os_rng()?)),
            #[cfg(feature = "pcg")]
            cli::Rng::Pcg64Mcg => Ok(Self::Pcg64Mcg(rand_pcg::Pcg64Mcg::try_from_os_rng()?)),
            #[cfg(feature = "sfc")]
            cli::Rng::Sfc32 => Ok(Self::Sfc32(rand_sfc::Sfc32::try_from_os_rng()?)),
            #[cfg(feature = "sfc")]
            cli::Rng::Sfc64 => Ok(Self::Sfc64(rand_sfc::Sfc64::try_from_os_rng()?)),
            cli::Rng::SplitMix64 => Ok(Self::SplitMix64(SplitMix64::try_from_os_rng()?)),
            #[cfg(feature = "xorshift")]
            cli::Rng::XorShift => Ok(Self::XorShift(
                rand_xorshift::XorShiftRng::try_from_os_rng()?
            )),
            cli::Rng::Xoroshiro64Star => {
                Ok(Self::Xoroshiro64Star(Xoroshiro64Star::try_from_os_rng()?))
            }
            cli::Rng::Xoroshiro64StarStar => Ok(Self::Xoroshiro64StarStar(
                Xoroshiro64StarStar::try_from_os_rng()?,
            )),
            cli::Rng::Xoroshiro128Plus => {
                Ok(Self::Xoroshiro128Plus(Xoroshiro128Plus::try_from_os_rng()?))
            }
            cli::Rng::Xoroshiro128PlusPlus => Ok(Self::Xoroshiro128PlusPlus(
                Xoroshiro128PlusPlus::try_from_os_rng()?,
            )),
            cli::Rng::Xoroshiro128StarStar => Ok(Self::Xoroshiro128StarStar(
                Xoroshiro128StarStar::try_from_os_rng()?,
            )),
            cli::Rng::Xoshiro128Plus => {
                Ok(Self::Xoshiro128Plus(Xoshiro128Plus::try_from_os_rng()?))
            }
            cli::Rng::Xoshiro128PlusPlus => Ok(Self::Xoshiro128PlusPlus(
                Xoshiro128PlusPlus::try_from_os_rng()?,
            )),
            cli::Rng::Xoshiro128StarStar => Ok(Self::Xoshiro128StarStar(
                Xoshiro128StarStar::try_from_os_rng()?,
            )),
            cli::Rng::Xoshiro256Plus => {
                Ok(Self::Xoshiro256Plus(Xoshiro256Plus::try_from_os_rng()?))
            }
            cli::Rng::Xoshiro256PlusPlus => Ok(Self::Xoshiro256PlusPlus(
                Xoshiro256PlusPlus::try_from_os_rng()?,
            )),
            cli::Rng::Xoshiro256StarStar => Ok(Self::Xoshiro256StarStar(
                Xoshiro256StarStar::try_from_os_rng()?,
            )),
            cli::Rng::Xoshiro512Plus => {
                Ok(Self::Xoshiro512Plus(Xoshiro512Plus::try_from_os_rng()?))
            }
            cli::Rng::Xoshiro512PlusPlus => Ok(Self::Xoshiro512PlusPlus(
                Xoshiro512PlusPlus::try_from_os_rng()?,
            )),
            cli::Rng::Xoshiro512StarStar => Ok(Self::Xoshiro512StarStar(
                Xoshiro512StarStar::try_from_os_rng()?,
            )),
        }
    }
}
