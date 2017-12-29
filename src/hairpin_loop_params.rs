use utils::*;

pub type InitHairpinLoopDeltaFes = Vec<FreeEnergy>;

pub const MIN_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_HAIRPIN_LOOP_DELTA_FE: usize = 10;
pub const COEFFICENT_4_LOG_EXTRAPOLATION_OF_INIT_HAIRPIN_LOOP_DELTA_FE: FreeEnergy = 1.75 * GAS_CONST * TEMPARATURE;
lazy_static! {
  pub static ref INIT_HAIRPIN_LOOP_DELTA_FREE_ENERGIES: InitHairpinLoopDeltaFes = {
    let mut init_hairpin_loop_delta_fes = vec![0., 0., 0., 5.4, 5.6, 5.7, 5.4, 6.0, 5.5, 6.4, 6.5, 6.6, 6.7, 6.8, 6.9, 6.9, 7.0, 7.1, 7.1, 7.2, 7.2, 7.3, 7.3, 7.4, 7.4, 7.5, 7.5, 7.5, 7.6, 7.6, 7.7];
    let len_of_init_hairpin_loop_delta_fes = init_hairpin_loop_delta_fes.len();
    let basic_init_hairpin_loop_delta_fe = init_hairpin_loop_delta_fes[MIN_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_HAIRPIN_LOOP_DELTA_FE - 1];
    for i in len_of_init_hairpin_loop_delta_fes .. MAX_LOOP_LEN_4_EXTRAPOLATION_OF_INIT_LOOP_DELTA_FE {
      init_hairpin_loop_delta_fes.push((basic_init_hairpin_loop_delta_fe + COEFFICENT_4_LOG_EXTRAPOLATION_OF_INIT_HAIRPIN_LOOP_DELTA_FE * fast_ln(i as FreeEnergy / (MIN_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_HAIRPIN_LOOP_DELTA_FE - 1) as FreeEnergy)) / KL);
    }
    init_hairpin_loop_delta_fes
  };
}
