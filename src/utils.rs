pub use bio_seq_algos::utils::*;

pub type FreeEnergy = Prob;
pub type Base = usize;
pub type BasePair = (Base, Base);
pub type Seq = Vec<Base>;
pub type StackDeltaFes = [[[[FreeEnergy; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
pub type HlTmDeltaFes = StackDeltaFes;
pub type IlTmDeltaFes = HlTmDeltaFes;
pub type MlTmDeltaFes = HlTmDeltaFes;

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

pub fn scale(free_energy: FreeEnergy) -> FreeEnergy {
  - INVERSE_TEMPERATURE * free_energy
}
