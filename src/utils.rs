pub use bio_seq_algos::utils::*;
pub use std::path::Path;
pub use std::io::prelude::*;
pub use std::io::BufWriter;
pub use std::fs::File;

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
pub type ContraIlExplicitFes = [[FreeEnergy; NUM_OF_BASES]; NUM_OF_BASES];
pub type ContraBlLengthFes = [FreeEnergy; CONTRA_MAX_LOOP_LEN];
pub type ContraIlLengthFes = [FreeEnergy; CONTRA_MAX_LOOP_LEN - 1];
pub type ContraIlSymmLengthFes = [FreeEnergy; CONTRA_MAX_LOOP_LEN / 2];
pub type ContraIlAsymmLengthFes = [FreeEnergy; CONTRA_MAX_LOOP_LEN - 2];
pub type ContraBl0x1Fes = [FreeEnergy; NUM_OF_BASES];
pub type ContraIl1x1Fes = [[FreeEnergy; NUM_OF_BASES]; NUM_OF_BASES];
pub type ContraStackFes = [[[[FreeEnergy; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
pub type ContraHelixClosingFes = [[FreeEnergy; NUM_OF_BASES]; NUM_OF_BASES];
pub type ContraDangleFes = [[[FreeEnergy; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];

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

pub const CONTRA_IL_EXPLICIT_FES: ContraIlExplicitFes = [
  [-0.1754591076, 0.03083787104, -0.171565435, -0.2294680983],
  [0.03083787104,  -0.1304072693, -0.07730329553, 0.2782767264],
  [-0.171565435, -0.07730329553, -0.02898949617, 0.3112350694],
  [-0.2294680983, 0.2782767264, 0.3112350694, -0.3226348245],
];
pub const CONTRA_BL_0X1_FES: ContraBl0x1Fes = [
  -0.1216861662, -0.07111241127, 0.008947026647, -0.002685763742
];
pub const CONTRA_IL_1X1_FES: ContraIl1x1Fes = [
  [0.2944404686, 0.08641360967, -0.3664197228, -0.2053107048],
  [0.08641360967, -0.1582543624, 0.4175273724, 0.1368762582],
  [-0.3664197228, 0.4175273724,-0.1193514754, -0.4188101413],
  [-0.2053107048, 0.1368762582, -0.4188101413, 0.147140653],
];

pub const CONTRA_MAX_LOOP_LEN: usize = 30;
pub const CONTRA_ML_BASE_FE: FreeEnergy = -1.199055076;
pub const CONTRA_ML_UNPAIRED_FE: FreeEnergy = -0.1983300391;
pub const CONTRA_ML_PAIRED_FE: FreeEnergy = -0.9253883752;
pub const CONTRA_EL_UNPAIRED_FE: FreeEnergy = -0.00972883093;
pub const CONTRA_EL_PAIRED_FE: FreeEnergy = -0.0009674111431;

pub fn scale(free_energy: FreeEnergy) -> FreeEnergy {
  - INVERSE_TEMPERATURE * free_energy
}
