pub use bio_seq_algos::utils::*;
pub use std::path::Path;
pub use std::io::prelude::*;
pub use std::io::{BufReader, BufWriter};
pub use std::fs::File;
pub use getopts::Options;

pub type FreeEnergy = Prob;
pub type Base = usize;
pub type BasePair = (Base, Base);
pub type Seq = Vec<Base>;

pub type StackDeltaFes = [[[[FreeEnergy; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
pub type HlTmDeltaFes = StackDeltaFes;
pub type IlTmDeltaFes = HlTmDeltaFes;
pub type MlTmDeltaFes = HlTmDeltaFes;
pub type InitBlDeltaFes = [FreeEnergy; 31];
pub type InitHlDeltaFes = [FreeEnergy; MAX_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_LOOP_DELTA_FE + 1];
pub type SpecialHlDeltaFes = [(Seq, FreeEnergy); NUM_OF_SPECIAL_HLS];
pub type InitIlDeltaFes = [FreeEnergy; 31];
pub type IlTmBonusDeltaFes = [[FreeEnergy; NUM_OF_BASES]; NUM_OF_BASES];
pub type OneVs1IlDeltaFes = [[[[[[FreeEnergy; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
pub type OneVs2Il = (BasePair, Base);
pub type OneVs2IlDeltaFes = [[[[[[[FreeEnergy; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
pub type TwoVs2Il = (BasePair, BasePair);
pub type TwoVs2IlDeltaFes = [[[[[[[[FreeEnergy; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
pub type DeDeltaFes = [[[FreeEnergy; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];

pub type ContraBasePairFes = [[FreeEnergy; NUM_OF_BASES]; NUM_OF_BASES];
pub type ContraTerminalMismatchFes = [[[[FreeEnergy; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
pub type ContraHlLengthFes = [FreeEnergy; CONTRA_MAX_LOOP_LEN + 1];
pub type ContraIlExplicitFes = [[FreeEnergy; CONTRA_MAX_IL_EXPLICIT_LEN]; CONTRA_MAX_IL_EXPLICIT_LEN];
pub type ContraBlLengthFes = [FreeEnergy; CONTRA_MAX_LOOP_LEN];
pub type ContraIlLengthFes = [FreeEnergy; CONTRA_MAX_LOOP_LEN - 1];
pub type ContraIlSymmLengthFes = [FreeEnergy; CONTRA_MAX_LOOP_LEN / 2];
pub type ContraIlAsymmLengthFes = [FreeEnergy; CONTRA_MAX_LOOP_LEN - 2];
pub type ContraBl0x1Fes = [FreeEnergy; NUM_OF_BASES];
pub type ContraIl1x1Fes = [[FreeEnergy; NUM_OF_BASES]; NUM_OF_BASES];
pub type ContraStackFes = [[[[FreeEnergy; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
pub type ContraHelixClosingFes = [[FreeEnergy; NUM_OF_BASES]; NUM_OF_BASES];
pub type ContraDangleFes = [[[FreeEnergy; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];

type Arg = String;
pub type Args = Vec<Arg>;

pub const NUM_OF_TRANSITS: usize = 3;
pub const CONST_4_INIT_ML_DELTA_FE: FreeEnergy = - INVERSE_TEMPERATURE * 9.3;
pub const COEFFICIENT_4_TERM_OF_NUM_OF_BRANCHING_HELICES_ON_INIT_ML_DELTA_FE: FreeEnergy = - INVERSE_TEMPERATURE * (-0.9);
pub const HELIX_AU_OR_GU_END_PENALTY_DELTA_FE: FreeEnergy = - INVERSE_TEMPERATURE * 0.5;
pub const MAX_NINIO: FreeEnergy = - INVERSE_TEMPERATURE * 3.;
pub const COEFFICIENT_4_NINIO: FreeEnergy = - INVERSE_TEMPERATURE * 0.6;
pub const GAS_CONST: FreeEnergy = 1.98717 / KILO; // The unit is [kcal / (K * mol)].
pub const K0: FreeEnergy = 273.15; // The unit is [K].
pub const TEMPERATURE: FreeEnergy = 37. + K0; // The unit is [K].
pub const MAX_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_LOOP_DELTA_FE: usize = 1_000_000;
pub const MAX_2_LOOP_LEN: usize = 30;
pub const KILO: FreeEnergy = 1000.;
pub const INVERSE_TEMPERATURE: FreeEnergy = 1. / (GAS_CONST as FreeEnergy * TEMPERATURE as FreeEnergy); // The unit is [K * mol / (kcal * K)] = [mol / kcal].
pub const A: Base = 0;
pub const C: Base = 1;
pub const G: Base = 2;
pub const U: Base = 3;
pub const NUM_OF_BASES: usize = 4;
pub const AA: BasePair = (A, A);
pub const AC: BasePair = (A, C);
pub const AG: BasePair = (A, G);
pub const AU: BasePair = (A, U);
pub const CA: BasePair = (C, A);
pub const CC: BasePair = (C, C);
pub const CG: BasePair = (C, G);
pub const CU: BasePair = (C, U);
pub const GA: BasePair = (G, A);
pub const GC: BasePair = (G, C);
pub const GG: BasePair = (G, G);
pub const GU: BasePair = (G, U);
pub const UA: BasePair = (U, A);
pub const UC: BasePair = (U, C);
pub const UG: BasePair = (U, G);
pub const UU: BasePair = (U, U);
pub const NEG_INF: FreeEnergy = -1000_000_000_000_000.;

pub const NUM_OF_SPECIAL_HLS: usize = 22;
pub const MIN_HL_LEN: usize = 3;
pub const MIN_SPAN_OF_INDEX_PAIR_CLOSING_HL: usize = MIN_HL_LEN + 2;
pub const MIN_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_HL_DELTA_FE: usize = 10;
pub const COEFFICIENT_4_LOG_EXTRAPOLATION_OF_INIT_HL_DELTA_FE: FreeEnergy = - INVERSE_TEMPERATURE * 1.75 * GAS_CONST * TEMPERATURE; // The unit is [kcal / mol].
lazy_static! {
  pub static ref SPECIAL_HL_DELTA_FES: SpecialHlDeltaFes = {
    [
      (vec![C,A,A,C,G], scale(6.8)),
      (vec![G,U,U,A,C], scale(6.9)),
      (vec![C,U,A,C,G,G], scale(2.8)),
      (vec![C,U,C,C,G,G], scale(2.7)),
      (vec![C,U,U,C,G,G], scale(3.7)),
      (vec![C,U,U,U,G,G], scale(3.7)),
      (vec![C,C,A,A,G,G], scale(3.3)),
      (vec![C,C,C,A,G,G], scale(3.4)),
      (vec![C,C,G,A,G,G], scale(3.5)),
      (vec![C,C,U,A,G,G], scale(3.7)),
      (vec![C,C,A,C,G,G], scale(3.7)),
      (vec![C,C,G,C,G,G], scale(3.6)),
      (vec![C,C,U,C,G,G], scale(2.5)),
      (vec![C,U,A,A,G,G], scale(3.6)),
      (vec![C,U,C,A,G,G], scale(3.7)),
      (vec![C,U,U,A,G,G], scale(3.5)),
      (vec![C,U,G,C,G,G], scale(2.8)),
      (vec![C,A,A,C,G,G], scale(5.5)),
      (vec![A,C,A,G,U,G,C,U], scale(2.9)),
      (vec![A,C,A,G,U,G,A,U], scale(3.6)),
      (vec![A,C,A,G,U,G,U,U], scale(1.8)),
      (vec![A,C,A,G,U,A,C,U], scale(2.8)),
    ]
  };
}

pub const CONTRA_MAX_LOOP_LEN: usize = 30;
pub const CONTRA_MAX_IL_EXPLICIT_LEN: usize = 4;

pub const SMALL_A: u8 = 'a' as u8;
pub const BIG_A: u8 = 'A' as u8;
pub const SMALL_C: u8 = 'c' as u8;
pub const BIG_C: u8 = 'C' as u8;
pub const SMALL_G: u8 = 'g' as u8;
pub const BIG_G: u8 = 'G' as u8;
pub const SMALL_U: u8 = 'u' as u8;
pub const BIG_U: u8 = 'U' as u8;

pub fn scale(free_energy: FreeEnergy) -> FreeEnergy {
  - INVERSE_TEMPERATURE * free_energy
}

pub fn print_program_usage(program_name: &str, opts: &Options) {
  let program_usage = format!("The usage of this program: {} [options]", program_name);
  print!("{}", opts.usage(&program_usage));
}

pub fn convert_char<'a>(c: u8) -> usize {
  match c {
    SMALL_A | BIG_A => A,
    SMALL_C | BIG_C => C,
    SMALL_G | BIG_G => G,
    SMALL_U | BIG_U => U,
    _ => {assert!(false); U},
  }
}
