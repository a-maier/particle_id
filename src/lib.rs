#![allow(non_upper_case_globals)]

/// Particle ID according to the Monte Carlo Particle Numbering Scheme
///
/// See https://pdg.lbl.gov/2023/mcdata/mc_particle_id_contents.html
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ParticleID(i32);

pub mod quarks {
    use super::*;
    pub const d: ParticleID = ParticleID(1);
    pub const down: ParticleID = d;
    pub const u: ParticleID = ParticleID(2);
    pub const up: ParticleID = u;
    pub const s: ParticleID = ParticleID(3);
    pub const strange: ParticleID = s;
    pub const c: ParticleID = ParticleID(4);
    pub const charm: ParticleID = c;
    pub const b: ParticleID = ParticleID(5);
    pub const bottom: ParticleID = b;
    pub const t: ParticleID = ParticleID(6);
    pub const top: ParticleID = t;
    pub const b_prime: ParticleID = ParticleID(7);
    pub const t_prime: ParticleID = ParticleID(9);
}

pub mod leptons {
    use super::*;
    pub const e: ParticleID = ParticleID(11);
    pub const electron: ParticleID = e;
    pub const ν_e: ParticleID = ParticleID(12);
    pub const nu_e: ParticleID = ν_e;
    pub const electron_neutrino: ParticleID = ν_e;
    pub const μ: ParticleID = ParticleID(13);
    pub const mu: ParticleID = μ;
    pub const muon: ParticleID = μ;
    pub const ν_μ: ParticleID = ParticleID(14);
    pub const nu_mu: ParticleID = ν_μ;
    pub const muon_neutrino: ParticleID = ν_μ;
    pub const τ: ParticleID = ParticleID(15);
    pub const tau: ParticleID = τ;
    pub const ν_τ: ParticleID = ParticleID(16);
    pub const nu_tau: ParticleID = ν_τ;
    pub const tau_neutrino: ParticleID = ν_τ;
    pub const τ_prime: ParticleID = ParticleID(17);
    pub const tau_prime: ParticleID = τ_prime;
    pub const ν_τ_prime: ParticleID = ParticleID(18);
    pub const nu_tau_prime: ParticleID = ν_τ_prime;
    pub const tau_prime_neutrino: ParticleID = ν_τ_prime;
}

pub mod gauge_and_higgs_bosons {
    use super::*;
    pub const g: ParticleID = ParticleID(21);
    pub const gluon: ParticleID = g;
    pub const γ: ParticleID = ParticleID(22);
    pub const gamma: ParticleID = γ;
    pub const photon: ParticleID = γ;
    pub const Z: ParticleID = ParticleID(23);
    pub const W_plus: ParticleID = ParticleID(24);
    pub const h: ParticleID = ParticleID(25);
    pub const H: ParticleID = h;
    pub const Higgs: ParticleID = h;
    pub const Z_prime: ParticleID = ParticleID(32);
    pub const Z_prime_prime: ParticleID = ParticleID(33);
    pub const W_prime: ParticleID = ParticleID(34);
    pub const H0: ParticleID = ParticleID(35);
    pub const H_0: ParticleID = H0;
    pub const A0: ParticleID = ParticleID(36);
    pub const A_0: ParticleID = A0;
    pub const H_plus: ParticleID = ParticleID(37);
    pub const H_plus_plus: ParticleID = ParticleID(38);
    pub const a0: ParticleID = ParticleID(40);
    pub const a_0: ParticleID = a0;
}

pub mod special_particles {
    use super::*;
    pub const G: ParticleID = ParticleID(39);
    pub const graviton: ParticleID = G;
    pub const R_0: ParticleID = ParticleID(41);
    pub const LQ_c: ParticleID = ParticleID(42);
    pub const reggeon: ParticleID = ParticleID(110);
    pub const pomeron: ParticleID = ParticleID(990);
    pub const odderon: ParticleID = ParticleID(9990);
}

pub mod diquarks {
    use super::*;
    pub const dd_1: ParticleID = ParticleID(1103);
    pub const ud_0: ParticleID = ParticleID(2101);
    pub const ud_1: ParticleID = ParticleID(2103);
    pub const uu_1: ParticleID = ParticleID(2203);
    pub const sd_0: ParticleID = ParticleID(3101);
    pub const sd_1: ParticleID = ParticleID(3103);
    pub const su_0: ParticleID = ParticleID(3201);
    pub const su_1: ParticleID = ParticleID(3203);
    pub const ss_1: ParticleID = ParticleID(3303);
    pub const cd_0: ParticleID = ParticleID(4101);
    pub const cd_1: ParticleID = ParticleID(4103);
    pub const cu_0: ParticleID = ParticleID(4201);
    pub const cu_1: ParticleID = ParticleID(4203);
    pub const cs_0: ParticleID = ParticleID(4301);
    pub const cs_1: ParticleID = ParticleID(4303);
    pub const cc_1: ParticleID = ParticleID(4403);
    pub const bd_0: ParticleID = ParticleID(5101);
    pub const bd_1: ParticleID = ParticleID(5103);
    pub const bu_0: ParticleID = ParticleID(5201);
    pub const bu_1: ParticleID = ParticleID(5203);
    pub const bs_0: ParticleID = ParticleID(5301);
    pub const bs_1: ParticleID = ParticleID(5303);
    pub const bc_0: ParticleID = ParticleID(5401);
    pub const bc_1: ParticleID = ParticleID(5403);
    pub const bb_1: ParticleID = ParticleID(5503);
}

pub mod susy_particles {
    use super::*;
    pub const d_tilde_L: ParticleID = ParticleID(1000001);
    pub const u_tilde_L: ParticleID = ParticleID(1000002);
    pub const s_tilde_L: ParticleID = ParticleID(1000003);
    pub const c_tilde_L: ParticleID = ParticleID(1000004);
    pub const b_tilde_1: ParticleID = ParticleID(1000005);
    pub const t_tilde_1: ParticleID = ParticleID(1000006);
    pub const e_tilde_L: ParticleID = ParticleID(1000011);
    pub const ν_e_tilde_L: ParticleID = ParticleID(1000012);
    pub const μ_tilde_L: ParticleID = ParticleID(1000013);
    pub const ν_μ_tilde_L: ParticleID = ParticleID(1000014);
    pub const τ_tilde_1: ParticleID = ParticleID(1000015);
    pub const ν_τ_tilde_L: ParticleID = ParticleID(1000016);
    pub const d_tilde_R: ParticleID = ParticleID(2000001);
    pub const u_tilde_R: ParticleID = ParticleID(2000002);
    pub const s_tilde_R: ParticleID = ParticleID(2000003);
    pub const c_tilde_R: ParticleID = ParticleID(2000004);
    pub const b_tilde_2: ParticleID = ParticleID(2000005);
    pub const t_tilde_2: ParticleID = ParticleID(2000006);
    pub const e_tilde_R: ParticleID = ParticleID(2000011);
    pub const μ_tilde_R: ParticleID = ParticleID(2000013);
    pub const τ_tilde_2: ParticleID = ParticleID(2000015);
    pub const g_tilde: ParticleID = ParticleID(1000021);
    pub const χ_tilde_0_1: ParticleID = ParticleID(1000022);
    pub const χ_tilde_0_2: ParticleID = ParticleID(1000023);
    pub const χ_tilde_plus_1: ParticleID = ParticleID(1000024);
    pub const χ_tilde_0_3: ParticleID = ParticleID(1000025);
    pub const χ_tilde_0_4: ParticleID = ParticleID(1000035);
    pub const χ_tilde_plus_2: ParticleID = ParticleID(1000037);
    pub const G_tilde: ParticleID = ParticleID(1000039);
}

#[allow(non_snake_case)]
pub mod light_Ieq1_mesons {
    use super::*;
    pub const π_0: ParticleID = ParticleID(111);
    pub const π_plus: ParticleID = ParticleID(211);
    pub const a_0_980_0: ParticleID = ParticleID(9000111);
    pub const a_0_980_plus: ParticleID = ParticleID(9000211);
    pub const π_1300_0: ParticleID = ParticleID(100111);
    pub const π_1300_plus: ParticleID = ParticleID(100211);
    pub const a_0_1450_0: ParticleID = ParticleID(10111);
    pub const a_0_1450_plus: ParticleID = ParticleID(10211);
    pub const π_1800_0: ParticleID = ParticleID(9010111);
    pub const π_1800_plus: ParticleID = ParticleID(9010211);
    pub const ρ_770_0: ParticleID = ParticleID(113);
    pub const ρ_770_plus: ParticleID = ParticleID(213);
    pub const b_1_1235_0: ParticleID = ParticleID(10113);
    pub const b_1_1235_plus: ParticleID = ParticleID(10213);
    pub const a_1_1260_0: ParticleID = ParticleID(20113);
    pub const a_1_1260_plus: ParticleID = ParticleID(20213);
    pub const π_1_1400_0: ParticleID = ParticleID(9000113);
    pub const π_1_1400_plus: ParticleID = ParticleID(9000213);
    pub const ρ_1450_0: ParticleID = ParticleID(100113);
    pub const ρ_1450_plus: ParticleID = ParticleID(100213);
    pub const π_1_1600_0: ParticleID = ParticleID(9010113);
    pub const π_1_1600_plus: ParticleID = ParticleID(9010213);
    pub const a_1_1640_0: ParticleID = ParticleID(9020113);
    pub const a_1_1640_plus: ParticleID = ParticleID(9020213);
    pub const ρ_1700_0: ParticleID = ParticleID(30113);
    pub const ρ_1700_plus: ParticleID = ParticleID(30213);
    pub const ρ_1900_0: ParticleID = ParticleID(9030113);
    pub const ρ_1900_plus: ParticleID = ParticleID(9030213);
    pub const ρ_2150_0: ParticleID = ParticleID(9040113);
    pub const ρ_2150_plus: ParticleID = ParticleID(9040213);
    pub const a_2_1320_0: ParticleID = ParticleID(115);
    pub const a_2_1320_plus: ParticleID = ParticleID(215);
    pub const π_2_1670_0: ParticleID = ParticleID(10115);
    pub const π_2_1670_plus: ParticleID = ParticleID(10215);
    pub const a_2_1700_0: ParticleID = ParticleID(9000115);
    pub const a_2_1700_plus: ParticleID = ParticleID(9000215);
    pub const π_2_2100_0: ParticleID = ParticleID(9010115);
    pub const π_2_2100_plus: ParticleID = ParticleID(9010215);
    pub const ρ_3_1690_0: ParticleID = ParticleID(117);
    pub const ρ_3_1690_plus: ParticleID = ParticleID(217);
    pub const ρ_3_1990_0: ParticleID = ParticleID(9000117);
    pub const ρ_3_1990_plus: ParticleID = ParticleID(9000217);
    pub const ρ_3_2250_0: ParticleID = ParticleID(9010117);
    pub const ρ_3_2250_plus: ParticleID = ParticleID(9010217);
    pub const a_4_2040_0: ParticleID = ParticleID(119);
    pub const a_4_2040_plus: ParticleID = ParticleID(219);
}

#[allow(non_snake_case)]
pub mod light_Ieq0_mesons {
    use super::*;
    pub const η: ParticleID = ParticleID(221);
    pub const η_prime_958: ParticleID = ParticleID(331);
    pub const f_0_500: ParticleID = ParticleID(9000221);
    pub const f_0_980: ParticleID = ParticleID(9010221);
    pub const η_1295: ParticleID = ParticleID(100221);
    pub const f_0_1370: ParticleID = ParticleID(10221);
    pub const η_1405: ParticleID = ParticleID(9020221);
    pub const η_1475: ParticleID = ParticleID(100331);
    pub const f_0_1500: ParticleID = ParticleID(9030221);
    pub const f_0_1710: ParticleID = ParticleID(10331);
    pub const η_1760: ParticleID = ParticleID(9040221);
    pub const f0_2020: ParticleID = ParticleID(9050221);
    pub const f0_2100: ParticleID = ParticleID(9060221);
    pub const f0_2200: ParticleID = ParticleID(9070221);
    pub const η_2225: ParticleID = ParticleID(9080221);
    pub const ω_782: ParticleID = ParticleID(223);
    pub const φ_1020: ParticleID = ParticleID(333);
    pub const h_1_1170: ParticleID = ParticleID(10223);
    pub const f_1_1285: ParticleID = ParticleID(20223);
    pub const h_1_1380: ParticleID = ParticleID(10333);
    pub const f_1_1420: ParticleID = ParticleID(20333);
    pub const ω_1420: ParticleID = ParticleID(100223);
    pub const f_1_1510: ParticleID = ParticleID(9000223);
    pub const h_1_1595: ParticleID = ParticleID(9010223);
    pub const ω_1650: ParticleID = ParticleID(30223);
    pub const φ_1680: ParticleID = ParticleID(100333);
    pub const f_2_1270: ParticleID = ParticleID(225);
    pub const f_2_1430: ParticleID = ParticleID(9000225);
    pub const f_2_prime_1525: ParticleID = ParticleID(335);
    pub const f_2_1565: ParticleID = ParticleID(9010225);
    pub const f_2_1640: ParticleID = ParticleID(9020225);
    pub const η_2_1645: ParticleID = ParticleID(10225);
    pub const f_2_1810: ParticleID = ParticleID(9030225);
    pub const η_2_1870: ParticleID = ParticleID(10335);
    pub const f_2_1910: ParticleID = ParticleID(9040225);
    pub const f_2_1950: ParticleID = ParticleID(9050225);
    pub const f_2_2010: ParticleID = ParticleID(9060225);
    pub const f_2_2150: ParticleID = ParticleID(9070225);
    pub const f_2_2300: ParticleID = ParticleID(9080225);
    pub const f_2_2340: ParticleID = ParticleID(9090225);
    pub const ω_3_1670: ParticleID = ParticleID(227);
    pub const φ_3_1850: ParticleID = ParticleID(337);
    pub const f_4_2050: ParticleID = ParticleID(229);
    pub const f_J_2220: ParticleID = ParticleID(9000229);
    pub const f_4_2300: ParticleID = ParticleID(9010229);
}

pub mod strange_mesons {
    use super::*;
    pub const K_0_L: ParticleID = ParticleID(130);
    pub const K_0_S: ParticleID = ParticleID(310);
    pub const K_0: ParticleID = ParticleID(311);
    pub const K_plus: ParticleID = ParticleID(321);
    pub const K_0_star_700_0: ParticleID = ParticleID(9000311);
    pub const K_0_star_700_plus: ParticleID = ParticleID(9000321);
    pub const K_0_star_1430_0: ParticleID = ParticleID(10311);
    pub const K_0_star_1430_plus: ParticleID = ParticleID(10321);
    pub const K_1460_0: ParticleID = ParticleID(100311);
    pub const K_1460_plus: ParticleID = ParticleID(100321);
    pub const K_1830_0: ParticleID = ParticleID(9010311);
    pub const K_1830_plus: ParticleID = ParticleID(9010321);
    pub const K_0_star_1950_0: ParticleID = ParticleID(9020311);
    pub const K_0_star_1950_plus: ParticleID = ParticleID(9020321);
    pub const K_star_892_0: ParticleID = ParticleID(313);
    pub const K_star_892_plus: ParticleID = ParticleID(323);
    pub const K_1_1270_0: ParticleID = ParticleID(10313);
    pub const K_1_1270_plus: ParticleID = ParticleID(10323);
    pub const K_1_1400_0: ParticleID = ParticleID(20313);
    pub const K_1_1400_plus: ParticleID = ParticleID(20323);
    pub const K_star_1410_0: ParticleID = ParticleID(100313);
    pub const K_star_1410_plus: ParticleID = ParticleID(100323);
    pub const K_1_1650_0: ParticleID = ParticleID(9000313);
    pub const K_1_1650_plus: ParticleID = ParticleID(9000323);
    pub const K_star_1680_0: ParticleID = ParticleID(30313);
    pub const K_star_1680_plus: ParticleID = ParticleID(30323);
    pub const K_2_star_1430_0: ParticleID = ParticleID(315);
    pub const K_2_star_1430_plus: ParticleID = ParticleID(325);
    pub const K_2_1580_0: ParticleID = ParticleID(9000315);
    pub const K_2_1580_plus: ParticleID = ParticleID(9000325);
    pub const K_2_1770_0: ParticleID = ParticleID(10315);
    pub const K_2_1770_plus: ParticleID = ParticleID(10325);
    pub const K_2_1820_0: ParticleID = ParticleID(20315);
    pub const K_2_1820_plus: ParticleID = ParticleID(20325);
    pub const K_2_star_1980_0: ParticleID = ParticleID(9010315);
    pub const K_2_star_1980_plus: ParticleID = ParticleID(9010325);
    pub const K_2_2250_0: ParticleID = ParticleID(9020315);
    pub const K_2_2250_plus: ParticleID = ParticleID(9020325);
    pub const K_3_star_1780_0: ParticleID = ParticleID(317);
    pub const K_3_star_1780_plus: ParticleID = ParticleID(327);
    pub const K_3_2320_0: ParticleID = ParticleID(9010317);
    pub const K_3_2320_plus: ParticleID = ParticleID(9010327);
    pub const K_4_star_2045_0: ParticleID = ParticleID(319);
    pub const K_4_star_2045_plus: ParticleID = ParticleID(329);
    pub const K_4_2500_0: ParticleID = ParticleID(9000319);
    pub const K_4_2500_plus: ParticleID = ParticleID(9000329);
}

pub mod charmed_mesons {
    use super::*;
    pub const D_plus: ParticleID = ParticleID(411);
    pub const D_0: ParticleID = ParticleID(421);
    pub const D_0_star_2400_plus: ParticleID = ParticleID(10411);
    pub const D_0_star_2400_0: ParticleID = ParticleID(10421);
    pub const D_star_2010_plus: ParticleID = ParticleID(413);
    pub const D_star_2007_0: ParticleID = ParticleID(423);
    pub const D_1_2420_plus: ParticleID = ParticleID(10413);
    pub const D_1_2420_0: ParticleID = ParticleID(10423);
    pub const D_1_H_plus: ParticleID = ParticleID(20413);
    pub const D_1_2430_0: ParticleID = ParticleID(20423);
    pub const D_2_star_2460_plus: ParticleID = ParticleID(415);
    pub const D_2_star_2460_0: ParticleID = ParticleID(425);
    pub const D_s_plus: ParticleID = ParticleID(431);
    pub const D_s0_star_2317_plus: ParticleID = ParticleID(10431);
    pub const D_s_star_plus: ParticleID = ParticleID(433);
    pub const D_s_1_2536_plus: ParticleID = ParticleID(10433);
    pub const D_s_1_2460_plus: ParticleID = ParticleID(20433);
    pub const D_s_2_star_2573_plus: ParticleID = ParticleID(435);
}

pub mod bottom_mesons {
    use super::*;
    pub const B_0: ParticleID = ParticleID(511);
    pub const B_plus: ParticleID = ParticleID(521);
    pub const B_0_star_0: ParticleID = ParticleID(10511);
    pub const B_0_star_plus: ParticleID = ParticleID(10521);
    pub const B_star_0: ParticleID = ParticleID(513);
    pub const B_star_plus: ParticleID = ParticleID(523);
    pub const B_1_L_0: ParticleID = ParticleID(10513);
    pub const B_1_L_plus: ParticleID = ParticleID(10523);
    pub const B_1_H_0: ParticleID = ParticleID(20513);
    pub const B_1_H_plus: ParticleID = ParticleID(20523);
    pub const B_2_star0: ParticleID = ParticleID(515);
    pub const B_2_star_plus: ParticleID = ParticleID(525);
    pub const B_s_0: ParticleID = ParticleID(531);
    pub const B_s_0_star_0: ParticleID = ParticleID(10531);
    pub const B_s_star_0: ParticleID = ParticleID(533);
    pub const B_s_1_L_0: ParticleID = ParticleID(10533);
    pub const B_s_1_H_0: ParticleID = ParticleID(20533);
    pub const B_s_2_star_0: ParticleID = ParticleID(535);
    pub const B_c_plus: ParticleID = ParticleID(541);
    pub const B_c_0_star_plus: ParticleID = ParticleID(10541);
    pub const B_c_star_plus: ParticleID = ParticleID(543);
    pub const B_c_1_L_plus: ParticleID = ParticleID(10543);
    pub const B_c_1_H_plus: ParticleID = ParticleID(20543);
    pub const B_c_2_star_plus: ParticleID = ParticleID(545);
}

pub mod ccbar_mesons {
    use super::*;
    pub const η_c_1S: ParticleID = ParticleID(441);
    pub const χ_c_0_1P: ParticleID = ParticleID(10441);
    pub const η_c_2S: ParticleID = ParticleID(100441);
    pub const Jψ_1S: ParticleID = ParticleID(443);
    pub const h_c_1P: ParticleID = ParticleID(10443);
    pub const χ_c_1_1P: ParticleID = ParticleID(20443);
    pub const ψ_2S: ParticleID = ParticleID(100443);
    pub const ψ_3770: ParticleID = ParticleID(30443);
    pub const ψ_4040: ParticleID = ParticleID(9000443);
    pub const ψ_4160: ParticleID = ParticleID(9010443);
    pub const ψ_4415: ParticleID = ParticleID(9020443);
    pub const χ_c_2_1P: ParticleID = ParticleID(445);
    pub const χ_c_2_3930: ParticleID = ParticleID(100445);
}

pub mod bbbar_mesons {
    use super::*;
    pub const η_b_1S: ParticleID = ParticleID(551);
    pub const χ_b_0_1P: ParticleID = ParticleID(10551);
    pub const η_b_2S: ParticleID = ParticleID(100551);
    pub const χ_b_0_2P: ParticleID = ParticleID(110551);
    pub const η_b_3S: ParticleID = ParticleID(200551);
    pub const χ_b_0_3P: ParticleID = ParticleID(210551);
    pub const Υ_1S: ParticleID = ParticleID(553);
    pub const h_b_1P: ParticleID = ParticleID(10553);
    pub const χ_b_1_1P: ParticleID = ParticleID(20553);
    pub const Υ_1_1D: ParticleID = ParticleID(30553);
    pub const Υ_2S: ParticleID = ParticleID(100553);
    pub const h_b_2P: ParticleID = ParticleID(110553);
    pub const χ_b_1_2P: ParticleID = ParticleID(120553);
    pub const Υ_1_2D: ParticleID = ParticleID(130553);
    pub const Υ_3S: ParticleID = ParticleID(200553);
    pub const h_b_3P: ParticleID = ParticleID(210553);
    pub const χ_b_1_3P: ParticleID = ParticleID(220553);
    pub const Υ_4S: ParticleID = ParticleID(300553);
    pub const Υ_10860: ParticleID = ParticleID(9000553);
    pub const Υ_11020: ParticleID = ParticleID(9010553);
    pub const χ_b_2_1P: ParticleID = ParticleID(555);
    pub const η_b_2_1D: ParticleID = ParticleID(10555);
    pub const Υ_2_1D: ParticleID = ParticleID(20555);
    pub const χ_b_2_2P: ParticleID = ParticleID(100555);
    pub const η_b_2_2D: ParticleID = ParticleID(110555);
    pub const Υ_2_2D: ParticleID = ParticleID(120555);
    pub const χ_b_2_3P: ParticleID = ParticleID(200555);
    pub const Υ_3_1D: ParticleID = ParticleID(557);
    pub const Υ_3_2D: ParticleID = ParticleID(100557);
}

pub mod light_baryons {
    use super::*;
    pub const p: ParticleID = ParticleID(2212);
    pub const proton: ParticleID = p;
    pub const n: ParticleID = ParticleID(2112);
    pub const neutron: ParticleID = n;
    pub const Δ_plus_plus: ParticleID = ParticleID(2224);
    pub const Δ_plus: ParticleID = ParticleID(2214);
    pub const Δ_0: ParticleID = ParticleID(2114);
    pub const Δ_minus: ParticleID = ParticleID(1114);
}

pub mod strange_baryons {
    use super::*;
    pub const Λ: ParticleID = ParticleID(3122);
    pub const Σ_plus: ParticleID = ParticleID(3222);
    pub const Σ_0: ParticleID = ParticleID(3212);
    pub const Σ_minus: ParticleID = ParticleID(3112);
    pub const Σ_star_plus: ParticleID = ParticleID(3224);
    pub const Σ_star_0: ParticleID = ParticleID(3214);
    pub const Σ_star_minus: ParticleID = ParticleID(3114);
    pub const Ξ_0: ParticleID = ParticleID(3322);
    pub const Ξ_minus: ParticleID = ParticleID(3312);
    pub const Ξ_star_0: ParticleID = ParticleID(3324);
    pub const Ξ_star_minus: ParticleID = ParticleID(3314);
    pub const Ω_minus: ParticleID = ParticleID(3334);
}

pub mod charmed_baryons {
    use super::*;
    pub const Λ_c_plus: ParticleID = ParticleID(4122);
    pub const Σ_c_plus_plus: ParticleID = ParticleID(4222);
    pub const Σ_c_plus: ParticleID = ParticleID(4212);
    pub const Σ_c_0: ParticleID = ParticleID(4112);
    pub const Σ_c_star_plus_plus: ParticleID = ParticleID(4224);
    pub const Σ_c_star_plus: ParticleID = ParticleID(4214);
    pub const Σ_c_star_0: ParticleID = ParticleID(4114);
    pub const Ξ_c_plus: ParticleID = ParticleID(4232);
    pub const Ξ_c_0: ParticleID = ParticleID(4132);
    pub const Ξ_c_prime_plus: ParticleID = ParticleID(4322);
    pub const Ξ_c_prime_0: ParticleID = ParticleID(4312);
    pub const Ξ_c_star_plus: ParticleID = ParticleID(4324);
    pub const Ξ_c_star_0: ParticleID = ParticleID(4314);
    pub const Ω_c_0: ParticleID = ParticleID(4332);
    pub const Ω_c_star_0: ParticleID = ParticleID(4334);
    pub const Ξ_c_c_plus: ParticleID = ParticleID(4412);
    pub const Ξ_c_c_plus_plus: ParticleID = ParticleID(4422);
    pub const Ξ_c_c_star_plus: ParticleID = ParticleID(4414);
    pub const Ξ_c_c_star_plus_plus: ParticleID = ParticleID(4424);
    pub const Ω_c_c_plus: ParticleID = ParticleID(4432);
    pub const Ω_c_c_star_plus: ParticleID = ParticleID(4434);
    pub const Ω_c_c_c_plus_plus: ParticleID = ParticleID(4444);
}

pub mod bottom_baryons {
    use super::*;
    pub const Λ_b_0: ParticleID = ParticleID(5122);
    pub const Σ_b_minus: ParticleID = ParticleID(5112);
    pub const Σ_b0: ParticleID = ParticleID(5212);
    pub const Σ_b_plus: ParticleID = ParticleID(5222);
    pub const Σ_b_star_minus: ParticleID = ParticleID(5114);
    pub const Σ_b_star_0: ParticleID = ParticleID(5214);
    pub const Σ_b_star_plus: ParticleID = ParticleID(5224);
    pub const Ξ_b_minus: ParticleID = ParticleID(5132);
    pub const Ξ_b_0: ParticleID = ParticleID(5232);
    pub const Ξ_b_prime_minus: ParticleID = ParticleID(5312);
    pub const Ξ_b_prime_0: ParticleID = ParticleID(5322);
    pub const Ξ_b_star_minus: ParticleID = ParticleID(5314);
    pub const Ξ_b_star_0: ParticleID = ParticleID(5324);
    pub const Ω_b_minus: ParticleID = ParticleID(5332);
    pub const Ω_b_star_minus: ParticleID = ParticleID(5334);
    pub const Ξ_b_c_0: ParticleID = ParticleID(5142);
    pub const Ξ_b_c_plus: ParticleID = ParticleID(5242);
    pub const Ξ_b_c_prime_0: ParticleID = ParticleID(5412);
    pub const Ξ_b_c_prime_plus: ParticleID = ParticleID(5422);
    pub const Ξ_b_c_star0: ParticleID = ParticleID(5414);
    pub const Ξ_b_c_star_plus: ParticleID = ParticleID(5424);
    pub const Ω_b_c_0: ParticleID = ParticleID(5342);
    pub const Ω_b_c_prime_0: ParticleID = ParticleID(5432);
    pub const Ω_b_c_star0: ParticleID = ParticleID(5434);
    pub const Ω_b_c_c_plus: ParticleID = ParticleID(5442);
    pub const Ω_b_c_c_star_plus: ParticleID = ParticleID(5444);
    pub const Ξ_b_b_minus: ParticleID = ParticleID(5512);
    pub const Ξ_b_b_0: ParticleID = ParticleID(5522);
    pub const Ξ_b_b_star_minus: ParticleID = ParticleID(5514);
    pub const Ξ_b_b_star_0: ParticleID = ParticleID(5524);
    pub const Ω_b_b_minus: ParticleID = ParticleID(5532);
    pub const Ω_b_b_star_minus: ParticleID = ParticleID(5534);
    pub const Ω_b_b_c_0: ParticleID = ParticleID(5542);
    pub const Ω_b_b_c_star0: ParticleID = ParticleID(5544);
    pub const Ω_b_b_b_minus: ParticleID = ParticleID(5554);
}

pub mod pentaquarks {
    use super::*;
    pub const Θ_plus: ParticleID = ParticleID(100221132);
    pub const Φ_minus_minus: ParticleID = ParticleID(100331122);
}

pub mod anti_quarks {
    use super::*;
    pub const d_bar: ParticleID = ParticleID(-1);
    pub const anti_down: ParticleID = d_bar;
    pub const u_bar: ParticleID = ParticleID(-2);
    pub const anti_up: ParticleID = u_bar;
    pub const s_bar: ParticleID = ParticleID(-3);
    pub const anti_strange: ParticleID = s_bar;
    pub const c_bar: ParticleID = ParticleID(-4);
    pub const anti_charm: ParticleID = c_bar;
    pub const b_bar: ParticleID = ParticleID(-5);
    pub const anti_bottom: ParticleID = b_bar;
    pub const t_bar: ParticleID = ParticleID(-6);
    pub const anti_top: ParticleID = t_bar;
    pub const b_prime_bar: ParticleID = ParticleID(-7);
    pub const t_prime_bar: ParticleID = ParticleID(-9);
}

pub mod anti_leptons {
    use super::*;

    pub const e_bar: ParticleID = ParticleID(-11);
    pub const positron: ParticleID = e_bar;
    pub const ν_e_bar: ParticleID = ParticleID(-12);
    pub const nu_e_bar: ParticleID = ν_e_bar;
    pub const electron_anti_neutrino: ParticleID = ν_e_bar;
    pub const μ_bar: ParticleID = ParticleID(-13);
    pub const mu_bar: ParticleID = μ_bar;
    pub const mu_plus: ParticleID = μ_bar;
    pub const μ_plus: ParticleID = μ_bar;
    pub const ν_μ_bar: ParticleID = ParticleID(-14);
    pub const nu_mu_bar: ParticleID = ν_μ_bar;
    pub const muon_anti_neutrino: ParticleID = ν_μ_bar;
    pub const τ_bar: ParticleID = ParticleID(-15);
    pub const tau_bar: ParticleID = τ_bar;
    pub const tau_plus: ParticleID = τ_bar;
    pub const τ_plus: ParticleID = τ_bar;
    pub const ν_τ_bar: ParticleID = ParticleID(-16);
    pub const nu_tau_bar: ParticleID = ν_τ_bar;
    pub const tau_anti_neutrino: ParticleID = ν_τ_bar;
    pub const τ_prime_bar: ParticleID = ParticleID(-17);
    pub const tau_prime_bar: ParticleID = τ_prime_bar;
    pub const ν_τ_prime_bar: ParticleID = ParticleID(-18);
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

    /// Particle name in LaTeX format
    pub const fn latex_name(&self) -> Option<&'static str> {
        use sm_elementary_particles::*;
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

            _ => return None,
        };
        Some(name)
    }

    /// Name of the associated particle
    pub const fn name(&self) -> Option<&'static str> {
        use sm_elementary_particles::*;
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

            _ => return None,
        };
        Some(name)
    }

    pub const fn id(self) -> i32 {
        self.0
    }

    pub const fn anti(self) -> Self {
        Self(- self.0)
    }

    pub const fn is_anti_particle(&self) -> bool {
        self.0 < 0
    }

    pub const fn is_gauge_boson(&self) -> bool {
        use gauge_and_higgs_bosons::*;
        let abs_id = self.0.abs();
        gluon.id() <= abs_id && abs_id <= W_plus.id()
    }

    pub const fn is_quark(&self) -> bool {
        use quarks::*;
        down.id() < self.id() && self.id() < t_prime.id()
    }

    pub const fn is_anti_quark(&self) -> bool {
        self.is_anti_particle() && self.anti().is_quark()
    }

    pub const fn is_lepton(&self) -> bool {
        use leptons::*;
        electron.id() <= self.id() && self.id() <= tau_prime_neutrino.id()
    }

    pub const fn is_anti_lepton(&self) -> bool {
        self.is_anti_particle() && self.anti().is_lepton()
    }

    pub const fn is_neutrino(&self) -> bool {
        self.is_lepton() && (self.id() & 1 == 0)
    }

    pub const fn is_anti_neutrino(&self) -> bool {
        self.is_anti_particle() && self.anti().is_neutrino()
    }

    pub const fn is_charged_lepton(&self) -> bool {
        self.is_lepton() && (self.id() & 1 == 1)
    }

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
