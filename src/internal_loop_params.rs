use utils::*;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use fnv::FnvHasher;

pub type InitInternalLoopDeltaFes = Vec<FreeEnergy>;
type Hasher = BuildHasherDefault<FnvHasher>;
pub type Init1Vs1InternalLoopDeltaFes = HashMap<(BasePair, BasePair, BasePair), FreeEnergy, Hasher>;
pub type OneVs2InternalLoop = (BasePair, Base);
pub type Init1Vs2InternalLoopDeltaFes = HashMap<(BasePair, OneVs2InternalLoop, BasePair), FreeEnergy, Hasher>;
pub type TwoVs2InternalLoop = (BasePair, BasePair);
pub type Init2Vs2InternalLoopDeltaFes = HashMap<(BasePair, TwoVs2InternalLoop, BasePair), FreeEnergy, Hasher>;

pub const MIN_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_INTERNAL_LOOP_DELTA_FE: usize = 7;
pub const COEFFICENT_4_LOG_EXTRAPOLATION_OF_INIT_INTERNAL_LOOP_DELTA_FE: FreeEnergy = 1.08;
lazy_static! {
  pub static ref INIT_INTERNAL_LOOP_DELTA_FES: InitInternalLoopDeltaFes = {
    let mut init_internal_loop_delta_fes = vec![0., 0., 0., 0., 1.1, 2.0, 2.0, 2.1, 2.3, 2.4, 2.5, 2.6, 2.7, 2.8, 2.9, 2.9, 3.0, 3.1, 3.1, 3.2, 3.3, 3.3, 3.4, 3.4, 3.5, 3.5, 3.5, 3.6, 3.6, 3.7, 3.7];
    let len_of_init_internal_loop_delta_fes = init_internal_loop_delta_fes.len();
    let basic_init_internal_loop_delta_fe = init_internal_loop_delta_fes[MIN_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_INTERNAL_LOOP_DELTA_FE - 1];
    for i in len_of_init_internal_loop_delta_fes .. MAX_LOOP_LEN_4_EXTRAPOLATION_OF_INIT_LOOP_DELTA_FE {
      init_internal_loop_delta_fes.push((basic_init_internal_loop_delta_fe + COEFFICENT_4_LOG_EXTRAPOLATION_OF_INIT_INTERNAL_LOOP_DELTA_FE * fast_ln(i as FreeEnergy / (MIN_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_INTERNAL_LOOP_DELTA_FE - 1) as FreeEnergy)) / KL);
    }
    init_internal_loop_delta_fes
  };
  pub static ref INIT_1_VS_1_INTERNAL_LOOP_DELTA_FES: Init1Vs1InternalLoopDeltaFes = {
    [
      // For internal pairs behind the base pair "AU".
      // For internal pairs between the base pairs "AU" and "AU".
      ((AU, AA, AU), 1.9), ((AU, AC, AU), 1.9), ((AU, AG, AU), 1.9), ((AU, AU, AU), 1.9),
      ((AU, CA, AU), 1.9), ((AU, CC, AU), 1.9), ((AU, CG, AU), 1.9), ((AU, CU, AU), 1.9),
      ((AU, GA, AU), 1.9), ((AU, GC, AU), 1.9), ((AU, GG, AU), -0.7), ((AU, GU, AU), 1.9),
      ((AU, UA, AU), 1.9), ((AU, UC, AU), 1.9), ((AU, UG, AU), 1.9), ((AU, UU, AU), 1.5),
      // For internal pairs between the base pairs "AU" and "CG".
      ((AU, AA, CG), 1.2), ((AU, AC, CG), 1.2), ((AU, AG, CG), 1.2), ((AU, AU, CG), 1.2),
      ((AU, CA, CG), 1.2), ((AU, CC, CG), 1.2), ((AU, CG, CG), 1.2), ((AU, CU, CG), 1.2),
      ((AU, GA, CG), 1.2), ((AU, GC, CG), 1.2), ((AU, GG, CG), -1.4), ((AU, GU, CG), 1.2),
      ((AU, UA, CG), 1.2), ((AU, UC, CG), 1.2), ((AU, UG, CG), 1.2), ((AU, UU, CG), 0.8),
      // For internal pairs between the base pairs "AU" and "GC".
      ((AU, AA, GC), 1.2), ((AU, AC, GC), 1.2), ((AU, AG, GC), 1.2), ((AU, AU, GC), 1.2),
      ((AU, CA, GC), 1.2), ((AU, CC, GC), 1.2), ((AU, CG, GC), 1.2), ((AU, CU, GC), 1.2),
      ((AU, GA, GC), 1.2), ((AU, GC, GC), 1.2), ((AU, GG, GC), -1.4), ((AU, GU, GC), 1.2),
      ((AU, UA, GC), 1.2), ((AU, UC, GC), 1.2), ((AU, UG, GC), 1.2), ((AU, UU, GC), 0.8),
      // For internal pairs between the base pairs "AU" and "UA".
      ((AU, AA, UA), 1.9), ((AU, AC, UA), 1.9), ((AU, AG, UA), 1.9), ((AU, AU, UA), 1.9),
      ((AU, CA, UA), 1.9), ((AU, CC, UA), 1.9), ((AU, CG, UA), 1.9), ((AU, CU, UA), 1.9),
      ((AU, GA, UA), 1.9), ((AU, GC, UA), 1.9), ((AU, GG, UA), -0.7), ((AU, GU, UA), 1.9),
      ((AU, UA, UA), 1.9), ((AU, UC, UA), 1.9), ((AU, UG, UA), 1.9), ((AU, UU, UA), 1.2),
      // For internal pairs between the base pairs "AU" and "GU".
      ((AU, AA, GU), 1.9), ((AU, AC, GU), 1.9), ((AU, AG, GU), 1.9), ((AU, AU, GU), 1.9),
      ((AU, CA, GU), 1.9), ((AU, CC, GU), 1.9), ((AU, CG, GU), 1.9), ((AU, CU, GU), 1.9),
      ((AU, GA, GU), 1.9), ((AU, GC, GU), 1.9), ((AU, GG, GU), -0.7), ((AU, GU, GU), 1.9),
      ((AU, UA, GU), 1.9), ((AU, UC, GU), 1.9), ((AU, UG, GU), 1.9), ((AU, UU, GU), 1.6),
      // For internal pairs between the base pairs "AU" and "UG".
      ((AU, AA, UG), 1.9), ((AU, AC, UG), 1.9), ((AU, AG, UG), 1.9), ((AU, AU, UG), 1.9),
      ((AU, CA, UG), 1.9), ((AU, CC, UG), 1.9), ((AU, CG, UG), 1.9), ((AU, CU, UG), 1.9),
      ((AU, GA, UG), 1.9), ((AU, GC, UG), 1.9), ((AU, GG, UG), -0.7), ((AU, GU, UG), 1.9),
      ((AU, UA, UG), 1.9), ((AU, UC, UG), 1.9), ((AU, UG, UG), 1.9), ((AU, UU, UG), 1.2),
      // For internal pairs behind the base pair "CG".
      // For internal pairs between the base pairs "CG" and "AU".
      ((CG, AA, AU), 1.2), ((CG, AC, AU), 1.2), ((CG, AG, AU), 1.2), ((CG, AU, AU), 1.2),
      ((CG, CA, AU), 1.2), ((CG, CC, AU), 1.2), ((CG, CG, AU), 1.2), ((CG, CU, AU), 1.2),
      ((CG, GA, AU), 1.2), ((CG, GC, AU), 1.2), ((CG, GG, AU), -1.4), ((CG, GU, AU), 1.2),
      ((CG, UA, AU), 1.2), ((CG, UC, AU), 1.2), ((CG, UG, AU), 1.2), ((CG, UU, AU), 1.2),
      // For internal pairs between the base pairs "CG" and "CG".
      ((CG, AA, CG), 0.9), ((CG, AC, CG), -0.4), ((CG, AG, CG), 0.5), ((CG, AU, CG), 0.5),
      ((CG, CA, CG), 0.3), ((CG, CC, CG), 0.5), ((CG, CG, CG), 0.5), ((CG, CU, CG), 0.6),
      ((CG, GA, CG), -0.1), ((CG, GC, CG), 0.5), ((CG, GG, CG), -2.2), ((CG, GU, CG), 0.5),
      ((CG, UA, CG), 0.5), ((CG, UC, CG), 0.0), ((CG, UG, CG), 0.5), ((CG, UU, CG), -0.1),
      // For internal pairs between the base pairs "CG" and "GC".
      ((CG, AA, GC), 0.9), ((CG, AC, GC), 0.5), ((CG, AG, GC), 0.5), ((CG, AU, GC), 0.5),
      ((CG, CA, GC), 0.5), ((CG, CC, GC), 0.5), ((CG, CG, GC), 0.5), ((CG, CU, GC), 0.5),
      ((CG, GA, GC), 0.5), ((CG, GC, GC), 0.5), ((CG, GG, GC), -1.4), ((CG, GU, GC), 0.5),
      ((CG, UA, GC), 0.5), ((CG, UC, GC), 0.5), ((CG, UG, GC), 0.5), ((CG, UU, GC), 0.4),
      // For internal pairs between the base pairs "CG" and "UA".
      ((CG, AA, UA), 1.2), ((CG, AC, UA), 1.2), ((CG, AG, UA), 1.2), ((CG, AU, UA), 1.2),
      ((CG, CA, UA), 1.2), ((CG, CC, UA), 1.2), ((CG, CG, UA), 1.2), ((CG, CU, UA), 1.2),
      ((CG, GA, UA), 1.2), ((CG, GC, UA), 1.2), ((CG, GG, UA), -1.4), ((CG, GU, UA), 1.2),
      ((CG, UA, UA), 1.2), ((CG, UC, UA), 1.2), ((CG, UG, UA), 1.2), ((CG, UU, UA), 0.8),
      // For internal pairs between the base pairs "CG" and "GU".
      ((CG, AA, GU), 2.2), ((CG, AC, GU), 1.3), ((CG, AG, GU), 1.2), ((CG, AU, GU), 1.2),
      ((CG, CA, GU), 1.2), ((CG, CC, GU), 1.7), ((CG, CG, GU), 1.2), ((CG, CU, GU), 1.2),
      ((CG, GA, GU), 1.2), ((CG, GC, GU), 1.2), ((CG, GG, GU), -1.4), ((CG, GU, GU), 1.2),
      ((CG, UA, GU), 1.2), ((CG, UC, GU), 1.2), ((CG, UG, GU), 1.2), ((CG, UU, GU), 1.1),
      // For internal pairs between the base pairs "CG" and "UG".
      ((CG, AA, UG), 0.6), ((CG, AC, UG), 0.5), ((CG, AG, UG), 1.2), ((CG, AU, UG), 1.2),
      ((CG, CA, UG), 1.2), ((CG, CC, UG), 1.2), ((CG, CG, UG), 1.2), ((CG, CU, UG), 1.2),
      ((CG, GA, UG), -0.2), ((CG, GC, UG), 1.2), ((CG, GG, UG), -1.4), ((CG, GU, UG), 1.2),
      ((CG, UA, UG), 1.2), ((CG, UC, UG), 1.0), ((CG, UG, UG), 1.2), ((CG, UU, UG), 1.1),
      // For internal pairs behind the base pair "GC".
      // For internal pairs between the base pairs "GC" and "AU".
      ((GC, AA, AU), 1.2), ((GC, AC, AU), 1.2), ((GC, AG, AU), 1.2), ((GC, AU, AU), 1.2),
      ((GC, CA, AU), 1.2), ((GC, CC, AU), 1.2), ((GC, CG, AU), 1.2), ((GC, CU, AU), 1.2),
      ((GC, GA, AU), 1.2), ((GC, GC, AU), 1.2), ((GC, GG, AU), -1.4), ((GC, GU, AU), 1.2),
      ((GC, UA, AU), 1.2), ((GC, UC, AU), 1.2), ((GC, UG, AU), 1.2), ((GC, UU, AU), 1.2),
      // For internal pairs between the base pairs "GC" and "CG".
      ((GC, AA, CG), 0.8), ((GC, AC, CG), 0.5), ((GC, AG, CG), 0.5), ((GC, AU, CG), 0.5),
      ((GC, CA, CG), 0.5), ((GC, CC, CG), 0.5), ((GC, CG, CG), 0.5), ((GC, CU, CG), 0.5),
      ((GC, GA, CG), 0.5), ((GC, GC, CG), 0.5), ((GC, GG, CG), -2.3), ((GC, GU, CG), 0.5),
      ((GC, UA, CG), 0.5), ((GC, UC, CG), 0.5), ((GC, UG, CG), 0.5), ((GC, UU, CG), -0.6),
      // For internal pairs between the base pairs "GC" and "GC".
      ((GC, AA, GC), 0.9), ((GC, AC, GC), 0.3), ((GC, AG, GC), -0.1), ((GC, AU, GC), 0.5),
      ((GC, CA, GC), -0.4), ((GC, CC, GC), 0.5), ((GC, CG, GC), 0.5), ((GC, CU, GC), 0.0),
      ((GC, GA, GC), 0.5), ((GC, GC, GC), 0.5), ((GC, GG, GC), -2.2), ((GC, GU, GC), 0.5),
      ((GC, UA, GC), 0.5), ((GC, UC, GC), 0.6), ((GC, UG, GC), 0.5), ((GC, UU, GC), -0.1),
      // For internal pairs between the base pairs "GC" and "UA".
      ((GC, AA, UA), 1.2), ((GC, AC, UA), 1.2), ((GC, AG, UA), 1.2), ((GC, AU, UA), 1.2),
      ((GC, CA, UA), 1.2), ((GC, CC, UA), 1.2), ((GC, CG, UA), 1.2), ((GC, CU, UA), 1.2),
      ((GC, GA, UA), 1.2), ((GC, GC, UA), 1.2), ((GC, GG, UA), -1.4), ((GC, GU, UA), 1.2),
      ((GC, UA, UA), 1.2), ((GC, UC, UA), 1.2), ((GC, UG, UA), 1.2), ((GC, UU, UA), 0.8),
      // For internal pairs between the base pairs "GC" and "GU".
      ((GC, AA, GU), 1.6), ((GC, AC, GU), 1.2), ((GC, AG, GU), 1.0), ((GC, AU, GU), 1.2),
      ((GC, CA, GU), 1.2), ((GC, CC, GU), 1.2), ((GC, CG, GU), 1.2), ((GC, CU, GU), 1.2),
      ((GC, GA, GU), 1.2), ((GC, GC, GU), 1.2), ((GC, GG, GU), -1.4), ((GC, GU, GU), 1.2),
      ((GC, UA, GU), 1.2), ((GC, UC, GU), 1.2), ((GC, UG, GU), 1.2), ((GC, UU, GU), 0.7),
      // For internal pairs between the base pairs "GC" and "UG".
      ((GC, AA, UG), 1.9), ((GC, AC, UG), 1.2), ((GC, AG, UG), 1.5), ((GC, AU, UG), 1.2),
      ((GC, CA, UG), 1.2), ((GC, CC, UG), 1.2), ((GC, CG, UG), 1.2), ((GC, CU, UG), 1.2),
      ((GC, GA, UG), 1.2), ((GC, GC, UG), 1.2), ((GC, GG, UG), -1.4), ((GC, GU, UG), 1.2),
      ((GC, UA, UG), 1.2), ((GC, UC, UG), 1.2), ((GC, UG, UG), 1.2), ((GC, UU, UG), 1.5),
      // For internal pairs behind the base pair "UA".
      // For internal pairs between the base pairs "UA" and "AU".
      ((UA, AA, AU), 1.9), ((UA, AC, AU), 1.9), ((UA, AG, AU), 1.9), ((UA, AU, AU), 1.9),
      ((UA, CA, AU), 1.9), ((UA, CC, AU), 1.9), ((UA, CG, AU), 1.9), ((UA, CU, AU), 1.9),
      ((UA, GA, AU), 1.9), ((UA, GC, AU), 1.9), ((UA, GG, AU), -0.7), ((UA, GU, AU), 1.9),
      ((UA, UA, AU), 1.9), ((UA, UC, AU), 1.9), ((UA, UG, AU), 1.9), ((UA, UU, AU), 1.7),
      // For internal pairs between the base pairs "UA" and "CG".
      ((UA, AA, CG), 1.2), ((UA, AC, CG), 1.2), ((UA, AG, CG), 1.2), ((UA, AU, CG), 1.2),
      ((UA, CA, CG), 1.2), ((UA, CC, CG), 1.2), ((UA, CG, CG), 1.2), ((UA, CU, CG), 1.2),
      ((UA, GA, CG), 1.2), ((UA, GC, CG), 1.2), ((UA, GG, CG), -1.4), ((UA, GU, CG), 1.2),
      ((UA, UA, CG), 1.2), ((UA, UC, CG), 1.2), ((UA, UG, CG), 1.2), ((UA, UU, CG), 1.2),
      // For internal pairs between the base pairs "UA" and "GC".
      ((UA, AA, GC), 1.2), ((UA, AC, GC), 1.2), ((UA, AG, GC), 1.2), ((UA, AU, GC), 1.2),
      ((UA, CA, GC), 1.2), ((UA, CC, GC), 1.2), ((UA, CG, GC), 1.2), ((UA, CU, GC), 1.2),
      ((UA, GA, GC), 1.2), ((UA, GC, GC), 1.2), ((UA, GG, GC), -1.4), ((UA, GU, GC), 1.2),
      ((UA, UA, GC), 1.2), ((UA, UC, GC), 1.2), ((UA, UG, GC), 1.2), ((UA, UU, GC), 1.2),
      // For internal pairs between the base pairs "UA" and "UA".
      ((UA, AA, UA), 1.9), ((UA, AC, UA), 1.9), ((UA, AG, UA), 1.9), ((UA, AU, UA), 1.9),
      ((UA, CA, UA), 1.9), ((UA, CC, UA), 1.9), ((UA, CG, UA), 1.9), ((UA, CU, UA), 1.9),
      ((UA, GA, UA), 1.9), ((UA, GC, UA), 1.9), ((UA, GG, UA), -0.7), ((UA, GU, UA), 1.9),
      ((UA, UA, UA), 1.9), ((UA, UC, UA), 1.9), ((UA, UG, UA), 1.9), ((UA, UU, UA), 1.9),
      // For internal pairs between the base pairs "UA" and "GU".
      ((UA, AA, GU), 1.9), ((UA, AC, GU), 1.9), ((UA, AG, GU), 1.9), ((UA, AU, GU), 1.9),
      ((UA, CA, GU), 1.9), ((UA, CC, GU), 1.9), ((UA, CG, GU), 1.9), ((UA, CU, GU), 1.9),
      ((UA, GA, GU), 1.9), ((UA, GC, GU), 1.9), ((UA, GG, GU), -0.7), ((UA, GU, GU), 1.9),
      ((UA, UA, GU), 1.9), ((UA, UC, GU), 1.9), ((UA, UG, GU), 1.9), ((UA, UU, GU), 1.9),
      // For internal pairs between the base pairs "UA" and "UG".
      ((UA, AA, UG), 1.9), ((UA, AC, UG), 1.9), ((UA, AG, UG), 1.9), ((UA, AU, UG), 1.9),
      ((UA, CA, UG), 1.9), ((UA, CC, UG), 1.9), ((UA, CG, UG), 1.9), ((UA, CU, UG), 1.9),
      ((UA, GA, UG), 1.9), ((UA, GC, UG), 1.9), ((UA, GG, UG), -0.7), ((UA, GU, UG), 1.9),
      ((UA, UA, UG), 1.9), ((UA, UC, UG), 1.9), ((UA, UG, UG), 1.9), ((UA, UU, UG), 1.6),
      // For internal pairs behind the base pair "GU".
      // For internal pairs between the base pairs "GU" and "AU".
      ((GU, AA, AU), 1.9), ((GU, AC, AU), 1.9), ((GU, AG, AU), 1.9), ((GU, AU, AU), 1.9),
      ((GU, CA, AU), 1.9), ((GU, CC, AU), 1.9), ((GU, CG, AU), 1.9), ((GU, CU, AU), 1.9),
      ((GU, GA, AU), 1.9), ((GU, GC, AU), 1.9), ((GU, GG, AU), -0.7), ((GU, GU, AU), 1.9),
      ((GU, UA, AU), 1.9), ((GU, UC, AU), 1.9), ((GU, UG, AU), 1.6), ((GU, UU, AU), 1.2),
      // For internal pairs between the base pairs "GU" and "CG".
      ((GU, AA, CG), 1.9), ((GU, AC, CG), 1.2), ((GU, AG, CG), 1.2), ((GU, AU, CG), 1.2),
      ((GU, CA, CG), 1.2), ((GU, CC, CG), 1.2), ((GU, CG, CG), 1.2), ((GU, CU, CG), 1.2),
      ((GU, GA, CG), 1.9), ((GU, GC, CG), 1.2), ((GU, GG, CG), -1.4), ((GU, GU, CG), 1.2),
      ((GU, UA, CG), 1.2), ((GU, UC, CG), 1.2), ((GU, UG, CG), 1.2), ((GU, UU, CG), 1.5),
      // For internal pairs between the base pairs "GU" and "GC".
      ((GU, AA, GC), 0.6), ((GU, AC, GC), 1.2), ((GU, AG, GC), -0.2), ((GU, AU, GC), 1.2),
      ((GU, CA, GC), 0.5), ((GU, CC, GC), 1.2), ((GU, CG, GC), 1.2), ((GU, CU, GC), 1.0),
      ((GU, GA, GC), 1.2), ((GU, GC, GC), 1.2), ((GU, GG, GC), -1.4), ((GU, GU, GC), 1.2),
      ((GU, UA, GC), 1.5), ((GU, UC, GC), 1.2), ((GU, UG, GC), 1.2), ((GU, UU, GC), 1.1),
      // For internal pairs between the base pairs "GU" and "UA".
      ((GU, AA, UA), 1.9), ((GU, AC, UA), 1.9), ((GU, AG, UA), 1.9), ((GU, AU, UA), 1.9),
      ((GU, CA, UA), 1.9), ((GU, CC, UA), 1.9), ((GU, CG, UA), 1.9), ((GU, CU, UA), 1.9),
      ((GU, GA, UA), 1.9), ((GU, GC, UA), 1.9), ((GU, GG, UA), -0.7), ((GU, GU, UA), 1.9),
      ((GU, UA, UA), 1.9), ((GU, UC, UA), 1.9), ((GU, UG, UA), 1.9), ((GU, UU, UA), 1.2),
      // For internal pairs between the base pairs "GU" and "GU".
      ((GU, AA, GU), 1.9), ((GU, AC, GU), 1.9), ((GU, AG, GU), 1.9), ((GU, AU, GU), 1.9),
      ((GU, CA, GU), 1.9), ((GU, CC, GU), 1.9), ((GU, CG, GU), 1.9), ((GU, CU, GU), 1.9),
      ((GU, GA, GU), 1.9), ((GU, GC, GU), 1.9), ((GU, GG, GU), -0.7), ((GU, GU, GU), 1.9),
      ((GU, UA, GU), 1.9), ((GU, UC, GU), 1.9), ((GU, UG, GU), 1.9), ((GU, UU, GU), 1.6),
      // For internal pairs between the base pairs "GU" and "UG".
      ((GU, AA, UG), 1.9), ((GU, AC, UG), 1.9), ((GU, AG, UG), 1.9), ((GU, AU, UG), 1.9),
      ((GU, CA, UG), 1.9), ((GU, CC, UG), 1.9), ((GU, CG, UG), 1.9), ((GU, CU, UG), 1.9),
      ((GU, GA, UG), 1.9), ((GU, GC, UG), 1.9), ((GU, GG, UG), -0.7), ((GU, GU, UG), 1.9),
      ((GU, UA, UG), 1.9), ((GU, UC, UG), 1.9), ((GU, UG, UG), 1.9), ((GU, UU, UG), 1.2),
      // For internal pairs behind the base pair "UG".
      // For internal pairs between the base pairs "UG" and "AU".
      ((UG, AA, AU), 1.9), ((UG, AC, AU), 1.9), ((UG, AG, AU), 1.9), ((UG, AU, AU), 1.9),
      ((UG, CA, AU), 1.9), ((UG, CC, AU), 1.9), ((UG, CG, AU), 1.9), ((UG, CU, AU), 1.9),
      ((UG, GA, AU), 1.9), ((UG, GC, AU), 1.9), ((UG, GG, AU), -0.7), ((UG, GU, AU), 1.9),
      ((UG, UA, AU), 1.9), ((UG, UC, AU), 1.9), ((UG, UG, AU), 1.9), ((UG, UU, AU), 1.2),
      // For internal pairs between the base pairs "UG" and "CG".
      ((UG, AA, CG), 1.6), ((UG, AC, CG), 1.2), ((UG, AG, CG), 1.2), ((UG, AU, CG), 1.2),
      ((UG, CA, CG), 1.2), ((UG, CC, CG), 1.2), ((UG, CG, CG), 1.2), ((UG, CU, CG), 1.2),
      ((UG, GA, CG), 1.0), ((UG, GC, CG), 1.2), ((UG, GG, CG), -1.4), ((UG, GU, CG), 1.2),
      ((UG, UA, CG), 1.2), ((UG, UC, CG), 1.2), ((UG, UG, CG), 1.2), ((UG, UU, CG), 0.7),
      // For internal pairs between the base pairs "UG" and "GC".
      ((UG, AA, GC), 2.2), ((UG, AC, GC), 1.2), ((UG, AG, GC), 1.2), ((UG, AU, GC), 1.2),
      ((UG, CA, GC), 1.3), ((UG, CC, GC), 1.7), ((UG, CG, GC), 1.2), ((UG, CU, GC), 1.2),
      ((UG, GA, GC), 1.2), ((UG, GC, GC), 1.2), ((UG, GG, GC), -1.4), ((UG, GU, GC), 1.2),
      ((UG, UA, GC), 1.2), ((UG, UC, GC), 1.2), ((UG, UG, GC), 1.2), ((UG, UU, GC), 1.1),
      // For internal pairs between the base pairs "UG" and "UA".
      ((UG, AA, UA), 1.9), ((UG, AC, UA), 1.9), ((UG, AG, UA), 1.9), ((UG, AU, UA), 1.9),
      ((UG, CA, UA), 1.9), ((UG, CC, UA), 1.9), ((UG, CG, UA), 1.9), ((UG, CU, UA), 1.9),
      ((UG, GA, UA), 1.9), ((UG, GC, UA), 1.9), ((UG, GG, UA), -0.7), ((UG, GU, UA), 1.9),
      ((UG, UA, UA), 1.9), ((UG, UC, UA), 1.9), ((UG, UG, UA), 1.9), ((UG, UU, UA), 1.6),
      // For internal pairs between the base pairs "UG" and "GU".
      ((UG, AA, GU), 1.9), ((UG, AC, GU), 1.9), ((UG, AG, GU), 1.9), ((UG, AU, GU), 1.9),
      ((UG, CA, GU), 1.9), ((UG, CC, GU), 1.9), ((UG, CG, GU), 1.9), ((UG, CU, GU), 1.9),
      ((UG, GA, GU), 1.9), ((UG, GC, GU), 1.9), ((UG, GG, GU), -0.7), ((UG, GU, GU), 1.9),
      ((UG, UA, GU), 1.9), ((UG, UC, GU), 1.9), ((UG, UG, GU), 1.9), ((UG, UU, GU), 1.9),
      // For internal pairs between the base pairs "UG" and "UG".
      ((UG, AA, UG), 1.9), ((UG, AC, UG), 1.9), ((UG, AG, UG), 1.9), ((UG, AU, UG), 1.9),
      ((UG, CA, UG), 1.9), ((UG, CC, UG), 1.9), ((UG, CG, UG), 1.9), ((UG, CU, UG), 1.9),
      ((UG, GA, UG), 1.9), ((UG, GC, UG), 1.9), ((UG, GG, UG), -0.7), ((UG, GU, UG), 1.9),
      ((UG, UA, UG), 1.9), ((UG, UC, UG), 1.9), ((UG, UG, UG), 1.9), ((UG, UU, UG), 1.6),
    ].iter().cloned().collect()
  };
  pub static ref INIT_1_VS_2_INTERNAL_LOOP_DELTA_FES: Init1Vs2InternalLoopDeltaFes = {
    [
      // For internal loops behind the base pair "AU".
      // For internal loops between the base pairs "AU" and "AU".
      ((AU, (AA, A), AU), 3.7), ((AU, (AC, A), AU), 3.7), ((AU, (AG, A), AU), 3.7), ((AU, (AU, A), AU), 3.7),
      ((AU, (CA, A), AU), 3.7), ((AU, (CC, A), AU), 3.7), ((AU, (CG, A), AU), 3.7), ((AU, (CU, A), AU), 3.7),
      ((AU, (GA, A), AU), 2.6), ((AU, (GC, A), AU), 2.6), ((AU, (GG, A), AU), 2.6), ((AU, (GU, A), AU), 2.6),
      ((AU, (UA, A), AU), 3.7), ((AU, (UC, A), AU), 3.7), ((AU, (UG, A), AU), 3.7), ((AU, (UU, A), AU), 3.0),
      // For internal loops between the base pairs "AU" and "CG".
      ((AU, (AA, A), CG), 3.0), ((AU, (AC, A), CG), 3.0), ((AU, (AG, A), CG), 3.0), ((AU, (AU, A), CG), 3.0),
      ((AU, (CA, A), CG), 3.0), ((AU, (CC, A), CG), 3.0), ((AU, (CG, A), CG), 3.0), ((AU, (CU, A), CG), 3.0),
      ((AU, (GA, A), CG), 1.9), ((AU, (GC, A), CG), 3.0), ((AU, (GG, A), CG), 1.9), ((AU, (GU, A), CG), 3.0),
      ((AU, (UA, A), CG), 3.0), ((AU, (UC, A), CG), 3.0), ((AU, (UG, A), CG), 3.0), ((AU, (UU, A), CG), 2.2),
      // For internal loops between the base pairs "AU" and "GC".
      ((AU, (AA, A), GC), 3.0), ((AU, (AC, A), GC), 3.0), ((AU, (AG, A), GC), 3.0), ((AU, (AU, A), GC), 3.0),
      ((AU, (CA, A), GC), 3.0), ((AU, (CC, A), GC), 3.0), ((AU, (CG, A), GC), 3.0), ((AU, (CU, A), GC), 3.0),
      ((AU, (GA, A), GC), 1.9), ((AU, (GC, A), GC), 1.9), ((AU, (GG, A), GC), 1.9), ((AU, (GU, A), GC), 1.9),
      ((AU, (UA, A), GC), 3.0), ((AU, (UC, A), GC), 3.0), ((AU, (UG, A), GC), 3.0), ((AU, (UU, A), GC), 2.2),
      // For internal loops between the base pairs "AU" and "UA".
      ((AU, (AA, A), UA), 3.7), ((AU, (AC, A), UA), 3.7), ((AU, (AG, A), UA), 3.7), ((AU, (AU, A), UA), 3.7),
      ((AU, (CA, A), UA), 3.7), ((AU, (CC, A), UA), 3.7), ((AU, (CG, A), UA), 3.7), ((AU, (CU, A), UA), 3.7),
      ((AU, (GA, A), UA), 2.6), ((AU, (GC, A), UA), 3.7), ((AU, (GG, A), UA), 2.6), ((AU, (GU, A), UA), 3.7),
      ((AU, (UA, A), UA), 3.7), ((AU, (UC, A), UA), 3.7), ((AU, (UG, A), UA), 3.7), ((AU, (UU, A), UA), 3.0),
      // For internal loops between the base pairs "AU" and "GU".
      ((AU, (AA, A), GU), 3.7), ((AU, (AC, A), GU), 3.7), ((AU, (AG, A), GU), 3.7), ((AU, (AU, A), GU), 3.7),
      ((AU, (CA, A), GU), 3.7), ((AU, (CC, A), GU), 3.7), ((AU, (CG, A), GU), 3.7), ((AU, (CU, A), GU), 3.7),
      ((AU, (GA, A), GU), 2.6), ((AU, (GC, A), GU), 2.6), ((AU, (GG, A), GU), 2.6), ((AU, (GU, A), GU), 2.6),
      ((AU, (UA, A), GU), 3.7), ((AU, (UC, A), GU), 3.7), ((AU, (UG, A), GU), 3.7), ((AU, (UU, A), GU), 3.0),
      // For internal loops between the base pairs "AU" and "UG".
      ((AU, (AA, A), UG), 3.7), ((AU, (AC, A), UG), 3.7), ((AU, (AG, A), UG), 3.7), ((AU, (AU, A), UG), 3.7),
      ((AU, (CA, A), UG), 3.7), ((AU, (CC, A), UG), 3.7), ((AU, (CG, A), UG), 3.7), ((AU, (CU, A), UG), 3.7),
      ((AU, (GA, A), UG), 2.6), ((AU, (GC, A), UG), 3.7), ((AU, (GG, A), UG), 2.6), ((AU, (GU, A), UG), 3.7),
      ((AU, (UA, A), UG), 3.7), ((AU, (UC, A), UG), 3.7), ((AU, (UG, A), UG), 3.7), ((AU, (UU, A), UG), 3.0),
      // For internal loops between the base pairs "AU" and "AU".
      ((AU, (AA, C), AU), 3.7), ((AU, (AC, C), AU), 3.7), ((AU, (AG, C), AU), 3.7), ((AU, (AU, C), AU), 3.7),
      ((AU, (CA, C), AU), 3.7), ((AU, (CC, C), AU), 3.7), ((AU, (CG, C), AU), 3.7), ((AU, (CU, C), AU), 3.7),
      ((AU, (GA, C), AU), 2.6), ((AU, (GC, C), AU), 3.7), ((AU, (GG, C), AU), 2.6), ((AU, (GU, C), AU), 3.7),
      ((AU, (UA, C), AU), 3.7), ((AU, (UC, C), AU), 3.7), ((AU, (UG, C), AU), 3.7), ((AU, (UU, C), AU), 3.0),
      // For internal loops between the base pairs "AU" and "CG".
      ((AU, (AA, C), CG), 3.0), ((AU, (AC, C), CG), 3.0), ((AU, (AG, C), CG), 3.0), ((AU, (AU, C), CG), 3.0),
      ((AU, (CA, C), CG), 3.0), ((AU, (CC, C), CG), 3.0), ((AU, (CG, A), CG), 3.0), ((AU, (CU, C), CG), 3.0),
      ((AU, (GA, C), CG), 1.9), ((AU, (GC, C), CG), 3.0), ((AU, (GG, C), CG), 1.9), ((AU, (GU, C), CG), 3.0),
      ((AU, (UA, C), CG), 3.0), ((AU, (UC, C), CG), 3.0), ((AU, (UG, C), CG), 3.0), ((AU, (UU, C), CG), 2.2),
      // For internal loops between the base pairs "AU" and "GC".
      ((AU, (AA, C), GC), 3.0), ((AU, (AC, C), GC), 3.0), ((AU, (AG, C), GC), 3.0), ((AU, (AU, C), GC), 3.0),
      ((AU, (CA, C), GC), 3.0), ((AU, (CC, C), GC), 3.0), ((AU, (CG, C), GC), 3.0), ((AU, (CU, C), GC), 3.0),
      ((AU, (GA, C), GC), 1.9), ((AU, (GC, C), GC), 3.0), ((AU, (GG, C), GC), 1.9), ((AU, (GU, C), GC), 3.0),
      ((AU, (UA, C), GC), 3.0), ((AU, (UC, C), GC), 3.0), ((AU, (UG, C), GC), 3.0), ((AU, (UU, C), GC), 2.2),
      // For internal loops between the base pairs "AU" and "UA".
      ((AU, (AA, C), UA), 3.7), ((AU, (AC, C), UA), 3.7), ((AU, (AG, C), UA), 3.7), ((AU, (AU, C), UA), 3.7),
      ((AU, (CA, C), UA), 3.7), ((AU, (CC, C), UA), 3.7), ((AU, (CG, C), UA), 3.7), ((AU, (CU, C), UA), 3.7),
      ((AU, (GA, C), UA), 2.6), ((AU, (GC, C), UA), 3.7), ((AU, (GG, C), UA), 2.6), ((AU, (GU, C), UA), 3.7),
      ((AU, (UA, C), UA), 3.7), ((AU, (UC, C), UA), 3.7), ((AU, (UG, C), UA), 3.7), ((AU, (UU, C), UA), 3.0),
      // For internal loops between the base pairs "AU" and "GU".
      ((AU, (AA, C), GU), 3.7), ((AU, (AC, C), GU), 3.7), ((AU, (AG, C), GU), 3.7), ((AU, (AU, C), GU), 3.7),
      ((AU, (CA, C), GU), 3.7), ((AU, (CC, C), GU), 3.7), ((AU, (CG, C), GU), 3.7), ((AU, (CU, C), GU), 3.7),
      ((AU, (GA, C), GU), 2.6), ((AU, (GC, C), GU), 3.7), ((AU, (GG, C), GU), 2.6), ((AU, (GU, C), GU), 3.7),
      ((AU, (UA, C), GU), 3.7), ((AU, (UC, C), GU), 3.7), ((AU, (UG, C), GU), 3.7), ((AU, (UU, C), GU), 3.0),
      // For internal loops between the base pairs "AU" and "UG".
      ((AU, (AA, C), UG), 3.7), ((AU, (AC, C), UG), 3.7), ((AU, (AG, C), UG), 3.7), ((AU, (AU, C), UG), 3.7),
      ((AU, (CA, C), UG), 3.7), ((AU, (CC, C), UG), 3.7), ((AU, (CG, C), UG), 3.7), ((AU, (CU, C), UG), 3.7),
      ((AU, (GA, C), UG), 2.6), ((AU, (GC, C), UG), 3.7), ((AU, (GG, C), UG), 2.6), ((AU, (GU, C), UG), 3.7),
      ((AU, (UA, C), UG), 3.7), ((AU, (UC, C), UG), 3.7), ((AU, (UG, C), UG), 3.7), ((AU, (UU, C), UG), 3.0),
      // For internal loops between the base pairs "AU" and "AU".
      ((AU, (AA, G), AU), 2.6), ((AU, (AC, G), AU), 2.6), ((AU, (AG, G), AU), 2.6), ((AU, (AU, G), AU), 2.6),
      ((AU, (CA, G), AU), 3.7), ((AU, (CC, G), AU), 3.7), ((AU, (CG, G), AU), 3.7), ((AU, (CU, G), AU), 3.7),
      ((AU, (GA, G), AU), 2.6), ((AU, (GC, G), AU), 2.6), ((AU, (GG, G), AU), 2.6), ((AU, (GU, G), AU), 2.6),
      ((AU, (UA, G), AU), 3.7), ((AU, (UC, G), AU), 3.7), ((AU, (UG, G), AU), 3.7), ((AU, (UU, G), AU), 3.0),
      // For internal loops between the base pairs "AU" and "CG".
      ((AU, (AA, G), CG), 1.9), ((AU, (AC, G), CG), 1.9), ((AU, (AG, G), CG), 1.9), ((AU, (AU, G), CG), 1.9),
      ((AU, (CA, G), CG), 3.0), ((AU, (CC, G), CG), 3.0), ((AU, (CG, A), CG), 3.0), ((AU, (CU, G), CG), 3.0),
      ((AU, (GA, G), CG), 1.9), ((AU, (GC, G), CG), 1.9), ((AU, (GG, G), CG), 1.9), ((AU, (GU, G), CG), 1.9),
      ((AU, (UA, G), CG), 3.0), ((AU, (UC, G), CG), 3.0), ((AU, (UG, G), CG), 3.0), ((AU, (UU, G), CG), 2.2),
      // For internal loops between the base pairs "AU" and "GC".
      ((AU, (AA, G), GC), 1.9), ((AU, (AC, G), GC), 1.9), ((AU, (AG, G), GC), 1.9), ((AU, (AU, G), GC), 1.9),
      ((AU, (CA, G), GC), 3.0), ((AU, (CC, G), GC), 3.0), ((AU, (CG, G), GC), 3.0), ((AU, (CU, G), GC), 3.0),
      ((AU, (GA, G), GC), 1.9), ((AU, (GC, G), GC), 1.9), ((AU, (GG, G), GC), 1.9), ((AU, (GU, G), GC), 1.9),
      ((AU, (UA, G), GC), 3.0), ((AU, (UC, G), GC), 3.0), ((AU, (UG, G), GC), 3.0), ((AU, (UU, G), GC), 2.2),
      // For internal loops between the base pairs "AU" and "UA".
      ((AU, (AA, G), UA), 2.6), ((AU, (AC, G), UA), 2.6), ((AU, (AG, G), UA), 2.6), ((AU, (AU, G), UA), 2.6),
      ((AU, (CA, G), UA), 3.7), ((AU, (CC, G), UA), 3.7), ((AU, (CG, G), UA), 3.7), ((AU, (CU, G), UA), 3.7),
      ((AU, (GA, G), UA), 2.6), ((AU, (GC, G), UA), 2.6), ((AU, (GG, G), UA), 2.6), ((AU, (GU, G), UA), 2.6),
      ((AU, (UA, G), UA), 3.7), ((AU, (UC, G), UA), 3.7), ((AU, (UG, G), UA), 3.7), ((AU, (UU, G), UA), 3.0),
      // For internal loops between the base pairs "AU" and "GU".
      ((AU, (AA, G), GU), 2.6), ((AU, (AC, G), GU), 2.6), ((AU, (AG, G), GU), 2.6), ((AU, (AU, G), GU), 2.6),
      ((AU, (CA, G), GU), 3.7), ((AU, (CC, G), GU), 3.7), ((AU, (CG, G), GU), 3.7), ((AU, (CU, G), GU), 3.7),
      ((AU, (GA, G), GU), 2.6), ((AU, (GC, G), GU), 2.6), ((AU, (GG, G), GU), 2.6), ((AU, (GU, G), GU), 2.6),
      ((AU, (UA, G), GU), 3.7), ((AU, (UC, G), GU), 3.7), ((AU, (UG, G), GU), 3.7), ((AU, (UU, G), GU), 3.0),
      // For internal loops between the base pairs "AU" and "UG".
      ((AU, (AA, G), UG), 2.6), ((AU, (AC, G), UG), 2.6), ((AU, (AG, G), UG), 2.6), ((AU, (AU, G), UG), 2.6),
      ((AU, (CA, G), UG), 3.7), ((AU, (CC, G), UG), 3.7), ((AU, (CG, G), UG), 3.7), ((AU, (CU, G), UG), 3.7),
      ((AU, (GA, G), UG), 2.6), ((AU, (GC, G), UG), 2.6), ((AU, (GG, G), UG), 2.6), ((AU, (GU, G), UG), 2.6),
      ((AU, (UA, G), UG), 3.7), ((AU, (UC, G), UG), 3.7), ((AU, (UG, G), UG), 3.7), ((AU, (UU, G), UG), 3.0),
      // For internal loops between the base pairs "AU" and "AU".
      ((AU, (AA, U), AU), 3.7), ((AU, (AC, U), AU), 3.7), ((AU, (AG, U), AU), 3.7), ((AU, (AU, U), AU), 3.7),
      ((AU, (CA, U), AU), 3.7), ((AU, (CC, U), AU), 3.7), ((AU, (CG, U), AU), 3.7), ((AU, (CU, U), AU), 3.7),
      ((AU, (GA, U), AU), 2.6), ((AU, (GC, U), AU), 3.7), ((AU, (GG, U), AU), 2.6), ((AU, (GU, U), AU), 3.7),
      ((AU, (UA, U), AU), 3.0), ((AU, (UC, U), AU), 3.0), ((AU, (UG, U), AU), 3.0), ((AU, (UU, U), AU), 3.0),
      // For internal loops between the base pairs "AU" and "CG".
      ((AU, (AA, U), CG), 3.0), ((AU, (AC, U), CG), 3.0), ((AU, (AG, U), CG), 3.0), ((AU, (AU, U), CG), 3.0),
      ((AU, (CA, U), CG), 3.0), ((AU, (CC, U), CG), 3.0), ((AU, (CG, A), CG), 3.0), ((AU, (CU, U), CG), 3.0),
      ((AU, (GA, U), CG), 1.9), ((AU, (GC, U), CG), 3.0), ((AU, (GG, U), CG), 1.9), ((AU, (GU, U), CG), 3.0),
      ((AU, (UA, U), CG), 2.2), ((AU, (UC, U), CG), 2.2), ((AU, (UG, U), CG), 2.2), ((AU, (UU, U), CG), 2.2),
      // For internal loops between the base pairs "AU" and "GC".
      ((AU, (AA, U), GC), 3.0), ((AU, (AC, U), GC), 3.0), ((AU, (AG, U), GC), 3.0), ((AU, (AU, U), GC), 3.0),
      ((AU, (CA, U), GC), 3.0), ((AU, (CC, U), GC), 3.0), ((AU, (CG, U), GC), 3.0), ((AU, (CU, U), GC), 3.0),
      ((AU, (GA, U), GC), 1.9), ((AU, (GC, U), GC), 3.0), ((AU, (GG, U), GC), 1.9), ((AU, (GU, U), GC), 3.0),
      ((AU, (UA, U), GC), 2.2), ((AU, (UC, U), GC), 2.2), ((AU, (UG, U), GC), 2.2), ((AU, (UU, U), GC), 2.2),
      // For internal loops between the base pairs "AU" and "UA".
      ((AU, (AA, U), UA), 3.7), ((AU, (AC, U), UA), 3.7), ((AU, (AG, U), UA), 3.7), ((AU, (AU, U), UA), 3.7),
      ((AU, (CA, U), UA), 3.7), ((AU, (CC, U), UA), 3.7), ((AU, (CG, U), UA), 3.7), ((AU, (CU, U), UA), 3.7),
      ((AU, (GA, U), UA), 2.6), ((AU, (GC, U), UA), 3.7), ((AU, (GG, U), UA), 2.6), ((AU, (GU, U), UA), 3.7),
      ((AU, (UA, U), UA), 3.0), ((AU, (UC, U), UA), 3.0), ((AU, (UG, U), UA), 3.0), ((AU, (UU, U), UA), 3.0),
      // For internal loops between the base pairs "AU" and "GU".
      ((AU, (AA, U), GU), 3.7), ((AU, (AC, U), GU), 3.7), ((AU, (AG, U), GU), 3.7), ((AU, (AU, U), GU), 3.7),
      ((AU, (CA, U), GU), 3.7), ((AU, (CC, U), GU), 3.7), ((AU, (CG, U), GU), 3.7), ((AU, (CU, U), GU), 3.7),
      ((AU, (GA, U), GU), 2.6), ((AU, (GC, U), GU), 3.7), ((AU, (GG, U), GU), 2.6), ((AU, (GU, U), GU), 3.7),
      ((AU, (UA, U), GU), 3.0), ((AU, (UC, U), GU), 3.0), ((AU, (UG, U), GU), 3.0), ((AU, (UU, U), GU), 3.0),
      // For internal loops between the base pairs "AU" and "UG".
      ((AU, (AA, U), UG), 3.7), ((AU, (AC, U), UG), 3.7), ((AU, (AG, U), UG), 3.7), ((AU, (AU, U), UG), 3.7),
      ((AU, (CA, U), UG), 3.7), ((AU, (CC, U), UG), 3.7), ((AU, (CG, U), UG), 3.7), ((AU, (CU, U), UG), 3.7),
      ((AU, (GA, U), UG), 2.6), ((AU, (GC, U), UG), 3.7), ((AU, (GG, U), UG), 2.6), ((AU, (GU, U), UG), 3.7),
      ((AU, (UA, U), UG), 3.0), ((AU, (UC, U), UG), 3.0), ((AU, (UG, U), UG), 3.0), ((AU, (UU, U), UG), 3.0),
      // For internal loops behind the base pair "CG".
      // For internal loops between the base pairs "CG" and "AU".
      ((CG, (AA, A), AU), 3.0), ((CG, (AC, A), AU), 3.0), ((CG, (AG, A), AU), 1.9), ((CG, (CG, A), AU), 3.0),
      ((CG, (CA, A), AU), 3.0), ((CG, (CC, A), AU), 3.0), ((CG, (CG, A), AU), 3.0), ((CG, (CU, A), AU), 3.0),
      ((CG, (GA, A), AU), 1.9), ((CG, (GC, A), AU), 1.9), ((CG, (GG, A), AU), 1.9), ((CG, (GU, A), AU), 1.9),
      ((CG, (UA, A), AU), 3.0), ((CG, (UC, A), AU), 3.0), ((CG, (UG, A), AU), 3.0), ((CG, (UU, A), AU), 2.2),
      // For internal loops between the base pairs "CG" and "CG".
      ((CG, (AA, A), CG), 2.5), ((CG, (AC, A), CG), 2.3), ((CG, (AG, A), CG), 1.1), ((CG, (CG, A), CG), 2.3),
      ((CG, (CA, A), CG), 2.3), ((CG, (CC, A), CG), 2.3), ((CG, (CG, A), CG), 2.3), ((CG, (CU, A), CG), 2.3),
      ((CG, (GA, A), CG), 1.7), ((CG, (GC, A), CG), 2.3), ((CG, (GG, A), CG), 0.8), ((CG, (GU, A), CG), 2.3),
      ((CG, (UA, A), CG), 2.3), ((CG, (UC, A), CG), 2.3), ((CG, (UG, A), CG), 2.3), ((CG, (UU, A), CG), 1.5),
      // For internal loops between the base pairs "CG" and "GC".
      ((CG, (AA, A), GC), 2.3), ((CG, (AC, A), GC), 2.3), ((CG, (AG, A), GC), 1.1), ((CG, (CG, A), GC), 2.3),
      ((CG, (CA, A), GC), 2.3), ((CG, (CC, A), GC), 2.3), ((CG, (CG, A), GC), 2.3), ((CG, (CU, A), GC), 2.3),
      ((CG, (GA, A), GC), 1.1), ((CG, (GC, A), GC), 1.1), ((CG, (GG, A), GC), 1.1), ((CG, (GU, A), GC), 1.1),
      ((CG, (UA, A), GC), 2.3), ((CG, (UC, A), GC), 2.3), ((CG, (UG, A), GC), 2.3), ((CG, (UU, A), GC), 1.5),
      // For internal loops between the base pairs "CG" and "UA".
      ((CG, (AA, A), UA), 3.0), ((CG, (AC, A), UA), 3.0), ((CG, (AG, A), UA), 1.9), ((CG, (CG, A), UA), 3.0),
      ((CG, (CA, A), UA), 3.0), ((CG, (CC, A), UA), 3.0), ((CG, (CG, A), UA), 3.0), ((CG, (CU, A), UA), 3.0),
      ((CG, (GA, A), UA), 1.9), ((CG, (GC, A), UA), 1.9), ((CG, (GG, A), UA), 1.9), ((CG, (GU, A), UA), 3.0),
      ((CG, (UA, A), UA), 3.0), ((CG, (UC, A), UA), 3.0), ((CG, (UG, A), UA), 3.0), ((CG, (UU, A), UA), 2.2),
      // For internal loops between the base pairs "CG" and "GU".
      ((CG, (AA, A), GU), 3.0), ((CG, (AC, A), GU), 3.0), ((CG, (AG, A), GU), 1.9), ((CG, (CG, A), GU), 3.0),
      ((CG, (CA, A), GU), 3.0), ((CG, (CC, A), GU), 3.0), ((CG, (CG, A), GU), 3.0), ((CG, (CU, A), GU), 3.0),
      ((CG, (GA, A), GU), 1.9), ((CG, (GC, A), GU), 1.9), ((CG, (GG, A), GU), 1.9), ((CG, (GU, A), GU), 1.9),
      ((CG, (UA, A), GU), 3.0), ((CG, (UC, A), GU), 3.0), ((CG, (UG, A), GU), 3.0), ((CG, (UU, A), GU), 2.2),
      // For internal loops between the base pairs "CG" and "UG".
      ((CG, (AA, A), UG), 3.0), ((CG, (AC, A), UG), 3.0), ((CG, (AG, A), UG), 1.9), ((CG, (CG, A), UG), 3.0),
      ((CG, (CA, A), UG), 3.0), ((CG, (CC, A), UG), 3.0), ((CG, (CG, A), UG), 3.0), ((CG, (CU, A), UG), 3.0),
      ((CG, (GA, A), UG), 1.9), ((CG, (GC, A), UG), 3.0), ((CG, (GG, A), UG), 1.9), ((CG, (GU, A), UG), 3.0),
      ((CG, (UA, A), UG), 3.0), ((CG, (UC, A), UG), 3.0), ((CG, (UG, A), UG), 3.0), ((CG, (UU, A), UG), 2.2),
      // For internal loops between the base pairs "CG" and "AU".
      ((CG, (AA, C), AU), 3.0), ((CG, (AC, C), AU), 3.0), ((CG, (AG, C), AU), 1.9), ((CG, (CG, C), AU), 3.0),
      ((CG, (CA, C), AU), 3.0), ((CG, (CC, C), AU), 3.0), ((CG, (CG, C), AU), 3.0), ((CG, (CU, C), AU), 3.0),
      ((CG, (GA, C), AU), 1.9), ((CG, (GC, C), AU), 3.0), ((CG, (GG, C), AU), 1.9), ((CG, (GU, C), AU), 3.0),
      ((CG, (UA, C), AU), 3.0), ((CG, (UC, C), AU), 3.0), ((CG, (UG, C), AU), 3.0), ((CG, (UU, C), AU), 2.2),
      // For internal loops between the base pairs "CG" and "CG".
      ((CG, (AA, C), CG), 2.3), ((CG, (AC, C), CG), 1.7), ((CG, (AG, C), CG), 1.1), ((CG, (CG, C), CG), 2.3),
      ((CG, (CA, C), CG), 2.3), ((CG, (CC, C), CG), 2.5), ((CG, (CG, A), CG), 2.3), ((CG, (CU, C), CG), 2.3),
      ((CG, (GA, C), CG), 1.1), ((CG, (GC, C), CG), 2.3), ((CG, (GG, C), CG), 1.1), ((CG, (GU, C), CG), 2.3),
      ((CG, (UA, C), CG), 2.3), ((CG, (UC, C), CG), 2.2), ((CG, (UG, C), CG), 2.3), ((CG, (UU, C), CG), 1.5),
      // For internal loops between the base pairs "CG" and "GC".
      ((CG, (AA, C), GC), 2.3), ((CG, (AC, C), GC), 2.3), ((CG, (AG, C), GC), 1.1), ((CG, (CG, C), GC), 2.3),
      ((CG, (CA, C), GC), 2.3), ((CG, (CC, C), GC), 2.3), ((CG, (CG, C), GC), 2.3), ((CG, (CU, C), GC), 2.3),
      ((CG, (GA, C), GC), 1.1), ((CG, (GC, C), GC), 2.3), ((CG, (GG, C), GC), 1.1), ((CG, (GU, C), GC), 2.3),
      ((CG, (UA, C), GC), 2.3), ((CG, (UC, C), GC), 2.3), ((CG, (UG, C), GC), 2.3), ((CG, (UU, C), GC), 1.5),
      // For internal loops between the base pairs "CG" and "UA".
      ((CG, (AA, C), UA), 3.0), ((CG, (AC, C), UA), 3.0), ((CG, (AG, C), UA), 1.9), ((CG, (CG, C), UA), 3.0),
      ((CG, (CA, C), UA), 3.0), ((CG, (CC, C), UA), 3.0), ((CG, (CG, C), UA), 3.0), ((CG, (CU, C), UA), 3.0),
      ((CG, (GA, C), UA), 1.9), ((CG, (GC, C), UA), 3.0), ((CG, (GG, C), UA), 1.9), ((CG, (GU, C), UA), 3.0),
      ((CG, (UA, C), UA), 3.0), ((CG, (UC, C), UA), 3.0), ((CG, (UG, C), UA), 3.0), ((CG, (UU, C), UA), 2.2),
      // For internal loops between the base pairs "CG" and "GU".
      ((CG, (AA, C), GU), 3.0), ((CG, (AC, C), GU), 3.0), ((CG, (AG, C), GU), 1.9), ((CG, (CG, C), GU), 3.0),
      ((CG, (CA, C), GU), 3.0), ((CG, (CC, C), GU), 3.0), ((CG, (CG, C), GU), 3.0), ((CG, (CU, C), GU), 3.0),
      ((CG, (GA, C), GU), 1.9), ((CG, (GC, C), GU), 3.0), ((CG, (GG, C), GU), 1.9), ((CG, (GU, C), GU), 3.0),
      ((CG, (UA, C), GU), 3.0), ((CG, (UC, C), GU), 3.0), ((CG, (UG, C), GU), 3.0), ((CG, (UU, C), GU), 2.2),
      // For internal loops between the base pairs "CG" and "UG".
      ((CG, (AA, C), UG), 3.0), ((CG, (AC, C), UG), 3.0), ((CG, (AG, C), UG), 1.9), ((CG, (CG, C), UG), 3.0),
      ((CG, (CA, C), UG), 3.0), ((CG, (CC, C), UG), 3.0), ((CG, (CG, C), UG), 3.0), ((CG, (CU, C), UG), 3.0),
      ((CG, (GA, C), UG), 1.9), ((CG, (GC, C), UG), 3.0), ((CG, (GG, C), UG), 1.9), ((CG, (GU, C), UG), 3.0),
      ((CG, (UA, C), UG), 3.0), ((CG, (UC, C), UG), 3.0), ((CG, (UG, C), UG), 3.0), ((CG, (UU, C), UG), 2.2),
      // For internal loops between the base pairs "CG" and "AU".
      ((CG, (AA, G), AU), 1.9), ((CG, (AC, G), AU), 1.9), ((CG, (AG, G), AU), 1.9), ((CG, (CG, G), AU), 1.9),
      ((CG, (CA, G), AU), 3.0), ((CG, (CC, G), AU), 3.0), ((CG, (CG, G), AU), 3.0), ((CG, (CU, G), AU), 3.0),
      ((CG, (GA, G), AU), 1.9), ((CG, (GC, G), AU), 1.9), ((CG, (GG, G), AU), 1.9), ((CG, (GU, G), AU), 1.9),
      ((CG, (UA, G), AU), 3.0), ((CG, (UC, G), AU), 3.0), ((CG, (UG, G), AU), 3.0), ((CG, (UU, G), AU), 2.2),
      // For internal loops between the base pairs "CG" and "CG".
      ((CG, (AA, G), CG), 0.8), ((CG, (AC, G), CG), 1.1), ((CG, (AG, G), CG), 1.1), ((CG, (CG, G), CG), 1.1),
      ((CG, (CA, G), CG), 2.3), ((CG, (CC, G), CG), 2.3), ((CG, (CG, A), CG), 2.3), ((CG, (CU, G), CG), 2.3),
      ((CG, (GA, G), CG), 1.2), ((CG, (GC, G), CG), 1.1), ((CG, (GG, G), CG), 1.1), ((CG, (GU, G), CG), 1.1),
      ((CG, (UA, G), CG), 2.3), ((CG, (UC, G), CG), 2.3), ((CG, (UG, G), CG), 2.3), ((CG, (UU, G), CG), 1.5),
      // For internal loops between the base pairs "CG" and "GC".
      ((CG, (AA, G), GC), 1.1), ((CG, (AC, G), GC), 1.1), ((CG, (AG, G), GC), 1.1), ((CG, (CG, G), GC), 1.1),
      ((CG, (CA, G), GC), 2.3), ((CG, (CC, G), GC), 2.3), ((CG, (CG, G), GC), 2.3), ((CG, (CU, G), GC), 2.3),
      ((CG, (GA, G), GC), 1.1), ((CG, (GC, G), GC), 1.1), ((CG, (GG, G), GC), 1.1), ((CG, (GU, G), GC), 1.1),
      ((CG, (UA, G), GC), 2.3), ((CG, (UC, G), GC), 2.3), ((CG, (UG, G), GC), 2.3), ((CG, (UU, G), GC), 1.5),
      // For internal loops between the base pairs "CG" and "UA".
      ((CG, (AA, G), UA), 1.9), ((CG, (AC, G), UA), 1.9), ((CG, (AG, G), UA), 1.9), ((CG, (CG, G), UA), 1.9),
      ((CG, (CA, G), UA), 3.0), ((CG, (CC, G), UA), 3.0), ((CG, (CG, G), UA), 3.0), ((CG, (CU, G), UA), 3.0),
      ((CG, (GA, G), UA), 1.9), ((CG, (GC, G), UA), 1.9), ((CG, (GG, G), UA), 1.9), ((CG, (GU, G), UA), 1.9),
      ((CG, (UA, G), UA), 3.0), ((CG, (UC, G), UA), 3.0), ((CG, (UG, G), UA), 3.0), ((CG, (UU, G), UA), 2.2),
      // For internal loops between the base pairs "CG" and "GU".
      ((CG, (AA, G), GU), 1.9), ((CG, (AC, G), GU), 1.9), ((CG, (AG, G), GU), 1.9), ((CG, (CG, G), GU), 1.9),
      ((CG, (CA, G), GU), 3.0), ((CG, (CC, G), GU), 3.0), ((CG, (CG, G), GU), 3.0), ((CG, (CU, G), GU), 3.0),
      ((CG, (GA, G), GU), 1.9), ((CG, (GC, G), GU), 1.9), ((CG, (GG, G), GU), 1.9), ((CG, (GU, G), GU), 1.9),
      ((CG, (UA, G), GU), 3.0), ((CG, (UC, G), GU), 3.0), ((CG, (UG, G), GU), 3.0), ((CG, (UU, G), GU), 2.2),
      // For internal loops between the base pairs "CG" and "UG".
      ((CG, (AA, G), UG), 1.9), ((CG, (AC, G), UG), 1.9), ((CG, (AG, G), UG), 1.9), ((CG, (CG, G), UG), 1.9),
      ((CG, (CA, G), UG), 3.0), ((CG, (CC, G), UG), 3.0), ((CG, (CG, G), UG), 3.0), ((CG, (CU, G), UG), 3.0),
      ((CG, (GA, G), UG), 1.9), ((CG, (GC, G), UG), 1.9), ((CG, (GG, G), UG), 1.9), ((CG, (GU, G), UG), 1.9),
      ((CG, (UA, G), UG), 3.0), ((CG, (UC, G), UG), 3.0), ((CG, (UG, G), UG), 3.0), ((CG, (UU, G), UG), 2.2),
      // For internal loops between the base pairs "CG" and "AU".
      ((CG, (AA, U), AU), 3.0), ((CG, (AC, U), AU), 3.0), ((CG, (AG, U), AU), 1.9), ((CG, (CG, U), AU), 3.0),
      ((CG, (CA, U), AU), 3.0), ((CG, (CC, U), AU), 3.0), ((CG, (CG, U), AU), 3.0), ((CG, (CU, U), AU), 3.0),
      ((CG, (GA, U), AU), 1.9), ((CG, (GC, U), AU), 3.0), ((CG, (GG, U), AU), 1.9), ((CG, (GU, U), AU), 3.0),
      ((CG, (UA, U), AU), 2.2), ((CG, (UC, U), AU), 2.2), ((CG, (UG, U), AU), 2.2), ((CG, (UU, U), AU), 2.2),
      // For internal loops between the base pairs "CG" and "CG".
      ((CG, (AA, U), CG), 2.3), ((CG, (AC, U), CG), 2.3), ((CG, (AG, U), CG), 1.1), ((CG, (CG, U), CG), 2.3),
      ((CG, (CA, U), CG), 2.5), ((CG, (CC, U), CG), 2.3), ((CG, (CG, A), CG), 2.3), ((CG, (CU, U), CG), 2.3),
      ((CG, (GA, U), CG), 1.1), ((CG, (GC, U), CG), 2.3), ((CG, (GG, U), CG), 1.1), ((CG, (GU, U), CG), 2.3),
      ((CG, (UA, U), CG), 1.5), ((CG, (UC, U), CG), 1.7), ((CG, (UG, U), CG), 1.5), ((CG, (UU, U), CG), 1.4),
      // For internal loops between the base pairs "CG" and "GC".
      ((CG, (AA, U), GC), 2.3), ((CG, (AC, U), GC), 2.3), ((CG, (AG, U), GC), 1.1), ((CG, (CG, U), GC), 2.3),
      ((CG, (CA, U), GC), 2.3), ((CG, (CC, U), GC), 2.3), ((CG, (CG, U), GC), 2.3), ((CG, (CU, U), GC), 2.3),
      ((CG, (GA, U), GC), 1.1), ((CG, (GC, U), GC), 2.3), ((CG, (GG, U), GC), 1.1), ((CG, (GU, U), GC), 2.3),
      ((CG, (UA, U), GC), 1.5), ((CG, (UC, U), GC), 1.5), ((CG, (UG, U), GC), 1.5), ((CG, (UU, U), GC), 1.5),
      // For internal loops between the base pairs "CG" and "UA".
      ((CG, (AA, U), UA), 3.0), ((CG, (AC, U), UA), 3.0), ((CG, (AG, U), UA), 1.9), ((CG, (CG, U), UA), 3.0),
      ((CG, (CA, U), UA), 3.0), ((CG, (CC, U), UA), 3.0), ((CG, (CG, U), UA), 3.0), ((CG, (CU, U), UA), 3.0),
      ((CG, (GA, U), UA), 1.9), ((CG, (GC, U), UA), 3.0), ((CG, (GG, U), UA), 1.9), ((CG, (GU, U), UA), 3.0),
      ((CG, (UA, U), UA), 2.2), ((CG, (UC, U), UA), 2.2), ((CG, (UG, U), UA), 2.2), ((CG, (UU, U), UA), 2.2),
      // For internal loops between the base pairs "CG" and "GU".
      ((CG, (AA, U), GU), 3.0), ((CG, (AC, U), GU), 3.0), ((CG, (AG, U), GU), 1.9), ((CG, (CG, U), GU), 3.0),
      ((CG, (CA, U), GU), 3.0), ((CG, (CC, U), GU), 3.0), ((CG, (CG, U), GU), 3.0), ((CG, (CU, U), GU), 3.0),
      ((CG, (GA, U), GU), 1.9), ((CG, (GC, U), GU), 3.0), ((CG, (GG, U), GU), 1.9), ((CG, (GU, U), GU), 3.0),
      ((CG, (UA, U), GU), 2.2), ((CG, (UC, U), GU), 2.2), ((CG, (UG, U), GU), 2.2), ((CG, (UU, U), GU), 2.2),
      // For internal loops between the base pairs "CG" and "UG".
      ((CG, (AA, U), UG), 3.0), ((CG, (AC, U), UG), 3.0), ((CG, (AG, U), UG), 1.9), ((CG, (CG, U), UG), 3.0),
      ((CG, (CA, U), UG), 3.0), ((CG, (CC, U), UG), 3.0), ((CG, (CG, U), UG), 3.0), ((CG, (CU, U), UG), 3.0),
      ((CG, (GA, U), UG), 1.9), ((CG, (GC, U), UG), 3.0), ((CG, (GG, U), UG), 1.9), ((CG, (GU, U), UG), 3.0),
      ((CG, (UA, U), UG), 2.2), ((CG, (UC, U), UG), 2.2), ((CG, (UG, U), UG), 2.2), ((CG, (UU, U), UG), 2.2),
      // For internal loops behind the base pair "GC".
      // For internal loops between the base pairs "GC" and "AU".
      ((GC, (AA, A), AU), 3.0), ((GC, (AC, A), AU), 3.0), ((GC, (AG, A), AU), 3.0), ((GC, (GC, A), AU), 3.0),
      ((GC, (CA, A), AU), 3.0), ((GC, (CC, A), AU), 3.0), ((GC, (GC, A), AU), 3.0), ((GC, (CU, A), AU), 3.0),
      ((GC, (GA, A), AU), 1.9), ((GC, (GC, A), AU), 1.9), ((GC, (GG, A), AU), 1.9), ((GC, (GU, A), AU), 1.9),
      ((GC, (UA, A), AU), 3.0), ((GC, (UC, A), AU), 3.0), ((GC, (UG, A), AU), 3.0), ((GC, (UU, A), AU), 2.2),
      // For internal loops between the base pairs "GC" and "CG".
      ((GC, (AA, A), CG), 2.3), ((GC, (AC, A), CG), 2.3), ((GC, (AG, A), CG), 2.3), ((GC, (GC, A), CG), 2.3),
      ((GC, (CA, A), CG), 2.3), ((GC, (CC, A), CG), 2.3), ((GC, (GC, A), CG), 2.3), ((GC, (CU, A), CG), 2.3),
      ((GC, (GA, A), CG), 1.1), ((GC, (GC, A), CG), 2.3), ((GC, (GG, A), CG), 1.1), ((GC, (GU, A), CG), 2.3),
      ((GC, (UA, A), CG), 2.3), ((GC, (UC, A), CG), 2.3), ((GC, (UG, A), CG), 2.3), ((GC, (UU, A), CG), 1.5),
      // For internal loops between the base pairs "GC" and "GC".
      ((GC, (AA, A), GC), 2.5), ((GC, (AC, A), GC), 2.3), ((GC, (AG, A), GC), 2.1), ((GC, (GC, A), GC), 2.3),
      ((GC, (CA, A), GC), 2.3), ((GC, (CC, A), GC), 2.3), ((GC, (GC, A), GC), 2.3), ((GC, (CU, A), GC), 2.3),
      ((GC, (GA, A), GC), 1.1), ((GC, (GC, A), GC), 1.1), ((GC, (GG, A), GC), 1.1), ((GC, (GU, A), GC), 1.1),
      ((GC, (UA, A), GC), 2.3), ((GC, (UC, A), GC), 2.3), ((GC, (UG, A), GC), 2.3), ((GC, (UU, A), GC), 1.5),
      // For internal loops between the base pairs "GC" and "UA".
      ((GC, (AA, A), UA), 3.0), ((GC, (AC, A), UA), 3.0), ((GC, (AG, A), UA), 3.0), ((GC, (GC, A), UA), 3.0),
      ((GC, (CA, A), UA), 3.0), ((GC, (CC, A), UA), 3.0), ((GC, (GC, A), UA), 3.0), ((GC, (CU, A), UA), 3.0),
      ((GC, (GA, A), UA), 1.9), ((GC, (GC, A), UA), 3.0), ((GC, (GG, A), UA), 1.9), ((GC, (GU, A), UA), 3.0),
      ((GC, (UA, A), UA), 3.0), ((GC, (UC, A), UA), 3.0), ((GC, (UG, A), UA), 3.0), ((GC, (UU, A), UA), 2.2),
      // For internal loops between the base pairs "GC" and "GU".
      ((GC, (AA, A), GU), 2.5), ((GC, (AC, A), GU), 3.0), ((GC, (AG, A), GU), 2.1), ((GC, (GC, A), GU), 3.0),
      ((GC, (CA, A), GU), 3.0), ((GC, (CC, A), GU), 3.0), ((GC, (GC, A), GU), 3.0), ((GC, (CU, A), GU), 3.0),
      ((GC, (GA, A), GU), 1.9), ((GC, (GC, A), GU), 1.9), ((GC, (GG, A), GU), 1.9), ((GC, (GU, A), GU), 1.9),
      ((GC, (UA, A), GU), 3.0), ((GC, (UC, A), GU), 3.0), ((GC, (UG, A), GU), 3.0), ((GC, (UU, A), GU), 2.2),
      // For internal loops between the base pairs "GC" and "UG".
      ((GC, (AA, A), UG), 3.0), ((GC, (AC, A), UG), 3.0), ((GC, (AG, A), UG), 3.0), ((GC, (GC, A), UG), 3.0),
      ((GC, (CA, A), UG), 3.0), ((GC, (CC, A), UG), 3.0), ((GC, (GC, A), UG), 3.0), ((GC, (CU, A), UG), 3.0),
      ((GC, (GA, A), UG), 1.9), ((GC, (GC, A), UG), 3.0), ((GC, (GG, A), UG), 1.9), ((GC, (GU, A), UG), 3.0),
      ((GC, (UA, A), UG), 3.0), ((GC, (UC, A), UG), 3.0), ((GC, (UG, A), UG), 3.0), ((GC, (UU, A), UG), 2.2),
      // For internal loops between the base pairs "GC" and "AU".
      ((GC, (AA, C), AU), 3.0), ((GC, (AC, C), AU), 3.0), ((GC, (AG, C), AU), 3.0), ((GC, (GC, C), AU), 3.0),
      ((GC, (CA, C), AU), 3.0), ((GC, (CC, C), AU), 3.0), ((GC, (GC, C), AU), 3.0), ((GC, (CU, C), AU), 3.0),
      ((GC, (GA, C), AU), 1.9), ((GC, (GC, C), AU), 3.0), ((GC, (GG, C), AU), 1.9), ((GC, (GU, C), AU), 3.0),
      ((GC, (UA, C), AU), 3.0), ((GC, (UC, C), AU), 3.0), ((GC, (UG, C), AU), 3.0), ((GC, (UU, C), AU), 2.2),
      // For internal loops between the base pairs "GC" and "CG".
      ((GC, (AA, C), CG), 2.3), ((GC, (AC, C), CG), 2.3), ((GC, (AG, C), CG), 2.3), ((GC, (GC, C), CG), 2.3),
      ((GC, (CA, C), CG), 2.3), ((GC, (CC, C), CG), 2.3), ((GC, (GC, A), CG), 2.3), ((GC, (CU, C), CG), 2.3),
      ((GC, (GA, C), CG), 1.1), ((GC, (GC, C), CG), 2.3), ((GC, (GG, C), CG), 1.1), ((GC, (GU, C), CG), 2.3),
      ((GC, (UA, C), CG), 2.3), ((GC, (UC, C), CG), 2.3), ((GC, (UG, C), CG), 2.3), ((GC, (UU, C), CG), 1.5),
      // For internal loops between the base pairs "GC" and "GC".
      ((GC, (AA, C), GC), 2.3), ((GC, (AC, C), GC), 2.3), ((GC, (AG, C), GC), 2.3), ((GC, (GC, C), GC), 2.3),
      ((GC, (CA, C), GC), 2.3), ((GC, (CC, C), GC), 2.3), ((GC, (GC, C), GC), 2.3), ((GC, (CU, C), GC), 2.3),
      ((GC, (GA, C), GC), 1.1), ((GC, (GC, C), GC), 2.3), ((GC, (GG, C), GC), 1.1), ((GC, (GU, C), GC), 2.3),
      ((GC, (UA, C), GC), 2.3), ((GC, (UC, C), GC), 2.3), ((GC, (UG, C), GC), 2.3), ((GC, (UU, C), GC), 1.5),
      // For internal loops between the base pairs "GC" and "UA".
      ((GC, (AA, C), UA), 3.0), ((GC, (AC, C), UA), 3.0), ((GC, (AG, C), UA), 3.0), ((GC, (GC, C), UA), 3.0),
      ((GC, (CA, C), UA), 3.0), ((GC, (CC, C), UA), 3.0), ((GC, (GC, C), UA), 3.0), ((GC, (CU, C), UA), 3.0),
      ((GC, (GA, C), UA), 1.9), ((GC, (GC, C), UA), 3.0), ((GC, (GG, C), UA), 1.9), ((GC, (GU, C), UA), 3.0),
      ((GC, (UA, C), UA), 3.0), ((GC, (UC, C), UA), 3.0), ((GC, (UG, C), UA), 3.0), ((GC, (UU, C), UA), 2.2),
      // For internal loops between the base pairs "GC" and "GU".
      ((GC, (AA, C), GU), 3.0), ((GC, (AC, C), GU), 3.0), ((GC, (AG, C), GU), 3.0), ((GC, (GC, C), GU), 3.0),
      ((GC, (CA, C), GU), 3.0), ((GC, (CC, C), GU), 3.0), ((GC, (GC, C), GU), 3.0), ((GC, (CU, C), GU), 3.0),
      ((GC, (GA, C), GU), 1.9), ((GC, (GC, C), GU), 3.0), ((GC, (GG, C), GU), 1.9), ((GC, (GU, C), GU), 3.0),
      ((GC, (UA, C), GU), 3.0), ((GC, (UC, C), GU), 3.0), ((GC, (UG, C), GU), 3.0), ((GC, (UU, C), GU), 2.2),
      // For internal loops between the base pairs "GC" and "UG".
      ((GC, (AA, C), UG), 3.0), ((GC, (AC, C), UG), 3.0), ((GC, (AG, C), UG), 3.0), ((GC, (GC, C), UG), 3.0),
      ((GC, (CA, C), UG), 3.0), ((GC, (CC, C), UG), 3.0), ((GC, (GC, C), UG), 3.0), ((GC, (CU, C), UG), 3.0),
      ((GC, (GA, C), UG), 1.9), ((GC, (GC, C), UG), 3.0), ((GC, (GG, C), UG), 1.9), ((GC, (GU, C), UG), 3.0),
      ((GC, (UA, C), UG), 3.0), ((GC, (UC, C), UG), 3.0), ((GC, (UG, C), UG), 3.0), ((GC, (UU, C), UG), 2.2),
      // For internal loops between the base pairs "GC" and "AU".
      ((GC, (AA, G), AU), 1.9), ((GC, (AC, G), AU), 1.9), ((GC, (AG, G), AU), 1.9), ((GC, (GC, G), AU), 1.9),
      ((GC, (CA, G), AU), 3.0), ((GC, (CC, G), AU), 3.0), ((GC, (GC, G), AU), 3.0), ((GC, (CU, G), AU), 3.0),
      ((GC, (GA, G), AU), 1.9), ((GC, (GC, G), AU), 1.9), ((GC, (GG, G), AU), 1.9), ((GC, (GU, G), AU), 1.9),
      ((GC, (UA, G), AU), 3.0), ((GC, (UC, G), AU), 3.0), ((GC, (UG, G), AU), 3.0), ((GC, (UU, G), AU), 2.2),
      // For internal loops between the base pairs "GC" and "CG".
      ((GC, (AA, G), CG), 1.1), ((GC, (AC, G), CG), 1.1), ((GC, (AG, G), CG), 1.1), ((GC, (GC, G), CG), 1.1),
      ((GC, (CA, G), CG), 2.3), ((GC, (CC, G), CG), 2.3), ((GC, (GC, A), CG), 2.3), ((GC, (CU, G), CG), 2.3),
      ((GC, (GA, G), CG), 1.1), ((GC, (GC, G), CG), 1.1), ((GC, (GG, G), CG), 1.1), ((GC, (GU, G), CG), 1.1),
      ((GC, (UA, G), CG), 2.3), ((GC, (UC, G), CG), 2.3), ((GC, (UG, G), CG), 2.3), ((GC, (UU, G), CG), 1.5),
      // For internal loops between the base pairs "GC" and "GC".
      ((GC, (AA, G), GC), 1.2), ((GC, (AC, G), GC), 1.1), ((GC, (AG, G), GC), 1.1), ((GC, (GC, G), GC), 1.1),
      ((GC, (CA, G), GC), 2.3), ((GC, (CC, G), GC), 2.3), ((GC, (GC, G), GC), 2.3), ((GC, (CU, G), GC), 2.3),
      ((GC, (GA, G), GC), 1.1), ((GC, (GC, G), GC), 1.1), ((GC, (GG, G), GC), 1.1), ((GC, (GU, G), GC), 1.1),
      ((GC, (UA, G), GC), 2.3), ((GC, (UC, G), GC), 2.3), ((GC, (UG, G), GC), 2.3), ((GC, (UU, G), GC), 1.5),
      // For internal loops between the base pairs "GC" and "UA".
      ((GC, (AA, G), UA), 1.9), ((GC, (AC, G), UA), 1.9), ((GC, (AG, G), UA), 1.9), ((GC, (GC, G), UA), 1.9),
      ((GC, (CA, G), UA), 3.0), ((GC, (CC, G), UA), 3.0), ((GC, (GC, G), UA), 3.0), ((GC, (CU, G), UA), 3.0),
      ((GC, (GA, G), UA), 1.9), ((GC, (GC, G), UA), 1.9), ((GC, (GG, G), UA), 1.9), ((GC, (GU, G), UA), 1.9),
      ((GC, (UA, G), UA), 3.0), ((GC, (UC, G), UA), 3.0), ((GC, (UG, G), UA), 3.0), ((GC, (UU, G), UA), 2.2),
      // For internal loops between the base pairs "GC" and "GU".
      ((GC, (AA, G), GU), 1.2), ((GC, (AC, G), GU), 1.9), ((GC, (AG, G), GU), 1.9), ((GC, (GC, G), GU), 1.9),
      ((GC, (CA, G), GU), 3.0), ((GC, (CC, G), GU), 3.0), ((GC, (GC, G), GU), 3.0), ((GC, (CU, G), GU), 3.0),
      ((GC, (GA, G), GU), 1.9), ((GC, (GC, G), GU), 1.9), ((GC, (GG, G), GU), 1.9), ((GC, (GU, G), GU), 1.9),
      ((GC, (UA, G), GU), 3.0), ((GC, (UC, G), GU), 3.0), ((GC, (UG, G), GU), 3.0), ((GC, (UU, G), GU), 2.2),
      // For internal loops between the base pairs "GC" and "UG".
      ((GC, (AA, G), UG), 1.9), ((GC, (AC, G), UG), 1.9), ((GC, (AG, G), UG), 1.9), ((GC, (GC, G), UG), 1.9),
      ((GC, (CA, G), UG), 3.0), ((GC, (CC, G), UG), 3.0), ((GC, (GC, G), UG), 3.0), ((GC, (CU, G), UG), 3.0),
      ((GC, (GA, G), UG), 1.9), ((GC, (GC, G), UG), 1.9), ((GC, (GG, G), UG), 1.9), ((GC, (GU, G), UG), 1.9),
      ((GC, (UA, G), UG), 3.0), ((GC, (UC, G), UG), 3.0), ((GC, (UG, G), UG), 3.0), ((GC, (UU, G), UG), 2.2),
      // For internal loops between the base pairs "GC" and "AU".
      ((GC, (AA, U), AU), 3.0), ((GC, (AC, U), AU), 3.0), ((GC, (AG, U), AU), 1.9), ((GC, (GC, U), AU), 3.0),
      ((GC, (CA, U), AU), 3.0), ((GC, (CC, U), AU), 3.0), ((GC, (GC, U), AU), 3.0), ((GC, (CU, U), AU), 3.0),
      ((GC, (GA, U), AU), 1.9), ((GC, (GC, U), AU), 3.0), ((GC, (GG, U), AU), 1.9), ((GC, (GU, U), AU), 3.0),
      ((GC, (UA, U), AU), 2.2), ((GC, (UC, U), AU), 2.2), ((GC, (UG, U), AU), 2.2), ((GC, (UU, U), AU), 2.2),
      // For internal loops between the base pairs "GC" and "CG".
      ((GC, (AA, U), CG), 2.3), ((GC, (AC, U), CG), 2.3), ((GC, (AG, U), CG), 2.3), ((GC, (GC, U), CG), 2.3),
      ((GC, (CA, U), CG), 2.3), ((GC, (CC, U), CG), 2.3), ((GC, (GC, A), CG), 2.3), ((GC, (CU, U), CG), 2.3),
      ((GC, (GA, U), CG), 1.1), ((GC, (GC, U), CG), 2.3), ((GC, (GG, U), CG), 1.1), ((GC, (GU, U), CG), 2.3),
      ((GC, (UA, U), CG), 1.5), ((GC, (UC, U), CG), 1.7), ((GC, (UG, U), CG), 1.5), ((GC, (UU, U), CG), 1.5),
      // For internal loops between the base pairs "GC" and "GC".
      ((GC, (AA, U), GC), 2.3), ((GC, (AC, U), GC), 2.3), ((GC, (AG, U), GC), 2.3), ((GC, (GC, U), GC), 2.3),
      ((GC, (CA, U), GC), 2.3), ((GC, (CC, U), GC), 1.9), ((GC, (GC, U), GC), 2.3), ((GC, (CU, U), GC), 2.3),
      ((GC, (GA, U), GC), 1.1), ((GC, (GC, U), GC), 2.3), ((GC, (GG, U), GC), 1.1), ((GC, (GU, U), GC), 2.3),
      ((GC, (UA, U), GC), 1.5), ((GC, (UC, U), GC), 1.5), ((GC, (UG, U), GC), 1.5), ((GC, (UU, U), GC), 1.5),
      // For internal loops between the base pairs "GC" and "UA".
      ((GC, (AA, U), UA), 3.0), ((GC, (AC, U), UA), 3.0), ((GC, (AG, U), UA), 3.0), ((GC, (GC, U), UA), 3.0),
      ((GC, (CA, U), UA), 3.0), ((GC, (CC, U), UA), 3.0), ((GC, (GC, U), UA), 3.0), ((GC, (CU, U), UA), 3.0),
      ((GC, (GA, U), UA), 1.9), ((GC, (GC, U), UA), 3.0), ((GC, (GG, U), UA), 1.9), ((GC, (GU, U), UA), 3.0),
      ((GC, (UA, U), UA), 2.2), ((GC, (UC, U), UA), 2.2), ((GC, (UG, U), UA), 2.2), ((GC, (UU, U), UA), 2.2),
      // For internal loops between the base pairs "GC" and "GU".
      ((GC, (AA, U), GU), 3.0), ((GC, (AC, U), GU), 3.0), ((GC, (AG, U), GU), 3.0), ((GC, (GC, U), GU), 3.0),
      ((GC, (CA, U), GU), 3.0), ((GC, (CC, U), GU), 1.9), ((GC, (GC, U), GU), 3.0), ((GC, (CU, U), GU), 3.0),
      ((GC, (GA, U), GU), 1.9), ((GC, (GC, U), GU), 3.0), ((GC, (GG, U), GU), 1.9), ((GC, (GU, U), GU), 3.0),
      ((GC, (UA, U), GU), 2.2), ((GC, (UC, U), GU), 2.2), ((GC, (UG, U), GU), 2.2), ((GC, (UU, U), GU), 2.2),
      // For internal loops between the base pairs "GC" and "UG".
      ((GC, (AA, U), UG), 3.0), ((GC, (AC, U), UG), 3.0), ((GC, (AG, U), UG), 3.0), ((GC, (GC, U), UG), 3.0),
      ((GC, (CA, U), UG), 3.0), ((GC, (CC, U), UG), 3.0), ((GC, (GC, U), UG), 3.0), ((GC, (CU, U), UG), 3.0),
      ((GC, (GA, U), UG), 1.9), ((GC, (GC, U), UG), 3.0), ((GC, (GG, U), UG), 1.9), ((GC, (GU, U), UG), 3.0),
      ((GC, (UA, U), UG), 2.2), ((GC, (UC, U), UG), 2.2), ((GC, (UG, U), UG), 2.2), ((GC, (UU, U), UG), 2.2),
      // For internal loops behind the base pair "UA".
      // For internal loops between the base pairs "UA" and "AU".
      ((UA, (AA, A), AU), 3.7), ((UA, (AC, A), AU), 3.7), ((UA, (AG, A), AU), 2.6), ((UA, (UA, A), AU), 3.7),
      ((UA, (CA, A), AU), 3.7), ((UA, (CC, A), AU), 3.7), ((UA, (UA, A), AU), 3.7), ((UA, (CU, A), AU), 3.7),
      ((UA, (GA, A), AU), 2.6), ((UA, (UA, A), AU), 2.6), ((UA, (GG, A), AU), 2.6), ((UA, (GU, A), AU), 2.6),
      ((UA, (UA, A), AU), 3.7), ((UA, (UC, A), AU), 3.7), ((UA, (UG, A), AU), 3.7), ((UA, (UU, A), AU), 3.0),
      // For internal loops between the base pairs "UA" and "CG".
      ((UA, (AA, A), CG), 3.0), ((UA, (AC, A), CG), 3.0), ((UA, (AG, A), CG), 1.9), ((UA, (UA, A), CG), 3.0),
      ((UA, (CA, A), CG), 3.0), ((UA, (CC, A), CG), 3.0), ((UA, (UA, A), CG), 3.0), ((UA, (CU, A), CG), 3.0),
      ((UA, (GA, A), CG), 1.9), ((UA, (UA, A), CG), 3.0), ((UA, (GG, A), CG), 1.9), ((UA, (GU, A), CG), 3.0),
      ((UA, (UA, A), CG), 3.0), ((UA, (UC, A), CG), 3.0), ((UA, (UG, A), CG), 3.0), ((UA, (UU, A), CG), 2.2),
      // For internal loops between the base pairs "UA" and "GC".
      ((UA, (AA, A), GC), 3.0), ((UA, (AC, A), GC), 3.0), ((UA, (AG, A), GC), 1.9), ((UA, (UA, A), GC), 3.0),
      ((UA, (CA, A), GC), 3.0), ((UA, (CC, A), GC), 3.0), ((UA, (UA, A), GC), 3.0), ((UA, (CU, A), GC), 3.0),
      ((UA, (GA, A), GC), 1.9), ((UA, (UA, A), GC), 1.9), ((UA, (GG, A), GC), 1.9), ((UA, (GU, A), GC), 1.9),
      ((UA, (UA, A), GC), 3.0), ((UA, (UC, A), GC), 3.0), ((UA, (UG, A), GC), 3.0), ((UA, (UU, A), GC), 2.2),
      // For internal loops between the base pairs "UA" and "UA".
      ((UA, (AA, A), UA), 3.7), ((UA, (AC, A), UA), 3.7), ((UA, (AG, A), UA), 2.6), ((UA, (UA, A), UA), 3.7),
      ((UA, (CA, A), UA), 3.7), ((UA, (CC, A), UA), 3.7), ((UA, (UA, A), UA), 3.7), ((UA, (CU, A), UA), 3.7),
      ((UA, (GA, A), UA), 2.6), ((UA, (UA, A), UA), 3.7), ((UA, (GG, A), UA), 2.6), ((UA, (GU, A), UA), 3.7),
      ((UA, (UA, A), UA), 3.7), ((UA, (UC, A), UA), 3.7), ((UA, (UG, A), UA), 3.7), ((UA, (UU, A), UA), 3.0),
      // For internal loops between the base pairs "UA" and "GU".
      ((UA, (AA, A), GU), 3.7), ((UA, (AC, A), GU), 3.7), ((UA, (AG, A), GU), 2.6), ((UA, (UA, A), GU), 3.7),
      ((UA, (CA, A), GU), 3.7), ((UA, (CC, A), GU), 3.7), ((UA, (UA, A), GU), 3.7), ((UA, (CU, A), GU), 3.7),
      ((UA, (GA, A), GU), 2.6), ((UA, (UA, A), GU), 2.6), ((UA, (GG, A), GU), 2.6), ((UA, (GU, A), GU), 2.6),
      ((UA, (UA, A), GU), 3.7), ((UA, (UC, A), GU), 3.7), ((UA, (UG, A), GU), 3.7), ((UA, (UU, A), GU), 3.0),
      // For internal loops between the base pairs "UA" and "UG".
      ((UA, (AA, A), UG), 3.7), ((UA, (AC, A), UG), 3.7), ((UA, (AG, A), UG), 2.6), ((UA, (UA, A), UG), 3.7),
      ((UA, (CA, A), UG), 3.7), ((UA, (CC, A), UG), 3.7), ((UA, (UA, A), UG), 3.7), ((UA, (CU, A), UG), 3.7),
      ((UA, (GA, A), UG), 2.6), ((UA, (UA, A), UG), 3.7), ((UA, (GG, A), UG), 2.6), ((UA, (GU, A), UG), 3.7),
      ((UA, (UA, A), UG), 3.7), ((UA, (UC, A), UG), 3.7), ((UA, (UG, A), UG), 3.7), ((UA, (UU, A), UG), 3.0),
      // For internal loops between the base pairs "UA" and "AU".
      ((UA, (AA, C), AU), 3.7), ((UA, (AC, C), AU), 3.7), ((UA, (AG, C), AU), 2.6), ((UA, (UA, C), AU), 3.7),
      ((UA, (CA, C), AU), 3.7), ((UA, (CC, C), AU), 3.7), ((UA, (UA, C), AU), 3.7), ((UA, (CU, C), AU), 3.7),
      ((UA, (GA, C), AU), 2.6), ((UA, (UA, C), AU), 3.7), ((UA, (GG, C), AU), 2.6), ((UA, (GU, C), AU), 3.7),
      ((UA, (UA, C), AU), 3.7), ((UA, (UC, C), AU), 3.7), ((UA, (UG, C), AU), 3.7), ((UA, (UU, C), AU), 3.0),
      // For internal loops between the base pairs "UA" and "CG".
      ((UA, (AA, C), CG), 3.0), ((UA, (AC, C), CG), 3.0), ((UA, (AG, C), CG), 1.9), ((UA, (UA, C), CG), 3.0),
      ((UA, (CA, C), CG), 3.0), ((UA, (CC, C), CG), 3.0), ((UA, (UA, A), CG), 3.0), ((UA, (CU, C), CG), 3.0),
      ((UA, (GA, C), CG), 1.9), ((UA, (UA, C), CG), 3.0), ((UA, (GG, C), CG), 1.9), ((UA, (GU, C), CG), 3.0),
      ((UA, (UA, C), CG), 3.0), ((UA, (UC, C), CG), 3.0), ((UA, (UG, C), CG), 3.0), ((UA, (UU, C), CG), 2.2),
      // For internal loops between the base pairs "UA" and "GC".
      ((UA, (AA, C), GC), 3.0), ((UA, (AC, C), GC), 3.0), ((UA, (AG, C), GC), 1.9), ((UA, (UA, C), GC), 3.0),
      ((UA, (CA, C), GC), 3.0), ((UA, (CC, C), GC), 3.0), ((UA, (UA, C), GC), 3.0), ((UA, (CU, C), GC), 3.0),
      ((UA, (GA, C), GC), 1.9), ((UA, (UA, C), GC), 3.0), ((UA, (GG, C), GC), 1.9), ((UA, (GU, C), GC), 3.0),
      ((UA, (UA, C), GC), 3.0), ((UA, (UC, C), GC), 3.0), ((UA, (UG, C), GC), 3.0), ((UA, (UU, C), GC), 2.2),
      // For internal loops between the base pairs "UA" and "UA".
      ((UA, (AA, C), UA), 3.7), ((UA, (AC, C), UA), 3.7), ((UA, (AG, C), UA), 2.6), ((UA, (UA, C), UA), 3.7),
      ((UA, (CA, C), UA), 3.7), ((UA, (CC, C), UA), 3.7), ((UA, (UA, C), UA), 3.7), ((UA, (CU, C), UA), 3.7),
      ((UA, (GA, C), UA), 2.6), ((UA, (UA, C), UA), 3.7), ((UA, (GG, C), UA), 2.6), ((UA, (GU, C), UA), 3.7),
      ((UA, (UA, C), UA), 3.7), ((UA, (UC, C), UA), 3.7), ((UA, (UG, C), UA), 3.7), ((UA, (UU, C), UA), 3.0),
      // For internal loops between the base pairs "UA" and "GU".
      ((UA, (AA, C), GU), 3.7), ((UA, (AC, C), GU), 3.7), ((UA, (AG, C), GU), 2.6), ((UA, (UA, C), GU), 3.7),
      ((UA, (CA, C), GU), 3.7), ((UA, (CC, C), GU), 3.7), ((UA, (UA, C), GU), 3.7), ((UA, (CU, C), GU), 3.7),
      ((UA, (GA, C), GU), 2.6), ((UA, (UA, C), GU), 3.7), ((UA, (GG, C), GU), 2.6), ((UA, (GU, C), GU), 3.7),
      ((UA, (UA, C), GU), 3.7), ((UA, (UC, C), GU), 3.7), ((UA, (UG, C), GU), 3.7), ((UA, (UU, C), GU), 3.0),
      // For internal loops between the base pairs "UA" and "UG".
      ((UA, (AA, C), UG), 3.7), ((UA, (AC, C), UG), 3.7), ((UA, (AG, C), UG), 2.6), ((UA, (UA, C), UG), 3.7),
      ((UA, (CA, C), UG), 3.7), ((UA, (CC, C), UG), 3.7), ((UA, (UA, C), UG), 3.7), ((UA, (CU, C), UG), 3.7),
      ((UA, (GA, C), UG), 2.6), ((UA, (UA, C), UG), 3.7), ((UA, (GG, C), UG), 2.6), ((UA, (GU, C), UG), 3.7),
      ((UA, (UA, C), UG), 3.7), ((UA, (UC, C), UG), 3.7), ((UA, (UG, C), UG), 3.7), ((UA, (UU, C), UG), 3.0),
      // For internal loops between the base pairs "UA" and "AU".
      ((UA, (AA, G), AU), 2.6), ((UA, (AC, G), AU), 2.6), ((UA, (AG, G), AU), 2.6), ((UA, (UA, G), AU), 2.6),
      ((UA, (CA, G), AU), 3.7), ((UA, (CC, G), AU), 3.7), ((UA, (UA, G), AU), 3.7), ((UA, (CU, G), AU), 3.7),
      ((UA, (GA, G), AU), 2.6), ((UA, (UA, G), AU), 2.6), ((UA, (GG, G), AU), 2.6), ((UA, (GU, G), AU), 2.6),
      ((UA, (UA, G), AU), 3.7), ((UA, (UC, G), AU), 3.7), ((UA, (UG, G), AU), 3.7), ((UA, (UU, G), AU), 3.0),
      // For internal loops between the base pairs "UA" and "CG".
      ((UA, (AA, G), CG), 1.9), ((UA, (AC, G), CG), 1.9), ((UA, (AG, G), CG), 1.9), ((UA, (UA, G), CG), 1.9),
      ((UA, (CA, G), CG), 3.0), ((UA, (CC, G), CG), 3.0), ((UA, (UA, A), CG), 3.0), ((UA, (CU, G), CG), 3.0),
      ((UA, (GA, G), CG), 1.9), ((UA, (UA, G), CG), 1.9), ((UA, (GG, G), CG), 1.9), ((UA, (GU, G), CG), 1.9),
      ((UA, (UA, G), CG), 3.0), ((UA, (UC, G), CG), 3.0), ((UA, (UG, G), CG), 3.0), ((UA, (UU, G), CG), 2.2),
      // For internal loops between the base pairs "UA" and "GC".
      ((UA, (AA, G), GC), 1.9), ((UA, (AC, G), GC), 1.9), ((UA, (AG, G), GC), 1.9), ((UA, (UA, G), GC), 1.9),
      ((UA, (CA, G), GC), 3.0), ((UA, (CC, G), GC), 3.0), ((UA, (UA, G), GC), 3.0), ((UA, (CU, G), GC), 3.0),
      ((UA, (GA, G), GC), 1.9), ((UA, (UA, G), GC), 1.9), ((UA, (GG, G), GC), 1.9), ((UA, (GU, G), GC), 1.9),
      ((UA, (UA, G), GC), 3.0), ((UA, (UC, G), GC), 3.0), ((UA, (UG, G), GC), 3.0), ((UA, (UU, G), GC), 2.2),
      // For internal loops between the base pairs "UA" and "UA".
      ((UA, (AA, G), UA), 2.6), ((UA, (AC, G), UA), 2.6), ((UA, (AG, G), UA), 2.6), ((UA, (UA, G), UA), 2.6),
      ((UA, (CA, G), UA), 3.7), ((UA, (CC, G), UA), 3.7), ((UA, (UA, G), UA), 3.7), ((UA, (CU, G), UA), 3.7),
      ((UA, (GA, G), UA), 2.6), ((UA, (UA, G), UA), 2.6), ((UA, (GG, G), UA), 2.6), ((UA, (GU, G), UA), 2.6),
      ((UA, (UA, G), UA), 3.7), ((UA, (UC, G), UA), 3.7), ((UA, (UG, G), UA), 3.7), ((UA, (UU, G), UA), 3.0),
      // For internal loops between the base pairs "UA" and "GU".
      ((UA, (AA, G), GU), 2.6), ((UA, (AC, G), GU), 2.6), ((UA, (AG, G), GU), 2.6), ((UA, (UA, G), GU), 2.6),
      ((UA, (CA, G), GU), 3.7), ((UA, (CC, G), GU), 3.7), ((UA, (UA, G), GU), 3.7), ((UA, (CU, G), GU), 3.7),
      ((UA, (GA, G), GU), 2.6), ((UA, (UA, G), GU), 2.6), ((UA, (GG, G), GU), 2.6), ((UA, (GU, G), GU), 2.6),
      ((UA, (UA, G), GU), 3.7), ((UA, (UC, G), GU), 3.7), ((UA, (UG, G), GU), 3.7), ((UA, (UU, G), GU), 3.0),
      // For internal loops between the base pairs "UA" and "UG".
      ((UA, (AA, G), UG), 2.6), ((UA, (AC, G), UG), 2.6), ((UA, (AG, G), UG), 2.6), ((UA, (UA, G), UG), 2.6),
      ((UA, (CA, G), UG), 3.7), ((UA, (CC, G), UG), 3.7), ((UA, (UA, G), UG), 3.7), ((UA, (CU, G), UG), 3.7),
      ((UA, (GA, G), UG), 2.6), ((UA, (UA, G), UG), 2.6), ((UA, (GG, G), UG), 2.6), ((UA, (GU, G), UG), 2.6),
      ((UA, (UA, G), UG), 3.7), ((UA, (UC, G), UG), 3.7), ((UA, (UG, G), UG), 3.7), ((UA, (UU, G), UG), 3.0),
      // For internal loops between the base pairs "UA" and "AU".
      ((UA, (AA, U), AU), 3.7), ((UA, (AC, U), AU), 3.7), ((UA, (AG, U), AU), 2.6), ((UA, (UA, U), AU), 3.7),
      ((UA, (CA, U), AU), 3.7), ((UA, (CC, U), AU), 3.7), ((UA, (UA, U), AU), 3.7), ((UA, (CU, U), AU), 3.7),
      ((UA, (GA, U), AU), 2.6), ((UA, (UA, U), AU), 3.7), ((UA, (GG, U), AU), 2.6), ((UA, (GU, U), AU), 3.7),
      ((UA, (UA, U), AU), 3.0), ((UA, (UC, U), AU), 3.0), ((UA, (UG, U), AU), 3.0), ((UA, (UU, U), AU), 3.0),
      // For internal loops between the base pairs "UA" and "CG".
      ((UA, (AA, U), CG), 3.0), ((UA, (AC, U), CG), 3.0), ((UA, (AG, U), CG), 1.9), ((UA, (UA, U), CG), 3.0),
      ((UA, (CA, U), CG), 3.0), ((UA, (CC, U), CG), 3.0), ((UA, (UA, A), CG), 3.0), ((UA, (CU, U), CG), 3.0),
      ((UA, (GA, U), CG), 1.9), ((UA, (UA, U), CG), 3.0), ((UA, (GG, U), CG), 1.9), ((UA, (GU, U), CG), 3.0),
      ((UA, (UA, U), CG), 2.2), ((UA, (UC, U), CG), 2.2), ((UA, (UG, U), CG), 2.2), ((UA, (UU, U), CG), 2.2),
      // For internal loops between the base pairs "UA" and "GC".
      ((UA, (AA, U), GC), 3.0), ((UA, (AC, U), GC), 3.0), ((UA, (AG, U), GC), 1.9), ((UA, (UA, U), GC), 3.0),
      ((UA, (CA, U), GC), 3.0), ((UA, (CC, U), GC), 3.0), ((UA, (UA, U), GC), 3.0), ((UA, (CU, U), GC), 3.0),
      ((UA, (GA, U), GC), 1.9), ((UA, (UA, U), GC), 3.0), ((UA, (GG, U), GC), 1.9), ((UA, (GU, U), GC), 3.0),
      ((UA, (UA, U), GC), 2.2), ((UA, (UC, U), GC), 2.2), ((UA, (UG, U), GC), 2.2), ((UA, (UU, U), GC), 2.2),
      // For internal loops between the base pairs "UA" and "UA".
      ((UA, (AA, U), UA), 3.7), ((UA, (AC, U), UA), 3.7), ((UA, (AG, U), UA), 2.6), ((UA, (UA, U), UA), 3.7),
      ((UA, (CA, U), UA), 3.7), ((UA, (CC, U), UA), 3.7), ((UA, (UA, U), UA), 3.7), ((UA, (CU, U), UA), 3.7),
      ((UA, (GA, U), UA), 2.6), ((UA, (UA, U), UA), 3.7), ((UA, (GG, U), UA), 2.6), ((UA, (GU, U), UA), 3.7),
      ((UA, (UA, U), UA), 3.0), ((UA, (UC, U), UA), 3.0), ((UA, (UG, U), UA), 3.0), ((UA, (UU, U), UA), 3.0),
      // For internal loops between the base pairs "UA" and "GU".
      ((UA, (AA, U), GU), 3.7), ((UA, (AC, U), GU), 3.7), ((UA, (AG, U), GU), 2.6), ((UA, (UA, U), GU), 3.7),
      ((UA, (CA, U), GU), 3.7), ((UA, (CC, U), GU), 3.7), ((UA, (UA, U), GU), 3.7), ((UA, (CU, U), GU), 3.7),
      ((UA, (GA, U), GU), 2.6), ((UA, (UA, U), GU), 3.7), ((UA, (GG, U), GU), 2.6), ((UA, (GU, U), GU), 3.7),
      ((UA, (UA, U), GU), 3.0), ((UA, (UC, U), GU), 3.0), ((UA, (UG, U), GU), 3.0), ((UA, (UU, U), GU), 3.0),
      // For internal loops between the base pairs "UA" and "UG".
      ((UA, (AA, U), UG), 3.7), ((UA, (AC, U), UG), 3.7), ((UA, (AG, U), UG), 2.6), ((UA, (UA, U), UG), 3.7),
      ((UA, (CA, U), UG), 3.7), ((UA, (CC, U), UG), 3.7), ((UA, (UA, U), UG), 3.7), ((UA, (CU, U), UG), 3.7),
      ((UA, (GA, U), UG), 2.6), ((UA, (UA, U), UG), 3.7), ((UA, (GG, U), UG), 2.6), ((UA, (GU, U), UG), 3.7),
      ((UA, (UA, U), UG), 3.0), ((UA, (UC, U), UG), 3.0), ((UA, (UG, U), UG), 3.0), ((UA, (UU, U), UG), 3.0),
      // For internal loops behind the base pair "GU".
      // For internal loops between the base pairs "GU" and "AU".
      ((GU, (AA, A), AU), 3.7), ((GU, (AC, A), AU), 3.7), ((GU, (AG, A), AU), 3.7), ((GU, (GU, A), AU), 3.7),
      ((GU, (CA, A), AU), 3.7), ((GU, (CC, A), AU), 3.7), ((GU, (GU, A), AU), 3.7), ((GU, (CU, A), AU), 3.7),
      ((GU, (GA, A), AU), 2.6), ((GU, (GU, A), AU), 2.6), ((GU, (GG, A), AU), 2.6), ((GU, (GU, A), AU), 2.6),
      ((GU, (GU, A), AU), 3.7), ((GU, (UC, A), AU), 3.7), ((GU, (UG, A), AU), 3.7), ((GU, (UU, A), AU), 3.0),
      // For internal loops between the base pairs "GU" and "CG".
      ((GU, (AA, A), CG), 3.0), ((GU, (AC, A), CG), 3.0), ((GU, (AG, A), CG), 3.0), ((GU, (GU, A), CG), 3.0),
      ((GU, (CA, A), CG), 3.0), ((GU, (CC, A), CG), 3.0), ((GU, (GU, A), CG), 3.0), ((GU, (CU, A), CG), 3.0),
      ((GU, (GA, A), CG), 1.9), ((GU, (GU, A), CG), 3.0), ((GU, (GG, A), CG), 1.9), ((GU, (GU, A), CG), 3.0),
      ((GU, (GU, A), CG), 3.0), ((GU, (UC, A), CG), 3.0), ((GU, (UG, A), CG), 3.0), ((GU, (UU, A), CG), 2.2),
      // For internal loops between the base pairs "GU" and "GC".
      ((GU, (AA, A), GC), 2.5), ((GU, (AC, A), GC), 3.0), ((GU, (AG, A), GC), 2.1), ((GU, (GU, A), GC), 3.0),
      ((GU, (CA, A), GC), 3.0), ((GU, (CC, A), GC), 3.0), ((GU, (GU, A), GC), 3.0), ((GU, (CU, A), GC), 3.0),
      ((GU, (GA, A), GC), 1.9), ((GU, (GU, A), GC), 1.9), ((GU, (GG, A), GC), 1.9), ((GU, (GU, A), GC), 1.9),
      ((GU, (GU, A), GC), 3.0), ((GU, (UC, A), GC), 3.0), ((GU, (UG, A), GC), 3.0), ((GU, (UU, A), GC), 2.2),
      // For internal loops between the base pairs "GU" and "UA".
      ((GU, (AA, A), UA), 3.7), ((GU, (AC, A), UA), 3.7), ((GU, (AG, A), UA), 3.7), ((GU, (GU, A), UA), 3.7),
      ((GU, (CA, A), UA), 3.7), ((GU, (CC, A), UA), 3.7), ((GU, (GU, A), UA), 3.7), ((GU, (CU, A), UA), 3.7),
      ((GU, (GA, A), UA), 2.6), ((GU, (GU, A), UA), 3.7), ((GU, (GG, A), UA), 2.6), ((GU, (GU, A), UA), 3.7),
      ((GU, (GU, A), UA), 3.7), ((GU, (UC, A), UA), 3.7), ((GU, (UG, A), UA), 3.7), ((GU, (UU, A), UA), 3.0),
      // For internal loops between the base pairs "GU" and "GU".
      ((GU, (AA, A), GU), 2.5), ((GU, (AC, A), GU), 3.7), ((GU, (AG, A), GU), 2.1), ((GU, (GU, A), GU), 3.7),
      ((GU, (CA, A), GU), 3.7), ((GU, (CC, A), GU), 3.7), ((GU, (GU, A), GU), 3.7), ((GU, (CU, A), GU), 3.7),
      ((GU, (GA, A), GU), 2.6), ((GU, (GU, A), GU), 2.6), ((GU, (GG, A), GU), 2.6), ((GU, (GU, A), GU), 2.6),
      ((GU, (GU, A), GU), 3.7), ((GU, (UC, A), GU), 3.7), ((GU, (UG, A), GU), 3.7), ((GU, (UU, A), GU), 3.0),
      // For internal loops between the base pairs "GU" and "UG".
      ((GU, (AA, A), UG), 3.7), ((GU, (AC, A), UG), 3.7), ((GU, (AG, A), UG), 2.6), ((GU, (GU, A), UG), 3.7),
      ((GU, (CA, A), UG), 3.7), ((GU, (CC, A), UG), 3.7), ((GU, (GU, A), UG), 3.7), ((GU, (CU, A), UG), 3.7),
      ((GU, (GA, A), UG), 2.6), ((GU, (GU, A), UG), 3.7), ((GU, (GG, A), UG), 2.6), ((GU, (GU, A), UG), 3.7),
      ((GU, (GU, A), UG), 3.7), ((GU, (UC, A), UG), 3.7), ((GU, (UG, A), UG), 3.7), ((GU, (UU, A), UG), 3.0),
      // For internal loops between the base pairs "GU" and "AU".
      ((GU, (AA, C), AU), 3.7), ((GU, (AC, C), AU), 3.7), ((GU, (AG, C), AU), 3.7), ((GU, (GU, C), AU), 3.7),
      ((GU, (CA, C), AU), 3.7), ((GU, (CC, C), AU), 3.7), ((GU, (GU, C), AU), 3.7), ((GU, (CU, C), AU), 3.7),
      ((GU, (GA, C), AU), 2.6), ((GU, (GU, C), AU), 3.7), ((GU, (GG, C), AU), 2.6), ((GU, (GU, C), AU), 3.7),
      ((GU, (GU, C), AU), 3.7), ((GU, (UC, C), AU), 3.7), ((GU, (UG, C), AU), 3.7), ((GU, (UU, C), AU), 3.0),
      // For internal loops between the base pairs "GU" and "CG".
      ((GU, (AA, C), CG), 3.0), ((GU, (AC, C), CG), 3.0), ((GU, (AG, C), CG), 3.0), ((GU, (GU, C), CG), 3.0),
      ((GU, (CA, C), CG), 3.0), ((GU, (CC, C), CG), 3.0), ((GU, (GU, A), CG), 3.0), ((GU, (CU, C), CG), 3.0),
      ((GU, (GA, C), CG), 1.9), ((GU, (GU, C), CG), 3.0), ((GU, (GG, C), CG), 1.9), ((GU, (GU, C), CG), 3.0),
      ((GU, (GU, C), CG), 3.0), ((GU, (UC, C), CG), 3.0), ((GU, (UG, C), CG), 3.0), ((GU, (UU, C), CG), 2.2),
      // For internal loops between the base pairs "GU" and "GC".
      ((GU, (AA, C), GC), 3.0), ((GU, (AC, C), GC), 3.0), ((GU, (AG, C), GC), 3.0), ((GU, (GU, C), GC), 3.0),
      ((GU, (CA, C), GC), 3.0), ((GU, (CC, C), GC), 3.0), ((GU, (GU, C), GC), 3.0), ((GU, (CU, C), GC), 3.0),
      ((GU, (GA, C), GC), 1.9), ((GU, (GU, C), GC), 3.0), ((GU, (GG, C), GC), 1.9), ((GU, (GU, C), GC), 3.0),
      ((GU, (GU, C), GC), 3.0), ((GU, (UC, C), GC), 3.0), ((GU, (UG, C), GC), 3.0), ((GU, (UU, C), GC), 2.2),
      // For internal loops between the base pairs "GU" and "UA".
      ((GU, (AA, C), UA), 3.7), ((GU, (AC, C), UA), 3.7), ((GU, (AG, C), UA), 3.7), ((GU, (GU, C), UA), 3.7),
      ((GU, (CA, C), UA), 3.7), ((GU, (CC, C), UA), 3.7), ((GU, (GU, C), UA), 3.7), ((GU, (CU, C), UA), 3.7),
      ((GU, (GA, C), UA), 2.6), ((GU, (GU, C), UA), 3.7), ((GU, (GG, C), UA), 2.6), ((GU, (GU, C), UA), 3.7),
      ((GU, (GU, C), UA), 3.7), ((GU, (UC, C), UA), 3.7), ((GU, (UG, C), UA), 3.7), ((GU, (UU, C), UA), 3.0),
      // For internal loops between the base pairs "GU" and "GU".
      ((GU, (AA, C), GU), 3.7), ((GU, (AC, C), GU), 3.7), ((GU, (AG, C), GU), 3.7), ((GU, (GU, C), GU), 3.7),
      ((GU, (CA, C), GU), 3.7), ((GU, (CC, C), GU), 3.7), ((GU, (GU, C), GU), 3.7), ((GU, (CU, C), GU), 3.7),
      ((GU, (GA, C), GU), 2.6), ((GU, (GU, C), GU), 3.7), ((GU, (GG, C), GU), 2.6), ((GU, (GU, C), GU), 3.7),
      ((GU, (GU, C), GU), 3.7), ((GU, (UC, C), GU), 3.7), ((GU, (UG, C), GU), 3.7), ((GU, (UU, C), GU), 3.0),
      // For internal loops between the base pairs "GU" and "UG".
      ((GU, (AA, C), UG), 3.7), ((GU, (AC, C), UG), 3.7), ((GU, (AG, C), UG), 3.7), ((GU, (GU, C), UG), 3.7),
      ((GU, (CA, C), UG), 3.7), ((GU, (CC, C), UG), 3.7), ((GU, (GU, C), UG), 3.7), ((GU, (CU, C), UG), 3.7),
      ((GU, (GA, C), UG), 2.6), ((GU, (GU, C), UG), 3.7), ((GU, (GG, C), UG), 2.6), ((GU, (GU, C), UG), 3.7),
      ((GU, (GU, C), UG), 3.7), ((GU, (UC, C), UG), 3.7), ((GU, (UG, C), UG), 3.7), ((GU, (UU, C), UG), 3.0),
      // For internal loops between the base pairs "GU" and "AU".
      ((GU, (AA, G), AU), 2.6), ((GU, (AC, G), AU), 2.6), ((GU, (AG, G), AU), 2.6), ((GU, (GU, G), AU), 2.6),
      ((GU, (CA, G), AU), 3.7), ((GU, (CC, G), AU), 3.7), ((GU, (GU, G), AU), 3.7), ((GU, (CU, G), AU), 3.7),
      ((GU, (GA, G), AU), 2.6), ((GU, (GU, G), AU), 2.6), ((GU, (GG, G), AU), 2.6), ((GU, (GU, G), AU), 2.6),
      ((GU, (GU, G), AU), 3.7), ((GU, (UC, G), AU), 3.7), ((GU, (UG, G), AU), 3.7), ((GU, (UU, G), AU), 3.0),
      // For internal loops between the base pairs "GU" and "CG".
      ((GU, (AA, G), CG), 1.9), ((GU, (AC, G), CG), 1.9), ((GU, (AG, G), CG), 1.9), ((GU, (GU, G), CG), 1.9),
      ((GU, (CA, G), CG), 3.0), ((GU, (CC, G), CG), 3.0), ((GU, (GU, A), CG), 3.0), ((GU, (CU, G), CG), 3.0),
      ((GU, (GA, G), CG), 1.9), ((GU, (GU, G), CG), 1.9), ((GU, (GG, G), CG), 1.9), ((GU, (GU, G), CG), 1.9),
      ((GU, (GU, G), CG), 3.0), ((GU, (UC, G), CG), 3.0), ((GU, (UG, G), CG), 3.0), ((GU, (UU, G), CG), 2.2),
      // For internal loops between the base pairs "GU" and "GC".
      ((GU, (AA, G), GC), 1.2), ((GU, (AC, G), GC), 1.9), ((GU, (AG, G), GC), 1.9), ((GU, (GU, G), GC), 1.9),
      ((GU, (CA, G), GC), 3.0), ((GU, (CC, G), GC), 3.0), ((GU, (GU, G), GC), 3.0), ((GU, (CU, G), GC), 3.0),
      ((GU, (GA, G), GC), 1.9), ((GU, (GU, G), GC), 1.9), ((GU, (GG, G), GC), 1.9), ((GU, (GU, G), GC), 1.9),
      ((GU, (GU, G), GC), 3.0), ((GU, (UC, G), GC), 3.0), ((GU, (UG, G), GC), 3.0), ((GU, (UU, G), GC), 2.2),
      // For internal loops between the base pairs "GU" and "UA".
      ((GU, (AA, G), UA), 2.6), ((GU, (AC, G), UA), 2.6), ((GU, (AG, G), UA), 2.6), ((GU, (GU, G), UA), 2.6),
      ((GU, (CA, G), UA), 3.7), ((GU, (CC, G), UA), 3.7), ((GU, (GU, G), UA), 3.7), ((GU, (CU, G), UA), 3.7),
      ((GU, (GA, G), UA), 2.6), ((GU, (GU, G), UA), 2.6), ((GU, (GG, G), UA), 2.6), ((GU, (GU, G), UA), 2.6),
      ((GU, (GU, G), UA), 3.7), ((GU, (UC, G), UA), 3.7), ((GU, (UG, G), UA), 3.7), ((GU, (UU, G), UA), 3.0),
      // For internal loops between the base pairs "GU" and "GU".
      ((GU, (AA, G), GU), 1.2), ((GU, (AC, G), GU), 2.6), ((GU, (AG, G), GU), 2.6), ((GU, (GU, G), GU), 2.6),
      ((GU, (CA, G), GU), 3.7), ((GU, (CC, G), GU), 3.7), ((GU, (GU, G), GU), 3.7), ((GU, (CU, G), GU), 3.7),
      ((GU, (GA, G), GU), 2.6), ((GU, (GU, G), GU), 2.6), ((GU, (GG, G), GU), 2.6), ((GU, (GU, G), GU), 2.6),
      ((GU, (GU, G), GU), 3.7), ((GU, (UC, G), GU), 3.7), ((GU, (UG, G), GU), 3.7), ((GU, (UU, G), GU), 3.0),
      // For internal loops between the base pairs "GU" and "UG".
      ((GU, (AA, G), UG), 2.6), ((GU, (AC, G), UG), 2.6), ((GU, (AG, G), UG), 2.6), ((GU, (GU, G), UG), 2.6),
      ((GU, (CA, G), UG), 3.7), ((GU, (CC, G), UG), 3.7), ((GU, (GU, G), UG), 3.7), ((GU, (CU, G), UG), 3.7),
      ((GU, (GA, G), UG), 2.6), ((GU, (GU, G), UG), 2.6), ((GU, (GG, G), UG), 2.6), ((GU, (GU, G), UG), 2.6),
      ((GU, (GU, G), UG), 3.7), ((GU, (UC, G), UG), 3.7), ((GU, (UG, G), UG), 3.7), ((GU, (UU, G), UG), 3.0),
      // For internal loops between the base pairs "GU" and "AU".
      ((GU, (AA, U), AU), 3.7), ((GU, (AC, U), AU), 3.7), ((GU, (AG, U), AU), 3.7), ((GU, (GU, U), AU), 3.7),
      ((GU, (CA, U), AU), 3.7), ((GU, (CC, U), AU), 3.7), ((GU, (GU, U), AU), 3.7), ((GU, (CU, U), AU), 3.7),
      ((GU, (GA, U), AU), 2.6), ((GU, (GU, U), AU), 3.7), ((GU, (GG, U), AU), 2.6), ((GU, (GU, U), AU), 3.7),
      ((GU, (GU, U), AU), 3.0), ((GU, (UC, U), AU), 3.0), ((GU, (UG, U), AU), 3.0), ((GU, (UU, U), AU), 3.0),
      // For internal loops between the base pairs "GU" and "CG".
      ((GU, (AA, U), CG), 3.0), ((GU, (AC, U), CG), 3.0), ((GU, (AG, U), CG), 3.0), ((GU, (GU, U), CG), 3.0),
      ((GU, (CA, U), CG), 3.0), ((GU, (CC, U), CG), 3.0), ((GU, (GU, A), CG), 3.0), ((GU, (CU, U), CG), 3.0),
      ((GU, (GA, U), CG), 1.9), ((GU, (GU, U), CG), 3.0), ((GU, (GG, U), CG), 1.9), ((GU, (GU, U), CG), 3.0),
      ((GU, (GU, U), CG), 2.2), ((GU, (UC, U), CG), 2.2), ((GU, (UG, U), CG), 2.2), ((GU, (UU, U), CG), 2.2),
      // For internal loops between the base pairs "GU" and "GC".
      ((GU, (AA, U), GC), 3.0), ((GU, (AC, U), GC), 3.0), ((GU, (AG, U), GC), 3.0), ((GU, (GU, U), GC), 3.0),
      ((GU, (CA, U), GC), 3.0), ((GU, (CC, U), GC), 1.9), ((GU, (GU, U), GC), 3.0), ((GU, (CU, U), GC), 3.0),
      ((GU, (GA, U), GC), 1.9), ((GU, (GU, U), GC), 3.0), ((GU, (GG, U), GC), 1.9), ((GU, (GU, U), GC), 3.0),
      ((GU, (GU, U), GC), 2.2), ((GU, (UC, U), GC), 2.2), ((GU, (UG, U), GC), 2.2), ((GU, (UU, U), GC), 2.2),
      // For internal loops between the base pairs "GU" and "UA".
      ((GU, (AA, U), UA), 3.7), ((GU, (AC, U), UA), 3.7), ((GU, (AG, U), UA), 3.7), ((GU, (GU, U), UA), 3.7),
      ((GU, (CA, U), UA), 3.7), ((GU, (CC, U), UA), 3.7), ((GU, (GU, U), UA), 3.7), ((GU, (CU, U), UA), 3.7),
      ((GU, (GA, U), UA), 2.6), ((GU, (GU, U), UA), 3.7), ((GU, (GG, U), UA), 2.6), ((GU, (GU, U), UA), 3.7),
      ((GU, (GU, U), UA), 3.0), ((GU, (UC, U), UA), 3.0), ((GU, (UG, U), UA), 3.0), ((GU, (UU, U), UA), 3.0),
      // For internal loops between the base pairs "GU" and "GU".
      ((GU, (AA, U), GU), 3.7), ((GU, (AC, U), GU), 3.7), ((GU, (AG, U), GU), 3.7), ((GU, (GU, U), GU), 3.7),
      ((GU, (CA, U), GU), 3.7), ((GU, (CC, U), GU), 1.9), ((GU, (GU, U), GU), 3.7), ((GU, (CU, U), GU), 3.7),
      ((GU, (GA, U), GU), 2.6), ((GU, (GU, U), GU), 3.7), ((GU, (GG, U), GU), 2.6), ((GU, (GU, U), GU), 3.7),
      ((GU, (GU, U), GU), 3.0), ((GU, (UC, U), GU), 3.0), ((GU, (UG, U), GU), 3.0), ((GU, (UU, U), GU), 3.0),
      // For internal loops between the base pairs "GU" and "UG".
      ((GU, (AA, U), UG), 3.7), ((GU, (AC, U), UG), 3.7), ((GU, (AG, U), UG), 3.7), ((GU, (GU, U), UG), 3.7),
      ((GU, (CA, U), UG), 3.7), ((GU, (CC, U), UG), 3.7), ((GU, (GU, U), UG), 3.7), ((GU, (CU, U), UG), 3.7),
      ((GU, (GA, U), UG), 2.6), ((GU, (GU, U), UG), 3.7), ((GU, (GG, U), UG), 2.6), ((GU, (GU, U), UG), 3.7),
      ((GU, (GU, U), UG), 3.0), ((GU, (UC, U), UG), 3.0), ((GU, (UG, U), UG), 3.0), ((GU, (UU, U), UG), 3.0),
      // For internal loops behind the base pair "UG".
      // For internal loops between the base pairs "UG" and "AU".
      ((UG, (AA, A), AU), 3.7), ((UG, (AC, A), AU), 3.7), ((UG, (AG, A), AU), 2.6), ((UG, (UG, A), AU), 3.7),
      ((UG, (CA, A), AU), 3.7), ((UG, (CC, A), AU), 3.7), ((UG, (UG, A), AU), 3.7), ((UG, (CU, A), AU), 3.7),
      ((UG, (GA, A), AU), 2.6), ((UG, (UG, A), AU), 2.6), ((UG, (GG, A), AU), 2.6), ((UG, (UG, A), AU), 2.6),
      ((UG, (UG, A), AU), 3.7), ((UG, (UC, A), AU), 3.7), ((UG, (UG, A), AU), 3.7), ((UG, (UU, A), AU), 3.0),
      // For internal loops between the base pairs "UG" and "CG".
      ((UG, (AA, A), CG), 3.0), ((UG, (AC, A), CG), 3.0), ((UG, (AG, A), CG), 3.0), ((UG, (UG, A), CG), 3.0),
      ((UG, (CA, A), CG), 3.0), ((UG, (CC, A), CG), 3.0), ((UG, (UG, A), CG), 3.0), ((UG, (CU, A), CG), 3.0),
      ((UG, (GA, A), CG), 1.9), ((UG, (UG, A), CG), 3.0), ((UG, (GG, A), CG), 1.9), ((UG, (UG, A), CG), 3.0),
      ((UG, (UG, A), CG), 3.0), ((UG, (UC, A), CG), 3.0), ((UG, (UG, A), CG), 3.0), ((UG, (UU, A), CG), 2.2),
      // For internal loops between the base pairs "UG" and "GC".
      ((UG, (AA, A), GC), 3.0), ((UG, (AC, A), GC), 3.0), ((UG, (AG, A), GC), 1.9), ((UG, (UG, A), GC), 3.0),
      ((UG, (CA, A), GC), 3.0), ((UG, (CC, A), GC), 3.0), ((UG, (UG, A), GC), 3.0), ((UG, (CU, A), GC), 3.0),
      ((UG, (GA, A), GC), 1.9), ((UG, (UG, A), GC), 1.9), ((UG, (GG, A), GC), 1.9), ((UG, (UG, A), GC), 1.9),
      ((UG, (UG, A), GC), 3.0), ((UG, (UC, A), GC), 3.0), ((UG, (UG, A), GC), 3.0), ((UG, (UU, A), GC), 2.2),
      // For internal loops between the base pairs "UG" and "UA".
      ((UG, (AA, A), UA), 3.7), ((UG, (AC, A), UA), 3.7), ((UG, (AG, A), UA), 2.6), ((UG, (UG, A), UA), 3.7),
      ((UG, (CA, A), UA), 3.7), ((UG, (CC, A), UA), 3.7), ((UG, (UG, A), UA), 3.7), ((UG, (CU, A), UA), 3.7),
      ((UG, (GA, A), UA), 2.6), ((UG, (UG, A), UA), 3.7), ((UG, (GG, A), UA), 2.6), ((UG, (UG, A), UA), 3.7),
      ((UG, (UG, A), UA), 3.7), ((UG, (UC, A), UA), 3.7), ((UG, (UG, A), UA), 3.7), ((UG, (UU, A), UA), 3.0),
      // For internal loops between the base pairs "UG" and "GU".
      ((UG, (AA, A), GU), 2.5), ((UG, (AC, A), GU), 3.7), ((UG, (AG, A), GU), 2.6), ((UG, (UG, A), GU), 3.7),
      ((UG, (CA, A), GU), 3.7), ((UG, (CC, A), GU), 3.7), ((UG, (UG, A), GU), 3.7), ((UG, (CU, A), GU), 3.7),
      ((UG, (GA, A), GU), 2.6), ((UG, (UG, A), GU), 2.6), ((UG, (GG, A), GU), 2.6), ((UG, (UG, A), GU), 2.6),
      ((UG, (UG, A), GU), 3.7), ((UG, (UC, A), GU), 3.7), ((UG, (UG, A), GU), 3.7), ((UG, (UU, A), GU), 3.0),
      // For internal loops between the base pairs "UG" and "UG".
      ((UG, (AA, A), UG), 3.7), ((UG, (AC, A), UG), 3.7), ((UG, (AG, A), UG), 2.6), ((UG, (UG, A), UG), 3.7),
      ((UG, (CA, A), UG), 3.7), ((UG, (CC, A), UG), 3.7), ((UG, (UG, A), UG), 3.7), ((UG, (CU, A), UG), 3.7),
      ((UG, (GA, A), UG), 2.6), ((UG, (UG, A), UG), 3.7), ((UG, (GG, A), UG), 2.6), ((UG, (UG, A), UG), 3.7),
      ((UG, (UG, A), UG), 3.7), ((UG, (UC, A), UG), 3.7), ((UG, (UG, A), UG), 3.7), ((UG, (UU, A), UG), 3.0),
      // For internal loops between the base pairs "UG" and "AU".
      ((UG, (AA, C), AU), 3.7), ((UG, (AC, C), AU), 3.7), ((UG, (AG, C), AU), 2.6), ((UG, (UG, C), AU), 3.7),
      ((UG, (CA, C), AU), 3.7), ((UG, (CC, C), AU), 3.7), ((UG, (UG, C), AU), 3.7), ((UG, (CU, C), AU), 3.7),
      ((UG, (GA, C), AU), 2.6), ((UG, (UG, C), AU), 3.7), ((UG, (GG, C), AU), 2.6), ((UG, (UG, C), AU), 3.7),
      ((UG, (UG, C), AU), 3.7), ((UG, (UC, C), AU), 3.7), ((UG, (UG, C), AU), 3.7), ((UG, (UU, C), AU), 3.0),
      // For internal loops between the base pairs "UG" and "CG".
      ((UG, (AA, C), CG), 3.0), ((UG, (AC, C), CG), 3.0), ((UG, (AG, C), CG), 1.9), ((UG, (UG, C), CG), 3.0),
      ((UG, (CA, C), CG), 3.0), ((UG, (CC, C), CG), 3.0), ((UG, (UG, A), CG), 3.0), ((UG, (CU, C), CG), 3.0),
      ((UG, (GA, C), CG), 1.9), ((UG, (UG, C), CG), 3.0), ((UG, (GG, C), CG), 1.9), ((UG, (UG, C), CG), 3.0),
      ((UG, (UG, C), CG), 3.0), ((UG, (UC, C), CG), 3.0), ((UG, (UG, C), CG), 3.0), ((UG, (UU, C), CG), 2.2),
      // For internal loops between the base pairs "UG" and "GC".
      ((UG, (AA, C), GC), 3.0), ((UG, (AC, C), GC), 3.0), ((UG, (AG, C), GC), 1.9), ((UG, (UG, C), GC), 3.0),
      ((UG, (CA, C), GC), 3.0), ((UG, (CC, C), GC), 3.0), ((UG, (UG, C), GC), 3.0), ((UG, (CU, C), GC), 3.0),
      ((UG, (GA, C), GC), 1.9), ((UG, (UG, C), GC), 3.0), ((UG, (GG, C), GC), 1.9), ((UG, (UG, C), GC), 3.0),
      ((UG, (UG, C), GC), 3.0), ((UG, (UC, C), GC), 3.0), ((UG, (UG, C), GC), 3.0), ((UG, (UU, C), GC), 2.2),
      // For internal loops between the base pairs "UG" and "UA".
      ((UG, (AA, C), UA), 3.7), ((UG, (AC, C), UA), 3.7), ((UG, (AG, C), UA), 2.6), ((UG, (UG, C), UA), 3.7),
      ((UG, (CA, C), UA), 3.7), ((UG, (CC, C), UA), 3.7), ((UG, (UG, C), UA), 3.7), ((UG, (CU, C), UA), 3.7),
      ((UG, (GA, C), UA), 2.6), ((UG, (UG, C), UA), 3.7), ((UG, (GG, C), UA), 2.6), ((UG, (UG, C), UA), 3.7),
      ((UG, (UG, C), UA), 3.7), ((UG, (UC, C), UA), 3.7), ((UG, (UG, C), UA), 3.7), ((UG, (UU, C), UA), 3.0),
      // For internal loops between the base pairs "UG" and "GU".
      ((UG, (AA, C), GU), 3.7), ((UG, (AC, C), GU), 3.7), ((UG, (AG, C), GU), 2.6), ((UG, (UG, C), GU), 3.7),
      ((UG, (CA, C), GU), 3.7), ((UG, (CC, C), GU), 3.7), ((UG, (UG, C), GU), 3.7), ((UG, (CU, C), GU), 3.7),
      ((UG, (GA, C), GU), 2.6), ((UG, (UG, C), GU), 3.7), ((UG, (GG, C), GU), 2.6), ((UG, (UG, C), GU), 3.7),
      ((UG, (UG, C), GU), 3.7), ((UG, (UC, C), GU), 3.7), ((UG, (UG, C), GU), 3.7), ((UG, (UU, C), GU), 3.0),
      // For internal loops between the base pairs "UG" and "UG".
      ((UG, (AA, C), UG), 3.7), ((UG, (AC, C), UG), 3.7), ((UG, (AG, C), UG), 2.6), ((UG, (UG, C), UG), 3.7),
      ((UG, (CA, C), UG), 3.7), ((UG, (CC, C), UG), 3.7), ((UG, (UG, C), UG), 3.7), ((UG, (CU, C), UG), 3.7),
      ((UG, (GA, C), UG), 2.6), ((UG, (UG, C), UG), 3.7), ((UG, (GG, C), UG), 2.6), ((UG, (UG, C), UG), 3.7),
      ((UG, (UG, C), UG), 3.7), ((UG, (UC, C), UG), 3.7), ((UG, (UG, C), UG), 3.7), ((UG, (UU, C), UG), 3.0),
      // For internal loops between the base pairs "UG" and "AU".
      ((UG, (AA, G), AU), 2.6), ((UG, (AC, G), AU), 2.6), ((UG, (AG, G), AU), 2.6), ((UG, (UG, G), AU), 2.6),
      ((UG, (CA, G), AU), 3.7), ((UG, (CC, G), AU), 3.7), ((UG, (UG, G), AU), 3.7), ((UG, (CU, G), AU), 3.7),
      ((UG, (GA, G), AU), 2.6), ((UG, (UG, G), AU), 2.6), ((UG, (GG, G), AU), 2.6), ((UG, (UG, G), AU), 2.6),
      ((UG, (UG, G), AU), 3.7), ((UG, (UC, G), AU), 3.7), ((UG, (UG, G), AU), 3.7), ((UG, (UU, G), AU), 3.0),
      // For internal loops between the base pairs "UG" and "CG".
      ((UG, (AA, G), CG), 1.9), ((UG, (AC, G), CG), 1.9), ((UG, (AG, G), CG), 1.9), ((UG, (UG, G), CG), 1.9),
      ((UG, (CA, G), CG), 3.0), ((UG, (CC, G), CG), 3.0), ((UG, (UG, A), CG), 3.0), ((UG, (CU, G), CG), 3.0),
      ((UG, (GA, G), CG), 1.9), ((UG, (UG, G), CG), 1.9), ((UG, (GG, G), CG), 1.9), ((UG, (UG, G), CG), 1.9),
      ((UG, (UG, G), CG), 3.0), ((UG, (UC, G), CG), 3.0), ((UG, (UG, G), CG), 3.0), ((UG, (UU, G), CG), 2.2),
      // For internal loops between the base pairs "UG" and "GC".
      ((UG, (AA, G), GC), 1.9), ((UG, (AC, G), GC), 1.9), ((UG, (AG, G), GC), 1.9), ((UG, (UG, G), GC), 1.9),
      ((UG, (CA, G), GC), 3.0), ((UG, (CC, G), GC), 3.0), ((UG, (UG, G), GC), 3.0), ((UG, (CU, G), GC), 3.0),
      ((UG, (GA, G), GC), 1.9), ((UG, (UG, G), GC), 1.9), ((UG, (GG, G), GC), 1.9), ((UG, (UG, G), GC), 1.9),
      ((UG, (UG, G), GC), 3.0), ((UG, (UC, G), GC), 3.0), ((UG, (UG, G), GC), 3.0), ((UG, (UU, G), GC), 2.2),
      // For internal loops between the base pairs "UG" and "UA".
      ((UG, (AA, G), UA), 2.6), ((UG, (AC, G), UA), 2.6), ((UG, (AG, G), UA), 2.6), ((UG, (UG, G), UA), 2.6),
      ((UG, (CA, G), UA), 3.7), ((UG, (CC, G), UA), 3.7), ((UG, (UG, G), UA), 3.7), ((UG, (CU, G), UA), 3.7),
      ((UG, (GA, G), UA), 2.6), ((UG, (UG, G), UA), 2.6), ((UG, (GG, G), UA), 2.6), ((UG, (UG, G), UA), 2.6),
      ((UG, (UG, G), UA), 3.7), ((UG, (UC, G), UA), 3.7), ((UG, (UG, G), UA), 3.7), ((UG, (UU, G), UA), 3.0),
      // For internal loops between the base pairs "UG" and "GU".
      ((UG, (AA, G), GU), 2.6), ((UG, (AC, G), GU), 2.6), ((UG, (AG, G), GU), 2.6), ((UG, (UG, G), GU), 2.6),
      ((UG, (CA, G), GU), 3.7), ((UG, (CC, G), GU), 3.7), ((UG, (UG, G), GU), 3.7), ((UG, (CU, G), GU), 3.7),
      ((UG, (GA, G), GU), 2.6), ((UG, (UG, G), GU), 2.6), ((UG, (GG, G), GU), 2.6), ((UG, (UG, G), GU), 2.6),
      ((UG, (UG, G), GU), 3.7), ((UG, (UC, G), GU), 3.7), ((UG, (UG, G), GU), 3.7), ((UG, (UU, G), GU), 3.0),
      // For internal loops between the base pairs "UG" and "UG".
      ((UG, (AA, G), UG), 2.6), ((UG, (AC, G), UG), 2.6), ((UG, (AG, G), UG), 2.6), ((UG, (UG, G), UG), 2.6),
      ((UG, (CA, G), UG), 3.7), ((UG, (CC, G), UG), 3.7), ((UG, (UG, G), UG), 3.7), ((UG, (CU, G), UG), 3.7),
      ((UG, (GA, G), UG), 2.6), ((UG, (UG, G), UG), 2.6), ((UG, (GG, G), UG), 2.6), ((UG, (UG, G), UG), 2.6),
      ((UG, (UG, G), UG), 3.7), ((UG, (UC, G), UG), 3.7), ((UG, (UG, G), UG), 3.7), ((UG, (UU, G), UG), 3.0),
      // For internal loops between the base pairs "UG" and "AU".
      ((UG, (AA, U), AU), 3.7), ((UG, (AC, U), AU), 3.7), ((UG, (AG, U), AU), 2.6), ((UG, (UG, U), AU), 3.7),
      ((UG, (CA, U), AU), 3.7), ((UG, (CC, U), AU), 3.7), ((UG, (UG, U), AU), 3.7), ((UG, (CU, U), AU), 3.7),
      ((UG, (GA, U), AU), 2.6), ((UG, (UG, U), AU), 3.7), ((UG, (GG, U), AU), 2.6), ((UG, (UG, U), AU), 3.7),
      ((UG, (UG, U), AU), 3.0), ((UG, (UC, U), AU), 3.0), ((UG, (UG, U), AU), 3.0), ((UG, (UU, U), AU), 3.0),
      // For internal loops between the base pairs "UG" and "CG".
      ((UG, (AA, U), CG), 3.0), ((UG, (AC, U), CG), 3.0), ((UG, (AG, U), CG), 1.9), ((UG, (UG, U), CG), 3.0),
      ((UG, (CA, U), CG), 3.0), ((UG, (CC, U), CG), 3.0), ((UG, (UG, A), CG), 3.0), ((UG, (CU, U), CG), 3.0),
      ((UG, (GA, U), CG), 1.9), ((UG, (UG, U), CG), 3.0), ((UG, (GG, U), CG), 1.9), ((UG, (UG, U), CG), 3.0),
      ((UG, (UG, U), CG), 2.2), ((UG, (UC, U), CG), 2.2), ((UG, (UG, U), CG), 2.2), ((UG, (UU, U), CG), 2.2),
      // For internal loops between the base pairs "UG" and "GC".
      ((UG, (AA, U), GC), 3.0), ((UG, (AC, U), GC), 3.0), ((UG, (AG, U), GC), 1.9), ((UG, (UG, U), GC), 3.0),
      ((UG, (CA, U), GC), 3.0), ((UG, (CC, U), GC), 1.9), ((UG, (UG, U), GC), 3.0), ((UG, (CU, U), GC), 3.0),
      ((UG, (GA, U), GC), 1.9), ((UG, (UG, U), GC), 3.0), ((UG, (GG, U), GC), 1.9), ((UG, (UG, U), GC), 3.0),
      ((UG, (UG, U), GC), 2.2), ((UG, (UC, U), GC), 2.2), ((UG, (UG, U), GC), 2.2), ((UG, (UU, U), GC), 2.2),
      // For internal loops between the base pairs "UG" and "UA".
      ((UG, (AA, U), UA), 3.7), ((UG, (AC, U), UA), 3.7), ((UG, (AG, U), UA), 2.6), ((UG, (UG, U), UA), 3.7),
      ((UG, (CA, U), UA), 3.7), ((UG, (CC, U), UA), 3.7), ((UG, (UG, U), UA), 3.7), ((UG, (CU, U), UA), 3.7),
      ((UG, (GA, U), UA), 2.6), ((UG, (UG, U), UA), 3.7), ((UG, (GG, U), UA), 2.6), ((UG, (UG, U), UA), 3.7),
      ((UG, (UG, U), UA), 3.0), ((UG, (UC, U), UA), 3.0), ((UG, (UG, U), UA), 3.0), ((UG, (UU, U), UA), 3.0),
      // For internal loops between the base pairs "UG" and "GU".
      ((UG, (AA, U), GU), 3.7), ((UG, (AC, U), GU), 3.7), ((UG, (AG, U), GU), 2.6), ((UG, (UG, U), GU), 3.7),
      ((UG, (CA, U), GU), 3.7), ((UG, (CC, U), GU), 3.7), ((UG, (UG, U), GU), 3.7), ((UG, (CU, U), GU), 3.7),
      ((UG, (GA, U), GU), 2.6), ((UG, (UG, U), GU), 3.7), ((UG, (GG, U), GU), 2.6), ((UG, (UG, U), GU), 3.7),
      ((UG, (UG, U), GU), 3.0), ((UG, (UC, U), GU), 3.0), ((UG, (UG, U), GU), 3.0), ((UG, (UU, U), GU), 3.0),
      // For internal loops between the base pairs "UG" and "UG".
      ((UG, (AA, U), UG), 3.7), ((UG, (AC, U), UG), 3.7), ((UG, (AG, U), UG), 2.6), ((UG, (UG, U), UG), 3.7),
      ((UG, (CA, U), UG), 3.7), ((UG, (CC, U), UG), 3.7), ((UG, (UG, U), UG), 3.7), ((UG, (CU, U), UG), 3.7),
      ((UG, (GA, U), UG), 2.6), ((UG, (UG, U), UG), 3.7), ((UG, (GG, U), UG), 2.6), ((UG, (UG, U), UG), 3.7),
      ((UG, (UG, U), UG), 3.0), ((UG, (UC, U), UG), 3.0), ((UG, (UG, U), UG), 3.0), ((UG, (UU, U), UG), 3.0),
    ].iter().cloned().collect()
  };
    pub static ref INIT_2_VS_2_INTERNAL_LOOP_DELTA_FES: Init2Vs2InternalLoopDeltaFes = {
    [
      // For internal loops behind the base pair "AU".
      // For internal loops between the base pairs "AU" and "AU".
      ((AU, (AA, AA), AU), 2.8), ((AU, (AA, AC), AU), 2.3), ((AU, (AA, AG), AU), 1.7), ((AU, (AA, AU), AU), 2.3), ((AU, (AA, CA), AU), 2.8), ((AU, (AA, CC), AU), 2.8), ((AU, (AA, CG), AU), 2.8), ((AU, (AA, CU), AU), 2.8), ((AU, (AA, GA), AU), 1.8), ((AU, (AA, GC), AU), 2.3), ((AU, (AA, GG), AU), 1.2), ((AU, (AA, GU), AU), 2.3), ((AU, (AA, UA), AU), 2.8), ((AU, (AA, UC), AU), 2.5), ((AU, (AA, UG), AU), 2.8), ((AU, (AA, UU), AU), 2.5),
      ((AU, (AC, AA), AU), 2.6), ((AU, (AC, AC), AU), 2.2), ((AU, (AC, AG), AU), 1.6), ((AU, (AC, AU), AU), 2.2), ((AU, (AC, CA), AU), 2.6), ((AU, (AC, CC), AU), 2.6), ((AU, (AC, CG), AU), 2.6), ((AU, (AC, CU), AU), 2.6), ((AU, (AC, GA), AU), 1.7), ((AU, (AC, GC), AU), 2.2), ((AU, (AC, GG), AU), 1.1), ((AU, (AC, GU), AU), 2.2), ((AU, (AC, UA), AU), 2.6), ((AU, (AC, UC), AU), 2.3), ((AU, (AC, UG), AU), 2.6), ((AU, (AC, UU), AU), 1.8),
      ((AU, (AG, AA), AU), 2.2), ((AU, (AG, AC), AU), 1.8), ((AU, (AG, AG), AU), 1.2), ((AU, (AG, AU), AU), 1.8), ((AU, (AG, CA), AU), 2.2), ((AU, (AG, CC), AU), 2.8), ((AU, (AG, CG), AU), 2.2), ((AU, (AG, CU), AU), 2.8), ((AU, (AG, GA), AU), 1.3), ((AU, (AG, GC), AU), 1.8), ((AU, (AG, GG), AU), 2.0), ((AU, (AG, GU), AU), 1.8), ((AU, (AG, UA), AU), 2.2), ((AU, (AG, UC), AU), 2.5), ((AU, (AG, UG), AU), 2.2), ((AU, (AG, UU), AU), 1.4),
      ((AU, (AU, AA), AU), 2.6), ((AU, (AU, AC), AU), 2.2), ((AU, (AU, AG), AU), 1.6), ((AU, (AU, AU), AU), 2.2), ((AU, (AU, CA), AU), 2.6), ((AU, (AU, CC), AU), 2.6), ((AU, (AU, CG), AU), 2.6), ((AU, (AU, CU), AU), 2.6), ((AU, (AU, GA), AU), 1.7), ((AU, (AU, GC), AU), 2.2), ((AU, (AU, GG), AU), 1.1), ((AU, (AU, GU), AU), 2.2), ((AU, (AU, UA), AU), 2.6), ((AU, (AU, UC), AU), 2.3), ((AU, (AU, UG), AU), 2.6), ((AU, (AU, UU), AU), 1.8),
      ((AU, (CA, AA), AU), 2.5), ((AU, (CA, AC), AU), 2.1), ((AU, (CA, AG), AU), 1.5), ((AU, (CA, AU), AU), 2.1), ((AU, (CA, CA), AU), 2.5), ((AU, (CA, CC), AU), 2.5), ((AU, (CA, CG), AU), 2.5), ((AU, (CA, CU), AU), 2.5), ((AU, (CA, GA), AU), 1.6), ((AU, (CA, GC), AU), 2.1), ((AU, (CA, GG), AU), 1.0), ((AU, (CA, GU), AU), 2.1), ((AU, (CA, UA), AU), 2.5), ((AU, (CA, UC), AU), 2.2), ((AU, (CA, UG), AU), 2.5), ((AU, (CA, UU), AU), 1.7),
      ((AU, (CC, AA), AU), 2.6), ((AU, (CC, AC), AU), 2.1), ((AU, (CC, AG), AU), 2.1), ((AU, (CC, AU), AU), 2.1), ((AU, (CC, CA), AU), 2.6), ((AU, (CC, CC), AU), 2.6), ((AU, (CC, CG), AU), 2.6), ((AU, (CC, CU), AU), 2.6), ((AU, (CC, GA), AU), 2.2), ((AU, (CC, GC), AU), 2.1), ((AU, (CC, GG), AU), 1.0), ((AU, (CC, GU), AU), 2.1), ((AU, (CC, UA), AU), 2.6), ((AU, (CC, UC), AU), 2.3), ((AU, (CC, UG), AU), 2.6), ((AU, (CC, UU), AU), 1.8),
      ((AU, (CG, AA), AU), 2.5), ((AU, (CG, AC), AU), 2.1), ((AU, (CG, AG), AU), 1.5), ((AU, (CG, AU), AU), 2.1), ((AU, (CG, CA), AU), 2.5), ((AU, (CG, CC), AU), 2.5), ((AU, (CG, CG), AU), 2.5), ((AU, (CG, CU), AU), 2.5), ((AU, (CG, GA), AU), 1.6), ((AU, (CG, GC), AU), 2.1), ((AU, (CG, GG), AU), 1.0), ((AU, (CG, GU), AU), 2.1), ((AU, (CG, UA), AU), 2.5), ((AU, (CG, UC), AU), 2.2), ((AU, (CG, UG), AU), 2.5), ((AU, (CG, UU), AU), 1.7),
      ((AU, (CU, AA), AU), 2.6), ((AU, (CU, AC), AU), 2.1), ((AU, (CU, AG), AU), 2.1), ((AU, (CU, AU), AU), 2.1), ((AU, (CU, CA), AU), 2.6), ((AU, (CU, CC), AU), 2.6), ((AU, (CU, CG), AU), 2.6), ((AU, (CU, CU), AU), 2.6), ((AU, (CU, GA), AU), 2.2), ((AU, (CU, GC), AU), 2.1), ((AU, (CU, GG), AU), 1.0), ((AU, (CU, GU), AU), 2.1), ((AU, (CU, UA), AU), 2.6), ((AU, (CU, UC), AU), 2.3), ((AU, (CU, UG), AU), 2.6), ((AU, (CU, UU), AU), 1.7),
      ((AU, (GA, AA), AU), 1.5), ((AU, (GA, AC), AU), 1.1), ((AU, (GA, AG), AU), 0.5), ((AU, (GA, AU), AU), 1.1), ((AU, (GA, CA), AU), 1.5), ((AU, (GA, CC), AU), 2.1), ((AU, (GA, CG), AU), 1.5), ((AU, (GA, CU), AU), 2.1), ((AU, (GA, GA), AU), 0.6), ((AU, (GA, GC), AU), 1.1), ((AU, (GA, GG), AU), 1.3), ((AU, (GA, GU), AU), 1.1), ((AU, (GA, UA), AU), 1.5), ((AU, (GA, UC), AU), 1.8), ((AU, (GA, UG), AU), 1.5), ((AU, (GA, UU), AU), 0.7),
      ((AU, (GC, AA), AU), 2.6), ((AU, (GC, AC), AU), 2.2), ((AU, (GC, AG), AU), 1.6), ((AU, (GC, AU), AU), 2.2), ((AU, (GC, CA), AU), 2.6), ((AU, (GC, CC), AU), 2.6), ((AU, (GC, CG), AU), 2.6), ((AU, (GC, CU), AU), 2.6), ((AU, (GC, GA), AU), 1.7), ((AU, (GC, GC), AU), 2.2), ((AU, (GC, GG), AU), 1.1), ((AU, (GC, GU), AU), 2.2), ((AU, (GC, UA), AU), 2.6), ((AU, (GC, UC), AU), 2.3), ((AU, (GC, UG), AU), 2.6), ((AU, (GC, UU), AU), 1.8),
      ((AU, (GG, AA), AU), 1.0), ((AU, (GG, AC), AU), 0.6), ((AU, (GG, AG), AU), 1.3), ((AU, (GG, AU), AU), 0.6), ((AU, (GG, CA), AU), 1.0), ((AU, (GG, CC), AU), 1.0), ((AU, (GG, CG), AU), 1.0), ((AU, (GG, CU), AU), 1.0), ((AU, (GG, GA), AU), 1.4), ((AU, (GG, GC), AU), 0.6), ((AU, (GG, GG), AU), 2.1), ((AU, (GG, GU), AU), 0.6), ((AU, (GG, UA), AU), 1.0), ((AU, (GG, UC), AU), 0.7), ((AU, (GG, UG), AU), 1.0), ((AU, (GG, UU), AU), 1.5),
      ((AU, (GU, AA), AU), 2.6), ((AU, (GU, AC), AU), 2.2), ((AU, (GU, AG), AU), 1.6), ((AU, (GU, AU), AU), 2.2), ((AU, (GU, CA), AU), 2.6), ((AU, (GU, CC), AU), 2.6), ((AU, (GU, CG), AU), 2.6), ((AU, (GU, CU), AU), 2.6), ((AU, (GU, GA), AU), 1.7), ((AU, (GU, GC), AU), 2.2), ((AU, (GU, GG), AU), 1.1), ((AU, (GU, GU), AU), 2.2), ((AU, (GU, UA), AU), 2.6), ((AU, (GU, UC), AU), 2.3), ((AU, (GU, UG), AU), 2.6), ((AU, (GU, UU), AU), 1.8),
      ((AU, (UA, AA), AU), 2.5), ((AU, (UA, AC), AU), 2.1), ((AU, (UA, AG), AU), 1.5), ((AU, (UA, AU), AU), 2.1), ((AU, (UA, CA), AU), 2.5), ((AU, (UA, CC), AU), 2.5), ((AU, (UA, CG), AU), 2.5), ((AU, (UA, CU), AU), 2.5), ((AU, (UA, GA), AU), 1.6), ((AU, (UA, GC), AU), 2.1), ((AU, (UA, GG), AU), 1.0), ((AU, (UA, GU), AU), 2.1), ((AU, (UA, UA), AU), 2.5), ((AU, (UA, UC), AU), 2.2), ((AU, (UA, UG), AU), 2.5), ((AU, (UA, UU), AU), 1.7),
      ((AU, (UC, AA), AU), 2.6), ((AU, (UC, AC), AU), 2.1), ((AU, (UC, AG), AU), 2.1), ((AU, (UC, AU), AU), 2.1), ((AU, (UC, CA), AU), 2.6), ((AU, (UC, CC), AU), 2.6), ((AU, (UC, CG), AU), 2.6), ((AU, (UC, CU), AU), 2.6), ((AU, (UC, GA), AU), 2.2), ((AU, (UC, GC), AU), 2.1), ((AU, (UC, GG), AU), 1.0), ((AU, (UC, GU), AU), 2.1), ((AU, (UC, UA), AU), 2.6), ((AU, (UC, UC), AU), 2.3), ((AU, (UC, UG), AU), 2.6), ((AU, (UC, UU), AU), 1.7),
      ((AU, (UG, AA), AU), 2.5), ((AU, (UG, AC), AU), 2.1), ((AU, (UG, AG), AU), 1.5), ((AU, (UG, AU), AU), 2.1), ((AU, (UG, CA), AU), 2.5), ((AU, (UG, CC), AU), 2.5), ((AU, (UG, CG), AU), 2.5), ((AU, (UG, CU), AU), 2.5), ((AU, (UG, GA), AU), 1.6), ((AU, (UG, GC), AU), 2.1), ((AU, (UG, GG), AU), 1.0), ((AU, (UG, GU), AU), 2.1), ((AU, (UG, UA), AU), 2.5), ((AU, (UG, UC), AU), 2.2), ((AU, (UG, UG), AU), 2.5), ((AU, (UG, UU), AU), 1.7),
      ((AU, (UU, AA), AU), 2.3), ((AU, (UU, AC), AU), 1.2), ((AU, (UU, AG), AU), 0.6), ((AU, (UU, AU), AU), 1.2), ((AU, (UU, CA), AU), 1.7), ((AU, (UU, CC), AU), 1.7), ((AU, (UU, CG), AU), 1.7), ((AU, (UU, CU), AU), 1.7), ((AU, (UU, GA), AU), 0.7), ((AU, (UU, GC), AU), 1.2), ((AU, (UU, GG), AU), 1.4), ((AU, (UU, GU), AU), 1.2), ((AU, (UU, UA), AU), 1.7), ((AU, (UU, UC), AU), 1.4), ((AU, (UU, UG), AU), 1.7), ((AU, (UU, UU), AU), 0.8),
      // For internal loops between the base pairs "AU" and "CG".
      ((AU, (AA, AA), CG), 2.1), ((AU, (AA, AC), CG), 1.9), ((AU, (AA, AG), CG), 0.1), ((AU, (AA, CG), AU), 1.9), ((AU, (AA, CA), CG), 1.8), ((AU, (AA, CC), CG), 1.9), ((AU, (AA, CG), CG), 1.8), ((AU, (AA, CU), CG), 1.9), ((AU, (AA, GA), CG), 0.7), ((AU, (AA, GC), CG), 1.9), ((AU, (AA, GG), CG), 0.5), ((AU, (AA, GU), CG), 1.9), ((AU, (AA, UA), CG), 1.8), ((AU, (AA, UC), CG), 1.9), ((AU, (AA, UG), CG), 1.8), ((AU, (AA, UU), CG), 1.7),
      ((AU, (AC, AA), CG), 2.0), ((AU, (AC, AC), CG), 1.7), ((AU, (AC, AG), CG), 0.0), ((AU, (AC, CG), AU), 1.7), ((AU, (AC, CA), CG), 1.7), ((AU, (AC, CC), CG), 1.7), ((AU, (AC, CG), CG), 1.7), ((AU, (AC, CU), CG), 1.7), ((AU, (AC, GA), CG), 0.6), ((AU, (AC, GC), CG), 1.7), ((AU, (AC, GG), CG), 0.3), ((AU, (AC, GU), CG), 1.7), ((AU, (AC, UA), CG), 1.7), ((AU, (AC, UC), CG), 1.8), ((AU, (AC, UG), CG), 1.7), ((AU, (AC, UU), CG), 1.0),
      ((AU, (AG, AA), CG), 1.6), ((AU, (AG, AC), CG), 1.3), ((AU, (AG, AG), CG), -0.4), ((AU, (AG, CG), AU), 1.3), ((AU, (AG, CA), CG), 1.3), ((AU, (AG, CC), CG), 1.9), ((AU, (AG, CG), CG), 1.3), ((AU, (AG, CU), CG), 1.9), ((AU, (AG, GA), CG), 0.2), ((AU, (AG, GC), CG), 1.3), ((AU, (AG, GG), CG), 1.2), ((AU, (AG, GU), CG), 1.3), ((AU, (AG, UA), CG), 1.3), ((AU, (AG, UC), CG), 2.0), ((AU, (AG, UG), CG), 1.3), ((AU, (AG, UU), CG), 0.6),
      ((AU, (AU, AA), CG), 2.0), ((AU, (AU, AC), CG), 1.7), ((AU, (AU, AG), CG), 0.0), ((AU, (AU, CG), AU), 1.7), ((AU, (AU, CA), CG), 1.7), ((AU, (AU, CC), CG), 1.7), ((AU, (AU, CG), CG), 1.7), ((AU, (AU, CU), CG), 1.7), ((AU, (AU, GA), CG), 0.6), ((AU, (AU, GC), CG), 1.7), ((AU, (AU, GG), CG), 0.3), ((AU, (AU, GU), CG), 1.7), ((AU, (AU, UA), CG), 1.7), ((AU, (AU, UC), CG), 1.8), ((AU, (AU, UG), CG), 1.7), ((AU, (AU, UU), CG), 1.0),
      ((AU, (CA, AA), CG), 1.9), ((AU, (CA, AC), CG), 1.6), ((AU, (CA, AG), CG), -0.1), ((AU, (CA, CG), AU), 1.6), ((AU, (CA, CA), CG), 1.6), ((AU, (CA, CC), CG), 1.6), ((AU, (CA, CG), CG), 1.6), ((AU, (CA, CU), CG), 1.6), ((AU, (CA, GA), CG), 0.5), ((AU, (CA, GC), CG), 1.6), ((AU, (CA, GG), CG), 0.2), ((AU, (CA, GU), CG), 1.6), ((AU, (CA, UA), CG), 1.6), ((AU, (CA, UC), CG), 1.7), ((AU, (CA, UG), CG), 1.6), ((AU, (CA, UU), CG), 0.9),
      ((AU, (CC, AA), CG), 1.9), ((AU, (CC, AC), CG), 1.7), ((AU, (CC, AG), CG), 0.5), ((AU, (CC, CG), AU), 1.7), ((AU, (CC, CA), CG), 1.6), ((AU, (CC, CC), CG), 1.7), ((AU, (CC, CG), CG), 1.6), ((AU, (CC, CU), CG), 1.7), ((AU, (CC, GA), CG), 1.1), ((AU, (CC, GC), CG), 1.7), ((AU, (CC, GG), CG), 0.3), ((AU, (CC, GU), CG), 1.7), ((AU, (CC, UA), CG), 1.6), ((AU, (CC, UC), CG), 1.7), ((AU, (CC, UG), CG), 1.6), ((AU, (CC, UU), CG), 0.9),
      ((AU, (CG, AA), CG), 1.9), ((AU, (CG, AC), CG), 1.6), ((AU, (CG, AG), CG), -0.1), ((AU, (CG, CG), AU), 1.6), ((AU, (CG, CA), CG), 1.6), ((AU, (CG, CC), CG), 1.6), ((AU, (CG, CG), CG), 1.6), ((AU, (CG, CU), CG), 1.6), ((AU, (CG, GA), CG), 0.5), ((AU, (CG, GC), CG), 1.6), ((AU, (CG, GG), CG), 0.2), ((AU, (CG, GU), CG), 1.6), ((AU, (CG, UA), CG), 1.6), ((AU, (CG, UC), CG), 1.7), ((AU, (CG, UG), CG), 1.6), ((AU, (CG, UU), CG), 0.9),
      ((AU, (CU, AA), CG), 1.9), ((AU, (CU, AC), CG), 1.7), ((AU, (CU, AG), CG), 0.5), ((AU, (CU, CG), AU), 1.7), ((AU, (CU, CA), CG), 1.6), ((AU, (CU, CC), CG), 1.7), ((AU, (CU, CG), CG), 1.6), ((AU, (CU, CU), CG), 1.7), ((AU, (CU, GA), CG), 1.1), ((AU, (CU, GC), CG), 1.7), ((AU, (CU, GG), CG), 0.3), ((AU, (CU, GU), CG), 1.7), ((AU, (CU, UA), CG), 1.6), ((AU, (CU, UC), CG), 1.7), ((AU, (CU, UG), CG), 1.6), ((AU, (CU, UU), CG), 0.9),
      ((AU, (GA, AA), CG), 0.9), ((AU, (GA, AC), CG), 0.6), ((AU, (GA, AG), CG), -1.1), ((AU, (GA, CG), AU), 0.6), ((AU, (GA, CA), CG), 0.6), ((AU, (GA, CC), CG), 1.2), ((AU, (GA, CG), CG), 0.6), ((AU, (GA, CU), CG), 1.2), ((AU, (GA, GA), CG), -0.5), ((AU, (GA, GC), CG), 0.6), ((AU, (GA, GG), CG), 0.5), ((AU, (GA, GU), CG), 0.6), ((AU, (GA, UA), CG), 0.6), ((AU, (GA, UC), CG), 1.3), ((AU, (GA, UG), CG), 0.6), ((AU, (GA, UU), CG), -0.1),
      ((AU, (GC, AA), CG), 2.0), ((AU, (GC, AC), CG), 1.7), ((AU, (GC, AG), CG), 0.0), ((AU, (GC, CG), AU), 1.7), ((AU, (GC, CA), CG), 1.7), ((AU, (GC, CC), CG), 1.7), ((AU, (GC, CG), CG), 1.7), ((AU, (GC, CU), CG), 1.7), ((AU, (GC, GA), CG), 0.6), ((AU, (GC, GC), CG), 1.7), ((AU, (GC, GG), CG), 0.3), ((AU, (GC, GU), CG), 1.7), ((AU, (GC, UA), CG), 1.7), ((AU, (GC, UC), CG), 1.8), ((AU, (GC, UG), CG), 1.7), ((AU, (GC, UU), CG), 1.0),
      ((AU, (GG, AA), CG), 0.4), ((AU, (GG, AC), CG), 0.1), ((AU, (GG, AG), CG), -0.3), ((AU, (GG, CG), AU), 0.1), ((AU, (GG, CA), CG), 0.1), ((AU, (GG, CC), CG), 0.1), ((AU, (GG, CG), CG), 0.1), ((AU, (GG, CU), CG), 0.1), ((AU, (GG, GA), CG), 0.3), ((AU, (GG, GC), CG), 0.1), ((AU, (GG, GG), CG), 1.3), ((AU, (GG, GU), CG), 0.1), ((AU, (GG, UA), CG), 0.1), ((AU, (GG, UC), CG), 0.2), ((AU, (GG, UG), CG), 0.1), ((AU, (GG, UU), CG), 0.7),
      ((AU, (GU, AA), CG), 2.0), ((AU, (GU, AC), CG), 1.7), ((AU, (GU, AG), CG), 0.0), ((AU, (GU, CG), AU), 1.7), ((AU, (GU, CA), CG), 1.7), ((AU, (GU, CC), CG), 1.7), ((AU, (GU, CG), CG), 1.7), ((AU, (GU, CU), CG), 1.7), ((AU, (GU, GA), CG), 0.6), ((AU, (GU, GC), CG), 1.7), ((AU, (GU, GG), CG), 0.3), ((AU, (GU, GU), CG), 1.7), ((AU, (GU, UA), CG), 1.7), ((AU, (GU, UC), CG), 1.8), ((AU, (GU, UG), CG), 1.7), ((AU, (GU, UU), CG), 1.0),
      ((AU, (UA, AA), CG), 1.9), ((AU, (UA, AC), CG), 1.6), ((AU, (UA, AG), CG), -0.1), ((AU, (UA, CG), AU), 1.6), ((AU, (UA, CA), CG), 1.6), ((AU, (UA, CC), CG), 1.6), ((AU, (UA, CG), CG), 1.6), ((AU, (UA, CU), CG), 1.6), ((AU, (UA, GA), CG), 0.5), ((AU, (UA, GC), CG), 1.6), ((AU, (UA, GG), CG), 0.2), ((AU, (UA, GU), CG), 1.6), ((AU, (UA, UA), CG), 1.6), ((AU, (UA, UC), CG), 1.7), ((AU, (UA, UG), CG), 1.6), ((AU, (UA, UU), CG), 0.9),
      ((AU, (UC, AA), CG), 1.9), ((AU, (UC, AC), CG), 1.7), ((AU, (UC, AG), CG), 0.5), ((AU, (UC, CG), AU), 1.7), ((AU, (UC, CA), CG), 1.6), ((AU, (UC, CC), CG), 1.7), ((AU, (UC, CG), CG), 1.6), ((AU, (UC, CU), CG), 1.7), ((AU, (UC, GA), CG), 1.1), ((AU, (UC, GC), CG), 1.7), ((AU, (UC, GG), CG), 0.3), ((AU, (UC, GU), CG), 1.7), ((AU, (UC, UA), CG), 1.6), ((AU, (UC, UC), CG), 1.7), ((AU, (UC, UG), CG), 1.6), ((AU, (UC, UU), CG), 0.9),
      ((AU, (UG, AA), CG), 1.9), ((AU, (UG, AC), CG), 1.6), ((AU, (UG, AG), CG), -0.1), ((AU, (UG, CG), AU), 1.6), ((AU, (UG, CA), CG), 1.6), ((AU, (UG, CC), CG), 1.6), ((AU, (UG, CG), CG), 1.6), ((AU, (UG, CU), CG), 1.6), ((AU, (UG, GA), CG), 0.5), ((AU, (UG, GC), CG), 1.6), ((AU, (UG, GG), CG), 0.2), ((AU, (UG, GU), CG), 1.6), ((AU, (UG, UA), CG), 1.6), ((AU, (UG, UC), CG), 1.7), ((AU, (UG, UG), CG), 1.6), ((AU, (UG, UU), CG), 0.9),
      ((AU, (UU, AA), CG), 1.6), ((AU, (UU, AC), CG), 0.8), ((AU, (UU, AG), CG), -1.0), ((AU, (UU, CG), AU), 0.8), ((AU, (UU, CA), CG), 0.7), ((AU, (UU, CC), CG), 0.8), ((AU, (UU, CG), CG), 0.7), ((AU, (UU, CU), CG), 0.8), ((AU, (UU, GA), CG), -0.3), ((AU, (UU, GC), CG), 0.8), ((AU, (UU, GG), CG), 0.7), ((AU, (UU, GU), CG), 0.8), ((AU, (UU, UA), CG), 0.7), ((AU, (UU, UC), CG), 0.8), ((AU, (UU, UG), CG), 0.7), ((AU, (UU, UU), CG), 0.0),
      // For internal loops between the base pairs "AU" and "GC".
      ((AU, (AA, AA), GC), 2.0), ((AU, (AA, AC), GC), 1.9), ((AU, (AA, AG), GC), 1.0), ((AU, (AA, GC), AU), 1.9), ((AU, (AA, CA), GC), 2.4), ((AU, (AA, CC), GC), 2.2), ((AU, (AA, GC), CG), 2.4), ((AU, (AA, CU), GC), 2.1), ((AU, (AA, GA), GC), 1.0), ((AU, (AA, GC), GC), 1.9), ((AU, (AA, GG), GC), 0.5), ((AU, (AA, GU), GC), 1.9), ((AU, (AA, UA), GC), 2.4), ((AU, (AA, UC), GC), 2.1), ((AU, (AA, UG), GC), 2.4), ((AU, (AA, UU), GC), 1.8),
      ((AU, (AC, AA), GC), 1.8), ((AU, (AC, AC), GC), 1.8), ((AU, (AC, AG), GC), 0.9), ((AU, (AC, GC), AU), 1.8), ((AU, (AC, CA), GC), 2.2), ((AU, (AC, CC), GC), 2.1), ((AU, (AC, GC), CG), 2.2), ((AU, (AC, CU), GC), 1.9), ((AU, (AC, GA), GC), 0.9), ((AU, (AC, GC), GC), 1.8), ((AU, (AC, GG), GC), 0.3), ((AU, (AC, GU), GC), 1.8), ((AU, (AC, UA), GC), 2.2), ((AU, (AC, UC), GC), 1.9), ((AU, (AC, UG), GC), 2.2), ((AU, (AC, UU), GC), 1.0),
      ((AU, (AG, AA), GC), 1.4), ((AU, (AG, AC), GC), 1.4), ((AU, (AG, AG), GC), 0.5), ((AU, (AG, GC), AU), 1.4), ((AU, (AG, CA), GC), 1.8), ((AU, (AG, CC), GC), 2.3), ((AU, (AG, GC), CG), 1.8), ((AU, (AG, CU), GC), 2.1), ((AU, (AG, GA), GC), 0.5), ((AU, (AG, GC), GC), 1.4), ((AU, (AG, GG), GC), 1.2), ((AU, (AG, GU), GC), 1.4), ((AU, (AG, UA), GC), 1.8), ((AU, (AG, UC), GC), 2.1), ((AU, (AG, UG), GC), 1.8), ((AU, (AG, UU), GC), 0.6),
      ((AU, (AU, AA), GC), 1.8), ((AU, (AU, AC), GC), 1.8), ((AU, (AU, AG), GC), 0.9), ((AU, (AU, GC), AU), 1.8), ((AU, (AU, CA), GC), 2.2), ((AU, (AU, CC), GC), 2.1), ((AU, (AU, GC), CG), 2.2), ((AU, (AU, CU), GC), 1.9), ((AU, (AU, GA), GC), 0.9), ((AU, (AU, GC), GC), 1.8), ((AU, (AU, GG), GC), 0.3), ((AU, (AU, GU), GC), 1.8), ((AU, (AU, UA), GC), 2.2), ((AU, (AU, UC), GC), 1.9), ((AU, (AU, UG), GC), 2.2), ((AU, (AU, UU), GC), 1.0),
      ((AU, (CA, AA), GC), 1.7), ((AU, (CA, AC), GC), 1.7), ((AU, (CA, AG), GC), 0.8), ((AU, (CA, GC), AU), 1.7), ((AU, (CA, CA), GC), 2.1), ((AU, (CA, CC), GC), 2.0), ((AU, (CA, GC), CG), 2.1), ((AU, (CA, CU), GC), 1.8), ((AU, (CA, GA), GC), 0.8), ((AU, (CA, GC), GC), 1.7), ((AU, (CA, GG), GC), 0.2), ((AU, (CA, GU), GC), 1.7), ((AU, (CA, UA), GC), 2.1), ((AU, (CA, UC), GC), 1.8), ((AU, (CA, UG), GC), 2.1), ((AU, (CA, UU), GC), 0.9),
      ((AU, (CC, AA), GC), 1.8), ((AU, (CC, AC), GC), 1.7), ((AU, (CC, AG), GC), 1.4), ((AU, (CC, GC), AU), 1.7), ((AU, (CC, CA), GC), 2.2), ((AU, (CC, CC), GC), 2.0), ((AU, (CC, GC), CG), 2.2), ((AU, (CC, CU), GC), 1.9), ((AU, (CC, GA), GC), 1.4), ((AU, (CC, GC), GC), 1.7), ((AU, (CC, GG), GC), 0.3), ((AU, (CC, GU), GC), 1.7), ((AU, (CC, UA), GC), 2.2), ((AU, (CC, UC), GC), 1.9), ((AU, (CC, UG), GC), 2.2), ((AU, (CC, UU), GC), 1.0),
      ((AU, (CG, AA), GC), 1.7), ((AU, (CG, AC), GC), 1.7), ((AU, (CG, AG), GC), 0.8), ((AU, (CG, GC), AU), 1.7), ((AU, (CG, CA), GC), 2.1), ((AU, (CG, CC), GC), 2.0), ((AU, (CG, GC), CG), 2.1), ((AU, (CG, CU), GC), 1.8), ((AU, (CG, GA), GC), 0.8), ((AU, (CG, GC), GC), 1.7), ((AU, (CG, GG), GC), 0.2), ((AU, (CG, GU), GC), 1.7), ((AU, (CG, UA), GC), 2.1), ((AU, (CG, UC), GC), 1.8), ((AU, (CG, UG), GC), 2.1), ((AU, (CG, UU), GC), 0.9),
      ((AU, (CU, AA), GC), 1.8), ((AU, (CU, AC), GC), 1.7), ((AU, (CU, AG), GC), 1.4), ((AU, (CU, GC), AU), 1.7), ((AU, (CU, CA), GC), 2.2), ((AU, (CU, CC), GC), 2.0), ((AU, (CU, GC), CG), 2.2), ((AU, (CU, CU), GC), 1.9), ((AU, (CU, GA), GC), 1.4), ((AU, (CU, GC), GC), 1.7), ((AU, (CU, GG), GC), 0.3), ((AU, (CU, GU), GC), 1.7), ((AU, (CU, UA), GC), 2.2), ((AU, (CU, UC), GC), 1.9), ((AU, (CU, UG), GC), 2.2), ((AU, (CU, UU), GC), 1.0),
      ((AU, (GA, AA), GC), 0.7), ((AU, (GA, AC), GC), 0.7), ((AU, (GA, AG), GC), -0.2), ((AU, (GA, GC), AU), 0.7), ((AU, (GA, CA), GC), 1.1), ((AU, (GA, CC), GC), 1.6), ((AU, (GA, GC), CG), 1.1), ((AU, (GA, CU), GC), 1.4), ((AU, (GA, GA), GC), -0.2), ((AU, (GA, GC), GC), 0.7), ((AU, (GA, GG), GC), 0.5), ((AU, (GA, GU), GC), 0.7), ((AU, (GA, UA), GC), 1.1), ((AU, (GA, UC), GC), 1.4), ((AU, (GA, UG), GC), 1.1), ((AU, (GA, UU), GC), 0.0),
      ((AU, (GC, AA), GC), 1.8), ((AU, (GC, AC), GC), 1.8), ((AU, (GC, AG), GC), 0.9), ((AU, (GC, GC), AU), 1.8), ((AU, (GC, CA), GC), 2.2), ((AU, (GC, CC), GC), 2.1), ((AU, (GC, GC), CG), 2.2), ((AU, (GC, CU), GC), 1.9), ((AU, (GC, GA), GC), 0.9), ((AU, (GC, GC), GC), 1.8), ((AU, (GC, GG), GC), 0.3), ((AU, (GC, GU), GC), 1.8), ((AU, (GC, UA), GC), 2.2), ((AU, (GC, UC), GC), 1.9), ((AU, (GC, UG), GC), 2.2), ((AU, (GC, UU), GC), 1.0),
      ((AU, (GG, AA), GC), 0.2), ((AU, (GG, AC), GC), 0.2), ((AU, (GG, AG), GC), 0.6), ((AU, (GG, GC), AU), 0.2), ((AU, (GG, CA), GC), 0.6), ((AU, (GG, CC), GC), 0.5), ((AU, (GG, GC), CG), 0.6), ((AU, (GG, CU), GC), 0.3), ((AU, (GG, GA), GC), 0.6), ((AU, (GG, GC), GC), 0.2), ((AU, (GG, GG), GC), 1.3), ((AU, (GG, GU), GC), 0.2), ((AU, (GG, UA), GC), 0.6), ((AU, (GG, UC), GC), 0.3), ((AU, (GG, UG), GC), 0.6), ((AU, (GG, UU), GC), 0.7),
      ((AU, (GU, AA), GC), 1.8), ((AU, (GU, AC), GC), 1.8), ((AU, (GU, AG), GC), 0.9), ((AU, (GU, GC), AU), 1.8), ((AU, (GU, CA), GC), 2.2), ((AU, (GU, CC), GC), 2.1), ((AU, (GU, GC), CG), 2.2), ((AU, (GU, CU), GC), 1.9), ((AU, (GU, GA), GC), 0.9), ((AU, (GU, GC), GC), 1.8), ((AU, (GU, GG), GC), 0.3), ((AU, (GU, GU), GC), 1.8), ((AU, (GU, UA), GC), 2.2), ((AU, (GU, UC), GC), 1.9), ((AU, (GU, UG), GC), 2.2), ((AU, (GU, UU), GC), 1.0),
      ((AU, (UA, AA), GC), 1.7), ((AU, (UA, AC), GC), 1.7), ((AU, (UA, AG), GC), 0.8), ((AU, (UA, GC), AU), 1.7), ((AU, (UA, CA), GC), 2.1), ((AU, (UA, CC), GC), 2.0), ((AU, (UA, GC), CG), 2.1), ((AU, (UA, CU), GC), 1.8), ((AU, (UA, GA), GC), 0.8), ((AU, (UA, GC), GC), 1.7), ((AU, (UA, GG), GC), 0.2), ((AU, (UA, GU), GC), 1.7), ((AU, (UA, UA), GC), 2.1), ((AU, (UA, UC), GC), 1.8), ((AU, (UA, UG), GC), 2.1), ((AU, (UA, UU), GC), 0.9),
      ((AU, (UC, AA), GC), 1.8), ((AU, (UC, AC), GC), 1.7), ((AU, (UC, AG), GC), 1.4), ((AU, (UC, GC), AU), 1.7), ((AU, (UC, CA), GC), 2.2), ((AU, (UC, CC), GC), 2.0), ((AU, (UC, GC), CG), 2.2), ((AU, (UC, CU), GC), 1.9), ((AU, (UC, GA), GC), 1.4), ((AU, (UC, GC), GC), 1.7), ((AU, (UC, GG), GC), 0.3), ((AU, (UC, GU), GC), 1.7), ((AU, (UC, UA), GC), 2.2), ((AU, (UC, UC), GC), 1.9), ((AU, (UC, UG), GC), 2.2), ((AU, (UC, UU), GC), 1.0),
      ((AU, (UG, AA), GC), 1.7), ((AU, (UG, AC), GC), 1.7), ((AU, (UG, AG), GC), 0.8), ((AU, (UG, GC), AU), 1.7), ((AU, (UG, CA), GC), 2.1), ((AU, (UG, CC), GC), 2.0), ((AU, (UG, GC), CG), 2.1), ((AU, (UG, CU), GC), 1.8), ((AU, (UG, GA), GC), 0.8), ((AU, (UG, GC), GC), 1.7), ((AU, (UG, GG), GC), 0.2), ((AU, (UG, GU), GC), 1.7), ((AU, (UG, UA), GC), 2.1), ((AU, (UG, UC), GC), 1.8), ((AU, (UG, UG), GC), 2.1), ((AU, (UG, UU), GC), 0.9),
      ((AU, (UU, AA), GC), 1.5), ((AU, (UU, AC), GC), 0.8), ((AU, (UU, AG), GC), 0.0), ((AU, (UU, GC), AU), 0.8), ((AU, (UU, CA), GC), 1.3), ((AU, (UU, CC), GC), 1.1), ((AU, (UU, GC), CG), 1.3), ((AU, (UU, CU), GC), 1.0), ((AU, (UU, GA), GC), 0.0), ((AU, (UU, GC), GC), 0.8), ((AU, (UU, GG), GC), 0.7), ((AU, (UU, GU), GC), 0.8), ((AU, (UU, UA), GC), 1.3), ((AU, (UU, UC), GC), 1.0), ((AU, (UU, UG), GC), 1.3), ((AU, (UU, UU), GC), 0.1),
      // For internal loops between the base pairs "AU" and "GU".
      ((AU, (AA, AA), GU), 2.4), ((AU, (AA, AC), GU), 2.8), ((AU, (AA, AG), GU), 1.4), ((AU, (AA, GU), AU), 2.8), ((AU, (AA, CA), GU), 2.8), ((AU, (AA, CC), GU), 2.8), ((AU, (AA, GU), CG), 2.8), ((AU, (AA, CU), GU), 2.8), ((AU, (AA, GA), GU), 3.1), ((AU, (AA, GU), GC), 2.8), ((AU, (AA, GG), GU), 1.5), ((AU, (AA, GU), GU), 2.8), ((AU, (AA, UA), GU), 2.8), ((AU, (AA, UC), GU), 2.8), ((AU, (AA, UG), GU), 2.8), ((AU, (AA, UU), GU), 3.4),
      ((AU, (AC, AA), GU), 2.3), ((AU, (AC, AC), GU), 2.6), ((AU, (AC, AG), GU), 1.3), ((AU, (AC, GU), AU), 2.6), ((AU, (AC, CA), GU), 2.6), ((AU, (AC, CC), GU), 2.6), ((AU, (AC, GU), CG), 2.6), ((AU, (AC, CU), GU), 2.6), ((AU, (AC, GA), GU), 2.9), ((AU, (AC, GU), GC), 2.6), ((AU, (AC, GG), GU), 1.3), ((AU, (AC, GU), GU), 2.6), ((AU, (AC, UA), GU), 2.6), ((AU, (AC, UC), GU), 2.6), ((AU, (AC, UG), GU), 2.6), ((AU, (AC, UU), GU), 2.6),
      ((AU, (AG, AA), GU), 1.9), ((AU, (AG, AC), GU), 2.2), ((AU, (AG, AG), GU), 0.9), ((AU, (AG, GU), AU), 2.2), ((AU, (AG, CA), GU), 2.2), ((AU, (AG, CC), GU), 2.8), ((AU, (AG, GU), CG), 2.2), ((AU, (AG, CU), GU), 2.8), ((AU, (AG, GA), GU), 2.5), ((AU, (AG, GU), GC), 2.2), ((AU, (AG, GG), GU), 2.2), ((AU, (AG, GU), GU), 2.2), ((AU, (AG, UA), GU), 2.2), ((AU, (AG, UC), GU), 2.8), ((AU, (AG, UG), GU), 2.2), ((AU, (AG, UU), GU), 2.2),
      ((AU, (AU, AA), GU), 2.3), ((AU, (AU, AC), GU), 2.6), ((AU, (AU, AG), GU), 1.3), ((AU, (AU, GU), AU), 2.6), ((AU, (AU, CA), GU), 2.6), ((AU, (AU, CC), GU), 2.6), ((AU, (AU, GU), CG), 2.6), ((AU, (AU, CU), GU), 2.6), ((AU, (AU, GA), GU), 2.6), ((AU, (AU, GU), GC), 2.9), ((AU, (AU, GG), GU), 2.6), ((AU, (AU, GU), GU), 1.3), ((AU, (AU, UA), GU), 2.6), ((AU, (AU, UC), GU), 2.6), ((AU, (AU, UG), GU), 2.6), ((AU, (AU, UU), GU), 2.6),
      ((AU, (CA, AA), GU), 2.2), ((AU, (CA, AC), GU), 2.5), ((AU, (CA, AG), GU), 1.2), ((AU, (CA, GU), AU), 2.5), ((AU, (CA, CA), GU), 2.5), ((AU, (CA, CC), GU), 2.5), ((AU, (CA, GU), CG), 2.5), ((AU, (CA, CU), GU), 2.5), ((AU, (CA, GA), GU), 2.8), ((AU, (CA, GU), GC), 2.5), ((AU, (CA, GG), GU), 1.2), ((AU, (CA, GU), GU), 2.5), ((AU, (CA, UA), GU), 2.5), ((AU, (CA, UC), GU), 2.5), ((AU, (CA, UG), GU), 2.5), ((AU, (CA, UU), GU), 2.5),
      ((AU, (CC, AA), GU), 2.2), ((AU, (CC, AC), GU), 2.6), ((AU, (CC, AG), GU), 1.8), ((AU, (CC, GU), AU), 2.6), ((AU, (CC, CA), GU), 2.6), ((AU, (CC, CC), GU), 2.6), ((AU, (CC, GU), CG), 2.6), ((AU, (CC, CU), GU), 2.6), ((AU, (CC, GA), GU), 3.5), ((AU, (CC, GU), GC), 2.6), ((AU, (CC, GG), GU), 1.3), ((AU, (CC, GU), GU), 2.6), ((AU, (CC, UA), GU), 2.6), ((AU, (CC, UC), GU), 2.6), ((AU, (CC, UG), GU), 2.6), ((AU, (CC, UU), GU), 2.6),
      ((AU, (CG, AA), GU), 2.2), ((AU, (CG, AC), GU), 2.5), ((AU, (CG, AG), GU), 1.2), ((AU, (CG, GU), AU), 2.5), ((AU, (CG, CA), GU), 2.5), ((AU, (CG, CC), GU), 2.5), ((AU, (CG, GU), CG), 2.5), ((AU, (CG, CU), GU), 2.5), ((AU, (CG, GA), GU), 2.8), ((AU, (CG, GU), GC), 2.5), ((AU, (CG, GG), GU), 1.2), ((AU, (CG, GU), GU), 2.5), ((AU, (CG, UA), GU), 2.5), ((AU, (CG, UC), GU), 2.5), ((AU, (CG, UG), GU), 2.5), ((AU, (CG, UU), GU), 2.5),
      ((AU, (CU, AA), GU), 2.2), ((AU, (CU, AC), GU), 2.6), ((AU, (CU, AG), GU), 1.8), ((AU, (CU, GU), AU), 2.6), ((AU, (CU, CA), GU), 2.6), ((AU, (CU, CC), GU), 2.6), ((AU, (CU, GU), CG), 2.6), ((AU, (CU, CU), GU), 2.6), ((AU, (CU, GA), GU), 3.5), ((AU, (CU, GU), GC), 2.6), ((AU, (CU, GG), GU), 1.3), ((AU, (CU, GU), GU), 2.6), ((AU, (CU, UA), GU), 2.6), ((AU, (CU, UC), GU), 2.6), ((AU, (CU, UG), GU), 2.6), ((AU, (CU, UU), GU), 2.6),
      ((AU, (GA, AA), GU), 1.2), ((AU, (GA, AC), GU), 1.5), ((AU, (GA, AG), GU), 0.2), ((AU, (GA, GU), AU), 1.5), ((AU, (GA, CA), GU), 1.5), ((AU, (GA, CC), GU), 2.1), ((AU, (GA, GU), CG), 1.5), ((AU, (GA, CU), GU), 2.1), ((AU, (GA, GA), GU), 1.8), ((AU, (GA, GU), GC), 1.5), ((AU, (GA, GG), GU), 1.5), ((AU, (GA, GU), GU), 1.5), ((AU, (GA, UA), GU), 1.5), ((AU, (GA, UC), GU), 2.1), ((AU, (GA, UG), GU), 1.5), ((AU, (GA, UU), GU), 1.5),
      ((AU, (GC, AA), GU), 2.3), ((AU, (GC, AC), GU), 2.6), ((AU, (GC, AG), GU), 1.3), ((AU, (GC, GU), AU), 2.6), ((AU, (GC, CA), GU), 2.6), ((AU, (GC, CC), GU), 2.6), ((AU, (GC, GU), CG), 2.6), ((AU, (GC, CU), GU), 2.6), ((AU, (GC, GA), GU), 2.9), ((AU, (GC, GU), GC), 2.6), ((AU, (GC, GG), GU), 1.3), ((AU, (GC, GU), GU), 2.6), ((AU, (GC, UA), GU), 2.6), ((AU, (GC, UC), GU), 2.6), ((AU, (GC, UG), GU), 2.6), ((AU, (GC, UU), GU), 2.6),
      ((AU, (GG, AA), GU), 0.7), ((AU, (GG, AC), GU), 1.0), ((AU, (GG, AG), GU), 1.0), ((AU, (GG, GU), AU), 1.0), ((AU, (GG, CA), GU), 1.0), ((AU, (GG, CC), GU), 1.0), ((AU, (GG, GU), CG), 1.0), ((AU, (GG, CU), GU), 1.0), ((AU, (GG, GA), GU), 2.6), ((AU, (GG, GU), GC), 1.0), ((AU, (GG, GG), GU), 2.3), ((AU, (GG, GU), GU), 1.0), ((AU, (GG, UA), GU), 1.0), ((AU, (GG, UC), GU), 1.0), ((AU, (GG, UG), GU), 1.0), ((AU, (GG, UU), GU), 2.3),
      ((AU, (GU, AA), GU), 2.3), ((AU, (GU, AC), GU), 2.6), ((AU, (GU, AG), GU), 1.3), ((AU, (GU, GU), AU), 2.6), ((AU, (GU, CA), GU), 2.6), ((AU, (GU, CC), GU), 2.6), ((AU, (GU, GU), CG), 2.6), ((AU, (GU, CU), GU), 2.6), ((AU, (GU, GA), GU), 2.9), ((AU, (GU, GU), GC), 2.6), ((AU, (GU, GG), GU), 1.3), ((AU, (GU, GU), GU), 2.6), ((AU, (GU, UA), GU), 2.6), ((AU, (GU, UC), GU), 2.6), ((AU, (GU, UG), GU), 2.6), ((AU, (GU, UU), GU), 2.6),
      ((AU, (UA, AA), GU), 2.2), ((AU, (UA, AC), GU), 2.5), ((AU, (UA, AG), GU), 1.2), ((AU, (UA, GU), AU), 2.5), ((AU, (UA, CA), GU), 2.5), ((AU, (UA, CC), GU), 2.5), ((AU, (UA, GU), CG), 2.5), ((AU, (UA, CU), GU), 2.5), ((AU, (UA, GA), GU), 2.8), ((AU, (UA, GU), GC), 2.5), ((AU, (UA, GG), GU), 1.2), ((AU, (UA, GU), GU), 2.5), ((AU, (UA, UA), GU), 2.5), ((AU, (UA, UC), GU), 2.5), ((AU, (UA, UG), GU), 2.5), ((AU, (UA, UU), GU), 2.5),
      ((AU, (UC, AA), GU), 2.2), ((AU, (UC, AC), GU), 2.6), ((AU, (UC, AG), GU), 1.8), ((AU, (UC, GU), AU), 2.6), ((AU, (UC, CA), GU), 2.6), ((AU, (UC, CC), GU), 2.6), ((AU, (UC, GU), CG), 2.6), ((AU, (UC, CU), GU), 2.6), ((AU, (UC, GA), GU), 3.5), ((AU, (UC, GU), GC), 2.6), ((AU, (UC, GG), GU), 1.3), ((AU, (UC, GU), GU), 2.6), ((AU, (UC, UA), GU), 2.6), ((AU, (UC, UC), GU), 2.6), ((AU, (UC, UG), GU), 2.6), ((AU, (UC, UU), GU), 2.6),
      ((AU, (UG, AA), GU), 2.2), ((AU, (UG, AC), GU), 2.5), ((AU, (UG, AG), GU), 1.2), ((AU, (UG, GU), AU), 2.5), ((AU, (UG, CA), GU), 2.5), ((AU, (UG, CC), GU), 2.5), ((AU, (UG, GU), CG), 2.5), ((AU, (UG, CU), GU), 2.5), ((AU, (UG, GA), GU), 2.8), ((AU, (UG, GU), GC), 2.5), ((AU, (UG, GG), GU), 1.2), ((AU, (UG, GU), GU), 2.5), ((AU, (UG, UA), GU), 2.5), ((AU, (UG, UC), GU), 2.5), ((AU, (UG, UG), GU), 2.5), ((AU, (UG, UU), GU), 2.5),
      ((AU, (UU, AA), GU), 1.9), ((AU, (UU, AC), GU), 1.7), ((AU, (UU, AG), GU), 0.3), ((AU, (UU, GU), AU), 1.7), ((AU, (UU, CA), GU), 1.7), ((AU, (UU, CC), GU), 1.7), ((AU, (UU, GU), CG), 1.7), ((AU, (UU, CU), GU), 1.7), ((AU, (UU, GA), GU), 2.0), ((AU, (UU, GU), GC), 1.7), ((AU, (UU, GG), GU), 1.7), ((AU, (UU, GU), GU), 1.7), ((AU, (UU, UA), GU), 1.7), ((AU, (UU, UC), GU), 1.7), ((AU, (UU, UG), GU), 1.7), ((AU, (UU, UU), GU), 1.7),
      // For internal loops between the base pairs "AU" and "UA".
      ((AU, (AA, AA), UA), 2.8), ((AU, (AA, AC), UA), 2.5), ((AU, (AA, AG), UA), 1.5), ((AU, (AA, UA), AU), 2.5), ((AU, (AA, CA), UA), 2.6), ((AU, (AA, CC), UA), 2.6), ((AU, (AA, UA), CG), 2.8), ((AU, (AA, CU), UA), 2.6), ((AU, (AA, GA), UA), 2.2), ((AU, (AA, UA), GC), 2.5), ((AU, (AA, GG), UA), 1.0), ((AU, (AA, UA), GU), 2.5), ((AU, (AA, UA), UA), 2.6), ((AU, (AA, UC), UA), 2.6), ((AU, (AA, UG), UA), 2.6), ((AU, (AA, UU), UA), 2.3),
      ((AU, (AC, AA), UA), 2.6), ((AU, (AC, AC), UA), 2.4), ((AU, (AC, AG), UA), 1.4), ((AU, (AC, UA), AU), 2.4), ((AU, (AC, CA), UA), 2.5), ((AU, (AC, CC), UA), 2.4), ((AU, (AC, UA), CG), 2.5), ((AU, (AC, CU), UA), 2.4), ((AU, (AC, GA), UA), 2.1), ((AU, (AC, UA), GC), 2.4), ((AU, (AC, GG), UA), 0.9), ((AU, (AC, UA), GU), 2.4), ((AU, (AC, UA), UA), 2.5), ((AU, (AC, UC), UA), 2.4), ((AU, (AC, UG), UA), 2.5), ((AU, (AC, UU), UA), 1.5),
      ((AU, (AG, AA), UA), 2.2), ((AU, (AG, AC), UA), 2.0), ((AU, (AG, AG), UA), 1.0), ((AU, (AG, UA), AU), 2.0), ((AU, (AG, CA), UA), 2.1), ((AU, (AG, CC), UA), 2.6), ((AU, (AG, UA), CG), 2.1), ((AU, (AG, CU), UA), 2.6), ((AU, (AG, GA), UA), 1.7), ((AU, (AG, UA), GC), 2.0), ((AU, (AG, GG), UA), 1.8), ((AU, (AG, UA), GU), 2.0), ((AU, (AG, UA), UA), 2.1), ((AU, (AG, UC), UA), 2.6), ((AU, (AG, UG), UA), 2.1), ((AU, (AG, UU), UA), 1.1),
      ((AU, (AU, AA), UA), 2.6), ((AU, (AU, AC), UA), 2.4), ((AU, (AU, AG), UA), 1.4), ((AU, (AU, UA), AU), 2.4), ((AU, (AU, CA), UA), 2.5), ((AU, (AU, CC), UA), 2.4), ((AU, (AU, UA), CG), 2.5), ((AU, (AU, CU), UA), 2.4), ((AU, (AU, GA), UA), 2.1), ((AU, (AU, UA), GC), 2.4), ((AU, (AU, GG), UA), 0.9), ((AU, (AU, UA), GU), 2.4), ((AU, (AU, UA), UA), 2.5), ((AU, (AU, UC), UA), 2.4), ((AU, (AU, UG), UA), 2.5), ((AU, (AU, UU), UA), 1.5),
      ((AU, (CA, AA), UA), 2.5), ((AU, (CA, AC), UA), 2.3), ((AU, (CA, AG), UA), 1.3), ((AU, (CA, UA), AU), 2.3), ((AU, (CA, CA), UA), 2.4), ((AU, (CA, CC), UA), 2.3), ((AU, (CA, UA), CG), 2.4), ((AU, (CA, CU), UA), 2.3), ((AU, (CA, GA), UA), 2.0), ((AU, (CA, UA), GC), 2.3), ((AU, (CA, GG), UA), 0.8), ((AU, (CA, UA), GU), 2.3), ((AU, (CA, UA), UA), 2.4), ((AU, (CA, UC), UA), 2.3), ((AU, (CA, UG), UA), 2.4), ((AU, (CA, UU), UA), 1.4),
      ((AU, (CC, AA), UA), 2.6), ((AU, (CC, AC), UA), 2.3), ((AU, (CC, AG), UA), 1.9), ((AU, (CC, UA), AU), 2.3), ((AU, (CC, CA), UA), 2.4), ((AU, (CC, CC), UA), 2.4), ((AU, (CC, UA), CG), 2.4), ((AU, (CC, CU), UA), 2.4), ((AU, (CC, GA), UA), 2.6), ((AU, (CC, UA), GC), 2.3), ((AU, (CC, GG), UA), 0.8), ((AU, (CC, UA), GU), 2.3), ((AU, (CC, UA), UA), 2.4), ((AU, (CC, UC), UA), 2.4), ((AU, (CC, UG), UA), 2.4), ((AU, (CC, UU), UA), 2.4),
      ((AU, (CG, AA), UA), 2.5), ((AU, (CG, AC), UA), 2.3), ((AU, (CG, AG), UA), 1.3), ((AU, (CG, UA), AU), 2.3), ((AU, (CG, CA), UA), 2.4), ((AU, (CG, CC), UA), 2.3), ((AU, (CG, UA), CG), 2.4), ((AU, (CG, CU), UA), 2.3), ((AU, (CG, GA), UA), 2.0), ((AU, (CG, UA), GC), 2.3), ((AU, (CG, GG), UA), 0.8), ((AU, (CG, UA), GU), 2.3), ((AU, (CG, UA), UA), 2.4), ((AU, (CG, UC), UA), 2.3), ((AU, (CG, UG), UA), 2.4), ((AU, (CG, UU), UA), 1.4),
      ((AU, (CU, AA), UA), 2.6), ((AU, (CU, AC), UA), 2.3), ((AU, (CU, AG), UA), 1.9), ((AU, (CU, UA), AU), 2.3), ((AU, (CU, CA), UA), 2.4), ((AU, (CU, CC), UA), 2.4), ((AU, (CU, UA), CG), 2.4), ((AU, (CU, CU), UA), 2.4), ((AU, (CU, GA), UA), 2.6), ((AU, (CU, UA), GC), 2.3), ((AU, (CU, GG), UA), 0.8), ((AU, (CU, UA), GU), 2.3), ((AU, (CU, UA), UA), 2.4), ((AU, (CU, UC), UA), 2.4), ((AU, (CU, UG), UA), 2.4), ((AU, (CU, UU), UA), 1.5),
      ((AU, (GA, AA), UA), 1.5), ((AU, (GA, AC), UA), 1.3), ((AU, (GA, AG), UA), 0.3), ((AU, (GA, UA), AU), 1.3), ((AU, (GA, CA), UA), 1.4), ((AU, (GA, CC), UA), 1.9), ((AU, (GA, UA), CG), 1.4), ((AU, (GA, CU), UA), 1.9), ((AU, (GA, GA), UA), 1.0), ((AU, (GA, UA), GC), 1.3), ((AU, (GA, GG), UA), 1.1), ((AU, (GA, UA), GU), 1.3), ((AU, (GA, UA), UA), 1.4), ((AU, (GA, UC), UA), 1.9), ((AU, (GA, UG), UA), 1.4), ((AU, (GA, UU), UA), 0.4),
      ((AU, (GC, AA), UA), 2.6), ((AU, (GC, AC), UA), 2.4), ((AU, (GC, AG), UA), 1.4), ((AU, (GC, UA), AU), 2.4), ((AU, (GC, CA), UA), 2.5), ((AU, (GC, CC), UA), 2.4), ((AU, (GC, UA), CG), 2.5), ((AU, (GC, CU), UA), 2.4), ((AU, (GC, GA), UA), 2.1), ((AU, (GC, UA), GC), 2.4), ((AU, (GC, GG), UA), 0.9), ((AU, (GC, UA), GU), 2.4), ((AU, (GC, UA), UA), 2.5), ((AU, (GC, UC), UA), 2.4), ((AU, (GC, UG), UA), 2.5), ((AU, (GC, UU), UA), 1.5),
      ((AU, (GG, AA), UA), 1.0), ((AU, (GG, AC), UA), 0.8), ((AU, (GG, AG), UA), 1.1), ((AU, (GG, UA), AU), 0.8), ((AU, (GG, CA), UA), 0.9), ((AU, (GG, CC), UA), 0.8), ((AU, (GG, UA), CG), 0.9), ((AU, (GG, CU), UA), 0.8), ((AU, (GG, GA), UA), 1.8), ((AU, (GG, UA), GC), 0.8), ((AU, (GG, GG), UA), 1.9), ((AU, (GG, UA), GU), 0.8), ((AU, (GG, UA), UA), 0.9), ((AU, (GG, UC), UA), 0.8), ((AU, (GG, UG), UA), 0.9), ((AU, (GG, UU), UA), 1.2),
      ((AU, (GU, AA), UA), 2.6), ((AU, (GU, AC), UA), 2.4), ((AU, (GU, AG), UA), 1.4), ((AU, (GU, UA), AU), 2.4), ((AU, (GU, CA), UA), 2.5), ((AU, (GU, CC), UA), 2.4), ((AU, (GU, UA), CG), 2.5), ((AU, (GU, CU), UA), 2.4), ((AU, (GU, GA), UA), 2.1), ((AU, (GU, UA), GC), 2.4), ((AU, (GU, GG), UA), 0.9), ((AU, (GU, UA), GU), 2.4), ((AU, (GU, UA), UA), 2.5), ((AU, (GU, UC), UA), 2.4), ((AU, (GU, UG), UA), 2.5), ((AU, (GU, UU), UA), 1.5),
      ((AU, (UA, AA), UA), 2.5), ((AU, (UA, AC), UA), 2.3), ((AU, (UA, AG), UA), 1.3), ((AU, (UA, UA), AU), 2.3), ((AU, (UA, CA), UA), 2.4), ((AU, (UA, CC), UA), 2.3), ((AU, (UA, UA), CG), 2.4), ((AU, (UA, CU), UA), 2.3), ((AU, (UA, GA), UA), 2.0), ((AU, (UA, UA), GC), 2.3), ((AU, (UA, GG), UA), 0.8), ((AU, (UA, UA), GU), 2.3), ((AU, (UA, UA), UA), 2.4), ((AU, (UA, UC), UA), 2.3), ((AU, (UA, UG), UA), 2.4), ((AU, (UA, UU), UA), 1.4),
      ((AU, (UC, AA), UA), 2.6), ((AU, (UC, AC), UA), 2.3), ((AU, (UC, AG), UA), 1.9), ((AU, (UC, UA), AU), 2.3), ((AU, (UC, CA), UA), 2.4), ((AU, (UC, CC), UA), 2.4), ((AU, (UC, UA), CG), 2.4), ((AU, (UC, CU), UA), 2.4), ((AU, (UC, GA), UA), 2.6), ((AU, (UC, UA), GC), 2.3), ((AU, (UC, GG), UA), 0.8), ((AU, (UC, UA), GU), 2.3), ((AU, (UC, UA), UA), 2.4), ((AU, (UC, UC), UA), 2.4), ((AU, (UC, UG), UA), 2.4), ((AU, (UC, UU), UA), 1.5),
      ((AU, (UG, AA), UA), 2.5), ((AU, (UG, AC), UA), 2.3), ((AU, (UG, AG), UA), 1.3), ((AU, (UG, UA), AU), 2.3), ((AU, (UG, CA), UA), 2.4), ((AU, (UG, CC), UA), 2.3), ((AU, (UG, UA), CG), 2.4), ((AU, (UG, CU), UA), 2.3), ((AU, (UG, GA), UA), 2.0), ((AU, (UG, UA), GC), 2.3), ((AU, (UG, GG), UA), 0.8), ((AU, (UG, UA), GU), 2.3), ((AU, (UG, UA), UA), 2.4), ((AU, (UG, UC), UA), 2.3), ((AU, (UG, UG), UA), 2.4), ((AU, (UG, UU), UA), 1.4),
      ((AU, (UU, AA), UA), 2.3), ((AU, (UU, AC), UA), 1.4), ((AU, (UU, AG), UA), 0.4), ((AU, (UU, UA), AU), 1.4), ((AU, (UU, CA), UA), 1.5), ((AU, (UU, CC), UA), 1.5), ((AU, (UU, UA), CG), 1.5), ((AU, (UU, CU), UA), 1.5), ((AU, (UU, GA), UA), 1.1), ((AU, (UU, UA), GC), 1.4), ((AU, (UU, GG), UA), 1.2), ((AU, (UU, UA), GU), 1.4), ((AU, (UU, UA), UA), 1.5), ((AU, (UU, UC), UA), 1.5), ((AU, (UU, UG), UA), 1.5), ((AU, (UU, UU), UA), 0.6),
      // For internal loops between the base pairs "AU" and "UG".
      ((AU, (AA, AA), UG), 3.4), ((AU, (AA, AC), UG), 3.1), ((AU, (AA, AG), UG), 2.3), ((AU, (AA, UG), AU), 3.1), ((AU, (AA, CA), UG), 3.1), ((AU, (AA, CC), UG), 3.1), ((AU, (AA, UG), CG), 3.1), ((AU, (AA, CU), UG), 3.1), ((AU, (AA, GA), UG), 2.7), ((AU, (AA, UG), GC), 3.1), ((AU, (AA, GG), UG), 1.8), ((AU, (AA, UG), GU), 3.1), ((AU, (AA, UG), UA), 3.1), ((AU, (AA, UC), UG), 3.1), ((AU, (AA, UG), UG), 3.1), ((AU, (AA, UU), UG), 3.7),
      ((AU, (AC, AA), UG), 3.3), ((AU, (AC, AC), UG), 2.9), ((AU, (AC, AG), UG), 2.1), ((AU, (AC, UG), AU), 2.9), ((AU, (AC, CA), UG), 2.9), ((AU, (AC, CC), UG), 2.9), ((AU, (AC, UG), CG), 2.9), ((AU, (AC, CU), UG), 2.9), ((AU, (AC, GA), UG), 2.5), ((AU, (AC, UG), GC), 2.9), ((AU, (AC, GG), UG), 1.6), ((AU, (AC, UG), GU), 2.9), ((AU, (AC, UG), UA), 2.9), ((AU, (AC, UC), UG), 2.9), ((AU, (AC, UG), UG), 2.9), ((AU, (AC, UU), UG), 2.9),
      ((AU, (AG, AA), UG), 2.9), ((AU, (AG, AC), UG), 2.5), ((AU, (AG, AG), UG), 1.7), ((AU, (AG, UG), AU), 2.5), ((AU, (AG, CA), UG), 2.5), ((AU, (AG, CC), UG), 3.1), ((AU, (AG, UG), CG), 2.5), ((AU, (AG, CU), UG), 3.1), ((AU, (AG, GA), UG), 2.1), ((AU, (AG, UG), GC), 2.5), ((AU, (AG, GG), UG), 2.5), ((AU, (AG, UG), GU), 2.5), ((AU, (AG, UG), UA), 2.5), ((AU, (AG, UC), UG), 3.1), ((AU, (AG, UG), UG), 2.5), ((AU, (AG, UU), UG), 2.5),
      ((AU, (AU, AA), UG), 3.3), ((AU, (AU, AC), UG), 2.9), ((AU, (AU, AG), UG), 2.1), ((AU, (AU, UG), AU), 2.9), ((AU, (AU, CA), UG), 2.9), ((AU, (AU, CC), UG), 2.9), ((AU, (AU, UG), CG), 2.9), ((AU, (AU, CU), UG), 2.9), ((AU, (AU, GA), UG), 2.5), ((AU, (AU, UG), GC), 2.9), ((AU, (AU, GG), UG), 1.6), ((AU, (AU, UG), GU), 2.9), ((AU, (AU, UG), UA), 2.9), ((AU, (AU, UC), UG), 2.9), ((AU, (AU, UG), UG), 2.9), ((AU, (AU, UU), UG), 2.9),
      ((AU, (CA, AA), UG), 3.2), ((AU, (CA, AC), UG), 2.8), ((AU, (CA, AG), UG), 2.0), ((AU, (CA, UG), AU), 2.8), ((AU, (CA, CA), UG), 2.8), ((AU, (CA, CC), UG), 2.8), ((AU, (CA, UG), CG), 2.8), ((AU, (CA, CU), UG), 2.8), ((AU, (CA, GA), UG), 2.4), ((AU, (CA, UG), GC), 2.8), ((AU, (CA, GG), UG), 1.5), ((AU, (CA, UG), GU), 2.8), ((AU, (CA, UG), UA), 2.8), ((AU, (CA, UC), UG), 2.8), ((AU, (CA, UG), UG), 2.8), ((AU, (CA, UU), UG), 2.8),
      ((AU, (CC, AA), UG), 3.2), ((AU, (CC, AC), UG), 2.9), ((AU, (CC, AG), UG), 2.7), ((AU, (CC, UG), AU), 2.9), ((AU, (CC, CA), UG), 2.9), ((AU, (CC, CC), UG), 2.9), ((AU, (CC, UG), CG), 2.9), ((AU, (CC, CU), UG), 2.9), ((AU, (CC, GA), UG), 3.1), ((AU, (CC, UG), GC), 2.9), ((AU, (CC, GG), UG), 1.6), ((AU, (CC, UG), GU), 2.9), ((AU, (CC, UG), UA), 2.9), ((AU, (CC, UC), UG), 2.9), ((AU, (CC, UG), UG), 2.9), ((AU, (CC, UU), UG), 2.9),
      ((AU, (CG, AA), UG), 3.2), ((AU, (CG, AC), UG), 2.8), ((AU, (CG, AG), UG), 2.0), ((AU, (CG, UG), AU), 2.8), ((AU, (CG, CA), UG), 2.8), ((AU, (CG, CC), UG), 2.8), ((AU, (CG, UG), CG), 2.8), ((AU, (CG, CU), UG), 2.8), ((AU, (CG, GA), UG), 2.4), ((AU, (CG, UG), GC), 2.8), ((AU, (CG, GG), UG), 1.5), ((AU, (CG, UG), GU), 2.8), ((AU, (CG, UG), UA), 2.8), ((AU, (CG, UC), UG), 2.8), ((AU, (CG, UG), UG), 2.8), ((AU, (CG, UU), UG), 2.8),
      ((AU, (CU, AA), UG), 3.2), ((AU, (CU, AC), UG), 2.9), ((AU, (CU, AG), UG), 2.7), ((AU, (CU, UG), AU), 2.9), ((AU, (CU, CA), UG), 2.9), ((AU, (CU, CC), UG), 2.9), ((AU, (CU, UG), CG), 2.9), ((AU, (CU, CU), UG), 2.9), ((AU, (CU, GA), UG), 3.1), ((AU, (CU, UG), GC), 2.9), ((AU, (CU, GG), UG), 1.6), ((AU, (CU, UG), GU), 2.9), ((AU, (CU, UG), UA), 2.9), ((AU, (CU, UC), UG), 2.9), ((AU, (CU, UG), UG), 2.9), ((AU, (CU, UU), UG), 2.9),
      ((AU, (GA, AA), UG), 2.2), ((AU, (GA, AC), UG), 1.8), ((AU, (GA, AG), UG), 1.0), ((AU, (GA, UG), AU), 1.8), ((AU, (GA, CA), UG), 1.8), ((AU, (GA, CC), UG), 2.4), ((AU, (GA, UG), CG), 1.8), ((AU, (GA, CU), UG), 2.4), ((AU, (GA, GA), UG), 1.4), ((AU, (GA, UG), GC), 1.8), ((AU, (GA, GG), UG), 1.8), ((AU, (GA, UG), GU), 1.8), ((AU, (GA, UG), UA), 1.8), ((AU, (GA, UC), UG), 2.4), ((AU, (GA, UG), UG), 1.8), ((AU, (GA, UU), UG), 1.8),
      ((AU, (GC, AA), UG), 3.3), ((AU, (GC, AC), UG), 2.9), ((AU, (GC, AG), UG), 2.1), ((AU, (GC, UG), AU), 2.9), ((AU, (GC, CA), UG), 2.9), ((AU, (GC, CC), UG), 2.9), ((AU, (GC, UG), CG), 2.9), ((AU, (GC, CU), UG), 2.9), ((AU, (GC, GA), UG), 2.5), ((AU, (GC, UG), GC), 2.9), ((AU, (GC, GG), UG), 1.6), ((AU, (GC, UG), GU), 2.9), ((AU, (GC, UG), UA), 2.9), ((AU, (GC, UC), UG), 2.9), ((AU, (GC, UG), UG), 2.9), ((AU, (GC, UU), UG), 2.9),
      ((AU, (GG, AA), UG), 1.7), ((AU, (GG, AC), UG), 1.3), ((AU, (GG, AG), UG), 1.8), ((AU, (GG, UG), AU), 1.3), ((AU, (GG, CA), UG), 1.3), ((AU, (GG, CC), UG), 1.3), ((AU, (GG, UG), CG), 1.3), ((AU, (GG, CU), UG), 1.3), ((AU, (GG, GA), UG), 2.2), ((AU, (GG, UG), GC), 1.3), ((AU, (GG, GG), UG), 2.6), ((AU, (GG, UG), GU), 1.3), ((AU, (GG, UG), UA), 1.3), ((AU, (GG, UC), UG), 1.3), ((AU, (GG, UG), UG), 1.3), ((AU, (GG, UU), UG), 2.6),
      ((AU, (GU, AA), UG), 3.3), ((AU, (GU, AC), UG), 2.9), ((AU, (GU, AG), UG), 2.1), ((AU, (GU, UG), AU), 2.9), ((AU, (GU, CA), UG), 2.9), ((AU, (GU, CC), UG), 2.9), ((AU, (GU, UG), CG), 2.9), ((AU, (GU, CU), UG), 2.9), ((AU, (GU, GA), UG), 2.5), ((AU, (GU, UG), GC), 2.9), ((AU, (GU, GG), UG), 1.6), ((AU, (GU, UG), GU), 2.9), ((AU, (GU, UG), UA), 2.9), ((AU, (GU, UC), UG), 2.9), ((AU, (GU, UG), UG), 2.9), ((AU, (GU, UU), UG), 2.9),
      ((AU, (UA, AA), UG), 3.2), ((AU, (UA, AC), UG), 2.8), ((AU, (UA, AG), UG), 2.0), ((AU, (UA, UG), AU), 2.8), ((AU, (UA, CA), UG), 2.8), ((AU, (UA, CC), UG), 2.8), ((AU, (UA, UG), CG), 2.8), ((AU, (UA, CU), UG), 2.8), ((AU, (UA, GA), UG), 2.4), ((AU, (UA, UG), GC), 2.8), ((AU, (UA, GG), UG), 1.5), ((AU, (UA, UG), GU), 2.8), ((AU, (UA, UG), UA), 2.8), ((AU, (UA, UC), UG), 2.8), ((AU, (UA, UG), UG), 2.8), ((AU, (UA, UU), UG), 2.8),
      ((AU, (UC, AA), UG), 3.2), ((AU, (UC, AC), UG), 2.9), ((AU, (UC, AG), UG), 2.7), ((AU, (UC, UG), AU), 2.9), ((AU, (UC, CA), UG), 2.9), ((AU, (UC, CC), UG), 2.9), ((AU, (UC, UG), CG), 2.9), ((AU, (UC, CU), UG), 2.9), ((AU, (UC, GA), UG), 3.1), ((AU, (UC, UG), GC), 2.9), ((AU, (UC, GG), UG), 1.6), ((AU, (UC, UG), GU), 2.9), ((AU, (UC, UG), UA), 2.9), ((AU, (UC, UC), UG), 2.9), ((AU, (UC, UG), UG), 2.9), ((AU, (UC, UU), UG), 2.9),
      ((AU, (UG, AA), UG), 3.2), ((AU, (UG, AC), UG), 2.8), ((AU, (UG, AG), UG), 2.0), ((AU, (UG, UG), AU), 2.8), ((AU, (UG, CA), UG), 2.8), ((AU, (UG, CC), UG), 2.8), ((AU, (UG, UG), CG), 2.8), ((AU, (UG, CU), UG), 2.8), ((AU, (UG, GA), UG), 2.4), ((AU, (UG, UG), GC), 2.8), ((AU, (UG, GG), UG), 1.5), ((AU, (UG, UG), GU), 2.8), ((AU, (UG, UG), UA), 2.8), ((AU, (UG, UC), UG), 2.8), ((AU, (UG, UG), UG), 2.8), ((AU, (UG, UU), UG), 2.8),
      ((AU, (UU, AA), UG), 2.9), ((AU, (UU, AC), UG), 2.0), ((AU, (UU, AG), UG), 1.2), ((AU, (UU, UG), AU), 2.0), ((AU, (UU, CA), UG), 2.0), ((AU, (UU, CC), UG), 2.0), ((AU, (UU, UG), CG), 2.0), ((AU, (UU, CU), UG), 2.0), ((AU, (UU, GA), UG), 1.6), ((AU, (UU, UG), GC), 2.0), ((AU, (UU, GG), UG), 2.0), ((AU, (UU, UG), GU), 2.0), ((AU, (UU, UG), UA), 2.0), ((AU, (UU, UC), UG), 2.0), ((AU, (UU, UG), UG), 2.0), ((AU, (UU, UU), UG), 2.0),
      // For internal loops behind the base pair "CG".
      // For internal loops between the base pairs "CG" and "AU".
      ((CG, (AA, AA), AU), 2.0), ((CG, (AA, AC), AU), 1.5), ((CG, (AA, AG), AU), 0.9), ((CG, (AA, AU), AU), 1.5), ((CG, (AA, CA), AU), 2.0), ((CG, (AA, CC), AU), 2.0), ((CG, (AA, AU), CG), 2.0), ((CG, (AA, CU), AU), 2.0), ((CG, (AA, GA), AU), 1.0), ((CG, (AA, AU), GC), 1.5), ((CG, (AA, GG), AU), 0.4), ((CG, (AA, AU), GU), 1.5), ((CG, (AA, AU), UA), 2.0), ((CG, (AA, UC), AU), 1.7), ((CG, (AA, AU), UG), 2.0), ((CG, (AA, UU), AU), 1.7),
      ((CG, (AC, AA), AU), 2.4), ((CG, (AC, AC), AU), 1.9), ((CG, (AC, AG), AU), 1.3), ((CG, (AC, AU), AU), 1.9), ((CG, (AC, CA), AU), 2.4), ((CG, (AC, CC), AU), 2.4), ((CG, (AC, AU), CG), 2.4), ((CG, (AC, CU), AU), 2.4), ((CG, (AC, GA), AU), 1.4), ((CG, (AC, AU), GC), 1.9), ((CG, (AC, GG), AU), 0.8), ((CG, (AC, AU), GU), 1.9), ((CG, (AC, AU), UA), 2.4), ((CG, (AC, UC), AU), 2.1), ((CG, (AC, AU), UG), 2.4), ((CG, (AC, UU), AU), 1.5),
      ((CG, (AG, AA), AU), 1.0), ((CG, (AG, AC), AU), 0.6), ((CG, (AG, AG), AU), 0.0), ((CG, (AG, AU), AU), 0.6), ((CG, (AG, CA), AU), 1.0), ((CG, (AG, CC), AU), 1.6), ((CG, (AG, AU), CG), 1.0), ((CG, (AG, CU), AU), 1.6), ((CG, (AG, GA), AU), 0.1), ((CG, (AG, AU), GC), 0.6), ((CG, (AG, GG), AU), 0.8), ((CG, (AG, AU), GU), 0.6), ((CG, (AG, AU), UA), 1.0), ((CG, (AG, UC), AU), 1.3), ((CG, (AG, AU), UG), 1.0), ((CG, (AG, UU), AU), 0.2),
      ((CG, (AU, AA), AU), 2.4), ((CG, (AU, AC), AU), 1.9), ((CG, (AU, AG), AU), 1.3), ((CG, (AU, AU), AU), 1.9), ((CG, (AU, CA), AU), 2.4), ((CG, (AU, CC), AU), 2.4), ((CG, (AU, AU), CG), 2.4), ((CG, (AU, CU), AU), 2.4), ((CG, (AU, GA), AU), 1.4), ((CG, (AU, AU), GC), 1.9), ((CG, (AU, GG), AU), 0.8), ((CG, (AU, AU), GU), 1.9), ((CG, (AU, AU), UA), 2.4), ((CG, (AU, UC), AU), 2.1), ((CG, (AU, AU), UG), 2.4), ((CG, (AU, UU), AU), 1.5),
      ((CG, (CA, AA), AU), 1.9), ((CG, (CA, AC), AU), 1.5), ((CG, (CA, AG), AU), 0.9), ((CG, (CA, AU), AU), 1.5), ((CG, (CA, CA), AU), 1.9), ((CG, (CA, CC), AU), 1.9), ((CG, (CA, AU), CG), 1.9), ((CG, (CA, CU), AU), 1.9), ((CG, (CA, GA), AU), 1.0), ((CG, (CA, AU), GC), 1.5), ((CG, (CA, GG), AU), 0.4), ((CG, (CA, AU), GU), 1.5), ((CG, (CA, AU), UA), 1.9), ((CG, (CA, UC), AU), 1.6), ((CG, (CA, AU), UG), 1.9), ((CG, (CA, UU), AU), 1.1),
      ((CG, (CC, AA), AU), 2.2), ((CG, (CC, AC), AU), 1.8), ((CG, (CC, AG), AU), 1.8), ((CG, (CC, AU), AU), 1.8), ((CG, (CC, CA), AU), 2.2), ((CG, (CC, CC), AU), 2.2), ((CG, (CC, AU), CG), 2.2), ((CG, (CC, CU), AU), 2.2), ((CG, (CC, GA), AU), 1.9), ((CG, (CC, AU), GC), 1.8), ((CG, (CC, GG), AU), 0.7), ((CG, (CC, AU), GU), 1.8), ((CG, (CC, AU), UA), 2.2), ((CG, (CC, UC), AU), 1.9), ((CG, (CC, AU), UG), 2.2), ((CG, (CC, UU), AU), 1.4),
      ((CG, (CG, AA), AU), 1.9), ((CG, (CG, AC), AU), 1.5), ((CG, (CG, AG), AU), 0.9), ((CG, (CG, AU), AU), 1.5), ((CG, (CG, CA), AU), 1.9), ((CG, (CG, CC), AU), 1.9), ((CG, (CG, AU), CG), 1.9), ((CG, (CG, CU), AU), 1.9), ((CG, (CG, GA), AU), 1.0), ((CG, (CG, AU), GC), 1.5), ((CG, (CG, GG), AU), 0.4), ((CG, (CG, AU), GU), 1.5), ((CG, (CG, AU), UA), 1.9), ((CG, (CG, UC), AU), 1.6), ((CG, (CG, AU), UG), 1.9), ((CG, (CG, UU), AU), 1.1),
      ((CG, (CU, AA), AU), 2.1), ((CG, (CU, AC), AU), 1.6), ((CG, (CU, AG), AU), 1.6), ((CG, (CU, AU), AU), 1.6), ((CG, (CU, CA), AU), 2.1), ((CG, (CU, CC), AU), 2.1), ((CG, (CU, AU), CG), 2.1), ((CG, (CU, CU), AU), 2.1), ((CG, (CU, GA), AU), 1.7), ((CG, (CU, AU), GC), 1.6), ((CG, (CU, GG), AU), 0.5), ((CG, (CU, AU), GU), 1.6), ((CG, (CU, AU), UA), 2.1), ((CG, (CU, UC), AU), 1.8), ((CG, (CU, AU), UG), 2.1), ((CG, (CU, UU), AU), 1.2),
      ((CG, (GA, AA), AU), 1.0), ((CG, (GA, AC), AU), 0.6), ((CG, (GA, AG), AU), 0.0), ((CG, (GA, AU), AU), 0.6), ((CG, (GA, CA), AU), 1.0), ((CG, (GA, CC), AU), 1.6), ((CG, (GA, AU), CG), 1.0), ((CG, (GA, CU), AU), 1.6), ((CG, (GA, GA), AU), 0.1), ((CG, (GA, AU), GC), 0.6), ((CG, (GA, GG), AU), 0.8), ((CG, (GA, AU), GU), 0.6), ((CG, (GA, AU), UA), 1.0), ((CG, (GA, UC), AU), 1.3), ((CG, (GA, AU), UG), 1.0), ((CG, (GA, UU), AU), 0.2),
      ((CG, (GC, AA), AU), 2.4), ((CG, (GC, AC), AU), 1.9), ((CG, (GC, AG), AU), 1.3), ((CG, (GC, AU), AU), 1.9), ((CG, (GC, CA), AU), 2.4), ((CG, (GC, CC), AU), 2.4), ((CG, (GC, AU), CG), 2.4), ((CG, (GC, CU), AU), 2.4), ((CG, (GC, GA), AU), 1.4), ((CG, (GC, AU), GC), 1.9), ((CG, (GC, GG), AU), 0.8), ((CG, (GC, AU), GU), 1.9), ((CG, (GC, AU), UA), 2.4), ((CG, (GC, UC), AU), 2.1), ((CG, (GC, AU), UG), 2.4), ((CG, (GC, UU), AU), 1.5),
      ((CG, (GG, AA), AU), 0.5), ((CG, (GG, AC), AU), 0.0), ((CG, (GG, AG), AU), 0.7), ((CG, (GG, AU), AU), 0.0), ((CG, (GG, CA), AU), 0.5), ((CG, (GG, CC), AU), 0.5), ((CG, (GG, AU), CG), 0.5), ((CG, (GG, CU), AU), 0.5), ((CG, (GG, GA), AU), 0.8), ((CG, (GG, AU), GC), 0.0), ((CG, (GG, GG), AU), 1.5), ((CG, (GG, AU), GU), 0.0), ((CG, (GG, AU), UA), 0.5), ((CG, (GG, UC), AU), 0.2), ((CG, (GG, AU), UG), 0.5), ((CG, (GG, UU), AU), 0.9),
      ((CG, (GU, AA), AU), 2.4), ((CG, (GU, AC), AU), 1.9), ((CG, (GU, AG), AU), 1.3), ((CG, (GU, AU), AU), 1.9), ((CG, (GU, CA), AU), 2.4), ((CG, (GU, CC), AU), 2.4), ((CG, (GU, AU), CG), 2.4), ((CG, (GU, CU), AU), 2.4), ((CG, (GU, GA), AU), 1.4), ((CG, (GU, AU), GC), 1.9), ((CG, (GU, GG), AU), 0.8), ((CG, (GU, AU), GU), 1.9), ((CG, (GU, AU), UA), 2.4), ((CG, (GU, UC), AU), 2.1), ((CG, (GU, AU), UG), 2.4), ((CG, (GU, UU), AU), 1.5),
      ((CG, (UA, AA), AU), 1.9), ((CG, (UA, AC), AU), 1.5), ((CG, (UA, AG), AU), 0.9), ((CG, (UA, AU), AU), 1.5), ((CG, (UA, CA), AU), 1.9), ((CG, (UA, CC), AU), 1.9), ((CG, (UA, AU), CG), 1.9), ((CG, (UA, CU), AU), 1.9), ((CG, (UA, GA), AU), 1.0), ((CG, (UA, AU), GC), 1.5), ((CG, (UA, GG), AU), 0.4), ((CG, (UA, AU), GU), 1.5), ((CG, (UA, AU), UA), 1.9), ((CG, (UA, UC), AU), 1.6), ((CG, (UA, AU), UG), 1.9), ((CG, (UA, UU), AU), 1.1),
      ((CG, (UC, AA), AU), 2.1), ((CG, (UC, AC), AU), 1.6), ((CG, (UC, AG), AU), 1.6), ((CG, (UC, AU), AU), 1.6), ((CG, (UC, CA), AU), 2.1), ((CG, (UC, CC), AU), 2.1), ((CG, (UC, AU), CG), 2.1), ((CG, (UC, CU), AU), 2.1), ((CG, (UC, GA), AU), 1.7), ((CG, (UC, AU), GC), 1.6), ((CG, (UC, GG), AU), 0.5), ((CG, (UC, AU), GU), 1.6), ((CG, (UC, AU), UA), 2.1), ((CG, (UC, UC), AU), 1.8), ((CG, (UC, AU), UG), 2.1), ((CG, (UC, UU), AU), 1.2),
      ((CG, (UG, AA), AU), 1.9), ((CG, (UG, AC), AU), 1.5), ((CG, (UG, AG), AU), 0.9), ((CG, (UG, AU), AU), 1.5), ((CG, (UG, CA), AU), 1.9), ((CG, (UG, CC), AU), 1.9), ((CG, (UG, AU), CG), 1.9), ((CG, (UG, CU), AU), 1.9), ((CG, (UG, GA), AU), 1.0), ((CG, (UG, AU), GC), 1.5), ((CG, (UG, GG), AU), 0.4), ((CG, (UG, AU), GU), 1.5), ((CG, (UG, AU), UA), 1.9), ((CG, (UG, UC), AU), 1.6), ((CG, (UG, AU), UG), 1.9), ((CG, (UG, UU), AU), 1.1),
      ((CG, (UU, AA), AU), 1.8), ((CG, (UU, AC), AU), 0.7), ((CG, (UU, AG), AU), 0.1), ((CG, (UU, AU), AU), 0.7), ((CG, (UU, CA), AU), 1.2), ((CG, (UU, CC), AU), 1.2), ((CG, (UU, AU), CG), 1.2), ((CG, (UU, CU), AU), 1.2), ((CG, (UU, GA), AU), 0.2), ((CG, (UU, AU), GC), 0.7), ((CG, (UU, GG), AU), 0.9), ((CG, (UU, AU), GU), 0.7), ((CG, (UU, AU), UA), 1.2), ((CG, (UU, UC), AU), 0.9), ((CG, (UU, AU), UG), 1.2), ((CG, (UU, UU), AU), 0.3),
      // For internal loops between the base pairs "CG" and "CG".
      ((CG, (AA, AA), CG), 1.3), ((CG, (AA, AC), CG), 1.1), ((CG, (AA, AG), CG), -0.3), ((CG, (AA, CG), AU), 1.1), ((CG, (AA, CA), CG), 1.0), ((CG, (AA, CC), CG), 1.1), ((CG, (AA, CG), CG), 1.0), ((CG, (AA, CU), CG), 1.1), ((CG, (AA, GA), CG), 0.4), ((CG, (AA, CG), GC), 1.1), ((CG, (AA, GG), CG), -0.3), ((CG, (AA, CG), GU), 1.1), ((CG, (AA, CG), UA), 1.0), ((CG, (AA, UC), CG), 1.1), ((CG, (AA, CG), UG), 1.0), ((CG, (AA, UU), CG), 1.5),
      ((CG, (AC, AA), CG), 0.6), ((CG, (AC, AC), CG), 1.5), ((CG, (AC, AG), CG), 0.1), ((CG, (AC, CG), AU), 1.5), ((CG, (AC, CA), CG), 0.5), ((CG, (AC, CC), CG), 1.5), ((CG, (AC, CG), CG), 1.4), ((CG, (AC, CU), CG), 1.5), ((CG, (AC, GA), CG), 0.3), ((CG, (AC, CG), GC), 1.5), ((CG, (AC, GG), CG), -0.3), ((CG, (AC, CG), GU), 1.5), ((CG, (AC, CG), UA), 1.4), ((CG, (AC, UC), CG), 1.5), ((CG, (AC, CG), UG), 1.4), ((CG, (AC, UU), CG), 0.0),
      ((CG, (AG, AA), CG), 0.0), ((CG, (AG, AC), CG), -0.7), ((CG, (AG, AG), CG), -1.6), ((CG, (AG, CG), AU), 0.1), ((CG, (AG, CA), CG), -1.0), ((CG, (AG, CC), CG), -0.6), ((CG, (AG, CG), CG), 0.1), ((CG, (AG, CU), CG), 0.7), ((CG, (AG, GA), CG), -0.7), ((CG, (AG, CG), GC), 0.1), ((CG, (AG, GG), CG), 0.0), ((CG, (AG, CG), GU), 0.1), ((CG, (AG, CG), UA), 0.1), ((CG, (AG, UC), CG), 0.8), ((CG, (AG, CG), UG), 0.1), ((CG, (AG, UU), CG), 0.9),
      ((CG, (AU, AA), CG), 1.7), ((CG, (AU, AC), CG), 1.5), ((CG, (AU, AG), CG), -0.3), ((CG, (AU, CG), AU), 1.5), ((CG, (AU, CA), CG), 1.4), ((CG, (AU, CC), CG), 1.5), ((CG, (AU, CG), CG), 1.4), ((CG, (AU, CU), CG), 1.5), ((CG, (AU, GA), CG), 0.3), ((CG, (AU, CG), GC), 1.5), ((CG, (AU, GG), CG), 0.1), ((CG, (AU, CG), GU), 1.5), ((CG, (AU, CG), UA), 1.4), ((CG, (AU, UC), CG), 1.5), ((CG, (AU, CG), UG), 1.4), ((CG, (AU, UU), CG), 0.7),
      ((CG, (CA, AA), CG), 1.3), ((CG, (CA, AC), CG), 1.0), ((CG, (CA, AG), CG), -0.7), ((CG, (CA, CG), AU), 1.0), ((CG, (CA, CA), CG), 1.1), ((CG, (CA, CC), CG), 1.0), ((CG, (CA, CG), CG), 1.0), ((CG, (CA, CU), CG), 1.0), ((CG, (CA, GA), CG), 0.7), ((CG, (CA, CG), GC), 1.0), ((CG, (CA, GG), CG), -0.4), ((CG, (CA, CG), GU), 1.0), ((CG, (CA, CG), UA), 1.0), ((CG, (CA, UC), CG), 1.1), ((CG, (CA, CG), UG), 1.0), ((CG, (CA, UU), CG), -0.2),
      ((CG, (CC, AA), CG), 2.2), ((CG, (CC, AC), CG), 1.3), ((CG, (CC, AG), CG), 0.7), ((CG, (CC, CG), AU), 1.3), ((CG, (CC, CA), CG), 1.9), ((CG, (CC, CC), CG), 1.3), ((CG, (CC, CG), CG), 1.3), ((CG, (CC, CU), CG), 1.3), ((CG, (CC, GA), CG), 0.7), ((CG, (CC, CG), GC), 1.3), ((CG, (CC, GG), CG), -0.1), ((CG, (CC, CG), GU), 1.3), ((CG, (CC, CG), UA), 1.3), ((CG, (CC, UC), CG), 1.4), ((CG, (CC, CG), UG), 1.3), ((CG, (CC, UU), CG), -0.1),
      ((CG, (CG, AA), CG), 1.3), ((CG, (CG, AC), CG), 1.0), ((CG, (CG, AG), CG), -0.7), ((CG, (CG, CG), AU), 1.0), ((CG, (CG, CA), CG), 1.0), ((CG, (CG, CC), CG), 1.0), ((CG, (CG, CG), CG), 1.0), ((CG, (CG, CU), CG), 1.0), ((CG, (CG, GA), CG), -0.1), ((CG, (CG, CG), GC), 1.0), ((CG, (CG, GG), CG), -0.4), ((CG, (CG, CG), GU), 1.0), ((CG, (CG, CG), UA), 1.0), ((CG, (CG, UC), CG), 1.1), ((CG, (CG, CG), UG), 1.0), ((CG, (CG, UU), CG), 0.3),
      ((CG, (CU, AA), CG), 1.4), ((CG, (CU, AC), CG), 1.2), ((CG, (CU, AG), CG), 0.0), ((CG, (CU, CG), AU), 1.2), ((CG, (CU, CA), CG), 1.1), ((CG, (CU, CC), CG), 1.2), ((CG, (CU, CG), CG), 1.1), ((CG, (CU, CU), CG), 1.7), ((CG, (CU, GA), CG), 0.6), ((CG, (CU, CG), GC), 1.2), ((CG, (CU, GG), CG), 0.2), ((CG, (CU, CG), GU), 1.2), ((CG, (CU, CG), UA), 1.1), ((CG, (CU, UC), CG), 1.2), ((CG, (CU, CG), UG), 1.1), ((CG, (CU, UU), CG), 0.2),
      ((CG, (GA, AA), CG), -0.2), ((CG, (GA, AC), CG), -0.4), ((CG, (GA, AG), CG), -1.7), ((CG, (GA, CG), AU), 0.1), ((CG, (GA, CA), CG), 0.7), ((CG, (GA, CC), CG), 0.7), ((CG, (GA, CG), CG), 0.1), ((CG, (GA, CU), CG), 0.7), ((CG, (GA, GA), CG), -0.5), ((CG, (GA, CG), GC), 0.1), ((CG, (GA, GG), CG), -0.3), ((CG, (GA, CG), GU), 0.1), ((CG, (GA, CG), UA), 0.1), ((CG, (GA, UC), CG), 0.8), ((CG, (GA, CG), UG), 0.1), ((CG, (GA, UU), CG), 0.9),
      ((CG, (GC, AA), CG), 1.7), ((CG, (GC, AC), CG), 1.5), ((CG, (GC, AG), CG), -0.3), ((CG, (GC, CG), AU), 1.5), ((CG, (GC, CA), CG), 1.4), ((CG, (GC, CC), CG), 1.5), ((CG, (GC, CG), CG), 1.4), ((CG, (GC, CU), CG), 1.5), ((CG, (GC, GA), CG), 0.3), ((CG, (GC, CG), GC), 1.5), ((CG, (GC, GG), CG), 0.1), ((CG, (GC, CG), GU), 1.5), ((CG, (GC, CG), UA), 1.4), ((CG, (GC, UC), CG), 1.5), ((CG, (GC, CG), UG), 1.4), ((CG, (GC, UU), CG), 0.7),
      ((CG, (GG, AA), CG), -0.1), ((CG, (GG, AC), CG), -0.4), ((CG, (GG, AG), CG), -0.9), ((CG, (GG, CG), AU), -0.4), ((CG, (GG, CA), CG), -0.5), ((CG, (GG, CC), CG), -0.4), ((CG, (GG, CG), CG), -0.5), ((CG, (GG, CU), CG), 0.2), ((CG, (GG, GA), CG), -0.3), ((CG, (GG, CG), GC), -0.4), ((CG, (GG, GG), CG), 0.8), ((CG, (GG, CG), GU), -0.4), ((CG, (GG, CG), UA), -0.5), ((CG, (GG, UC), CG), -0.5), ((CG, (GG, CG), UG), -0.5), ((CG, (GG, UU), CG), 1.4),
      ((CG, (GU, AA), CG), 1.7), ((CG, (GU, AC), CG), 1.5), ((CG, (GU, AG), CG), -0.3), ((CG, (GU, CG), AU), 1.5), ((CG, (GU, CA), CG), 1.4), ((CG, (GU, CC), CG), 1.5), ((CG, (GU, CG), CG), 1.4), ((CG, (GU, CU), CG), 1.5), ((CG, (GU, GA), CG), 0.3), ((CG, (GU, CG), GC), 1.5), ((CG, (GU, GG), CG), 0.1), ((CG, (GU, CG), GU), 1.5), ((CG, (GU, CG), UA), 1.4), ((CG, (GU, UC), CG), 1.5), ((CG, (GU, CG), UG), 1.4), ((CG, (GU, UU), CG), 0.7),
      ((CG, (UA, AA), CG), 1.3), ((CG, (UA, AC), CG), 1.0), ((CG, (UA, AG), CG), -0.7), ((CG, (UA, CG), AU), 1.0), ((CG, (UA, CA), CG), 1.0), ((CG, (UA, CC), CG), 1.0), ((CG, (UA, CG), CG), 1.0), ((CG, (UA, CU), CG), 1.0), ((CG, (UA, GA), CG), -0.1), ((CG, (UA, CG), GC), 1.0), ((CG, (UA, GG), CG), -0.4), ((CG, (UA, CG), GU), 1.0), ((CG, (UA, CG), UA), 1.0), ((CG, (UA, UC), CG), 1.1), ((CG, (UA, CG), UG), 1.0), ((CG, (UA, UU), CG), 0.3),
      ((CG, (UC, AA), CG), 1.4), ((CG, (UC, AC), CG), 1.2), ((CG, (UC, AG), CG), 0.0), ((CG, (UC, CG), AU), 1.2), ((CG, (UC, CA), CG), 1.1), ((CG, (UC, CC), CG), 1.2), ((CG, (UC, CG), CG), 1.1), ((CG, (UC, CU), CG), 1.2), ((CG, (UC, GA), CG), 0.5), ((CG, (UC, CG), GC), 1.2), ((CG, (UC, GG), CG), -0.6), ((CG, (UC, CG), GU), 1.2), ((CG, (UC, CG), UA), 1.1), ((CG, (UC, UC), CG), 1.2), ((CG, (UC, CG), UG), 1.1), ((CG, (UC, UU), CG), 0.4),
      ((CG, (UG, AA), CG), 1.3), ((CG, (UG, AC), CG), 1.0), ((CG, (UG, AG), CG), -0.7), ((CG, (UG, CG), AU), 1.0), ((CG, (UG, CA), CG), 1.0), ((CG, (UG, CC), CG), 1.0), ((CG, (UG, CG), CG), 1.0), ((CG, (UG, CU), CG), 1.0), ((CG, (UG, GA), CG), -0.1), ((CG, (UG, CG), GC), 1.0), ((CG, (UG, GG), CG), -0.4), ((CG, (UG, CG), GU), 1.0), ((CG, (UG, CG), UA), 1.0), ((CG, (UG, UC), CG), 1.1), ((CG, (UG, CG), UG), 1.0), ((CG, (UG, UU), CG), 0.3),
      ((CG, (UU, AA), CG), 1.4), ((CG, (UU, AC), CG), 0.3), ((CG, (UU, AG), CG), 0.5), ((CG, (UU, CG), AU), 0.3), ((CG, (UU, CA), CG), 0.3), ((CG, (UU, CC), CG), 0.3), ((CG, (UU, CG), CG), 0.2), ((CG, (UU, CU), CG), 0.3), ((CG, (UU, GA), CG), 1.4), ((CG, (UU, CG), GC), 0.3), ((CG, (UU, GG), CG), 0.7), ((CG, (UU, CG), GU), 0.3), ((CG, (UU, CG), UA), 0.2), ((CG, (UU, UC), CG), 0.3), ((CG, (UU, CG), UG), 0.2), ((CG, (UU, UU), CG), -0.6),
      // For internal loops between the base pairs "CG" and "GC".
      ((CG, (AA, AA), GC), 1.2), ((CG, (AA, AC), GC), 1.1), ((CG, (AA, AG), GC), 0.2), ((CG, (AA, GC), AU), 1.1), ((CG, (AA, CA), GC), 1.6), ((CG, (AA, CC), GC), 1.4), ((CG, (AA, GC), CG), 1.6), ((CG, (AA, CU), GC), 1.3), ((CG, (AA, GA), GC), 0.2), ((CG, (AA, GC), GC), 1.1), ((CG, (AA, GG), GC), -0.3), ((CG, (AA, GC), GU), 1.1), ((CG, (AA, GC), UA), 1.6), ((CG, (AA, UC), GC), 1.3), ((CG, (AA, GC), UG), 1.6), ((CG, (AA, UU), GC), 1.0),
      ((CG, (AC, AA), GC), 1.6), ((CG, (AC, AC), GC), 1.5), ((CG, (AC, AG), GC), 0.6), ((CG, (AC, GC), AU), 1.5), ((CG, (AC, CA), GC), 2.0), ((CG, (AC, CC), GC), 1.8), ((CG, (AC, GC), CG), 2.0), ((CG, (AC, CU), GC), 1.7), ((CG, (AC, GA), GC), 0.6), ((CG, (AC, GC), GC), 1.5), ((CG, (AC, GG), GC), 0.1), ((CG, (AC, GC), GU), 1.5), ((CG, (AC, GC), UA), 2.0), ((CG, (AC, UC), GC), 1.7), ((CG, (AC, GC), UG), 2.0), ((CG, (AC, UU), GC), 0.8),
      ((CG, (AG, AA), GC), 0.2), ((CG, (AG, AC), GC), 0.2), ((CG, (AG, AG), GC), -0.7), ((CG, (AG, GC), AU), 0.2), ((CG, (AG, CA), GC), 0.6), ((CG, (AG, CC), GC), 1.1), ((CG, (AG, GC), CG), 0.6), ((CG, (AG, CU), GC), 0.9), ((CG, (AG, GA), GC), -0.7), ((CG, (AG, GC), GC), 0.2), ((CG, (AG, GG), GC), 0.0), ((CG, (AG, GC), GU), 0.2), ((CG, (AG, GC), UA), 0.6), ((CG, (AG, UC), GC), 0.9), ((CG, (AG, GC), UG), 0.6), ((CG, (AG, UU), GC), -0.5),
      ((CG, (AU, AA), GC), 1.6), ((CG, (AU, AC), GC), 1.5), ((CG, (AU, AG), GC), 0.6), ((CG, (AU, GC), AU), 1.5), ((CG, (AU, CA), GC), 2.0), ((CG, (AU, CC), GC), 1.8), ((CG, (AU, GC), CG), 2.0), ((CG, (AU, CU), GC), 1.7), ((CG, (AU, GA), GC), 0.6), ((CG, (AU, GC), GC), 1.5), ((CG, (AU, GG), GC), 0.1), ((CG, (AU, GC), GU), 1.5), ((CG, (AU, GC), UA), 2.0), ((CG, (AU, UC), GC), 1.7), ((CG, (AU, GC), UG), 2.0), ((CG, (AU, UU), GC), 0.8),
      ((CG, (CA, AA), GC), 1.6), ((CG, (CA, AC), GC), 1.5), ((CG, (CA, AG), GC), 0.6), ((CG, (CA, GC), AU), 1.5), ((CG, (CA, CA), GC), 2.0), ((CG, (CA, CC), GC), 1.8), ((CG, (CA, GC), CG), 2.0), ((CG, (CA, CU), GC), 1.7), ((CG, (CA, GA), GC), 0.6), ((CG, (CA, GC), GC), 1.5), ((CG, (CA, GG), GC), 0.1), ((CG, (CA, GC), GU), 1.5), ((CG, (CA, GC), UA), 2.0), ((CG, (CA, UC), GC), 1.7), ((CG, (CA, GC), UG), 2.0), ((CG, (CA, UU), GC), 0.8),
      ((CG, (CC, AA), GC), 1.1), ((CG, (CC, AC), GC), 1.1), ((CG, (CC, AG), GC), 0.2), ((CG, (CC, GC), AU), 1.1), ((CG, (CC, CA), GC), 1.5), ((CG, (CC, CC), GC), 1.4), ((CG, (CC, GC), CG), 1.5), ((CG, (CC, CU), GC), 1.2), ((CG, (CC, GA), GC), 0.2), ((CG, (CC, GC), GC), 1.1), ((CG, (CC, GG), GC), -0.4), ((CG, (CC, GC), GU), 1.1), ((CG, (CC, GC), UA), 1.5), ((CG, (CC, UC), GC), 1.2), ((CG, (CC, GC), UG), 1.5), ((CG, (CC, UU), GC), 0.3),
      ((CG, (CG, AA), GC), 1.4), ((CG, (CG, AC), GC), 1.4), ((CG, (CG, AG), GC), 1.1), ((CG, (CG, GC), AU), 1.4), ((CG, (CG, CA), GC), 1.8), ((CG, (CG, CC), GC), 1.7), ((CG, (CG, GC), CG), 1.8), ((CG, (CG, CU), GC), 1.5), ((CG, (CG, GA), GC), 1.1), ((CG, (CG, GC), GC), 1.4), ((CG, (CG, GG), GC), -0.1), ((CG, (CG, GC), GU), 1.4), ((CG, (CG, GC), UA), 1.8), ((CG, (CG, UC), GC), 1.5), ((CG, (CG, GC), UG), 1.8), ((CG, (CG, UU), GC), 0.6),
      ((CG, (CU, AA), GC), 1.1), ((CG, (CU, AC), GC), 1.1), ((CG, (CU, AG), GC), 0.2), ((CG, (CU, GC), AU), 1.1), ((CG, (CU, CA), GC), 1.5), ((CG, (CU, CC), GC), 1.4), ((CG, (CU, GC), CG), 1.5), ((CG, (CU, CU), GC), 1.2), ((CG, (CU, GA), GC), 0.2), ((CG, (CU, GC), GC), 1.1), ((CG, (CU, GG), GC), -0.4), ((CG, (CU, GC), GU), 1.1), ((CG, (CU, GC), UA), 1.5), ((CG, (CU, UC), GC), 1.2), ((CG, (CU, GC), UG), 1.5), ((CG, (CU, UU), GC), 0.3),
      ((CG, (GA, AA), GC), 1.3), ((CG, (GA, AC), GC), 1.2), ((CG, (GA, AG), GC), 0.9), ((CG, (GA, GC), AU), 1.2), ((CG, (GA, CA), GC), 1.7), ((CG, (GA, CC), GC), 1.5), ((CG, (GA, GC), CG), 1.7), ((CG, (GA, CU), GC), 1.4), ((CG, (GA, GA), GC), 0.9), ((CG, (GA, GC), GC), 1.2), ((CG, (GA, GG), GC), -0.2), ((CG, (GA, GC), GU), 1.2), ((CG, (GA, GC), UA), 1.7), ((CG, (GA, UC), GC), 1.4), ((CG, (GA, GC), UG), 1.7), ((CG, (GA, UU), GC), 0.5),
      ((CG, (GC, AA), GC), 0.2), ((CG, (GC, AC), GC), 0.2), ((CG, (GC, AG), GC), -0.7), ((CG, (GC, GC), AU), 0.2), ((CG, (GC, CA), GC), 0.6), ((CG, (GC, CC), GC), 1.1), ((CG, (GC, GC), CG), 0.6), ((CG, (GC, CU), GC), 0.9), ((CG, (GC, GA), GC), -0.7), ((CG, (GC, GC), GC), 0.2), ((CG, (GC, GG), GC), 0.0), ((CG, (GC, GC), GU), 0.2), ((CG, (GC, GC), UA), 0.6), ((CG, (GC, UC), GC), 0.9), ((CG, (GC, GC), UG), 0.6), ((CG, (GC, UU), GC), -0.5),
      ((CG, (GG, AA), GC), 1.6), ((CG, (GG, AC), GC), 1.5), ((CG, (GG, AG), GC), 0.6), ((CG, (GG, GC), AU), 1.5), ((CG, (GG, CA), GC), 2.0), ((CG, (GG, CC), GC), 1.8), ((CG, (GG, GC), CG), 2.0), ((CG, (GG, CU), GC), 1.7), ((CG, (GG, GA), GC), 0.6), ((CG, (GG, GC), GC), 1.5), ((CG, (GG, GG), GC), 0.1), ((CG, (GG, GC), GU), 1.5), ((CG, (GG, GC), UA), 2.0), ((CG, (GG, UC), GC), 1.7), ((CG, (GG, GC), UG), 2.0), ((CG, (GG, UU), GC), 0.8),
      ((CG, (GU, AA), GC), 1.1), ((CG, (GU, AC), GC), 1.1), ((CG, (GU, AG), GC), 0.2), ((CG, (GU, GC), AU), 1.1), ((CG, (GU, CA), GC), 1.5), ((CG, (GU, CC), GC), 1.4), ((CG, (GU, GC), CG), 1.5), ((CG, (GU, CU), GC), 1.2), ((CG, (GU, GA), GC), 0.2), ((CG, (GU, GC), GC), 1.1), ((CG, (GU, GG), GC), -0.4), ((CG, (GU, GC), GU), 1.1), ((CG, (GU, GC), UA), 1.5), ((CG, (GU, UC), GC), 1.2), ((CG, (GU, GC), UG), 1.5), ((CG, (GU, UU), GC), 0.3),
      ((CG, (UA, AA), GC), 1.3), ((CG, (UA, AC), GC), 1.2), ((CG, (UA, AG), GC), 0.9), ((CG, (UA, GC), AU), 1.2), ((CG, (UA, CA), GC), 1.7), ((CG, (UA, CC), GC), 1.5), ((CG, (UA, GC), CG), 1.7), ((CG, (UA, CU), GC), 1.4), ((CG, (UA, GA), GC), 0.9), ((CG, (UA, GC), GC), 1.2), ((CG, (UA, GG), GC), -0.2), ((CG, (UA, GC), GU), 1.2), ((CG, (UA, GC), UA), 1.7), ((CG, (UA, UC), GC), 1.4), ((CG, (UA, GC), UG), 1.7), ((CG, (UA, UU), GC), 0.5),
      ((CG, (UC, AA), GC), 1.1), ((CG, (UC, AC), GC), 1.1), ((CG, (UC, AG), GC), 0.2), ((CG, (UC, GC), AU), 1.1), ((CG, (UC, CA), GC), 1.5), ((CG, (UC, CC), GC), 1.4), ((CG, (UC, GC), CG), 1.5), ((CG, (UC, CU), GC), 1.2), ((CG, (UC, GA), GC), 0.2), ((CG, (UC, GC), GC), 1.1), ((CG, (UC, GG), GC), -0.4), ((CG, (UC, GC), GU), 1.1), ((CG, (UC, GC), UA), 1.5), ((CG, (UC, UC), GC), 1.2), ((CG, (UC, GC), UG), 1.5), ((CG, (UC, UU), GC), 0.3),
      ((CG, (UG, AA), GC), 1.0), ((CG, (UG, AC), GC), 0.3), ((CG, (UG, AG), GC), -0.5), ((CG, (UG, GC), AU), 0.3), ((CG, (UG, CA), GC), 0.8), ((CG, (UG, CC), GC), 0.6), ((CG, (UG, GC), CG), 0.8), ((CG, (UG, CU), GC), 0.5), ((CG, (UG, GA), GC), -0.5), ((CG, (UG, GC), GC), 0.3), ((CG, (UG, GG), GC), 0.2), ((CG, (UG, GC), GU), 0.3), ((CG, (UG, GC), UA), 0.8), ((CG, (UG, UC), GC), 0.5), ((CG, (UG, GC), UG), 0.8), ((CG, (UG, UU), GC), -0.4),
      ((CG, (UU, AA), GC), 1.4), ((CG, (UU, AC), GC), 0.3), ((CG, (UU, AG), GC), 0.5), ((CG, (UU, GC), AU), 0.3), ((CG, (UU, CA), GC), 0.3), ((CG, (UU, CC), GC), 0.3), ((CG, (UU, GC), CG), 0.2), ((CG, (UU, CU), GC), 0.3), ((CG, (UU, GA), GC), 1.4), ((CG, (UU, GC), GC), 0.3), ((CG, (UU, GG), GC), 0.7), ((CG, (UU, GC), GU), 0.3), ((CG, (UU, GC), UA), 0.2), ((CG, (UU, UC), GC), 0.3), ((CG, (UU, GC), UG), 0.2), ((CG, (UU, UU), GC), -0.6),
      // For internal loops between the base pairs "CG" and "GU".
      ((CG, (AA, AA), GU), 1.6), ((CG, (AA, AC), GU), 2.0), ((CG, (AA, AG), GU), 0.6), ((CG, (AA, GU), AU), 2.0), ((CG, (AA, CA), GU), 2.0), ((CG, (AA, CC), GU), 2.0), ((CG, (AA, GU), CG), 2.0), ((CG, (AA, CU), GU), 2.0), ((CG, (AA, GA), GU), 2.3), ((CG, (AA, GU), GC), 2.0), ((CG, (AA, GG), GU), 0.7), ((CG, (AA, GU), GU), 2.0), ((CG, (AA, GU), UA), 2.0), ((CG, (AA, UC), GU), 2.0), ((CG, (AA, GU), UG), 2.0), ((CG, (AA, UU), GU), 2.6),
      ((CG, (AC, AA), GU), 2.0), ((CG, (AC, AC), GU), 2.4), ((CG, (AC, AG), GU), 1.0), ((CG, (AC, GU), AU), 2.4), ((CG, (AC, CA), GU), 2.4), ((CG, (AC, CC), GU), 2.4), ((CG, (AC, GU), CG), 2.4), ((CG, (AC, CU), GU), 2.4), ((CG, (AC, GA), GU), 2.7), ((CG, (AC, GU), GC), 2.4), ((CG, (AC, GG), GU), 1.1), ((CG, (AC, GU), GU), 2.4), ((CG, (AC, GU), UA), 2.4), ((CG, (AC, UC), GU), 2.4), ((CG, (AC, GU), UG), 2.4), ((CG, (AC, UU), GU), 2.4),
      ((CG, (AG, AA), GU), 0.7), ((CG, (AG, AC), GU), 1.0), ((CG, (AG, AG), GU), -0.3), ((CG, (AG, GU), AU), 1.0), ((CG, (AG, CA), GU), 1.0), ((CG, (AG, CC), GU), 1.6), ((CG, (AG, GU), CG), 1.0), ((CG, (AG, CU), GU), 1.6), ((CG, (AG, GA), GU), 1.3), ((CG, (AG, GU), GC), 1.0), ((CG, (AG, GG), GU), 1.0), ((CG, (AG, GU), GU), 1.0), ((CG, (AG, GU), UA), 1.0), ((CG, (AG, UC), GU), 1.6), ((CG, (AG, GU), UG), 1.0), ((CG, (AG, UU), GU), 1.0),
      ((CG, (AU, AA), GU), 2.0), ((CG, (AU, AC), GU), 2.4), ((CG, (AU, AG), GU), 1.0), ((CG, (AU, GU), AU), 2.4), ((CG, (AU, CA), GU), 2.4), ((CG, (AU, CC), GU), 2.4), ((CG, (AU, GU), CG), 2.4), ((CG, (AU, CU), GU), 2.4), ((CG, (AU, GA), GU), 2.7), ((CG, (AU, GU), GC), 2.4), ((CG, (AU, GG), GU), 1.1), ((CG, (AU, GU), GU), 2.4), ((CG, (AU, GU), UA), 2.4), ((CG, (AU, UC), GU), 2.4), ((CG, (AU, GU), UG), 2.4), ((CG, (AU, UU), GU), 2.4),
      ((CG, (CA, AA), GU), 1.6), ((CG, (CA, AC), GU), 1.9), ((CG, (CA, AG), GU), 0.6), ((CG, (CA, GU), AU), 1.9), ((CG, (CA, CA), GU), 1.9), ((CG, (CA, CC), GU), 1.9), ((CG, (CA, GU), CG), 1.9), ((CG, (CA, CU), GU), 1.9), ((CG, (CA, GA), GU), 2.2), ((CG, (CA, GU), GC), 1.9), ((CG, (CA, GG), GU), 0.6), ((CG, (CA, GU), GU), 1.9), ((CG, (CA, GU), UA), 1.9), ((CG, (CA, UC), GU), 1.9), ((CG, (CA, GU), UG), 1.9), ((CG, (CA, UU), GU), 1.9),
      ((CG, (CC, AA), GU), 1.9), ((CG, (CC, AC), GU), 2.2), ((CG, (CC, AG), GU), 1.5), ((CG, (CC, GU), AU), 2.2), ((CG, (CC, CA), GU), 2.2), ((CG, (CC, CC), GU), 2.2), ((CG, (CC, GU), CG), 2.2), ((CG, (CC, CU), GU), 2.2), ((CG, (CC, GA), GU), 3.1), ((CG, (CC, GU), GC), 2.2), ((CG, (CC, GG), GU), 0.9), ((CG, (CC, GU), GU), 2.2), ((CG, (CC, GU), UA), 2.2), ((CG, (CC, UC), GU), 2.2), ((CG, (CC, GU), UG), 2.2), ((CG, (CC, UU), GU), 2.2),
      ((CG, (CG, AA), GU), 1.6), ((CG, (CG, AC), GU), 1.9), ((CG, (CG, AG), GU), 0.6), ((CG, (CG, GU), AU), 1.9), ((CG, (CG, CA), GU), 1.9), ((CG, (CG, CC), GU), 1.9), ((CG, (CG, GU), CG), 1.9), ((CG, (CG, CU), GU), 1.9), ((CG, (CG, GA), GU), 2.2), ((CG, (CG, GU), GC), 1.9), ((CG, (CG, GG), GU), 0.6), ((CG, (CG, GU), GU), 1.9), ((CG, (CG, GU), UA), 1.9), ((CG, (CG, UC), GU), 1.9), ((CG, (CG, GU), UG), 1.9), ((CG, (CG, UU), GU), 1.9),
      ((CG, (CU, AA), GU), 1.7), ((CG, (CU, AC), GU), 2.1), ((CG, (CU, AG), GU), 1.3), ((CG, (CU, GU), AU), 2.1), ((CG, (CU, CA), GU), 2.1), ((CG, (CU, CC), GU), 2.1), ((CG, (CU, GU), CG), 2.1), ((CG, (CU, CU), GU), 2.1), ((CG, (CU, GA), GU), 3.0), ((CG, (CU, GU), GC), 2.1), ((CG, (CU, GG), GU), 0.8), ((CG, (CU, GU), GU), 2.1), ((CG, (CU, GU), UA), 2.1), ((CG, (CU, UC), GU), 2.1), ((CG, (CU, GU), UG), 2.1), ((CG, (CU, UU), GU), 2.1),
      ((CG, (GA, AA), GU), 0.7), ((CG, (GA, AC), GU), 1.0), ((CG, (GA, AG), GU), -0.3), ((CG, (GA, GU), AU), 1.0), ((CG, (GA, CA), GU), 1.0), ((CG, (GA, CC), GU), 1.6), ((CG, (GA, GU), CG), 1.0), ((CG, (GA, CU), GU), 1.6), ((CG, (GA, GA), GU), 1.3), ((CG, (GA, GU), GC), 1.0), ((CG, (GA, GG), GU), 1.0), ((CG, (GA, GU), GU), 1.0), ((CG, (GA, GU), UA), 1.0), ((CG, (GA, UC), GU), 1.6), ((CG, (GA, GU), UG), 1.0), ((CG, (GA, UU), GU), 1.0),
      ((CG, (GC, AA), GU), 2.0), ((CG, (GC, AC), GU), 2.4), ((CG, (GC, AG), GU), 1.0), ((CG, (GC, GU), AU), 2.4), ((CG, (GC, CA), GU), 2.4), ((CG, (GC, CC), GU), 2.4), ((CG, (GC, GU), CG), 2.4), ((CG, (GC, CU), GU), 2.4), ((CG, (GC, GA), GU), 2.7), ((CG, (GC, GU), GC), 2.4), ((CG, (GC, GG), GU), 1.1), ((CG, (GC, GU), GU), 2.4), ((CG, (GC, GU), UA), 2.4), ((CG, (GC, UC), GU), 2.4), ((CG, (GC, GU), UG), 2.4), ((CG, (GC, UU), GU), 2.4),
      ((CG, (GG, AA), GU), 0.1), ((CG, (GG, AC), GU), 0.5), ((CG, (GG, AG), GU), 0.4), ((CG, (GG, GU), AU), 0.5), ((CG, (GG, CA), GU), 0.5), ((CG, (GG, CC), GU), 0.5), ((CG, (GG, GU), CG), 0.5), ((CG, (GG, CU), GU), 0.5), ((CG, (GG, GA), GU), 2.1), ((CG, (GG, GU), GC), 0.5), ((CG, (GG, GG), GU), 1.8), ((CG, (GG, GU), GU), 0.5), ((CG, (GG, GU), UA), 0.5), ((CG, (GG, UC), GU), 0.5), ((CG, (GG, GU), UG), 0.5), ((CG, (GG, UU), GU), 1.8),
      ((CG, (GU, AA), GU), 2.0), ((CG, (GU, AC), GU), 2.4), ((CG, (GU, AG), GU), 1.0), ((CG, (GU, GU), AU), 2.4), ((CG, (GU, CA), GU), 2.4), ((CG, (GU, CC), GU), 2.4), ((CG, (GU, GU), CG), 2.4), ((CG, (GU, CU), GU), 2.4), ((CG, (GU, GA), GU), 2.7), ((CG, (GU, GU), GC), 2.4), ((CG, (GU, GG), GU), 1.1), ((CG, (GU, GU), GU), 2.4), ((CG, (GU, GU), UA), 2.4), ((CG, (GU, UC), GU), 2.4), ((CG, (GU, GU), UG), 2.4), ((CG, (GU, UU), GU), 2.4),
      ((CG, (UA, AA), GU), 1.6), ((CG, (UA, AC), GU), 1.9), ((CG, (UA, AG), GU), 0.6), ((CG, (UA, GU), AU), 1.9), ((CG, (UA, CA), GU), 1.9), ((CG, (UA, CC), GU), 1.9), ((CG, (UA, GU), CG), 1.9), ((CG, (UA, CU), GU), 1.9), ((CG, (UA, GA), GU), 2.2), ((CG, (UA, GU), GC), 1.9), ((CG, (UA, GG), GU), 0.6), ((CG, (UA, GU), GU), 1.9), ((CG, (UA, GU), UA), 1.9), ((CG, (UA, UC), GU), 1.9), ((CG, (UA, GU), UG), 1.9), ((CG, (UA, UU), GU), 1.9),
      ((CG, (UC, AA), GU), 1.7), ((CG, (UC, AC), GU), 2.1), ((CG, (UC, AG), GU), 1.3), ((CG, (UC, GU), AU), 2.1), ((CG, (UC, CA), GU), 2.1), ((CG, (UC, CC), GU), 2.1), ((CG, (UC, GU), CG), 2.1), ((CG, (UC, CU), GU), 2.1), ((CG, (UC, GA), GU), 3.0), ((CG, (UC, GU), GC), 2.1), ((CG, (UC, GG), GU), 0.8), ((CG, (UC, GU), GU), 2.1), ((CG, (UC, GU), UA), 2.1), ((CG, (UC, UC), GU), 2.1), ((CG, (UC, GU), UG), 2.1), ((CG, (UC, UU), GU), 2.1),
      ((CG, (UG, AA), GU), 1.6), ((CG, (UG, AC), GU), 1.9), ((CG, (UG, AG), GU), 0.6), ((CG, (UG, GU), AU), 1.9), ((CG, (UG, CA), GU), 1.9), ((CG, (UG, CC), GU), 1.9), ((CG, (UG, GU), CG), 1.9), ((CG, (UG, CU), GU), 1.9), ((CG, (UG, GA), GU), 2.2), ((CG, (UG, GU), GC), 1.9), ((CG, (UG, GG), GU), 0.6), ((CG, (UG, GU), GU), 1.9), ((CG, (UG, GU), UA), 1.9), ((CG, (UG, UC), GU), 1.9), ((CG, (UG, GU), UG), 1.9), ((CG, (UG, UU), GU), 1.9),
      ((CG, (UU, AA), GU), 1.4), ((CG, (UU, AC), GU), 1.2), ((CG, (UU, AG), GU), -0.1), ((CG, (UU, GU), AU), 1.2), ((CG, (UU, CA), GU), 1.2), ((CG, (UU, CC), GU), 1.2), ((CG, (UU, GU), CG), 1.2), ((CG, (UU, CU), GU), 1.2), ((CG, (UU, GA), GU), 1.5), ((CG, (UU, GU), GC), 1.2), ((CG, (UU, GG), GU), 1.2), ((CG, (UU, GU), GU), 1.2), ((CG, (UU, GU), UA), 1.2), ((CG, (UU, UC), GU), 1.2), ((CG, (UU, GU), UG), 1.2), ((CG, (UU, UU), GU), 1.2),
      // For internal loops between the base pairs "CG" and "UA".
      ((CG, (AA, AA), UA), 2.0), ((CG, (AA, AC), UA), 1.7), ((CG, (AA, AG), UA), 0.7), ((CG, (AA, UA), AU), 1.7), ((CG, (AA, CA), UA), 1.8), ((CG, (AA, CC), UA), 1.8), ((CG, (AA, UA), CG), 1.8), ((CG, (AA, CU), UA), 1.8), ((CG, (AA, GA), UA), 1.4), ((CG, (AA, UA), GC), 1.7), ((CG, (AA, GG), UA), 0.2), ((CG, (AA, UA), GU), 1.7), ((CG, (AA, UA), UA), 1.8), ((CG, (AA, UC), UA), 1.8), ((CG, (AA, UA), UG), 1.8), ((CG, (AA, UU), UA), 1.5),
      ((CG, (AC, AA), UA), 2.4), ((CG, (AC, AC), UA), 2.1), ((CG, (AC, AG), UA), 1.1), ((CG, (AC, UA), AU), 2.1), ((CG, (AC, CA), UA), 2.2), ((CG, (AC, CC), UA), 2.2), ((CG, (AC, UA), CG), 2.2), ((CG, (AC, CU), UA), 2.2), ((CG, (AC, GA), UA), 1.8), ((CG, (AC, UA), GC), 2.1), ((CG, (AC, GG), UA), 0.6), ((CG, (AC, UA), GU), 2.1), ((CG, (AC, UA), UA), 2.2), ((CG, (AC, UC), UA), 2.2), ((CG, (AC, UA), UG), 2.2), ((CG, (AC, UU), UA), 1.3),
      ((CG, (AG, AA), UA), 1.0), ((CG, (AG, AC), UA), 0.8), ((CG, (AG, AG), UA), -0.2), ((CG, (AG, UA), AU), 0.8), ((CG, (AG, CA), UA), 0.9), ((CG, (AG, CC), UA), 1.4), ((CG, (AG, UA), CG), 0.9), ((CG, (AG, CU), UA), 1.4), ((CG, (AG, GA), UA), 0.5), ((CG, (AG, UA), GC), 0.8), ((CG, (AG, GG), UA), 0.6), ((CG, (AG, UA), GU), 0.8), ((CG, (AG, UA), UA), 0.9), ((CG, (AG, UC), UA), 1.4), ((CG, (AG, UA), UG), 0.9), ((CG, (AG, UU), UA), 0.0),
      ((CG, (AU, AA), UA), 2.4), ((CG, (AU, AC), UA), 2.1), ((CG, (AU, AG), UA), 1.1), ((CG, (AU, UA), AU), 2.1), ((CG, (AU, CA), UA), 2.2), ((CG, (AU, CC), UA), 2.2), ((CG, (AU, UA), CG), 2.2), ((CG, (AU, CU), UA), 2.2), ((CG, (AU, GA), UA), 1.8), ((CG, (AU, UA), GC), 2.1), ((CG, (AU, GG), UA), 0.6), ((CG, (AU, UA), GU), 2.1), ((CG, (AU, UA), UA), 2.2), ((CG, (AU, UC), UA), 2.2), ((CG, (AU, UA), UG), 2.2), ((CG, (AU, UU), UA), 1.3),
      ((CG, (CA, AA), UA), 1.9), ((CG, (CA, AC), UA), 1.7), ((CG, (CA, AG), UA), 0.7), ((CG, (CA, UA), AU), 1.7), ((CG, (CA, CA), UA), 1.8), ((CG, (CA, CC), UA), 1.7), ((CG, (CA, UA), CG), 1.8), ((CG, (CA, CU), UA), 1.7), ((CG, (CA, GA), UA), 1.4), ((CG, (CA, UA), GC), 1.7), ((CG, (CA, GG), UA), 0.2), ((CG, (CA, UA), GU), 1.7), ((CG, (CA, UA), UA), 1.8), ((CG, (CA, UC), UA), 1.7), ((CG, (CA, UA), UG), 1.8), ((CG, (CA, UU), UA), 0.8),
      ((CG, (CC, AA), UA), 2.2), ((CG, (CC, AC), UA), 2.0), ((CG, (CC, AG), UA), 1.6), ((CG, (CC, UA), AU), 2.0), ((CG, (CC, CA), UA), 2.1), ((CG, (CC, CC), UA), 2.0), ((CG, (CC, UA), CG), 2.1), ((CG, (CC, CU), UA), 2.0), ((CG, (CC, GA), UA), 2.3), ((CG, (CC, UA), GC), 2.0), ((CG, (CC, GG), UA), 0.5), ((CG, (CC, UA), GU), 2.0), ((CG, (CC, UA), UA), 2.1), ((CG, (CC, UC), UA), 2.0), ((CG, (CC, UA), UG), 2.1), ((CG, (CC, UU), UA), 1.1),
      ((CG, (CG, AA), UA), 1.9), ((CG, (CG, AC), UA), 1.7), ((CG, (CG, AG), UA), 0.7), ((CG, (CG, UA), AU), 1.7), ((CG, (CG, CA), UA), 1.8), ((CG, (CG, CC), UA), 1.7), ((CG, (CG, UA), CG), 1.8), ((CG, (CG, CU), UA), 1.7), ((CG, (CG, GA), UA), 1.4), ((CG, (CG, UA), GC), 1.7), ((CG, (CG, GG), UA), 0.2), ((CG, (CG, UA), GU), 1.7), ((CG, (CG, UA), UA), 1.8), ((CG, (CG, UC), UA), 1.7), ((CG, (CG, UA), UG), 1.8), ((CG, (CG, UU), UA), 0.8),
      ((CG, (CU, AA), UA), 2.1), ((CG, (CU, AC), UA), 1.8), ((CG, (CU, AG), UA), 1.4), ((CG, (CU, UA), AU), 1.8), ((CG, (CU, CA), UA), 1.9), ((CG, (CU, CC), UA), 1.9), ((CG, (CU, UA), CG), 1.9), ((CG, (CU, CU), UA), 1.9), ((CG, (CU, GA), UA), 2.1), ((CG, (CU, UA), GC), 1.8), ((CG, (CU, GG), UA), 0.3), ((CG, (CU, UA), GU), 1.8), ((CG, (CU, UA), UA), 1.9), ((CG, (CU, UC), UA), 1.9), ((CG, (CU, UA), UG), 1.9), ((CG, (CU, UU), UA), 1.0),
      ((CG, (GA, AA), UA), 1.0), ((CG, (GA, AC), UA), 0.8), ((CG, (GA, AG), UA), -0.2), ((CG, (GA, UA), AU), 0.8), ((CG, (GA, CA), UA), 0.9), ((CG, (GA, CC), UA), 1.4), ((CG, (GA, UA), CG), 0.9), ((CG, (GA, CU), UA), 1.4), ((CG, (GA, GA), UA), 0.5), ((CG, (GA, UA), GC), 0.8), ((CG, (GA, GG), UA), 0.6), ((CG, (GA, UA), GU), 0.8), ((CG, (GA, UA), UA), 0.9), ((CG, (GA, UC), UA), 1.4), ((CG, (GA, UA), UG), 0.9), ((CG, (GA, UU), UA), 0.0),
      ((CG, (GC, AA), UA), 2.4), ((CG, (GC, AC), UA), 2.1), ((CG, (GC, AG), UA), 1.1), ((CG, (GC, UA), AU), 2.1), ((CG, (GC, CA), UA), 2.2), ((CG, (GC, CC), UA), 2.2), ((CG, (GC, UA), CG), 2.2), ((CG, (GC, CU), UA), 2.2), ((CG, (GC, GA), UA), 1.8), ((CG, (GC, UA), GC), 2.1), ((CG, (GC, GG), UA), 0.6), ((CG, (GC, UA), GU), 2.1), ((CG, (GC, UA), UA), 2.2), ((CG, (GC, UC), UA), 2.2), ((CG, (GC, UA), UG), 2.2), ((CG, (GC, UU), UA), 1.3),
      ((CG, (GG, AA), UA), 0.5), ((CG, (GG, AC), UA), 0.2), ((CG, (GG, AG), UA), 0.5), ((CG, (GG, UA), AU), 0.2), ((CG, (GG, CA), UA), 0.3), ((CG, (GG, CC), UA), 0.3), ((CG, (GG, UA), CG), 0.3), ((CG, (GG, CU), UA), 0.3), ((CG, (GG, GA), UA), 1.2), ((CG, (GG, UA), GC), 0.2), ((CG, (GG, GG), UA), 1.3), ((CG, (GG, UA), GU), 0.2), ((CG, (GG, UA), UA), 0.3), ((CG, (GG, UC), UA), 0.3), ((CG, (GG, UA), UG), 0.3), ((CG, (GG, UU), UA), 0.7),
      ((CG, (GU, AA), UA), 2.4), ((CG, (GU, AC), UA), 2.1), ((CG, (GU, AG), UA), 1.1), ((CG, (GU, UA), AU), 2.1), ((CG, (GU, CA), UA), 2.2), ((CG, (GU, CC), UA), 2.2), ((CG, (GU, UA), CG), 2.2), ((CG, (GU, CU), UA), 2.2), ((CG, (GU, GA), UA), 1.8), ((CG, (GU, UA), GC), 2.1), ((CG, (GU, GG), UA), 0.6), ((CG, (GU, UA), GU), 2.1), ((CG, (GU, UA), UA), 2.2), ((CG, (GU, UC), UA), 2.2), ((CG, (GU, UA), UG), 2.2), ((CG, (GU, UU), UA), 1.3),
      ((CG, (UA, AA), UA), 1.9), ((CG, (UA, AC), UA), 1.7), ((CG, (UA, AG), UA), 0.7), ((CG, (UA, UA), AU), 1.7), ((CG, (UA, CA), UA), 1.8), ((CG, (UA, CC), UA), 1.7), ((CG, (UA, UA), CG), 1.8), ((CG, (UA, CU), UA), 1.7), ((CG, (UA, GA), UA), 1.4), ((CG, (UA, UA), GC), 1.7), ((CG, (UA, GG), UA), 0.2), ((CG, (UA, UA), GU), 1.7), ((CG, (UA, UA), UA), 1.8), ((CG, (UA, UC), UA), 1.7), ((CG, (UA, UA), UG), 1.8), ((CG, (UA, UU), UA), 0.8),
      ((CG, (UC, AA), UA), 2.1), ((CG, (UC, AC), UA), 1.8), ((CG, (UC, AG), UA), 1.4), ((CG, (UC, UA), AU), 1.8), ((CG, (UC, CA), UA), 1.9), ((CG, (UC, CC), UA), 1.9), ((CG, (UC, UA), CG), 1.9), ((CG, (UC, CU), UA), 1.9), ((CG, (UC, GA), UA), 2.1), ((CG, (UC, UA), GC), 1.8), ((CG, (UC, GG), UA), 0.3), ((CG, (UC, UA), GU), 1.8), ((CG, (UC, UA), UA), 1.9), ((CG, (UC, UC), UA), 1.9), ((CG, (UC, UA), UG), 1.9), ((CG, (UC, UU), UA), 1.0),
      ((CG, (UG, AA), UA), 1.9), ((CG, (UG, AC), UA), 1.7), ((CG, (UG, AG), UA), 0.7), ((CG, (UG, UA), AU), 1.7), ((CG, (UG, CA), UA), 1.8), ((CG, (UG, CC), UA), 1.7), ((CG, (UG, UA), CG), 1.8), ((CG, (UG, CU), UA), 1.7), ((CG, (UG, GA), UA), 1.4), ((CG, (UG, UA), GC), 1.7), ((CG, (UG, GG), UA), 0.2), ((CG, (UG, UA), GU), 1.7), ((CG, (UG, UA), UA), 1.8), ((CG, (UG, UC), UA), 1.7), ((CG, (UG, UA), UG), 1.8), ((CG, (UG, UU), UA), 0.8),
      ((CG, (UU, AA), UA), 1.8), ((CG, (UU, AC), UA), 0.9), ((CG, (UU, AG), UA), 0.0), ((CG, (UU, UA), AU), 0.9), ((CG, (UU, CA), UA), 1.0), ((CG, (UU, CC), UA), 1.0), ((CG, (UU, UA), CG), 1.0), ((CG, (UU, CU), UA), 1.0), ((CG, (UU, GA), UA), 0.6), ((CG, (UU, UA), GC), 0.9), ((CG, (UU, GG), UA), 0.7), ((CG, (UU, UA), GU), 0.9), ((CG, (UU, UA), UA), 1.0), ((CG, (UU, UC), UA), 1.0), ((CG, (UU, UA), UG), 1.0), ((CG, (UU, UU), UA), 0.1),
      // For internal loops between the base pairs "CG" and "UG".
      ((CG, (AA, AA), UG), 2.7), ((CG, (AA, AC), UG), 2.3), ((CG, (AA, AG), UG), 1.5), ((CG, (AA, UG), AU), 2.3), ((CG, (AA, CA), UG), 2.3), ((CG, (AA, CC), UG), 2.3), ((CG, (AA, UG), CG), 2.3), ((CG, (AA, CU), UG), 2.3), ((CG, (AA, GA), UG), 1.9), ((CG, (AA, UG), GC), 2.3), ((CG, (AA, GG), UG), 1.0), ((CG, (AA, UG), GU), 2.3), ((CG, (AA, UG), UA), 2.3), ((CG, (AA, UC), UG), 2.3), ((CG, (AA, UG), UG), 2.3), ((CG, (AA, UU), UG), 2.9),
      ((CG, (AC, AA), UG), 3.0), ((CG, (AC, AC), UG), 2.7), ((CG, (AC, AG), UG), 1.9), ((CG, (AC, UG), AU), 2.7), ((CG, (AC, CA), UG), 2.7), ((CG, (AC, CC), UG), 2.7), ((CG, (AC, UG), CG), 2.7), ((CG, (AC, CU), UG), 2.7), ((CG, (AC, GA), UG), 2.3), ((CG, (AC, UG), GC), 2.7), ((CG, (AC, GG), UG), 1.4), ((CG, (AC, UG), GU), 2.7), ((CG, (AC, UG), UA), 2.7), ((CG, (AC, UC), UG), 2.7), ((CG, (AC, UG), UG), 2.7), ((CG, (AC, UU), UG), 2.7),
      ((CG, (AG, AA), UG), 1.7), ((CG, (AG, AC), UG), 1.3), ((CG, (AG, AG), UG), 0.5), ((CG, (AG, UG), AU), 1.3), ((CG, (AG, CA), UG), 1.3), ((CG, (AG, CC), UG), 1.9), ((CG, (AG, UG), CG), 1.3), ((CG, (AG, CU), UG), 1.9), ((CG, (AG, GA), UG), 0.9), ((CG, (AG, UG), GC), 1.3), ((CG, (AG, GG), UG), 1.3), ((CG, (AG, UG), GU), 1.3), ((CG, (AG, UG), UA), 1.3), ((CG, (AG, UC), UG), 1.9), ((CG, (AG, UG), UG), 1.3), ((CG, (AG, UU), UG), 1.3),
      ((CG, (AU, AA), UG), 3.0), ((CG, (AU, AC), UG), 2.7), ((CG, (AU, AG), UG), 1.9), ((CG, (AU, UG), AU), 2.7), ((CG, (AU, CA), UG), 2.7), ((CG, (AU, CC), UG), 2.7), ((CG, (AU, UG), CG), 2.7), ((CG, (AU, CU), UG), 2.7), ((CG, (AU, GA), UG), 2.3), ((CG, (AU, UG), GC), 2.7), ((CG, (AU, GG), UG), 1.4), ((CG, (AU, UG), GU), 2.7), ((CG, (AU, UG), UA), 2.7), ((CG, (AU, UC), UG), 2.7), ((CG, (AU, UG), UG), 2.7), ((CG, (AU, UU), UG), 2.7),
      ((CG, (CA, AA), UG), 2.6), ((CG, (CA, AC), UG), 2.2), ((CG, (CA, AG), UG), 1.4), ((CG, (CA, UG), AU), 2.2), ((CG, (CA, CA), UG), 2.2), ((CG, (CA, CC), UG), 2.2), ((CG, (CA, UG), CG), 2.2), ((CG, (CA, CU), UG), 2.2), ((CG, (CA, GA), UG), 1.8), ((CG, (CA, UG), GC), 2.2), ((CG, (CA, GG), UG), 0.9), ((CG, (CA, UG), GU), 2.2), ((CG, (CA, UG), UA), 2.2), ((CG, (CA, UC), UG), 2.2), ((CG, (CA, UG), UG), 2.2), ((CG, (CA, UU), UG), 2.2),
      ((CG, (CC, AA), UG), 2.9), ((CG, (CC, AC), UG), 2.5), ((CG, (CC, AG), UG), 2.3), ((CG, (CC, UG), AU), 2.5), ((CG, (CC, CA), UG), 2.5), ((CG, (CC, CC), UG), 2.5), ((CG, (CC, UG), CG), 2.5), ((CG, (CC, CU), UG), 2.5), ((CG, (CC, GA), UG), 2.7), ((CG, (CC, UG), GC), 2.5), ((CG, (CC, GG), UG), 1.2), ((CG, (CC, UG), GU), 2.5), ((CG, (CC, UG), UA), 2.5), ((CG, (CC, UC), UG), 2.5), ((CG, (CC, UG), UG), 2.5), ((CG, (CC, UU), UG), 2.5),
      ((CG, (CG, AA), UG), 2.6), ((CG, (CG, AC), UG), 2.2), ((CG, (CG, AG), UG), 1.4), ((CG, (CG, UG), AU), 2.2), ((CG, (CG, CA), UG), 2.2), ((CG, (CG, CC), UG), 2.2), ((CG, (CG, UG), CG), 2.2), ((CG, (CG, CU), UG), 2.2), ((CG, (CG, GA), UG), 1.8), ((CG, (CG, UG), GC), 2.2), ((CG, (CG, GG), UG), 0.9), ((CG, (CG, UG), GU), 2.2), ((CG, (CG, UG), UA), 2.2), ((CG, (CG, UC), UG), 2.2), ((CG, (CG, UG), UG), 2.2), ((CG, (CG, UU), UG), 2.2),
      ((CG, (CU, AA), UG), 2.7), ((CG, (CU, AC), UG), 2.4), ((CG, (CU, AG), UG), 2.2), ((CG, (CU, UG), AU), 2.4), ((CG, (CU, CA), UG), 2.4), ((CG, (CU, CC), UG), 2.4), ((CG, (CU, UG), CG), 2.4), ((CG, (CU, CU), UG), 2.4), ((CG, (CU, GA), UG), 2.6), ((CG, (CU, UG), GC), 2.4), ((CG, (CU, GG), UG), 1.1), ((CG, (CU, UG), GU), 2.4), ((CG, (CU, UG), UA), 2.4), ((CG, (CU, UC), UG), 2.4), ((CG, (CU, UG), UG), 2.4), ((CG, (CU, UU), UG), 2.4),
      ((CG, (GA, AA), UG), 1.7), ((CG, (GA, AC), UG), 1.3), ((CG, (GA, AG), UG), 0.5), ((CG, (GA, UG), AU), 1.3), ((CG, (GA, CA), UG), 1.3), ((CG, (GA, CC), UG), 1.9), ((CG, (GA, UG), CG), 1.3), ((CG, (GA, CU), UG), 1.9), ((CG, (GA, GA), UG), 0.9), ((CG, (GA, UG), GC), 1.3), ((CG, (GA, GG), UG), 1.3), ((CG, (GA, UG), GU), 1.3), ((CG, (GA, UG), UA), 1.3), ((CG, (GA, UC), UG), 1.9), ((CG, (GA, UG), UG), 1.3), ((CG, (GA, UU), UG), 1.3),
      ((CG, (GC, AA), UG), 3.0), ((CG, (GC, AC), UG), 2.7), ((CG, (GC, AG), UG), 1.9), ((CG, (GC, UG), AU), 2.7), ((CG, (GC, CA), UG), 2.7), ((CG, (GC, CC), UG), 2.2), ((CG, (GC, UG), CG), 2.7), ((CG, (GC, CU), UG), 2.7), ((CG, (GC, GA), UG), 2.3), ((CG, (GC, UG), GC), 2.7), ((CG, (GC, GG), UG), 1.4), ((CG, (GC, UG), GU), 2.7), ((CG, (GC, UG), UA), 2.7), ((CG, (GC, UC), UG), 2.7), ((CG, (GC, UG), UG), 2.7), ((CG, (GC, UU), UG), 2.7),
      ((CG, (GG, AA), UG), 1.1), ((CG, (GG, AC), UG), 0.8), ((CG, (GG, AG), UG), 1.3), ((CG, (GG, UG), AU), 0.8), ((CG, (GG, CA), UG), 0.8), ((CG, (GG, CC), UG), 0.8), ((CG, (GG, UG), CG), 0.8), ((CG, (GG, CU), UG), 0.8), ((CG, (GG, GA), UG), 1.7), ((CG, (GG, UG), GC), 0.8), ((CG, (GG, GG), UG), 2.1), ((CG, (GG, UG), GU), 0.8), ((CG, (GG, UG), UA), 0.8), ((CG, (GG, UC), UG), 0.8), ((CG, (GG, UG), UG), 0.8), ((CG, (GG, UU), UG), 2.1),
      ((CG, (GU, AA), UG), 3.0), ((CG, (GU, AC), UG), 2.7), ((CG, (GU, AG), UG), 1.9), ((CG, (GU, UG), AU), 2.7), ((CG, (GU, CA), UG), 2.7), ((CG, (GU, CC), UG), 2.7), ((CG, (GU, UG), CG), 2.7), ((CG, (GU, CU), UG), 2.7), ((CG, (GU, GA), UG), 2.3), ((CG, (GU, UG), GC), 2.7), ((CG, (GU, GG), UG), 1.4), ((CG, (GU, UG), GU), 2.7), ((CG, (GU, UG), UA), 2.7), ((CG, (GU, UC), UG), 2.7), ((CG, (GU, UG), UG), 2.7), ((CG, (GU, UU), UG), 2.7),
      ((CG, (UA, AA), UG), 2.6), ((CG, (UA, AC), UG), 2.2), ((CG, (UA, AG), UG), 1.4), ((CG, (UA, UG), AU), 2.2), ((CG, (UA, CA), UG), 2.2), ((CG, (UA, CC), UG), 2.2), ((CG, (UA, UG), CG), 2.2), ((CG, (UA, CU), UG), 2.2), ((CG, (UA, GA), UG), 1.8), ((CG, (UA, UG), GC), 2.2), ((CG, (UA, GG), UG), 0.9), ((CG, (UA, UG), GU), 2.2), ((CG, (UA, UG), UA), 2.2), ((CG, (UA, UC), UG), 2.2), ((CG, (UA, UG), UG), 2.2), ((CG, (UA, UU), UG), 2.2),
      ((CG, (UC, AA), UG), 2.7), ((CG, (UC, AC), UG), 2.4), ((CG, (UC, AG), UG), 2.2), ((CG, (UC, UG), AU), 2.4), ((CG, (UC, CA), UG), 2.4), ((CG, (UC, CC), UG), 2.4), ((CG, (UC, UG), CG), 2.4), ((CG, (UC, CU), UG), 2.4), ((CG, (UC, GA), UG), 2.6), ((CG, (UC, UG), GC), 2.4), ((CG, (UC, GG), UG), 1.1), ((CG, (UC, UG), GU), 2.4), ((CG, (UC, UG), UA), 2.4), ((CG, (UC, UC), UG), 2.4), ((CG, (UC, UG), UG), 2.4), ((CG, (UC, UU), UG), 2.4),
      ((CG, (UG, AA), UG), 2.6), ((CG, (UG, AC), UG), 2.2), ((CG, (UG, AG), UG), 1.4), ((CG, (UG, UG), AU), 2.2), ((CG, (UG, CA), UG), 2.2), ((CG, (UG, CC), UG), 2.2), ((CG, (UG, UG), CG), 2.2), ((CG, (UG, CU), UG), 2.2), ((CG, (UG, GA), UG), 1.8), ((CG, (UG, UG), GC), 2.2), ((CG, (UG, GG), UG), 0.9), ((CG, (UG, UG), GU), 2.2), ((CG, (UG, UG), UA), 2.2), ((CG, (UG, UC), UG), 2.2), ((CG, (UG, UG), UG), 2.2), ((CG, (UG, UU), UG), 2.2),
      ((CG, (UU, AA), UG), 2.4), ((CG, (UU, AC), UG), 1.5), ((CG, (UU, AG), UG), 0.7), ((CG, (UU, UG), AU), 1.5), ((CG, (UU, CA), UG), 1.5), ((CG, (UU, CC), UG), 1.5), ((CG, (UU, UG), CG), 1.5), ((CG, (UU, CU), UG), 1.5), ((CG, (UU, GA), UG), 1.1), ((CG, (UU, UG), GC), 1.5), ((CG, (UU, GG), UG), 1.5), ((CG, (UU, UG), GU), 1.5), ((CG, (UU, UG), UA), 1.5), ((CG, (UU, UC), UG), 1.5), ((CG, (UU, UG), UG), 1.5), ((CG, (UU, UU), UG), 1.5),
    ].iter().cloned().collect()
  };
}
