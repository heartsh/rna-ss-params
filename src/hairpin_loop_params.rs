use utils::*;

pub type InitHlDeltaFes = Vec<FreeEnergy>;
pub type SpecialHlDeltaFes = HashMap<Seq, FreeEnergy, Hasher>;

pub const MIN_HL_LEN: usize = 3;
pub const MIN_SPAN_OF_INDEX_PAIR_CLOSING_HL: usize = MIN_HL_LEN + 2;
pub const MIN_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_HL_DELTA_FE: usize = 10;
pub const COEFFICIENT_4_LOG_EXTRAPOLATION_OF_INIT_HL_DELTA_FE: FreeEnergy = - INVERSE_TEMPERATURE * 1.75 * GAS_CONST * TEMPERATURE; // The unit is [kcal / mol].
pub const HL_UU_OR_GA_FIRST_MISMATCH_BONUS_DELTA_FE: FreeEnergy = - INVERSE_TEMPERATURE * (-0.9);
pub const HL_GG_FIRST_MISMATCH_BONUS_DELTA_FE: FreeEnergy = - INVERSE_TEMPERATURE * (-0.8);
pub const HL_SPECIAL_GU_CLOSURE_BONUS_DELTA_FE: FreeEnergy = - INVERSE_TEMPERATURE * (-2.2);
pub const HL_OF_3_CS_PENALTY_DELTA_FE: FreeEnergy = - INVERSE_TEMPERATURE * 1.5;
pub const COEFFICIENT_4_ALL_C_HL_DELTA_FE: FreeEnergy = - INVERSE_TEMPERATURE * 0.3;
pub const CONST_4_ALL_C_HL_DELTA_FE: FreeEnergy = - INVERSE_TEMPERATURE * 1.6;
lazy_static! {
  pub static ref EXP_COEFFICIENT_4_LOG_EXTRAPOLATION_OF_INIT_HL_DELTA_FE: FreeEnergy = COEFFICIENT_4_LOG_EXTRAPOLATION_OF_INIT_HL_DELTA_FE.exp();
  pub static ref EXP_HL_UU_OR_GA_FIRST_MISMATCH_BONUS_DELTA_FE: FreeEnergy = HL_UU_OR_GA_FIRST_MISMATCH_BONUS_DELTA_FE.exp();
  pub static ref EXP_HL_GG_FIRST_MISMATCH_BONUS_DELTA_FE: FreeEnergy = HL_GG_FIRST_MISMATCH_BONUS_DELTA_FE.exp();
  pub static ref EXP_HL_SPECIAL_GU_CLOSURE_BONUS_DELTA_FE: FreeEnergy = HL_SPECIAL_GU_CLOSURE_BONUS_DELTA_FE.exp();
  pub static ref EXP_HL_OF_3_CS_PENALTY_DELTA_FE: FreeEnergy = HL_OF_3_CS_PENALTY_DELTA_FE.exp();
  pub static ref EXP_COEFFICIENT_4_ALL_C_HL_DELTA_FE: FreeEnergy = COEFFICIENT_4_ALL_C_HL_DELTA_FE.exp();
  pub static ref EXP_CONST_4_ALL_C_HL_DELTA_FE: FreeEnergy = CONST_4_ALL_C_HL_DELTA_FE.exp();
  pub static ref INIT_HL_DELTA_FES: InitHlDeltaFes = {
    let mut init_hl_delta_fes: InitHlDeltaFes = vec![0., 0., 0., 5.4, 5.6, 5.7, 5.4, 6.0, 5.5, 6.4].iter().map(|&x| {scale(x)}).collect();
    let len_of_init_hl_delta_fes = init_hl_delta_fes.len();
    let basic_init_hl_delta_fe = init_hl_delta_fes[MIN_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_HL_DELTA_FE - 1];
    for i in len_of_init_hl_delta_fes .. MAX_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_LOOP_DELTA_FE {
      init_hl_delta_fes.push(basic_init_hl_delta_fe + COEFFICIENT_4_LOG_EXTRAPOLATION_OF_INIT_HL_DELTA_FE * (i as FreeEnergy / (MIN_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_HL_DELTA_FE - 1) as FreeEnergy).ln());
    }
    init_hl_delta_fes
  };
  pub static ref EXP_INIT_HL_DELTA_FES: InitHlDeltaFes = {INIT_HL_DELTA_FES.iter().map(|&x| {x.exp()}).collect()};
  pub static ref SPECIAL_HL_DELTA_FES: SpecialHlDeltaFes = {
    [
      (String::from("CAACG").into_bytes(), 6.8),
      (String::from("GUUAC").into_bytes(), 6.9),
      (String::from("CUACGG").into_bytes(), 2.8),
      (String::from("CUCCGG").into_bytes(), 2.7),
      (String::from("CUUCGG").into_bytes(), 3.7),
      (String::from("CUUUGG").into_bytes(), 3.7),
      (String::from("CCAAGG").into_bytes(), 3.3),
      (String::from("CCCAGG").into_bytes(), 3.4),
      (String::from("CCGAGG").into_bytes(), 3.5),
      (String::from("CCUAGG").into_bytes(), 3.7),
      (String::from("CCACGG").into_bytes(), 3.7),
      (String::from("CCGCGG").into_bytes(), 3.6),
      (String::from("CCUCGG").into_bytes(), 2.5),
      (String::from("CUAAGG").into_bytes(), 3.6),
      (String::from("CUCAGG").into_bytes(), 3.7),
      (String::from("CUUAGG").into_bytes(), 3.5),
      (String::from("CUGCGG").into_bytes(), 2.8),
      (String::from("CAACGG").into_bytes(), 5.5),
      (String::from("ACAGUGCU").into_bytes(), 2.9),
      (String::from("ACAGUGAU").into_bytes(), 3.6),
      (String::from("ACAGUGUU").into_bytes(), 1.8),
      (String::from("ACAGUACU").into_bytes(), 2.8),
    ].iter().map(|(x, y)| {(x.clone(), scale(*y))}).collect()
  };
  pub static ref EXP_SPECIAL_HL_DELTA_FES: SpecialHlDeltaFes = {SPECIAL_HL_DELTA_FES.iter().map(|(x, &y)| {(x.clone(), y.exp())}).collect()};
}
