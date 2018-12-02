use utils::*;

pub type InitBlDeltaFes = [FreeEnergy; MAX_2_LOOP_LEN + 1];

pub const MIN_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_BL_DELTA_FE: usize = 7;
pub const COEFFICIENT_4_LOG_EXTRAPOLATION_OF_INIT_BL_DELTA_FE: FreeEnergy = 1.75 * GAS_CONST * TEMPERATURE; // The unit is [kcal / mol].
pub const BL_SPECIAL_C_BULGE_BONUS_DELTA_FE: FreeEnergy = -0.9;
pub const INIT_BL_DELTA_FES: InitBlDeltaFes = [
  0., 3.8, 2.8, 3.2, 3.6, 4.0, 4.4, 4.6, 4.7, 4.8, 4.9,
  5., 5.1, 5.2, 5.3, 5.4, 5.4, 5.5, 5.5, 5.6, 5.7,
  5.7, 5.8, 5.8, 5.8, 5.9, 5.9, 6., 6., 6., 6.1
];
