use utils::*;

pub type InitBulgeLoopFes = Vec<FreeEnergy>;

pub const MIN_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_BULGE_LOOP_DELTA_FE: usize = 7;
pub const COEFFICENT_4_LOG_EXTRAPOLATION_OF_INIT_BULGE_LOOP_DELTA_FE: FreeEnergy = 1.75 * GAS_CONST * TEMPARATURE;
lazy_static! {
  pub static ref INIT_BULGE_LOOP_DELTA_FES: InitBulgeLoopFes = {
    let mut init_bulge_loop_delta_fes = vec![3.8, 2.8, 3.2, 3.6, 4.0, 4.4, 4.6, 4.7, 4.8, 4.9, 5.0, 5.1, 5.2, 5.3, 5.4, 5.4, 5.5, 5.5, 5.6, 5.7, 5.7, 5.8, 5.8, 5.8, 5.9, 5.9, 6.0, 6.0, 6.0, 6.1];
    let len_of_init_bulge_loop_delta_fes = init_bulge_loop_delta_fes.len();
    let basic_init_bulge_loop_delta_fe = init_bulge_loop_delta_fes[MIN_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_BULGE_LOOP_DELTA_FE - 1];
    for i in len_of_init_bulge_loop_delta_fes .. MAX_LOOP_LEN_4_EXTRAPOLATION_OF_INIT_LOOP_DELTA_FE {
      init_bulge_loop_delta_fes.push((basic_init_bulge_loop_delta_fe + COEFFICENT_4_LOG_EXTRAPOLATION_OF_INIT_BULGE_LOOP_DELTA_FE * fast_ln(i as FreeEnergy / (MIN_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_BULGE_LOOP_DELTA_FE - 1) as FreeEnergy))/ KL);
    }
    init_bulge_loop_delta_fes
  };
}
