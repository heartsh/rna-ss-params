use utils::*;

/* pub type TmDeltaFes = [[[[FreeEnergy; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];

lazy_static! {
  pub static ref TM_DELTA_FES: TmDeltaFes = {
    let mut tm_delta_fes = [[[[NEG_INFINITY; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
    for &(x, y) in [
      // For the base pair "AU" against which another base pair is stacked.
      ((AU, AA), -0.8), ((AU, AC), -1.0), ((AU, AG), -0.8), ((AU, AU), -1.0),
      ((AU, CA), -0.6), ((AU, CC), -0.7), ((AU, CG), -0.6), ((AU, CU), -0.7),
      ((AU, GA), -0.8), ((AU, GC), -1.0), ((AU, GG), -0.8), ((AU, GU), -1.0),
      ((AU, UA), -0.6), ((AU, UC), -0.8), ((AU, UG), -0.6), ((AU, UU), -0.8),
      // For the base pair "CG" against which another base pair is stacked.
      ((CG, AA), -1.5), ((CG, AC), -1.5), ((CG, AG), -1.4), ((CG, AU), -1.5),
      ((CG, CA), -1.0), ((CG, CC), -1.1), ((CG, CG), -1.0), ((CG, CU), -0.8),
      ((CG, GA), -1.4), ((CG, GC), -1.5), ((CG, GG), -1.6), ((CG, GU), -1.5),
      ((CG, UA), -1.0), ((CG, UC), -1.4), ((CG, UG), -1.0), ((CG, UU), -1.2),
      // For the base pair "GC" against which another base pair is stacked.
      ((GC, AA), -1.1), ((GC, AC), -1.5), ((GC, AG), -1.3), ((GC, AU), -1.5),
      ((GC, CA), -1.1), ((GC, CC), -0.7), ((GC, CG), -1.1), ((GC, CU), -0.5),
      ((GC, GA), -1.6), ((GC, GC), -1.5), ((GC, GG), -1.4), ((GC, GU), -1.5),
      ((GC, UA), -1.1), ((GC, UC), -1.0), ((GC, UG), -1.1), ((GC, UU), -0.7),
      // For the base pair "GU" against which another base pair is stacked.
      ((GU, AA), -0.3), ((GU, AC), -1.0), ((GU, AG), -0.8), ((GU, AU), -1.0),
      ((GU, CA), -0.6), ((GU, CC), -0.7), ((GU, CG), -0.6), ((GU, CU), -0.7),
      ((GU, GA), -0.6), ((GU, GC), -1.0), ((GU, GG), -0.8), ((GU, GU), -1.0),
      ((GU, UA), -0.6), ((GU, UC), -0.8), ((GU, UG), -0.6), ((GU, UU), -0.8),
      // For the base pair "UA" against which another base pair is stacked.
      ((UA, AA), -1.0), ((UA, AC), -0.8), ((UA, AG), -1.1), ((UA, AU), -0.8),
      ((UA, CA), -0.7), ((UA, CC), -0.6), ((UA, CG), -0.7), ((UA, CU), -0.5),
      ((UA, GA), -1.1), ((UA, GC), -0.8), ((UA, GG), -1.2), ((UA, GU), -0.8),
      ((UA, UA), -0.7), ((UA, UC), -0.6), ((UA, UG), -0.7), ((UA, UU), -0.5),
      // For the base pair "UG" against which another base pair is stacked.
      ((UG, AA), -1.0), ((UG, AC), -0.8), ((UG, AG), -1.1), ((UG, AU), -0.8),
      ((UG, CA), -0.7), ((UG, CC), -0.6), ((UG, CG), -0.7), ((UG, CU), -0.5),
      ((UG, GA), -0.5), ((UG, GC), -0.8), ((UG, GG), -0.8), ((UG, GU), -0.8),
      ((UG, UA), -0.7), ((UG, UC), -0.6), ((UG, UG), -0.7), ((UG, UU), -0.5),
    ].iter() {tm_delta_fes[(x.0).0][(x.0).1][(x.1).0][(x.1).1] = scale(y);}
    tm_delta_fes
  };
} */
