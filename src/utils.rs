pub use getopts::Options;
pub use std::fs::File;
pub use std::io::prelude::*;
pub use std::io::{BufReader, BufWriter};
pub use std::path::Path;

pub type Prob = f32;
pub type Score = Prob;
pub type Base = usize;
pub type Basepair = (Base, Base);
pub type Seq = Vec<Base>;

pub type StackScores = [[[[Score; NUM_BASES]; NUM_BASES]; NUM_BASES]; NUM_BASES];
pub type TerminalMismatchScores = StackScores;
pub type BulgeScoresInit = [Score; MAX_2LOOP_LEN + 1];
pub type HairpinScoresInit = [Score; MAX_HAIRPIN_LEN_EXTRAPOLATION + 1];
pub type HairpinScoresSpecial = [(Seq, Score); NUM_SPECIAL_HAIRPINS];
pub type InteriorScoresInit = [Score; MAX_2LOOP_LEN + 1];
pub type InteriorScoresBonus = [[Score; NUM_BASES]; NUM_BASES];
pub type InteriorScores1x1 =
  [[[[[[Score; NUM_BASES]; NUM_BASES]; NUM_BASES]; NUM_BASES]; NUM_BASES]; NUM_BASES];
pub type Interior1x2 = (Basepair, Base);
pub type InteriorScores1x2 =
  [[[[[[[Score; NUM_BASES]; NUM_BASES]; NUM_BASES]; NUM_BASES]; NUM_BASES]; NUM_BASES]; NUM_BASES];
pub type Interior2x2 = (Basepair, Basepair);
pub type InteriorScores2x2 = [[[[[[[[Score; NUM_BASES]; NUM_BASES]; NUM_BASES]; NUM_BASES];
  NUM_BASES]; NUM_BASES]; NUM_BASES]; NUM_BASES];
pub type DanglingScores = [[[Score; NUM_BASES]; NUM_BASES]; NUM_BASES];

// CONTRAfold scoring parameters
pub type BasepairScores = [[Score; NUM_BASES]; NUM_BASES];
pub type HairpinScoresLen = [Score; MAX_LOOP_LEN + 1];
pub type InteriorScoresExplicit = [[Score; MAX_INTERIOR_EXPLICIT]; MAX_INTERIOR_EXPLICIT];
pub type BulgeScoresLen = [Score; MAX_LOOP_LEN];
pub type InteriorScoresLen = [Score; MAX_LOOP_LEN - 1];
pub type InteriorScoresSymmetric = [Score; MAX_INTERIOR_SYMMETRIC];
pub type InteriorScoresAsymmetric = [Score; MAX_INTERIOR_ASYMMETRIC];
pub type BulgeScores0x1 = [Score; NUM_BASES];
pub type InteriorScores1x1Contra = [[Score; NUM_BASES]; NUM_BASES];
pub type HelixCloseScores = [[Score; NUM_BASES]; NUM_BASES];

type Arg = String;
pub type Args = Vec<Arg>;
pub type Char = u8;

pub const NUM_TRANSITS: usize = 3;
pub const INIT_MULTIBRANCH_BASE: Score = -INVERSE_TEMPERATURE * 9.3;
pub const COEFF_NUM_BRANCHES: Score = -INVERSE_TEMPERATURE * (-0.9);
pub const HELIX_AUGU_END_PENALTY: Score = -INVERSE_TEMPERATURE * 0.5;
pub const NINIO_MAX: Score = -INVERSE_TEMPERATURE * 3.;
pub const NINIO_COEFF: Score = -INVERSE_TEMPERATURE * 0.6;
pub const GAS_CONST: Score = 1.98717 / KILO; // The unit is [kcal / (K * mol)]
pub const K0: Score = 273.15; // The unit is [K]
pub const TEMPERATURE: Score = 37. + K0; // The unit is [K]
pub const MAX_HAIRPIN_LEN_EXTRAPOLATION: usize = 1_000_000;
pub const MAX_2LOOP_LEN: usize = 30;
pub const KILO: Score = 1000.;
pub const INVERSE_TEMPERATURE: Score = 1. / (GAS_CONST as Score * TEMPERATURE as Score); // The unit is [K * mol / (kcal * K)] = [mol / kcal]
pub const A: Base = 0;
pub const C: Base = 1;
pub const G: Base = 2;
pub const U: Base = 3;
pub const NUM_BASES: usize = 4;
pub const AA: Basepair = (A, A);
pub const AC: Basepair = (A, C);
pub const AG: Basepair = (A, G);
pub const AU: Basepair = (A, U);
pub const CA: Basepair = (C, A);
pub const CC: Basepair = (C, C);
pub const CG: Basepair = (C, G);
pub const CU: Basepair = (C, U);
pub const GA: Basepair = (G, A);
pub const GC: Basepair = (G, C);
pub const GG: Basepair = (G, G);
pub const GU: Basepair = (G, U);
pub const UA: Basepair = (U, A);
pub const UC: Basepair = (U, C);
pub const UG: Basepair = (U, G);
pub const UU: Basepair = (U, U);
pub const NEG_INF: Score = -1_000_000_000_000_000.;

pub const NUM_SPECIAL_HAIRPINS: usize = 22;
pub const MIN_HAIRPIN_LEN: usize = 3;
pub const MIN_SPAN_HAIRPIN_CLOSE: usize = MIN_HAIRPIN_LEN + 2;
pub const MIN_HAIRPIN_LEN_EXTRAPOLATION: usize = 10;
pub const COEFF_HAIRPIN_LEN_EXTRAPOLATION: Score =
  -INVERSE_TEMPERATURE * 1.75 * GAS_CONST * TEMPERATURE; // The unit is [kcal / mol].
lazy_static! {
  pub static ref HAIRPIN_SCORES_SPECIAL: HairpinScoresSpecial = {
    [
      (vec![C, A, A, C, G], scale(6.8)),
      (vec![G, U, U, A, C], scale(6.9)),
      (vec![C, U, A, C, G, G], scale(2.8)),
      (vec![C, U, C, C, G, G], scale(2.7)),
      (vec![C, U, U, C, G, G], scale(3.7)),
      (vec![C, U, U, U, G, G], scale(3.7)),
      (vec![C, C, A, A, G, G], scale(3.3)),
      (vec![C, C, C, A, G, G], scale(3.4)),
      (vec![C, C, G, A, G, G], scale(3.5)),
      (vec![C, C, U, A, G, G], scale(3.7)),
      (vec![C, C, A, C, G, G], scale(3.7)),
      (vec![C, C, G, C, G, G], scale(3.6)),
      (vec![C, C, U, C, G, G], scale(2.5)),
      (vec![C, U, A, A, G, G], scale(3.6)),
      (vec![C, U, C, A, G, G], scale(3.7)),
      (vec![C, U, U, A, G, G], scale(3.5)),
      (vec![C, U, G, C, G, G], scale(2.8)),
      (vec![C, A, A, C, G, G], scale(5.5)),
      (vec![A, C, A, G, U, G, C, U], scale(2.9)),
      (vec![A, C, A, G, U, G, A, U], scale(3.6)),
      (vec![A, C, A, G, U, G, U, U], scale(1.8)),
      (vec![A, C, A, G, U, A, C, U], scale(2.8)),
    ]
  };
}

pub const MAX_LOOP_LEN: usize = 30;
pub const MAX_INTERIOR_EXPLICIT: usize = 4;
pub const MAX_INTERIOR_SYMMETRIC: usize = MAX_LOOP_LEN / 2;
pub const MAX_INTERIOR_ASYMMETRIC: usize = MAX_LOOP_LEN - 2;

pub const A_LOWER: u8 = b'a';
pub const A_UPPER: u8 = b'A';
pub const C_LOWER: u8 = b'c';
pub const C_UPPER: u8 = b'C';
pub const G_LOWER: u8 = b'g';
pub const G_UPPER: u8 = b'G';
pub const U_LOWER: u8 = b'u';
pub const U_UPPER: u8 = b'U';

pub fn scale(x: Score) -> Score {
  -INVERSE_TEMPERATURE * x
}

pub fn print_program_usage(program_name: &str, opts: &Options) {
  let program_usage = format!("The usage of this program: {} [options]", program_name);
  print!("{}", opts.usage(&program_usage));
}

pub fn char2base(x: Char) -> Base {
  match x {
    A_LOWER | A_UPPER => A,
    C_LOWER | C_UPPER => C,
    G_LOWER | G_UPPER => G,
    U_LOWER | U_UPPER => U,
    _ => {
      panic!();
    }
  }
}
