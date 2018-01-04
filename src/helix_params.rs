use utils::*;

pub type SDeltaFes = HashMap<(BasePair, BasePair), FreeEnergy, Hasher>;

pub const INTERMOLECULAR_INIT_DELTA_FE: FreeEnergy = 4.09;
pub const AU_OR_GU_END_PENALTY_DELTA_FE: FreeEnergy = 0.45;
pub const SYMMETRY_PENALTY_DELTA_FE: FreeEnergy = 0.43;

lazy_static! {
  pub static ref S_DELTA_FES: SDeltaFes = {
    [
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
    ].iter().cloned().collect()
  };
}