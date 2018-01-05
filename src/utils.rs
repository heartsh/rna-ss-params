pub use std::collections::HashMap;
use std::f32::consts::LOG2_E;
use std::hash::BuildHasherDefault;
use fnv::FnvHasher;

pub type FreeEnergy = f32;
pub type Base = u8;
pub type BasePair = (Base, Base);
pub type Hasher = BuildHasherDefault<FnvHasher>;
pub type Seq = Vec<Base>;

pub const MAX_2LOOP_LEN: usize = 30;
pub const MAX_SPAN_OF_INDEX_PAIR_CLOSING_2LOOP: usize = MAX_2LOOP_LEN + 2;
pub const GAS_CONST: FreeEnergy = 1.98717;
pub const K0: FreeEnergy = 273.15;
pub const TEMPERATURE: FreeEnergy = 37. + K0;
pub const MAX_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_LOOP_DELTA_FE: usize = 10_000;
pub const KILO: FreeEnergy = 1000.;
pub const A: Base = 'A' as Base;
pub const C: Base = 'C' as Base;
pub const G: Base = 'G' as Base;
pub const U: Base = 'U' as Base;
pub const AA: BasePair = (A, A);
pub const AC: BasePair = (A, C);
pub const AG: BasePair = (A, G);
pub const AU: BasePair = (A, U);
pub const CA: BasePair = (C, A);
pub const CC: BasePair = (C, C);
pub const CG: BasePair = (C, G);
pub const CU: BasePair = (C, U);
pub const GA: BasePair = (G, A);
pub const GC: BasePair = (G, C);
pub const GG: BasePair = (G, G);
pub const GU: BasePair = (G, U);
pub const UA: BasePair = (U, A);
pub const UC: BasePair = (U, C);
pub const UG: BasePair = (U, G);
pub const UU: BasePair = (U, U);
const INVERSE_LOG2_E: FreeEnergy = 1. / LOG2_E;

#[inline]
pub fn fast_ln(x: f32) -> f32 {
  x.log2() * INVERSE_LOG2_E
}
