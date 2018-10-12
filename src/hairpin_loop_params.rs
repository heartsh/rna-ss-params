use utils::*;

pub type InitHlDeltaFes = Vec<FreeEnergy>;
pub type SpecialHlDeltaFes = HashMap<Seq, FreeEnergy, Hasher>;

pub const MIN_HL_LEN: usize = 3;
pub const MIN_SPAN_OF_INDEX_PAIR_CLOSING_HL: usize = MIN_HL_LEN + 2;
pub const MIN_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_HL_DELTA_FE: usize = 10;
pub const COEFFICENT_4_LOG_EXTRAPOLATION_OF_INIT_HL_DELTA_FE: FreeEnergy = 1.75 * GAS_CONST * TEMPERATURE; // The unit is [kcal / mol].
pub const HL_UU_OR_GA_FIRST_MISMATCH_BONUS_DELTA_FE: FreeEnergy = -0.9;
pub const HL_GG_FIRST_MISMATCH_BONUS_DELTA_FE: FreeEnergy = -0.8;
pub const HL_SPECIAL_GU_CLOSURE_BONUS_DELTA_FE: FreeEnergy = -2.2;
pub const HL_OF_3_CS_PENALTY_DELTA_FE: FreeEnergy = 1.5;
pub const COEFFICENT_4_ALL_C_HL_DELTA_FE: FreeEnergy = 0.3;
pub const CONST_4_ALL_C_HL_DELTA_FE: FreeEnergy = 1.6;
lazy_static! {
  pub static ref INIT_HL_DELTA_FES: InitHlDeltaFes = {
    let mut init_hl_delta_fes = vec![0., 0., 0., 5.4, 5.6, 5.7, 5.4, 6.0, 5.5, 6.4];
    let len_of_init_hl_delta_fes = init_hl_delta_fes.len();
    let basic_init_hl_delta_fe = init_hl_delta_fes[MIN_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_HL_DELTA_FE - 1];
    for i in len_of_init_hl_delta_fes .. MAX_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_LOOP_DELTA_FE {
      init_hl_delta_fes.push(basic_init_hl_delta_fe + COEFFICENT_4_LOG_EXTRAPOLATION_OF_INIT_HL_DELTA_FE * (i as FreeEnergy / (MIN_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_HL_DELTA_FE - 1) as FreeEnergy).ln());
    }
    init_hl_delta_fes
  };
  pub static ref SPECIAL_HL_DELTA_FES: SpecialHlDeltaFes = {
    [
      (String::from("CAACG").into_bytes(), 6.8),
      (String::from("GUUAC").into_bytes(), 6.9),
      (String::from("CUACGG").into_bytes(), -10.7),
      (String::from("CUCCGG").into_bytes(), -12.9),
      (String::from("CUUCGG").into_bytes(), -15.3),
      (String::from("CUUUGG").into_bytes(), -6.8),
      (String::from("CCAAGG").into_bytes(), -10.3),
      (String::from("CCCAGG").into_bytes(), -8.9),
      (String::from("CCGAGG").into_bytes(), -6.6),
      (String::from("CCUAGG").into_bytes(), -3.5),
      (String::from("CCACGG").into_bytes(), -3.3),
      (String::from("CCGCGG").into_bytes(), -7.5),
      (String::from("CCUCGG").into_bytes(), -13.9),
      (String::from("CUAAGG").into_bytes(), -7.6),
      (String::from("CUCAGG").into_bytes(), -6.6),
      (String::from("CUUAGG").into_bytes(), -6.2),
      (String::from("CUGCGG").into_bytes(), -10.7),
      (String::from("CAACGG").into_bytes(), 6.9),
      (String::from("ACAGUGCU").into_bytes(), -12.8),
      (String::from("ACAGUGAU").into_bytes(), -11.4),
      (String::from("ACAGUGUU").into_bytes(), -15.4),
      (String::from("ACAGUACU").into_bytes(), -16.8),
    ].iter().cloned().collect()
  };
}
