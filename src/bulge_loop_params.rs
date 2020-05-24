use utils::*;

pub type InitBlDeltaFes = [FreeEnergy; 31];

pub const MIN_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_BL_DELTA_FE: usize = 7;
pub const COEFFICIENT_4_LOG_EXTRAPOLATION_OF_INIT_BL_DELTA_FE: FreeEnergy = - INVERSE_TEMPERATURE * 1.75 * GAS_CONST * TEMPERATURE; // The unit is [kcal / mol].
pub const BL_SPECIAL_C_BULGE_BONUS_DELTA_FE: FreeEnergy = - INVERSE_TEMPERATURE * (-0.9);
