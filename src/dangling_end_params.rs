use utils::*;

pub type DeDeltaFes = HashMap<(BasePair, Base), FreeEnergy, Hasher>;

lazy_static! {
  pub static ref THREE_PRIME_DE_DELTA_FES: DeDeltaFes = {
    [
    // For the base pair "AU" against which a base is stacked.
    ((AU, A), -0.8), ((AU, C), -0.5), ((AU, G), -0.8), ((AU, U), -0.6), 
    // For the base pair "CG" against which a base is stacked.
    ((CG, A), -1.7), ((CG, C), -0.8), ((CG, G), -1.7), ((CG, U), -1.2), 
    // For the base pair "GC" against which a base is stacked.
    ((GC, A), -1.1), ((GC, C), -0.4), ((GC, G), -1.3), ((GC, U), -0.6), 
    // For the base pair "GU" against which a base is stacked.
    ((GU, A), -0.8), ((GU, C), -0.5), ((GU, G), -0.8), ((GU, U), -0.6), 
    // For the base pair "UA" against which a base is stacked.
    ((UA, A), -0.7), ((UA, C), -0.1), ((UA, G), -0.7), ((UA, U), -0.1), 
    // For the base pair "UG" against which a base is stacked.
    ((UG, A), -0.7), ((UG, C), -0.1), ((UG, G), -0.7), ((UG, U), -0.1), 
    ].iter().cloned().collect()
  };
  pub static ref FIVE_PRIME_DE_DELTA_FES: DeDeltaFes = {
    [
    // For the base pair "AU" against which a base is stacked.
    ((AU, A), -0.3), ((AU, C), -0.1), ((AU, G), -0.2), ((AU, U), -0.2), 
    // For the base pair "CG" against which a base is stacked.
    ((CG, A), -0.2), ((CG, C), -0.3), ((CG, G), -0.0), ((CG, U), -0.0), 
    // For the base pair "GC" against which a base is stacked.
    ((GC, A), -0.5), ((GC, C), -0.7), ((GC, G), -0.2), ((GC, U), -0.1), 
    // For the base pair "GU" against which a base is stacked.
    ((GU, A), -0.3), ((GU, C), -0.1), ((GU, G), -0.2), ((GU, U), -0.2), 
    // For the base pair "UA" against which a base is stacked.
    ((UA, A), -0.3), ((UA, C), -0.3), ((UA, G), -0.4), ((UA, U), -0.2), 
    // For the base pair "UG" against which a base is stacked.
    ((UG, A), -0.3), ((UG, C), -0.3), ((UG, G), -0.4), ((UG, U), -0.2), 
    ].iter().cloned().collect()
  };
}
