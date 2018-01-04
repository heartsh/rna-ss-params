use utils::*;

pub type InitHlDeltaFes = Vec<FreeEnergy>;

pub const MIN_HL_LEN: usize = 3;
pub const MIN_SPAN_OF_INDEX_PAIR_CLOSING_HL: usize = MIN_HL_LEN + 2;
pub const MIN_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_HL_DELTA_FE: usize = 10;
pub const COEFFICENT_4_LOG_EXTRAPOLATION_OF_INIT_HL_DELTA_FE: FreeEnergy = 1.75 * GAS_CONST * TEMPERATURE;
lazy_static! {
  pub static ref INIT_HL_DELTA_FREE_ENERGIES: InitHlDeltaFes = {
    let mut init_hl_delta_fes = vec![0., 0., 0., 5.4, 5.6, 5.7, 5.4, 6.0, 5.5, 6.4];
    let len_of_init_hl_delta_fes = init_hl_delta_fes.len();
    let basic_init_hl_delta_fe = init_hl_delta_fes[MIN_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_HL_DELTA_FE - 1];
    for i in len_of_init_hl_delta_fes .. MAX_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_LOOP_DELTA_FE {
      init_hl_delta_fes.push(basic_init_hl_delta_fe + (COEFFICENT_4_LOG_EXTRAPOLATION_OF_INIT_HL_DELTA_FE * fast_ln(i as FreeEnergy / (MIN_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_HL_DELTA_FE - 1) as FreeEnergy)) / KILO);
    }
    init_hl_delta_fes
  };
}
