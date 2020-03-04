use utils::*;

pub type StackDeltaFes = [[[[FreeEnergy; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];

pub const HELIX_INTERMOLECULAR_INIT_DELTA_FE: FreeEnergy = - INVERSE_TEMPERATURE * 4.09;
pub const HELIX_AU_OR_GU_END_PENALTY_DELTA_FE: FreeEnergy = - INVERSE_TEMPERATURE * 0.45;
pub const HELIX_SYMMETRY_PENALTY_DELTA_FE: FreeEnergy = - INVERSE_TEMPERATURE * 0.43;

lazy_static! {
  pub static ref EXP_HELIX_INTERMOLECULAR_INIT_DELTA_FE: FreeEnergy = HELIX_INTERMOLECULAR_INIT_DELTA_FE.exp();
  pub static ref EXP_HELIX_AU_OR_GU_END_PENALTY_DELTA_FE: FreeEnergy = HELIX_AU_OR_GU_END_PENALTY_DELTA_FE.exp();
  pub static ref EXP_HELIX_SYMMETRY_PENALTY_DELTA_FE: FreeEnergy = HELIX_SYMMETRY_PENALTY_DELTA_FE.exp();
  pub static ref STACK_DELTA_FES: StackDeltaFes = {
    let mut stack_delta_fes = [[[[NEG_INFINITY; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
    for &(x, y) in [
      // For the base pair "AU" against which another base pair is stacked.
      ((AU, AU), -0.9), ((AU, CG), -2.2), ((AU, GC), -2.1), ((AU, GU), -0.6), ((AU, UA), -1.1), ((AU, UG), -1.4), 
      // For the base pair "CG" against which another base pair is stacked.
      ((CG, AU), -2.1), ((CG, CG), -3.3), ((CG, GC), -2.4), ((CG, GU), -1.4), ((CG, UA), -2.1), ((CG, UG), -2.1), 
      // For the base pair "GC" against which another base pair is stacked.
      ((GC, AU), -2.4), ((GC, CG), -3.4), ((GC, GC), -3.3), ((GC, GU), -1.5), ((GC, UA), -2.2), ((GC, UG), -2.5), 
      // For the base pair "GU" against which another base pair is stacked.
      ((GU, AU), -1.3), ((GU, CG), -2.5), ((GU, GC), -2.1), ((GU, GU), -0.5), ((GU, UA), -1.4), ((GU, UG), 1.3), 
      // For the base pair "UA" against which another base pair is stacked.
      ((UA, AU), -1.3), ((UA, CG), -2.4), ((UA, GC), -2.1), ((UA, GU), -1.0), ((UA, UA), -0.9), ((UA, UG), -1.3), 
      // For the base pair "UG" against which another base pair is stacked.
      ((UG, AU), -1.0), ((UG, CG), -1.5), ((UG, GC), -1.4), ((UG, GU), 0.3), ((UG, UA), -0.6), ((UG, UG), -0.5), 
    ].iter() {stack_delta_fes[(x.0).0][(x.0).1][(x.1).0][(x.1).1] = scale(y);}
    stack_delta_fes
  };
  pub static ref EXP_STACK_DELTA_FES: StackDeltaFes = {
    let mut exp_stack_delta_fes = STACK_DELTA_FES.clone();
    for fe_sets in &mut exp_stack_delta_fes {
      for fe_set in fe_sets {
        for fes in fe_set {
          for fe in fes {
            *fe = fe.exp();
          }
        }
      }
    }
    exp_stack_delta_fes
  };
}
