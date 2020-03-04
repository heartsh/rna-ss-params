use utils::*;

pub type InitHlDeltaFes = Vec<FreeEnergy>;
pub type SpecialHlDeltaFes = [(Seq, FreeEnergy); NUM_OF_SPECIAL_HLS];

pub const NUM_OF_SPECIAL_HLS: usize = 22;
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
      (vec![C,A,A,C,G], scale(6.8)),
      (vec![G,U,U,A,C], scale(6.9)),
      (vec![C,U,A,C,G,G], scale(2.8)),
      (vec![C,U,C,C,G,G], scale(2.7)),
      (vec![C,U,U,C,G,G], scale(3.7)),
      (vec![C,U,U,U,G,G], scale(3.7)),
      (vec![C,C,A,A,G,G], scale(3.3)),
      (vec![C,C,C,A,G,G], scale(3.4)),
      (vec![C,C,G,A,G,G], scale(3.5)),
      (vec![C,C,U,A,G,G], scale(3.7)),
      (vec![C,C,A,C,G,G], scale(3.7)),
      (vec![C,C,G,C,G,G], scale(3.6)),
      (vec![C,C,U,C,G,G], scale(2.5)),
      (vec![C,U,A,A,G,G], scale(3.6)),
      (vec![C,U,C,A,G,G], scale(3.7)),
      (vec![C,U,U,A,G,G], scale(3.5)),
      (vec![C,U,G,C,G,G], scale(2.8)),
      (vec![C,A,A,C,G,G], scale(5.5)),
      (vec![A,C,A,G,U,G,C,U], scale(2.9)),
      (vec![A,C,A,G,U,G,A,U], scale(3.6)),
      (vec![A,C,A,G,U,G,U,U], scale(1.8)),
      (vec![A,C,A,G,U,A,C,U], scale(2.8)),
    ]
  };
  pub static ref EXP_SPECIAL_HL_DELTA_FES: SpecialHlDeltaFes = {
    let mut special_hl_delta_fes = SPECIAL_HL_DELTA_FES.clone();
    for special_hl_delta_fe in &mut special_hl_delta_fes {
      special_hl_delta_fe.1 = special_hl_delta_fe.1.exp(); 
    }
    special_hl_delta_fes
  };
}
