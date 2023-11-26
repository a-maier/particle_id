//! Particle numbers according to the [Monte Carlo Particle Numbering
//! Scheme](https://pdg.lbl.gov/2023/mcdata/mc_particle_id_contents.html)
//!
//! # Example
//!
//! ```
//! use particle_id::light_baryons::*;
//!
//! assert_eq!(proton.name().unwrap(), "proton");
//! assert_eq!(proton.id(), 2212);
//! assert_eq!(proton.anti().id(), -proton.id());
//! ```
#![allow(non_upper_case_globals)]
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// Particle ID according to the [Monte Carlo Particle Numbering
/// Scheme](https://pdg.lbl.gov/2023/mcdata/mc_particle_id_contents.html)
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ParticleID(i32);

const fn concat<const A: usize, const B: usize, const C: usize>(
    a: [ParticleID; A],
    b: [ParticleID; B],
) -> [ParticleID; C] {
    // Assert that `A + B == C`.
    // Any index other than `0` would panic, since we access out of bounds otherwise.
    let _ = [0; 1][(A + B) - C]; // Assert that `A + B == C`

    let mut result = [ParticleID(0); C];

    let mut i = 0;
    while i < A {
        result[i] = a[i];
        i += 1;
    }

    while i < A + B {
        result[i] = b[i - A];
        i += 1;
    }

    result
}

macro_rules! concat_arrays {
    ($($arr:expr),*) => {
        concat_arrays!(@concat $( [$arr ; $arr.len()] )*)
    };

    (@concat [$a:expr; $a_len:expr]) => {
        $a
    };

    (@concat [$a:expr; $a_len:expr] [$b:expr; $b_len:expr] $($tail:tt)*) => {
        concat_arrays!(
            @concat
                [crate::concat::<{ $a_len }, { $b_len }, { $a_len + $b_len }>($a, $b); $a_len + $b_len]
                $($tail)*
        )
    };
}

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

macro_rules! particle_set {
    ($set:ident = {$($particle:ident: $id:literal,)*}) => {
        $(
            pub const $particle: ParticleID = ParticleID($id);
        )*
            pub const $set: [ParticleID;  count!($($particle)*)] = [
                $($particle,)*
            ];
    };
}

pub mod quarks {
    use super::*;
    particle_set!(
        QUARKS = {
            d: 1,
            u: 2,
            s: 3,
            c: 4,
            b: 5,
            t: 6,
            b_prime: 7,
            t_prime: 9,
        }
    );

    pub const down: ParticleID = d;
    pub const up: ParticleID = u;
    pub const strange: ParticleID = s;
    pub const charm: ParticleID = c;
    pub const bottom: ParticleID = b;
    pub const top: ParticleID = t;
}

pub mod leptons {
    use super::*;
    particle_set!(
        LEPTONS = {
            e: 11,
            ν_e: 12,
            μ: 13,
            ν_μ: 14,
            τ: 15,
            ν_τ: 16,
            τ_prime: 17,
            ν_τ_prime: 18,
        }
    );

    pub const electron: ParticleID = e;
    pub const nu_e: ParticleID = ν_e;
    pub const electron_neutrino: ParticleID = ν_e;
    pub const mu: ParticleID = μ;
    pub const muon: ParticleID = μ;
    pub const nu_mu: ParticleID = ν_μ;
    pub const muon_neutrino: ParticleID = ν_μ;
    pub const tau: ParticleID = τ;
    pub const nu_tau: ParticleID = ν_τ;
    pub const tau_neutrino: ParticleID = ν_τ;
    pub const tau_prime: ParticleID = τ_prime;
    pub const nu_tau_prime: ParticleID = ν_τ_prime;
    pub const tau_prime_neutrino: ParticleID = ν_τ_prime;
}

pub mod gauge_bosons {
    use super::*;
    particle_set!(
        GAUGE_BOSONS = {
            g: 21,
            γ: 22,
            Z: 23,
            W_plus: 24,
            Z_prime: 32,
            Z_prime_prime: 33,
            W_prime: 34,
        }
    );
    pub const gluon: ParticleID = g;
    pub const gamma: ParticleID = γ;
    pub const photon: ParticleID = γ;
}

pub mod higgs_bosons {
    use super::*;
    particle_set!(
        HIGGS_BOSONS = {
            h: 25,
            H0: 35,
            A0: 36,
            H_plus: 37,
            H_plus_plus: 38,
            a0: 40,
        }
    );
    pub const H: ParticleID = h;
    pub const Higgs: ParticleID = h;
    pub const H_0: ParticleID = H0;
    pub const A_0: ParticleID = A0;
    pub const a_0: ParticleID = a0;
}

pub mod gauge_and_higgs_bosons {
    pub use super::gauge_bosons::*;
    pub use super::higgs_bosons::*;
}

pub mod special_particles {
    use super::*;
    particle_set!(
        SPECIAL_PARTICLES = {
            G: 39,
            R_0: 41,
            LQ_c: 42,
            reggeon: 110,
            pomeron: 990,
            odderon: 9990,
        }
    );
    pub const graviton: ParticleID = G;
}

pub mod diquarks {
    use super::*;
    particle_set!(
        DIQUARKS = {
            dd_1: 1103,
            ud_0: 2101,
            ud_1: 2103,
            uu_1: 2203,
            sd_0: 3101,
            sd_1: 3103,
            su_0: 3201,
            su_1: 3203,
            ss_1: 3303,
            cd_0: 4101,
            cd_1: 4103,
            cu_0: 4201,
            cu_1: 4203,
            cs_0: 4301,
            cs_1: 4303,
            cc_1: 4403,
            bd_0: 5101,
            bd_1: 5103,
            bu_0: 5201,
            bu_1: 5203,
            bs_0: 5301,
            bs_1: 5303,
            bc_0: 5401,
            bc_1: 5403,
            bb_1: 5503,
        }
    );
}

pub mod susy_particles {
    use super::*;
    particle_set!(
        SUSY_PARTICLES = {
            d_tilde_L: 1000001,
            u_tilde_L: 1000002,
            s_tilde_L: 1000003,
            c_tilde_L: 1000004,
            b_tilde_1: 1000005,
            t_tilde_1: 1000006,
            e_tilde_L: 1000011,
            ν_e_tilde_L: 1000012,
            μ_tilde_L: 1000013,
            ν_μ_tilde_L: 1000014,
            τ_tilde_1: 1000015,
            ν_τ_tilde_L: 1000016,
            d_tilde_R: 2000001,
            u_tilde_R: 2000002,
            s_tilde_R: 2000003,
            c_tilde_R: 2000004,
            b_tilde_2: 2000005,
            t_tilde_2: 2000006,
            e_tilde_R: 2000011,
            μ_tilde_R: 2000013,
            τ_tilde_2: 2000015,
            g_tilde: 1000021,
            χ_tilde_0_1: 1000022,
            χ_tilde_0_2: 1000023,
            χ_tilde_plus_1: 1000024,
            χ_tilde_0_3: 1000025,
            χ_tilde_0_4: 1000035,
            χ_tilde_plus_2: 1000037,
            G_tilde: 1000039,
        }
    );
}

#[allow(non_snake_case)]
pub mod light_Ieq1_mesons {
    use super::*;
    particle_set!(
        LIGHT_IEQ1_MESONS = {
            π_0: 111,
            π_plus: 211,
            a_0_980_0: 9000111,
            a_0_980_plus: 9000211,
            π_1300_0: 100111,
            π_1300_plus: 100211,
            a_0_1450_0: 10111,
            a_0_1450_plus: 10211,
            π_1800_0: 9010111,
            π_1800_plus: 9010211,
            ρ_770_0: 113,
            ρ_770_plus: 213,
            b_1_1235_0: 10113,
            b_1_1235_plus: 10213,
            a_1_1260_0: 20113,
            a_1_1260_plus: 20213,
            π_1_1400_0: 9000113,
            π_1_1400_plus: 9000213,
            ρ_1450_0: 100113,
            ρ_1450_plus: 100213,
            π_1_1600_0: 9010113,
            π_1_1600_plus: 9010213,
            a_1_1640_0: 9020113,
            a_1_1640_plus: 9020213,
            ρ_1700_0: 30113,
            ρ_1700_plus: 30213,
            ρ_1900_0: 9030113,
            ρ_1900_plus: 9030213,
            ρ_2150_0: 9040113,
            ρ_2150_plus: 9040213,
            a_2_1320_0: 115,
            a_2_1320_plus: 215,
            π_2_1670_0: 10115,
            π_2_1670_plus: 10215,
            a_2_1700_0: 9000115,
            a_2_1700_plus: 9000215,
            π_2_2100_0: 9010115,
            π_2_2100_plus: 9010215,
            ρ_3_1690_0: 117,
            ρ_3_1690_plus: 217,
            ρ_3_1990_0: 9000117,
            ρ_3_1990_plus: 9000217,
            ρ_3_2250_0: 9010117,
            ρ_3_2250_plus: 9010217,
            a_4_2040_0: 119,
            a_4_2040_plus: 219,
        }
    );
}

#[allow(non_snake_case)]
pub mod light_Ieq0_mesons {
    use super::*;
    particle_set!(
        LIGHT_IEQ0_MESONS = {
            η: 221,
            η_prime_958: 331,
            f_0_500: 9000221,
            f_0_980: 9010221,
            η_1295: 100221,
            f_0_1370: 10221,
            η_1405: 9020221,
            η_1475: 100331,
            f_0_1500: 9030221,
            f_0_1710: 10331,
            η_1760: 9040221,
            f_0_2020: 9050221,
            f_0_2100: 9060221,
            f_0_2200: 9070221,
            η_2225: 9080221,
            ω_782: 223,
            φ_1020: 333,
            h_1_1170: 10223,
            f_1_1285: 20223,
            h_1_1380: 10333,
            f_1_1420: 20333,
            ω_1420: 100223,
            f_1_1510: 9000223,
            h_1_1595: 9010223,
            ω_1650: 30223,
            φ_1680: 100333,
            f_2_1270: 225,
            f_2_1430: 9000225,
            f_2_prime_1525: 335,
            f_2_1565: 9010225,
            f_2_1640: 9020225,
            η_2_1645: 10225,
            f_2_1810: 9030225,
            η_2_1870: 10335,
            f_2_1910: 9040225,
            f_2_1950: 9050225,
            f_2_2010: 9060225,
            f_2_2150: 9070225,
            f_2_2300: 9080225,
            f_2_2340: 9090225,
            ω_3_1670: 227,
            φ_3_1850: 337,
            f_4_2050: 229,
            f_J_2220: 9000229,
            f_4_2300: 9010229,
        }
    );
}

pub mod light_mesons {
    use crate::ParticleID;

    pub use super::light_Ieq0_mesons::*;
    pub use super::light_Ieq1_mesons::*;
    pub const LIGHT_MESONS: [ParticleID; 91] = concat_arrays!(
        LIGHT_IEQ0_MESONS,
        LIGHT_IEQ1_MESONS
    );
}

pub mod strange_mesons {
    use super::*;
    particle_set!(
        STRANGE_MESONS = {
            K_0_L: 130,
            K_0_S: 310,
            K_0: 311,
            K_plus: 321,
            K_0_star_700_0: 9000311,
            K_0_star_700_plus: 9000321,
            K_0_star_1430_0: 10311,
            K_0_star_1430_plus: 10321,
            K_1460_0: 100311,
            K_1460_plus: 100321,
            K_1830_0: 9010311,
            K_1830_plus: 9010321,
            K_0_star_1950_0: 9020311,
            K_0_star_1950_plus: 9020321,
            K_star_892_0: 313,
            K_star_892_plus: 323,
            K_1_1270_0: 10313,
            K_1_1270_plus: 10323,
            K_1_1400_0: 20313,
            K_1_1400_plus: 20323,
            K_star_1410_0: 100313,
            K_star_1410_plus: 100323,
            K_1_1650_0: 9000313,
            K_1_1650_plus: 9000323,
            K_star_1680_0: 30313,
            K_star_1680_plus: 30323,
            K_2_star_1430_0: 315,
            K_2_star_1430_plus: 325,
            K_2_1580_0: 9000315,
            K_2_1580_plus: 9000325,
            K_2_1770_0: 10315,
            K_2_1770_plus: 10325,
            K_2_1820_0: 20315,
            K_2_1820_plus: 20325,
            K_2_star_1980_0: 9010315,
            K_2_star_1980_plus: 9010325,
            K_2_2250_0: 9020315,
            K_2_2250_plus: 9020325,
            K_3_star_1780_0: 317,
            K_3_star_1780_plus: 327,
            K_3_2320_0: 9010317,
            K_3_2320_plus: 9010327,
            K_4_star_2045_0: 319,
            K_4_star_2045_plus: 329,
            K_4_2500_0: 9000319,
            K_4_2500_plus: 9000329,
        }
    );
}

pub mod charmed_mesons {
    use super::*;

    particle_set!(
        CHARMED_MESONS = {
            D_plus: 411,
            D_0: 421,
            D_0_star_2400_plus: 10411,
            D_0_star_2400_0: 10421,
            D_star_2010_plus: 413,
            D_star_2007_0: 423,
            D_1_2420_plus: 10413,
            D_1_2420_0: 10423,
            D_1_H_plus: 20413,
            D_1_2430_0: 20423,
            D_2_star_2460_plus: 415,
            D_2_star_2460_0: 425,
            D_s_plus: 431,
            D_s0_star_2317_plus: 10431,
            D_s_star_plus: 433,
            D_s_1_2536_plus: 10433,
            D_s_1_2460_plus: 20433,
            D_s_2_star_2573_plus: 435,
        }
    );
}

pub mod bottom_mesons {
    use super::*;
    particle_set!(
        BOTTOM_MESONS = {
            B_0: 511,
            B_plus: 521,
            B_0_star_0: 10511,
            B_0_star_plus: 10521,
            B_star_0: 513,
            B_star_plus: 523,
            B_1_L_0: 10513,
            B_1_L_plus: 10523,
            B_1_H_0: 20513,
            B_1_H_plus: 20523,
            B_2_star0: 515,
            B_2_star_plus: 525,
            B_s_0: 531,
            B_s_0_star_0: 10531,
            B_s_star_0: 533,
            B_s_1_L_0: 10533,
            B_s_1_H_0: 20533,
            B_s_2_star_0: 535,
            B_c_plus: 541,
            B_c_0_star_plus: 10541,
            B_c_star_plus: 543,
            B_c_1_L_plus: 10543,
            B_c_1_H_plus: 20543,
            B_c_2_star_plus: 545,
        }
    );
}

pub mod ccbar_mesons {
    use super::*;
    particle_set!(
        CCBAR_MESONS = {
            η_c_1S: 441,
            χ_c_0_1P: 10441,
            η_c_2S: 100441,
            Jψ_1S: 443,
            h_c_1P: 10443,
            χ_c_1_1P: 20443,
            ψ_2S: 100443,
            ψ_3770: 30443,
            ψ_4040: 9000443,
            ψ_4160: 9010443,
            ψ_4415: 9020443,
            χ_c_2_1P: 445,
            χ_c_2_3930: 100445,
        }
    );
}

pub mod bbbar_mesons {
    use super::*;
    particle_set!(
        BBBAR_MESONS = {
            η_b_1S: 551,
            χ_b_0_1P: 10551,
            η_b_2S: 100551,
            χ_b_0_2P: 110551,
            η_b_3S: 200551,
            χ_b_0_3P: 210551,
            Υ_1S: 553,
            h_b_1P: 10553,
            χ_b_1_1P: 20553,
            Υ_1_1D: 30553,
            Υ_2S: 100553,
            h_b_2P: 110553,
            χ_b_1_2P: 120553,
            Υ_1_2D: 130553,
            Υ_3S: 200553,
            h_b_3P: 210553,
            χ_b_1_3P: 220553,
            Υ_4S: 300553,
            Υ_10860: 9000553,
            Υ_11020: 9010553,
            χ_b_2_1P: 555,
            η_b_2_1D: 10555,
            Υ_2_1D: 20555,
            χ_b_2_2P: 100555,
            η_b_2_2D: 110555,
            Υ_2_2D: 120555,
            χ_b_2_3P: 200555,
            Υ_3_1D: 557,
            Υ_3_2D: 100557,
        }
    );
}

pub mod mesons {
    use crate::ParticleID;

    pub use super::light_mesons::*;
    pub use super::strange_mesons::*;
    pub use super::charmed_mesons::*;
    pub use super::bottom_mesons::*;
    pub use super::ccbar_mesons::*;
    pub use super::bbbar_mesons::*;
    pub const MESONS: [ParticleID; 221] = concat_arrays!(
        LIGHT_MESONS,
        STRANGE_MESONS,
        CHARMED_MESONS,
        BOTTOM_MESONS,
        CCBAR_MESONS,
        BBBAR_MESONS
    );
}

pub mod light_baryons {
    use super::*;

    particle_set!(
        LIGHT_BARYONS = {
            p: 2212,
            n: 2112,
            Δ_plus_plus: 2224,
            Δ_plus: 2214,
            Δ_0: 2114,
            Δ_minus: 1114,
        }
    );
    pub const proton: ParticleID = p;
    pub const neutron: ParticleID = n;
}

pub mod strange_baryons {
    use super::*;
    particle_set!(
        STRANGE_BARYONS = {
            Λ: 3122,
            Σ_plus: 3222,
            Σ_0: 3212,
            Σ_minus: 3112,
            Σ_star_plus: 3224,
            Σ_star_0: 3214,
            Σ_star_minus: 3114,
            Ξ_0: 3322,
            Ξ_minus: 3312,
            Ξ_star_0: 3324,
            Ξ_star_minus: 3314,
            Ω_minus: 3334,
        }
    );
}

pub mod charmed_baryons {
    use super::*;
    particle_set!(
        CHARMED_BARYONS = {
            Λ_c_plus: 4122,
            Σ_c_plus_plus: 4222,
            Σ_c_plus: 4212,
            Σ_c_0: 4112,
            Σ_c_star_plus_plus: 4224,
            Σ_c_star_plus: 4214,
            Σ_c_star_0: 4114,
            Ξ_c_plus: 4232,
            Ξ_c_0: 4132,
            Ξ_c_prime_plus: 4322,
            Ξ_c_prime_0: 4312,
            Ξ_c_star_plus: 4324,
            Ξ_c_star_0: 4314,
            Ω_c_0: 4332,
            Ω_c_star_0: 4334,
            Ξ_c_c_plus: 4412,
            Ξ_c_c_plus_plus: 4422,
            Ξ_c_c_star_plus: 4414,
            Ξ_c_c_star_plus_plus: 4424,
            Ω_c_c_plus: 4432,
            Ω_c_c_star_plus: 4434,
            Ω_c_c_c_plus_plus: 4444,
        }
    );
}

pub mod bottom_baryons {
    use super::*;
    particle_set!(
        BOTTOM_BARYONS = {
            Λ_b_0: 5122,
            Σ_b_minus: 5112,
            Σ_b_0: 5212,
            Σ_b_plus: 5222,
            Σ_b_star_minus: 5114,
            Σ_b_star_0: 5214,
            Σ_b_star_plus: 5224,
            Ξ_b_minus: 5132,
            Ξ_b_0: 5232,
            Ξ_b_prime_minus: 5312,
            Ξ_b_prime_0: 5322,
            Ξ_b_star_minus: 5314,
            Ξ_b_star_0: 5324,
            Ω_b_minus: 5332,
            Ω_b_star_minus: 5334,
            Ξ_b_c_0: 5142,
            Ξ_b_c_plus: 5242,
            Ξ_b_c_prime_0: 5412,
            Ξ_b_c_prime_plus: 5422,
            Ξ_b_c_star_0: 5414,
            Ξ_b_c_star_plus: 5424,
            Ω_b_c_0: 5342,
            Ω_b_c_prime_0: 5432,
            Ω_b_c_star_0: 5434,
            Ω_b_c_c_plus: 5442,
            Ω_b_c_c_star_plus: 5444,
            Ξ_b_b_minus: 5512,
            Ξ_b_b_0: 5522,
            Ξ_b_b_star_minus: 5514,
            Ξ_b_b_star_0: 5524,
            Ω_b_b_minus: 5532,
            Ω_b_b_star_minus: 5534,
            Ω_b_b_c_0: 5542,
            Ω_b_b_c_star_0: 5544,
            Ω_b_b_b_minus: 5554,
        }
    );
}

pub mod pentaquarks {
    use super::*;
    particle_set!(
        PENTAQUARKS = {
            Θ_plus: 100221132,
            Φ_minus_minus: 100331122,
        }
    );
}

pub mod baryons {
    use crate::ParticleID;

    pub use super::light_baryons::*;
    pub use super::strange_baryons::*;
    pub use super::charmed_baryons::*;
    pub use super::bottom_baryons::*;
    pub const BARYONS: [ParticleID; 75] = concat_arrays!(
        LIGHT_BARYONS,
        STRANGE_BARYONS,
        CHARMED_BARYONS,
        BOTTOM_BARYONS
    );
}

pub mod hadrons {
    use crate::ParticleID;

    pub use super::mesons::MESONS;
    pub use super::baryons::BARYONS;
    pub const HADRONS: [ParticleID; 296] = concat_arrays!(
        MESONS,
        BARYONS
    );
}


pub mod anti_quarks {
    use super::*;
    particle_set!(
        ANTI_QUARKS = {
            d_bar: -1,
            u_bar: -2,
            s_bar: -3,
            c_bar: -4,
            b_bar: -5,
            t_bar: -6,
            b_prime_bar: -7,
            t_prime_bar: -9,
        }
    );
    pub const anti_down: ParticleID = d_bar;
    pub const anti_up: ParticleID = u_bar;
    pub const anti_strange: ParticleID = s_bar;
    pub const anti_charm: ParticleID = c_bar;
    pub const anti_bottom: ParticleID = b_bar;
    pub const anti_top: ParticleID = t_bar;
}

pub mod anti_leptons {
    use super::*;

    particle_set!(
        ANTI_LEPTONS = {
            e_bar: -11,
            ν_e_bar: -12,
            μ_bar: -13,
            ν_μ_bar: -14,
            τ_bar: -15,
            ν_τ_bar: -16,
            τ_prime_bar: -17,
            ν_τ_prime_bar: -18,
        }
    );
    pub const positron: ParticleID = e_bar;
    pub const nu_e_bar: ParticleID = ν_e_bar;
    pub const electron_anti_neutrino: ParticleID = ν_e_bar;
    pub const mu_bar: ParticleID = μ_bar;
    pub const mu_plus: ParticleID = μ_bar;
    pub const μ_plus: ParticleID = μ_bar;
    pub const anti_muon: ParticleID = μ_bar;
    pub const nu_mu_bar: ParticleID = ν_μ_bar;
    pub const muon_anti_neutrino: ParticleID = ν_μ_bar;
    pub const tau_bar: ParticleID = τ_bar;
    pub const tau_plus: ParticleID = τ_bar;
    pub const τ_plus: ParticleID = τ_bar;
    pub const anti_tau: ParticleID = τ_bar;
    pub const nu_tau_bar: ParticleID = ν_τ_bar;
    pub const tau_anti_neutrino: ParticleID = ν_τ_bar;
    pub const tau_prime_bar: ParticleID = τ_prime_bar;
    pub const nu_tau_prime_bar: ParticleID = ν_τ_prime_bar;
    pub const tau_prime_anti_neutrino: ParticleID = ν_τ_prime_bar;
}

pub mod anti_gauge_and_higgs_bosons {
    use super::*;

    pub const W_minus: ParticleID = ParticleID(-24);
    pub const W_prime_minus: ParticleID = ParticleID(-34);
    pub const H_minus: ParticleID = ParticleID(-37);
    pub const H_minus_minus: ParticleID = ParticleID(-38);
}

pub mod light_anti_baryons {
    use super::{ParticleID, light_baryons::{p, n}};
    pub const p_bar: ParticleID = p.anti();
    pub const anti_proton: ParticleID = p_bar;
    pub const n_bar: ParticleID = n.anti();
    pub const anti_neutron: ParticleID = n_bar;
}

pub mod susy_anti_particles {
    use super::*;

    pub const d_tilde_bar_L: ParticleID = ParticleID(-1000001);
    pub const u_tilde_bar_L: ParticleID = ParticleID(-1000002);
    pub const s_tilde_bar_L: ParticleID = ParticleID(-1000003);
    pub const c_tilde_bar_L: ParticleID = ParticleID(-1000004);
    pub const b_tilde_bar_1: ParticleID = ParticleID(-1000005);
    pub const t_tilde_bar_1: ParticleID = ParticleID(-1000006);
    pub const e_tilde_bar_L: ParticleID = ParticleID(-1000011);
    pub const ν_e_tilde_bar_L: ParticleID = ParticleID(-1000012);
    pub const μ_tilde_bar_L: ParticleID = ParticleID(-1000013);
    pub const ν_μ_tilde_bar_L: ParticleID = ParticleID(-1000014);
    pub const τ_tilde_bar_1: ParticleID = ParticleID(-1000015);
    pub const ν_τ_tilde_bar_L: ParticleID = ParticleID(-1000016);
    pub const d_tilde_bar_R: ParticleID = ParticleID(-2000001);
    pub const u_tilde_bar_R: ParticleID = ParticleID(-2000002);
    pub const s_tilde_bar_R: ParticleID = ParticleID(-2000003);
    pub const c_tilde_bar_R: ParticleID = ParticleID(-2000004);
    pub const b_tilde_bar_2: ParticleID = ParticleID(-2000005);
    pub const t_tilde_bar_2: ParticleID = ParticleID(-2000006);
    pub const e_tilde_bar_R: ParticleID = ParticleID(-2000011);
    pub const μ_tilde_bar_R: ParticleID = ParticleID(-2000013);
    pub const τ_tilde_bar_2: ParticleID = ParticleID(-2000015);
    pub const χ_tilde_minus_1: ParticleID = ParticleID(-1000024);
    pub const χ_tilde_minus_2: ParticleID = ParticleID(-1000037);
}

pub mod sm_elementary_particles {
    pub use super::anti_gauge_and_higgs_bosons::*;
    pub use super::anti_quarks::*;
    pub use super::anti_leptons::*;
    pub use super::gauge_and_higgs_bosons::*;
    pub use super::quarks::*;
    pub use super::leptons::*;
}

impl ParticleID {
    /// Construct from the given `id`
    pub const fn new(id: i32) -> Self {
        Self(id)
    }

    /// Particle symbol in LaTeX format
    pub const fn latex_symbol(&self) -> Option<&'static str> {
        // TODO: many missing
        use sm_elementary_particles::*;
        use light_baryons::*;
        use light_anti_baryons::*;

        let name = match *self {
            d => "d",
            u => "u",
            s => "s",
            c => "c",
            b => "b",
            t => "t",
            b_prime => "b'",
            t_prime => "t'",
            e => "e^-",
            ν_e => r"\nu_e",
            μ => r"\mu^-",
            ν_μ => r"\nu_\mu",
            τ => r"\tau^-",
            ν_τ => r"\nu_\tau",
            τ_prime => r"\tau'^-",
            ν_τ_prime => r"\nu_{\tau'}",
            g => "g",
            γ => r"\gamma",
            Z => r"Z",
            W_plus => r"W^+",
            h => "h",
            Z_prime => "Z'",
            Z_prime_prime => "Z''",
            W_prime => "W'",
            H0 => "H^0",
            A0 => "A^0",
            H_plus  => "H^+",
            H_plus_plus => "H^{++}",
            a0 => "a_0",
            p => "p",
            n => "n",

            d_bar => r"\bar{d}",
            u_bar => r"\bar{u}",
            s_bar => r"\bar{s}",
            c_bar => r"\bar{c}",
            b_bar => r"\bar{b}",
            t_bar => r"\bar{t}",
            b_prime_bar => r"\bar{b}'",
            t_prime_bar => r"\bar{t}'",
            e_bar => r"e^+",
            ν_e_bar => r"\bar{\nu}_e",
            μ_bar => r"\mu^+",
            ν_μ_bar => r"\bar{\nu}_\mu",
            τ_bar => r"\tau^+",
            ν_τ_bar => r"\bar{\nu}_\tau",
            τ_prime_bar => r"\tau'^+",
            ν_τ_prime_bar => r"\bar{\nu}_{\tau'}",
            W_minus => "W^-",
            W_prime_minus => "W^{--}",
            H_minus => "H^-",
            H_minus_minus => "H^{--}",
            anti_proton => r"\bar{p}",
            anti_neutron => r"\bar{p}",

            light_Ieq1_mesons::π_0 => r"\pi^0",
            light_Ieq1_mesons::π_plus => r"\pi^+",
            light_Ieq1_mesons::a_0_980_0 => r"a_0(980)^0",
            light_Ieq1_mesons::a_0_980_plus => r"a_0(980)^+",
            light_Ieq1_mesons::π_1300_0 => r"\pi(1300)^0",
            light_Ieq1_mesons::π_1300_plus => r"\pi(1300)^+",
            light_Ieq1_mesons::a_0_1450_0 => r"a_0(1450)^0",
            light_Ieq1_mesons::a_0_1450_plus => r"a_0(1450)^+",
            light_Ieq1_mesons::π_1800_0 => r"\pi(1800)^0",
            light_Ieq1_mesons::π_1800_plus => r"\pi(1800)^+",
            light_Ieq1_mesons::ρ_770_0 => r"\rho(770)^0",
            light_Ieq1_mesons::ρ_770_plus => r"\rho(770)^+",
            light_Ieq1_mesons::b_1_1235_0 => r"b_1(1235)^0",
            light_Ieq1_mesons::b_1_1235_plus => r"b_1(1235)^+",
            light_Ieq1_mesons::a_1_1260_0 => r"a_1(1260)^0",
            light_Ieq1_mesons::a_1_1260_plus => r"a_1(1260)^+",
            light_Ieq1_mesons::π_1_1400_0 => r"\pi_1(1400)^0",
            light_Ieq1_mesons::π_1_1400_plus => r"\pi_1(1400)^+",
            light_Ieq1_mesons::ρ_1450_0 => r"\rho(1450)^0",
            light_Ieq1_mesons::ρ_1450_plus => r"\rho(1450)^+",
            light_Ieq1_mesons::π_1_1600_0 => r"\pi_1(1600)^0",
            light_Ieq1_mesons::π_1_1600_plus => r"\pi_1(1600)^+",
            light_Ieq1_mesons::a_1_1640_0 => r"a_1(1640)^0",
            light_Ieq1_mesons::a_1_1640_plus => r"a_1(1640)^+",
            light_Ieq1_mesons::ρ_1700_0 => r"\rho(1700)^0",
            light_Ieq1_mesons::ρ_1700_plus => r"\rho(1700)^+",
            light_Ieq1_mesons::ρ_1900_0 => r"\rho(1900)^0",
            light_Ieq1_mesons::ρ_1900_plus => r"\rho(1900)^+",
            light_Ieq1_mesons::ρ_2150_0 => r"\rho(2150)^0",
            light_Ieq1_mesons::ρ_2150_plus => r"\rho(2150)^+",
            light_Ieq1_mesons::a_2_1320_0 => r"a_2(1320)^0",
            light_Ieq1_mesons::a_2_1320_plus => r"a_2(1320)^+",
            light_Ieq1_mesons::π_2_1670_0 => r"\pi_2(1670)^0",
            light_Ieq1_mesons::π_2_1670_plus => r"\pi_2(1670)^+",
            light_Ieq1_mesons::a_2_1700_0 => r"a_2(1700)^0",
            light_Ieq1_mesons::a_2_1700_plus => r"a_2(1700)^+",
            light_Ieq1_mesons::π_2_2100_0 => r"\pi_2(2100)^0",
            light_Ieq1_mesons::π_2_2100_plus => r"\pi_2(2100)^+",
            light_Ieq1_mesons::ρ_3_1690_0 => r"\rho_3(1690)^0",
            light_Ieq1_mesons::ρ_3_1690_plus => r"\rho_3(1690)^+",
            light_Ieq1_mesons::ρ_3_1990_0 => r"\rho_3(1990)^0",
            light_Ieq1_mesons::ρ_3_1990_plus => r"\rho_3(1990)^+",
            light_Ieq1_mesons::ρ_3_2250_0 => r"\rho_3(2250)^0",
            light_Ieq1_mesons::ρ_3_2250_plus => r"\rho_3(2250)^+",
            light_Ieq1_mesons::a_4_2040_0 => r"a_4(2040)^0",
            light_Ieq1_mesons::a_4_2040_plus => r"a_4(2040)^+",

            light_Ieq0_mesons::η => r"\eta",
            light_Ieq0_mesons::η_prime_958 => r"\eta'(958)",
            light_Ieq0_mesons::f_0_500 => "f_0(500)",
            light_Ieq0_mesons::f_0_980 => "f_0(980)",
            light_Ieq0_mesons::η_1295 => r"\eta(1295)",
            light_Ieq0_mesons::f_0_1370 => "f_0(1370)",
            light_Ieq0_mesons::η_1405 => r"\eta(1405)",
            light_Ieq0_mesons::η_1475 => r"\eta(1475)",
            light_Ieq0_mesons::f_0_1500 => "f_0(1500)",
            light_Ieq0_mesons::f_0_1710 => "f_0(1710)",
            light_Ieq0_mesons::η_1760 => r"\eta(1760)",
            light_Ieq0_mesons::f_0_2020 => "f_0(2020)",
            light_Ieq0_mesons::f_0_2100 => "f_0(2100)",
            light_Ieq0_mesons::f_0_2200 => "f_0(2200)",
            light_Ieq0_mesons::η_2225 => r"\eta(2225)",
            light_Ieq0_mesons::ω_782 => r"\omega(782)",
            light_Ieq0_mesons::φ_1020 => r"\phi(1020)",
            light_Ieq0_mesons::h_1_1170 => r"h_1(1170)",
            light_Ieq0_mesons::f_1_1285 => "f_1(1285)",
            light_Ieq0_mesons::h_1_1380 => r"h_1(1380)",
            light_Ieq0_mesons::f_1_1420 => "f_1(1420)",
            light_Ieq0_mesons::ω_1420 => r"\omega(1420)",
            light_Ieq0_mesons::f_1_1510 => "f_1(1510)",
            light_Ieq0_mesons::h_1_1595 => r"h_1(1595)",
            light_Ieq0_mesons::ω_1650 => r"\omega(1650)",
            light_Ieq0_mesons::φ_1680 => r"\phi(1680)",
            light_Ieq0_mesons::f_2_1270 => "f_2(1270)",
            light_Ieq0_mesons::f_2_1430 => "f_2(1430)",
            light_Ieq0_mesons::f_2_prime_1525 => "f_2'(1525)",
            light_Ieq0_mesons::f_2_1565 => "f_2(1565)",
            light_Ieq0_mesons::f_2_1640 => "f_2(1640)",
            light_Ieq0_mesons::η_2_1645 => r"\eta_2(1645)",
            light_Ieq0_mesons::f_2_1810 => "f_2(1810)",
            light_Ieq0_mesons::η_2_1870 => r"\eta_2(1870)",
            light_Ieq0_mesons::f_2_1910 => "f_2(1910)",
            light_Ieq0_mesons::f_2_1950 => "f_2(1950)",
            light_Ieq0_mesons::f_2_2010 => "f_2(2010)",
            light_Ieq0_mesons::f_2_2150 => "f_2(2150)",
            light_Ieq0_mesons::f_2_2300 => "f_2(2300)",
            light_Ieq0_mesons::f_2_2340 => "f_2(2340)",
            light_Ieq0_mesons::ω_3_1670 => r"\omega_3(1670)",
            light_Ieq0_mesons::φ_3_1850 => r"\phi_3(1850)",
            light_Ieq0_mesons::f_4_2050 => "f_4(2050)",
            light_Ieq0_mesons::f_J_2220 => "f_J(2220)",
            light_Ieq0_mesons::f_4_2300 => "f_4(2300)",

            strange_mesons::K_0_L=> "K^0_L",
            strange_mesons::K_0_S => "K^0_S",
            strange_mesons::K_0 => "K^0",
            strange_mesons::K_plus => "K^+",
            strange_mesons::K_0_star_700_0 => "K_0^*(700)^0",
            strange_mesons::K_0_star_700_plus => "K_0^*(700)^+",
            strange_mesons::K_0_star_1430_0 => "K_0^*(1430)^0",
            strange_mesons::K_0_star_1430_plus => "K_0^*(1430)^+",
            strange_mesons::K_1460_0 => "K(1460)^0",
            strange_mesons::K_1460_plus => "K(1460)^+",
            strange_mesons::K_1830_0 => "K(1830)^0",
            strange_mesons::K_1830_plus => "K(1830)^+",
            strange_mesons::K_0_star_1950_0 => "K_0^*(1950)^0",
            strange_mesons::K_0_star_1950_plus => "K_0^*(1950)^+",
            strange_mesons::K_star_892_0 => "K^*(892)^0",
            strange_mesons::K_star_892_plus => "K^*(892)^+",
            strange_mesons::K_1_1270_0 => "K_1(1270)^0",
            strange_mesons::K_1_1270_plus => "K_1(1270)^+",
            strange_mesons::K_1_1400_0 => "K_1(1400)^0",
            strange_mesons::K_1_1400_plus => "K_1(1400)^+",
            strange_mesons::K_star_1410_0 => "K^*(1410)^0",
            strange_mesons::K_star_1410_plus => "K^*(1410)^+",
            strange_mesons::K_1_1650_0 => "K_1(1650)^0",
            strange_mesons::K_1_1650_plus => "K_1(1650)^+",
            strange_mesons::K_star_1680_0 => "K^*(1680)^0",
            strange_mesons::K_star_1680_plus => "K^*(1680)^+",
            strange_mesons::K_2_star_1430_0 => "K_2^*(1430)^0",
            strange_mesons::K_2_star_1430_plus => "K_2^*(1430)^+",
            strange_mesons::K_2_1580_0 => "K_2(1580)^0",
            strange_mesons::K_2_1580_plus => "K_2(1580)^+",
            strange_mesons::K_2_1770_0 => "K_2(1770)^0",
            strange_mesons::K_2_1770_plus => "K_2(1770)^+",
            strange_mesons::K_2_1820_0 => "K_2(1820)^0",
            strange_mesons::K_2_1820_plus => "K_2(1820)^+",
            strange_mesons::K_2_star_1980_0 => "K_2^*(1980)^0",
            strange_mesons::K_2_star_1980_plus => "K_2^*(1980)^+",
            strange_mesons::K_2_2250_0 => "K_2(2250)^0",
            strange_mesons::K_2_2250_plus => "K_2(2250)^+",
            strange_mesons::K_3_star_1780_0 => "K_3^*(1780)^0",
            strange_mesons::K_3_star_1780_plus => "K_3^*(1780)^+",
            strange_mesons::K_3_2320_0 => "K_3(2320)^0",
            strange_mesons::K_3_2320_plus => "K_3(2320)^+",
            strange_mesons::K_4_star_2045_0 => "K_4^*(2045)^0",
            strange_mesons::K_4_star_2045_plus => "K_4^*(2045)^+",
            strange_mesons::K_4_2500_0 => "K_4(2500)^0",
            strange_mesons::K_4_2500_plus => "K_4(2500)^+",

            charmed_mesons::D_plus => "D^+",
            charmed_mesons::D_0 => "D^0",
            charmed_mesons::D_0_star_2400_plus => "D_0^*(2400)^+",
            charmed_mesons::D_0_star_2400_0 => "D_0^*(2400)^0",
            charmed_mesons::D_star_2010_plus => "D^*(2010)^+",
            charmed_mesons::D_star_2007_0 => "D^*(2010)^0",
            charmed_mesons::D_1_2420_plus => "D_1(2420)^+",
            charmed_mesons::D_1_2420_0 => "D_1(2420)^0",
            charmed_mesons::D_1_H_plus => "D_1(H)^+",
            charmed_mesons::D_1_2430_0 => "D_1(2430)^0",
            charmed_mesons::D_2_star_2460_plus => "D_2(2460)^+",
            charmed_mesons::D_2_star_2460_0 => "D_2(2460)^0",
            charmed_mesons::D_s_plus => "D_s^+",
            charmed_mesons::D_s0_star_2317_plus => "D_{s0}^*(2317)^+",
            charmed_mesons::D_s_star_plus => "D_s^{*+}",
            charmed_mesons::D_s_1_2536_plus => "D_{s1}(2536)^+",
            charmed_mesons::D_s_1_2460_plus => "D_{s1}(2460)^+",
            charmed_mesons::D_s_2_star_2573_plus => "D_{s2}^*(2573)^+",

            bottom_mesons::B_0 => "B^0",
            bottom_mesons::B_plus => "B^+",
            bottom_mesons::B_0_star_0 => "B_0^{*0}",
            bottom_mesons::B_0_star_plus => "B_0^{*+}",
            bottom_mesons::B_star_0 => "B^{*0}",
            bottom_mesons::B_star_plus => "B^{*+}",
            bottom_mesons::B_1_L_0 => "B_1(L)^0",
            bottom_mesons::B_1_L_plus => "B_1(L)^+",
            bottom_mesons::B_1_H_0 => "B_1(H)^0",
            bottom_mesons::B_1_H_plus => "B_1(L)^+",
            bottom_mesons::B_2_star0 => "B_2^{*0}",
            bottom_mesons::B_2_star_plus => "B_2^{*+}",
            bottom_mesons::B_s_0 => "B_s^0",
            bottom_mesons::B_s_0_star_0 => "B_{s0}^{*0}",
            bottom_mesons::B_s_star_0 => "B_s^{*0}",
            bottom_mesons::B_s_1_L_0 => "B_{s1}(L)^0",
            bottom_mesons::B_s_1_H_0 => "B_{s1}(H)^0",
            bottom_mesons::B_s_2_star_0 => "B_{s2}^{*0}",
            bottom_mesons::B_c_plus => "B_c^+",
            bottom_mesons::B_c_0_star_plus => "B_{c0}^{*+}",
            bottom_mesons::B_c_star_plus => "B_c^{*+}",
            bottom_mesons::B_c_1_L_plus => "B_{c1}(L)^+",
            bottom_mesons::B_c_1_H_plus => "B_{c1}(H)^+",
            bottom_mesons::B_c_2_star_plus => "B_{c2}^{*+}",

            ccbar_mesons::η_c_1S => r"\eta_c(1S)",
            ccbar_mesons::χ_c_0_1P => r"\chi_{c0}(1P)",
            ccbar_mesons::η_c_2S => r"\eta_c(2S)",
            ccbar_mesons::Jψ_1S => r"J/\psi(1S)",
            ccbar_mesons::h_c_1P => r"h_c(1P)",
            ccbar_mesons::χ_c_1_1P => r"\chi_{c1}(1P)",
            ccbar_mesons::ψ_2S => r"\psi(2S)",
            ccbar_mesons::ψ_3770 => r"\psi(3770)",
            ccbar_mesons::ψ_4040 => r"\psi(4040)",
            ccbar_mesons::ψ_4160 => r"\psi(4160)",
            ccbar_mesons::ψ_4415 => r"\psi(4415)",
            ccbar_mesons::χ_c_2_1P => r"\chi_{c2}(1P)",
            ccbar_mesons::χ_c_2_3930 => r"\chi_{c2}(3930)",

            bbbar_mesons::η_b_1S => r"\eta_b(1S)",
            bbbar_mesons::χ_b_0_1P => r"\chi_{b0}(1P)",
            bbbar_mesons::η_b_2S => r"\eta_b(2S)",
            bbbar_mesons::χ_b_0_2P => r"\chi_{b0}(2P)",
            bbbar_mesons::η_b_3S => r"\eta_b(3S)",
            bbbar_mesons::χ_b_0_3P => r"\chi_{b0}(3P)",
            bbbar_mesons::Υ_1S => r"\Upsilon(1S)",
            bbbar_mesons::h_b_1P => r"h_b(1P)",
            bbbar_mesons::χ_b_1_1P => r"\chi_b",
            bbbar_mesons::Υ_1_1D => r"\Upsilon_1(1D)",
            bbbar_mesons::Υ_2S => r"\Upsilon(2S)",
            bbbar_mesons::h_b_2P => r"h_b(2P)",
            bbbar_mesons::χ_b_1_2P => r"\chi_{b1}(2P)",
            bbbar_mesons::Υ_1_2D => r"\Upsilon_1(2D)",
            bbbar_mesons::Υ_3S => r"\Upsilon(3S)",
            bbbar_mesons::h_b_3P => r"h_b(3P)",
            bbbar_mesons::χ_b_1_3P => r"\chi_{b1}(3P)",
            bbbar_mesons::Υ_4S => r"\Upsilon(4S)",
            bbbar_mesons::Υ_10860 => r"\Upsilon(10860)",
            bbbar_mesons::Υ_11020 => r"\Upsilon(11020)",
            bbbar_mesons::χ_b_2_1P => r"\chi_{b2}(1P)",
            bbbar_mesons::η_b_2_1D => r"\eta_{b2}(1P)",
            bbbar_mesons::Υ_2_1D => r"\Upsilon_2(1D)",
            bbbar_mesons::χ_b_2_2P => r"\chi_{b2}(2P)",
            bbbar_mesons::η_b_2_2D => r"\eta_{b2}(2D)",
            bbbar_mesons::Υ_2_2D => r"\Upsilon_2(2D)",
            bbbar_mesons::χ_b_2_3P => r"\chi_{b2}(3P)",
            bbbar_mesons::Υ_3_1D => r"\Upsilon_3(1D)",
            bbbar_mesons::Υ_3_2D => r"\Upsilon_3(2D)",

            light_baryons::Δ_plus_plus => r"\Delta^{++}",
            light_baryons::Δ_plus => r"\Delta^+",
            light_baryons::Δ_0 => r"\Delta^0",
            light_baryons::Δ_minus => r"\Delta^-",

            strange_baryons::Λ => r"\Lambda",
            strange_baryons::Σ_plus => r"\Sigma^+",
            strange_baryons::Σ_0 => r"\Sigma^0",
            strange_baryons::Σ_minus => r"\Sigma^-",
            strange_baryons::Σ_star_plus => r"\Sigma^{*+}",
            strange_baryons::Σ_star_0 => r"\Sigma^{*0}",
            strange_baryons::Σ_star_minus => r"\Sigma^{*-}",
            strange_baryons::Ξ_0 => r"\Xi^0",
            strange_baryons::Ξ_minus => r"\Xi^-",
            strange_baryons::Ξ_star_0 => r"\Xi^{*0}",
            strange_baryons::Ξ_star_minus => r"\Xi^{*-}",
            strange_baryons::Ω_minus => r"\Omega^-",

            charmed_baryons::Λ_c_plus => r"\Lambda_c^+",
            charmed_baryons::Σ_c_plus_plus => r"\Sigma_c^{++}",
            charmed_baryons::Σ_c_plus => r"\Sigma_c^+",
            charmed_baryons::Σ_c_0 => r"\Sigma_c^0",
            charmed_baryons::Σ_c_star_plus_plus => r"\Sigma_c^{*++}",
            charmed_baryons::Σ_c_star_plus => r"\Sigma_c^{*+}",
            charmed_baryons::Σ_c_star_0 => r"\Sigma_c^{*0}",
            charmed_baryons::Ξ_c_plus => r"\Xi_c^+",
            charmed_baryons::Ξ_c_0 => r"\Xi_c^0",
            charmed_baryons::Ξ_c_prime_plus => r"\Xi_c'^+",
            charmed_baryons::Ξ_c_prime_0 => r"\Xi_c'^0",
            charmed_baryons::Ξ_c_star_plus => r"\Xi_c^{*+}",
            charmed_baryons::Ξ_c_star_0 => r"\Xi_c^{*0}",
            charmed_baryons::Ω_c_0 => r"\Omega_c^0",
            charmed_baryons::Ω_c_star_0 => r"\Omega_c^{*0}",
            charmed_baryons::Ξ_c_c_plus => r"\Xi_{cc}^+",
            charmed_baryons::Ξ_c_c_plus_plus => r"\Xi_{cc}^{++}",
            charmed_baryons::Ξ_c_c_star_plus => r"\Xi_{cc}^{*+}",
            charmed_baryons::Ξ_c_c_star_plus_plus => r"\Xi_{cc}^{*++}",
            charmed_baryons::Ω_c_c_plus => r"\Omega_{cc}^+",
            charmed_baryons::Ω_c_c_star_plus => r"\Omega_{cc}^{*+}",
            charmed_baryons::Ω_c_c_c_plus_plus => r"\Omega_{ccc}^{++}",

            bottom_baryons::Λ_b_0 => r"\Lambda_b^0",
            bottom_baryons::Σ_b_minus => r"\Sigma_b^-",
            bottom_baryons::Σ_b_0 => r"\Sigma_b^0",
            bottom_baryons::Σ_b_plus => r"\Sigma_b^+",
            bottom_baryons::Σ_b_star_minus => r"\Sigma_b^{*-}",
            bottom_baryons::Σ_b_star_0 => r"\Sigma_b^0",
            bottom_baryons::Σ_b_star_plus => r"\Sigma_b^+",
            bottom_baryons::Ξ_b_minus => r"\Xi_b^-",
            bottom_baryons::Ξ_b_0 => r"\Xi_b^0",
            bottom_baryons::Ξ_b_prime_minus => r"\Xi_b'^-",
            bottom_baryons::Ξ_b_prime_0 => r"\Xi_b^0",
            bottom_baryons::Ξ_b_star_minus => r"\Xi_b^-",
            bottom_baryons::Ξ_b_star_0 => r"\Xi_b^0",
            bottom_baryons::Ω_b_minus => r"\Omega_b^-",
            bottom_baryons::Ω_b_star_minus => r"\Omega_b^{*-}",
            bottom_baryons::Ξ_b_c_0 => r"\Xi_{bc}^0",
            bottom_baryons::Ξ_b_c_plus => r"\Xi_{bc}^+",
            bottom_baryons::Ξ_b_c_prime_0 => r"\Xi_{bc}'^0",
            bottom_baryons::Ξ_b_c_prime_plus => r"\Xi_{bc}'^+",
            bottom_baryons::Ξ_b_c_star_0 => r"\Xi_{bc}^{*0}",
            bottom_baryons::Ξ_b_c_star_plus => r"\Xi_{bc}^{*+}",
            bottom_baryons::Ω_b_c_0 => r"\Omega_{bc}^0",
            bottom_baryons::Ω_b_c_prime_0 => r"\Omega_{bc}'^0",
            bottom_baryons::Ω_b_c_star_0 => r"\Omega_{bc}^{*0}",
            bottom_baryons::Ω_b_c_c_plus => r"\Omega_{bcc}^+",
            bottom_baryons::Ω_b_c_c_star_plus => r"\Omega_{bcc}^{*+}",
            bottom_baryons::Ξ_b_b_minus => r"\Xi_{bb}^-",
            bottom_baryons::Ξ_b_b_0 => r"\Xi_{bb}^0",
            bottom_baryons::Ξ_b_b_star_minus => r"\Xi_{bb}^{*-}",
            bottom_baryons::Ξ_b_b_star_0 => r"\Xi_{bb}^{*-}",
            bottom_baryons::Ω_b_b_minus => r"\Omega_{bb}^-",
            bottom_baryons::Ω_b_b_star_minus => r"\Omega_{bb}^{*-}",
            bottom_baryons::Ω_b_b_c_0 => r"\Omega_{bbc}^0",
            bottom_baryons::Ω_b_b_c_star_0 => r"\Omega_{bbc}^{*0}",
            bottom_baryons::Ω_b_b_b_minus => r"\Omega_{bbb}^-",

            pentaquarks::Θ_plus => r"\Theta^+",
            pentaquarks::Φ_minus_minus => r"\Phi^{--}",

            _ => return None,
        };
        Some(name)
    }

    /// Particle symbol in UTF-8 format
    pub const fn symbol(&self) -> Option<&'static str> {
        // TODO: many missing
        use sm_elementary_particles::*;
        use light_baryons::*;
        use light_anti_baryons::*;

        let name = match *self {
            d => "d",
            u => "u",
            s => "s",
            c => "c",
            b => "b",
            t => "t",
            b_prime => "b'",
            t_prime => "t'",
            e => "e",
            ν_e => "νₑ",
            μ => "μ",
            ν_μ => "ν(μ)",
            τ => "τ",
            ν_τ => "ν(τ)",
            τ_prime => "τ'",
            ν_τ_prime => "ν(τ)",
            g => "g",
            γ => "γ",
            Z => "Z",
            W_plus => "W⁺",
            h => "h",
            Z_prime => "Z'",
            Z_prime_prime => "Z''",
            W_prime => "W'",
            H0 => "H⁰",
            A0 => "A⁰",
            H_plus  => "H⁺",
            H_plus_plus => "H⁺⁺",
            a0 => "a₋",
            p => "p",
            n => "n",

            d_bar => r" ̅d",
            u_bar => r" ̅u",
            s_bar => r" ̅s",
            c_bar => r" ̅c",
            b_bar => r" ̅b",
            t_bar => r" ̅t",
            b_prime_bar => r" ̅b'",
            t_prime_bar => r" ̅t'",
            e_bar => "e⁺",
            ν_e_bar => " ̅νₑ",
            μ_bar => "μ⁺",
            ν_μ_bar => " ̅ν(μ)",
            τ_bar => "τ⁺",
            ν_τ_bar => " ̅ν(τ)",
            τ_prime_bar => "τ'⁺",
            ν_τ_prime_bar => " ̅ν(τ')",
            W_minus => "W⁻",
            W_prime_minus => "W⁻⁻",
            H_minus => "H⁻",
            H_minus_minus => "H⁻⁻",
            anti_proton => " ̅p",
            anti_neutron => " ̅n",

            light_Ieq1_mesons::π_0 => r"π⁰",
            light_Ieq1_mesons::π_plus => r"π⁺",
            light_Ieq1_mesons::a_0_980_0 => r"a₀(980)⁰",
            light_Ieq1_mesons::a_0_980_plus => r"a₀(980)⁺",
            light_Ieq1_mesons::π_1300_0 => r"π(1300)⁰",
            light_Ieq1_mesons::π_1300_plus => r"π(1300)⁺",
            light_Ieq1_mesons::a_0_1450_0 => r"a₀(1450)⁰",
            light_Ieq1_mesons::a_0_1450_plus => r"a₀(1450)⁺",
            light_Ieq1_mesons::π_1800_0 => r"π(1800)⁰",
            light_Ieq1_mesons::π_1800_plus => r"π(1800)⁺",
            light_Ieq1_mesons::ρ_770_0 => r"ρ(770)⁰",
            light_Ieq1_mesons::ρ_770_plus => r"ρ(770)⁺",
            light_Ieq1_mesons::b_1_1235_0 => r"b₁(1235)⁰",
            light_Ieq1_mesons::b_1_1235_plus => r"b₁(1235)⁺",
            light_Ieq1_mesons::a_1_1260_0 => r"a₁(1260)⁰",
            light_Ieq1_mesons::a_1_1260_plus => r"a₁(1260)⁺",
            light_Ieq1_mesons::π_1_1400_0 => r"π₁(1400)⁰",
            light_Ieq1_mesons::π_1_1400_plus => r"π₁(1400)⁺",
            light_Ieq1_mesons::ρ_1450_0 => r"ρ(1450)⁰",
            light_Ieq1_mesons::ρ_1450_plus => r"ρ(1450)⁺",
            light_Ieq1_mesons::π_1_1600_0 => r"π₁(1600)⁰",
            light_Ieq1_mesons::π_1_1600_plus => r"π₁(1600)⁺",
            light_Ieq1_mesons::a_1_1640_0 => r"a₁(1640)⁰",
            light_Ieq1_mesons::a_1_1640_plus => r"a₁(1640)⁺",
            light_Ieq1_mesons::ρ_1700_0 => r"ρ(1700)⁰",
            light_Ieq1_mesons::ρ_1700_plus => r"ρ(1700)⁺",
            light_Ieq1_mesons::ρ_1900_0 => r"ρ(1900)⁰",
            light_Ieq1_mesons::ρ_1900_plus => r"ρ(1900)⁺",
            light_Ieq1_mesons::ρ_2150_0 => r"ρ(2150)⁰",
            light_Ieq1_mesons::ρ_2150_plus => r"ρ(2150)⁺",
            light_Ieq1_mesons::a_2_1320_0 => r"a₂(1320)⁰",
            light_Ieq1_mesons::a_2_1320_plus => r"a₂(1320)⁺",
            light_Ieq1_mesons::π_2_1670_0 => r"π₂(1670)⁰",
            light_Ieq1_mesons::π_2_1670_plus => r"π₂(1670)⁺",
            light_Ieq1_mesons::a_2_1700_0 => r"a₂(1700)⁰",
            light_Ieq1_mesons::a_2_1700_plus => r"a₂(1700)⁺",
            light_Ieq1_mesons::π_2_2100_0 => r"π₂(2100)⁰",
            light_Ieq1_mesons::π_2_2100_plus => r"π₂(2100)⁺",
            light_Ieq1_mesons::ρ_3_1690_0 => r"ρ₃(1690)⁰",
            light_Ieq1_mesons::ρ_3_1690_plus => r"ρ₃(1690)⁺",
            light_Ieq1_mesons::ρ_3_1990_0 => r"ρ₃(1990)⁰",
            light_Ieq1_mesons::ρ_3_1990_plus => r"ρ₃(1990)⁺",
            light_Ieq1_mesons::ρ_3_2250_0 => r"ρ₃(2250)⁰",
            light_Ieq1_mesons::ρ_3_2250_plus => r"ρ₃(2250)⁺",
            light_Ieq1_mesons::a_4_2040_0 => r"a₄(2040)⁰",
            light_Ieq1_mesons::a_4_2040_plus => r"a₄(2040)⁺",

            light_Ieq0_mesons::η => r"η",
            light_Ieq0_mesons::η_prime_958 => r"η'(958)",
            light_Ieq0_mesons::f_0_500 => "f₀(500)",
            light_Ieq0_mesons::f_0_980 => "f₀(980)",
            light_Ieq0_mesons::η_1295 => r"η(1295)",
            light_Ieq0_mesons::f_0_1370 => "f₀(1370)",
            light_Ieq0_mesons::η_1405 => r"η(1405)",
            light_Ieq0_mesons::η_1475 => r"η(1475)",
            light_Ieq0_mesons::f_0_1500 => "f₀(1500)",
            light_Ieq0_mesons::f_0_1710 => "f₀(1710)",
            light_Ieq0_mesons::η_1760 => r"η(1760)",
            light_Ieq0_mesons::f_0_2020 => "f₀(2020)",
            light_Ieq0_mesons::f_0_2100 => "f₀(2100)",
            light_Ieq0_mesons::f_0_2200 => "f₀(2200)",
            light_Ieq0_mesons::η_2225 => r"η(2225)",
            light_Ieq0_mesons::ω_782 => r"ω(782)",
            light_Ieq0_mesons::φ_1020 => r"φ(1020)",
            light_Ieq0_mesons::h_1_1170 => r"h₁(1170)",
            light_Ieq0_mesons::f_1_1285 => "f₁(1285)",
            light_Ieq0_mesons::h_1_1380 => r"h₁(1380)",
            light_Ieq0_mesons::f_1_1420 => "f₁(1420)",
            light_Ieq0_mesons::ω_1420 => r"ω(1420)",
            light_Ieq0_mesons::f_1_1510 => "f₁(1510)",
            light_Ieq0_mesons::h_1_1595 => r"h₁(1595)",
            light_Ieq0_mesons::ω_1650 => r"ω(1650)",
            light_Ieq0_mesons::φ_1680 => r"φ(1680)",
            light_Ieq0_mesons::f_2_1270 => "f₂(1270)",
            light_Ieq0_mesons::f_2_1430 => "f₂(1430)",
            light_Ieq0_mesons::f_2_prime_1525 => "f₂'(1525)",
            light_Ieq0_mesons::f_2_1565 => "f₂(1565)",
            light_Ieq0_mesons::f_2_1640 => "f₂(1640)",
            light_Ieq0_mesons::η_2_1645 => r"η₂(1645)",
            light_Ieq0_mesons::f_2_1810 => "f₂(1810)",
            light_Ieq0_mesons::η_2_1870 => r"η₂(1870)",
            light_Ieq0_mesons::f_2_1910 => "f₂(1910)",
            light_Ieq0_mesons::f_2_1950 => "f₂(1950)",
            light_Ieq0_mesons::f_2_2010 => "f₂(2010)",
            light_Ieq0_mesons::f_2_2150 => "f₂(2150)",
            light_Ieq0_mesons::f_2_2300 => "f₂(2300)",
            light_Ieq0_mesons::f_2_2340 => "f₂(2340)",
            light_Ieq0_mesons::ω_3_1670 => r"ω₃(1670)",
            light_Ieq0_mesons::φ_3_1850 => r"φ₃(1850)",
            light_Ieq0_mesons::f_4_2050 => "f₄(2050)",
            light_Ieq0_mesons::f_J_2220 => "f(J)(2220)",
            light_Ieq0_mesons::f_4_2300 => "f₄(2300)",

            strange_mesons::K_0_L=> "K⁰(L)",
            strange_mesons::K_0_S => "K⁰(S)",
            strange_mesons::K_0 => "K⁰",
            strange_mesons::K_plus => "K⁺",
            strange_mesons::K_0_star_700_0 => "K₀⃰(700)⁰",
            strange_mesons::K_0_star_700_plus => "K₀⃰(700)⁺",
            strange_mesons::K_0_star_1430_0 => "K₀⃰(1430)⁰",
            strange_mesons::K_0_star_1430_plus => "K₀⃰(1430)⁺",
            strange_mesons::K_1460_0 => "K(1460)⁰",
            strange_mesons::K_1460_plus => "K(1460)⁺",
            strange_mesons::K_1830_0 => "K(1830)⁰",
            strange_mesons::K_1830_plus => "K(1830)⁺",
            strange_mesons::K_0_star_1950_0 => "K₀⃰(1950)⁰",
            strange_mesons::K_0_star_1950_plus => "K₀⃰(1950)⁺",
            strange_mesons::K_star_892_0 => "K⃰(892)⁰",
            strange_mesons::K_star_892_plus => "K⃰(892)⁺",
            strange_mesons::K_1_1270_0 => "K₁(1270)⁰",
            strange_mesons::K_1_1270_plus => "K₁(1270)⁺",
            strange_mesons::K_1_1400_0 => "K₁(1400)⁰",
            strange_mesons::K_1_1400_plus => "K₁(1400)⁺",
            strange_mesons::K_star_1410_0 => "K⃰(1410)⁰",
            strange_mesons::K_star_1410_plus => "K⃰(1410)⁺",
            strange_mesons::K_1_1650_0 => "K₁(1650)⁰",
            strange_mesons::K_1_1650_plus => "K₁(1650)⁺",
            strange_mesons::K_star_1680_0 => "K⃰(1680)⁰",
            strange_mesons::K_star_1680_plus => "K⃰(1680)⁺",
            strange_mesons::K_2_star_1430_0 => "K₂⃰(1430)⁰",
            strange_mesons::K_2_star_1430_plus => "K₂⃰(1430)⁺",
            strange_mesons::K_2_1580_0 => "K₂(1580)⁰",
            strange_mesons::K_2_1580_plus => "K₂(1580)⁺",
            strange_mesons::K_2_1770_0 => "K₂(1770)⁰",
            strange_mesons::K_2_1770_plus => "K₂(1770)⁺",
            strange_mesons::K_2_1820_0 => "K₂(1820)⁰",
            strange_mesons::K_2_1820_plus => "K₂(1820)⁺",
            strange_mesons::K_2_star_1980_0 => "K₂⃰(1980)⁰",
            strange_mesons::K_2_star_1980_plus => "K₂⃰(1980)⁺",
            strange_mesons::K_2_2250_0 => "K₂(2250)⁰",
            strange_mesons::K_2_2250_plus => "K₂(2250)⁺",
            strange_mesons::K_3_star_1780_0 => "K₃⃰(1780)⁰",
            strange_mesons::K_3_star_1780_plus => "K₃⃰(1780)⁺",
            strange_mesons::K_3_2320_0 => "K₃(2320)⁰",
            strange_mesons::K_3_2320_plus => "K₃(2320)⁺",
            strange_mesons::K_4_star_2045_0 => "K₄⃰(2045)⁰",
            strange_mesons::K_4_star_2045_plus => "K₄⃰(2045)⁺",
            strange_mesons::K_4_2500_0 => "K₄(2500)⁰",
            strange_mesons::K_4_2500_plus => "K₄(2500)⁺",

            charmed_mesons::D_plus => "D⁺",
            charmed_mesons::D_0 => "D⁰",
            charmed_mesons::D_0_star_2400_plus => "D₀⃰(2400)⁺",
            charmed_mesons::D_0_star_2400_0 => "D₀⃰(2400)⁰",
            charmed_mesons::D_star_2010_plus => "D⃰(2010)⁺",
            charmed_mesons::D_star_2007_0 => "D⃰(2010)⁰",
            charmed_mesons::D_1_2420_plus => "D₁(2420)⁺",
            charmed_mesons::D_1_2420_0 => "D₁(2420)⁰",
            charmed_mesons::D_1_H_plus => "D₁(H)⁺",
            charmed_mesons::D_1_2430_0 => "D₁(2430)⁰",
            charmed_mesons::D_2_star_2460_plus => "D₂(2460)⁺",
            charmed_mesons::D_2_star_2460_0 => "D₂(2460)⁰",
            charmed_mesons::D_s_plus => "Dₛ⁺",
            charmed_mesons::D_s0_star_2317_plus => "Dₛ₀⃰(2317)⁺",
            charmed_mesons::D_s_star_plus => "Dₛ⃰⁺",
            charmed_mesons::D_s_1_2536_plus => "Dₛ₁(2536)⁺",
            charmed_mesons::D_s_1_2460_plus => "Dₛ₁(2460)⁺",
            charmed_mesons::D_s_2_star_2573_plus => "Dₛ₂⃰(2573)⁺",

            bottom_mesons::B_0 => "B⁰",
            bottom_mesons::B_plus => "B⁺",
            bottom_mesons::B_0_star_0 => "B₀⃰⁰",
            bottom_mesons::B_0_star_plus => "B₀⃰⁺",
            bottom_mesons::B_star_0 => "B⃰⁰",
            bottom_mesons::B_star_plus => "B⃰⁺",
            bottom_mesons::B_1_L_0 => "B₁(L)⁰",
            bottom_mesons::B_1_L_plus => "B₁(L)⁺",
            bottom_mesons::B_1_H_0 => "B₁(H)⁰",
            bottom_mesons::B_1_H_plus => "B₁(L)⁺",
            bottom_mesons::B_2_star0 => "B₂⃰⁰",
            bottom_mesons::B_2_star_plus => "B₂⃰⁺",
            bottom_mesons::B_s_0 => "Bₛ⁰",
            bottom_mesons::B_s_0_star_0 => "Bₛ₀⃰⁰",
            bottom_mesons::B_s_star_0 => "Bₛ⃰⁰",
            bottom_mesons::B_s_1_L_0 => "Bₛ₁(L)⁰",
            bottom_mesons::B_s_1_H_0 => "Bₛ₁(H)⁰",
            bottom_mesons::B_s_2_star_0 => "Bₛ₂⃰⁰",
            bottom_mesons::B_c_plus => "B(c)⁺",
            bottom_mesons::B_c_0_star_plus => "B(c)₀⃰⁺",
            bottom_mesons::B_c_star_plus => "B(c)⃰⁺",
            bottom_mesons::B_c_1_L_plus => "B(c)₁(L)⁺",
            bottom_mesons::B_c_1_H_plus => "B(c)₁(H)⁺",
            bottom_mesons::B_c_2_star_plus => "B(c)₂⃰⁺",

            ccbar_mesons::η_c_1S => r"η(c)(1S)",
            ccbar_mesons::χ_c_0_1P => r"χ(c)₀(1P)",
            ccbar_mesons::η_c_2S => r"η(c)(2S)",
            ccbar_mesons::Jψ_1S => r"J/ψ(1S)",
            ccbar_mesons::h_c_1P => r"h(c)(1P)",
            ccbar_mesons::χ_c_1_1P => r"χ(c)₁(1P)",
            ccbar_mesons::ψ_2S => r"ψ(2S)",
            ccbar_mesons::ψ_3770 => r"ψ(3770)",
            ccbar_mesons::ψ_4040 => r"ψ(4040)",
            ccbar_mesons::ψ_4160 => r"ψ(4160)",
            ccbar_mesons::ψ_4415 => r"ψ(4415)",
            ccbar_mesons::χ_c_2_1P => r"χ(c)₂(1P)",
            ccbar_mesons::χ_c_2_3930 => r"χ(c)₂(3930)",

            bbbar_mesons::η_b_1S => r"η(b)(1S)",
            bbbar_mesons::χ_b_0_1P => r"χ(b)₀(1P)",
            bbbar_mesons::η_b_2S => r"η(b)(2S)",
            bbbar_mesons::χ_b_0_2P => r"χ(b)₀(2P)",
            bbbar_mesons::η_b_3S => r"η(b)(3S)",
            bbbar_mesons::χ_b_0_3P => r"χ(b)₀(3P)",
            bbbar_mesons::Υ_1S => r"Υ(1S)",
            bbbar_mesons::h_b_1P => r"h(b)(1P)",
            bbbar_mesons::χ_b_1_1P => r"χ(b)",
            bbbar_mesons::Υ_1_1D => r"Υ₁(1D)",
            bbbar_mesons::Υ_2S => r"Υ(2S)",
            bbbar_mesons::h_b_2P => r"h(b)(2P)",
            bbbar_mesons::χ_b_1_2P => r"χ(b)₁(2P)",
            bbbar_mesons::Υ_1_2D => r"Υ₁(2D)",
            bbbar_mesons::Υ_3S => r"Υ(3S)",
            bbbar_mesons::h_b_3P => r"h(b)(3P)",
            bbbar_mesons::χ_b_1_3P => r"χ(b)₁(3P)",
            bbbar_mesons::Υ_4S => r"Υ(4S)",
            bbbar_mesons::Υ_10860 => r"Υ(10860)",
            bbbar_mesons::Υ_11020 => r"Υ(11020)",
            bbbar_mesons::χ_b_2_1P => r"χ(b)₂(1P)",
            bbbar_mesons::η_b_2_1D => r"η(b)₂(1P)",
            bbbar_mesons::Υ_2_1D => r"Υ₂(1D)",
            bbbar_mesons::χ_b_2_2P => r"χ(b)₂(2P)",
            bbbar_mesons::η_b_2_2D => r"η(b)₂(2D)",
            bbbar_mesons::Υ_2_2D => r"Υ₂(2D)",
            bbbar_mesons::χ_b_2_3P => r"χ(b)₂(3P)",
            bbbar_mesons::Υ_3_1D => r"Υ₃(1D)",
            bbbar_mesons::Υ_3_2D => r"Υ₃(2D)",

            light_baryons::Δ_plus_plus => r"Δ⁺⁺",
            light_baryons::Δ_plus => r"Δ⁺",
            light_baryons::Δ_0 => r"Δ⁰",
            light_baryons::Δ_minus => r"Δ⁻",

            strange_baryons::Λ => r"Λ",
            strange_baryons::Σ_plus => r"Σ⁺",
            strange_baryons::Σ_0 => r"Σ⁰",
            strange_baryons::Σ_minus => r"Σ⁻",
            strange_baryons::Σ_star_plus => r"Σ⃰⁺",
            strange_baryons::Σ_star_0 => r"Σ⃰⁰",
            strange_baryons::Σ_star_minus => r"Σ⃰⁻",
            strange_baryons::Ξ_0 => r"Ξ⁰",
            strange_baryons::Ξ_minus => r"Ξ⁻",
            strange_baryons::Ξ_star_0 => r"Ξ⃰⁰",
            strange_baryons::Ξ_star_minus => r"Ξ⃰⁻",
            strange_baryons::Ω_minus => r"Ω⁻",

            charmed_baryons::Λ_c_plus => r"Λ(c)⁺",
            charmed_baryons::Σ_c_plus_plus => r"Σ(c)⁺⁺",
            charmed_baryons::Σ_c_plus => r"Σ(c)⁺",
            charmed_baryons::Σ_c_0 => r"Σ(c)⁰",
            charmed_baryons::Σ_c_star_plus_plus => r"Σ(c)⃰⁺⁺",
            charmed_baryons::Σ_c_star_plus => r"Σ(c)⃰⁺",
            charmed_baryons::Σ_c_star_0 => r"Σ(c)⃰⁰",
            charmed_baryons::Ξ_c_plus => r"Ξ(c)⁺",
            charmed_baryons::Ξ_c_0 => r"Ξ(c)⁰",
            charmed_baryons::Ξ_c_prime_plus => r"Ξ(c)'⁺",
            charmed_baryons::Ξ_c_prime_0 => r"Ξ(c)'⁰",
            charmed_baryons::Ξ_c_star_plus => r"Ξ(c)⃰⁺",
            charmed_baryons::Ξ_c_star_0 => r"Ξ(c)⃰⁰",
            charmed_baryons::Ω_c_0 => r"Ω(c)⁰",
            charmed_baryons::Ω_c_star_0 => r"Ω(c)⃰⁰",
            charmed_baryons::Ξ_c_c_plus => r"Ξ(cc)⁺",
            charmed_baryons::Ξ_c_c_plus_plus => r"Ξ(cc)⁺⁺",
            charmed_baryons::Ξ_c_c_star_plus => r"Ξ(cc)⃰⁺",
            charmed_baryons::Ξ_c_c_star_plus_plus => r"Ξ(cc)⃰⁺⁺",
            charmed_baryons::Ω_c_c_plus => r"Ω(cc)⁺",
            charmed_baryons::Ω_c_c_star_plus => r"Ω(cc)⃰⁺",
            charmed_baryons::Ω_c_c_c_plus_plus => r"Ω(ccc)⁺⁺",

            bottom_baryons::Λ_b_0 => r"Λ(b)⁰",
            bottom_baryons::Σ_b_minus => r"Σ(b)⁻",
            bottom_baryons::Σ_b_0 => r"Σ(b)⁰",
            bottom_baryons::Σ_b_plus => r"Σ(b)⁺",
            bottom_baryons::Σ_b_star_minus => r"Σ(b)⃰⁻",
            bottom_baryons::Σ_b_star_0 => r"Σ(b)⁰",
            bottom_baryons::Σ_b_star_plus => r"Σ(b)⁺",
            bottom_baryons::Ξ_b_minus => r"Ξ(b)⁻",
            bottom_baryons::Ξ_b_0 => r"Ξ(b)⁰",
            bottom_baryons::Ξ_b_prime_minus => r"Ξ(b)'⁻",
            bottom_baryons::Ξ_b_prime_0 => r"Ξ(b)⁰",
            bottom_baryons::Ξ_b_star_minus => r"Ξ(b)⁻",
            bottom_baryons::Ξ_b_star_0 => r"Ξ(b)⁰",
            bottom_baryons::Ω_b_minus => r"Ω(b)⁻",
            bottom_baryons::Ω_b_star_minus => r"Ω(b)⃰⁻",
            bottom_baryons::Ξ_b_c_0 => r"Ξ(bc)⁰",
            bottom_baryons::Ξ_b_c_plus => r"Ξ(bc)⁺",
            bottom_baryons::Ξ_b_c_prime_0 => r"Ξ(bc)'⁰",
            bottom_baryons::Ξ_b_c_prime_plus => r"Ξ(bc)'⁺",
            bottom_baryons::Ξ_b_c_star_0 => r"Ξ(bc)⃰⁰",
            bottom_baryons::Ξ_b_c_star_plus => r"Ξ(bc)⃰⁺",
            bottom_baryons::Ω_b_c_0 => r"Ω(bc)⁰",
            bottom_baryons::Ω_b_c_prime_0 => r"Ω(bc)'⁰",
            bottom_baryons::Ω_b_c_star_0 => r"Ω(bc)⃰⁰",
            bottom_baryons::Ω_b_c_c_plus => r"Ω(bcc)⁺",
            bottom_baryons::Ω_b_c_c_star_plus => r"Ω(bcc)⃰⁺",
            bottom_baryons::Ξ_b_b_minus => r"Ξ(bb)⁻",
            bottom_baryons::Ξ_b_b_0 => r"Ξ(bb)⁰",
            bottom_baryons::Ξ_b_b_star_minus => r"Ξ(bb)⃰⁻",
            bottom_baryons::Ξ_b_b_star_0 => r"Ξ(bb)⃰⁻",
            bottom_baryons::Ω_b_b_minus => r"Ω(bb)⁻",
            bottom_baryons::Ω_b_b_star_minus => r"Ω(bb)⃰⁻",
            bottom_baryons::Ω_b_b_c_0 => r"Ω(bbc)⁰",
            bottom_baryons::Ω_b_b_c_star_0 => r"Ω(bbc)⃰⁰",
            bottom_baryons::Ω_b_b_b_minus => r"Ω(bbb)⁻",

            pentaquarks::Θ_plus => r"Θ⁺",
            pentaquarks::Φ_minus_minus => r"Φ⁻⁻",

            _ => return None,
        };
        Some(name)
    }

    /// Name of the associated particle
    pub const fn name(&self) -> Option<&'static str> {
        // TODO: many missing
        use sm_elementary_particles::*;
        use light_baryons::*;
        use light_anti_baryons::*;
        let name = match *self {
            d => "down",
            u => "up",
            s => "strange",
            c => "charm",
            b => "bottom",
            t => "top",
            b_prime => "bottom prime",
            t_prime => "top prime",
            e => "electron",
            ν_e => "electron neutrino",
            μ => "muon",
            ν_μ => "muon neutrino",
            τ => "tau",
            ν_τ => "tau neutrino",
            τ_prime => "tau prime",
            ν_τ_prime => "tau prime neutrino",
            g => "gluon",
            γ => "photon",
            Z => "Z",
            W_plus => "W plus",
            h => "Higgs",
            Z_prime => "Z prime",
            Z_prime_prime => "Z prime prime",
            W_prime => "W prime",
            H0 => "heavy Higgs",
            A0 => "pseudoscalar Higgs",
            H_plus  => "Higgs plus",
            H_plus_plus => "Higgs plus plus",
            p => "proton",
            n => "neutron",

            d_bar => "anti-down",
            u_bar => "anti-up",
            s_bar => "anti-strange",
            c_bar => "anti-charm",
            b_bar => "anti-bottom",
            t_bar => "anti-top",
            b_prime_bar => "anti-bottom prime",
            t_prime_bar => "anti-top prime",
            e_bar => "positron",
            ν_e_bar => "electron anti-neutrino",
            μ_bar => "anti-muon",
            ν_μ_bar => "muon anti-neutrino",
            τ_bar => "anti-tau",
            ν_τ_bar => "tau anti-neutrino",
            τ_prime_bar => "tau prime",
            ν_τ_prime_bar => "tau prime anti-neutrino",
            W_minus => "W minus",
            W_prime_minus => "W minus minus",
            H_minus => "Higgs minus",
            H_minus_minus => "H minus minus",
            p_bar => "anti-proton",
            n_bar => "anti-neutron",

            _ => return None,
        };
        Some(name)
    }

    /// Get the corresponding integer
    ///
    /// # Example
    ///
    /// ```
    /// use particle_id::sm_elementary_particles::*;
    /// assert_eq!(photon.id(), 22);
    /// ```
    pub const fn id(self) -> i32 {
        self.0
    }

    /// Get the corresponding anti-particle
    ///
    /// # Example
    ///
    /// ```
    /// use particle_id::sm_elementary_particles::*;
    /// assert_eq!(positron.anti(), electron);
    /// assert_eq!(electron.anti(), positron);
    /// ```
    pub const fn anti(self) -> Self {
        Self(- self.0)
    }

    /// Get the corresponding particle for an anti-particle
    ///
    /// If not used on an anti-particle, returns the particle itself
    ///
    /// # Example
    ///
    /// ```
    /// use particle_id::sm_elementary_particles::*;
    /// assert_eq!(positron.abs(), electron);
    /// assert_eq!(electron.abs(), electron);
    /// ```
    pub const fn abs(self) -> Self {
        Self(self.0.abs())
    }

    /// Check if this is an anti-particle
    ///
    /// # Example
    ///
    /// ```
    /// use particle_id::sm_elementary_particles::*;
    /// assert!(!electron.is_anti_particle());
    /// assert!(positron.is_anti_particle());
    /// ```
    pub const fn is_anti_particle(&self) -> bool {
        self.0 < 0
    }

    /// Check if this is a gauge boson
    ///
    /// # Example
    ///
    /// ```
    /// use particle_id::sm_elementary_particles::*;
    /// assert!(photon.is_gauge_boson());
    /// assert!(!electron.is_gauge_boson());
    /// assert!(!Higgs.is_gauge_boson());
    /// ```
    pub const fn is_gauge_boson(&self) -> bool {
        use gauge_and_higgs_bosons::*;
        let abs_id = self.0.abs();
        gluon.id() <= abs_id && abs_id <= W_plus.id()
    }

    /// Check if this is a quark
    ///
    /// Note that anti-quarks are not treated as quarks! Use `abs()`
    /// if you want to include both quarks and anti-quarks.
    ///
    /// # Example
    ///
    /// ```
    /// use particle_id::sm_elementary_particles::*;
    /// assert!(top.is_quark());
    /// assert!(!anti_top.is_quark());
    /// assert!(anti_top.abs().is_quark());
    /// assert!(anti_top.anti().is_quark());
    /// ```
    pub const fn is_quark(&self) -> bool {
        use quarks::*;
        down.id() < self.id() && self.id() < t_prime.id()
    }

    /// Check if this is an anti-quark
    ///
    /// # Example
    ///
    /// ```
    /// use particle_id::sm_elementary_particles::*;
    /// assert!(!top.is_anti_quark());
    /// assert!(anti_top.is_anti_quark());
    /// ```
    pub const fn is_anti_quark(&self) -> bool {
        self.anti().is_quark()
    }

    /// Check if this is a lepton
    ///
    /// Note that anti-leptons are not treated as leptons! Use `abs()`
    /// if you want to include both leptons and anti-leptons.
    ///
    /// Both charged leptons and neutrinos are included.
    ///
    /// # Example
    ///
    /// ```
    /// use particle_id::sm_elementary_particles::*;
    /// assert!(muon.is_lepton());
    /// assert!(!anti_muon.is_lepton());
    /// assert!(anti_muon.abs().is_lepton());
    /// assert!(anti_muon.anti().is_lepton());
    /// ```
    pub const fn is_lepton(&self) -> bool {
        use leptons::*;
        electron.id() <= self.id() && self.id() <= tau_prime_neutrino.id()
    }

    /// Check if this is an anti-lepton
    ///
    /// # Example
    ///
    /// ```
    /// use particle_id::sm_elementary_particles::*;
    /// assert!(!muon.is_anti_lepton());
    /// assert!(anti_muon.is_anti_lepton());
    /// ```
    pub const fn is_anti_lepton(&self) -> bool {
        self.anti().is_lepton()
    }

    /// Check if this is a neutrino
    ///
    /// Note that anti-neutrinos are not treated as neutrinos! Use `abs()`
    /// if you want to include both neutrinos and anti-neutrinos.
    ///
    /// # Example
    ///
    /// ```
    /// use particle_id::sm_elementary_particles::*;
    /// assert!(muon_neutrino.is_neutrino());
    /// assert!(!muon_anti_neutrino.is_neutrino());
    /// ```
    pub const fn is_neutrino(&self) -> bool {
        self.is_lepton() && (self.id() & 1 == 0)
    }

    /// Check if this is an anti-neutrino
    ///
    /// # Example
    ///
    /// ```
    /// use particle_id::sm_elementary_particles::*;
    /// assert!(!muon_neutrino.is_anti_neutrino());
    /// assert!(muon_anti_neutrino.is_anti_neutrino());
    /// ```
    pub const fn is_anti_neutrino(&self) -> bool {
        self.anti().is_neutrino()
    }

    /// Check if this is a charged lepton
    ///
    /// Note that anti-leptons are not treated as leptons! Use `abs()`
    /// if you want to include both leptons and anti-leptons.
    ///
    /// # Example
    ///
    /// ```
    /// use particle_id::sm_elementary_particles::*;
    /// assert!(muon.is_charged_lepton());
    /// assert!(!electron_neutrino.is_charged_lepton());
    /// ```
    pub const fn is_charged_lepton(&self) -> bool {
        self.is_lepton() && (self.id() & 1 == 1)
    }

    /// Check if this is a charged anti-lepton
    ///
    /// # Example
    ///
    /// ```
    /// use particle_id::sm_elementary_particles::*;
    /// assert!(!muon.is_charged_anti_lepton());
    /// assert!(anti_muon.is_charged_anti_lepton());
    /// ```
    pub const fn is_charged_anti_lepton(&self) -> bool {
        self.is_anti_particle() && self.anti().is_charged_lepton()
    }
}

#[cfg(test)]
mod tests {
    // TODO
    // #[test]
    // fn it_works() {
    //     assert_eq!(2 + 2, 4);
    // }
}
