use utils::*;

pub type InitBlFes = Vec<FreeEnergy>;

pub const MIN_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_BL_DELTA_FE: usize = 7;
pub const COEFFICENT_4_LOG_EXTRAPOLATION_OF_INIT_BL_DELTA_FE: FreeEnergy = 1.75 * GAS_CONST * TEMPERATURE; // The unit is [kcal / mol].
pub const BL_SPECIAL_C_BULGE_BONUS_DELTA_FE: FreeEnergy = -0.9;
lazy_static! {
  pub static ref INIT_BL_DELTA_FES: InitBlFes = {
    let mut init_bl_delta_fes = vec![0., 3.8, 2.8, 3.2, 3.6, 4.0, 4.4];
    let len_of_init_bl_delta_fes = init_bl_delta_fes.len();
    let basic_init_bl_delta_fe = init_bl_delta_fes[MIN_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_BL_DELTA_FE - 1];
    for i in len_of_init_bl_delta_fes .. MAX_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_LOOP_DELTA_FE {
      init_bl_delta_fes.push(basic_init_bl_delta_fe + COEFFICENT_4_LOG_EXTRAPOLATION_OF_INIT_BL_DELTA_FE * fast_ln(i as FreeEnergy / (MIN_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_BL_DELTA_FE - 1) as FreeEnergy));
    }
    init_bl_delta_fes
  };
}
