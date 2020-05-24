use utils::*;

pub type InitIlDeltaFes = [FreeEnergy; 31];
pub type IlTmBonusDeltaFes = [[FreeEnergy; NUM_OF_BASES]; NUM_OF_BASES];
pub type OneVs1IlDeltaFes = [[[[[[FreeEnergy; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
pub type OneVs2Il = (BasePair, Base);
pub type OneVs2IlDeltaFes = [[[[[[[FreeEnergy; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
pub type TwoVs2Il = (BasePair, BasePair);
pub type TwoVs2IlDeltaFes = [[[[[[[[FreeEnergy; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];

pub const MIN_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_IL_DELTA_FE: usize = 7;
pub const COEFFICIENT_4_LOG_EXTRAPOLATION_OF_INIT_IL_DELTA_FE: FreeEnergy = - INVERSE_TEMPERATURE * 1.08;
pub const IL_ASYMMETRY_PENALTY_DELTA_FE: FreeEnergy = - INVERSE_TEMPERATURE * 0.6;
pub const IL_AU_OR_GU_CLOSURE_PENALTY_DELTA_FE: FreeEnergy = - INVERSE_TEMPERATURE * 0.7;
