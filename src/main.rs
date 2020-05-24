extern crate rna_ss_params;

use rna_ss_params::utils::*;
use rna_ss_params::hairpin_loop_params::*;
use rna_ss_params::bulge_loop_params::*;
use rna_ss_params::dangling_end_params::*;
use rna_ss_params::internal_loop_params::*;
use std::path::Path;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;

pub const NEG_INF: FreeEnergy = -1000_000_000_000_000.;

fn main() {
  let mut output_file_path = Path::new("./src/compiled_free_energy_params.rs");
  let mut writer_2_output_file = BufWriter::new(File::create(&output_file_path).unwrap());
  let mut buf = format!("use utils::*;\n\
    use hairpin_loop_params::*;\n\
    use bulge_loop_params::*;\n\
    use dangling_end_params::*;\n\
    use internal_loop_params::*;\n\n");
  // From the file "bulge_loop_params.rs".
  let INIT_BL_DELTA_FES: Vec<FreeEnergy> = [
    0., 3.8, 2.8, 3.2, 3.6, 4.0, 4.4, 4.6, 4.7, 4.8, 4.9,
    5., 5.1, 5.2, 5.3, 5.4, 5.4, 5.5, 5.5, 5.6, 5.7,
    5.7, 5.8, 5.8, 5.8, 5.9, 5.9, 6., 6., 6., 6.1
  ].iter().map(|&x| {scale(x)}).collect();
  buf += &format!("pub const INIT_BL_DELTA_FES: InitBlDeltaFes = {:?};\n", &INIT_BL_DELTA_FES);
  // From the file "dangling_end_params.rs".
  let mut THREE_PRIME_DE_DELTA_FES: DeDeltaFes = [[[NEG_INF; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
  for &(x, y) in &[
    // For the base pair "AU" against which a base is stacked.
    ((AU, A), -0.7), ((AU, C), -0.1), ((AU, G), -0.7), ((AU, U), -0.1), 
    // For the base pair "CG" against which a base is stacked.
    ((CG, A), -1.1), ((CG, C), -0.4), ((CG, G), -1.3), ((CG, U), -0.6), 
    // For the base pair "GC" against which a base is stacked.
    ((GC, A), -1.7), ((GC, C), -0.8), ((GC, G), -1.7), ((GC, U), -1.2), 
    // For the base pair "GU" against which a base is stacked.
    ((GU, A), -0.7), ((GU, C), -0.1), ((GU, G), -0.7), ((GU, U), -0.1), 
    // For the base pair "UA" against which a base is stacked.
    ((UA, A), -0.8), ((UA, C), -0.5), ((UA, G), -0.8), ((UA, U), -0.6), 
    // For the base pair "UG" against which a base is stacked.
    ((UG, A), -0.8), ((UG, C), -0.5), ((UG, G), -0.8), ((UG, U), -0.6), 
  ] {
    THREE_PRIME_DE_DELTA_FES[(x.0).0][(x.0).1][x.1] = scale(y);
  }
  buf += &format!("pub const THREE_PRIME_DE_DELTA_FES: DeDeltaFes = {:?};\n", &THREE_PRIME_DE_DELTA_FES);
  let mut FIVE_PRIME_DE_DELTA_FES = [[[NEG_INF; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
  for &(x, y) in &[
    // For the base pair "AU" against which a base is stacked.
    ((AU, A), -0.3), ((AU, C), -0.3), ((AU, G), -0.4), ((AU, U), -0.2), 
    // For the base pair "CG" against which a base is stacked.
    ((CG, A), -0.5), ((CG, C), -0.3), ((CG, G), -0.2), ((CG, U), -0.1), 
    // For the base pair "GC" against which a base is stacked.
    ((GC, A), -0.2), ((GC, C), -0.3), ((GC, G), -0.0), ((GC, U), -0.0), 
    // For the base pair "GU" against which a base is stacked.
    ((GU, A), -0.3), ((GU, C), -0.3), ((GU, G), -0.4), ((GU, U), -0.2), 
    // For the base pair "UA" against which a base is stacked.
    ((UA, A), -0.3), ((UA, C), -0.1), ((UA, G), -0.2), ((UA, U), -0.2), 
    // For the base pair "UG" against which a base is stacked.
    ((UG, A), -0.3), ((UG, C), -0.1), ((UG, G), -0.2), ((UG, U), -0.2), 
  ] {
    FIVE_PRIME_DE_DELTA_FES[(x.0).0][(x.0).1][x.1] = scale(y);
  }
  buf += &format!("pub const FIVE_PRIME_DE_DELTA_FES: DeDeltaFes = {:?};\n", &FIVE_PRIME_DE_DELTA_FES);
  let mut INIT_HL_DELTA_FES: Vec<FreeEnergy> = vec![0., 0., 0., 5.4, 5.6, 5.7, 5.4, 6.0, 5.5, 6.4].iter().map(|&x| {scale(x)}).collect();
  let len_of_init_hl_delta_fes = INIT_HL_DELTA_FES.len();
  let basic_init_hl_delta_fe = INIT_HL_DELTA_FES[MIN_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_HL_DELTA_FE - 1];
  for i in len_of_init_hl_delta_fes .. MAX_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_LOOP_DELTA_FE + 1 {
    INIT_HL_DELTA_FES.push(basic_init_hl_delta_fe + COEFFICIENT_4_LOG_EXTRAPOLATION_OF_INIT_HL_DELTA_FE * (i as FreeEnergy / (MIN_LOOP_LEN_4_LOG_EXTRAPOLATION_OF_INIT_HL_DELTA_FE - 1) as FreeEnergy).ln());
  }
  buf += &format!("pub const INIT_HL_DELTA_FES: InitHlDeltaFes = {:?};\n", &INIT_HL_DELTA_FES);
  let EXP_INIT_HL_DELTA_FES: Vec<FreeEnergy> = INIT_HL_DELTA_FES.iter().map(|&x| {x.exp()}).collect();
  let INIT_IL_DELTA_FES: Vec<FreeEnergy> = vec![
    0., 0., 0., 0., 1.1, 2.0, 2.0, 2.1, 2.3, 2.4, 2.5,
    2.6, 2.7, 2.8, 2.9, 2.9, 3., 3.1, 3.1, 3.2, 3.3,
    3.3, 3.4, 3.4, 3.5, 3.5, 3.5, 3.6, 3.6, 3.7, 3.7
  ].iter().map(|&x| {scale(x)}).collect();
  buf += &format!("pub const INIT_IL_DELTA_FES: InitIlDeltaFes = {:?};\n", &INIT_IL_DELTA_FES);
  let mut TWO_VS_3_IL_TM_BONUS_DELTA_FES = [[NEG_INF; NUM_OF_BASES]; NUM_OF_BASES];
  for &(x, y) in &[(AG, -0.5), (GA, -1.2), (GG, -0.8), (UU, -0.4)] {
    TWO_VS_3_IL_TM_BONUS_DELTA_FES[x.0][x.1] = scale(y);
  }
  buf += &format!("pub const TWO_VS_3_IL_TM_BONUS_DELTA_FES: IlTmBonusDeltaFes = {:?};\n", &TWO_VS_3_IL_TM_BONUS_DELTA_FES);
  let mut OTHER_IL_TM_BONUS_DELTA_FES = [[NEG_INF; NUM_OF_BASES]; NUM_OF_BASES];
  for &(x, y) in &[(AG, -0.8), (GA, -1.0), (GG, -1.2), (UU, -0.7)] {
    OTHER_IL_TM_BONUS_DELTA_FES[x.0][x.1] = scale(y);
  }
  buf += &format!("pub const OTHER_IL_TM_BONUS_DELTA_FES: IlTmBonusDeltaFes = {:?};\n", &OTHER_IL_TM_BONUS_DELTA_FES);
  let mut ONE_VS_1_IL_DELTA_FES = [[[[[[NEG_INF; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
  for &(x, y) in [
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
    ((UA, UA, UA), 1.9), ((UA, UC, UA), 1.9), ((UA, UG, UA), 1.9), ((UA, UU, UA), 1.5),
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
    ((GU, UA, AU), 1.9), ((GU, UC, AU), 1.9), ((GU, UG, AU), 1.9), ((GU, UU, AU), 1.6),
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
    ((UG, UA, AU), 1.9), ((UG, UC, AU), 1.9), ((UG, UG, AU), 1.9), ((UG, UU, AU), 1.9),
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
  ].iter() {
    ONE_VS_1_IL_DELTA_FES[(x.0).0][(x.0).1][(x.1).0][(x.1).1][(x.2).0][(x.2).1] = scale(y);
  }
  buf += &format!("pub const ONE_VS_1_IL_DELTA_FES: OneVs1IlDeltaFes = {:?};\n", &ONE_VS_1_IL_DELTA_FES);

  let mut ONE_VS_2_IL_DELTA_FES = [[[[[[[NEG_INF; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
  for &(x, y) in [
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
    ((AU, (CA, C), CG), 3.0), ((AU, (CC, C), CG), 3.0), ((AU, (CG, C), CG), 3.0), ((AU, (CU, C), CG), 3.0),
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
    ((AU, (CA, G), CG), 3.0), ((AU, (CC, G), CG), 3.0), ((AU, (CG, G), CG), 3.0), ((AU, (CU, G), CG), 3.0),
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
    ((AU, (CA, U), CG), 3.0), ((AU, (CC, U), CG), 3.0), ((AU, (CG, U), CG), 3.0), ((AU, (CU, U), CG), 3.0),
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
    ((CG, (AA, A), AU), 3.0), ((CG, (AC, A), AU), 3.0), ((CG, (AG, A), AU), 1.9), ((CG, (AU, A), AU), 3.0),
    ((CG, (CA, A), AU), 3.0), ((CG, (CC, A), AU), 3.0), ((CG, (CG, A), AU), 3.0), ((CG, (CU, A), AU), 3.0),
    ((CG, (GA, A), AU), 1.9), ((CG, (GC, A), AU), 1.9), ((CG, (GG, A), AU), 1.9), ((CG, (GU, A), AU), 1.9),
    ((CG, (UA, A), AU), 3.0), ((CG, (UC, A), AU), 3.0), ((CG, (UG, A), AU), 3.0), ((CG, (UU, A), AU), 2.2),
    // For internal loops between the base pairs "CG" and "CG".
    ((CG, (AA, A), CG), 2.5), ((CG, (AC, A), CG), 2.3), ((CG, (AG, A), CG), 1.1), ((CG, (AU, A), CG), 2.3),
    ((CG, (CA, A), CG), 2.3), ((CG, (CC, A), CG), 2.3), ((CG, (CG, A), CG), 2.3), ((CG, (CU, A), CG), 2.3),
    ((CG, (GA, A), CG), 1.7), ((CG, (GC, A), CG), 2.3), ((CG, (GG, A), CG), 0.8), ((CG, (GU, A), CG), 2.3),
    ((CG, (UA, A), CG), 2.3), ((CG, (UC, A), CG), 2.3), ((CG, (UG, A), CG), 2.3), ((CG, (UU, A), CG), 1.5),
    // For internal loops between the base pairs "CG" and "GC".
    ((CG, (AA, A), GC), 2.3), ((CG, (AC, A), GC), 2.3), ((CG, (AG, A), GC), 1.1), ((CG, (AU, A), GC), 2.3),
    ((CG, (CA, A), GC), 2.3), ((CG, (CC, A), GC), 2.3), ((CG, (CG, A), GC), 2.3), ((CG, (CU, A), GC), 2.3),
    ((CG, (GA, A), GC), 1.1), ((CG, (GC, A), GC), 1.1), ((CG, (GG, A), GC), 1.1), ((CG, (GU, A), GC), 1.1),
    ((CG, (UA, A), GC), 2.3), ((CG, (UC, A), GC), 2.3), ((CG, (UG, A), GC), 2.3), ((CG, (UU, A), GC), 1.5),
    // For internal loops between the base pairs "CG" and "UA".
    ((CG, (AA, A), UA), 3.0), ((CG, (AC, A), UA), 3.0), ((CG, (AG, A), UA), 1.9), ((CG, (AU, A), UA), 3.0),
    ((CG, (CA, A), UA), 3.0), ((CG, (CC, A), UA), 3.0), ((CG, (CG, A), UA), 3.0), ((CG, (CU, A), UA), 3.0),
    ((CG, (GA, A), UA), 1.9), ((CG, (GC, A), UA), 3.0), ((CG, (GG, A), UA), 1.9), ((CG, (GU, A), UA), 3.0),
    ((CG, (UA, A), UA), 3.0), ((CG, (UC, A), UA), 3.0), ((CG, (UG, A), UA), 3.0), ((CG, (UU, A), UA), 2.2),
    // For internal loops between the base pairs "CG" and "GU".
    ((CG, (AA, A), GU), 3.0), ((CG, (AC, A), GU), 3.0), ((CG, (AG, A), GU), 1.9), ((CG, (AU, A), GU), 3.0),
    ((CG, (CA, A), GU), 3.0), ((CG, (CC, A), GU), 3.0), ((CG, (CG, A), GU), 3.0), ((CG, (CU, A), GU), 3.0),
    ((CG, (GA, A), GU), 1.9), ((CG, (GC, A), GU), 1.9), ((CG, (GG, A), GU), 1.9), ((CG, (GU, A), GU), 1.9),
    ((CG, (UA, A), GU), 3.0), ((CG, (UC, A), GU), 3.0), ((CG, (UG, A), GU), 3.0), ((CG, (UU, A), GU), 2.2),
    // For internal loops between the base pairs "CG" and "UG".
    ((CG, (AA, A), UG), 3.0), ((CG, (AC, A), UG), 3.0), ((CG, (AG, A), UG), 1.9), ((CG, (AU, A), UG), 3.0),
    ((CG, (CA, A), UG), 3.0), ((CG, (CC, A), UG), 3.0), ((CG, (CG, A), UG), 3.0), ((CG, (CU, A), UG), 3.0),
    ((CG, (GA, A), UG), 1.9), ((CG, (GC, A), UG), 3.0), ((CG, (GG, A), UG), 1.9), ((CG, (GU, A), UG), 3.0),
    ((CG, (UA, A), UG), 3.0), ((CG, (UC, A), UG), 3.0), ((CG, (UG, A), UG), 3.0), ((CG, (UU, A), UG), 2.2),
    // For internal loops between the base pairs "CG" and "AU".
    ((CG, (AA, C), AU), 3.0), ((CG, (AC, C), AU), 3.0), ((CG, (AG, C), AU), 1.9), ((CG, (AU, C), AU), 3.0),
    ((CG, (CA, C), AU), 3.0), ((CG, (CC, C), AU), 3.0), ((CG, (CG, C), AU), 3.0), ((CG, (CU, C), AU), 3.0),
    ((CG, (GA, C), AU), 1.9), ((CG, (GC, C), AU), 3.0), ((CG, (GG, C), AU), 1.9), ((CG, (GU, C), AU), 3.0),
    ((CG, (UA, C), AU), 3.0), ((CG, (UC, C), AU), 3.0), ((CG, (UG, C), AU), 3.0), ((CG, (UU, C), AU), 2.2),
    // For internal loops between the base pairs "CG" and "CG".
    ((CG, (AA, C), CG), 2.3), ((CG, (AC, C), CG), 1.7), ((CG, (AG, C), CG), 1.1), ((CG, (AU, C), CG), 2.3),
    ((CG, (CA, C), CG), 2.3), ((CG, (CC, C), CG), 2.5), ((CG, (CG, C), CG), 2.3), ((CG, (CU, C), CG), 2.3),
    ((CG, (GA, C), CG), 1.1), ((CG, (GC, C), CG), 2.3), ((CG, (GG, C), CG), 1.1), ((CG, (GU, C), CG), 2.3),
    ((CG, (UA, C), CG), 2.3), ((CG, (UC, C), CG), 2.2), ((CG, (UG, C), CG), 2.3), ((CG, (UU, C), CG), 1.5),
    // For internal loops between the base pairs "CG" and "GC".
    ((CG, (AA, C), GC), 2.3), ((CG, (AC, C), GC), 2.3), ((CG, (AG, C), GC), 1.1), ((CG, (AU, C), GC), 2.3),
    ((CG, (CA, C), GC), 2.3), ((CG, (CC, C), GC), 2.3), ((CG, (CG, C), GC), 2.3), ((CG, (CU, C), GC), 2.3),
    ((CG, (GA, C), GC), 1.1), ((CG, (GC, C), GC), 2.3), ((CG, (GG, C), GC), 1.1), ((CG, (GU, C), GC), 2.3),
    ((CG, (UA, C), GC), 2.3), ((CG, (UC, C), GC), 2.3), ((CG, (UG, C), GC), 2.3), ((CG, (UU, C), GC), 1.5),
    // For internal loops between the base pairs "CG" and "UA".
    ((CG, (AA, C), UA), 3.0), ((CG, (AC, C), UA), 3.0), ((CG, (AG, C), UA), 1.9), ((CG, (AU, C), UA), 3.0),
    ((CG, (CA, C), UA), 3.0), ((CG, (CC, C), UA), 3.0), ((CG, (CG, C), UA), 3.0), ((CG, (CU, C), UA), 3.0),
    ((CG, (GA, C), UA), 1.9), ((CG, (GC, C), UA), 3.0), ((CG, (GG, C), UA), 1.9), ((CG, (GU, C), UA), 3.0),
    ((CG, (UA, C), UA), 3.0), ((CG, (UC, C), UA), 3.0), ((CG, (UG, C), UA), 3.0), ((CG, (UU, C), UA), 2.2),
    // For internal loops between the base pairs "CG" and "GU".
    ((CG, (AA, C), GU), 3.0), ((CG, (AC, C), GU), 3.0), ((CG, (AG, C), GU), 1.9), ((CG, (AU, C), GU), 3.0),
    ((CG, (CA, C), GU), 3.0), ((CG, (CC, C), GU), 3.0), ((CG, (CG, C), GU), 3.0), ((CG, (CU, C), GU), 3.0),
    ((CG, (GA, C), GU), 1.9), ((CG, (GC, C), GU), 3.0), ((CG, (GG, C), GU), 1.9), ((CG, (GU, C), GU), 3.0),
    ((CG, (UA, C), GU), 3.0), ((CG, (UC, C), GU), 3.0), ((CG, (UG, C), GU), 3.0), ((CG, (UU, C), GU), 2.2),
    // For internal loops between the base pairs "CG" and "UG".
    ((CG, (AA, C), UG), 3.0), ((CG, (AC, C), UG), 3.0), ((CG, (AG, C), UG), 1.9), ((CG, (AU, C), UG), 3.0),
    ((CG, (CA, C), UG), 3.0), ((CG, (CC, C), UG), 3.0), ((CG, (CG, C), UG), 3.0), ((CG, (CU, C), UG), 3.0),
    ((CG, (GA, C), UG), 1.9), ((CG, (GC, C), UG), 3.0), ((CG, (GG, C), UG), 1.9), ((CG, (GU, C), UG), 3.0),
    ((CG, (UA, C), UG), 3.0), ((CG, (UC, C), UG), 3.0), ((CG, (UG, C), UG), 3.0), ((CG, (UU, C), UG), 2.2),
    // For internal loops between the base pairs "CG" and "AU".
    ((CG, (AA, G), AU), 1.9), ((CG, (AC, G), AU), 1.9), ((CG, (AG, G), AU), 1.9), ((CG, (AU, G), AU), 1.9),
    ((CG, (CA, G), AU), 3.0), ((CG, (CC, G), AU), 3.0), ((CG, (CG, G), AU), 3.0), ((CG, (CU, G), AU), 3.0),
    ((CG, (GA, G), AU), 1.9), ((CG, (GC, G), AU), 1.9), ((CG, (GG, G), AU), 1.9), ((CG, (GU, G), AU), 1.9),
    ((CG, (UA, G), AU), 3.0), ((CG, (UC, G), AU), 3.0), ((CG, (UG, G), AU), 3.0), ((CG, (UU, G), AU), 2.2),
    // For internal loops between the base pairs "CG" and "CG".
    ((CG, (AA, G), CG), 0.8), ((CG, (AC, G), CG), 1.1), ((CG, (AG, G), CG), 1.1), ((CG, (AU, G), CG), 1.1),
    ((CG, (CA, G), CG), 2.3), ((CG, (CC, G), CG), 2.3), ((CG, (CG, G), CG), 2.3), ((CG, (CU, G), CG), 2.3),
    ((CG, (GA, G), CG), 1.2), ((CG, (GC, G), CG), 1.1), ((CG, (GG, G), CG), 1.1), ((CG, (GU, G), CG), 1.1),
    ((CG, (UA, G), CG), 2.3), ((CG, (UC, G), CG), 2.3), ((CG, (UG, G), CG), 2.3), ((CG, (UU, G), CG), 1.5),
    // For internal loops between the base pairs "CG" and "GC".
    ((CG, (AA, G), GC), 1.1), ((CG, (AC, G), GC), 1.1), ((CG, (AG, G), GC), 1.1), ((CG, (AU, G), GC), 1.1),
    ((CG, (CA, G), GC), 2.3), ((CG, (CC, G), GC), 2.3), ((CG, (CG, G), GC), 2.3), ((CG, (CU, G), GC), 2.3),
    ((CG, (GA, G), GC), 1.1), ((CG, (GC, G), GC), 1.1), ((CG, (GG, G), GC), 1.1), ((CG, (GU, G), GC), 1.1),
    ((CG, (UA, G), GC), 2.3), ((CG, (UC, G), GC), 2.3), ((CG, (UG, G), GC), 2.3), ((CG, (UU, G), GC), 1.5),
    // For internal loops between the base pairs "CG" and "UA".
    ((CG, (AA, G), UA), 1.9), ((CG, (AC, G), UA), 1.9), ((CG, (AG, G), UA), 1.9), ((CG, (AU, G), UA), 1.9),
    ((CG, (CA, G), UA), 3.0), ((CG, (CC, G), UA), 3.0), ((CG, (CG, G), UA), 3.0), ((CG, (CU, G), UA), 3.0),
    ((CG, (GA, G), UA), 1.9), ((CG, (GC, G), UA), 1.9), ((CG, (GG, G), UA), 1.9), ((CG, (GU, G), UA), 1.9),
    ((CG, (UA, G), UA), 3.0), ((CG, (UC, G), UA), 3.0), ((CG, (UG, G), UA), 3.0), ((CG, (UU, G), UA), 2.2),
    // For internal loops between the base pairs "CG" and "GU".
    ((CG, (AA, G), GU), 1.9), ((CG, (AC, G), GU), 1.9), ((CG, (AG, G), GU), 1.9), ((CG, (AU, G), GU), 1.9),
    ((CG, (CA, G), GU), 3.0), ((CG, (CC, G), GU), 3.0), ((CG, (CG, G), GU), 3.0), ((CG, (CU, G), GU), 3.0),
    ((CG, (GA, G), GU), 1.9), ((CG, (GC, G), GU), 1.9), ((CG, (GG, G), GU), 1.9), ((CG, (GU, G), GU), 1.9),
    ((CG, (UA, G), GU), 3.0), ((CG, (UC, G), GU), 3.0), ((CG, (UG, G), GU), 3.0), ((CG, (UU, G), GU), 2.2),
    // For internal loops between the base pairs "CG" and "UG".
    ((CG, (AA, G), UG), 1.9), ((CG, (AC, G), UG), 1.9), ((CG, (AG, G), UG), 1.9), ((CG, (AU, G), UG), 1.9),
    ((CG, (CA, G), UG), 3.0), ((CG, (CC, G), UG), 3.0), ((CG, (CG, G), UG), 3.0), ((CG, (CU, G), UG), 3.0),
    ((CG, (GA, G), UG), 1.9), ((CG, (GC, G), UG), 1.9), ((CG, (GG, G), UG), 1.9), ((CG, (GU, G), UG), 1.9),
    ((CG, (UA, G), UG), 3.0), ((CG, (UC, G), UG), 3.0), ((CG, (UG, G), UG), 3.0), ((CG, (UU, G), UG), 2.2),
    // For internal loops between the base pairs "CG" and "AU".
    ((CG, (AA, U), AU), 3.0), ((CG, (AC, U), AU), 3.0), ((CG, (AG, U), AU), 1.9), ((CG, (AU, U), AU), 3.0),
    ((CG, (CA, U), AU), 3.0), ((CG, (CC, U), AU), 3.0), ((CG, (CG, U), AU), 3.0), ((CG, (CU, U), AU), 3.0),
    ((CG, (GA, U), AU), 1.9), ((CG, (GC, U), AU), 3.0), ((CG, (GG, U), AU), 1.9), ((CG, (GU, U), AU), 3.0),
    ((CG, (UA, U), AU), 2.2), ((CG, (UC, U), AU), 2.2), ((CG, (UG, U), AU), 2.2), ((CG, (UU, U), AU), 2.2),
    // For internal loops between the base pairs "CG" and "CG".
    ((CG, (AA, U), CG), 2.3), ((CG, (AC, U), CG), 2.3), ((CG, (AG, U), CG), 1.1), ((CG, (AU, U), CG), 2.3),
    ((CG, (CA, U), CG), 2.5), ((CG, (CC, U), CG), 2.3), ((CG, (CG, U), CG), 2.3), ((CG, (CU, U), CG), 2.3),
    ((CG, (GA, U), CG), 1.1), ((CG, (GC, U), CG), 2.3), ((CG, (GG, U), CG), 1.1), ((CG, (GU, U), CG), 2.3),
    ((CG, (UA, U), CG), 1.5), ((CG, (UC, U), CG), 1.7), ((CG, (UG, U), CG), 1.5), ((CG, (UU, U), CG), 1.4),
    // For internal loops between the base pairs "CG" and "GC".
    ((CG, (AA, U), GC), 2.3), ((CG, (AC, U), GC), 2.3), ((CG, (AG, U), GC), 1.1), ((CG, (AU, U), GC), 2.3),
    ((CG, (CA, U), GC), 2.3), ((CG, (CC, U), GC), 2.3), ((CG, (CG, U), GC), 2.3), ((CG, (CU, U), GC), 2.3),
    ((CG, (GA, U), GC), 1.1), ((CG, (GC, U), GC), 2.3), ((CG, (GG, U), GC), 1.1), ((CG, (GU, U), GC), 2.3),
    ((CG, (UA, U), GC), 1.5), ((CG, (UC, U), GC), 1.5), ((CG, (UG, U), GC), 1.5), ((CG, (UU, U), GC), 1.5),
    // For internal loops between the base pairs "CG" and "UA".
    ((CG, (AA, U), UA), 3.0), ((CG, (AC, U), UA), 3.0), ((CG, (AG, U), UA), 1.9), ((CG, (AU, U), UA), 3.0),
    ((CG, (CA, U), UA), 3.0), ((CG, (CC, U), UA), 3.0), ((CG, (CG, U), UA), 3.0), ((CG, (CU, U), UA), 3.0),
    ((CG, (GA, U), UA), 1.9), ((CG, (GC, U), UA), 3.0), ((CG, (GG, U), UA), 1.9), ((CG, (GU, U), UA), 3.0),
    ((CG, (UA, U), UA), 2.2), ((CG, (UC, U), UA), 2.2), ((CG, (UG, U), UA), 2.2), ((CG, (UU, U), UA), 2.2),
    // For internal loops between the base pairs "CG" and "GU".
    ((CG, (AA, U), GU), 3.0), ((CG, (AC, U), GU), 3.0), ((CG, (AG, U), GU), 1.9), ((CG, (AU, U), GU), 3.0),
    ((CG, (CA, U), GU), 3.0), ((CG, (CC, U), GU), 3.0), ((CG, (CG, U), GU), 3.0), ((CG, (CU, U), GU), 3.0),
    ((CG, (GA, U), GU), 1.9), ((CG, (GC, U), GU), 3.0), ((CG, (GG, U), GU), 1.9), ((CG, (GU, U), GU), 3.0),
    ((CG, (UA, U), GU), 2.2), ((CG, (UC, U), GU), 2.2), ((CG, (UG, U), GU), 2.2), ((CG, (UU, U), GU), 2.2),
    // For internal loops between the base pairs "CG" and "UG".
    ((CG, (AA, U), UG), 3.0), ((CG, (AC, U), UG), 3.0), ((CG, (AG, U), UG), 1.9), ((CG, (AU, U), UG), 3.0),
    ((CG, (CA, U), UG), 3.0), ((CG, (CC, U), UG), 3.0), ((CG, (CG, U), UG), 3.0), ((CG, (CU, U), UG), 3.0),
    ((CG, (GA, U), UG), 1.9), ((CG, (GC, U), UG), 3.0), ((CG, (GG, U), UG), 1.9), ((CG, (GU, U), UG), 3.0),
    ((CG, (UA, U), UG), 2.2), ((CG, (UC, U), UG), 2.2), ((CG, (UG, U), UG), 2.2), ((CG, (UU, U), UG), 2.2),
    // For internal loops behind the base pair "GC".
    // For internal loops between the base pairs "GC" and "AU".
    ((GC, (AA, A), AU), 3.0), ((GC, (AC, A), AU), 3.0), ((GC, (AG, A), AU), 3.0), ((GC, (AU, A), AU), 3.0),
    ((GC, (CA, A), AU), 3.0), ((GC, (CC, A), AU), 3.0), ((GC, (CG, A), AU), 3.0), ((GC, (CU, A), AU), 3.0),
    ((GC, (GA, A), AU), 1.9), ((GC, (GC, A), AU), 1.9), ((GC, (GG, A), AU), 1.9), ((GC, (GU, A), AU), 1.9),
    ((GC, (UA, A), AU), 3.0), ((GC, (UC, A), AU), 3.0), ((GC, (UG, A), AU), 3.0), ((GC, (UU, A), AU), 2.2),
    // For internal loops between the base pairs "GC" and "CG".
    ((GC, (AA, A), CG), 2.3), ((GC, (AC, A), CG), 2.3), ((GC, (AG, A), CG), 2.3), ((GC, (AU, A), CG), 2.3),
    ((GC, (CA, A), CG), 2.3), ((GC, (CC, A), CG), 2.3), ((GC, (CG, A), CG), 2.3), ((GC, (CU, A), CG), 2.3),
    ((GC, (GA, A), CG), 1.1), ((GC, (GC, A), CG), 2.3), ((GC, (GG, A), CG), 1.1), ((GC, (GU, A), CG), 2.3),
    ((GC, (UA, A), CG), 2.3), ((GC, (UC, A), CG), 2.3), ((GC, (UG, A), CG), 2.3), ((GC, (UU, A), CG), 1.5),
    // For internal loops between the base pairs "GC" and "GC".
    ((GC, (AA, A), GC), 2.5), ((GC, (AC, A), GC), 2.3), ((GC, (AG, A), GC), 2.1), ((GC, (AU, A), GC), 2.3),
    ((GC, (CA, A), GC), 2.3), ((GC, (CC, A), GC), 2.3), ((GC, (CG, A), GC), 2.3), ((GC, (CU, A), GC), 2.3),
    ((GC, (GA, A), GC), 1.1), ((GC, (GC, A), GC), 1.1), ((GC, (GG, A), GC), 1.1), ((GC, (GU, A), GC), 1.1),
    ((GC, (UA, A), GC), 2.3), ((GC, (UC, A), GC), 2.3), ((GC, (UG, A), GC), 2.3), ((GC, (UU, A), GC), 1.5),
    // For internal loops between the base pairs "GC" and "UA".
    ((GC, (AA, A), UA), 3.0), ((GC, (AC, A), UA), 3.0), ((GC, (AG, A), UA), 3.0), ((GC, (AU, A), UA), 3.0),
    ((GC, (CA, A), UA), 3.0), ((GC, (CC, A), UA), 3.0), ((GC, (CG, A), UA), 3.0), ((GC, (CU, A), UA), 3.0),
    ((GC, (GA, A), UA), 1.9), ((GC, (GC, A), UA), 3.0), ((GC, (GG, A), UA), 1.9), ((GC, (GU, A), UA), 3.0),
    ((GC, (UA, A), UA), 3.0), ((GC, (UC, A), UA), 3.0), ((GC, (UG, A), UA), 3.0), ((GC, (UU, A), UA), 2.2),
    // For internal loops between the base pairs "GC" and "GU".
    ((GC, (AA, A), GU), 2.5), ((GC, (AC, A), GU), 3.0), ((GC, (AG, A), GU), 2.1), ((GC, (AU, A), GU), 3.0),
    ((GC, (CA, A), GU), 3.0), ((GC, (CC, A), GU), 3.0), ((GC, (CG, A), GU), 3.0), ((GC, (CU, A), GU), 3.0),
    ((GC, (GA, A), GU), 1.9), ((GC, (GC, A), GU), 1.9), ((GC, (GG, A), GU), 1.9), ((GC, (GU, A), GU), 1.9),
    ((GC, (UA, A), GU), 3.0), ((GC, (UC, A), GU), 3.0), ((GC, (UG, A), GU), 3.0), ((GC, (UU, A), GU), 2.2),
    // For internal loops between the base pairs "GC" and "UG".
    ((GC, (AA, A), UG), 3.0), ((GC, (AC, A), UG), 3.0), ((GC, (AG, A), UG), 3.0), ((GC, (AU, A), UG), 3.0),
    ((GC, (CA, A), UG), 3.0), ((GC, (CC, A), UG), 3.0), ((GC, (CG, A), UG), 3.0), ((GC, (CU, A), UG), 3.0),
    ((GC, (GA, A), UG), 1.9), ((GC, (GC, A), UG), 3.0), ((GC, (GG, A), UG), 1.9), ((GC, (GU, A), UG), 3.0),
    ((GC, (UA, A), UG), 3.0), ((GC, (UC, A), UG), 3.0), ((GC, (UG, A), UG), 3.0), ((GC, (UU, A), UG), 2.2),
    // For internal loops between the base pairs "GC" and "AU".
    ((GC, (AA, C), AU), 3.0), ((GC, (AC, C), AU), 3.0), ((GC, (AG, C), AU), 3.0), ((GC, (AU, C), AU), 3.0),
    ((GC, (CA, C), AU), 3.0), ((GC, (CC, C), AU), 3.0), ((GC, (CG, C), AU), 3.0), ((GC, (CU, C), AU), 3.0),
    ((GC, (GA, C), AU), 1.9), ((GC, (GC, C), AU), 3.0), ((GC, (GG, C), AU), 1.9), ((GC, (GU, C), AU), 3.0),
    ((GC, (UA, C), AU), 3.0), ((GC, (UC, C), AU), 3.0), ((GC, (UG, C), AU), 3.0), ((GC, (UU, C), AU), 2.2),
    // For internal loops between the base pairs "GC" and "CG".
    ((GC, (AA, C), CG), 2.3), ((GC, (AC, C), CG), 2.3), ((GC, (AG, C), CG), 2.3), ((GC, (AU, C), CG), 2.3),
    ((GC, (CA, C), CG), 2.3), ((GC, (CC, C), CG), 2.3), ((GC, (CG, C), CG), 2.3), ((GC, (CU, C), CG), 2.3),
    ((GC, (GA, C), CG), 1.1), ((GC, (GC, C), CG), 2.3), ((GC, (GG, C), CG), 1.1), ((GC, (GU, C), CG), 2.3),
    ((GC, (UA, C), CG), 2.3), ((GC, (UC, C), CG), 2.3), ((GC, (UG, C), CG), 2.3), ((GC, (UU, C), CG), 1.5),
    // For internal loops between the base pairs "GC" and "GC".
    ((GC, (AA, C), GC), 2.3), ((GC, (AC, C), GC), 2.3), ((GC, (AG, C), GC), 2.3), ((GC, (AU, C), GC), 2.3),
    ((GC, (CA, C), GC), 2.3), ((GC, (CC, C), GC), 2.3), ((GC, (CG, C), GC), 2.3), ((GC, (CU, C), GC), 2.3),
    ((GC, (GA, C), GC), 1.1), ((GC, (GC, C), GC), 2.3), ((GC, (GG, C), GC), 1.1), ((GC, (GU, C), GC), 2.3),
    ((GC, (UA, C), GC), 2.3), ((GC, (UC, C), GC), 2.3), ((GC, (UG, C), GC), 2.3), ((GC, (UU, C), GC), 1.5),
    // For internal loops between the base pairs "GC" and "UA".
    ((GC, (AA, C), UA), 3.0), ((GC, (AC, C), UA), 3.0), ((GC, (AG, C), UA), 3.0), ((GC, (AU, C), UA), 3.0),
    ((GC, (CA, C), UA), 3.0), ((GC, (CC, C), UA), 3.0), ((GC, (CG, C), UA), 3.0), ((GC, (CU, C), UA), 3.0),
    ((GC, (GA, C), UA), 1.9), ((GC, (GC, C), UA), 3.0), ((GC, (GG, C), UA), 1.9), ((GC, (GU, C), UA), 3.0),
    ((GC, (UA, C), UA), 3.0), ((GC, (UC, C), UA), 3.0), ((GC, (UG, C), UA), 3.0), ((GC, (UU, C), UA), 2.2),
    // For internal loops between the base pairs "GC" and "GU".
    ((GC, (AA, C), GU), 3.0), ((GC, (AC, C), GU), 3.0), ((GC, (AG, C), GU), 3.0), ((GC, (AU, C), GU), 3.0),
    ((GC, (CA, C), GU), 3.0), ((GC, (CC, C), GU), 3.0), ((GC, (CG, C), GU), 3.0), ((GC, (CU, C), GU), 3.0),
    ((GC, (GA, C), GU), 1.9), ((GC, (GC, C), GU), 3.0), ((GC, (GG, C), GU), 1.9), ((GC, (GU, C), GU), 3.0),
    ((GC, (UA, C), GU), 3.0), ((GC, (UC, C), GU), 3.0), ((GC, (UG, C), GU), 3.0), ((GC, (UU, C), GU), 2.2),
    // For internal loops between the base pairs "GC" and "UG".
    ((GC, (AA, C), UG), 3.0), ((GC, (AC, C), UG), 3.0), ((GC, (AG, C), UG), 3.0), ((GC, (AU, C), UG), 3.0),
    ((GC, (CA, C), UG), 3.0), ((GC, (CC, C), UG), 3.0), ((GC, (CG, C), UG), 3.0), ((GC, (CU, C), UG), 3.0),
    ((GC, (GA, C), UG), 1.9), ((GC, (GC, C), UG), 3.0), ((GC, (GG, C), UG), 1.9), ((GC, (GU, C), UG), 3.0),
    ((GC, (UA, C), UG), 3.0), ((GC, (UC, C), UG), 3.0), ((GC, (UG, C), UG), 3.0), ((GC, (UU, C), UG), 2.2),
    // For internal loops between the base pairs "GC" and "AU".
    ((GC, (AA, G), AU), 1.9), ((GC, (AC, G), AU), 1.9), ((GC, (AG, G), AU), 1.9), ((GC, (AU, G), AU), 1.9),
    ((GC, (CA, G), AU), 3.0), ((GC, (CC, G), AU), 3.0), ((GC, (CG, G), AU), 3.0), ((GC, (CU, G), AU), 3.0),
    ((GC, (GA, G), AU), 1.9), ((GC, (GC, G), AU), 1.9), ((GC, (GG, G), AU), 1.9), ((GC, (GU, G), AU), 1.9),
    ((GC, (UA, G), AU), 3.0), ((GC, (UC, G), AU), 3.0), ((GC, (UG, G), AU), 3.0), ((GC, (UU, G), AU), 2.2),
    // For internal loops between the base pairs "GC" and "CG".
    ((GC, (AA, G), CG), 1.1), ((GC, (AC, G), CG), 1.1), ((GC, (AG, G), CG), 1.1), ((GC, (AU, G), CG), 1.1),
    ((GC, (CA, G), CG), 2.3), ((GC, (CC, G), CG), 2.3), ((GC, (CG, G), CG), 2.3), ((GC, (CU, G), CG), 2.3),
    ((GC, (GA, G), CG), 1.1), ((GC, (GC, G), CG), 1.1), ((GC, (GG, G), CG), 1.1), ((GC, (GU, G), CG), 1.1),
    ((GC, (UA, G), CG), 2.3), ((GC, (UC, G), CG), 2.3), ((GC, (UG, G), CG), 2.3), ((GC, (UU, G), CG), 1.5),
    // For internal loops between the base pairs "GC" and "GC".
    ((GC, (AA, G), GC), 1.2), ((GC, (AC, G), GC), 1.1), ((GC, (AG, G), GC), 1.1), ((GC, (AU, G), GC), 1.1),
    ((GC, (CA, G), GC), 2.3), ((GC, (CC, G), GC), 2.3), ((GC, (CG, G), GC), 2.3), ((GC, (CU, G), GC), 2.3),
    ((GC, (GA, G), GC), 1.1), ((GC, (GC, G), GC), 1.1), ((GC, (GG, G), GC), 1.1), ((GC, (GU, G), GC), 1.1),
    ((GC, (UA, G), GC), 2.3), ((GC, (UC, G), GC), 2.3), ((GC, (UG, G), GC), 2.3), ((GC, (UU, G), GC), 1.5),
    // For internal loops between the base pairs "GC" and "UA".
    ((GC, (AA, G), UA), 1.9), ((GC, (AC, G), UA), 1.9), ((GC, (AG, G), UA), 1.9), ((GC, (AU, G), UA), 1.9),
    ((GC, (CA, G), UA), 3.0), ((GC, (CC, G), UA), 3.0), ((GC, (CG, G), UA), 3.0), ((GC, (CU, G), UA), 3.0),
    ((GC, (GA, G), UA), 1.9), ((GC, (GC, G), UA), 1.9), ((GC, (GG, G), UA), 1.9), ((GC, (GU, G), UA), 1.9),
    ((GC, (UA, G), UA), 3.0), ((GC, (UC, G), UA), 3.0), ((GC, (UG, G), UA), 3.0), ((GC, (UU, G), UA), 2.2),
    // For internal loops between the base pairs "GC" and "GU".
    ((GC, (AA, G), GU), 1.2), ((GC, (AC, G), GU), 1.9), ((GC, (AG, G), GU), 1.9), ((GC, (AU, G), GU), 1.9),
    ((GC, (CA, G), GU), 3.0), ((GC, (CC, G), GU), 3.0), ((GC, (CG, G), GU), 3.0), ((GC, (CU, G), GU), 3.0),
    ((GC, (GA, G), GU), 1.9), ((GC, (GC, G), GU), 1.9), ((GC, (GG, G), GU), 1.9), ((GC, (GU, G), GU), 1.9),
    ((GC, (UA, G), GU), 3.0), ((GC, (UC, G), GU), 3.0), ((GC, (UG, G), GU), 3.0), ((GC, (UU, G), GU), 2.2),
    // For internal loops between the base pairs "GC" and "UG".
    ((GC, (AA, G), UG), 1.9), ((GC, (AC, G), UG), 1.9), ((GC, (AG, G), UG), 1.9), ((GC, (AU, G), UG), 1.9),
    ((GC, (CA, G), UG), 3.0), ((GC, (CC, G), UG), 3.0), ((GC, (CG, G), UG), 3.0), ((GC, (CU, G), UG), 3.0),
    ((GC, (GA, G), UG), 1.9), ((GC, (GC, G), UG), 1.9), ((GC, (GG, G), UG), 1.9), ((GC, (GU, G), UG), 1.9),
    ((GC, (UA, G), UG), 3.0), ((GC, (UC, G), UG), 3.0), ((GC, (UG, G), UG), 3.0), ((GC, (UU, G), UG), 2.2),
    // For internal loops between the base pairs "GC" and "AU".
    ((GC, (AA, U), AU), 3.0), ((GC, (AC, U), AU), 3.0), ((GC, (AG, U), AU), 3.0), ((GC, (AU, U), AU), 3.0),
    ((GC, (CA, U), AU), 3.0), ((GC, (CC, U), AU), 3.0), ((GC, (CG, U), AU), 3.0), ((GC, (CU, U), AU), 3.0),
    ((GC, (GA, U), AU), 1.9), ((GC, (GC, U), AU), 3.0), ((GC, (GG, U), AU), 1.9), ((GC, (GU, U), AU), 3.0),
    ((GC, (UA, U), AU), 2.2), ((GC, (UC, U), AU), 2.2), ((GC, (UG, U), AU), 2.2), ((GC, (UU, U), AU), 2.2),
    // For internal loops between the base pairs "GC" and "CG".
    ((GC, (AA, U), CG), 2.3), ((GC, (AC, U), CG), 2.3), ((GC, (AG, U), CG), 2.3), ((GC, (AU, U), CG), 2.3),
    ((GC, (CA, U), CG), 2.3), ((GC, (CC, U), CG), 2.3), ((GC, (CG, U), CG), 2.3), ((GC, (CU, U), CG), 2.3),
    ((GC, (GA, U), CG), 1.1), ((GC, (GC, U), CG), 2.3), ((GC, (GG, U), CG), 1.1), ((GC, (GU, U), CG), 2.3),
    ((GC, (UA, U), CG), 1.5), ((GC, (UC, U), CG), 1.5), ((GC, (UG, U), CG), 1.5), ((GC, (UU, U), CG), 1.5),
    // For internal loops between the base pairs "GC" and "GC".
    ((GC, (AA, U), GC), 2.3), ((GC, (AC, U), GC), 2.3), ((GC, (AG, U), GC), 2.3), ((GC, (AU, U), GC), 2.3),
    ((GC, (CA, U), GC), 2.3), ((GC, (CC, U), GC), 1.9), ((GC, (CG, U), GC), 2.3), ((GC, (CU, U), GC), 2.3),
    ((GC, (GA, U), GC), 1.1), ((GC, (GC, U), GC), 2.3), ((GC, (GG, U), GC), 1.1), ((GC, (GU, U), GC), 2.3),
    ((GC, (UA, U), GC), 1.5), ((GC, (UC, U), GC), 1.5), ((GC, (UG, U), GC), 1.5), ((GC, (UU, U), GC), 1.5),
    // For internal loops between the base pairs "GC" and "UA".
    ((GC, (AA, U), UA), 3.0), ((GC, (AC, U), UA), 3.0), ((GC, (AG, U), UA), 3.0), ((GC, (AU, U), UA), 3.0),
    ((GC, (CA, U), UA), 3.0), ((GC, (CC, U), UA), 3.0), ((GC, (CG, U), UA), 3.0), ((GC, (CU, U), UA), 3.0),
    ((GC, (GA, U), UA), 1.9), ((GC, (GC, U), UA), 3.0), ((GC, (GG, U), UA), 1.9), ((GC, (GU, U), UA), 3.0),
    ((GC, (UA, U), UA), 2.2), ((GC, (UC, U), UA), 2.2), ((GC, (UG, U), UA), 2.2), ((GC, (UU, U), UA), 2.2),
    // For internal loops between the base pairs "GC" and "GU".
    ((GC, (AA, U), GU), 3.0), ((GC, (AC, U), GU), 3.0), ((GC, (AG, U), GU), 3.0), ((GC, (AU, U), GU), 3.0),
    ((GC, (CA, U), GU), 3.0), ((GC, (CC, U), GU), 1.9), ((GC, (CG, U), GU), 3.0), ((GC, (CU, U), GU), 3.0),
    ((GC, (GA, U), GU), 1.9), ((GC, (GC, U), GU), 3.0), ((GC, (GG, U), GU), 1.9), ((GC, (GU, U), GU), 3.0),
    ((GC, (UA, U), GU), 2.2), ((GC, (UC, U), GU), 2.2), ((GC, (UG, U), GU), 2.2), ((GC, (UU, U), GU), 2.2),
    // For internal loops between the base pairs "GC" and "UG".
    ((GC, (AA, U), UG), 3.0), ((GC, (AC, U), UG), 3.0), ((GC, (AG, U), UG), 3.0), ((GC, (AU, U), UG), 3.0),
    ((GC, (CA, U), UG), 3.0), ((GC, (CC, U), UG), 3.0), ((GC, (CG, U), UG), 3.0), ((GC, (CU, U), UG), 3.0),
    ((GC, (GA, U), UG), 1.9), ((GC, (GC, U), UG), 3.0), ((GC, (GG, U), UG), 1.9), ((GC, (GU, U), UG), 3.0),
    ((GC, (UA, U), UG), 2.2), ((GC, (UC, U), UG), 2.2), ((GC, (UG, U), UG), 2.2), ((GC, (UU, U), UG), 2.2),
    // For internal loops behind the base pair "UA".
    // For internal loops between the base pairs "UA" and "AU".
    ((UA, (AA, A), AU), 3.7), ((UA, (AC, A), AU), 3.7), ((UA, (AG, A), AU), 2.6), ((UA, (AU, A), AU), 3.7),
    ((UA, (CA, A), AU), 3.7), ((UA, (CC, A), AU), 3.7), ((UA, (CG, A), AU), 3.7), ((UA, (CU, A), AU), 3.7),
    ((UA, (GA, A), AU), 2.6), ((UA, (GC, A), AU), 2.6), ((UA, (GG, A), AU), 2.6), ((UA, (GU, A), AU), 2.6),
    ((UA, (UA, A), AU), 3.7), ((UA, (UC, A), AU), 3.7), ((UA, (UG, A), AU), 3.7), ((UA, (UU, A), AU), 3.0),
    // For internal loops between the base pairs "UA" and "CG".
    ((UA, (AA, A), CG), 3.0), ((UA, (AC, A), CG), 3.0), ((UA, (AG, A), CG), 1.9), ((UA, (AU, A), CG), 3.0),
    ((UA, (CA, A), CG), 3.0), ((UA, (CC, A), CG), 3.0), ((UA, (CG, A), CG), 3.0), ((UA, (CU, A), CG), 3.0),
    ((UA, (GA, A), CG), 1.9), ((UA, (GC, A), CG), 3.0), ((UA, (GG, A), CG), 1.9), ((UA, (GU, A), CG), 3.0),
    ((UA, (UA, A), CG), 3.0), ((UA, (UC, A), CG), 3.0), ((UA, (UG, A), CG), 3.0), ((UA, (UU, A), CG), 2.2),
    // For internal loops between the base pairs "UA" and "GC".
    ((UA, (AA, A), GC), 3.0), ((UA, (AC, A), GC), 3.0), ((UA, (AG, A), GC), 1.9), ((UA, (AU, A), GC), 3.0),
    ((UA, (CA, A), GC), 3.0), ((UA, (CC, A), GC), 3.0), ((UA, (CG, A), GC), 3.0), ((UA, (CU, A), GC), 3.0),
    ((UA, (GA, A), GC), 1.9), ((UA, (GC, A), GC), 1.9), ((UA, (GG, A), GC), 1.9), ((UA, (GU, A), GC), 1.9),
    ((UA, (UA, A), GC), 3.0), ((UA, (UC, A), GC), 3.0), ((UA, (UG, A), GC), 3.0), ((UA, (UU, A), GC), 2.2),
    // For internal loops between the base pairs "UA" and "UA".
    ((UA, (AA, A), UA), 3.7), ((UA, (AC, A), UA), 3.7), ((UA, (AG, A), UA), 2.6), ((UA, (AU, A), UA), 3.7),
    ((UA, (CA, A), UA), 3.7), ((UA, (CC, A), UA), 3.7), ((UA, (CG, A), UA), 3.7), ((UA, (CU, A), UA), 3.7),
    ((UA, (GA, A), UA), 2.6), ((UA, (GC, A), UA), 3.7), ((UA, (GG, A), UA), 2.6), ((UA, (GU, A), UA), 3.7),
    ((UA, (UA, A), UA), 3.7), ((UA, (UC, A), UA), 3.7), ((UA, (UG, A), UA), 3.7), ((UA, (UU, A), UA), 3.0),
    // For internal loops between the base pairs "UA" and "GU".
    ((UA, (AA, A), GU), 3.7), ((UA, (AC, A), GU), 3.7), ((UA, (AG, A), GU), 2.6), ((UA, (AU, A), GU), 3.7),
    ((UA, (CA, A), GU), 3.7), ((UA, (CC, A), GU), 3.7), ((UA, (CG, A), GU), 3.7), ((UA, (CU, A), GU), 3.7),
    ((UA, (GA, A), GU), 2.6), ((UA, (GC, A), GU), 2.6), ((UA, (GG, A), GU), 2.6), ((UA, (GU, A), GU), 2.6),
    ((UA, (UA, A), GU), 3.7), ((UA, (UC, A), GU), 3.7), ((UA, (UG, A), GU), 3.7), ((UA, (UU, A), GU), 3.0),
    // For internal loops between the base pairs "UA" and "UG".
    ((UA, (AA, A), UG), 3.7), ((UA, (AC, A), UG), 3.7), ((UA, (AG, A), UG), 2.6), ((UA, (AU, A), UG), 3.7),
    ((UA, (CA, A), UG), 3.7), ((UA, (CC, A), UG), 3.7), ((UA, (CG, A), UG), 3.7), ((UA, (CU, A), UG), 3.7),
    ((UA, (GA, A), UG), 2.6), ((UA, (GC, A), UG), 3.7), ((UA, (GG, A), UG), 2.6), ((UA, (GU, A), UG), 3.7),
    ((UA, (UA, A), UG), 3.7), ((UA, (UC, A), UG), 3.7), ((UA, (UG, A), UG), 3.7), ((UA, (UU, A), UG), 3.0),
    // For internal loops between the base pairs "UA" and "AU".
    ((UA, (AA, C), AU), 3.7), ((UA, (AC, C), AU), 3.7), ((UA, (AG, C), AU), 2.6), ((UA, (AU, C), AU), 3.7),
    ((UA, (CA, C), AU), 3.7), ((UA, (CC, C), AU), 3.7), ((UA, (CG, C), AU), 3.7), ((UA, (CU, C), AU), 3.7),
    ((UA, (GA, C), AU), 2.6), ((UA, (GC, C), AU), 3.7), ((UA, (GG, C), AU), 2.6), ((UA, (GU, C), AU), 3.7),
    ((UA, (UA, C), AU), 3.7), ((UA, (UC, C), AU), 3.7), ((UA, (UG, C), AU), 3.7), ((UA, (UU, C), AU), 3.0),
    // For internal loops between the base pairs "UA" and "CG".
    ((UA, (AA, C), CG), 3.0), ((UA, (AC, C), CG), 3.0), ((UA, (AG, C), CG), 1.9), ((UA, (AU, C), CG), 3.0),
    ((UA, (CA, C), CG), 3.0), ((UA, (CC, C), CG), 3.0), ((UA, (CG, C), CG), 3.0), ((UA, (CU, C), CG), 3.0),
    ((UA, (GA, C), CG), 1.9), ((UA, (GC, C), CG), 3.0), ((UA, (GG, C), CG), 1.9), ((UA, (GU, C), CG), 3.0),
    ((UA, (UA, C), CG), 3.0), ((UA, (UC, C), CG), 3.0), ((UA, (UG, C), CG), 3.0), ((UA, (UU, C), CG), 2.2),
    // For internal loops between the base pairs "UA" and "GC".
    ((UA, (AA, C), GC), 3.0), ((UA, (AC, C), GC), 3.0), ((UA, (AG, C), GC), 1.9), ((UA, (AU, C), GC), 3.0),
    ((UA, (CA, C), GC), 3.0), ((UA, (CC, C), GC), 3.0), ((UA, (CG, C), GC), 3.0), ((UA, (CU, C), GC), 3.0),
    ((UA, (GA, C), GC), 1.9), ((UA, (GC, C), GC), 3.0), ((UA, (GG, C), GC), 1.9), ((UA, (GU, C), GC), 3.0),
    ((UA, (UA, C), GC), 3.0), ((UA, (UC, C), GC), 3.0), ((UA, (UG, C), GC), 3.0), ((UA, (UU, C), GC), 2.2),
    // For internal loops between the base pairs "UA" and "UA".
    ((UA, (AA, C), UA), 3.7), ((UA, (AC, C), UA), 3.7), ((UA, (AG, C), UA), 2.6), ((UA, (AU, C), UA), 3.7),
    ((UA, (CA, C), UA), 3.7), ((UA, (CC, C), UA), 3.7), ((UA, (CG, C), UA), 3.7), ((UA, (CU, C), UA), 3.7),
    ((UA, (GA, C), UA), 2.6), ((UA, (GC, C), UA), 3.7), ((UA, (GG, C), UA), 2.6), ((UA, (GU, C), UA), 3.7),
    ((UA, (UA, C), UA), 3.7), ((UA, (UC, C), UA), 3.7), ((UA, (UG, C), UA), 3.7), ((UA, (UU, C), UA), 3.0),
    // For internal loops between the base pairs "UA" and "GU".
    ((UA, (AA, C), GU), 3.7), ((UA, (AC, C), GU), 3.7), ((UA, (AG, C), GU), 2.6), ((UA, (AU, C), GU), 3.7),
    ((UA, (CA, C), GU), 3.7), ((UA, (CC, C), GU), 3.7), ((UA, (CG, C), GU), 3.7), ((UA, (CU, C), GU), 3.7),
    ((UA, (GA, C), GU), 2.6), ((UA, (GC, C), GU), 3.7), ((UA, (GG, C), GU), 2.6), ((UA, (GU, C), GU), 3.7),
    ((UA, (UA, C), GU), 3.7), ((UA, (UC, C), GU), 3.7), ((UA, (UG, C), GU), 3.7), ((UA, (UU, C), GU), 3.0),
    // For internal loops between the base pairs "UA" and "UG".
    ((UA, (AA, C), UG), 3.7), ((UA, (AC, C), UG), 3.7), ((UA, (AG, C), UG), 2.6), ((UA, (AU, C), UG), 3.7),
    ((UA, (CA, C), UG), 3.7), ((UA, (CC, C), UG), 3.7), ((UA, (CG, C), UG), 3.7), ((UA, (CU, C), UG), 3.7),
    ((UA, (GA, C), UG), 2.6), ((UA, (GC, C), UG), 3.7), ((UA, (GG, C), UG), 2.6), ((UA, (GU, C), UG), 3.7),
    ((UA, (UA, C), UG), 3.7), ((UA, (UC, C), UG), 3.7), ((UA, (UG, C), UG), 3.7), ((UA, (UU, C), UG), 3.0),
    // For internal loops between the base pairs "UA" and "AU".
    ((UA, (AA, G), AU), 2.6), ((UA, (AC, G), AU), 2.6), ((UA, (AG, G), AU), 2.6), ((UA, (AU, G), AU), 2.6),
    ((UA, (CA, G), AU), 3.7), ((UA, (CC, G), AU), 3.7), ((UA, (CG, G), AU), 3.7), ((UA, (CU, G), AU), 3.7),
    ((UA, (GA, G), AU), 2.6), ((UA, (GC, G), AU), 2.6), ((UA, (GG, G), AU), 2.6), ((UA, (GU, G), AU), 2.6),
    ((UA, (UA, G), AU), 3.7), ((UA, (UC, G), AU), 3.7), ((UA, (UG, G), AU), 3.7), ((UA, (UU, G), AU), 3.0),
    // For internal loops between the base pairs "UA" and "CG".
    ((UA, (AA, G), CG), 1.9), ((UA, (AC, G), CG), 1.9), ((UA, (AG, G), CG), 1.9), ((UA, (AU, G), CG), 1.9),
    ((UA, (CA, G), CG), 3.0), ((UA, (CC, G), CG), 3.0), ((UA, (CG, G), CG), 3.0), ((UA, (CU, G), CG), 3.0),
    ((UA, (GA, G), CG), 1.9), ((UA, (GC, G), CG), 1.9), ((UA, (GG, G), CG), 1.9), ((UA, (GU, G), CG), 1.9),
    ((UA, (UA, G), CG), 3.0), ((UA, (UC, G), CG), 3.0), ((UA, (UG, G), CG), 3.0), ((UA, (UU, G), CG), 2.2),
    // For internal loops between the base pairs "UA" and "GC".
    ((UA, (AA, G), GC), 1.9), ((UA, (AC, G), GC), 1.9), ((UA, (AG, G), GC), 1.9), ((UA, (AU, G), GC), 1.9),
    ((UA, (CA, G), GC), 3.0), ((UA, (CC, G), GC), 3.0), ((UA, (CG, G), GC), 3.0), ((UA, (CU, G), GC), 3.0),
    ((UA, (GA, G), GC), 1.9), ((UA, (GC, G), GC), 1.9), ((UA, (GG, G), GC), 1.9), ((UA, (GU, G), GC), 1.9),
    ((UA, (UA, G), GC), 3.0), ((UA, (UC, G), GC), 3.0), ((UA, (UG, G), GC), 3.0), ((UA, (UU, G), GC), 2.2),
    // For internal loops between the base pairs "UA" and "UA".
    ((UA, (AA, G), UA), 2.6), ((UA, (AC, G), UA), 2.6), ((UA, (AG, G), UA), 2.6), ((UA, (AU, G), UA), 2.6),
    ((UA, (CA, G), UA), 3.7), ((UA, (CC, G), UA), 3.7), ((UA, (CG, G), UA), 3.7), ((UA, (CU, G), UA), 3.7),
    ((UA, (GA, G), UA), 2.6), ((UA, (GC, G), UA), 2.6), ((UA, (GG, G), UA), 2.6), ((UA, (GU, G), UA), 2.6),
    ((UA, (UA, G), UA), 3.7), ((UA, (UC, G), UA), 3.7), ((UA, (UG, G), UA), 3.7), ((UA, (UU, G), UA), 3.0),
    // For internal loops between the base pairs "UA" and "GU".
    ((UA, (AA, G), GU), 2.6), ((UA, (AC, G), GU), 2.6), ((UA, (AG, G), GU), 2.6), ((UA, (AU, G), GU), 2.6),
    ((UA, (CA, G), GU), 3.7), ((UA, (CC, G), GU), 3.7), ((UA, (CG, G), GU), 3.7), ((UA, (CU, G), GU), 3.7),
    ((UA, (GA, G), GU), 2.6), ((UA, (GC, G), GU), 2.6), ((UA, (GG, G), GU), 2.6), ((UA, (GU, G), GU), 2.6),
    ((UA, (UA, G), GU), 3.7), ((UA, (UC, G), GU), 3.7), ((UA, (UG, G), GU), 3.7), ((UA, (UU, G), GU), 3.0),
    // For internal loops between the base pairs "UA" and "UG".
    ((UA, (AA, G), UG), 2.6), ((UA, (AC, G), UG), 2.6), ((UA, (AG, G), UG), 2.6), ((UA, (AU, G), UG), 2.6),
    ((UA, (CA, G), UG), 3.7), ((UA, (CC, G), UG), 3.7), ((UA, (CG, G), UG), 3.7), ((UA, (CU, G), UG), 3.7),
    ((UA, (GA, G), UG), 2.6), ((UA, (GC, G), UG), 2.6), ((UA, (GG, G), UG), 2.6), ((UA, (GU, G), UG), 2.6),
    ((UA, (UA, G), UG), 3.7), ((UA, (UC, G), UG), 3.7), ((UA, (UG, G), UG), 3.7), ((UA, (UU, G), UG), 3.0),
    // For internal loops between the base pairs "UA" and "AU".
    ((UA, (AA, U), AU), 3.7), ((UA, (AC, U), AU), 3.7), ((UA, (AG, U), AU), 2.6), ((UA, (AU, U), AU), 3.7),
    ((UA, (CA, U), AU), 3.7), ((UA, (CC, U), AU), 3.7), ((UA, (CG, U), AU), 3.7), ((UA, (CU, U), AU), 3.7),
    ((UA, (GA, U), AU), 2.6), ((UA, (GC, U), AU), 3.7), ((UA, (GG, U), AU), 2.6), ((UA, (GU, U), AU), 3.7),
    ((UA, (UA, U), AU), 3.0), ((UA, (UC, U), AU), 3.0), ((UA, (UG, U), AU), 3.0), ((UA, (UU, U), AU), 3.0),
    // For internal loops between the base pairs "UA" and "CG".
    ((UA, (AA, U), CG), 3.0), ((UA, (AC, U), CG), 3.0), ((UA, (AG, U), CG), 1.9), ((UA, (AU, U), CG), 3.0),
    ((UA, (CA, U), CG), 3.0), ((UA, (CC, U), CG), 3.0), ((UA, (CG, U), CG), 3.0), ((UA, (CU, U), CG), 3.0),
    ((UA, (GA, U), CG), 1.9), ((UA, (GC, U), CG), 3.0), ((UA, (GG, U), CG), 1.9), ((UA, (GU, U), CG), 3.0),
    ((UA, (UA, U), CG), 2.2), ((UA, (UC, U), CG), 2.2), ((UA, (UG, U), CG), 2.2), ((UA, (UU, U), CG), 2.2),
    // For internal loops between the base pairs "UA" and "GC".
    ((UA, (AA, U), GC), 3.0), ((UA, (AC, U), GC), 3.0), ((UA, (AG, U), GC), 1.9), ((UA, (AU, U), GC), 3.0),
    ((UA, (CA, U), GC), 3.0), ((UA, (CC, U), GC), 3.0), ((UA, (CG, U), GC), 3.0), ((UA, (CU, U), GC), 3.0),
    ((UA, (GA, U), GC), 1.9), ((UA, (GC, U), GC), 3.0), ((UA, (GG, U), GC), 1.9), ((UA, (GU, U), GC), 3.0),
    ((UA, (UA, U), GC), 2.2), ((UA, (UC, U), GC), 2.2), ((UA, (UG, U), GC), 2.2), ((UA, (UU, U), GC), 2.2),
    // For internal loops between the base pairs "UA" and "UA".
    ((UA, (AA, U), UA), 3.7), ((UA, (AC, U), UA), 3.7), ((UA, (AG, U), UA), 2.6), ((UA, (AU, U), UA), 3.7),
    ((UA, (CA, U), UA), 3.7), ((UA, (CC, U), UA), 3.7), ((UA, (CG, U), UA), 3.7), ((UA, (CU, U), UA), 3.7),
    ((UA, (GA, U), UA), 2.6), ((UA, (GC, U), UA), 3.7), ((UA, (GG, U), UA), 2.6), ((UA, (GU, U), UA), 3.7),
    ((UA, (UA, U), UA), 3.0), ((UA, (UC, U), UA), 3.0), ((UA, (UG, U), UA), 3.0), ((UA, (UU, U), UA), 3.0),
    // For internal loops between the base pairs "UA" and "GU".
    ((UA, (AA, U), GU), 3.7), ((UA, (AC, U), GU), 3.7), ((UA, (AG, U), GU), 2.6), ((UA, (AU, U), GU), 3.7),
    ((UA, (CA, U), GU), 3.7), ((UA, (CC, U), GU), 3.7), ((UA, (CG, U), GU), 3.7), ((UA, (CU, U), GU), 3.7),
    ((UA, (GA, U), GU), 2.6), ((UA, (GC, U), GU), 3.7), ((UA, (GG, U), GU), 2.6), ((UA, (GU, U), GU), 3.7),
    ((UA, (UA, U), GU), 3.0), ((UA, (UC, U), GU), 3.0), ((UA, (UG, U), GU), 3.0), ((UA, (UU, U), GU), 3.0),
    // For internal loops between the base pairs "UA" and "UG".
    ((UA, (AA, U), UG), 3.7), ((UA, (AC, U), UG), 3.7), ((UA, (AG, U), UG), 2.6), ((UA, (AU, U), UG), 3.7),
    ((UA, (CA, U), UG), 3.7), ((UA, (CC, U), UG), 3.7), ((UA, (CG, U), UG), 3.7), ((UA, (CU, U), UG), 3.7),
    ((UA, (GA, U), UG), 2.6), ((UA, (GC, U), UG), 3.7), ((UA, (GG, U), UG), 2.6), ((UA, (GU, U), UG), 3.7),
    ((UA, (UA, U), UG), 3.0), ((UA, (UC, U), UG), 3.0), ((UA, (UG, U), UG), 3.0), ((UA, (UU, U), UG), 3.0),
    // For internal loops behind the base pair "GU".
    // For internal loops between the base pairs "GU" and "AU".
    ((GU, (AA, A), AU), 3.7), ((GU, (AC, A), AU), 3.7), ((GU, (AG, A), AU), 3.7), ((GU, (AU, A), AU), 3.7),
    ((GU, (CA, A), AU), 3.7), ((GU, (CC, A), AU), 3.7), ((GU, (CG, A), AU), 3.7), ((GU, (CU, A), AU), 3.7),
    ((GU, (GA, A), AU), 2.6), ((GU, (GC, A), AU), 2.6), ((GU, (GG, A), AU), 2.6), ((GU, (GU, A), AU), 2.6),
    ((GU, (UA, A), AU), 3.7), ((GU, (UC, A), AU), 3.7), ((GU, (UG, A), AU), 3.7), ((GU, (UU, A), AU), 3.0),
    // For internal loops between the base pairs "GU" and "CG".
    ((GU, (AA, A), CG), 3.0), ((GU, (AC, A), CG), 3.0), ((GU, (AG, A), CG), 3.0), ((GU, (AU, A), CG), 3.0),
    ((GU, (CA, A), CG), 3.0), ((GU, (CC, A), CG), 3.0), ((GU, (CG, A), CG), 3.0), ((GU, (CU, A), CG), 3.0),
    ((GU, (GA, A), CG), 1.9), ((GU, (GC, A), CG), 3.0), ((GU, (GG, A), CG), 1.9), ((GU, (GU, A), CG), 3.0),
    ((GU, (UA, A), CG), 3.0), ((GU, (UC, A), CG), 3.0), ((GU, (UG, A), CG), 3.0), ((GU, (UU, A), CG), 2.2),
    // For internal loops between the base pairs "GU" and "GC".
    ((GU, (AA, A), GC), 2.5), ((GU, (AC, A), GC), 3.0), ((GU, (AG, A), GC), 2.1), ((GU, (AU, A), GC), 3.0),
    ((GU, (CA, A), GC), 3.0), ((GU, (CC, A), GC), 3.0), ((GU, (CG, A), GC), 3.0), ((GU, (CU, A), GC), 3.0),
    ((GU, (GA, A), GC), 1.9), ((GU, (GC, A), GC), 1.9), ((GU, (GG, A), GC), 1.9), ((GU, (GU, A), GC), 1.9),
    ((GU, (UA, A), GC), 3.0), ((GU, (UC, A), GC), 3.0), ((GU, (UG, A), GC), 3.0), ((GU, (UU, A), GC), 2.2),
    // For internal loops between the base pairs "GU" and "UA".
    ((GU, (AA, A), UA), 3.7), ((GU, (AC, A), UA), 3.7), ((GU, (AG, A), UA), 3.7), ((GU, (AU, A), UA), 3.7),
    ((GU, (CA, A), UA), 3.7), ((GU, (CC, A), UA), 3.7), ((GU, (CG, A), UA), 3.7), ((GU, (CU, A), UA), 3.7),
    ((GU, (GA, A), UA), 2.6), ((GU, (GC, A), UA), 3.7), ((GU, (GG, A), UA), 2.6), ((GU, (GU, A), UA), 3.7),
    ((GU, (UA, A), UA), 3.7), ((GU, (UC, A), UA), 3.7), ((GU, (UG, A), UA), 3.7), ((GU, (UU, A), UA), 3.0),
    // For internal loops between the base pairs "GU" and "GU".
    ((GU, (AA, A), GU), 2.5), ((GU, (AC, A), GU), 3.7), ((GU, (AG, A), GU), 2.1), ((GU, (AU, A), GU), 3.7),
    ((GU, (CA, A), GU), 3.7), ((GU, (CC, A), GU), 3.7), ((GU, (CG, A), GU), 3.7), ((GU, (CU, A), GU), 3.7),
    ((GU, (GA, A), GU), 2.6), ((GU, (GC, A), GU), 2.6), ((GU, (GG, A), GU), 2.6), ((GU, (GU, A), GU), 2.6),
    ((GU, (UA, A), GU), 3.7), ((GU, (UC, A), GU), 3.7), ((GU, (UG, A), GU), 3.7), ((GU, (UU, A), GU), 3.0),
    // For internal loops between the base pairs "GU" and "UG".
    ((GU, (AA, A), UG), 3.7), ((GU, (AC, A), UG), 3.7), ((GU, (AG, A), UG), 2.6), ((GU, (AU, A), UG), 3.7),
    ((GU, (CA, A), UG), 3.7), ((GU, (CC, A), UG), 3.7), ((GU, (CG, A), UG), 3.7), ((GU, (CU, A), UG), 3.7),
    ((GU, (GA, A), UG), 2.6), ((GU, (GC, A), UG), 3.7), ((GU, (GG, A), UG), 2.6), ((GU, (GU, A), UG), 3.7),
    ((GU, (UA, A), UG), 3.7), ((GU, (UC, A), UG), 3.7), ((GU, (UG, A), UG), 3.7), ((GU, (UU, A), UG), 3.0),
    // For internal loops between the base pairs "GU" and "AU".
    ((GU, (AA, C), AU), 3.7), ((GU, (AC, C), AU), 3.7), ((GU, (AG, C), AU), 3.7), ((GU, (AU, C), AU), 3.7),
    ((GU, (CA, C), AU), 3.7), ((GU, (CC, C), AU), 3.7), ((GU, (CG, C), AU), 3.7), ((GU, (CU, C), AU), 3.7),
    ((GU, (GA, C), AU), 2.6), ((GU, (GC, C), AU), 3.7), ((GU, (GG, C), AU), 2.6), ((GU, (GU, C), AU), 3.7),
    ((GU, (UA, C), AU), 3.7), ((GU, (UC, C), AU), 3.7), ((GU, (UG, C), AU), 3.7), ((GU, (UU, C), AU), 3.0),
    // For internal loops between the base pairs "GU" and "CG".
    ((GU, (AA, C), CG), 3.0), ((GU, (AC, C), CG), 3.0), ((GU, (AG, C), CG), 3.0), ((GU, (AU, C), CG), 3.0),
    ((GU, (CA, C), CG), 3.0), ((GU, (CC, C), CG), 3.0), ((GU, (CG, C), CG), 3.0), ((GU, (CU, C), CG), 3.0),
    ((GU, (GA, C), CG), 1.9), ((GU, (GC, C), CG), 3.0), ((GU, (GG, C), CG), 1.9), ((GU, (GU, C), CG), 3.0),
    ((GU, (UA, C), CG), 3.0), ((GU, (UC, C), CG), 3.0), ((GU, (UG, C), CG), 3.0), ((GU, (UU, C), CG), 2.2),
    // For internal loops between the base pairs "GU" and "GC".
    ((GU, (AA, C), GC), 3.0), ((GU, (AC, C), GC), 3.0), ((GU, (AG, C), GC), 3.0), ((GU, (AU, C), GC), 3.0),
    ((GU, (CA, C), GC), 3.0), ((GU, (CC, C), GC), 3.0), ((GU, (CG, C), GC), 3.0), ((GU, (CU, C), GC), 3.0),
    ((GU, (GA, C), GC), 1.9), ((GU, (GC, C), GC), 3.0), ((GU, (GG, C), GC), 1.9), ((GU, (GU, C), GC), 3.0),
    ((GU, (UA, C), GC), 3.0), ((GU, (UC, C), GC), 3.0), ((GU, (UG, C), GC), 3.0), ((GU, (UU, C), GC), 2.2),
    // For internal loops between the base pairs "GU" and "UA".
    ((GU, (AA, C), UA), 3.7), ((GU, (AC, C), UA), 3.7), ((GU, (AG, C), UA), 3.7), ((GU, (AU, C), UA), 3.7),
    ((GU, (CA, C), UA), 3.7), ((GU, (CC, C), UA), 3.7), ((GU, (CG, C), UA), 3.7), ((GU, (CU, C), UA), 3.7),
    ((GU, (GA, C), UA), 2.6), ((GU, (GC, C), UA), 3.7), ((GU, (GG, C), UA), 2.6), ((GU, (GU, C), UA), 3.7),
    ((GU, (UA, C), UA), 3.7), ((GU, (UC, C), UA), 3.7), ((GU, (UG, C), UA), 3.7), ((GU, (UU, C), UA), 3.0),
    // For internal loops between the base pairs "GU" and "GU".
    ((GU, (AA, C), GU), 3.7), ((GU, (AC, C), GU), 3.7), ((GU, (AG, C), GU), 3.7), ((GU, (AU, C), GU), 3.7),
    ((GU, (CA, C), GU), 3.7), ((GU, (CC, C), GU), 3.7), ((GU, (CG, C), GU), 3.7), ((GU, (CU, C), GU), 3.7),
    ((GU, (GA, C), GU), 2.6), ((GU, (GC, C), GU), 3.7), ((GU, (GG, C), GU), 2.6), ((GU, (GU, C), GU), 3.7),
    ((GU, (UA, C), GU), 3.7), ((GU, (UC, C), GU), 3.7), ((GU, (UG, C), GU), 3.7), ((GU, (UU, C), GU), 3.0),
    // For internal loops between the base pairs "GU" and "UG".
    ((GU, (AA, C), UG), 3.7), ((GU, (AC, C), UG), 3.7), ((GU, (AG, C), UG), 3.7), ((GU, (AU, C), UG), 3.7),
    ((GU, (CA, C), UG), 3.7), ((GU, (CC, C), UG), 3.7), ((GU, (CG, C), UG), 3.7), ((GU, (CU, C), UG), 3.7),
    ((GU, (GA, C), UG), 2.6), ((GU, (GC, C), UG), 3.7), ((GU, (GG, C), UG), 2.6), ((GU, (GU, C), UG), 3.7),
    ((GU, (UA, C), UG), 3.7), ((GU, (UC, C), UG), 3.7), ((GU, (UG, C), UG), 3.7), ((GU, (UU, C), UG), 3.0),
    // For internal loops between the base pairs "GU" and "AU".
    ((GU, (AA, G), AU), 2.6), ((GU, (AC, G), AU), 2.6), ((GU, (AG, G), AU), 2.6), ((GU, (AU, G), AU), 2.6),
    ((GU, (CA, G), AU), 3.7), ((GU, (CC, G), AU), 3.7), ((GU, (CG, G), AU), 3.7), ((GU, (CU, G), AU), 3.7),
    ((GU, (GA, G), AU), 2.6), ((GU, (GC, G), AU), 2.6), ((GU, (GG, G), AU), 2.6), ((GU, (GU, G), AU), 2.6),
    ((GU, (UA, G), AU), 3.7), ((GU, (UC, G), AU), 3.7), ((GU, (UG, G), AU), 3.7), ((GU, (UU, G), AU), 3.0),
    // For internal loops between the base pairs "GU" and "CG".
    ((GU, (AA, G), CG), 1.9), ((GU, (AC, G), CG), 1.9), ((GU, (AG, G), CG), 1.9), ((GU, (AU, G), CG), 1.9),
    ((GU, (CA, G), CG), 3.0), ((GU, (CC, G), CG), 3.0), ((GU, (CG, G), CG), 3.0), ((GU, (CU, G), CG), 3.0),
    ((GU, (GA, G), CG), 1.9), ((GU, (GC, G), CG), 1.9), ((GU, (GG, G), CG), 1.9), ((GU, (GU, G), CG), 1.9),
    ((GU, (UA, G), CG), 3.0), ((GU, (UC, G), CG), 3.0), ((GU, (UG, G), CG), 3.0), ((GU, (UU, G), CG), 2.2),
    // For internal loops between the base pairs "GU" and "GC".
    ((GU, (AA, G), GC), 1.2), ((GU, (AC, G), GC), 1.9), ((GU, (AG, G), GC), 1.9), ((GU, (AU, G), GC), 1.9),
    ((GU, (CA, G), GC), 3.0), ((GU, (CC, G), GC), 3.0), ((GU, (CG, G), GC), 3.0), ((GU, (CU, G), GC), 3.0),
    ((GU, (GA, G), GC), 1.9), ((GU, (GC, G), GC), 1.9), ((GU, (GG, G), GC), 1.9), ((GU, (GU, G), GC), 1.9),
    ((GU, (UA, G), GC), 3.0), ((GU, (UC, G), GC), 3.0), ((GU, (UG, G), GC), 3.0), ((GU, (UU, G), GC), 2.2),
    // For internal loops between the base pairs "GU" and "UA".
    ((GU, (AA, G), UA), 2.6), ((GU, (AC, G), UA), 2.6), ((GU, (AG, G), UA), 2.6), ((GU, (AU, G), UA), 2.6),
    ((GU, (CA, G), UA), 3.7), ((GU, (CC, G), UA), 3.7), ((GU, (CG, G), UA), 3.7), ((GU, (CU, G), UA), 3.7),
    ((GU, (GA, G), UA), 2.6), ((GU, (GC, G), UA), 2.6), ((GU, (GG, G), UA), 2.6), ((GU, (GU, G), UA), 2.6),
    ((GU, (UA, G), UA), 3.7), ((GU, (UC, G), UA), 3.7), ((GU, (UG, G), UA), 3.7), ((GU, (UU, G), UA), 3.0),
    // For internal loops between the base pairs "GU" and "GU".
    ((GU, (AA, G), GU), 1.2), ((GU, (AC, G), GU), 2.6), ((GU, (AG, G), GU), 2.6), ((GU, (AU, G), GU), 2.6),
    ((GU, (CA, G), GU), 3.7), ((GU, (CC, G), GU), 3.7), ((GU, (CG, G), GU), 3.7), ((GU, (CU, G), GU), 3.7),
    ((GU, (GA, G), GU), 2.6), ((GU, (GC, G), GU), 2.6), ((GU, (GG, G), GU), 2.6), ((GU, (GU, G), GU), 2.6),
    ((GU, (UA, G), GU), 3.7), ((GU, (UC, G), GU), 3.7), ((GU, (UG, G), GU), 3.7), ((GU, (UU, G), GU), 3.0),
    // For internal loops between the base pairs "GU" and "UG".
    ((GU, (AA, G), UG), 2.6), ((GU, (AC, G), UG), 2.6), ((GU, (AG, G), UG), 2.6), ((GU, (AU, G), UG), 2.6),
    ((GU, (CA, G), UG), 3.7), ((GU, (CC, G), UG), 3.7), ((GU, (CG, G), UG), 3.7), ((GU, (CU, G), UG), 3.7),
    ((GU, (GA, G), UG), 2.6), ((GU, (GC, G), UG), 2.6), ((GU, (GG, G), UG), 2.6), ((GU, (GU, G), UG), 2.6),
    ((GU, (UA, G), UG), 3.7), ((GU, (UC, G), UG), 3.7), ((GU, (UG, G), UG), 3.7), ((GU, (UU, G), UG), 3.0),
    // For internal loops between the base pairs "GU" and "AU".
    ((GU, (AA, U), AU), 3.7), ((GU, (AC, U), AU), 3.7), ((GU, (AG, U), AU), 3.7), ((GU, (AU, U), AU), 3.7),
    ((GU, (CA, U), AU), 3.7), ((GU, (CC, U), AU), 3.7), ((GU, (CG, U), AU), 3.7), ((GU, (CU, U), AU), 3.7),
    ((GU, (GA, U), AU), 2.6), ((GU, (GC, U), AU), 3.7), ((GU, (GG, U), AU), 2.6), ((GU, (GU, U), AU), 3.7),
    ((GU, (UA, U), AU), 3.0), ((GU, (UC, U), AU), 3.0), ((GU, (UG, U), AU), 3.0), ((GU, (UU, U), AU), 3.0),
    // For internal loops between the base pairs "GU" and "CG".
    ((GU, (AA, U), CG), 3.0), ((GU, (AC, U), CG), 3.0), ((GU, (AG, U), CG), 3.0), ((GU, (AU, U), CG), 3.0),
    ((GU, (CA, U), CG), 3.0), ((GU, (CC, U), CG), 3.0), ((GU, (CG, U), CG), 3.0), ((GU, (CU, U), CG), 3.0),
    ((GU, (GA, U), CG), 1.9), ((GU, (GC, U), CG), 3.0), ((GU, (GG, U), CG), 1.9), ((GU, (GU, U), CG), 3.0),
    ((GU, (UA, U), CG), 2.2), ((GU, (UC, U), CG), 2.2), ((GU, (UG, U), CG), 2.2), ((GU, (UU, U), CG), 2.2),
    // For internal loops between the base pairs "GU" and "GC".
    ((GU, (AA, U), GC), 3.0), ((GU, (AC, U), GC), 3.0), ((GU, (AG, U), GC), 3.0), ((GU, (AU, U), GC), 3.0),
    ((GU, (CA, U), GC), 3.0), ((GU, (CC, U), GC), 1.9), ((GU, (CG, U), GC), 3.0), ((GU, (CU, U), GC), 3.0),
    ((GU, (GA, U), GC), 1.9), ((GU, (GC, U), GC), 3.0), ((GU, (GG, U), GC), 1.9), ((GU, (GU, U), GC), 3.0),
    ((GU, (UA, U), GC), 2.2), ((GU, (UC, U), GC), 2.2), ((GU, (UG, U), GC), 2.2), ((GU, (UU, U), GC), 2.2),
    // For internal loops between the base pairs "GU" and "UA".
    ((GU, (AA, U), UA), 3.7), ((GU, (AC, U), UA), 3.7), ((GU, (AG, U), UA), 3.7), ((GU, (AU, U), UA), 3.7),
    ((GU, (CA, U), UA), 3.7), ((GU, (CC, U), UA), 3.7), ((GU, (CG, U), UA), 3.7), ((GU, (CU, U), UA), 3.7),
    ((GU, (GA, U), UA), 2.6), ((GU, (GC, U), UA), 3.7), ((GU, (GG, U), UA), 2.6), ((GU, (GU, U), UA), 3.7),
    ((GU, (UA, U), UA), 3.0), ((GU, (UC, U), UA), 3.0), ((GU, (UG, U), UA), 3.0), ((GU, (UU, U), UA), 3.0),
    // For internal loops between the base pairs "GU" and "GU".
    ((GU, (AA, U), GU), 3.7), ((GU, (AC, U), GU), 3.7), ((GU, (AG, U), GU), 3.7), ((GU, (AU, U), GU), 3.7),
    ((GU, (CA, U), GU), 3.7), ((GU, (CC, U), GU), 1.9), ((GU, (CG, U), GU), 3.7), ((GU, (CU, U), GU), 3.7),
    ((GU, (GA, U), GU), 2.6), ((GU, (GC, U), GU), 3.7), ((GU, (GG, U), GU), 2.6), ((GU, (GU, U), GU), 3.7),
    ((GU, (UA, U), GU), 3.0), ((GU, (UC, U), GU), 3.0), ((GU, (UG, U), GU), 3.0), ((GU, (UU, U), GU), 3.0),
    // For internal loops between the base pairs "GU" and "UG".
    ((GU, (AA, U), UG), 3.7), ((GU, (AC, U), UG), 3.7), ((GU, (AG, U), UG), 3.7), ((GU, (AU, U), UG), 3.7),
    ((GU, (CA, U), UG), 3.7), ((GU, (CC, U), UG), 3.7), ((GU, (CG, U), UG), 3.7), ((GU, (CU, U), UG), 3.7),
    ((GU, (GA, U), UG), 2.6), ((GU, (GC, U), UG), 3.7), ((GU, (GG, U), UG), 2.6), ((GU, (GU, U), UG), 3.7),
    ((GU, (UA, U), UG), 3.0), ((GU, (UC, U), UG), 3.0), ((GU, (UG, U), UG), 3.0), ((GU, (UU, U), UG), 3.0),
    // For internal loops behind the base pair "UG".
    // For internal loops between the base pairs "UG" and "AU".
    ((UG, (AA, A), AU), 3.7), ((UG, (AC, A), AU), 3.7), ((UG, (AG, A), AU), 2.6), ((UG, (AU, A), AU), 3.7),
    ((UG, (CA, A), AU), 3.7), ((UG, (CC, A), AU), 3.7), ((UG, (CG, A), AU), 3.7), ((UG, (CU, A), AU), 3.7),
    ((UG, (GA, A), AU), 2.6), ((UG, (GC, A), AU), 2.6), ((UG, (GG, A), AU), 2.6), ((UG, (GU, A), AU), 2.6),
    ((UG, (UA, A), AU), 3.7), ((UG, (UC, A), AU), 3.7), ((UG, (UG, A), AU), 3.7), ((UG, (UU, A), AU), 3.0),
    // For internal loops between the base pairs "UG" and "CG".
    ((UG, (AA, A), CG), 3.0), ((UG, (AC, A), CG), 3.0), ((UG, (AG, A), CG), 1.9), ((UG, (AU, A), CG), 3.0),
    ((UG, (CA, A), CG), 3.0), ((UG, (CC, A), CG), 3.0), ((UG, (CG, A), CG), 3.0), ((UG, (CU, A), CG), 3.0),
    ((UG, (GA, A), CG), 1.9), ((UG, (GC, A), CG), 3.0), ((UG, (GG, A), CG), 1.9), ((UG, (GU, A), CG), 3.0),
    ((UG, (UA, A), CG), 3.0), ((UG, (UC, A), CG), 3.0), ((UG, (UG, A), CG), 3.0), ((UG, (UU, A), CG), 2.2),
    // For internal loops between the base pairs "UG" and "GC".
    ((UG, (AA, A), GC), 3.0), ((UG, (AC, A), GC), 3.0), ((UG, (AG, A), GC), 1.9), ((UG, (AU, A), GC), 3.0),
    ((UG, (CA, A), GC), 3.0), ((UG, (CC, A), GC), 3.0), ((UG, (CG, A), GC), 3.0), ((UG, (CU, A), GC), 3.0),
    ((UG, (GA, A), GC), 1.9), ((UG, (GC, A), GC), 1.9), ((UG, (GG, A), GC), 1.9), ((UG, (GU, A), GC), 1.9),
    ((UG, (UA, A), GC), 3.0), ((UG, (UC, A), GC), 3.0), ((UG, (UG, A), GC), 3.0), ((UG, (UU, A), GC), 2.2),
    // For internal loops between the base pairs "UG" and "UA".
    ((UG, (AA, A), UA), 3.7), ((UG, (AC, A), UA), 3.7), ((UG, (AG, A), UA), 2.6), ((UG, (AU, A), UA), 3.7),
    ((UG, (CA, A), UA), 3.7), ((UG, (CC, A), UA), 3.7), ((UG, (CG, A), UA), 3.7), ((UG, (CU, A), UA), 3.7),
    ((UG, (GA, A), UA), 2.6), ((UG, (GC, A), UA), 3.7), ((UG, (GG, A), UA), 2.6), ((UG, (GU, A), UA), 3.7),
    ((UG, (UA, A), UA), 3.7), ((UG, (UC, A), UA), 3.7), ((UG, (UG, A), UA), 3.7), ((UG, (UU, A), UA), 3.0),
    // For internal loops between the base pairs "UG" and "GU".
    ((UG, (AA, A), GU), 2.5), ((UG, (AC, A), GU), 3.7), ((UG, (AG, A), GU), 2.6), ((UG, (AU, A), GU), 3.7),
    ((UG, (CA, A), GU), 3.7), ((UG, (CC, A), GU), 3.7), ((UG, (CG, A), GU), 3.7), ((UG, (CU, A), GU), 3.7),
    ((UG, (GA, A), GU), 2.6), ((UG, (GC, A), GU), 2.6), ((UG, (GG, A), GU), 2.6), ((UG, (GU, A), GU), 2.6),
    ((UG, (UA, A), GU), 3.7), ((UG, (UC, A), GU), 3.7), ((UG, (UG, A), GU), 3.7), ((UG, (UU, A), GU), 3.0),
    // For internal loops between the base pairs "UG" and "UG".
    ((UG, (AA, A), UG), 3.7), ((UG, (AC, A), UG), 3.7), ((UG, (AG, A), UG), 2.6), ((UG, (AU, A), UG), 3.7),
    ((UG, (CA, A), UG), 3.7), ((UG, (CC, A), UG), 3.7), ((UG, (CG, A), UG), 3.7), ((UG, (CU, A), UG), 3.7),
    ((UG, (GA, A), UG), 2.6), ((UG, (GC, A), UG), 3.7), ((UG, (GG, A), UG), 2.6), ((UG, (GU, A), UG), 3.7),
    ((UG, (UA, A), UG), 3.7), ((UG, (UC, A), UG), 3.7), ((UG, (UG, A), UG), 3.7), ((UG, (UU, A), UG), 3.0),
    // For internal loops between the base pairs "UG" and "AU".
    ((UG, (AA, C), AU), 3.7), ((UG, (AC, C), AU), 3.7), ((UG, (AG, C), AU), 2.6), ((UG, (AU, C), AU), 3.7),
    ((UG, (CA, C), AU), 3.7), ((UG, (CC, C), AU), 3.7), ((UG, (CG, C), AU), 3.7), ((UG, (CU, C), AU), 3.7),
    ((UG, (GA, C), AU), 2.6), ((UG, (GC, C), AU), 3.7), ((UG, (GG, C), AU), 2.6), ((UG, (GU, C), AU), 3.7),
    ((UG, (UA, C), AU), 3.7), ((UG, (UC, C), AU), 3.7), ((UG, (UG, C), AU), 3.7), ((UG, (UU, C), AU), 3.0),
    // For internal loops between the base pairs "UG" and "CG".
    ((UG, (AA, C), CG), 3.0), ((UG, (AC, C), CG), 3.0), ((UG, (AG, C), CG), 1.9), ((UG, (AU, C), CG), 3.0),
    ((UG, (CA, C), CG), 3.0), ((UG, (CC, C), CG), 3.0), ((UG, (CG, C), CG), 3.0), ((UG, (CU, C), CG), 3.0),
    ((UG, (GA, C), CG), 1.9), ((UG, (GC, C), CG), 3.0), ((UG, (GG, C), CG), 1.9), ((UG, (GU, C), CG), 3.0),
    ((UG, (UA, C), CG), 3.0), ((UG, (UC, C), CG), 3.0), ((UG, (UG, C), CG), 3.0), ((UG, (UU, C), CG), 2.2),
    // For internal loops between the base pairs "UG" and "GC".
    ((UG, (AA, C), GC), 3.0), ((UG, (AC, C), GC), 3.0), ((UG, (AG, C), GC), 1.9), ((UG, (AU, C), GC), 3.0),
    ((UG, (CA, C), GC), 3.0), ((UG, (CC, C), GC), 3.0), ((UG, (CG, C), GC), 3.0), ((UG, (CU, C), GC), 3.0),
    ((UG, (GA, C), GC), 1.9), ((UG, (GC, C), GC), 3.0), ((UG, (GG, C), GC), 1.9), ((UG, (GU, C), GC), 3.0),
    ((UG, (UA, C), GC), 3.0), ((UG, (UC, C), GC), 3.0), ((UG, (UG, C), GC), 3.0), ((UG, (UU, C), GC), 2.2),
    // For internal loops between the base pairs "UG" and "UA".
    ((UG, (AA, C), UA), 3.7), ((UG, (AC, C), UA), 3.7), ((UG, (AG, C), UA), 2.6), ((UG, (AU, C), UA), 3.7),
    ((UG, (CA, C), UA), 3.7), ((UG, (CC, C), UA), 3.7), ((UG, (CG, C), UA), 3.7), ((UG, (CU, C), UA), 3.7),
    ((UG, (GA, C), UA), 2.6), ((UG, (GC, C), UA), 3.7), ((UG, (GG, C), UA), 2.6), ((UG, (GU, C), UA), 3.7),
    ((UG, (UA, C), UA), 3.7), ((UG, (UC, C), UA), 3.7), ((UG, (UG, C), UA), 3.7), ((UG, (UU, C), UA), 3.0),
    // For internal loops between the base pairs "UG" and "GU".
    ((UG, (AA, C), GU), 3.7), ((UG, (AC, C), GU), 3.7), ((UG, (AG, C), GU), 2.6), ((UG, (AU, C), GU), 3.7),
    ((UG, (CA, C), GU), 3.7), ((UG, (CC, C), GU), 3.7), ((UG, (CG, C), GU), 3.7), ((UG, (CU, C), GU), 3.7),
    ((UG, (GA, C), GU), 2.6), ((UG, (GC, C), GU), 3.7), ((UG, (GG, C), GU), 2.6), ((UG, (GU, C), GU), 3.7),
    ((UG, (UA, C), GU), 3.7), ((UG, (UC, C), GU), 3.7), ((UG, (UG, C), GU), 3.7), ((UG, (UU, C), GU), 3.0),
    // For internal loops between the base pairs "UG" and "UG".
    ((UG, (AA, C), UG), 3.7), ((UG, (AC, C), UG), 3.7), ((UG, (AG, C), UG), 2.6), ((UG, (AU, C), UG), 3.7),
    ((UG, (CA, C), UG), 3.7), ((UG, (CC, C), UG), 3.7), ((UG, (CG, C), UG), 3.7), ((UG, (CU, C), UG), 3.7),
    ((UG, (GA, C), UG), 2.6), ((UG, (GC, C), UG), 3.7), ((UG, (GG, C), UG), 2.6), ((UG, (GU, C), UG), 3.7),
    ((UG, (UA, C), UG), 3.7), ((UG, (UC, C), UG), 3.7), ((UG, (UG, C), UG), 3.7), ((UG, (UU, C), UG), 3.0),
    // For internal loops between the base pairs "UG" and "AU".
    ((UG, (AA, G), AU), 2.6), ((UG, (AC, G), AU), 2.6), ((UG, (AG, G), AU), 2.6), ((UG, (AU, G), AU), 2.6),
    ((UG, (CA, G), AU), 3.7), ((UG, (CC, G), AU), 3.7), ((UG, (CG, G), AU), 3.7), ((UG, (CU, G), AU), 3.7),
    ((UG, (GA, G), AU), 2.6), ((UG, (GC, G), AU), 2.6), ((UG, (GG, G), AU), 2.6), ((UG, (GU, G), AU), 2.6),
    ((UG, (UA, G), AU), 3.7), ((UG, (UC, G), AU), 3.7), ((UG, (UG, G), AU), 3.7), ((UG, (UU, G), AU), 3.0),
    // For internal loops between the base pairs "UG" and "CG".
    ((UG, (AA, G), CG), 1.9), ((UG, (AC, G), CG), 1.9), ((UG, (AG, G), CG), 1.9), ((UG, (AU, G), CG), 1.9),
    ((UG, (CA, G), CG), 3.0), ((UG, (CC, G), CG), 3.0), ((UG, (CG, G), CG), 3.0), ((UG, (CU, G), CG), 3.0),
    ((UG, (GA, G), CG), 1.9), ((UG, (GC, G), CG), 1.9), ((UG, (GG, G), CG), 1.9), ((UG, (GU, G), CG), 1.9),
    ((UG, (UA, G), CG), 3.0), ((UG, (UC, G), CG), 3.0), ((UG, (UG, G), CG), 3.0), ((UG, (UU, G), CG), 2.2),
    // For internal loops between the base pairs "UG" and "GC".
    ((UG, (AA, G), GC), 1.9), ((UG, (AC, G), GC), 1.9), ((UG, (AG, G), GC), 1.9), ((UG, (AU, G), GC), 1.9),
    ((UG, (CA, G), GC), 3.0), ((UG, (CC, G), GC), 3.0), ((UG, (CG, G), GC), 3.0), ((UG, (CU, G), GC), 3.0),
    ((UG, (GA, G), GC), 1.9), ((UG, (GC, G), GC), 1.9), ((UG, (GG, G), GC), 1.9), ((UG, (GU, G), GC), 1.9),
    ((UG, (UA, G), GC), 3.0), ((UG, (UC, G), GC), 3.0), ((UG, (UG, G), GC), 3.0), ((UG, (UU, G), GC), 2.2),
    // For internal loops between the base pairs "UG" and "UA".
    ((UG, (AA, G), UA), 2.6), ((UG, (AC, G), UA), 2.6), ((UG, (AG, G), UA), 2.6), ((UG, (AU, G), UA), 2.6),
    ((UG, (CA, G), UA), 3.7), ((UG, (CC, G), UA), 3.7), ((UG, (CG, G), UA), 3.7), ((UG, (CU, G), UA), 3.7),
    ((UG, (GA, G), UA), 2.6), ((UG, (GC, G), UA), 2.6), ((UG, (GG, G), UA), 2.6), ((UG, (GU, G), UA), 2.6),
    ((UG, (UA, G), UA), 3.7), ((UG, (UC, G), UA), 3.7), ((UG, (UG, G), UA), 3.7), ((UG, (UU, G), UA), 3.0),
    // For internal loops between the base pairs "UG" and "GU".
    ((UG, (AA, G), GU), 2.6), ((UG, (AC, G), GU), 2.6), ((UG, (AG, G), GU), 2.6), ((UG, (AU, G), GU), 2.6),
    ((UG, (CA, G), GU), 3.7), ((UG, (CC, G), GU), 3.7), ((UG, (CG, G), GU), 3.7), ((UG, (CU, G), GU), 3.7),
    ((UG, (GA, G), GU), 2.6), ((UG, (GC, G), GU), 2.6), ((UG, (GG, G), GU), 2.6), ((UG, (GU, G), GU), 2.6),
    ((UG, (UA, G), GU), 3.7), ((UG, (UC, G), GU), 3.7), ((UG, (UG, G), GU), 3.7), ((UG, (UU, G), GU), 3.0),
    // For internal loops between the base pairs "UG" and "UG".
    ((UG, (AA, G), UG), 2.6), ((UG, (AC, G), UG), 2.6), ((UG, (AG, G), UG), 2.6), ((UG, (AU, G), UG), 2.6),
    ((UG, (CA, G), UG), 3.7), ((UG, (CC, G), UG), 3.7), ((UG, (CG, G), UG), 3.7), ((UG, (CU, G), UG), 3.7),
    ((UG, (GA, G), UG), 2.6), ((UG, (GC, G), UG), 2.6), ((UG, (GG, G), UG), 2.6), ((UG, (GU, G), UG), 2.6),
    ((UG, (UA, G), UG), 3.7), ((UG, (UC, G), UG), 3.7), ((UG, (UG, G), UG), 3.7), ((UG, (UU, G), UG), 3.0),
    // For internal loops between the base pairs "UG" and "AU".
    ((UG, (AA, U), AU), 3.7), ((UG, (AC, U), AU), 3.7), ((UG, (AG, U), AU), 2.6), ((UG, (AU, U), AU), 3.7),
    ((UG, (CA, U), AU), 3.7), ((UG, (CC, U), AU), 3.7), ((UG, (CG, U), AU), 3.7), ((UG, (CU, U), AU), 3.7),
    ((UG, (GA, U), AU), 2.6), ((UG, (GC, U), AU), 3.7), ((UG, (GG, U), AU), 2.6), ((UG, (GU, U), AU), 3.7),
    ((UG, (UA, U), AU), 3.0), ((UG, (UC, U), AU), 3.0), ((UG, (UG, U), AU), 3.0), ((UG, (UU, U), AU), 3.0),
    // For internal loops between the base pairs "UG" and "CG".
    ((UG, (AA, U), CG), 3.0), ((UG, (AC, U), CG), 3.0), ((UG, (AG, U), CG), 1.9), ((UG, (AU, U), CG), 3.0),
    ((UG, (CA, U), CG), 3.0), ((UG, (CC, U), CG), 3.0), ((UG, (CG, U), CG), 3.0), ((UG, (CU, U), CG), 3.0),
    ((UG, (GA, U), CG), 1.9), ((UG, (GC, U), CG), 3.0), ((UG, (GG, U), CG), 1.9), ((UG, (GU, U), CG), 3.0),
    ((UG, (UA, U), CG), 2.2), ((UG, (UC, U), CG), 2.2), ((UG, (UG, U), CG), 2.2), ((UG, (UU, U), CG), 2.2),
    // For internal loops between the base pairs "UG" and "GC".
    ((UG, (AA, U), GC), 3.0), ((UG, (AC, U), GC), 3.0), ((UG, (AG, U), GC), 1.9), ((UG, (AU, U), GC), 3.0),
    ((UG, (CA, U), GC), 3.0), ((UG, (CC, U), GC), 3.0), ((UG, (CG, U), GC), 3.0), ((UG, (CU, U), GC), 3.0),
    ((UG, (GA, U), GC), 1.9), ((UG, (GC, U), GC), 3.0), ((UG, (GG, U), GC), 1.9), ((UG, (GU, U), GC), 3.0),
    ((UG, (UA, U), GC), 2.2), ((UG, (UC, U), GC), 2.2), ((UG, (UG, U), GC), 2.2), ((UG, (UU, U), GC), 2.2),
    // For internal loops between the base pairs "UG" and "UA".
    ((UG, (AA, U), UA), 3.7), ((UG, (AC, U), UA), 3.7), ((UG, (AG, U), UA), 2.6), ((UG, (AU, U), UA), 3.7),
    ((UG, (CA, U), UA), 3.7), ((UG, (CC, U), UA), 3.7), ((UG, (CG, U), UA), 3.7), ((UG, (CU, U), UA), 3.7),
    ((UG, (GA, U), UA), 2.6), ((UG, (GC, U), UA), 3.7), ((UG, (GG, U), UA), 2.6), ((UG, (GU, U), UA), 3.7),
    ((UG, (UA, U), UA), 3.0), ((UG, (UC, U), UA), 3.0), ((UG, (UG, U), UA), 3.0), ((UG, (UU, U), UA), 3.0),
    // For internal loops between the base pairs "UG" and "GU".
    ((UG, (AA, U), GU), 3.7), ((UG, (AC, U), GU), 3.7), ((UG, (AG, U), GU), 2.6), ((UG, (AU, U), GU), 3.7),
    ((UG, (CA, U), GU), 3.7), ((UG, (CC, U), GU), 3.7), ((UG, (CG, U), GU), 3.7), ((UG, (CU, U), GU), 3.7),
    ((UG, (GA, U), GU), 2.6), ((UG, (GC, U), GU), 3.7), ((UG, (GG, U), GU), 2.6), ((UG, (GU, U), GU), 3.7),
    ((UG, (UA, U), GU), 3.0), ((UG, (UC, U), GU), 3.0), ((UG, (UG, U), GU), 3.0), ((UG, (UU, U), GU), 3.0),
    // For internal loops between the base pairs "UG" and "UG".
    ((UG, (AA, U), UG), 3.7), ((UG, (AC, U), UG), 3.7), ((UG, (AG, U), UG), 2.6), ((UG, (AU, U), UG), 3.7),
    ((UG, (CA, U), UG), 3.7), ((UG, (CC, U), UG), 3.7), ((UG, (CG, U), UG), 3.7), ((UG, (CU, U), UG), 3.7),
    ((UG, (GA, U), UG), 2.6), ((UG, (GC, U), UG), 3.7), ((UG, (GG, U), UG), 2.6), ((UG, (GU, U), UG), 3.7),
    ((UG, (UA, U), UG), 3.0), ((UG, (UC, U), UG), 3.0), ((UG, (UG, U), UG), 3.0), ((UG, (UU, U), UG), 3.0),
  ].iter() {
    ONE_VS_2_IL_DELTA_FES[(x.0).0][(x.0).1][((x.1).0).0][((x.1).0).1][(x.1).1][(x.2).0][(x.2).1] = scale(y);
  }
  buf += &format!("pub const ONE_VS_2_IL_DELTA_FES: OneVs2IlDeltaFes = {:?};\n", &ONE_VS_2_IL_DELTA_FES);

  let mut TWO_VS_2_IL_DELTA_FES = [[[[[[[[NEG_INF; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
  for &(x, y) in [
    // For internal loops behind the base pair "AU".
    // For internal loops between the base pairs "AU" and "AU".
    ((AU, (AA, AA), AU), 2.8), ((AU, (AA, AC), AU), 2.3), ((AU, (AA, AG), AU), 1.7), ((AU, (AA, AU), AU), 2.3), ((AU, (AA, CA), AU), 2.8), ((AU, (AA, CC), AU), 2.8), ((AU, (AA, CG), AU), 2.8), ((AU, (AA, CU), AU), 2.8), ((AU, (AA, GA), AU), 1.8), ((AU, (AA, GC), AU), 2.3), ((AU, (AA, GG), AU), 1.2), ((AU, (AA, GU), AU), 2.3), ((AU, (AA, UA), AU), 2.8), ((AU, (AA, UC), AU), 2.5), ((AU, (AA, UG), AU), 2.8), ((AU, (AA, UU), AU), 2.5),
    ((AU, (AC, AA), AU), 2.6), ((AU, (AC, AC), AU), 2.2), ((AU, (AC, AG), AU), 1.6), ((AU, (AC, AU), AU), 2.2), ((AU, (AC, CA), AU), 2.6), ((AU, (AC, CC), AU), 2.6), ((AU, (AC, CG), AU), 2.6), ((AU, (AC, CU), AU), 2.6), ((AU, (AC, GA), AU), 1.7), ((AU, (AC, GC), AU), 2.2), ((AU, (AC, GG), AU), 1.1), ((AU, (AC, GU), AU), 2.2), ((AU, (AC, UA), AU), 2.6), ((AU, (AC, UC), AU), 2.3), ((AU, (AC, UG), AU), 2.6), ((AU, (AC, UU), AU), 1.8),
    ((AU, (AG, AA), AU), 2.2), ((AU, (AG, AC), AU), 1.8), ((AU, (AG, AG), AU), 1.2), ((AU, (AG, AU), AU), 1.8), ((AU, (AG, CA), AU), 2.2), ((AU, (AG, CC), AU), 2.8), ((AU, (AG, CG), AU), 2.2), ((AU, (AG, CU), AU), 2.8), ((AU, (AG, GA), AU), 1.3), ((AU, (AG, GC), AU), 1.8), ((AU, (AG, GG), AU), 2.0), ((AU, (AG, GU), AU), 1.8), ((AU, (AG, UA), AU), 2.2), ((AU, (AG, UC), AU), 2.5), ((AU, (AG, UG), AU), 2.2), ((AU, (AG, UU), AU), 1.4),
    ((AU, (AU, AA), AU), 2.6), ((AU, (AU, AC), AU), 2.2), ((AU, (AU, AG), AU), 1.6), ((AU, (AU, AU), AU), 2.2), ((AU, (AU, CA), AU), 2.6), ((AU, (AU, CC), AU), 2.6), ((AU, (AU, CG), AU), 2.6), ((AU, (AU, CU), AU), 2.6), ((AU, (AU, GA), AU), 1.7), ((AU, (AU, GC), AU), 2.2), ((AU, (AU, GG), AU), 1.1), ((AU, (AU, GU), AU), 2.2), ((AU, (AU, UA), AU), 2.6), ((AU, (AU, UC), AU), 2.3), ((AU, (AU, UG), AU), 2.6), ((AU, (AU, UU), AU), 1.8),
    ((AU, (CA, AA), AU), 2.5), ((AU, (CA, AC), AU), 2.1), ((AU, (CA, AG), AU), 1.5), ((AU, (CA, AU), AU), 2.1), ((AU, (CA, CA), AU), 2.5), ((AU, (CA, CC), AU), 2.5), ((AU, (CA, CG), AU), 2.5), ((AU, (CA, CU), AU), 2.5), ((AU, (CA, GA), AU), 1.6), ((AU, (CA, GC), AU), 2.1), ((AU, (CA, GG), AU), 1.0), ((AU, (CA, GU), AU), 2.1), ((AU, (CA, UA), AU), 2.5), ((AU, (CA, UC), AU), 2.2), ((AU, (CA, UG), AU), 2.5), ((AU, (CA, UU), AU), 1.7),
    ((AU, (CC, AA), AU), 2.6), ((AU, (CC, AC), AU), 2.1), ((AU, (CC, AG), AU), 2.1), ((AU, (CC, AU), AU), 2.1), ((AU, (CC, CA), AU), 2.6), ((AU, (CC, CC), AU), 2.6), ((AU, (CC, CG), AU), 2.6), ((AU, (CC, CU), AU), 2.6), ((AU, (CC, GA), AU), 2.2), ((AU, (CC, GC), AU), 2.1), ((AU, (CC, GG), AU), 1.0), ((AU, (CC, GU), AU), 2.1), ((AU, (CC, UA), AU), 2.6), ((AU, (CC, UC), AU), 2.3), ((AU, (CC, UG), AU), 2.6), ((AU, (CC, UU), AU), 1.7),
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
    ((AU, (AA, AA), CG), 2.1), ((AU, (AA, AC), CG), 1.9), ((AU, (AA, AG), CG), 0.1), ((AU, (AA, AU), CG), 1.9), ((AU, (AA, CA), CG), 1.8), ((AU, (AA, CC), CG), 1.9), ((AU, (AA, CG), CG), 1.8), ((AU, (AA, CU), CG), 1.9), ((AU, (AA, GA), CG), 0.7), ((AU, (AA, GC), CG), 1.9), ((AU, (AA, GG), CG), 0.5), ((AU, (AA, GU), CG), 1.9), ((AU, (AA, UA), CG), 1.8), ((AU, (AA, UC), CG), 1.9), ((AU, (AA, UG), CG), 1.8), ((AU, (AA, UU), CG), 1.7),
    ((AU, (AC, AA), CG), 2.0), ((AU, (AC, AC), CG), 1.7), ((AU, (AC, AG), CG), 0.0), ((AU, (AC, AU), CG), 1.7), ((AU, (AC, CA), CG), 1.7), ((AU, (AC, CC), CG), 1.7), ((AU, (AC, CG), CG), 1.7), ((AU, (AC, CU), CG), 1.7), ((AU, (AC, GA), CG), 0.6), ((AU, (AC, GC), CG), 1.7), ((AU, (AC, GG), CG), 0.3), ((AU, (AC, GU), CG), 1.7), ((AU, (AC, UA), CG), 1.7), ((AU, (AC, UC), CG), 1.8), ((AU, (AC, UG), CG), 1.7), ((AU, (AC, UU), CG), 1.0),
    ((AU, (AG, AA), CG), 1.6), ((AU, (AG, AC), CG), 1.3), ((AU, (AG, AG), CG), -0.4), ((AU, (AG, AU), CG), 1.3), ((AU, (AG, CA), CG), 1.3), ((AU, (AG, CC), CG), 1.9), ((AU, (AG, CG), CG), 1.3), ((AU, (AG, CU), CG), 1.9), ((AU, (AG, GA), CG), 0.2), ((AU, (AG, GC), CG), 1.3), ((AU, (AG, GG), CG), 1.2), ((AU, (AG, GU), CG), 1.3), ((AU, (AG, UA), CG), 1.3), ((AU, (AG, UC), CG), 2.0), ((AU, (AG, UG), CG), 1.3), ((AU, (AG, UU), CG), 0.6),
    ((AU, (AU, AA), CG), 2.0), ((AU, (AU, AC), CG), 1.7), ((AU, (AU, AG), CG), 0.0), ((AU, (AU, AU), CG), 1.7), ((AU, (AU, CA), CG), 1.7), ((AU, (AU, CC), CG), 1.7), ((AU, (AU, CG), CG), 1.7), ((AU, (AU, CU), CG), 1.7), ((AU, (AU, GA), CG), 0.6), ((AU, (AU, GC), CG), 1.7), ((AU, (AU, GG), CG), 0.3), ((AU, (AU, GU), CG), 1.7), ((AU, (AU, UA), CG), 1.7), ((AU, (AU, UC), CG), 1.8), ((AU, (AU, UG), CG), 1.7), ((AU, (AU, UU), CG), 1.0),
    ((AU, (CA, AA), CG), 1.9), ((AU, (CA, AC), CG), 1.6), ((AU, (CA, AG), CG), -0.1), ((AU, (CA, AU), CG), 1.6), ((AU, (CA, CA), CG), 1.6), ((AU, (CA, CC), CG), 1.6), ((AU, (CA, CG), CG), 1.6), ((AU, (CA, CU), CG), 1.6), ((AU, (CA, GA), CG), 0.5), ((AU, (CA, GC), CG), 1.6), ((AU, (CA, GG), CG), 0.2), ((AU, (CA, GU), CG), 1.6), ((AU, (CA, UA), CG), 1.6), ((AU, (CA, UC), CG), 1.7), ((AU, (CA, UG), CG), 1.6), ((AU, (CA, UU), CG), 0.9),
    ((AU, (CC, AA), CG), 1.9), ((AU, (CC, AC), CG), 1.7), ((AU, (CC, AG), CG), 0.5), ((AU, (CC, AU), CG), 1.7), ((AU, (CC, CA), CG), 1.6), ((AU, (CC, CC), CG), 1.7), ((AU, (CC, CG), CG), 1.6), ((AU, (CC, CU), CG), 1.7), ((AU, (CC, GA), CG), 1.1), ((AU, (CC, GC), CG), 1.7), ((AU, (CC, GG), CG), 0.3), ((AU, (CC, GU), CG), 1.7), ((AU, (CC, UA), CG), 1.6), ((AU, (CC, UC), CG), 1.7), ((AU, (CC, UG), CG), 1.6), ((AU, (CC, UU), CG), 0.9),
    ((AU, (CG, AA), CG), 1.9), ((AU, (CG, AC), CG), 1.6), ((AU, (CG, AG), CG), -0.1), ((AU, (CG, AU), CG), 1.6), ((AU, (CG, CA), CG), 1.6), ((AU, (CG, CC), CG), 1.6), ((AU, (CG, CG), CG), 1.6), ((AU, (CG, CU), CG), 1.6), ((AU, (CG, GA), CG), 0.5), ((AU, (CG, GC), CG), 1.6), ((AU, (CG, GG), CG), 0.2), ((AU, (CG, GU), CG), 1.6), ((AU, (CG, UA), CG), 1.6), ((AU, (CG, UC), CG), 1.7), ((AU, (CG, UG), CG), 1.6), ((AU, (CG, UU), CG), 0.9),
    ((AU, (CU, AA), CG), 1.9), ((AU, (CU, AC), CG), 1.7), ((AU, (CU, AG), CG), 0.5), ((AU, (CU, AU), CG), 1.7), ((AU, (CU, CA), CG), 1.6), ((AU, (CU, CC), CG), 1.7), ((AU, (CU, CG), CG), 1.6), ((AU, (CU, CU), CG), 1.7), ((AU, (CU, GA), CG), 1.1), ((AU, (CU, GC), CG), 1.7), ((AU, (CU, GG), CG), 0.3), ((AU, (CU, GU), CG), 1.7), ((AU, (CU, UA), CG), 1.6), ((AU, (CU, UC), CG), 1.7), ((AU, (CU, UG), CG), 1.6), ((AU, (CU, UU), CG), 0.9),
    ((AU, (GA, AA), CG), 0.9), ((AU, (GA, AC), CG), 0.6), ((AU, (GA, AG), CG), -1.1), ((AU, (GA, AU), CG), 0.6), ((AU, (GA, CA), CG), 0.6), ((AU, (GA, CC), CG), 1.2), ((AU, (GA, CG), CG), 0.6), ((AU, (GA, CU), CG), 1.2), ((AU, (GA, GA), CG), -0.5), ((AU, (GA, GC), CG), 0.6), ((AU, (GA, GG), CG), 0.5), ((AU, (GA, GU), CG), 0.6), ((AU, (GA, UA), CG), 0.6), ((AU, (GA, UC), CG), 1.3), ((AU, (GA, UG), CG), 0.6), ((AU, (GA, UU), CG), -0.1),
    ((AU, (GC, AA), CG), 2.0), ((AU, (GC, AC), CG), 1.7), ((AU, (GC, AG), CG), 0.0), ((AU, (GC, AU), CG), 1.7), ((AU, (GC, CA), CG), 1.7), ((AU, (GC, CC), CG), 1.7), ((AU, (GC, CG), CG), 1.7), ((AU, (GC, CU), CG), 1.7), ((AU, (GC, GA), CG), 0.6), ((AU, (GC, GC), CG), 1.7), ((AU, (GC, GG), CG), 0.3), ((AU, (GC, GU), CG), 1.7), ((AU, (GC, UA), CG), 1.7), ((AU, (GC, UC), CG), 1.8), ((AU, (GC, UG), CG), 1.7), ((AU, (GC, UU), CG), 1.0),
    ((AU, (GG, AA), CG), 0.4), ((AU, (GG, AC), CG), 0.1), ((AU, (GG, AG), CG), -0.3), ((AU, (GG, AU), CG), 0.1), ((AU, (GG, CA), CG), 0.1), ((AU, (GG, CC), CG), 0.1), ((AU, (GG, CG), CG), 0.1), ((AU, (GG, CU), CG), 0.1), ((AU, (GG, GA), CG), 0.3), ((AU, (GG, GC), CG), 0.1), ((AU, (GG, GG), CG), 1.3), ((AU, (GG, GU), CG), 0.1), ((AU, (GG, UA), CG), 0.1), ((AU, (GG, UC), CG), 0.2), ((AU, (GG, UG), CG), 0.1), ((AU, (GG, UU), CG), 0.7),
    ((AU, (GU, AA), CG), 2.0), ((AU, (GU, AC), CG), 1.7), ((AU, (GU, AG), CG), 0.0), ((AU, (GU, AU), CG), 1.7), ((AU, (GU, CA), CG), 1.7), ((AU, (GU, CC), CG), 1.7), ((AU, (GU, CG), CG), 1.7), ((AU, (GU, CU), CG), 1.7), ((AU, (GU, GA), CG), 0.6), ((AU, (GU, GC), CG), 1.7), ((AU, (GU, GG), CG), 0.3), ((AU, (GU, GU), CG), 1.7), ((AU, (GU, UA), CG), 1.7), ((AU, (GU, UC), CG), 1.8), ((AU, (GU, UG), CG), 1.7), ((AU, (GU, UU), CG), 1.0),
    ((AU, (UA, AA), CG), 1.9), ((AU, (UA, AC), CG), 1.6), ((AU, (UA, AG), CG), -0.1), ((AU, (UA, AU), CG), 1.6), ((AU, (UA, CA), CG), 1.6), ((AU, (UA, CC), CG), 1.6), ((AU, (UA, CG), CG), 1.6), ((AU, (UA, CU), CG), 1.6), ((AU, (UA, GA), CG), 0.5), ((AU, (UA, GC), CG), 1.6), ((AU, (UA, GG), CG), 0.2), ((AU, (UA, GU), CG), 1.6), ((AU, (UA, UA), CG), 1.6), ((AU, (UA, UC), CG), 1.7), ((AU, (UA, UG), CG), 1.6), ((AU, (UA, UU), CG), 0.9),
    ((AU, (UC, AA), CG), 1.9), ((AU, (UC, AC), CG), 1.7), ((AU, (UC, AG), CG), 0.5), ((AU, (UC, AU), CG), 1.7), ((AU, (UC, CA), CG), 1.6), ((AU, (UC, CC), CG), 1.7), ((AU, (UC, CG), CG), 1.6), ((AU, (UC, CU), CG), 1.7), ((AU, (UC, GA), CG), 1.1), ((AU, (UC, GC), CG), 1.7), ((AU, (UC, GG), CG), 0.3), ((AU, (UC, GU), CG), 1.7), ((AU, (UC, UA), CG), 1.6), ((AU, (UC, UC), CG), 1.7), ((AU, (UC, UG), CG), 1.6), ((AU, (UC, UU), CG), 0.9),
    ((AU, (UG, AA), CG), 1.9), ((AU, (UG, AC), CG), 1.6), ((AU, (UG, AG), CG), -0.1), ((AU, (UG, AU), CG), 1.6), ((AU, (UG, CA), CG), 1.6), ((AU, (UG, CC), CG), 1.6), ((AU, (UG, CG), CG), 1.6), ((AU, (UG, CU), CG), 1.6), ((AU, (UG, GA), CG), 0.5), ((AU, (UG, GC), CG), 1.6), ((AU, (UG, GG), CG), 0.2), ((AU, (UG, GU), CG), 1.6), ((AU, (UG, UA), CG), 1.6), ((AU, (UG, UC), CG), 1.7), ((AU, (UG, UG), CG), 1.6), ((AU, (UG, UU), CG), 0.9),
    ((AU, (UU, AA), CG), 1.6), ((AU, (UU, AC), CG), 0.8), ((AU, (UU, AG), CG), -1.0), ((AU, (UU, AU), CG), 0.8), ((AU, (UU, CA), CG), 0.7), ((AU, (UU, CC), CG), 0.8), ((AU, (UU, CG), CG), 0.7), ((AU, (UU, CU), CG), 0.8), ((AU, (UU, GA), CG), -0.3), ((AU, (UU, GC), CG), 0.8), ((AU, (UU, GG), CG), 0.7), ((AU, (UU, GU), CG), 0.8), ((AU, (UU, UA), CG), 0.7), ((AU, (UU, UC), CG), 0.8), ((AU, (UU, UG), CG), 0.7), ((AU, (UU, UU), CG), 0.0),
    // For internal loops between the base pairs "AU" and "GC".
    ((AU, (AA, AA), GC), 2.0), ((AU, (AA, AC), GC), 1.9), ((AU, (AA, AG), GC), 1.0), ((AU, (AA, AU), GC), 1.9), ((AU, (AA, CA), GC), 2.4), ((AU, (AA, CC), GC), 2.2), ((AU, (AA, CG), GC), 2.4), ((AU, (AA, CU), GC), 2.1), ((AU, (AA, GA), GC), 1.0), ((AU, (AA, GC), GC), 1.9), ((AU, (AA, GG), GC), 0.5), ((AU, (AA, GU), GC), 1.9), ((AU, (AA, UA), GC), 2.4), ((AU, (AA, UC), GC), 2.1), ((AU, (AA, UG), GC), 2.4), ((AU, (AA, UU), GC), 1.8),
    ((AU, (AC, AA), GC), 1.8), ((AU, (AC, AC), GC), 1.8), ((AU, (AC, AG), GC), 0.9), ((AU, (AC, AU), GC), 1.8), ((AU, (AC, CA), GC), 2.2), ((AU, (AC, CC), GC), 2.1), ((AU, (AC, CG), GC), 2.2), ((AU, (AC, CU), GC), 1.9), ((AU, (AC, GA), GC), 0.9), ((AU, (AC, GC), GC), 1.8), ((AU, (AC, GG), GC), 0.3), ((AU, (AC, GU), GC), 1.8), ((AU, (AC, UA), GC), 2.2), ((AU, (AC, UC), GC), 1.9), ((AU, (AC, UG), GC), 2.2), ((AU, (AC, UU), GC), 1.0),
    ((AU, (AG, AA), GC), 1.4), ((AU, (AG, AC), GC), 1.4), ((AU, (AG, AG), GC), 0.5), ((AU, (AG, AU), GC), 1.4), ((AU, (AG, CA), GC), 1.8), ((AU, (AG, CC), GC), 2.3), ((AU, (AG, CG), GC), 1.8), ((AU, (AG, CU), GC), 2.1), ((AU, (AG, GA), GC), 0.5), ((AU, (AG, GC), GC), 1.4), ((AU, (AG, GG), GC), 1.2), ((AU, (AG, GU), GC), 1.4), ((AU, (AG, UA), GC), 1.8), ((AU, (AG, UC), GC), 2.1), ((AU, (AG, UG), GC), 1.8), ((AU, (AG, UU), GC), 0.6),
    ((AU, (AU, AA), GC), 1.8), ((AU, (AU, AC), GC), 1.8), ((AU, (AU, AG), GC), 0.9), ((AU, (AU, AU), GC), 1.8), ((AU, (AU, CA), GC), 2.2), ((AU, (AU, CC), GC), 2.1), ((AU, (AU, CG), GC), 2.2), ((AU, (AU, CU), GC), 1.9), ((AU, (AU, GA), GC), 0.9), ((AU, (AU, GC), GC), 1.8), ((AU, (AU, GG), GC), 0.3), ((AU, (AU, GU), GC), 1.8), ((AU, (AU, UA), GC), 2.2), ((AU, (AU, UC), GC), 1.9), ((AU, (AU, UG), GC), 2.2), ((AU, (AU, UU), GC), 1.0),
    ((AU, (CA, AA), GC), 1.7), ((AU, (CA, AC), GC), 1.7), ((AU, (CA, AG), GC), 0.8), ((AU, (CA, AU), GC), 1.7), ((AU, (CA, CA), GC), 2.1), ((AU, (CA, CC), GC), 2.0), ((AU, (CA, CG), GC), 2.1), ((AU, (CA, CU), GC), 1.8), ((AU, (CA, GA), GC), 0.8), ((AU, (CA, GC), GC), 1.7), ((AU, (CA, GG), GC), 0.2), ((AU, (CA, GU), GC), 1.7), ((AU, (CA, UA), GC), 2.1), ((AU, (CA, UC), GC), 1.8), ((AU, (CA, UG), GC), 2.1), ((AU, (CA, UU), GC), 0.9),
    ((AU, (CC, AA), GC), 1.8), ((AU, (CC, AC), GC), 1.7), ((AU, (CC, AG), GC), 1.4), ((AU, (CC, AU), GC), 1.7), ((AU, (CC, CA), GC), 2.2), ((AU, (CC, CC), GC), 2.0), ((AU, (CC, CG), GC), 2.2), ((AU, (CC, CU), GC), 1.9), ((AU, (CC, GA), GC), 1.4), ((AU, (CC, GC), GC), 1.7), ((AU, (CC, GG), GC), 0.3), ((AU, (CC, GU), GC), 1.7), ((AU, (CC, UA), GC), 2.2), ((AU, (CC, UC), GC), 1.9), ((AU, (CC, UG), GC), 2.2), ((AU, (CC, UU), GC), 1.0),
    ((AU, (CG, AA), GC), 1.7), ((AU, (CG, AC), GC), 1.7), ((AU, (CG, AG), GC), 0.8), ((AU, (CG, AU), GC), 1.7), ((AU, (CG, CA), GC), 2.1), ((AU, (CG, CC), GC), 2.0), ((AU, (CG, CG), GC), 2.1), ((AU, (CG, CU), GC), 1.8), ((AU, (CG, GA), GC), 0.8), ((AU, (CG, GC), GC), 1.7), ((AU, (CG, GG), GC), 0.2), ((AU, (CG, GU), GC), 1.7), ((AU, (CG, UA), GC), 2.1), ((AU, (CG, UC), GC), 1.8), ((AU, (CG, UG), GC), 2.1), ((AU, (CG, UU), GC), 0.9),
    ((AU, (CU, AA), GC), 1.8), ((AU, (CU, AC), GC), 1.7), ((AU, (CU, AG), GC), 1.4), ((AU, (CU, AU), GC), 1.7), ((AU, (CU, CA), GC), 2.2), ((AU, (CU, CC), GC), 2.0), ((AU, (CU, CG), GC), 2.2), ((AU, (CU, CU), GC), 1.9), ((AU, (CU, GA), GC), 1.4), ((AU, (CU, GC), GC), 1.7), ((AU, (CU, GG), GC), 0.3), ((AU, (CU, GU), GC), 1.7), ((AU, (CU, UA), GC), 2.2), ((AU, (CU, UC), GC), 1.9), ((AU, (CU, UG), GC), 2.2), ((AU, (CU, UU), GC), 1.0),
    ((AU, (GA, AA), GC), 0.7), ((AU, (GA, AC), GC), 0.7), ((AU, (GA, AG), GC), -0.2), ((AU, (GA, AU), GC), 0.7), ((AU, (GA, CA), GC), 1.1), ((AU, (GA, CC), GC), 1.6), ((AU, (GA, CG), GC), 1.1), ((AU, (GA, CU), GC), 1.4), ((AU, (GA, GA), GC), -0.2), ((AU, (GA, GC), GC), 0.7), ((AU, (GA, GG), GC), 0.5), ((AU, (GA, GU), GC), 0.7), ((AU, (GA, UA), GC), 1.1), ((AU, (GA, UC), GC), 1.4), ((AU, (GA, UG), GC), 1.1), ((AU, (GA, UU), GC), 0.0),
    ((AU, (GC, AA), GC), 1.8), ((AU, (GC, AC), GC), 1.8), ((AU, (GC, AG), GC), 0.9), ((AU, (GC, AU), GC), 1.8), ((AU, (GC, CA), GC), 2.2), ((AU, (GC, CC), GC), 2.1), ((AU, (GC, CG), GC), 2.2), ((AU, (GC, CU), GC), 1.9), ((AU, (GC, GA), GC), 0.9), ((AU, (GC, GC), GC), 1.8), ((AU, (GC, GG), GC), 0.3), ((AU, (GC, GU), GC), 1.8), ((AU, (GC, UA), GC), 2.2), ((AU, (GC, UC), GC), 1.9), ((AU, (GC, UG), GC), 2.2), ((AU, (GC, UU), GC), 1.0),
    ((AU, (GG, AA), GC), 0.2), ((AU, (GG, AC), GC), 0.2), ((AU, (GG, AG), GC), 0.6), ((AU, (GG, AU), GC), 0.2), ((AU, (GG, CA), GC), 0.6), ((AU, (GG, CC), GC), 0.5), ((AU, (GG, CG), GC), 0.6), ((AU, (GG, CU), GC), 0.3), ((AU, (GG, GA), GC), 0.6), ((AU, (GG, GC), GC), 0.2), ((AU, (GG, GG), GC), 1.3), ((AU, (GG, GU), GC), 0.2), ((AU, (GG, UA), GC), 0.6), ((AU, (GG, UC), GC), 0.3), ((AU, (GG, UG), GC), 0.6), ((AU, (GG, UU), GC), 0.7),
    ((AU, (GU, AA), GC), 1.8), ((AU, (GU, AC), GC), 1.8), ((AU, (GU, AG), GC), 0.9), ((AU, (GU, AU), GC), 1.8), ((AU, (GU, CA), GC), 2.2), ((AU, (GU, CC), GC), 2.1), ((AU, (GU, CG), GC), 2.2), ((AU, (GU, CU), GC), 1.9), ((AU, (GU, GA), GC), 0.9), ((AU, (GU, GC), GC), 1.8), ((AU, (GU, GG), GC), 0.3), ((AU, (GU, GU), GC), 1.8), ((AU, (GU, UA), GC), 2.2), ((AU, (GU, UC), GC), 1.9), ((AU, (GU, UG), GC), 2.2), ((AU, (GU, UU), GC), 1.0),
    ((AU, (UA, AA), GC), 1.7), ((AU, (UA, AC), GC), 1.7), ((AU, (UA, AG), GC), 0.8), ((AU, (UA, AU), GC), 1.7), ((AU, (UA, CA), GC), 2.1), ((AU, (UA, CC), GC), 2.0), ((AU, (UA, CG), GC), 2.1), ((AU, (UA, CU), GC), 1.8), ((AU, (UA, GA), GC), 0.8), ((AU, (UA, GC), GC), 1.7), ((AU, (UA, GG), GC), 0.2), ((AU, (UA, GU), GC), 1.7), ((AU, (UA, UA), GC), 2.1), ((AU, (UA, UC), GC), 1.8), ((AU, (UA, UG), GC), 2.1), ((AU, (UA, UU), GC), 0.9),
    ((AU, (UC, AA), GC), 1.8), ((AU, (UC, AC), GC), 1.7), ((AU, (UC, AG), GC), 1.4), ((AU, (UC, AU), GC), 1.7), ((AU, (UC, CA), GC), 2.2), ((AU, (UC, CC), GC), 2.0), ((AU, (UC, CG), GC), 2.2), ((AU, (UC, CU), GC), 1.9), ((AU, (UC, GA), GC), 1.4), ((AU, (UC, GC), GC), 1.7), ((AU, (UC, GG), GC), 0.3), ((AU, (UC, GU), GC), 1.7), ((AU, (UC, UA), GC), 2.2), ((AU, (UC, UC), GC), 1.9), ((AU, (UC, UG), GC), 2.2), ((AU, (UC, UU), GC), 1.0),
    ((AU, (UG, AA), GC), 1.7), ((AU, (UG, AC), GC), 1.7), ((AU, (UG, AG), GC), 0.8), ((AU, (UG, AU), GC), 1.7), ((AU, (UG, CA), GC), 2.1), ((AU, (UG, CC), GC), 2.0), ((AU, (UG, CG), GC), 2.1), ((AU, (UG, CU), GC), 1.8), ((AU, (UG, GA), GC), 0.8), ((AU, (UG, GC), GC), 1.7), ((AU, (UG, GG), GC), 0.2), ((AU, (UG, GU), GC), 1.7), ((AU, (UG, UA), GC), 2.1), ((AU, (UG, UC), GC), 1.8), ((AU, (UG, UG), GC), 2.1), ((AU, (UG, UU), GC), 0.9),
    ((AU, (UU, AA), GC), 1.5), ((AU, (UU, AC), GC), 0.8), ((AU, (UU, AG), GC), 0.0), ((AU, (UU, AU), GC), 0.8), ((AU, (UU, CA), GC), 1.3), ((AU, (UU, CC), GC), 1.1), ((AU, (UU, CG), GC), 1.3), ((AU, (UU, CU), GC), 1.0), ((AU, (UU, GA), GC), 0.0), ((AU, (UU, GC), GC), 0.8), ((AU, (UU, GG), GC), 0.7), ((AU, (UU, GU), GC), 0.8), ((AU, (UU, UA), GC), 1.3), ((AU, (UU, UC), GC), 1.0), ((AU, (UU, UG), GC), 1.3), ((AU, (UU, UU), GC), 0.1),
    // For internal loops between the base pairs "AU" and "GU".
    ((AU, (AA, AA), GU), 2.4), ((AU, (AA, AC), GU), 2.8), ((AU, (AA, AG), GU), 1.4), ((AU, (AA, AU), GU), 2.8), ((AU, (AA, CA), GU), 2.8), ((AU, (AA, CC), GU), 2.8), ((AU, (AA, CG), GU), 2.8), ((AU, (AA, CU), GU), 2.8), ((AU, (AA, GA), GU), 3.1), ((AU, (AA, GC), GU), 2.8), ((AU, (AA, GG), GU), 1.5), ((AU, (AA, GU), GU), 2.8), ((AU, (AA, UA), GU), 2.8), ((AU, (AA, UC), GU), 2.8), ((AU, (AA, UG), GU), 2.8), ((AU, (AA, UU), GU), 3.4),
    ((AU, (AC, AA), GU), 2.3), ((AU, (AC, AC), GU), 2.6), ((AU, (AC, AG), GU), 1.3), ((AU, (AC, AU), GU), 2.6), ((AU, (AC, CA), GU), 2.6), ((AU, (AC, CC), GU), 2.6), ((AU, (AC, CG), GU), 2.6), ((AU, (AC, CU), GU), 2.6), ((AU, (AC, GA), GU), 2.9), ((AU, (AC, GC), GU), 2.6), ((AU, (AC, GG), GU), 1.3), ((AU, (AC, GU), GU), 2.6), ((AU, (AC, UA), GU), 2.6), ((AU, (AC, UC), GU), 2.6), ((AU, (AC, UG), GU), 2.6), ((AU, (AC, UU), GU), 2.6),
    ((AU, (AG, AA), GU), 1.9), ((AU, (AG, AC), GU), 2.2), ((AU, (AG, AG), GU), 0.9), ((AU, (AG, AU), GU), 2.2), ((AU, (AG, CA), GU), 2.2), ((AU, (AG, CC), GU), 2.8), ((AU, (AG, CG), GU), 2.2), ((AU, (AG, CU), GU), 2.8), ((AU, (AG, GA), GU), 2.5), ((AU, (AG, GC), GU), 2.2), ((AU, (AG, GG), GU), 2.2), ((AU, (AG, GU), GU), 2.2), ((AU, (AG, UA), GU), 2.2), ((AU, (AG, UC), GU), 2.8), ((AU, (AG, UG), GU), 2.2), ((AU, (AG, UU), GU), 2.2),
    ((AU, (AU, AA), GU), 2.3), ((AU, (AU, AC), GU), 2.6), ((AU, (AU, AG), GU), 1.3), ((AU, (AU, AU), GU), 2.6), ((AU, (AU, CA), GU), 2.6), ((AU, (AU, CC), GU), 2.6), ((AU, (AU, CG), GU), 2.6), ((AU, (AU, CU), GU), 2.6), ((AU, (AU, GA), GU), 2.9), ((AU, (AU, GC), GU), 2.6), ((AU, (AU, GG), GU), 1.3), ((AU, (AU, GU), GU), 2.6), ((AU, (AU, UA), GU), 2.6), ((AU, (AU, UC), GU), 2.6), ((AU, (AU, UG), GU), 2.6), ((AU, (AU, UU), GU), 2.6),
    ((AU, (CA, AA), GU), 2.2), ((AU, (CA, AC), GU), 2.5), ((AU, (CA, AG), GU), 1.2), ((AU, (CA, AU), GU), 2.5), ((AU, (CA, CA), GU), 2.5), ((AU, (CA, CC), GU), 2.5), ((AU, (CA, CG), GU), 2.5), ((AU, (CA, CU), GU), 2.5), ((AU, (CA, GA), GU), 2.8), ((AU, (CA, GC), GU), 2.5), ((AU, (CA, GG), GU), 1.2), ((AU, (CA, GU), GU), 2.5), ((AU, (CA, UA), GU), 2.5), ((AU, (CA, UC), GU), 2.5), ((AU, (CA, UG), GU), 2.5), ((AU, (CA, UU), GU), 2.5),
    ((AU, (CC, AA), GU), 2.2), ((AU, (CC, AC), GU), 2.6), ((AU, (CC, AG), GU), 1.8), ((AU, (CC, AU), GU), 2.6), ((AU, (CC, CA), GU), 2.6), ((AU, (CC, CC), GU), 2.6), ((AU, (CC, CG), GU), 2.6), ((AU, (CC, CU), GU), 2.6), ((AU, (CC, GA), GU), 3.5), ((AU, (CC, GC), GU), 2.6), ((AU, (CC, GG), GU), 1.3), ((AU, (CC, GU), GU), 2.6), ((AU, (CC, UA), GU), 2.6), ((AU, (CC, UC), GU), 2.6), ((AU, (CC, UG), GU), 2.6), ((AU, (CC, UU), GU), 2.6),
    ((AU, (CG, AA), GU), 2.2), ((AU, (CG, AC), GU), 2.5), ((AU, (CG, AG), GU), 1.2), ((AU, (CG, AU), GU), 2.5), ((AU, (CG, CA), GU), 2.5), ((AU, (CG, CC), GU), 2.5), ((AU, (CG, CG), GU), 2.5), ((AU, (CG, CU), GU), 2.5), ((AU, (CG, GA), GU), 2.8), ((AU, (CG, GC), GU), 2.5), ((AU, (CG, GG), GU), 1.2), ((AU, (CG, GU), GU), 2.5), ((AU, (CG, UA), GU), 2.5), ((AU, (CG, UC), GU), 2.5), ((AU, (CG, UG), GU), 2.5), ((AU, (CG, UU), GU), 2.5),
    ((AU, (CU, AA), GU), 2.2), ((AU, (CU, AC), GU), 2.6), ((AU, (CU, AG), GU), 1.8), ((AU, (CU, AU), GU), 2.6), ((AU, (CU, CA), GU), 2.6), ((AU, (CU, CC), GU), 2.6), ((AU, (CU, CG), GU), 2.6), ((AU, (CU, CU), GU), 2.6), ((AU, (CU, GA), GU), 3.5), ((AU, (CU, GC), GU), 2.6), ((AU, (CU, GG), GU), 1.3), ((AU, (CU, GU), GU), 2.6), ((AU, (CU, UA), GU), 2.6), ((AU, (CU, UC), GU), 2.6), ((AU, (CU, UG), GU), 2.6), ((AU, (CU, UU), GU), 2.6),
    ((AU, (GA, AA), GU), 1.2), ((AU, (GA, AC), GU), 1.5), ((AU, (GA, AG), GU), 0.2), ((AU, (GA, AU), GU), 1.5), ((AU, (GA, CA), GU), 1.5), ((AU, (GA, CC), GU), 2.1), ((AU, (GA, CG), GU), 1.5), ((AU, (GA, CU), GU), 2.1), ((AU, (GA, GA), GU), 1.8), ((AU, (GA, GC), GU), 1.5), ((AU, (GA, GG), GU), 1.5), ((AU, (GA, GU), GU), 1.5), ((AU, (GA, UA), GU), 1.5), ((AU, (GA, UC), GU), 2.1), ((AU, (GA, UG), GU), 1.5), ((AU, (GA, UU), GU), 1.5),
    ((AU, (GC, AA), GU), 2.3), ((AU, (GC, AC), GU), 2.6), ((AU, (GC, AG), GU), 1.3), ((AU, (GC, AU), GU), 2.6), ((AU, (GC, CA), GU), 2.6), ((AU, (GC, CC), GU), 2.6), ((AU, (GC, CG), GU), 2.6), ((AU, (GC, CU), GU), 2.6), ((AU, (GC, GA), GU), 2.9), ((AU, (GC, GC), GU), 2.6), ((AU, (GC, GG), GU), 1.3), ((AU, (GC, GU), GU), 2.6), ((AU, (GC, UA), GU), 2.6), ((AU, (GC, UC), GU), 2.6), ((AU, (GC, UG), GU), 2.6), ((AU, (GC, UU), GU), 2.6),
    ((AU, (GG, AA), GU), 0.7), ((AU, (GG, AC), GU), 1.0), ((AU, (GG, AG), GU), 1.0), ((AU, (GG, AU), GU), 1.0), ((AU, (GG, CA), GU), 1.0), ((AU, (GG, CC), GU), 1.0), ((AU, (GG, CG), GU), 1.0), ((AU, (GG, CU), GU), 1.0), ((AU, (GG, GA), GU), 2.6), ((AU, (GG, GC), GU), 1.0), ((AU, (GG, GG), GU), 2.3), ((AU, (GG, GU), GU), 1.0), ((AU, (GG, UA), GU), 1.0), ((AU, (GG, UC), GU), 1.0), ((AU, (GG, UG), GU), 1.0), ((AU, (GG, UU), GU), 2.3),
    ((AU, (GU, AA), GU), 2.3), ((AU, (GU, AC), GU), 2.6), ((AU, (GU, AG), GU), 1.3), ((AU, (GU, AU), GU), 2.6), ((AU, (GU, CA), GU), 2.6), ((AU, (GU, CC), GU), 2.6), ((AU, (GU, CG), GU), 2.6), ((AU, (GU, CU), GU), 2.6), ((AU, (GU, GA), GU), 2.9), ((AU, (GU, GC), GU), 2.6), ((AU, (GU, GG), GU), 1.3), ((AU, (GU, GU), GU), 2.6), ((AU, (GU, UA), GU), 2.6), ((AU, (GU, UC), GU), 2.6), ((AU, (GU, UG), GU), 2.6), ((AU, (GU, UU), GU), 2.6),
    ((AU, (UA, AA), GU), 2.2), ((AU, (UA, AC), GU), 2.5), ((AU, (UA, AG), GU), 1.2), ((AU, (UA, AU), GU), 2.5), ((AU, (UA, CA), GU), 2.5), ((AU, (UA, CC), GU), 2.5), ((AU, (UA, CG), GU), 2.5), ((AU, (UA, CU), GU), 2.5), ((AU, (UA, GA), GU), 2.8), ((AU, (UA, GC), GU), 2.5), ((AU, (UA, GG), GU), 1.2), ((AU, (UA, GU), GU), 2.5), ((AU, (UA, UA), GU), 2.5), ((AU, (UA, UC), GU), 2.5), ((AU, (UA, UG), GU), 2.5), ((AU, (UA, UU), GU), 2.5),
    ((AU, (UC, AA), GU), 2.2), ((AU, (UC, AC), GU), 2.6), ((AU, (UC, AG), GU), 1.8), ((AU, (UC, AU), GU), 2.6), ((AU, (UC, CA), GU), 2.6), ((AU, (UC, CC), GU), 2.6), ((AU, (UC, CG), GU), 2.6), ((AU, (UC, CU), GU), 2.6), ((AU, (UC, GA), GU), 3.5), ((AU, (UC, GC), GU), 2.6), ((AU, (UC, GG), GU), 1.3), ((AU, (UC, GU), GU), 2.6), ((AU, (UC, UA), GU), 2.6), ((AU, (UC, UC), GU), 2.6), ((AU, (UC, UG), GU), 2.6), ((AU, (UC, UU), GU), 2.6),
    ((AU, (UG, AA), GU), 2.2), ((AU, (UG, AC), GU), 2.5), ((AU, (UG, AG), GU), 1.2), ((AU, (UG, AU), GU), 2.5), ((AU, (UG, CA), GU), 2.5), ((AU, (UG, CC), GU), 2.5), ((AU, (UG, CG), GU), 2.5), ((AU, (UG, CU), GU), 2.5), ((AU, (UG, GA), GU), 2.8), ((AU, (UG, GC), GU), 2.5), ((AU, (UG, GG), GU), 1.2), ((AU, (UG, GU), GU), 2.5), ((AU, (UG, UA), GU), 2.5), ((AU, (UG, UC), GU), 2.5), ((AU, (UG, UG), GU), 2.5), ((AU, (UG, UU), GU), 2.5),
    ((AU, (UU, AA), GU), 1.9), ((AU, (UU, AC), GU), 1.7), ((AU, (UU, AG), GU), 0.3), ((AU, (UU, AU), GU), 1.7), ((AU, (UU, CA), GU), 1.7), ((AU, (UU, CC), GU), 1.7), ((AU, (UU, CG), GU), 1.7), ((AU, (UU, CU), GU), 1.7), ((AU, (UU, GA), GU), 2.0), ((AU, (UU, GC), GU), 1.7), ((AU, (UU, GG), GU), 1.7), ((AU, (UU, GU), GU), 1.7), ((AU, (UU, UA), GU), 1.7), ((AU, (UU, UC), GU), 1.7), ((AU, (UU, UG), GU), 1.7), ((AU, (UU, UU), GU), 1.7),
    // For internal loops between the base pairs "AU" and "UA".
    ((AU, (AA, AA), UA), 2.8), ((AU, (AA, AC), UA), 2.5), ((AU, (AA, AG), UA), 1.5), ((AU, (AA, AU), UA), 2.5), ((AU, (AA, CA), UA), 2.6), ((AU, (AA, CC), UA), 2.6), ((AU, (AA, CG), UA), 2.8), ((AU, (AA, CU), UA), 2.6), ((AU, (AA, GA), UA), 2.2), ((AU, (AA, GC), UA), 2.5), ((AU, (AA, GG), UA), 1.0), ((AU, (AA, GU), UA), 2.5), ((AU, (AA, UA), UA), 2.6), ((AU, (AA, UC), UA), 2.6), ((AU, (AA, UG), UA), 2.6), ((AU, (AA, UU), UA), 2.3),
    ((AU, (AC, AA), UA), 2.6), ((AU, (AC, AC), UA), 2.4), ((AU, (AC, AG), UA), 1.4), ((AU, (AC, AU), UA), 2.4), ((AU, (AC, CA), UA), 2.5), ((AU, (AC, CC), UA), 2.4), ((AU, (AC, CG), UA), 2.5), ((AU, (AC, CU), UA), 2.4), ((AU, (AC, GA), UA), 2.1), ((AU, (AC, GC), UA), 2.4), ((AU, (AC, GG), UA), 0.9), ((AU, (AC, GU), UA), 2.4), ((AU, (AC, UA), UA), 2.5), ((AU, (AC, UC), UA), 2.4), ((AU, (AC, UG), UA), 2.5), ((AU, (AC, UU), UA), 1.5),
    ((AU, (AG, AA), UA), 2.2), ((AU, (AG, AC), UA), 2.0), ((AU, (AG, AG), UA), 1.0), ((AU, (AG, AU), UA), 2.0), ((AU, (AG, CA), UA), 2.1), ((AU, (AG, CC), UA), 2.6), ((AU, (AG, CG), UA), 2.1), ((AU, (AG, CU), UA), 2.6), ((AU, (AG, GA), UA), 1.7), ((AU, (AG, GC), UA), 2.0), ((AU, (AG, GG), UA), 1.8), ((AU, (AG, GU), UA), 2.0), ((AU, (AG, UA), UA), 2.1), ((AU, (AG, UC), UA), 2.6), ((AU, (AG, UG), UA), 2.1), ((AU, (AG, UU), UA), 1.1),
    ((AU, (AU, AA), UA), 2.6), ((AU, (AU, AC), UA), 2.4), ((AU, (AU, AG), UA), 1.4), ((AU, (AU, AU), UA), 2.4), ((AU, (AU, CA), UA), 2.5), ((AU, (AU, CC), UA), 2.4), ((AU, (AU, CG), UA), 2.5), ((AU, (AU, CU), UA), 2.4), ((AU, (AU, GA), UA), 2.1), ((AU, (AU, GC), UA), 2.4), ((AU, (AU, GG), UA), 0.9), ((AU, (AU, GU), UA), 2.4), ((AU, (AU, UA), UA), 2.5), ((AU, (AU, UC), UA), 2.4), ((AU, (AU, UG), UA), 2.5), ((AU, (AU, UU), UA), 1.5),
    ((AU, (CA, AA), UA), 2.5), ((AU, (CA, AC), UA), 2.3), ((AU, (CA, AG), UA), 1.3), ((AU, (CA, AU), UA), 2.3), ((AU, (CA, CA), UA), 2.4), ((AU, (CA, CC), UA), 2.3), ((AU, (CA, CG), UA), 2.4), ((AU, (CA, CU), UA), 2.3), ((AU, (CA, GA), UA), 2.0), ((AU, (CA, GC), UA), 2.3), ((AU, (CA, GG), UA), 0.8), ((AU, (CA, GU), UA), 2.3), ((AU, (CA, UA), UA), 2.4), ((AU, (CA, UC), UA), 2.3), ((AU, (CA, UG), UA), 2.4), ((AU, (CA, UU), UA), 1.4),
    ((AU, (CC, AA), UA), 2.6), ((AU, (CC, AC), UA), 2.3), ((AU, (CC, AG), UA), 1.9), ((AU, (CC, AU), UA), 2.3), ((AU, (CC, CA), UA), 2.4), ((AU, (CC, CC), UA), 2.4), ((AU, (CC, CG), UA), 2.4), ((AU, (CC, CU), UA), 2.4), ((AU, (CC, GA), UA), 2.6), ((AU, (CC, GC), UA), 2.3), ((AU, (CC, GG), UA), 0.8), ((AU, (CC, GU), UA), 2.3), ((AU, (CC, UA), UA), 2.4), ((AU, (CC, UC), UA), 2.4), ((AU, (CC, UG), UA), 2.4), ((AU, (CC, UU), UA), 1.5),
    ((AU, (CG, AA), UA), 2.5), ((AU, (CG, AC), UA), 2.3), ((AU, (CG, AG), UA), 1.3), ((AU, (CG, AU), UA), 2.3), ((AU, (CG, CA), UA), 2.4), ((AU, (CG, CC), UA), 2.3), ((AU, (CG, CG), UA), 2.4), ((AU, (CG, CU), UA), 2.3), ((AU, (CG, GA), UA), 2.0), ((AU, (CG, GC), UA), 2.3), ((AU, (CG, GG), UA), 0.8), ((AU, (CG, GU), UA), 2.3), ((AU, (CG, UA), UA), 2.4), ((AU, (CG, UC), UA), 2.3), ((AU, (CG, UG), UA), 2.4), ((AU, (CG, UU), UA), 1.4),
    ((AU, (CU, AA), UA), 2.6), ((AU, (CU, AC), UA), 2.3), ((AU, (CU, AG), UA), 1.9), ((AU, (CU, AU), UA), 2.3), ((AU, (CU, CA), UA), 2.4), ((AU, (CU, CC), UA), 2.4), ((AU, (CU, CG), UA), 2.4), ((AU, (CU, CU), UA), 2.4), ((AU, (CU, GA), UA), 2.6), ((AU, (CU, GC), UA), 2.3), ((AU, (CU, GG), UA), 0.8), ((AU, (CU, GU), UA), 2.3), ((AU, (CU, UA), UA), 2.4), ((AU, (CU, UC), UA), 2.4), ((AU, (CU, UG), UA), 2.4), ((AU, (CU, UU), UA), 1.5),
    ((AU, (GA, AA), UA), 1.5), ((AU, (GA, AC), UA), 1.3), ((AU, (GA, AG), UA), 0.3), ((AU, (GA, AU), UA), 1.3), ((AU, (GA, CA), UA), 1.4), ((AU, (GA, CC), UA), 1.9), ((AU, (GA, CG), UA), 1.4), ((AU, (GA, CU), UA), 1.9), ((AU, (GA, GA), UA), 1.0), ((AU, (GA, GC), UA), 1.3), ((AU, (GA, GG), UA), 1.1), ((AU, (GA, GU), UA), 1.3), ((AU, (GA, UA), UA), 1.4), ((AU, (GA, UC), UA), 1.9), ((AU, (GA, UG), UA), 1.4), ((AU, (GA, UU), UA), 0.4),
    ((AU, (GC, AA), UA), 2.6), ((AU, (GC, AC), UA), 2.4), ((AU, (GC, AG), UA), 1.4), ((AU, (GC, AU), UA), 2.4), ((AU, (GC, CA), UA), 2.5), ((AU, (GC, CC), UA), 2.4), ((AU, (GC, CG), UA), 2.5), ((AU, (GC, CU), UA), 2.4), ((AU, (GC, GA), UA), 2.1), ((AU, (GC, GC), UA), 2.4), ((AU, (GC, GG), UA), 0.9), ((AU, (GC, GU), UA), 2.4), ((AU, (GC, UA), UA), 2.5), ((AU, (GC, UC), UA), 2.4), ((AU, (GC, UG), UA), 2.5), ((AU, (GC, UU), UA), 1.5),
    ((AU, (GG, AA), UA), 1.0), ((AU, (GG, AC), UA), 0.8), ((AU, (GG, AG), UA), 1.1), ((AU, (GG, AU), UA), 0.8), ((AU, (GG, CA), UA), 0.9), ((AU, (GG, CC), UA), 0.8), ((AU, (GG, CG), UA), 0.9), ((AU, (GG, CU), UA), 0.8), ((AU, (GG, GA), UA), 1.8), ((AU, (GG, GC), UA), 0.8), ((AU, (GG, GG), UA), 1.9), ((AU, (GG, GU), UA), 0.8), ((AU, (GG, UA), UA), 0.9), ((AU, (GG, UC), UA), 0.8), ((AU, (GG, UG), UA), 0.9), ((AU, (GG, UU), UA), 1.2),
    ((AU, (GU, AA), UA), 2.6), ((AU, (GU, AC), UA), 2.4), ((AU, (GU, AG), UA), 1.4), ((AU, (GU, AU), UA), 2.4), ((AU, (GU, CA), UA), 2.5), ((AU, (GU, CC), UA), 2.4), ((AU, (GU, CG), UA), 2.5), ((AU, (GU, CU), UA), 2.4), ((AU, (GU, GA), UA), 2.1), ((AU, (GU, GC), UA), 2.4), ((AU, (GU, GG), UA), 0.9), ((AU, (GU, GU), UA), 2.4), ((AU, (GU, UA), UA), 2.5), ((AU, (GU, UC), UA), 2.4), ((AU, (GU, UG), UA), 2.5), ((AU, (GU, UU), UA), 1.5),
    ((AU, (UA, AA), UA), 2.5), ((AU, (UA, AC), UA), 2.3), ((AU, (UA, AG), UA), 1.3), ((AU, (UA, AU), UA), 2.3), ((AU, (UA, CA), UA), 2.4), ((AU, (UA, CC), UA), 2.3), ((AU, (UA, CG), UA), 2.4), ((AU, (UA, CU), UA), 2.3), ((AU, (UA, GA), UA), 2.0), ((AU, (UA, GC), UA), 2.3), ((AU, (UA, GG), UA), 0.8), ((AU, (UA, GU), UA), 2.3), ((AU, (UA, UA), UA), 2.4), ((AU, (UA, UC), UA), 2.3), ((AU, (UA, UG), UA), 2.4), ((AU, (UA, UU), UA), 1.4),
    ((AU, (UC, AA), UA), 2.6), ((AU, (UC, AC), UA), 2.3), ((AU, (UC, AG), UA), 1.9), ((AU, (UC, AU), UA), 2.3), ((AU, (UC, CA), UA), 2.4), ((AU, (UC, CC), UA), 2.4), ((AU, (UC, CG), UA), 2.4), ((AU, (UC, CU), UA), 2.4), ((AU, (UC, GA), UA), 2.6), ((AU, (UC, GC), UA), 2.3), ((AU, (UC, GG), UA), 0.8), ((AU, (UC, GU), UA), 2.3), ((AU, (UC, UA), UA), 2.4), ((AU, (UC, UC), UA), 2.4), ((AU, (UC, UG), UA), 2.4), ((AU, (UC, UU), UA), 1.5),
    ((AU, (UG, AA), UA), 2.5), ((AU, (UG, AC), UA), 2.3), ((AU, (UG, AG), UA), 1.3), ((AU, (UG, AU), UA), 2.3), ((AU, (UG, CA), UA), 2.4), ((AU, (UG, CC), UA), 2.3), ((AU, (UG, CG), UA), 2.4), ((AU, (UG, CU), UA), 2.3), ((AU, (UG, GA), UA), 2.0), ((AU, (UG, GC), UA), 2.3), ((AU, (UG, GG), UA), 0.8), ((AU, (UG, GU), UA), 2.3), ((AU, (UG, UA), UA), 2.4), ((AU, (UG, UC), UA), 2.3), ((AU, (UG, UG), UA), 2.4), ((AU, (UG, UU), UA), 1.4),
    ((AU, (UU, AA), UA), 2.3), ((AU, (UU, AC), UA), 1.4), ((AU, (UU, AG), UA), 0.4), ((AU, (UU, AU), UA), 1.4), ((AU, (UU, CA), UA), 1.5), ((AU, (UU, CC), UA), 1.5), ((AU, (UU, CG), UA), 1.5), ((AU, (UU, CU), UA), 1.5), ((AU, (UU, GA), UA), 1.1), ((AU, (UU, GC), UA), 1.4), ((AU, (UU, GG), UA), 1.2), ((AU, (UU, GU), UA), 1.4), ((AU, (UU, UA), UA), 1.5), ((AU, (UU, UC), UA), 1.5), ((AU, (UU, UG), UA), 1.5), ((AU, (UU, UU), UA), 0.6),
    // For internal loops between the base pairs "AU" and "UG".
    ((AU, (AA, AA), UG), 3.4), ((AU, (AA, AC), UG), 3.1), ((AU, (AA, AG), UG), 2.3), ((AU, (AA, AU), UG), 3.1), ((AU, (AA, CA), UG), 3.1), ((AU, (AA, CC), UG), 3.1), ((AU, (AA, CG), UG), 3.1), ((AU, (AA, CU), UG), 3.1), ((AU, (AA, GA), UG), 2.7), ((AU, (AA, GC), UG), 3.1), ((AU, (AA, GG), UG), 1.8), ((AU, (AA, GU), UG), 3.1), ((AU, (AA, UA), UG), 3.1), ((AU, (AA, UC), UG), 3.1), ((AU, (AA, UG), UG), 3.1), ((AU, (AA, UU), UG), 3.7),
    ((AU, (AC, AA), UG), 3.3), ((AU, (AC, AC), UG), 2.9), ((AU, (AC, AG), UG), 2.1), ((AU, (AC, AU), UG), 2.9), ((AU, (AC, CA), UG), 2.9), ((AU, (AC, CC), UG), 2.9), ((AU, (AC, CG), UG), 2.9), ((AU, (AC, CU), UG), 2.9), ((AU, (AC, GA), UG), 2.5), ((AU, (AC, GC), UG), 2.9), ((AU, (AC, GG), UG), 1.6), ((AU, (AC, GU), UG), 2.9), ((AU, (AC, UA), UG), 2.9), ((AU, (AC, UC), UG), 2.9), ((AU, (AC, UG), UG), 2.9), ((AU, (AC, UU), UG), 2.9),
    ((AU, (AG, AA), UG), 2.9), ((AU, (AG, AC), UG), 2.5), ((AU, (AG, AG), UG), 1.7), ((AU, (AG, AU), UG), 2.5), ((AU, (AG, CA), UG), 2.5), ((AU, (AG, CC), UG), 3.1), ((AU, (AG, CG), UG), 2.5), ((AU, (AG, CU), UG), 3.1), ((AU, (AG, GA), UG), 2.1), ((AU, (AG, GC), UG), 2.5), ((AU, (AG, GG), UG), 2.5), ((AU, (AG, GU), UG), 2.5), ((AU, (AG, UA), UG), 2.5), ((AU, (AG, UC), UG), 3.1), ((AU, (AG, UG), UG), 2.5), ((AU, (AG, UU), UG), 2.5),
    ((AU, (AU, AA), UG), 3.3), ((AU, (AU, AC), UG), 2.9), ((AU, (AU, AG), UG), 2.1), ((AU, (AU, AU), UG), 2.9), ((AU, (AU, CA), UG), 2.9), ((AU, (AU, CC), UG), 2.9), ((AU, (AU, CG), UG), 2.9), ((AU, (AU, CU), UG), 2.9), ((AU, (AU, GA), UG), 2.5), ((AU, (AU, GC), UG), 2.9), ((AU, (AU, GG), UG), 1.6), ((AU, (AU, GU), UG), 2.9), ((AU, (AU, UA), UG), 2.9), ((AU, (AU, UC), UG), 2.9), ((AU, (AU, UG), UG), 2.9), ((AU, (AU, UU), UG), 2.9),
    ((AU, (CA, AA), UG), 3.2), ((AU, (CA, AC), UG), 2.8), ((AU, (CA, AG), UG), 2.0), ((AU, (CA, AU), UG), 2.8), ((AU, (CA, CA), UG), 2.8), ((AU, (CA, CC), UG), 2.8), ((AU, (CA, CG), UG), 2.8), ((AU, (CA, CU), UG), 2.8), ((AU, (CA, GA), UG), 2.4), ((AU, (CA, GC), UG), 2.8), ((AU, (CA, GG), UG), 1.5), ((AU, (CA, GU), UG), 2.8), ((AU, (CA, UA), UG), 2.8), ((AU, (CA, UC), UG), 2.8), ((AU, (CA, UG), UG), 2.8), ((AU, (CA, UU), UG), 2.8),
    ((AU, (CC, AA), UG), 3.2), ((AU, (CC, AC), UG), 2.9), ((AU, (CC, AG), UG), 2.7), ((AU, (CC, AU), UG), 2.9), ((AU, (CC, CA), UG), 2.9), ((AU, (CC, CC), UG), 2.9), ((AU, (CC, CG), UG), 2.9), ((AU, (CC, CU), UG), 2.9), ((AU, (CC, GA), UG), 3.1), ((AU, (CC, GC), UG), 2.9), ((AU, (CC, GG), UG), 1.6), ((AU, (CC, GU), UG), 2.9), ((AU, (CC, UA), UG), 2.9), ((AU, (CC, UC), UG), 2.9), ((AU, (CC, UG), UG), 2.9), ((AU, (CC, UU), UG), 2.9),
    ((AU, (CG, AA), UG), 3.2), ((AU, (CG, AC), UG), 2.8), ((AU, (CG, AG), UG), 2.0), ((AU, (CG, AU), UG), 2.8), ((AU, (CG, CA), UG), 2.8), ((AU, (CG, CC), UG), 2.8), ((AU, (CG, CG), UG), 2.8), ((AU, (CG, CU), UG), 2.8), ((AU, (CG, GA), UG), 2.4), ((AU, (CG, GC), UG), 2.8), ((AU, (CG, GG), UG), 1.5), ((AU, (CG, GU), UG), 2.8), ((AU, (CG, UA), UG), 2.8), ((AU, (CG, UC), UG), 2.8), ((AU, (CG, UG), UG), 2.8), ((AU, (CG, UU), UG), 2.8),
    ((AU, (CU, AA), UG), 3.2), ((AU, (CU, AC), UG), 2.9), ((AU, (CU, AG), UG), 2.7), ((AU, (CU, AU), UG), 2.9), ((AU, (CU, CA), UG), 2.9), ((AU, (CU, CC), UG), 2.9), ((AU, (CU, CG), UG), 2.9), ((AU, (CU, CU), UG), 2.9), ((AU, (CU, GA), UG), 3.1), ((AU, (CU, GC), UG), 2.9), ((AU, (CU, GG), UG), 1.6), ((AU, (CU, GU), UG), 2.9), ((AU, (CU, UA), UG), 2.9), ((AU, (CU, UC), UG), 2.9), ((AU, (CU, UG), UG), 2.9), ((AU, (CU, UU), UG), 2.9),
    ((AU, (GA, AA), UG), 2.2), ((AU, (GA, AC), UG), 1.8), ((AU, (GA, AG), UG), 1.0), ((AU, (GA, AU), UG), 1.8), ((AU, (GA, CA), UG), 1.8), ((AU, (GA, CC), UG), 2.4), ((AU, (GA, CG), UG), 1.8), ((AU, (GA, CU), UG), 2.4), ((AU, (GA, GA), UG), 1.4), ((AU, (GA, GC), UG), 1.8), ((AU, (GA, GG), UG), 1.8), ((AU, (GA, GU), UG), 1.8), ((AU, (GA, UA), UG), 1.8), ((AU, (GA, UC), UG), 2.4), ((AU, (GA, UG), UG), 1.8), ((AU, (GA, UU), UG), 1.8),
    ((AU, (GC, AA), UG), 3.3), ((AU, (GC, AC), UG), 2.9), ((AU, (GC, AG), UG), 2.1), ((AU, (GC, AU), UG), 2.9), ((AU, (GC, CA), UG), 2.9), ((AU, (GC, CC), UG), 2.9), ((AU, (GC, CG), UG), 2.9), ((AU, (GC, CU), UG), 2.9), ((AU, (GC, GA), UG), 2.5), ((AU, (GC, GC), UG), 2.9), ((AU, (GC, GG), UG), 1.6), ((AU, (GC, GU), UG), 2.9), ((AU, (GC, UA), UG), 2.9), ((AU, (GC, UC), UG), 2.9), ((AU, (GC, UG), UG), 2.9), ((AU, (GC, UU), UG), 2.9),
    ((AU, (GG, AA), UG), 1.7), ((AU, (GG, AC), UG), 1.3), ((AU, (GG, AG), UG), 1.8), ((AU, (GG, AU), UG), 1.3), ((AU, (GG, CA), UG), 1.3), ((AU, (GG, CC), UG), 1.3), ((AU, (GG, CG), UG), 1.3), ((AU, (GG, CU), UG), 1.3), ((AU, (GG, GA), UG), 2.2), ((AU, (GG, GC), UG), 1.3), ((AU, (GG, GG), UG), 2.6), ((AU, (GG, GU), UG), 1.3), ((AU, (GG, UA), UG), 1.3), ((AU, (GG, UC), UG), 1.3), ((AU, (GG, UG), UG), 1.3), ((AU, (GG, UU), UG), 2.6),
    ((AU, (GU, AA), UG), 3.3), ((AU, (GU, AC), UG), 2.9), ((AU, (GU, AG), UG), 2.1), ((AU, (GU, AU), UG), 2.9), ((AU, (GU, CA), UG), 2.9), ((AU, (GU, CC), UG), 2.9), ((AU, (GU, CG), UG), 2.9), ((AU, (GU, CU), UG), 2.9), ((AU, (GU, GA), UG), 2.5), ((AU, (GU, GC), UG), 2.9), ((AU, (GU, GG), UG), 1.6), ((AU, (GU, GU), UG), 2.9), ((AU, (GU, UA), UG), 2.9), ((AU, (GU, UC), UG), 2.9), ((AU, (GU, UG), UG), 2.9), ((AU, (GU, UU), UG), 2.9),
    ((AU, (UA, AA), UG), 3.2), ((AU, (UA, AC), UG), 2.8), ((AU, (UA, AG), UG), 2.0), ((AU, (UA, AU), UG), 2.8), ((AU, (UA, CA), UG), 2.8), ((AU, (UA, CC), UG), 2.8), ((AU, (UA, CG), UG), 2.8), ((AU, (UA, CU), UG), 2.8), ((AU, (UA, GA), UG), 2.4), ((AU, (UA, GC), UG), 2.8), ((AU, (UA, GG), UG), 1.5), ((AU, (UA, GU), UG), 2.8), ((AU, (UA, UA), UG), 2.8), ((AU, (UA, UC), UG), 2.8), ((AU, (UA, UG), UG), 2.8), ((AU, (UA, UU), UG), 2.8),
    ((AU, (UC, AA), UG), 3.2), ((AU, (UC, AC), UG), 2.9), ((AU, (UC, AG), UG), 2.7), ((AU, (UC, AU), UG), 2.9), ((AU, (UC, CA), UG), 2.9), ((AU, (UC, CC), UG), 2.9), ((AU, (UC, CG), UG), 2.9), ((AU, (UC, CU), UG), 2.9), ((AU, (UC, GA), UG), 3.1), ((AU, (UC, GC), UG), 2.9), ((AU, (UC, GG), UG), 1.6), ((AU, (UC, GU), UG), 2.9), ((AU, (UC, UA), UG), 2.9), ((AU, (UC, UC), UG), 2.9), ((AU, (UC, UG), UG), 2.9), ((AU, (UC, UU), UG), 2.9),
    ((AU, (UG, AA), UG), 3.2), ((AU, (UG, AC), UG), 2.8), ((AU, (UG, AG), UG), 2.0), ((AU, (UG, AU), UG), 2.8), ((AU, (UG, CA), UG), 2.8), ((AU, (UG, CC), UG), 2.8), ((AU, (UG, CG), UG), 2.8), ((AU, (UG, CU), UG), 2.8), ((AU, (UG, GA), UG), 2.4), ((AU, (UG, GC), UG), 2.8), ((AU, (UG, GG), UG), 1.5), ((AU, (UG, GU), UG), 2.8), ((AU, (UG, UA), UG), 2.8), ((AU, (UG, UC), UG), 2.8), ((AU, (UG, UG), UG), 2.8), ((AU, (UG, UU), UG), 2.8),
    ((AU, (UU, AA), UG), 2.9), ((AU, (UU, AC), UG), 2.0), ((AU, (UU, AG), UG), 1.2), ((AU, (UU, AU), UG), 2.0), ((AU, (UU, CA), UG), 2.0), ((AU, (UU, CC), UG), 2.0), ((AU, (UU, CG), UG), 2.0), ((AU, (UU, CU), UG), 2.0), ((AU, (UU, GA), UG), 1.6), ((AU, (UU, GC), UG), 2.0), ((AU, (UU, GG), UG), 2.0), ((AU, (UU, GU), UG), 2.0), ((AU, (UU, UA), UG), 2.0), ((AU, (UU, UC), UG), 2.0), ((AU, (UU, UG), UG), 2.0), ((AU, (UU, UU), UG), 2.0),
    // For internal loops behind the base pair "CG".
    // For internal loops between the base pairs "CG" and "AU".
    ((CG, (AA, AA), AU), 2.0), ((CG, (AA, AC), AU), 1.5), ((CG, (AA, AG), AU), 0.9), ((CG, (AA, AU), AU), 1.5), ((CG, (AA, CA), AU), 2.0), ((CG, (AA, CC), AU), 2.0), ((CG, (AA, CG), AU), 2.0), ((CG, (AA, CU), AU), 2.0), ((CG, (AA, GA), AU), 1.0), ((CG, (AA, GC), AU), 1.5), ((CG, (AA, GG), AU), 0.4), ((CG, (AA, GU), AU), 1.5), ((CG, (AA, UA), AU), 2.0), ((CG, (AA, UC), AU), 1.7), ((CG, (AA, UG), AU), 2.0), ((CG, (AA, UU), AU), 1.7),
    ((CG, (AC, AA), AU), 2.4), ((CG, (AC, AC), AU), 1.9), ((CG, (AC, AG), AU), 1.3), ((CG, (AC, AU), AU), 1.9), ((CG, (AC, CA), AU), 2.4), ((CG, (AC, CC), AU), 2.4), ((CG, (AC, CG), AU), 2.4), ((CG, (AC, CU), AU), 2.4), ((CG, (AC, GA), AU), 1.4), ((CG, (AC, GC), AU), 1.9), ((CG, (AC, GG), AU), 0.8), ((CG, (AC, GU), AU), 1.9), ((CG, (AC, UA), AU), 2.4), ((CG, (AC, UC), AU), 2.1), ((CG, (AC, UG), AU), 2.4), ((CG, (AC, UU), AU), 1.5),
    ((CG, (AG, AA), AU), 1.0), ((CG, (AG, AC), AU), 0.6), ((CG, (AG, AG), AU), 0.0), ((CG, (AG, AU), AU), 0.6), ((CG, (AG, CA), AU), 1.0), ((CG, (AG, CC), AU), 1.6), ((CG, (AG, CG), AU), 1.0), ((CG, (AG, CU), AU), 1.6), ((CG, (AG, GA), AU), 0.1), ((CG, (AG, GC), AU), 0.6), ((CG, (AG, GG), AU), 0.8), ((CG, (AG, GU), AU), 0.6), ((CG, (AG, UA), AU), 1.0), ((CG, (AG, UC), AU), 1.3), ((CG, (AG, UG), AU), 1.0), ((CG, (AG, UU), AU), 0.2),
    ((CG, (AU, AA), AU), 2.4), ((CG, (AU, AC), AU), 1.9), ((CG, (AU, AG), AU), 1.3), ((CG, (AU, AU), AU), 1.9), ((CG, (AU, CA), AU), 2.4), ((CG, (AU, CC), AU), 2.4), ((CG, (AU, CG), AU), 2.4), ((CG, (AU, CU), AU), 2.4), ((CG, (AU, GA), AU), 1.4), ((CG, (AU, GC), AU), 1.9), ((CG, (AU, GG), AU), 0.8), ((CG, (AU, GU), AU), 1.9), ((CG, (AU, UA), AU), 2.4), ((CG, (AU, UC), AU), 2.1), ((CG, (AU, UG), AU), 2.4), ((CG, (AU, UU), AU), 1.5),
    ((CG, (CA, AA), AU), 1.9), ((CG, (CA, AC), AU), 1.5), ((CG, (CA, AG), AU), 0.9), ((CG, (CA, AU), AU), 1.5), ((CG, (CA, CA), AU), 1.9), ((CG, (CA, CC), AU), 1.9), ((CG, (CA, CG), AU), 1.9), ((CG, (CA, CU), AU), 1.9), ((CG, (CA, GA), AU), 1.0), ((CG, (CA, GC), AU), 1.5), ((CG, (CA, GG), AU), 0.4), ((CG, (CA, GU), AU), 1.5), ((CG, (CA, UA), AU), 1.9), ((CG, (CA, UC), AU), 1.6), ((CG, (CA, UG), AU), 1.9), ((CG, (CA, UU), AU), 1.1),
    ((CG, (CC, AA), AU), 2.2), ((CG, (CC, AC), AU), 1.8), ((CG, (CC, AG), AU), 1.8), ((CG, (CC, AU), AU), 1.8), ((CG, (CC, CA), AU), 2.2), ((CG, (CC, CC), AU), 2.2), ((CG, (CC, CG), AU), 2.2), ((CG, (CC, CU), AU), 2.2), ((CG, (CC, GA), AU), 1.9), ((CG, (CC, GC), AU), 1.8), ((CG, (CC, GG), AU), 0.7), ((CG, (CC, GU), AU), 1.8), ((CG, (CC, UA), AU), 2.2), ((CG, (CC, UC), AU), 1.9), ((CG, (CC, UG), AU), 2.2), ((CG, (CC, UU), AU), 1.4),
    ((CG, (CG, AA), AU), 1.9), ((CG, (CG, AC), AU), 1.5), ((CG, (CG, AG), AU), 0.9), ((CG, (CG, AU), AU), 1.5), ((CG, (CG, CA), AU), 1.9), ((CG, (CG, CC), AU), 1.9), ((CG, (CG, CG), AU), 1.9), ((CG, (CG, CU), AU), 1.9), ((CG, (CG, GA), AU), 1.0), ((CG, (CG, GC), AU), 1.5), ((CG, (CG, GG), AU), 0.4), ((CG, (CG, GU), AU), 1.5), ((CG, (CG, UA), AU), 1.9), ((CG, (CG, UC), AU), 1.6), ((CG, (CG, UG), AU), 1.9), ((CG, (CG, UU), AU), 1.1),
    ((CG, (CU, AA), AU), 2.1), ((CG, (CU, AC), AU), 1.6), ((CG, (CU, AG), AU), 1.6), ((CG, (CU, AU), AU), 1.6), ((CG, (CU, CA), AU), 2.1), ((CG, (CU, CC), AU), 2.1), ((CG, (CU, CG), AU), 2.1), ((CG, (CU, CU), AU), 2.1), ((CG, (CU, GA), AU), 1.7), ((CG, (CU, GC), AU), 1.6), ((CG, (CU, GG), AU), 0.5), ((CG, (CU, GU), AU), 1.6), ((CG, (CU, UA), AU), 2.1), ((CG, (CU, UC), AU), 1.8), ((CG, (CU, UG), AU), 2.1), ((CG, (CU, UU), AU), 1.2),
    ((CG, (GA, AA), AU), 1.0), ((CG, (GA, AC), AU), 0.6), ((CG, (GA, AG), AU), 0.0), ((CG, (GA, AU), AU), 0.6), ((CG, (GA, CA), AU), 1.0), ((CG, (GA, CC), AU), 1.6), ((CG, (GA, CG), AU), 1.0), ((CG, (GA, CU), AU), 1.6), ((CG, (GA, GA), AU), 0.1), ((CG, (GA, GC), AU), 0.6), ((CG, (GA, GG), AU), 0.8), ((CG, (GA, GU), AU), 0.6), ((CG, (GA, UA), AU), 1.0), ((CG, (GA, UC), AU), 1.3), ((CG, (GA, UG), AU), 1.0), ((CG, (GA, UU), AU), 0.2),
    ((CG, (GC, AA), AU), 2.4), ((CG, (GC, AC), AU), 1.9), ((CG, (GC, AG), AU), 1.3), ((CG, (GC, AU), AU), 1.9), ((CG, (GC, CA), AU), 2.4), ((CG, (GC, CC), AU), 2.4), ((CG, (GC, CG), AU), 2.4), ((CG, (GC, CU), AU), 2.4), ((CG, (GC, GA), AU), 1.4), ((CG, (GC, GC), AU), 1.9), ((CG, (GC, GG), AU), 0.8), ((CG, (GC, GU), AU), 1.9), ((CG, (GC, UA), AU), 2.4), ((CG, (GC, UC), AU), 2.1), ((CG, (GC, UG), AU), 2.4), ((CG, (GC, UU), AU), 1.5),
    ((CG, (GG, AA), AU), 0.5), ((CG, (GG, AC), AU), 0.0), ((CG, (GG, AG), AU), 0.7), ((CG, (GG, AU), AU), 0.0), ((CG, (GG, CA), AU), 0.5), ((CG, (GG, CC), AU), 0.5), ((CG, (GG, CG), AU), 0.5), ((CG, (GG, CU), AU), 0.5), ((CG, (GG, GA), AU), 0.8), ((CG, (GG, GC), AU), 0.0), ((CG, (GG, GG), AU), 1.5), ((CG, (GG, GU), AU), 0.0), ((CG, (GG, UA), AU), 0.5), ((CG, (GG, UC), AU), 0.2), ((CG, (GG, UG), AU), 0.5), ((CG, (GG, UU), AU), 0.9),
    ((CG, (GU, AA), AU), 2.4), ((CG, (GU, AC), AU), 1.9), ((CG, (GU, AG), AU), 1.3), ((CG, (GU, AU), AU), 1.9), ((CG, (GU, CA), AU), 2.4), ((CG, (GU, CC), AU), 2.4), ((CG, (GU, CG), AU), 2.4), ((CG, (GU, CU), AU), 2.4), ((CG, (GU, GA), AU), 1.4), ((CG, (GU, GC), AU), 1.9), ((CG, (GU, GG), AU), 0.8), ((CG, (GU, GU), AU), 1.9), ((CG, (GU, UA), AU), 2.4), ((CG, (GU, UC), AU), 2.1), ((CG, (GU, UG), AU), 2.4), ((CG, (GU, UU), AU), 1.5),
    ((CG, (UA, AA), AU), 1.9), ((CG, (UA, AC), AU), 1.5), ((CG, (UA, AG), AU), 0.9), ((CG, (UA, AU), AU), 1.5), ((CG, (UA, CA), AU), 1.9), ((CG, (UA, CC), AU), 1.9), ((CG, (UA, CG), AU), 1.9), ((CG, (UA, CU), AU), 1.9), ((CG, (UA, GA), AU), 1.0), ((CG, (UA, GC), AU), 1.5), ((CG, (UA, GG), AU), 0.4), ((CG, (UA, GU), AU), 1.5), ((CG, (UA, UA), AU), 1.9), ((CG, (UA, UC), AU), 1.6), ((CG, (UA, UG), AU), 1.9), ((CG, (UA, UU), AU), 1.1),
    ((CG, (UC, AA), AU), 2.1), ((CG, (UC, AC), AU), 1.6), ((CG, (UC, AG), AU), 1.6), ((CG, (UC, AU), AU), 1.6), ((CG, (UC, CA), AU), 2.1), ((CG, (UC, CC), AU), 2.1), ((CG, (UC, CG), AU), 2.1), ((CG, (UC, CU), AU), 2.1), ((CG, (UC, GA), AU), 1.7), ((CG, (UC, GC), AU), 1.6), ((CG, (UC, GG), AU), 0.5), ((CG, (UC, GU), AU), 1.6), ((CG, (UC, UA), AU), 2.1), ((CG, (UC, UC), AU), 1.8), ((CG, (UC, UG), AU), 2.1), ((CG, (UC, UU), AU), 1.2),
    ((CG, (UG, AA), AU), 1.9), ((CG, (UG, AC), AU), 1.5), ((CG, (UG, AG), AU), 0.9), ((CG, (UG, AU), AU), 1.5), ((CG, (UG, CA), AU), 1.9), ((CG, (UG, CC), AU), 1.9), ((CG, (UG, CG), AU), 1.9), ((CG, (UG, CU), AU), 1.9), ((CG, (UG, GA), AU), 1.0), ((CG, (UG, GC), AU), 1.5), ((CG, (UG, GG), AU), 0.4), ((CG, (UG, GU), AU), 1.5), ((CG, (UG, UA), AU), 1.9), ((CG, (UG, UC), AU), 1.6), ((CG, (UG, UG), AU), 1.9), ((CG, (UG, UU), AU), 1.1),
    ((CG, (UU, AA), AU), 1.8), ((CG, (UU, AC), AU), 0.7), ((CG, (UU, AG), AU), 0.1), ((CG, (UU, AU), AU), 0.7), ((CG, (UU, CA), AU), 1.2), ((CG, (UU, CC), AU), 1.2), ((CG, (UU, CG), AU), 1.2), ((CG, (UU, CU), AU), 1.2), ((CG, (UU, GA), AU), 0.2), ((CG, (UU, GC), AU), 0.7), ((CG, (UU, GG), AU), 0.9), ((CG, (UU, GU), AU), 0.7), ((CG, (UU, UA), AU), 1.2), ((CG, (UU, UC), AU), 0.9), ((CG, (UU, UG), AU), 1.2), ((CG, (UU, UU), AU), 0.3),
    // For internal loops between the base pairs "CG" and "CG".
    ((CG, (AA, AA), CG), 1.3), ((CG, (AA, AC), CG), 1.1), ((CG, (AA, AG), CG), -0.3), ((CG, (AA, AU), CG), 1.1), ((CG, (AA, CA), CG), 1.0), ((CG, (AA, CC), CG), 1.1), ((CG, (AA, CG), CG), 1.0), ((CG, (AA, CU), CG), 1.1), ((CG, (AA, GA), CG), 0.4), ((CG, (AA, GC), CG), 1.1), ((CG, (AA, GG), CG), -0.3), ((CG, (AA, GU), CG), 1.1), ((CG, (AA, UA), CG), 1.0), ((CG, (AA, UC), CG), 1.1), ((CG, (AA, UG), CG), 1.0), ((CG, (AA, UU), CG), 1.5),
    ((CG, (AC, AA), CG), 0.6), ((CG, (AC, AC), CG), 1.5), ((CG, (AC, AG), CG), 0.1), ((CG, (AC, AU), CG), 1.5), ((CG, (AC, CA), CG), 0.5), ((CG, (AC, CC), CG), 1.5), ((CG, (AC, CG), CG), 1.4), ((CG, (AC, CU), CG), 1.5), ((CG, (AC, GA), CG), 0.3), ((CG, (AC, GC), CG), 1.5), ((CG, (AC, GG), CG), -0.3), ((CG, (AC, GU), CG), 1.5), ((CG, (AC, UA), CG), 1.4), ((CG, (AC, UC), CG), 1.5), ((CG, (AC, UG), CG), 1.4), ((CG, (AC, UU), CG), 0.0),
    ((CG, (AG, AA), CG), 0.0), ((CG, (AG, AC), CG), -0.7), ((CG, (AG, AG), CG), -1.6), ((CG, (AG, AU), CG), 0.1), ((CG, (AG, CA), CG), -1.0), ((CG, (AG, CC), CG), -0.6), ((CG, (AG, CG), CG), 0.1), ((CG, (AG, CU), CG), 0.7), ((CG, (AG, GA), CG), -0.7), ((CG, (AG, GC), CG), 0.1), ((CG, (AG, GG), CG), 0.0), ((CG, (AG, GU), CG), 0.1), ((CG, (AG, UA), CG), 0.1), ((CG, (AG, UC), CG), 0.8), ((CG, (AG, UG), CG), 0.1), ((CG, (AG, UU), CG), 0.9),
    ((CG, (AU, AA), CG), 1.7), ((CG, (AU, AC), CG), 1.5), ((CG, (AU, AG), CG), -0.3), ((CG, (AU, AU), CG), 1.5), ((CG, (AU, CA), CG), 1.4), ((CG, (AU, CC), CG), 1.5), ((CG, (AU, CG), CG), 1.4), ((CG, (AU, CU), CG), 1.5), ((CG, (AU, GA), CG), 0.3), ((CG, (AU, GC), CG), 1.5), ((CG, (AU, GG), CG), 0.1), ((CG, (AU, GU), CG), 1.5), ((CG, (AU, UA), CG), 1.4), ((CG, (AU, UC), CG), 1.5), ((CG, (AU, UG), CG), 1.4), ((CG, (AU, UU), CG), 0.7),
    ((CG, (CA, AA), CG), 1.3), ((CG, (CA, AC), CG), 1.0), ((CG, (CA, AG), CG), -0.7), ((CG, (CA, AU), CG), 1.0), ((CG, (CA, CA), CG), 1.1), ((CG, (CA, CC), CG), 1.0), ((CG, (CA, CG), CG), 1.0), ((CG, (CA, CU), CG), 1.0), ((CG, (CA, GA), CG), 0.7), ((CG, (CA, GC), CG), 1.0), ((CG, (CA, GG), CG), -0.4), ((CG, (CA, GU), CG), 1.0), ((CG, (CA, UA), CG), 1.0), ((CG, (CA, UC), CG), 1.1), ((CG, (CA, UG), CG), 1.0), ((CG, (CA, UU), CG), -0.2),
    ((CG, (CC, AA), CG), 2.2), ((CG, (CC, AC), CG), 1.3), ((CG, (CC, AG), CG), 0.7), ((CG, (CC, AU), CG), 1.3), ((CG, (CC, CA), CG), 1.9), ((CG, (CC, CC), CG), 1.3), ((CG, (CC, CG), CG), 1.3), ((CG, (CC, CU), CG), 1.3), ((CG, (CC, GA), CG), 0.7), ((CG, (CC, GC), CG), 1.3), ((CG, (CC, GG), CG), -0.1), ((CG, (CC, GU), CG), 1.3), ((CG, (CC, UA), CG), 1.3), ((CG, (CC, UC), CG), 1.4), ((CG, (CC, UG), CG), 1.3), ((CG, (CC, UU), CG), -0.1),
    ((CG, (CG, AA), CG), 1.3), ((CG, (CG, AC), CG), 1.0), ((CG, (CG, AG), CG), -0.7), ((CG, (CG, AU), CG), 1.0), ((CG, (CG, CA), CG), 1.0), ((CG, (CG, CC), CG), 1.0), ((CG, (CG, CG), CG), 1.0), ((CG, (CG, CU), CG), 1.0), ((CG, (CG, GA), CG), -0.1), ((CG, (CG, GC), CG), 1.0), ((CG, (CG, GG), CG), -0.4), ((CG, (CG, GU), CG), 1.0), ((CG, (CG, UA), CG), 1.0), ((CG, (CG, UC), CG), 1.1), ((CG, (CG, UG), CG), 1.0), ((CG, (CG, UU), CG), 0.3),
    ((CG, (CU, AA), CG), 1.4), ((CG, (CU, AC), CG), 1.2), ((CG, (CU, AG), CG), 0.0), ((CG, (CU, AU), CG), 1.2), ((CG, (CU, CA), CG), 1.1), ((CG, (CU, CC), CG), 1.2), ((CG, (CU, CG), CG), 1.1), ((CG, (CU, CU), CG), 1.7), ((CG, (CU, GA), CG), 0.6), ((CG, (CU, GC), CG), 1.2), ((CG, (CU, GG), CG), 0.2), ((CG, (CU, GU), CG), 1.2), ((CG, (CU, UA), CG), 1.1), ((CG, (CU, UC), CG), 1.2), ((CG, (CU, UG), CG), 1.1), ((CG, (CU, UU), CG), 0.2),
    ((CG, (GA, AA), CG), -0.2), ((CG, (GA, AC), CG), -0.4), ((CG, (GA, AG), CG), -1.7), ((CG, (GA, AU), CG), 0.1), ((CG, (GA, CA), CG), 0.7), ((CG, (GA, CC), CG), 0.7), ((CG, (GA, CG), CG), 0.1), ((CG, (GA, CU), CG), 0.7), ((CG, (GA, GA), CG), -0.5), ((CG, (GA, GC), CG), 0.1), ((CG, (GA, GG), CG), -0.3), ((CG, (GA, GU), CG), 0.1), ((CG, (GA, UA), CG), 0.1), ((CG, (GA, UC), CG), 0.8), ((CG, (GA, UG), CG), 0.1), ((CG, (GA, UU), CG), 0.9),
    ((CG, (GC, AA), CG), 1.7), ((CG, (GC, AC), CG), 1.5), ((CG, (GC, AG), CG), -0.3), ((CG, (GC, AU), CG), 1.5), ((CG, (GC, CA), CG), 1.4), ((CG, (GC, CC), CG), 1.5), ((CG, (GC, CG), CG), 1.4), ((CG, (GC, CU), CG), 1.5), ((CG, (GC, GA), CG), 0.3), ((CG, (GC, GC), CG), 1.5), ((CG, (GC, GG), CG), 0.1), ((CG, (GC, GU), CG), 1.5), ((CG, (GC, UA), CG), 1.4), ((CG, (GC, UC), CG), 1.5), ((CG, (GC, UG), CG), 1.4), ((CG, (GC, UU), CG), 0.7),
    ((CG, (GG, AA), CG), -0.1), ((CG, (GG, AC), CG), -0.4), ((CG, (GG, AG), CG), -0.9), ((CG, (GG, AU), CG), -0.4), ((CG, (GG, CA), CG), -0.5), ((CG, (GG, CC), CG), -0.4), ((CG, (GG, CG), CG), -0.5), ((CG, (GG, CU), CG), 0.2), ((CG, (GG, GA), CG), -0.3), ((CG, (GG, GC), CG), -0.4), ((CG, (GG, GG), CG), 0.8), ((CG, (GG, GU), CG), -0.4), ((CG, (GG, UA), CG), -0.5), ((CG, (GG, UC), CG), -0.5), ((CG, (GG, UG), CG), -0.5), ((CG, (GG, UU), CG), 1.4),
    ((CG, (GU, AA), CG), 1.7), ((CG, (GU, AC), CG), 1.5), ((CG, (GU, AG), CG), -0.3), ((CG, (GU, AU), CG), 1.5), ((CG, (GU, CA), CG), 1.4), ((CG, (GU, CC), CG), 1.5), ((CG, (GU, CG), CG), 1.4), ((CG, (GU, CU), CG), 1.5), ((CG, (GU, GA), CG), 0.3), ((CG, (GU, GC), CG), 1.5), ((CG, (GU, GG), CG), 0.1), ((CG, (GU, GU), CG), 1.5), ((CG, (GU, UA), CG), 1.4), ((CG, (GU, UC), CG), 1.5), ((CG, (GU, UG), CG), 1.4), ((CG, (GU, UU), CG), 0.7),
    ((CG, (UA, AA), CG), 1.3), ((CG, (UA, AC), CG), 1.0), ((CG, (UA, AG), CG), -0.7), ((CG, (UA, AU), CG), 1.0), ((CG, (UA, CA), CG), 1.0), ((CG, (UA, CC), CG), 1.0), ((CG, (UA, CG), CG), 1.0), ((CG, (UA, CU), CG), 1.0), ((CG, (UA, GA), CG), -0.1), ((CG, (UA, GC), CG), 1.0), ((CG, (UA, GG), CG), -0.4), ((CG, (UA, GU), CG), 1.0), ((CG, (UA, UA), CG), 1.0), ((CG, (UA, UC), CG), 1.1), ((CG, (UA, UG), CG), 1.0), ((CG, (UA, UU), CG), 0.3),
    ((CG, (UC, AA), CG), 1.4), ((CG, (UC, AC), CG), 1.2), ((CG, (UC, AG), CG), 0.0), ((CG, (UC, AU), CG), 1.2), ((CG, (UC, CA), CG), 1.1), ((CG, (UC, CC), CG), 1.2), ((CG, (UC, CG), CG), 1.1), ((CG, (UC, CU), CG), 1.2), ((CG, (UC, GA), CG), 0.5), ((CG, (UC, GC), CG), 1.2), ((CG, (UC, GG), CG), -0.6), ((CG, (UC, GU), CG), 1.2), ((CG, (UC, UA), CG), 1.1), ((CG, (UC, UC), CG), 1.2), ((CG, (UC, UG), CG), 1.1), ((CG, (UC, UU), CG), 0.4),
    ((CG, (UG, AA), CG), 1.3), ((CG, (UG, AC), CG), 1.0), ((CG, (UG, AG), CG), -0.7), ((CG, (UG, AU), CG), 1.0), ((CG, (UG, CA), CG), 1.0), ((CG, (UG, CC), CG), 1.0), ((CG, (UG, CG), CG), 1.0), ((CG, (UG, CU), CG), 1.0), ((CG, (UG, GA), CG), -0.1), ((CG, (UG, GC), CG), 1.0), ((CG, (UG, GG), CG), -0.4), ((CG, (UG, GU), CG), 1.0), ((CG, (UG, UA), CG), 1.0), ((CG, (UG, UC), CG), 1.1), ((CG, (UG, UG), CG), 1.0), ((CG, (UG, UU), CG), 0.3),
    ((CG, (UU, AA), CG), 1.4), ((CG, (UU, AC), CG), 0.3), ((CG, (UU, AG), CG), 0.5), ((CG, (UU, AU), CG), 0.3), ((CG, (UU, CA), CG), 0.3), ((CG, (UU, CC), CG), 0.3), ((CG, (UU, CG), CG), 0.2), ((CG, (UU, CU), CG), 0.3), ((CG, (UU, GA), CG), 1.4), ((CG, (UU, GC), CG), 0.3), ((CG, (UU, GG), CG), 0.7), ((CG, (UU, GU), CG), 0.3), ((CG, (UU, UA), CG), 0.2), ((CG, (UU, UC), CG), 0.3), ((CG, (UU, UG), CG), 0.2), ((CG, (UU, UU), CG), -0.6),
    // For internal loops between the base pairs "CG" and "GC".
    ((CG, (AA, AA), GC), 1.2), ((CG, (AA, AC), GC), 1.1), ((CG, (AA, AG), GC), 0.2), ((CG, (AA, AU), GC), 1.1), ((CG, (AA, CA), GC), 1.6), ((CG, (AA, CC), GC), 1.4), ((CG, (AA, CG), GC), 1.6), ((CG, (AA, CU), GC), 1.3), ((CG, (AA, GA), GC), 0.2), ((CG, (AA, GC), GC), 1.1), ((CG, (AA, GG), GC), -0.3), ((CG, (AA, GU), GC), 1.1), ((CG, (AA, UA), GC), 1.6), ((CG, (AA, UC), GC), 1.3), ((CG, (AA, UG), GC), 1.6), ((CG, (AA, UU), GC), 1.0),
    ((CG, (AC, AA), GC), 1.6), ((CG, (AC, AC), GC), 1.5), ((CG, (AC, AG), GC), 0.6), ((CG, (AC, AU), GC), 1.5), ((CG, (AC, CA), GC), 2.0), ((CG, (AC, CC), GC), 1.8), ((CG, (AC, CG), GC), 2.0), ((CG, (AC, CU), GC), 1.7), ((CG, (AC, GA), GC), 0.6), ((CG, (AC, GC), GC), 1.5), ((CG, (AC, GG), GC), 0.1), ((CG, (AC, GU), GC), 1.5), ((CG, (AC, UA), GC), 2.0), ((CG, (AC, UC), GC), 1.7), ((CG, (AC, UG), GC), 2.0), ((CG, (AC, UU), GC), 0.8),
    ((CG, (AG, AA), GC), 0.2), ((CG, (AG, AC), GC), 0.2), ((CG, (AG, AG), GC), -0.7), ((CG, (AG, AU), GC), 0.2), ((CG, (AG, CA), GC), 0.6), ((CG, (AG, CC), GC), 1.1), ((CG, (AG, CG), GC), 0.6), ((CG, (AG, CU), GC), 0.9), ((CG, (AG, GA), GC), -0.7), ((CG, (AG, GC), GC), 0.2), ((CG, (AG, GG), GC), 0.0), ((CG, (AG, GU), GC), 0.2), ((CG, (AG, UA), GC), 0.6), ((CG, (AG, UC), GC), 0.9), ((CG, (AG, UG), GC), 0.6), ((CG, (AG, UU), GC), -0.5),
    ((CG, (AU, AA), GC), 1.6), ((CG, (AU, AC), GC), 1.5), ((CG, (AU, AG), GC), 0.6), ((CG, (AU, AU), GC), 1.5), ((CG, (AU, CA), GC), 2.0), ((CG, (AU, CC), GC), 1.8), ((CG, (AU, CG), GC), 2.0), ((CG, (AU, CU), GC), 1.7), ((CG, (AU, GA), GC), 0.6), ((CG, (AU, GC), GC), 1.5), ((CG, (AU, GG), GC), 0.1), ((CG, (AU, GU), GC), 1.5), ((CG, (AU, UA), GC), 2.0), ((CG, (AU, UC), GC), 1.7), ((CG, (AU, UG), GC), 2.0), ((CG, (AU, UU), GC), 0.8),
    ((CG, (CA, AA), GC), 1.1), ((CG, (CA, AC), GC), 1.1), ((CG, (CA, AG), GC), 0.2), ((CG, (CA, AU), GC), 1.1), ((CG, (CA, CA), GC), 1.5), ((CG, (CA, CC), GC), 1.4), ((CG, (CA, CG), GC), 1.5), ((CG, (CA, CU), GC), 1.2), ((CG, (CA, GA), GC), 0.2), ((CG, (CA, GC), GC), 1.1), ((CG, (CA, GG), GC), -0.4), ((CG, (CA, GU), GC), 1.1), ((CG, (CA, UA), GC), 1.5), ((CG, (CA, UC), GC), 1.2), ((CG, (CA, UG), GC), 1.5), ((CG, (CA, UU), GC), 0.3),
    ((CG, (CC, AA), GC), 1.4), ((CG, (CC, AC), GC), 1.4), ((CG, (CC, AG), GC), 1.1), ((CG, (CC, AU), GC), 1.4), ((CG, (CC, CA), GC), 1.8), ((CG, (CC, CC), GC), 1.7), ((CG, (CC, CG), GC), 1.8), ((CG, (CC, CU), GC), 1.5), ((CG, (CC, GA), GC), 1.1), ((CG, (CC, GC), GC), 1.4), ((CG, (CC, GG), GC), -0.1), ((CG, (CC, GU), GC), 1.4), ((CG, (CC, UA), GC), 1.8), ((CG, (CC, UC), GC), 1.5), ((CG, (CC, UG), GC), 1.8), ((CG, (CC, UU), GC), 0.6),
    ((CG, (CG, AA), GC), 1.1), ((CG, (CG, AC), GC), 1.1), ((CG, (CG, AG), GC), 0.2), ((CG, (CG, AU), GC), 1.1), ((CG, (CG, CA), GC), 1.5), ((CG, (CG, CC), GC), 1.4), ((CG, (CG, CG), GC), 1.5), ((CG, (CG, CU), GC), 1.2), ((CG, (CG, GA), GC), 0.2), ((CG, (CG, GC), GC), 1.1), ((CG, (CG, GG), GC), -0.4), ((CG, (CG, GU), GC), 1.1), ((CG, (CG, UA), GC), 1.5), ((CG, (CG, UC), GC), 1.2), ((CG, (CG, UG), GC), 1.5), ((CG, (CG, UU), GC), 0.3),
    ((CG, (CU, AA), GC), 1.3), ((CG, (CU, AC), GC), 1.2), ((CG, (CU, AG), GC), 0.9), ((CG, (CU, AU), GC), 1.2), ((CG, (CU, CA), GC), 1.7), ((CG, (CU, CC), GC), 1.5), ((CG, (CU, CG), GC), 1.7), ((CG, (CU, CU), GC), 1.4), ((CG, (CU, GA), GC), 0.9), ((CG, (CU, GC), GC), 1.2), ((CG, (CU, GG), GC), -0.2), ((CG, (CU, GU), GC), 1.2), ((CG, (CU, UA), GC), 1.7), ((CG, (CU, UC), GC), 1.4), ((CG, (CU, UG), GC), 1.7), ((CG, (CU, UU), GC), 0.5),
    ((CG, (GA, AA), GC), 0.2), ((CG, (GA, AC), GC), 0.2), ((CG, (GA, AG), GC), -0.7), ((CG, (GA, AU), GC), 0.2), ((CG, (GA, CA), GC), 0.6), ((CG, (GA, CC), GC), 1.1), ((CG, (GA, CG), GC), 0.6), ((CG, (GA, CU), GC), 0.9), ((CG, (GA, GA), GC), -0.7), ((CG, (GA, GC), GC), 0.2), ((CG, (GA, GG), GC), 0.0), ((CG, (GA, GU), GC), 0.2), ((CG, (GA, UA), GC), 0.6), ((CG, (GA, UC), GC), 0.9), ((CG, (GA, UG), GC), 0.6), ((CG, (GA, UU), GC), -0.5),
    ((CG, (GC, AA), GC), 1.6), ((CG, (GC, AC), GC), 1.5), ((CG, (GC, AG), GC), 0.6), ((CG, (GC, AU), GC), 1.5), ((CG, (GC, CA), GC), 2.0), ((CG, (GC, CC), GC), 1.8), ((CG, (GC, CG), GC), 2.0), ((CG, (GC, CU), GC), 1.7), ((CG, (GC, GA), GC), 0.6), ((CG, (GC, GC), GC), 1.5), ((CG, (GC, GG), GC), 0.1), ((CG, (GC, GU), GC), 1.5), ((CG, (GC, UA), GC), 2.0), ((CG, (GC, UC), GC), 1.7), ((CG, (GC, UG), GC), 2.0), ((CG, (GC, UU), GC), 0.8),
    ((CG, (GG, AA), GC), -0.3), ((CG, (GG, AC), GC), -0.4), ((CG, (GG, AG), GC), 0.0), ((CG, (GG, AU), GC), -0.4), ((CG, (GG, CA), GC), 0.1), ((CG, (GG, CC), GC), -0.1), ((CG, (GG, CG), GC), 0.1), ((CG, (GG, CU), GC), -0.2), ((CG, (GG, GA), GC), 0.0), ((CG, (GG, GC), GC), -0.4), ((CG, (GG, GG), GC), 0.8), ((CG, (GG, GU), GC), -0.4), ((CG, (GG, UA), GC), 0.1), ((CG, (GG, UC), GC), -0.2), ((CG, (GG, UG), GC), 0.1), ((CG, (GG, UU), GC), 0.2),
    ((CG, (GU, AA), GC), 1.6), ((CG, (GU, AC), GC), 1.5), ((CG, (GU, AG), GC), 0.6), ((CG, (GU, AU), GC), 1.5), ((CG, (GU, CA), GC), 2.0), ((CG, (GU, CC), GC), 1.8), ((CG, (GU, CG), GC), 2.0), ((CG, (GU, CU), GC), 1.7), ((CG, (GU, GA), GC), 0.6), ((CG, (GU, GC), GC), 1.5), ((CG, (GU, GG), GC), 0.1), ((CG, (GU, GU), GC), 1.5), ((CG, (GU, UA), GC), 2.0), ((CG, (GU, UC), GC), 1.7), ((CG, (GU, UG), GC), 2.0), ((CG, (GU, UU), GC), 0.8),
    ((CG, (UA, AA), GC), 1.1), ((CG, (UA, AC), GC), 1.1), ((CG, (UA, AG), GC), 0.2), ((CG, (UA, AU), GC), 1.1), ((CG, (UA, CA), GC), 1.5), ((CG, (UA, CC), GC), 1.4), ((CG, (UA, CG), GC), 1.5), ((CG, (UA, CU), GC), 1.2), ((CG, (UA, GA), GC), 0.2), ((CG, (UA, GC), GC), 1.1), ((CG, (UA, GG), GC), -0.4), ((CG, (UA, GU), GC), 1.1), ((CG, (UA, UA), GC), 1.5), ((CG, (UA, UC), GC), 1.2), ((CG, (UA, UG), GC), 1.5), ((CG, (UA, UU), GC), 0.3),
    ((CG, (UC, AA), GC), 1.3), ((CG, (UC, AC), GC), 1.2), ((CG, (UC, AG), GC), 0.9), ((CG, (UC, AU), GC), 1.2), ((CG, (UC, CA), GC), 1.7), ((CG, (UC, CC), GC), 1.5), ((CG, (UC, CG), GC), 1.7), ((CG, (UC, CU), GC), 1.4), ((CG, (UC, GA), GC), 0.9), ((CG, (UC, GC), GC), 1.2), ((CG, (UC, GG), GC), -0.2), ((CG, (UC, GU), GC), 1.2), ((CG, (UC, UA), GC), 1.7), ((CG, (UC, UC), GC), 1.4), ((CG, (UC, UG), GC), 1.7), ((CG, (UC, UU), GC), 0.5),
    ((CG, (UG, AA), GC), 1.1), ((CG, (UG, AC), GC), 1.1), ((CG, (UG, AG), GC), 0.2), ((CG, (UG, AU), GC), 1.1), ((CG, (UG, CA), GC), 1.5), ((CG, (UG, CC), GC), 1.4), ((CG, (UG, CG), GC), 1.5), ((CG, (UG, CU), GC), 1.2), ((CG, (UG, GA), GC), 0.2), ((CG, (UG, GC), GC), 1.1), ((CG, (UG, GG), GC), -0.4), ((CG, (UG, GU), GC), 1.1), ((CG, (UG, UA), GC), 1.5), ((CG, (UG, UC), GC), 1.2), ((CG, (UG, UG), GC), 1.5), ((CG, (UG, UU), GC), 0.3),
    ((CG, (UU, AA), GC), 1.0), ((CG, (UU, AC), GC), 0.3), ((CG, (UU, AG), GC), -0.5), ((CG, (UU, AU), GC), 0.3), ((CG, (UU, CA), GC), 0.8), ((CG, (UU, CC), GC), 0.6), ((CG, (UU, CG), GC), 0.8), ((CG, (UU, CU), GC), 0.5), ((CG, (UU, GA), GC), -0.5), ((CG, (UU, GC), GC), 0.3), ((CG, (UU, GG), GC), 0.2), ((CG, (UU, GU), GC), 0.3), ((CG, (UU, UA), GC), 0.8), ((CG, (UU, UC), GC), 0.5), ((CG, (UU, UG), GC), 0.8), ((CG, (UU, UU), GC), -0.4),
    // For internal loops between the base pairs "CG" and "GU".
    ((CG, (AA, AA), GU), 1.6), ((CG, (AA, AC), GU), 2.0), ((CG, (AA, AG), GU), 0.6), ((CG, (AA, AU), GU), 2.0), ((CG, (AA, CA), GU), 2.0), ((CG, (AA, CC), GU), 2.0), ((CG, (AA, CG), GU), 2.0), ((CG, (AA, CU), GU), 2.0), ((CG, (AA, GA), GU), 2.3), ((CG, (AA, GC), GU), 2.0), ((CG, (AA, GG), GU), 0.7), ((CG, (AA, GU), GU), 2.0), ((CG, (AA, UA), GU), 2.0), ((CG, (AA, UC), GU), 2.0), ((CG, (AA, UG), GU), 2.0), ((CG, (AA, UU), GU), 2.6),
    ((CG, (AC, AA), GU), 2.0), ((CG, (AC, AC), GU), 2.4), ((CG, (AC, AG), GU), 1.0), ((CG, (AC, AU), GU), 2.4), ((CG, (AC, CA), GU), 2.4), ((CG, (AC, CC), GU), 2.4), ((CG, (AC, CG), GU), 2.4), ((CG, (AC, CU), GU), 2.4), ((CG, (AC, GA), GU), 2.7), ((CG, (AC, GC), GU), 2.4), ((CG, (AC, GG), GU), 1.1), ((CG, (AC, GU), GU), 2.4), ((CG, (AC, UA), GU), 2.4), ((CG, (AC, UC), GU), 2.4), ((CG, (AC, UG), GU), 2.4), ((CG, (AC, UU), GU), 2.4),
    ((CG, (AG, AA), GU), 0.7), ((CG, (AG, AC), GU), 1.0), ((CG, (AG, AG), GU), -0.3), ((CG, (AG, AU), GU), 1.0), ((CG, (AG, CA), GU), 1.0), ((CG, (AG, CC), GU), 1.6), ((CG, (AG, CG), GU), 1.0), ((CG, (AG, CU), GU), 1.6), ((CG, (AG, GA), GU), 1.3), ((CG, (AG, GC), GU), 1.0), ((CG, (AG, GG), GU), 1.0), ((CG, (AG, GU), GU), 1.0), ((CG, (AG, UA), GU), 1.0), ((CG, (AG, UC), GU), 1.6), ((CG, (AG, UG), GU), 1.0), ((CG, (AG, UU), GU), 1.0),
    ((CG, (AU, AA), GU), 2.0), ((CG, (AU, AC), GU), 2.4), ((CG, (AU, AG), GU), 1.0), ((CG, (AU, AU), GU), 2.4), ((CG, (AU, CA), GU), 2.4), ((CG, (AU, CC), GU), 2.4), ((CG, (AU, CG), GU), 2.4), ((CG, (AU, CU), GU), 2.4), ((CG, (AU, GA), GU), 2.7), ((CG, (AU, GC), GU), 2.4), ((CG, (AU, GG), GU), 1.1), ((CG, (AU, GU), GU), 2.4), ((CG, (AU, UA), GU), 2.4), ((CG, (AU, UC), GU), 2.4), ((CG, (AU, UG), GU), 2.4), ((CG, (AU, UU), GU), 2.4),
    ((CG, (CA, AA), GU), 1.6), ((CG, (CA, AC), GU), 1.9), ((CG, (CA, AG), GU), 0.6), ((CG, (CA, AU), GU), 1.9), ((CG, (CA, CA), GU), 1.9), ((CG, (CA, CC), GU), 1.9), ((CG, (CA, CG), GU), 1.9), ((CG, (CA, CU), GU), 1.9), ((CG, (CA, GA), GU), 2.2), ((CG, (CA, GC), GU), 1.9), ((CG, (CA, GG), GU), 0.6), ((CG, (CA, GU), GU), 1.9), ((CG, (CA, UA), GU), 1.9), ((CG, (CA, UC), GU), 1.9), ((CG, (CA, UG), GU), 1.9), ((CG, (CA, UU), GU), 1.9),
    ((CG, (CC, AA), GU), 1.9), ((CG, (CC, AC), GU), 2.2), ((CG, (CC, AG), GU), 1.5), ((CG, (CC, AU), GU), 2.2), ((CG, (CC, CA), GU), 2.2), ((CG, (CC, CC), GU), 2.2), ((CG, (CC, CG), GU), 2.2), ((CG, (CC, CU), GU), 2.2), ((CG, (CC, GA), GU), 3.1), ((CG, (CC, GC), GU), 2.2), ((CG, (CC, GG), GU), 0.9), ((CG, (CC, GU), GU), 2.2), ((CG, (CC, UA), GU), 2.2), ((CG, (CC, UC), GU), 2.2), ((CG, (CC, UG), GU), 2.2), ((CG, (CC, UU), GU), 2.2),
    ((CG, (CG, AA), GU), 1.6), ((CG, (CG, AC), GU), 1.9), ((CG, (CG, AG), GU), 0.6), ((CG, (CG, AU), GU), 1.9), ((CG, (CG, CA), GU), 1.9), ((CG, (CG, CC), GU), 1.9), ((CG, (CG, CG), GU), 1.9), ((CG, (CG, CU), GU), 1.9), ((CG, (CG, GA), GU), 2.2), ((CG, (CG, GC), GU), 1.9), ((CG, (CG, GG), GU), 0.6), ((CG, (CG, GU), GU), 1.9), ((CG, (CG, UA), GU), 1.9), ((CG, (CG, UC), GU), 1.9), ((CG, (CG, UG), GU), 1.9), ((CG, (CG, UU), GU), 1.9),
    ((CG, (CU, AA), GU), 1.7), ((CG, (CU, AC), GU), 2.1), ((CG, (CU, AG), GU), 1.3), ((CG, (CU, AU), GU), 2.1), ((CG, (CU, CA), GU), 2.1), ((CG, (CU, CC), GU), 2.1), ((CG, (CU, CG), GU), 2.1), ((CG, (CU, CU), GU), 2.1), ((CG, (CU, GA), GU), 3.0), ((CG, (CU, GC), GU), 2.1), ((CG, (CU, GG), GU), 0.8), ((CG, (CU, GU), GU), 2.1), ((CG, (CU, UA), GU), 2.1), ((CG, (CU, UC), GU), 2.1), ((CG, (CU, UG), GU), 2.1), ((CG, (CU, UU), GU), 2.1),
    ((CG, (GA, AA), GU), 0.7), ((CG, (GA, AC), GU), 1.0), ((CG, (GA, AG), GU), -0.3), ((CG, (GA, AU), GU), 1.0), ((CG, (GA, CA), GU), 1.0), ((CG, (GA, CC), GU), 1.6), ((CG, (GA, CG), GU), 1.0), ((CG, (GA, CU), GU), 1.6), ((CG, (GA, GA), GU), 1.3), ((CG, (GA, GC), GU), 1.0), ((CG, (GA, GG), GU), 1.0), ((CG, (GA, GU), GU), 1.0), ((CG, (GA, UA), GU), 1.0), ((CG, (GA, UC), GU), 1.6), ((CG, (GA, UG), GU), 1.0), ((CG, (GA, UU), GU), 1.0),
    ((CG, (GC, AA), GU), 2.0), ((CG, (GC, AC), GU), 2.4), ((CG, (GC, AG), GU), 1.0), ((CG, (GC, AU), GU), 2.4), ((CG, (GC, CA), GU), 2.4), ((CG, (GC, CC), GU), 2.4), ((CG, (GC, CG), GU), 2.4), ((CG, (GC, CU), GU), 2.4), ((CG, (GC, GA), GU), 2.7), ((CG, (GC, GC), GU), 2.4), ((CG, (GC, GG), GU), 1.1), ((CG, (GC, GU), GU), 2.4), ((CG, (GC, UA), GU), 2.4), ((CG, (GC, UC), GU), 2.4), ((CG, (GC, UG), GU), 2.4), ((CG, (GC, UU), GU), 2.4),
    ((CG, (GG, AA), GU), 0.1), ((CG, (GG, AC), GU), 0.5), ((CG, (GG, AG), GU), 0.4), ((CG, (GG, AU), GU), 0.5), ((CG, (GG, CA), GU), 0.5), ((CG, (GG, CC), GU), 0.5), ((CG, (GG, CG), GU), 0.5), ((CG, (GG, CU), GU), 0.5), ((CG, (GG, GA), GU), 2.1), ((CG, (GG, GC), GU), 0.5), ((CG, (GG, GG), GU), 1.8), ((CG, (GG, GU), GU), 0.5), ((CG, (GG, UA), GU), 0.5), ((CG, (GG, UC), GU), 0.5), ((CG, (GG, UG), GU), 0.5), ((CG, (GG, UU), GU), 1.8),
    ((CG, (GU, AA), GU), 2.0), ((CG, (GU, AC), GU), 2.4), ((CG, (GU, AG), GU), 1.0), ((CG, (GU, AU), GU), 2.4), ((CG, (GU, CA), GU), 2.4), ((CG, (GU, CC), GU), 2.4), ((CG, (GU, CG), GU), 2.4), ((CG, (GU, CU), GU), 2.4), ((CG, (GU, GA), GU), 2.7), ((CG, (GU, GC), GU), 2.4), ((CG, (GU, GG), GU), 1.1), ((CG, (GU, GU), GU), 2.4), ((CG, (GU, UA), GU), 2.4), ((CG, (GU, UC), GU), 2.4), ((CG, (GU, UG), GU), 2.4), ((CG, (GU, UU), GU), 2.4),
    ((CG, (UA, AA), GU), 1.6), ((CG, (UA, AC), GU), 1.9), ((CG, (UA, AG), GU), 0.6), ((CG, (UA, AU), GU), 1.9), ((CG, (UA, CA), GU), 1.9), ((CG, (UA, CC), GU), 1.9), ((CG, (UA, CG), GU), 1.9), ((CG, (UA, CU), GU), 1.9), ((CG, (UA, GA), GU), 2.2), ((CG, (UA, GC), GU), 1.9), ((CG, (UA, GG), GU), 0.6), ((CG, (UA, GU), GU), 1.9), ((CG, (UA, UA), GU), 1.9), ((CG, (UA, UC), GU), 1.9), ((CG, (UA, UG), GU), 1.9), ((CG, (UA, UU), GU), 1.9),
    ((CG, (UC, AA), GU), 1.7), ((CG, (UC, AC), GU), 2.1), ((CG, (UC, AG), GU), 1.3), ((CG, (UC, AU), GU), 2.1), ((CG, (UC, CA), GU), 2.1), ((CG, (UC, CC), GU), 2.1), ((CG, (UC, CG), GU), 2.1), ((CG, (UC, CU), GU), 2.1), ((CG, (UC, GA), GU), 3.0), ((CG, (UC, GC), GU), 2.1), ((CG, (UC, GG), GU), 0.8), ((CG, (UC, GU), GU), 2.1), ((CG, (UC, UA), GU), 2.1), ((CG, (UC, UC), GU), 2.1), ((CG, (UC, UG), GU), 2.1), ((CG, (UC, UU), GU), 2.1),
    ((CG, (UG, AA), GU), 1.6), ((CG, (UG, AC), GU), 1.9), ((CG, (UG, AG), GU), 0.6), ((CG, (UG, AU), GU), 1.9), ((CG, (UG, CA), GU), 1.9), ((CG, (UG, CC), GU), 1.9), ((CG, (UG, CG), GU), 1.9), ((CG, (UG, CU), GU), 1.9), ((CG, (UG, GA), GU), 2.2), ((CG, (UG, GC), GU), 1.9), ((CG, (UG, GG), GU), 0.6), ((CG, (UG, GU), GU), 1.9), ((CG, (UG, UA), GU), 1.9), ((CG, (UG, UC), GU), 1.9), ((CG, (UG, UG), GU), 1.9), ((CG, (UG, UU), GU), 1.9),
    ((CG, (UU, AA), GU), 1.4), ((CG, (UU, AC), GU), 1.2), ((CG, (UU, AG), GU), -0.1), ((CG, (UU, AU), GU), 1.2), ((CG, (UU, CA), GU), 1.2), ((CG, (UU, CC), GU), 1.2), ((CG, (UU, CG), GU), 1.2), ((CG, (UU, CU), GU), 1.2), ((CG, (UU, GA), GU), 1.5), ((CG, (UU, GC), GU), 1.2), ((CG, (UU, GG), GU), 1.2), ((CG, (UU, GU), GU), 1.2), ((CG, (UU, UA), GU), 1.2), ((CG, (UU, UC), GU), 1.2), ((CG, (UU, UG), GU), 1.2), ((CG, (UU, UU), GU), 1.2),
    // For internal loops between the base pairs "CG" and "UA".
    ((CG, (AA, AA), UA), 2.0), ((CG, (AA, AC), UA), 1.7), ((CG, (AA, AG), UA), 0.7), ((CG, (AA, AU), UA), 1.7), ((CG, (AA, CA), UA), 1.8), ((CG, (AA, CC), UA), 1.8), ((CG, (AA, CG), UA), 1.8), ((CG, (AA, CU), UA), 1.8), ((CG, (AA, GA), UA), 1.4), ((CG, (AA, GC), UA), 1.7), ((CG, (AA, GG), UA), 0.2), ((CG, (AA, GU), UA), 1.7), ((CG, (AA, UA), UA), 1.8), ((CG, (AA, UC), UA), 1.8), ((CG, (AA, UG), UA), 1.8), ((CG, (AA, UU), UA), 1.5),
    ((CG, (AC, AA), UA), 2.4), ((CG, (AC, AC), UA), 2.1), ((CG, (AC, AG), UA), 1.1), ((CG, (AC, AU), UA), 2.1), ((CG, (AC, CA), UA), 2.2), ((CG, (AC, CC), UA), 2.2), ((CG, (AC, CG), UA), 2.2), ((CG, (AC, CU), UA), 2.2), ((CG, (AC, GA), UA), 1.8), ((CG, (AC, GC), UA), 2.1), ((CG, (AC, GG), UA), 0.6), ((CG, (AC, GU), UA), 2.1), ((CG, (AC, UA), UA), 2.2), ((CG, (AC, UC), UA), 2.2), ((CG, (AC, UG), UA), 2.2), ((CG, (AC, UU), UA), 1.3),
    ((CG, (AG, AA), UA), 1.0), ((CG, (AG, AC), UA), 0.8), ((CG, (AG, AG), UA), -0.2), ((CG, (AG, AU), UA), 0.8), ((CG, (AG, CA), UA), 0.9), ((CG, (AG, CC), UA), 1.4), ((CG, (AG, CG), UA), 0.9), ((CG, (AG, CU), UA), 1.4), ((CG, (AG, GA), UA), 0.5), ((CG, (AG, GC), UA), 0.8), ((CG, (AG, GG), UA), 0.6), ((CG, (AG, GU), UA), 0.8), ((CG, (AG, UA), UA), 0.9), ((CG, (AG, UC), UA), 1.4), ((CG, (AG, UG), UA), 0.9), ((CG, (AG, UU), UA), 0.0),
    ((CG, (AU, AA), UA), 2.4), ((CG, (AU, AC), UA), 2.1), ((CG, (AU, AG), UA), 1.1), ((CG, (AU, AU), UA), 2.1), ((CG, (AU, CA), UA), 2.2), ((CG, (AU, CC), UA), 2.2), ((CG, (AU, CG), UA), 2.2), ((CG, (AU, CU), UA), 2.2), ((CG, (AU, GA), UA), 1.8), ((CG, (AU, GC), UA), 2.1), ((CG, (AU, GG), UA), 0.6), ((CG, (AU, GU), UA), 2.1), ((CG, (AU, UA), UA), 2.2), ((CG, (AU, UC), UA), 2.2), ((CG, (AU, UG), UA), 2.2), ((CG, (AU, UU), UA), 1.3),
    ((CG, (CA, AA), UA), 1.9), ((CG, (CA, AC), UA), 1.7), ((CG, (CA, AG), UA), 0.7), ((CG, (CA, AU), UA), 1.7), ((CG, (CA, CA), UA), 1.8), ((CG, (CA, CC), UA), 1.7), ((CG, (CA, CG), UA), 1.8), ((CG, (CA, CU), UA), 1.7), ((CG, (CA, GA), UA), 1.4), ((CG, (CA, GC), UA), 1.7), ((CG, (CA, GG), UA), 0.2), ((CG, (CA, GU), UA), 1.7), ((CG, (CA, UA), UA), 1.8), ((CG, (CA, UC), UA), 1.7), ((CG, (CA, UG), UA), 1.8), ((CG, (CA, UU), UA), 0.8),
    ((CG, (CC, AA), UA), 2.2), ((CG, (CC, AC), UA), 2.0), ((CG, (CC, AG), UA), 1.6), ((CG, (CC, AU), UA), 2.0), ((CG, (CC, CA), UA), 2.1), ((CG, (CC, CC), UA), 2.0), ((CG, (CC, CG), UA), 2.1), ((CG, (CC, CU), UA), 2.0), ((CG, (CC, GA), UA), 2.3), ((CG, (CC, GC), UA), 2.0), ((CG, (CC, GG), UA), 0.5), ((CG, (CC, GU), UA), 2.0), ((CG, (CC, UA), UA), 2.1), ((CG, (CC, UC), UA), 2.0), ((CG, (CC, UG), UA), 2.1), ((CG, (CC, UU), UA), 1.1),
    ((CG, (CG, AA), UA), 1.9), ((CG, (CG, AC), UA), 1.7), ((CG, (CG, AG), UA), 0.7), ((CG, (CG, AU), UA), 1.7), ((CG, (CG, CA), UA), 1.8), ((CG, (CG, CC), UA), 1.7), ((CG, (CG, CG), UA), 1.8), ((CG, (CG, CU), UA), 1.7), ((CG, (CG, GA), UA), 1.4), ((CG, (CG, GC), UA), 1.7), ((CG, (CG, GG), UA), 0.2), ((CG, (CG, GU), UA), 1.7), ((CG, (CG, UA), UA), 1.8), ((CG, (CG, UC), UA), 1.7), ((CG, (CG, UG), UA), 1.8), ((CG, (CG, UU), UA), 0.8),
    ((CG, (CU, AA), UA), 2.1), ((CG, (CU, AC), UA), 1.8), ((CG, (CU, AG), UA), 1.4), ((CG, (CU, AU), UA), 1.8), ((CG, (CU, CA), UA), 1.9), ((CG, (CU, CC), UA), 1.9), ((CG, (CU, CG), UA), 1.9), ((CG, (CU, CU), UA), 1.9), ((CG, (CU, GA), UA), 2.1), ((CG, (CU, GC), UA), 1.8), ((CG, (CU, GG), UA), 0.3), ((CG, (CU, GU), UA), 1.8), ((CG, (CU, UA), UA), 1.9), ((CG, (CU, UC), UA), 1.9), ((CG, (CU, UG), UA), 1.9), ((CG, (CU, UU), UA), 1.0),
    ((CG, (GA, AA), UA), 1.0), ((CG, (GA, AC), UA), 0.8), ((CG, (GA, AG), UA), -0.2), ((CG, (GA, AU), UA), 0.8), ((CG, (GA, CA), UA), 0.9), ((CG, (GA, CC), UA), 1.4), ((CG, (GA, CG), UA), 0.9), ((CG, (GA, CU), UA), 1.4), ((CG, (GA, GA), UA), 0.5), ((CG, (GA, GC), UA), 0.8), ((CG, (GA, GG), UA), 0.6), ((CG, (GA, GU), UA), 0.8), ((CG, (GA, UA), UA), 0.9), ((CG, (GA, UC), UA), 1.4), ((CG, (GA, UG), UA), 0.9), ((CG, (GA, UU), UA), 0.0),
    ((CG, (GC, AA), UA), 2.4), ((CG, (GC, AC), UA), 2.1), ((CG, (GC, AG), UA), 1.1), ((CG, (GC, AU), UA), 2.1), ((CG, (GC, CA), UA), 2.2), ((CG, (GC, CC), UA), 2.2), ((CG, (GC, CG), UA), 2.2), ((CG, (GC, CU), UA), 2.2), ((CG, (GC, GA), UA), 1.8), ((CG, (GC, GC), UA), 2.1), ((CG, (GC, GG), UA), 0.6), ((CG, (GC, GU), UA), 2.1), ((CG, (GC, UA), UA), 2.2), ((CG, (GC, UC), UA), 2.2), ((CG, (GC, UG), UA), 2.2), ((CG, (GC, UU), UA), 1.3),
    ((CG, (GG, AA), UA), 0.5), ((CG, (GG, AC), UA), 0.2), ((CG, (GG, AG), UA), 0.5), ((CG, (GG, AU), UA), 0.2), ((CG, (GG, CA), UA), 0.3), ((CG, (GG, CC), UA), 0.3), ((CG, (GG, CG), UA), 0.3), ((CG, (GG, CU), UA), 0.3), ((CG, (GG, GA), UA), 1.2), ((CG, (GG, GC), UA), 0.2), ((CG, (GG, GG), UA), 1.3), ((CG, (GG, GU), UA), 0.2), ((CG, (GG, UA), UA), 0.3), ((CG, (GG, UC), UA), 0.3), ((CG, (GG, UG), UA), 0.3), ((CG, (GG, UU), UA), 0.7),
    ((CG, (GU, AA), UA), 2.4), ((CG, (GU, AC), UA), 2.1), ((CG, (GU, AG), UA), 1.1), ((CG, (GU, AU), UA), 2.1), ((CG, (GU, CA), UA), 2.2), ((CG, (GU, CC), UA), 2.2), ((CG, (GU, CG), UA), 2.2), ((CG, (GU, CU), UA), 2.2), ((CG, (GU, GA), UA), 1.8), ((CG, (GU, GC), UA), 2.1), ((CG, (GU, GG), UA), 0.6), ((CG, (GU, GU), UA), 2.1), ((CG, (GU, UA), UA), 2.2), ((CG, (GU, UC), UA), 2.2), ((CG, (GU, UG), UA), 2.2), ((CG, (GU, UU), UA), 1.3),
    ((CG, (UA, AA), UA), 1.9), ((CG, (UA, AC), UA), 1.7), ((CG, (UA, AG), UA), 0.7), ((CG, (UA, AU), UA), 1.7), ((CG, (UA, CA), UA), 1.8), ((CG, (UA, CC), UA), 1.7), ((CG, (UA, CG), UA), 1.8), ((CG, (UA, CU), UA), 1.7), ((CG, (UA, GA), UA), 1.4), ((CG, (UA, GC), UA), 1.7), ((CG, (UA, GG), UA), 0.2), ((CG, (UA, GU), UA), 1.7), ((CG, (UA, UA), UA), 1.8), ((CG, (UA, UC), UA), 1.7), ((CG, (UA, UG), UA), 1.8), ((CG, (UA, UU), UA), 0.8),
    ((CG, (UC, AA), UA), 2.1), ((CG, (UC, AC), UA), 1.8), ((CG, (UC, AG), UA), 1.4), ((CG, (UC, AU), UA), 1.8), ((CG, (UC, CA), UA), 1.9), ((CG, (UC, CC), UA), 1.9), ((CG, (UC, CG), UA), 1.9), ((CG, (UC, CU), UA), 1.9), ((CG, (UC, GA), UA), 2.1), ((CG, (UC, GC), UA), 1.8), ((CG, (UC, GG), UA), 0.3), ((CG, (UC, GU), UA), 1.8), ((CG, (UC, UA), UA), 1.9), ((CG, (UC, UC), UA), 1.9), ((CG, (UC, UG), UA), 1.9), ((CG, (UC, UU), UA), 1.0),
    ((CG, (UG, AA), UA), 1.9), ((CG, (UG, AC), UA), 1.7), ((CG, (UG, AG), UA), 0.7), ((CG, (UG, AU), UA), 1.7), ((CG, (UG, CA), UA), 1.8), ((CG, (UG, CC), UA), 1.7), ((CG, (UG, CG), UA), 1.8), ((CG, (UG, CU), UA), 1.7), ((CG, (UG, GA), UA), 1.4), ((CG, (UG, GC), UA), 1.7), ((CG, (UG, GG), UA), 0.2), ((CG, (UG, GU), UA), 1.7), ((CG, (UG, UA), UA), 1.8), ((CG, (UG, UC), UA), 1.7), ((CG, (UG, UG), UA), 1.8), ((CG, (UG, UU), UA), 0.8),
    ((CG, (UU, AA), UA), 1.8), ((CG, (UU, AC), UA), 0.9), ((CG, (UU, AG), UA), 0.0), ((CG, (UU, AU), UA), 0.9), ((CG, (UU, CA), UA), 1.0), ((CG, (UU, CC), UA), 1.0), ((CG, (UU, CG), UA), 1.0), ((CG, (UU, CU), UA), 1.0), ((CG, (UU, GA), UA), 0.6), ((CG, (UU, GC), UA), 0.9), ((CG, (UU, GG), UA), 0.7), ((CG, (UU, GU), UA), 0.9), ((CG, (UU, UA), UA), 1.0), ((CG, (UU, UC), UA), 1.0), ((CG, (UU, UG), UA), 1.0), ((CG, (UU, UU), UA), 0.1),
    // For internal loops between the base pairs "CG" and "UG".
    ((CG, (AA, AA), UG), 2.7), ((CG, (AA, AC), UG), 2.3), ((CG, (AA, AG), UG), 1.5), ((CG, (AA, AU), UG), 2.3), ((CG, (AA, CA), UG), 2.3), ((CG, (AA, CC), UG), 2.3), ((CG, (AA, CG), UG), 2.3), ((CG, (AA, CU), UG), 2.3), ((CG, (AA, GA), UG), 1.9), ((CG, (AA, GC), UG), 2.3), ((CG, (AA, GG), UG), 1.0), ((CG, (AA, GU), UG), 2.3), ((CG, (AA, UA), UG), 2.3), ((CG, (AA, UC), UG), 2.3), ((CG, (AA, UG), UG), 2.3), ((CG, (AA, UU), UG), 2.9),
    ((CG, (AC, AA), UG), 3.0), ((CG, (AC, AC), UG), 2.7), ((CG, (AC, AG), UG), 1.9), ((CG, (AC, AU), UG), 2.7), ((CG, (AC, CA), UG), 2.7), ((CG, (AC, CC), UG), 2.7), ((CG, (AC, CG), UG), 2.7), ((CG, (AC, CU), UG), 2.7), ((CG, (AC, GA), UG), 2.3), ((CG, (AC, GC), UG), 2.7), ((CG, (AC, GG), UG), 1.4), ((CG, (AC, GU), UG), 2.7), ((CG, (AC, UA), UG), 2.7), ((CG, (AC, UC), UG), 2.7), ((CG, (AC, UG), UG), 2.7), ((CG, (AC, UU), UG), 2.7),
    ((CG, (AG, AA), UG), 1.7), ((CG, (AG, AC), UG), 1.3), ((CG, (AG, AG), UG), 0.5), ((CG, (AG, AU), UG), 1.3), ((CG, (AG, CA), UG), 1.3), ((CG, (AG, CC), UG), 1.9), ((CG, (AG, CG), UG), 1.3), ((CG, (AG, CU), UG), 1.9), ((CG, (AG, GA), UG), 0.9), ((CG, (AG, GC), UG), 1.3), ((CG, (AG, GG), UG), 1.3), ((CG, (AG, GU), UG), 1.3), ((CG, (AG, UA), UG), 1.3), ((CG, (AG, UC), UG), 1.9), ((CG, (AG, UG), UG), 1.3), ((CG, (AG, UU), UG), 1.3),
    ((CG, (AU, AA), UG), 3.0), ((CG, (AU, AC), UG), 2.7), ((CG, (AU, AG), UG), 1.9), ((CG, (AU, AU), UG), 2.7), ((CG, (AU, CA), UG), 2.7), ((CG, (AU, CC), UG), 2.7), ((CG, (AU, CG), UG), 2.7), ((CG, (AU, CU), UG), 2.7), ((CG, (AU, GA), UG), 2.3), ((CG, (AU, GC), UG), 2.7), ((CG, (AU, GG), UG), 1.4), ((CG, (AU, GU), UG), 2.7), ((CG, (AU, UA), UG), 2.7), ((CG, (AU, UC), UG), 2.7), ((CG, (AU, UG), UG), 2.7), ((CG, (AU, UU), UG), 2.7),
    ((CG, (CA, AA), UG), 2.6), ((CG, (CA, AC), UG), 2.2), ((CG, (CA, AG), UG), 1.4), ((CG, (CA, AU), UG), 2.2), ((CG, (CA, CA), UG), 2.2), ((CG, (CA, CC), UG), 2.2), ((CG, (CA, CG), UG), 2.2), ((CG, (CA, CU), UG), 2.2), ((CG, (CA, GA), UG), 1.8), ((CG, (CA, GC), UG), 2.2), ((CG, (CA, GG), UG), 0.9), ((CG, (CA, GU), UG), 2.2), ((CG, (CA, UA), UG), 2.2), ((CG, (CA, UC), UG), 2.2), ((CG, (CA, UG), UG), 2.2), ((CG, (CA, UU), UG), 2.2),
    ((CG, (CC, AA), UG), 2.9), ((CG, (CC, AC), UG), 2.5), ((CG, (CC, AG), UG), 2.3), ((CG, (CC, AU), UG), 2.5), ((CG, (CC, CA), UG), 2.5), ((CG, (CC, CC), UG), 2.5), ((CG, (CC, CG), UG), 2.5), ((CG, (CC, CU), UG), 2.5), ((CG, (CC, GA), UG), 2.7), ((CG, (CC, GC), UG), 2.5), ((CG, (CC, GG), UG), 1.2), ((CG, (CC, GU), UG), 2.5), ((CG, (CC, UA), UG), 2.5), ((CG, (CC, UC), UG), 2.5), ((CG, (CC, UG), UG), 2.5), ((CG, (CC, UU), UG), 2.5),
    ((CG, (CG, AA), UG), 2.6), ((CG, (CG, AC), UG), 2.2), ((CG, (CG, AG), UG), 1.4), ((CG, (CG, AU), UG), 2.2), ((CG, (CG, CA), UG), 2.2), ((CG, (CG, CC), UG), 2.2), ((CG, (CG, CG), UG), 2.2), ((CG, (CG, CU), UG), 2.2), ((CG, (CG, GA), UG), 1.8), ((CG, (CG, GC), UG), 2.2), ((CG, (CG, GG), UG), 0.9), ((CG, (CG, GU), UG), 2.2), ((CG, (CG, UA), UG), 2.2), ((CG, (CG, UC), UG), 2.2), ((CG, (CG, UG), UG), 2.2), ((CG, (CG, UU), UG), 2.2),
    ((CG, (CU, AA), UG), 2.7), ((CG, (CU, AC), UG), 2.4), ((CG, (CU, AG), UG), 2.2), ((CG, (CU, AU), UG), 2.4), ((CG, (CU, CA), UG), 2.4), ((CG, (CU, CC), UG), 2.4), ((CG, (CU, CG), UG), 2.4), ((CG, (CU, CU), UG), 2.4), ((CG, (CU, GA), UG), 2.6), ((CG, (CU, GC), UG), 2.4), ((CG, (CU, GG), UG), 1.1), ((CG, (CU, GU), UG), 2.4), ((CG, (CU, UA), UG), 2.4), ((CG, (CU, UC), UG), 2.4), ((CG, (CU, UG), UG), 2.4), ((CG, (CU, UU), UG), 2.4),
    ((CG, (GA, AA), UG), 1.7), ((CG, (GA, AC), UG), 1.3), ((CG, (GA, AG), UG), 0.5), ((CG, (GA, AU), UG), 1.3), ((CG, (GA, CA), UG), 1.3), ((CG, (GA, CC), UG), 1.9), ((CG, (GA, CG), UG), 1.3), ((CG, (GA, CU), UG), 1.9), ((CG, (GA, GA), UG), 0.9), ((CG, (GA, GC), UG), 1.3), ((CG, (GA, GG), UG), 1.3), ((CG, (GA, GU), UG), 1.3), ((CG, (GA, UA), UG), 1.3), ((CG, (GA, UC), UG), 1.9), ((CG, (GA, UG), UG), 1.3), ((CG, (GA, UU), UG), 1.3),
    ((CG, (GC, AA), UG), 3.0), ((CG, (GC, AC), UG), 2.7), ((CG, (GC, AG), UG), 1.9), ((CG, (GC, AU), UG), 2.7), ((CG, (GC, CA), UG), 2.7), ((CG, (GC, CC), UG), 2.7), ((CG, (GC, CG), UG), 2.7), ((CG, (GC, CU), UG), 2.7), ((CG, (GC, GA), UG), 2.3), ((CG, (GC, GC), UG), 2.7), ((CG, (GC, GG), UG), 1.4), ((CG, (GC, GU), UG), 2.7), ((CG, (GC, UA), UG), 2.7), ((CG, (GC, UC), UG), 2.7), ((CG, (GC, UG), UG), 2.7), ((CG, (GC, UU), UG), 2.7),
    ((CG, (GG, AA), UG), 1.1), ((CG, (GG, AC), UG), 0.8), ((CG, (GG, AG), UG), 1.3), ((CG, (GG, AU), UG), 0.8), ((CG, (GG, CA), UG), 0.8), ((CG, (GG, CC), UG), 0.8), ((CG, (GG, CG), UG), 0.8), ((CG, (GG, CU), UG), 0.8), ((CG, (GG, GA), UG), 1.7), ((CG, (GG, GC), UG), 0.8), ((CG, (GG, GG), UG), 2.1), ((CG, (GG, GU), UG), 0.8), ((CG, (GG, UA), UG), 0.8), ((CG, (GG, UC), UG), 0.8), ((CG, (GG, UG), UG), 0.8), ((CG, (GG, UU), UG), 2.1),
    ((CG, (GU, AA), UG), 3.0), ((CG, (GU, AC), UG), 2.7), ((CG, (GU, AG), UG), 1.9), ((CG, (GU, AU), UG), 2.7), ((CG, (GU, CA), UG), 2.7), ((CG, (GU, CC), UG), 2.7), ((CG, (GU, CG), UG), 2.7), ((CG, (GU, CU), UG), 2.7), ((CG, (GU, GA), UG), 2.3), ((CG, (GU, GC), UG), 2.7), ((CG, (GU, GG), UG), 1.4), ((CG, (GU, GU), UG), 2.7), ((CG, (GU, UA), UG), 2.7), ((CG, (GU, UC), UG), 2.7), ((CG, (GU, UG), UG), 2.7), ((CG, (GU, UU), UG), 2.7),
    ((CG, (UA, AA), UG), 2.6), ((CG, (UA, AC), UG), 2.2), ((CG, (UA, AG), UG), 1.4), ((CG, (UA, AU), UG), 2.2), ((CG, (UA, CA), UG), 2.2), ((CG, (UA, CC), UG), 2.2), ((CG, (UA, CG), UG), 2.2), ((CG, (UA, CU), UG), 2.2), ((CG, (UA, GA), UG), 1.8), ((CG, (UA, GC), UG), 2.2), ((CG, (UA, GG), UG), 0.9), ((CG, (UA, GU), UG), 2.2), ((CG, (UA, UA), UG), 2.2), ((CG, (UA, UC), UG), 2.2), ((CG, (UA, UG), UG), 2.2), ((CG, (UA, UU), UG), 2.2),
    ((CG, (UC, AA), UG), 2.7), ((CG, (UC, AC), UG), 2.4), ((CG, (UC, AG), UG), 2.2), ((CG, (UC, AU), UG), 2.4), ((CG, (UC, CA), UG), 2.4), ((CG, (UC, CC), UG), 2.4), ((CG, (UC, CG), UG), 2.4), ((CG, (UC, CU), UG), 2.4), ((CG, (UC, GA), UG), 2.6), ((CG, (UC, GC), UG), 2.4), ((CG, (UC, GG), UG), 1.1), ((CG, (UC, GU), UG), 2.4), ((CG, (UC, UA), UG), 2.4), ((CG, (UC, UC), UG), 2.4), ((CG, (UC, UG), UG), 2.4), ((CG, (UC, UU), UG), 2.4),
    ((CG, (UG, AA), UG), 2.6), ((CG, (UG, AC), UG), 2.2), ((CG, (UG, AG), UG), 1.4), ((CG, (UG, AU), UG), 2.2), ((CG, (UG, CA), UG), 2.2), ((CG, (UG, CC), UG), 2.2), ((CG, (UG, CG), UG), 2.2), ((CG, (UG, CU), UG), 2.2), ((CG, (UG, GA), UG), 1.8), ((CG, (UG, GC), UG), 2.2), ((CG, (UG, GG), UG), 0.9), ((CG, (UG, GU), UG), 2.2), ((CG, (UG, UA), UG), 2.2), ((CG, (UG, UC), UG), 2.2), ((CG, (UG, UG), UG), 2.2), ((CG, (UG, UU), UG), 2.2),
    ((CG, (UU, AA), UG), 2.4), ((CG, (UU, AC), UG), 1.5), ((CG, (UU, AG), UG), 0.7), ((CG, (UU, AU), UG), 1.5), ((CG, (UU, CA), UG), 1.5), ((CG, (UU, CC), UG), 1.5), ((CG, (UU, CG), UG), 1.5), ((CG, (UU, CU), UG), 1.5), ((CG, (UU, GA), UG), 1.1), ((CG, (UU, GC), UG), 1.5), ((CG, (UU, GG), UG), 1.5), ((CG, (UU, GU), UG), 1.5), ((CG, (UU, UA), UG), 1.5), ((CG, (UU, UC), UG), 1.5), ((CG, (UU, UG), UG), 1.5), ((CG, (UU, UU), UG), 1.5),
    // For internal loops between the base pairs "GC" and "AU".
    ((GC, (AA, AA), AU), 2.1), ((GC, (AA, AC), AU), 1.7), ((GC, (AA, AG), AU), 1.1), ((GC, (AA, AU), AU), 1.7), ((GC, (AA, CA), AU), 2.1), ((GC, (AA, CC), AU), 2.1), ((GC, (AA, CG), AU), 2.1), ((GC, (AA, CU), AU), 2.1), ((GC, (AA, GA), AU), 1.2), ((GC, (AA, GC), AU), 1.7), ((GC, (AA, GG), AU), 0.6), ((GC, (AA, GU), AU), 1.7), ((GC, (AA, UA), AU), 2.1), ((GC, (AA, UC), AU), 1.8), ((GC, (AA, UG), AU), 2.1), ((GC, (AA, UU), AU), 1.9),
    ((GC, (AC, AA), AU), 1.8), ((GC, (AC, AC), AU), 1.4), ((GC, (AC, AG), AU), 0.8), ((GC, (AC, AU), AU), 1.4), ((GC, (AC, CA), AU), 1.8), ((GC, (AC, CC), AU), 1.8), ((GC, (AC, CG), AU), 1.8), ((GC, (AC, CU), AU), 1.8), ((GC, (AC, GA), AU), 0.9), ((GC, (AC, GC), AU), 1.4), ((GC, (AC, GG), AU), 0.3), ((GC, (AC, GU), AU), 1.4), ((GC, (AC, UA), AU), 1.8), ((GC, (AC, UC), AU), 1.5), ((GC, (AC, UG), AU), 1.8), ((GC, (AC, UU), AU), 1.0),
    ((GC, (AG, AA), AU), 0.7), ((GC, (AG, AC), AU), 0.3), ((GC, (AG, AG), AU), -0.3), ((GC, (AG, AU), AU), 0.3), ((GC, (AG, CA), AU), 0.7), ((GC, (AG, CC), AU), 1.3), ((GC, (AG, CG), AU), 0.7), ((GC, (AG, CU), AU), 1.3), ((GC, (AG, GA), AU), -0.2), ((GC, (AG, GC), AU), 0.3), ((GC, (AG, GG), AU), 0.5), ((GC, (AG, GU), AU), 0.3), ((GC, (AG, UA), AU), 0.7), ((GC, (AG, UC), AU), 1.0), ((GC, (AG, UG), AU), 0.7), ((GC, (AG, UU), AU), -0.1),
    ((GC, (AU, AA), AU), 1.8), ((GC, (AU, AC), AU), 1.4), ((GC, (AU, AG), AU), 0.8), ((GC, (AU, AU), AU), 1.4), ((GC, (AU, CA), AU), 1.8), ((GC, (AU, CC), AU), 1.8), ((GC, (AU, CG), AU), 1.8), ((GC, (AU, CU), AU), 1.8), ((GC, (AU, GA), AU), 0.9), ((GC, (AU, GC), AU), 1.4), ((GC, (AU, GG), AU), 0.3), ((GC, (AU, GU), AU), 1.4), ((GC, (AU, UA), AU), 1.8), ((GC, (AU, UC), AU), 1.5), ((GC, (AU, UG), AU), 1.8), ((GC, (AU, UU), AU), 1.0),
    ((GC, (CA, AA), AU), 1.9), ((GC, (CA, AC), AU), 1.4), ((GC, (CA, AG), AU), 0.8), ((GC, (CA, AU), AU), 1.4), ((GC, (CA, CA), AU), 1.9), ((GC, (CA, CC), AU), 1.9), ((GC, (CA, CG), AU), 1.9), ((GC, (CA, CU), AU), 1.9), ((GC, (CA, GA), AU), 0.9), ((GC, (CA, GC), AU), 1.4), ((GC, (CA, GG), AU), 0.3), ((GC, (CA, GU), AU), 1.4), ((GC, (CA, UA), AU), 1.9), ((GC, (CA, UC), AU), 1.6), ((GC, (CA, UG), AU), 1.9), ((GC, (CA, UU), AU), 1.0),
    ((GC, (CC, AA), AU), 1.9), ((GC, (CC, AC), AU), 1.4), ((GC, (CC, AG), AU), 1.4), ((GC, (CC, AU), AU), 1.4), ((GC, (CC, CA), AU), 1.9), ((GC, (CC, CC), AU), 1.9), ((GC, (CC, CG), AU), 1.9), ((GC, (CC, CU), AU), 1.9), ((GC, (CC, GA), AU), 1.5), ((GC, (CC, GC), AU), 1.4), ((GC, (CC, GG), AU), 0.3), ((GC, (CC, GU), AU), 1.4), ((GC, (CC, UA), AU), 1.9), ((GC, (CC, UC), AU), 1.6), ((GC, (CC, UG), AU), 1.9), ((GC, (CC, UU), AU), 1.0),
    ((GC, (CG, AA), AU), 1.9), ((GC, (CG, AC), AU), 1.4), ((GC, (CG, AG), AU), 0.8), ((GC, (CG, AU), AU), 1.4), ((GC, (CG, CA), AU), 1.9), ((GC, (CG, CC), AU), 1.9), ((GC, (CG, CG), AU), 1.9), ((GC, (CG, CU), AU), 1.9), ((GC, (CG, GA), AU), 0.9), ((GC, (CG, GC), AU), 1.4), ((GC, (CG, GG), AU), 0.3), ((GC, (CG, GU), AU), 1.4), ((GC, (CG, UA), AU), 1.9), ((GC, (CG, UC), AU), 1.6), ((GC, (CG, UG), AU), 1.9), ((GC, (CG, UU), AU), 1.0),
    ((GC, (CU, AA), AU), 1.9), ((GC, (CU, AC), AU), 1.5), ((GC, (CU, AG), AU), 1.5), ((GC, (CU, AU), AU), 1.5), ((GC, (CU, CA), AU), 1.9), ((GC, (CU, CC), AU), 1.9), ((GC, (CU, CG), AU), 1.9), ((GC, (CU, CU), AU), 1.9), ((GC, (CU, GA), AU), 1.6), ((GC, (CU, GC), AU), 1.5), ((GC, (CU, GG), AU), 0.4), ((GC, (CU, GU), AU), 1.5), ((GC, (CU, UA), AU), 1.9), ((GC, (CU, UC), AU), 1.6), ((GC, (CU, UG), AU), 1.9), ((GC, (CU, UU), AU), 1.1),
    ((GC, (GA, AA), AU), 0.1), ((GC, (GA, AC), AU), -0.3), ((GC, (GA, AG), AU), -0.9), ((GC, (GA, AU), AU), -0.3), ((GC, (GA, CA), AU), 0.1), ((GC, (GA, CC), AU), 0.7), ((GC, (GA, CG), AU), 0.1), ((GC, (GA, CU), AU), 0.7), ((GC, (GA, GA), AU), -0.8), ((GC, (GA, GC), AU), -0.3), ((GC, (GA, GG), AU), -0.1), ((GC, (GA, GU), AU), -0.3), ((GC, (GA, UA), AU), 0.1), ((GC, (GA, UC), AU), 0.4), ((GC, (GA, UG), AU), 0.1), ((GC, (GA, UU), AU), -0.7),
    ((GC, (GC, AA), AU), 1.8), ((GC, (GC, AC), AU), 1.4), ((GC, (GC, AG), AU), 0.8), ((GC, (GC, AU), AU), 1.4), ((GC, (GC, CA), AU), 1.8), ((GC, (GC, CC), AU), 1.8), ((GC, (GC, CG), AU), 1.8), ((GC, (GC, CU), AU), 1.8), ((GC, (GC, GA), AU), 0.9), ((GC, (GC, GC), AU), 1.4), ((GC, (GC, GG), AU), 0.3), ((GC, (GC, GU), AU), 1.4), ((GC, (GC, UA), AU), 1.8), ((GC, (GC, UC), AU), 1.5), ((GC, (GC, UG), AU), 1.8), ((GC, (GC, UU), AU), 1.0),
    ((GC, (GG, AA), AU), 0.5), ((GC, (GG, AC), AU), 0.0), ((GC, (GG, AG), AU), 0.7), ((GC, (GG, AU), AU), 0.0), ((GC, (GG, CA), AU), 0.5), ((GC, (GG, CC), AU), 0.5), ((GC, (GG, CG), AU), 0.5), ((GC, (GG, CU), AU), 0.5), ((GC, (GG, GA), AU), 0.8), ((GC, (GG, GC), AU), 0.0), ((GC, (GG, GG), AU), 1.5), ((GC, (GG, GU), AU), 0.0), ((GC, (GG, UA), AU), 0.5), ((GC, (GG, UC), AU), 0.2), ((GC, (GG, UG), AU), 0.5), ((GC, (GG, UU), AU), 0.9),
    ((GC, (GU, AA), AU), 1.8), ((GC, (GU, AC), AU), 1.4), ((GC, (GU, AG), AU), 0.8), ((GC, (GU, AU), AU), 1.4), ((GC, (GU, CA), AU), 1.8), ((GC, (GU, CC), AU), 1.8), ((GC, (GU, CG), AU), 1.8), ((GC, (GU, CU), AU), 1.8), ((GC, (GU, GA), AU), 0.9), ((GC, (GU, GC), AU), 1.4), ((GC, (GU, GG), AU), 0.3), ((GC, (GU, GU), AU), 1.4), ((GC, (GU, UA), AU), 1.8), ((GC, (GU, UC), AU), 1.5), ((GC, (GU, UG), AU), 1.8), ((GC, (GU, UU), AU), 1.0),
    ((GC, (UA, AA), AU), 1.9), ((GC, (UA, AC), AU), 1.4), ((GC, (UA, AG), AU), 0.8), ((GC, (UA, AU), AU), 1.4), ((GC, (UA, CA), AU), 1.9), ((GC, (UA, CC), AU), 1.9), ((GC, (UA, CG), AU), 1.9), ((GC, (UA, CU), AU), 1.9), ((GC, (UA, GA), AU), 0.9), ((GC, (UA, GC), AU), 1.4), ((GC, (UA, GG), AU), 0.3), ((GC, (UA, GU), AU), 1.4), ((GC, (UA, UA), AU), 1.9), ((GC, (UA, UC), AU), 1.6), ((GC, (UA, UG), AU), 1.9), ((GC, (UA, UU), AU), 1.0),
    ((GC, (UC, AA), AU), 1.9), ((GC, (UC, AC), AU), 1.4), ((GC, (UC, AG), AU), 1.4), ((GC, (UC, AU), AU), 1.4), ((GC, (UC, CA), AU), 1.9), ((GC, (UC, CC), AU), 1.9), ((GC, (UC, CG), AU), 1.9), ((GC, (UC, CU), AU), 1.9), ((GC, (UC, GA), AU), 1.5), ((GC, (UC, GC), AU), 1.4), ((GC, (UC, GG), AU), 0.3), ((GC, (UC, GU), AU), 1.4), ((GC, (UC, UA), AU), 1.9), ((GC, (UC, UC), AU), 1.6), ((GC, (UC, UG), AU), 1.9), ((GC, (UC, UU), AU), 1.0),
    ((GC, (UG, AA), AU), 1.9), ((GC, (UG, AC), AU), 1.4), ((GC, (UG, AG), AU), 0.8), ((GC, (UG, AU), AU), 1.4), ((GC, (UG, CA), AU), 1.9), ((GC, (UG, CC), AU), 1.9), ((GC, (UG, CG), AU), 1.9), ((GC, (UG, CU), AU), 1.9), ((GC, (UG, GA), AU), 0.9), ((GC, (UG, GC), AU), 1.4), ((GC, (UG, GG), AU), 0.3), ((GC, (UG, GU), AU), 1.4), ((GC, (UG, UA), AU), 1.9), ((GC, (UG, UC), AU), 1.6), ((GC, (UG, UG), AU), 1.9), ((GC, (UG, UU), AU), 1.0),
    ((GC, (UU, AA), AU), 1.7), ((GC, (UU, AC), AU), 0.7), ((GC, (UU, AG), AU), 0.1), ((GC, (UU, AU), AU), 0.7), ((GC, (UU, CA), AU), 1.1), ((GC, (UU, CC), AU), 1.1), ((GC, (UU, CG), AU), 1.1), ((GC, (UU, CU), AU), 1.1), ((GC, (UU, GA), AU), 0.2), ((GC, (UU, GC), AU), 0.7), ((GC, (UU, GG), AU), 0.9), ((GC, (UU, GU), AU), 0.7), ((GC, (UU, UA), AU), 1.1), ((GC, (UU, UC), AU), 0.8), ((GC, (UU, UG), AU), 1.1), ((GC, (UU, UU), AU), 0.3),
    // For internal loops between the base pairs "GC" and "CG".
    ((GC, (AA, AA), CG), 1.5), ((GC, (AA, AC), CG), 1.2), ((GC, (AA, AG), CG), -0.5), ((GC, (AA, AU), CG), 1.2), ((GC, (AA, CA), CG), 1.2), ((GC, (AA, CC), CG), 1.2), ((GC, (AA, CG), CG), 1.2), ((GC, (AA, CU), CG), 1.2), ((GC, (AA, GA), CG), 0.1), ((GC, (AA, GC), CG), 1.2), ((GC, (AA, GG), CG), -0.2), ((GC, (AA, GU), CG), 1.2), ((GC, (AA, UA), CG), 1.2), ((GC, (AA, UC), CG), 1.3), ((GC, (AA, UG), CG), 1.2), ((GC, (AA, UU), CG), 1.1),
    ((GC, (AC, AA), CG), 1.2), ((GC, (AC, AC), CG), 0.9), ((GC, (AC, AG), CG), -0.8), ((GC, (AC, AU), CG), 0.9), ((GC, (AC, CA), CG), 0.9), ((GC, (AC, CC), CG), 0.9), ((GC, (AC, CG), CG), 0.9), ((GC, (AC, CU), CG), 0.9), ((GC, (AC, GA), CG), -0.2), ((GC, (AC, GC), CG), 0.9), ((GC, (AC, GG), CG), -0.5), ((GC, (AC, GU), CG), 0.9), ((GC, (AC, UA), CG), 0.9), ((GC, (AC, UC), CG), 1.0), ((GC, (AC, UG), CG), 0.9), ((GC, (AC, UU), CG), 0.2),
    ((GC, (AG, AA), CG), 0.1), ((GC, (AG, AC), CG), -0.1), ((GC, (AG, AG), CG), -1.9), ((GC, (AG, AU), CG), -0.1), ((GC, (AG, CA), CG), -0.2), ((GC, (AG, CC), CG), 0.5), ((GC, (AG, CG), CG), -0.2), ((GC, (AG, CU), CG), 0.5), ((GC, (AG, GA), CG), -1.3), ((GC, (AG, GC), CG), -0.1), ((GC, (AG, GG), CG), -0.2), ((GC, (AG, GU), CG), -0.1), ((GC, (AG, UA), CG), -0.2), ((GC, (AG, UC), CG), 0.5), ((GC, (AG, UG), CG), -0.2), ((GC, (AG, UU), CG), -0.9),
    ((GC, (AU, AA), CG), 1.2), ((GC, (AU, AC), CG), 0.9), ((GC, (AU, AG), CG), -0.8), ((GC, (AU, AU), CG), 0.9), ((GC, (AU, CA), CG), 0.9), ((GC, (AU, CC), CG), 0.9), ((GC, (AU, CG), CG), 0.9), ((GC, (AU, CU), CG), 0.9), ((GC, (AU, GA), CG), -0.2), ((GC, (AU, GC), CG), 0.9), ((GC, (AU, GG), CG), -0.5), ((GC, (AU, GU), CG), 0.9), ((GC, (AU, UA), CG), 0.9), ((GC, (AU, UC), CG), 1.0), ((GC, (AU, UG), CG), 0.9), ((GC, (AU, UU), CG), 0.2),
    ((GC, (CA, AA), CG), 1.2), ((GC, (CA, AC), CG), 1.0), ((GC, (CA, AG), CG), -0.8), ((GC, (CA, AU), CG), 1.0), ((GC, (CA, CA), CG), 0.9), ((GC, (CA, CC), CG), 1.0), ((GC, (CA, CG), CG), 0.9), ((GC, (CA, CU), CG), 1.0), ((GC, (CA, GA), CG), -0.1), ((GC, (CA, GC), CG), 1.0), ((GC, (CA, GG), CG), -0.4), ((GC, (CA, GU), CG), 1.0), ((GC, (CA, UA), CG), 0.9), ((GC, (CA, UC), CG), 1.0), ((GC, (CA, UG), CG), 0.9), ((GC, (CA, UU), CG), 0.2),
    ((GC, (CC, AA), CG), 1.2), ((GC, (CC, AC), CG), 1.0), ((GC, (CC, AG), CG), -0.2), ((GC, (CC, AU), CG), 1.0), ((GC, (CC, CA), CG), 0.9), ((GC, (CC, CC), CG), 1.0), ((GC, (CC, CG), CG), 0.9), ((GC, (CC, CU), CG), 1.0), ((GC, (CC, GA), CG), 0.5), ((GC, (CC, GC), CG), 1.0), ((GC, (CC, GG), CG), -0.4), ((GC, (CC, GU), CG), 1.0), ((GC, (CC, UA), CG), 0.9), ((GC, (CC, UC), CG), 1.0), ((GC, (CC, UG), CG), 0.9), ((GC, (CC, UU), CG), 0.2),
    ((GC, (CG, AA), CG), 1.2), ((GC, (CG, AC), CG), 1.0), ((GC, (CG, AG), CG), -0.8), ((GC, (CG, AU), CG), 1.0), ((GC, (CG, CA), CG), 0.9), ((GC, (CG, CC), CG), 1.0), ((GC, (CG, CG), CG), 0.9), ((GC, (CG, CU), CG), 1.0), ((GC, (CG, GA), CG), -0.1), ((GC, (CG, GC), CG), 1.0), ((GC, (CG, GG), CG), -0.4), ((GC, (CG, GU), CG), 1.0), ((GC, (CG, UA), CG), 0.9), ((GC, (CG, UC), CG), 1.0), ((GC, (CG, UG), CG), 0.9), ((GC, (CG, UU), CG), 0.2),
    ((GC, (CU, AA), CG), 1.3), ((GC, (CU, AC), CG), 1.0), ((GC, (CU, AG), CG), -0.1), ((GC, (CU, AU), CG), 1.0), ((GC, (CU, CA), CG), 1.0), ((GC, (CU, CC), CG), 1.0), ((GC, (CU, CG), CG), 1.0), ((GC, (CU, CU), CG), 1.0), ((GC, (CU, GA), CG), 0.5), ((GC, (CU, GC), CG), 1.0), ((GC, (CU, GG), CG), -0.4), ((GC, (CU, GU), CG), 1.0), ((GC, (CU, UA), CG), 1.0), ((GC, (CU, UC), CG), 1.1), ((GC, (CU, UG), CG), 1.0), ((GC, (CU, UU), CG), 0.3),
    ((GC, (GA, AA), CG), -0.5), ((GC, (GA, AC), CG), -0.8), ((GC, (GA, AG), CG), -2.6), ((GC, (GA, AU), CG), -0.8), ((GC, (GA, CA), CG), -0.8), ((GC, (GA, CC), CG), -0.2), ((GC, (GA, CG), CG), -0.8), ((GC, (GA, CU), CG), -0.2), ((GC, (GA, GA), CG), -1.9), ((GC, (GA, GC), CG), -0.8), ((GC, (GA, GG), CG), -0.9), ((GC, (GA, GU), CG), -0.8), ((GC, (GA, UA), CG), -0.8), ((GC, (GA, UC), CG), -0.1), ((GC, (GA, UG), CG), -0.8), ((GC, (GA, UU), CG), -1.5),
    ((GC, (GC, AA), CG), 1.2), ((GC, (GC, AC), CG), 0.9), ((GC, (GC, AG), CG), -0.8), ((GC, (GC, AU), CG), 0.9), ((GC, (GC, CA), CG), 0.9), ((GC, (GC, CC), CG), 0.9), ((GC, (GC, CG), CG), 0.9), ((GC, (GC, CU), CG), 0.9), ((GC, (GC, GA), CG), -0.2), ((GC, (GC, GC), CG), 0.9), ((GC, (GC, GG), CG), -0.5), ((GC, (GC, GU), CG), 0.9), ((GC, (GC, UA), CG), 0.9), ((GC, (GC, UC), CG), 1.0), ((GC, (GC, UG), CG), 0.9), ((GC, (GC, UU), CG), 0.2),
    ((GC, (GG, AA), CG), -0.2), ((GC, (GG, AC), CG), -0.4), ((GC, (GG, AG), CG), -0.9), ((GC, (GG, AU), CG), -0.4), ((GC, (GG, CA), CG), -0.5), ((GC, (GG, CC), CG), -0.4), ((GC, (GG, CG), CG), -0.5), ((GC, (GG, CU), CG), -0.4), ((GC, (GG, GA), CG), -0.2), ((GC, (GG, GC), CG), -0.4), ((GC, (GG, GG), CG), 0.8), ((GC, (GG, GU), CG), -0.4), ((GC, (GG, UA), CG), -0.5), ((GC, (GG, UC), CG), -0.4), ((GC, (GG, UG), CG), -0.5), ((GC, (GG, UU), CG), 0.1),
    ((GC, (GU, AA), CG), 1.2), ((GC, (GU, AC), CG), 0.9), ((GC, (GU, AG), CG), -0.8), ((GC, (GU, AU), CG), 0.9), ((GC, (GU, CA), CG), 0.9), ((GC, (GU, CC), CG), 0.9), ((GC, (GU, CG), CG), 0.9), ((GC, (GU, CU), CG), 0.9), ((GC, (GU, GA), CG), -0.2), ((GC, (GU, GC), CG), 0.9), ((GC, (GU, GG), CG), -0.5), ((GC, (GU, GU), CG), 0.9), ((GC, (GU, UA), CG), 0.9), ((GC, (GU, UC), CG), 1.0), ((GC, (GU, UG), CG), 0.9), ((GC, (GU, UU), CG), 0.2),
    ((GC, (UA, AA), CG), 1.2), ((GC, (UA, AC), CG), 1.0), ((GC, (UA, AG), CG), -0.8), ((GC, (UA, AU), CG), 1.0), ((GC, (UA, CA), CG), 0.9), ((GC, (UA, CC), CG), 1.0), ((GC, (UA, CG), CG), 0.9), ((GC, (UA, CU), CG), 1.0), ((GC, (UA, GA), CG), -0.1), ((GC, (UA, GC), CG), 1.0), ((GC, (UA, GG), CG), -0.4), ((GC, (UA, GU), CG), 1.0), ((GC, (UA, UA), CG), 0.9), ((GC, (UA, UC), CG), 1.0), ((GC, (UA, UG), CG), 0.9), ((GC, (UA, UU), CG), 0.2),
    ((GC, (UC, AA), CG), 1.2), ((GC, (UC, AC), CG), 1.0), ((GC, (UC, AG), CG), -0.2), ((GC, (UC, AU), CG), 1.0), ((GC, (UC, CA), CG), 0.9), ((GC, (UC, CC), CG), 1.0), ((GC, (UC, CG), CG), 0.9), ((GC, (UC, CU), CG), 1.0), ((GC, (UC, GA), CG), 0.5), ((GC, (UC, GC), CG), 1.0), ((GC, (UC, GG), CG), -0.4), ((GC, (UC, GU), CG), 1.0), ((GC, (UC, UA), CG), 0.9), ((GC, (UC, UC), CG), 1.0), ((GC, (UC, UG), CG), 0.9), ((GC, (UC, UU), CG), 0.2),
    ((GC, (UG, AA), CG), 1.2), ((GC, (UG, AC), CG), 1.0), ((GC, (UG, AG), CG), -0.8), ((GC, (UG, AU), CG), 1.0), ((GC, (UG, CA), CG), 0.9), ((GC, (UG, CC), CG), 1.0), ((GC, (UG, CG), CG), 0.9), ((GC, (UG, CU), CG), 1.0), ((GC, (UG, GA), CG), -0.1), ((GC, (UG, GC), CG), 1.0), ((GC, (UG, GG), CG), -0.4), ((GC, (UG, GU), CG), 1.0), ((GC, (UG, UA), CG), 0.9), ((GC, (UG, UC), CG), 1.0), ((GC, (UG, UG), CG), 0.9), ((GC, (UG, UU), CG), 0.2),
    ((GC, (UU, AA), CG), 1.1), ((GC, (UU, AC), CG), 0.2), ((GC, (UU, AG), CG), -1.5), ((GC, (UU, AU), CG), 0.2), ((GC, (UU, CA), CG), 0.2), ((GC, (UU, CC), CG), 0.2), ((GC, (UU, CG), CG), 0.2), ((GC, (UU, CU), CG), 0.2), ((GC, (UU, GA), CG), -0.9), ((GC, (UU, GC), CG), 0.2), ((GC, (UU, GG), CG), 0.1), ((GC, (UU, GU), CG), 0.2), ((GC, (UU, UA), CG), 0.2), ((GC, (UU, UC), CG), 0.3), ((GC, (UU, UG), CG), 0.2), ((GC, (UU, UU), CG), -0.5),
    // For internal loops between the base pairs "GC" and "GC".
    ((GC, (AA, AA), GC), 1.3), ((GC, (AA, AC), GC), 1.3), ((GC, (AA, AG), GC), -0.2), ((GC, (AA, AU), GC), 1.3), ((GC, (AA, CA), GC), 0.6), ((GC, (AA, CC), GC), 2.2), ((GC, (AA, CG), GC), 1.7), ((GC, (AA, CU), GC), 1.4), ((GC, (AA, GA), GC), 0.0), ((GC, (AA, GC), GC), 1.3), ((GC, (AA, GG), GC), -0.1), ((GC, (AA, GU), GC), 1.3), ((GC, (AA, UA), GC), 1.7), ((GC, (AA, UC), GC), 1.4), ((GC, (AA, UG), GC), 1.7), ((GC, (AA, UU), GC), 1.4),
    ((GC, (AC, AA), GC), 1.0), ((GC, (AC, AC), GC), 1.1), ((GC, (AC, AG), GC), 0.7), ((GC, (AC, AU), GC), 1.0), ((GC, (AC, CA), GC), 0.5), ((GC, (AC, CC), GC), 1.9), ((GC, (AC, CG), GC), 1.4), ((GC, (AC, CU), GC), 1.1), ((GC, (AC, GA), GC), -1.0), ((GC, (AC, GC), GC), 1.0), ((GC, (AC, GG), GC), -0.5), ((GC, (AC, GU), GC), 1.0), ((GC, (AC, UA), GC), 1.4), ((GC, (AC, UC), GC), 1.1), ((GC, (AC, UG), GC), 1.4), ((GC, (AC, UU), GC), 0.3),
    ((GC, (AG, AA), GC), 0.4), ((GC, (AG, AC), GC), 0.7), ((GC, (AG, AG), GC), -0.5), ((GC, (AG, AU), GC), -0.1), ((GC, (AG, CA), GC), 0.3), ((GC, (AG, CC), GC), 0.7), ((GC, (AG, CG), GC), 0.3), ((GC, (AG, CU), GC), 0.5), ((GC, (AG, GA), GC), -0.7), ((GC, (AG, GC), GC), -0.1), ((GC, (AG, GG), GC), -0.3), ((GC, (AG, GU), GC), -0.1), ((GC, (AG, UA), GC), 0.3), ((GC, (AG, UC), GC), 0.6), ((GC, (AG, UG), GC), 0.3), ((GC, (AG, UU), GC), 1.4),
    ((GC, (AU, AA), GC), 1.0), ((GC, (AU, AC), GC), 1.0), ((GC, (AU, AG), GC), 0.1), ((GC, (AU, AU), GC), 1.0), ((GC, (AU, CA), GC), 1.4), ((GC, (AU, CC), GC), 1.3), ((GC, (AU, CG), GC), 1.4), ((GC, (AU, CU), GC), 1.1), ((GC, (AU, GA), GC), 0.1), ((GC, (AU, GC), GC), 1.0), ((GC, (AU, GG), GC), -0.5), ((GC, (AU, GU), GC), 1.0), ((GC, (AU, UA), GC), 1.4), ((GC, (AU, UC), GC), 1.1), ((GC, (AU, UG), GC), 1.4), ((GC, (AU, UU), GC), 0.2),
    ((GC, (CA, AA), GC), 1.1), ((GC, (CA, AC), GC), 1.0), ((GC, (CA, AG), GC), -0.4), ((GC, (CA, AU), GC), 1.0), ((GC, (CA, CA), GC), 1.5), ((GC, (CA, CC), GC), 1.3), ((GC, (CA, CG), GC), 1.5), ((GC, (CA, CU), GC), 1.2), ((GC, (CA, GA), GC), -0.7), ((GC, (CA, GC), GC), 1.0), ((GC, (CA, GG), GC), -0.4), ((GC, (CA, GU), GC), 1.0), ((GC, (CA, UA), GC), 1.5), ((GC, (CA, UC), GC), 1.2), ((GC, (CA, UG), GC), 1.5), ((GC, (CA, UU), GC), 0.3),
    ((GC, (CC, AA), GC), 1.1), ((GC, (CC, AC), GC), 1.0), ((GC, (CC, AG), GC), 0.7), ((GC, (CC, AU), GC), 1.0), ((GC, (CC, CA), GC), 1.5), ((GC, (CC, CC), GC), 1.3), ((GC, (CC, CG), GC), 1.5), ((GC, (CC, CU), GC), 1.2), ((GC, (CC, GA), GC), -0.6), ((GC, (CC, GC), GC), 1.0), ((GC, (CC, GG), GC), -0.4), ((GC, (CC, GU), GC), 1.0), ((GC, (CC, UA), GC), 1.5), ((GC, (CC, UC), GC), 1.2), ((GC, (CC, UG), GC), 1.5), ((GC, (CC, UU), GC), 0.3),
    ((GC, (CG, AA), GC), 1.1), ((GC, (CG, AC), GC), 1.0), ((GC, (CG, AG), GC), 0.1), ((GC, (CG, AU), GC), 1.0), ((GC, (CG, CA), GC), 1.5), ((GC, (CG, CC), GC), 1.3), ((GC, (CG, CG), GC), 1.5), ((GC, (CG, CU), GC), 1.2), ((GC, (CG, GA), GC), 0.1), ((GC, (CG, GC), GC), 1.0), ((GC, (CG, GG), GC), -0.4), ((GC, (CG, GU), GC), 1.0), ((GC, (CG, UA), GC), 1.5), ((GC, (CG, UC), GC), 1.2), ((GC, (CG, UG), GC), 1.5), ((GC, (CG, UU), GC), 0.3),
    ((GC, (CU, AA), GC), 1.1), ((GC, (CU, AC), GC), 1.1), ((GC, (CU, AG), GC), 0.8), ((GC, (CU, AU), GC), 1.1), ((GC, (CU, CA), GC), 1.5), ((GC, (CU, CC), GC), 1.4), ((GC, (CU, CG), GC), 1.5), ((GC, (CU, CU), GC), 1.2), ((GC, (CU, GA), GC), 0.8), ((GC, (CU, GC), GC), 1.1), ((GC, (CU, GG), GC), -0.5), ((GC, (CU, GU), GC), 1.1), ((GC, (CU, UA), GC), 1.5), ((GC, (CU, UC), GC), 1.2), ((GC, (CU, UG), GC), 1.5), ((GC, (CU, UU), GC), 0.3),
    ((GC, (GA, AA), GC), -0.3), ((GC, (GA, AC), GC), -0.7), ((GC, (GA, AG), GC), -1.7), ((GC, (GA, AU), GC), -0.7), ((GC, (GA, CA), GC), 0.1), ((GC, (GA, CC), GC), 0.7), ((GC, (GA, CG), GC), -0.3), ((GC, (GA, CU), GC), 0.0), ((GC, (GA, GA), GC), -1.6), ((GC, (GA, GC), GC), -0.7), ((GC, (GA, GG), GC), -0.9), ((GC, (GA, GU), GC), -0.7), ((GC, (GA, UA), GC), -0.3), ((GC, (GA, UC), GC), 0.0), ((GC, (GA, UG), GC), -0.3), ((GC, (GA, UU), GC), 0.5),
    ((GC, (GC, AA), GC), 1.0), ((GC, (GC, AC), GC), 1.0), ((GC, (GC, AG), GC), 0.1), ((GC, (GC, AU), GC), 1.0), ((GC, (GC, CA), GC), 1.4), ((GC, (GC, CC), GC), 1.3), ((GC, (GC, CG), GC), 1.4), ((GC, (GC, CU), GC), 1.1), ((GC, (GC, GA), GC), 0.1), ((GC, (GC, GC), GC), 1.0), ((GC, (GC, GG), GC), -0.5), ((GC, (GC, GU), GC), 1.0), ((GC, (GC, UA), GC), 1.4), ((GC, (GC, UC), GC), 1.1), ((GC, (GC, UG), GC), 1.4), ((GC, (GC, UU), GC), 0.2),
    ((GC, (GG, AA), GC), -0.3), ((GC, (GG, AC), GC), -0.4), ((GC, (GG, AG), GC), -0.3), ((GC, (GG, AU), GC), -0.4), ((GC, (GG, CA), GC), -0.3), ((GC, (GG, CC), GC), -0.1), ((GC, (GG, CG), GC), 0.1), ((GC, (GG, CU), GC), -0.6), ((GC, (GG, GA), GC), 0.0), ((GC, (GG, GC), GC), -0.4), ((GC, (GG, GG), GC), 0.8), ((GC, (GG, GU), GC), -0.4), ((GC, (GG, UA), GC), 0.1), ((GC, (GG, UC), GC), 0.2), ((GC, (GG, UG), GC), 0.1), ((GC, (GG, UU), GC), 0.7),
    ((GC, (GU, AA), GC), 1.0), ((GC, (GU, AC), GC), 1.0), ((GC, (GU, AG), GC), 0.1), ((GC, (GU, AU), GC), 1.0), ((GC, (GU, CA), GC), 1.4), ((GC, (GU, CC), GC), 1.3), ((GC, (GU, CG), GC), 1.4), ((GC, (GU, CU), GC), 1.1), ((GC, (GU, GA), GC), 0.1), ((GC, (GU, GC), GC), 1.0), ((GC, (GU, GG), GC), -0.5), ((GC, (GU, GU), GC), 1.0), ((GC, (GU, UA), GC), 1.4), ((GC, (GU, UC), GC), 1.1), ((GC, (GU, UG), GC), 1.4), ((GC, (GU, UU), GC), 0.2),
    ((GC, (UA, AA), GC), 1.1), ((GC, (UA, AC), GC), 1.0), ((GC, (UA, AG), GC), 0.1), ((GC, (UA, AU), GC), 1.0), ((GC, (UA, CA), GC), 1.5), ((GC, (UA, CC), GC), 1.3), ((GC, (UA, CG), GC), 1.5), ((GC, (UA, CU), GC), 1.2), ((GC, (UA, GA), GC), 0.1), ((GC, (UA, GC), GC), 1.0), ((GC, (UA, GG), GC), -0.4), ((GC, (UA, GU), GC), 1.0), ((GC, (UA, UA), GC), 1.5), ((GC, (UA, UC), GC), 1.2), ((GC, (UA, UG), GC), 1.5), ((GC, (UA, UU), GC), 0.3),
    ((GC, (UC, AA), GC), 1.1), ((GC, (UC, AC), GC), 1.0), ((GC, (UC, AG), GC), 0.7), ((GC, (UC, AU), GC), 1.0), ((GC, (UC, CA), GC), 1.5), ((GC, (UC, CC), GC), 1.3), ((GC, (UC, CG), GC), 1.5), ((GC, (UC, CU), GC), 1.2), ((GC, (UC, GA), GC), 0.7), ((GC, (UC, GC), GC), 1.0), ((GC, (UC, GG), GC), 0.2), ((GC, (UC, GU), GC), 1.0), ((GC, (UC, UA), GC), 1.5), ((GC, (UC, UC), GC), 1.7), ((GC, (UC, UG), GC), 1.5), ((GC, (UC, UU), GC), 0.3),
    ((GC, (UG, AA), GC), 1.1), ((GC, (UG, AC), GC), 1.0), ((GC, (UG, AG), GC), 0.1), ((GC, (UG, AU), GC), 1.0), ((GC, (UG, CA), GC), 1.5), ((GC, (UG, CC), GC), 1.3), ((GC, (UG, CG), GC), 1.5), ((GC, (UG, CU), GC), 1.2), ((GC, (UG, GA), GC), 0.1), ((GC, (UG, GC), GC), 1.0), ((GC, (UG, GG), GC), -0.4), ((GC, (UG, GU), GC), 1.0), ((GC, (UG, UA), GC), 1.5), ((GC, (UG, UC), GC), 1.2), ((GC, (UG, UG), GC), 1.5), ((GC, (UG, UU), GC), 0.3),
    ((GC, (UU, AA), GC), 1.5), ((GC, (UU, AC), GC), -0.2), ((GC, (UU, AG), GC), 0.9), ((GC, (UU, AU), GC), 0.3), ((GC, (UU, CA), GC), 0.0), ((GC, (UU, CC), GC), -0.1), ((GC, (UU, CG), GC), 0.7), ((GC, (UU, CU), GC), 0.4), ((GC, (UU, GA), GC), 0.9), ((GC, (UU, GC), GC), 0.3), ((GC, (UU, GG), GC), 1.4), ((GC, (UU, GU), GC), 0.3), ((GC, (UU, UA), GC), 0.7), ((GC, (UU, UC), GC), 0.2), ((GC, (UU, UG), GC), 0.7), ((GC, (UU, UU), GC), -0.6),
    // For internal loops between the base pairs "GC" and "GU".
    ((GC, (AA, AA), GU), 1.9), ((GC, (AA, AC), GU), 2.1), ((GC, (AA, AG), GU), 0.8), ((GC, (AA, AU), GU), 2.1), ((GC, (AA, CA), GU), 2.1), ((GC, (AA, CC), GU), 2.1), ((GC, (AA, CG), GU), 2.1), ((GC, (AA, CU), GU), 2.1), ((GC, (AA, GA), GU), 2.4), ((GC, (AA, GC), GU), 2.1), ((GC, (AA, GG), GU), 0.8), ((GC, (AA, GU), GU), 2.1), ((GC, (AA, UA), GU), 2.1), ((GC, (AA, UC), GU), 2.1), ((GC, (AA, UG), GU), 2.1), ((GC, (AA, UU), GU), 2.7),
    ((GC, (AC, AA), GU), 1.5), ((GC, (AC, AC), GU), 1.8), ((GC, (AC, AG), GU), 0.5), ((GC, (AC, AU), GU), 1.8), ((GC, (AC, CA), GU), 1.8), ((GC, (AC, CC), GU), 1.8), ((GC, (AC, CG), GU), 1.8), ((GC, (AC, CU), GU), 1.8), ((GC, (AC, GA), GU), 2.1), ((GC, (AC, GC), GU), 1.8), ((GC, (AC, GG), GU), 0.5), ((GC, (AC, GU), GU), 1.8), ((GC, (AC, UA), GU), 1.8), ((GC, (AC, UC), GU), 1.8), ((GC, (AC, UG), GU), 1.8), ((GC, (AC, UU), GU), 1.8),
    ((GC, (AG, AA), GU), 0.4), ((GC, (AG, AC), GU), 0.7), ((GC, (AG, AG), GU), -0.6), ((GC, (AG, AU), GU), 0.7), ((GC, (AG, CA), GU), 0.7), ((GC, (AG, CC), GU), 1.3), ((GC, (AG, CG), GU), 0.7), ((GC, (AG, CU), GU), 1.3), ((GC, (AG, GA), GU), 1.0), ((GC, (AG, GC), GU), 0.7), ((GC, (AG, GG), GU), 0.7), ((GC, (AG, GU), GU), 0.7), ((GC, (AG, UA), GU), 0.7), ((GC, (AG, UC), GU), 1.3), ((GC, (AG, UG), GU), 0.7), ((GC, (AG, UU), GU), 0.7),
    ((GC, (AU, AA), GU), 1.5), ((GC, (AU, AC), GU), 1.8), ((GC, (AU, AG), GU), 0.5), ((GC, (AU, AU), GU), 1.8), ((GC, (AU, CA), GU), 1.8), ((GC, (AU, CC), GU), 1.8), ((GC, (AU, CG), GU), 1.8), ((GC, (AU, CU), GU), 1.8), ((GC, (AU, GA), GU), 2.1), ((GC, (AU, GC), GU), 1.8), ((GC, (AU, GG), GU), 0.5), ((GC, (AU, GU), GU), 1.8), ((GC, (AU, UA), GU), 1.8), ((GC, (AU, UC), GU), 1.8), ((GC, (AU, UG), GU), 1.8), ((GC, (AU, UU), GU), 1.8),
    ((GC, (CA, AA), GU), 1.5), ((GC, (CA, AC), GU), 1.9), ((GC, (CA, AG), GU), 0.5), ((GC, (CA, AU), GU), 1.9), ((GC, (CA, CA), GU), 1.9), ((GC, (CA, CC), GU), 1.9), ((GC, (CA, CG), GU), 1.9), ((GC, (CA, CU), GU), 1.9), ((GC, (CA, GA), GU), 2.2), ((GC, (CA, GC), GU), 1.9), ((GC, (CA, GG), GU), 0.6), ((GC, (CA, GU), GU), 1.9), ((GC, (CA, UA), GU), 1.9), ((GC, (CA, UC), GU), 1.9), ((GC, (CA, UG), GU), 1.9), ((GC, (CA, UU), GU), 1.9),
    ((GC, (CC, AA), GU), 1.5), ((GC, (CC, AC), GU), 1.9), ((GC, (CC, AG), GU), 1.1), ((GC, (CC, AU), GU), 1.9), ((GC, (CC, CA), GU), 1.9), ((GC, (CC, CC), GU), 1.9), ((GC, (CC, CG), GU), 1.9), ((GC, (CC, CU), GU), 1.9), ((GC, (CC, GA), GU), 2.8), ((GC, (CC, GC), GU), 1.9), ((GC, (CC, GG), GU), 0.6), ((GC, (CC, GU), GU), 1.9), ((GC, (CC, UA), GU), 1.9), ((GC, (CC, UC), GU), 1.9), ((GC, (CC, UG), GU), 1.9), ((GC, (CC, UU), GU), 1.9),
    ((GC, (CG, AA), GU), 1.5), ((GC, (CG, AC), GU), 1.9), ((GC, (CG, AG), GU), 0.5), ((GC, (CG, AU), GU), 1.9), ((GC, (CG, CA), GU), 1.9), ((GC, (CG, CC), GU), 1.9), ((GC, (CG, CG), GU), 1.9), ((GC, (CG, CU), GU), 1.9), ((GC, (CG, GA), GU), 2.2), ((GC, (CG, GC), GU), 1.9), ((GC, (CG, GG), GU), 0.6), ((GC, (CG, GU), GU), 1.9), ((GC, (CG, UA), GU), 1.9), ((GC, (CG, UC), GU), 1.9), ((GC, (CG, UG), GU), 1.9), ((GC, (CG, UU), GU), 1.9),
    ((GC, (CU, AA), GU), 1.6), ((GC, (CU, AC), GU), 1.9), ((GC, (CU, AG), GU), 1.2), ((GC, (CU, AU), GU), 1.9), ((GC, (CU, CA), GU), 1.9), ((GC, (CU, CC), GU), 1.9), ((GC, (CU, CG), GU), 1.9), ((GC, (CU, CU), GU), 1.9), ((GC, (CU, GA), GU), 2.8), ((GC, (CU, GC), GU), 1.9), ((GC, (CU, GG), GU), 0.6), ((GC, (CU, GU), GU), 1.9), ((GC, (CU, UA), GU), 1.9), ((GC, (CU, UC), GU), 1.9), ((GC, (CU, UG), GU), 1.9), ((GC, (CU, UU), GU), 1.9),
    ((GC, (GA, AA), GU), -0.2), ((GC, (GA, AC), GU), 0.1), ((GC, (GA, AG), GU), -1.2), ((GC, (GA, AU), GU), 0.1), ((GC, (GA, CA), GU), 0.1), ((GC, (GA, CC), GU), 0.7), ((GC, (GA, CG), GU), 0.1), ((GC, (GA, CU), GU), 0.7), ((GC, (GA, GA), GU), 0.4), ((GC, (GA, GC), GU), 0.1), ((GC, (GA, GG), GU), 0.1), ((GC, (GA, GU), GU), 0.1), ((GC, (GA, UA), GU), 0.1), ((GC, (GA, UC), GU), 0.7), ((GC, (GA, UG), GU), 0.1), ((GC, (GA, UU), GU), 0.1),
    ((GC, (GC, AA), GU), 1.5), ((GC, (GC, AC), GU), 1.8), ((GC, (GC, AG), GU), 0.5), ((GC, (GC, AU), GU), 1.8), ((GC, (GC, CA), GU), 1.8), ((GC, (GC, CC), GU), 1.8), ((GC, (GC, CG), GU), 1.8), ((GC, (GC, CU), GU), 1.8), ((GC, (GC, GA), GU), 2.1), ((GC, (GC, GC), GU), 1.8), ((GC, (GC, GG), GU), 0.5), ((GC, (GC, GU), GU), 1.8), ((GC, (GC, UA), GU), 1.8), ((GC, (GC, UC), GU), 1.8), ((GC, (GC, UG), GU), 1.8), ((GC, (GC, UU), GU), 1.8),
    ((GC, (GG, AA), GU), 0.1), ((GC, (GG, AC), GU), 0.5), ((GC, (GG, AG), GU), 0.4), ((GC, (GG, AU), GU), 0.5), ((GC, (GG, CA), GU), 0.5), ((GC, (GG, CC), GU), 0.5), ((GC, (GG, CG), GU), 0.5), ((GC, (GG, CU), GU), 0.5), ((GC, (GG, GA), GU), 2.1), ((GC, (GG, GC), GU), 0.5), ((GC, (GG, GG), GU), 1.8), ((GC, (GG, GU), GU), 0.5), ((GC, (GG, UA), GU), 0.5), ((GC, (GG, UC), GU), 0.5), ((GC, (GG, UG), GU), 0.5), ((GC, (GG, UU), GU), 1.8),
    ((GC, (GU, AA), GU), 1.5), ((GC, (GU, AC), GU), 1.8), ((GC, (GU, AG), GU), 0.5), ((GC, (GU, AU), GU), 1.8), ((GC, (GU, CA), GU), 1.8), ((GC, (GU, CC), GU), 1.8), ((GC, (GU, CG), GU), 1.8), ((GC, (GU, CU), GU), 1.8), ((GC, (GU, GA), GU), 2.1), ((GC, (GU, GC), GU), 1.8), ((GC, (GU, GG), GU), 0.5), ((GC, (GU, GU), GU), 1.8), ((GC, (GU, UA), GU), 1.8), ((GC, (GU, UC), GU), 1.8), ((GC, (GU, UG), GU), 1.8), ((GC, (GU, UU), GU), 1.8),
    ((GC, (UA, AA), GU), 1.5), ((GC, (UA, AC), GU), 1.9), ((GC, (UA, AG), GU), 0.5), ((GC, (UA, AU), GU), 1.9), ((GC, (UA, CA), GU), 1.9), ((GC, (UA, CC), GU), 1.9), ((GC, (UA, CG), GU), 1.9), ((GC, (UA, CU), GU), 1.9), ((GC, (UA, GA), GU), 2.2), ((GC, (UA, GC), GU), 1.9), ((GC, (UA, GG), GU), 0.6), ((GC, (UA, GU), GU), 1.9), ((GC, (UA, UA), GU), 1.9), ((GC, (UA, UC), GU), 1.9), ((GC, (UA, UG), GU), 1.9), ((GC, (UA, UU), GU), 1.9),
    ((GC, (UC, AA), GU), 1.5), ((GC, (UC, AC), GU), 1.9), ((GC, (UC, AG), GU), 1.1), ((GC, (UC, AU), GU), 1.9), ((GC, (UC, CA), GU), 1.9), ((GC, (UC, CC), GU), 1.9), ((GC, (UC, CG), GU), 1.9), ((GC, (UC, CU), GU), 1.9), ((GC, (UC, GA), GU), 2.8), ((GC, (UC, GC), GU), 1.9), ((GC, (UC, GG), GU), 0.6), ((GC, (UC, GU), GU), 1.9), ((GC, (UC, UA), GU), 1.9), ((GC, (UC, UC), GU), 1.9), ((GC, (UC, UG), GU), 1.9), ((GC, (UC, UU), GU), 1.9),
    ((GC, (UG, AA), GU), 1.5), ((GC, (UG, AC), GU), 1.9), ((GC, (UG, AG), GU), 0.5), ((GC, (UG, AU), GU), 1.9), ((GC, (UG, CA), GU), 1.9), ((GC, (UG, CC), GU), 1.9), ((GC, (UG, CG), GU), 1.9), ((GC, (UG, CU), GU), 1.9), ((GC, (UG, GA), GU), 2.2), ((GC, (UG, GC), GU), 1.9), ((GC, (UG, GG), GU), 0.6), ((GC, (UG, GU), GU), 1.9), ((GC, (UG, UA), GU), 1.9), ((GC, (UG, UC), GU), 1.9), ((GC, (UG, UG), GU), 1.9), ((GC, (UG, UU), GU), 1.9),
    ((GC, (UU, AA), GU), 1.4), ((GC, (UU, AC), GU), 1.1), ((GC, (UU, AG), GU), -0.2), ((GC, (UU, AU), GU), 1.1), ((GC, (UU, CA), GU), 1.1), ((GC, (UU, CC), GU), 1.1), ((GC, (UU, CG), GU), 1.1), ((GC, (UU, CU), GU), 1.1), ((GC, (UU, GA), GU), 1.4), ((GC, (UU, GC), GU), 1.1), ((GC, (UU, GG), GU), 1.1), ((GC, (UU, GU), GU), 1.1), ((GC, (UU, UA), GU), 1.1), ((GC, (UU, UC), GU), 1.1), ((GC, (UU, UG), GU), 1.1), ((GC, (UU, UU), GU), 1.1),
    // For internal loops between the base pairs "GC" and "UA".
    ((GC, (AA, AA), UA), 2.1), ((GC, (AA, AC), UA), 1.9), ((GC, (AA, AG), UA), 0.9), ((GC, (AA, AU), UA), 1.9), ((GC, (AA, CA), UA), 2.0), ((GC, (AA, CC), UA), 1.9), ((GC, (AA, CG), UA), 2.0), ((GC, (AA, CU), UA), 1.9), ((GC, (AA, GA), UA), 1.6), ((GC, (AA, GC), UA), 1.9), ((GC, (AA, GG), UA), 0.4), ((GC, (AA, GU), UA), 1.9), ((GC, (AA, UA), UA), 2.0), ((GC, (AA, UC), UA), 1.9), ((GC, (AA, UG), UA), 2.0), ((GC, (AA, UU), UA), 1.6),
    ((GC, (AC, AA), UA), 1.8), ((GC, (AC, AC), UA), 1.6), ((GC, (AC, AG), UA), 0.6), ((GC, (AC, AU), UA), 1.6), ((GC, (AC, CA), UA), 1.7), ((GC, (AC, CC), UA), 1.6), ((GC, (AC, CG), UA), 1.7), ((GC, (AC, CU), UA), 1.6), ((GC, (AC, GA), UA), 1.3), ((GC, (AC, GC), UA), 1.6), ((GC, (AC, GG), UA), 0.1), ((GC, (AC, GU), UA), 1.6), ((GC, (AC, UA), UA), 1.7), ((GC, (AC, UC), UA), 1.6), ((GC, (AC, UG), UA), 1.7), ((GC, (AC, UU), UA), 0.7),
    ((GC, (AG, AA), UA), 0.7), ((GC, (AG, AC), UA), 0.5), ((GC, (AG, AG), UA), -0.5), ((GC, (AG, AU), UA), 0.5), ((GC, (AG, CA), UA), 0.6), ((GC, (AG, CC), UA), 1.1), ((GC, (AG, CG), UA), 0.6), ((GC, (AG, CU), UA), 1.1), ((GC, (AG, GA), UA), 0.2), ((GC, (AG, GC), UA), 0.5), ((GC, (AG, GG), UA), 0.3), ((GC, (AG, GU), UA), 0.5), ((GC, (AG, UA), UA), 0.6), ((GC, (AG, UC), UA), 1.1), ((GC, (AG, UG), UA), 0.6), ((GC, (AG, UU), UA), -0.3),
    ((GC, (AU, AA), UA), 1.8), ((GC, (AU, AC), UA), 1.6), ((GC, (AU, AG), UA), 0.6), ((GC, (AU, AU), UA), 1.6), ((GC, (AU, CA), UA), 1.7), ((GC, (AU, CC), UA), 1.6), ((GC, (AU, CG), UA), 1.7), ((GC, (AU, CU), UA), 1.6), ((GC, (AU, GA), UA), 1.3), ((GC, (AU, GC), UA), 1.6), ((GC, (AU, GG), UA), 0.1), ((GC, (AU, GU), UA), 1.6), ((GC, (AU, UA), UA), 1.7), ((GC, (AU, UC), UA), 1.6), ((GC, (AU, UG), UA), 1.7), ((GC, (AU, UU), UA), 0.7),
    ((GC, (CA, AA), UA), 1.9), ((GC, (CA, AC), UA), 1.6), ((GC, (CA, AG), UA), 0.6), ((GC, (CA, AU), UA), 1.6), ((GC, (CA, CA), UA), 1.7), ((GC, (CA, CC), UA), 1.7), ((GC, (CA, CG), UA), 1.7), ((GC, (CA, CU), UA), 1.7), ((GC, (CA, GA), UA), 1.3), ((GC, (CA, GC), UA), 1.6), ((GC, (CA, GG), UA), 0.1), ((GC, (CA, GU), UA), 1.6), ((GC, (CA, UA), UA), 1.7), ((GC, (CA, UC), UA), 1.7), ((GC, (CA, UG), UA), 1.7), ((GC, (CA, UU), UA), 0.8),
    ((GC, (CC, AA), UA), 1.9), ((GC, (CC, AC), UA), 1.6), ((GC, (CC, AG), UA), 1.2), ((GC, (CC, AU), UA), 1.6), ((GC, (CC, CA), UA), 1.7), ((GC, (CC, CC), UA), 1.7), ((GC, (CC, CG), UA), 1.7), ((GC, (CC, CU), UA), 1.7), ((GC, (CC, GA), UA), 1.9), ((GC, (CC, GC), UA), 1.6), ((GC, (CC, GG), UA), 0.1), ((GC, (CC, GU), UA), 1.6), ((GC, (CC, UA), UA), 1.7), ((GC, (CC, UC), UA), 1.7), ((GC, (CC, UG), UA), 1.7), ((GC, (CC, UU), UA), 0.8),
    ((GC, (CG, AA), UA), 1.9), ((GC, (CG, AC), UA), 1.6), ((GC, (CG, AG), UA), 0.6), ((GC, (CG, AU), UA), 1.6), ((GC, (CG, CA), UA), 1.7), ((GC, (CG, CC), UA), 1.7), ((GC, (CG, CG), UA), 1.7), ((GC, (CG, CU), UA), 1.7), ((GC, (CG, GA), UA), 1.3), ((GC, (CG, GC), UA), 1.6), ((GC, (CG, GG), UA), 0.1), ((GC, (CG, GU), UA), 1.6), ((GC, (CG, UA), UA), 1.7), ((GC, (CG, UC), UA), 1.7), ((GC, (CG, UG), UA), 1.7), ((GC, (CG, UU), UA), 0.8),
    ((GC, (CU, AA), UA), 1.9), ((GC, (CU, AC), UA), 1.7), ((GC, (CU, AG), UA), 1.3), ((GC, (CU, AU), UA), 1.7), ((GC, (CU, CA), UA), 1.8), ((GC, (CU, CC), UA), 1.7), ((GC, (CU, CG), UA), 1.8), ((GC, (CU, CU), UA), 1.7), ((GC, (CU, GA), UA), 2.0), ((GC, (CU, GC), UA), 1.7), ((GC, (CU, GG), UA), 0.2), ((GC, (CU, GU), UA), 1.7), ((GC, (CU, UA), UA), 1.8), ((GC, (CU, UC), UA), 1.7), ((GC, (CU, UG), UA), 1.8), ((GC, (CU, UU), UA), 0.8),
    ((GC, (GA, AA), UA), 0.1), ((GC, (GA, AC), UA), -0.1), ((GC, (GA, AG), UA), -1.1), ((GC, (GA, AU), UA), -0.1), ((GC, (GA, CA), UA), 0.0), ((GC, (GA, CC), UA), 0.5), ((GC, (GA, CG), UA), 0.0), ((GC, (GA, CU), UA), 0.5), ((GC, (GA, GA), UA), -0.4), ((GC, (GA, GC), UA), -0.1), ((GC, (GA, GG), UA), -0.3), ((GC, (GA, GU), UA), -0.1), ((GC, (GA, UA), UA), 0.0), ((GC, (GA, UC), UA), 0.5), ((GC, (GA, UG), UA), 0.0), ((GC, (GA, UU), UA), -1.0),
    ((GC, (GC, AA), UA), 1.8), ((GC, (GC, AC), UA), 1.6), ((GC, (GC, AG), UA), 0.6), ((GC, (GC, AU), UA), 1.6), ((GC, (GC, CA), UA), 1.7), ((GC, (GC, CC), UA), 1.6), ((GC, (GC, CG), UA), 1.7), ((GC, (GC, CU), UA), 1.6), ((GC, (GC, GA), UA), 1.3), ((GC, (GC, GC), UA), 1.6), ((GC, (GC, GG), UA), 0.1), ((GC, (GC, GU), UA), 1.6), ((GC, (GC, UA), UA), 1.7), ((GC, (GC, UC), UA), 1.6), ((GC, (GC, UG), UA), 1.7), ((GC, (GC, UU), UA), 0.7),
    ((GC, (GG, AA), UA), 0.5), ((GC, (GG, AC), UA), 0.2), ((GC, (GG, AG), UA), 0.5), ((GC, (GG, AU), UA), 0.2), ((GC, (GG, CA), UA), 0.3), ((GC, (GG, CC), UA), 0.3), ((GC, (GG, CG), UA), 0.3), ((GC, (GG, CU), UA), 0.3), ((GC, (GG, GA), UA), 1.2), ((GC, (GG, GC), UA), 0.2), ((GC, (GG, GG), UA), 1.3), ((GC, (GG, GU), UA), 0.2), ((GC, (GG, UA), UA), 0.3), ((GC, (GG, UC), UA), 0.3), ((GC, (GG, UG), UA), 0.3), ((GC, (GG, UU), UA), 0.7),
    ((GC, (GU, AA), UA), 1.8), ((GC, (GU, AC), UA), 1.6), ((GC, (GU, AG), UA), 0.6), ((GC, (GU, AU), UA), 1.6), ((GC, (GU, CA), UA), 1.7), ((GC, (GU, CC), UA), 1.6), ((GC, (GU, CG), UA), 1.7), ((GC, (GU, CU), UA), 1.6), ((GC, (GU, GA), UA), 1.3), ((GC, (GU, GC), UA), 1.6), ((GC, (GU, GG), UA), 0.1), ((GC, (GU, GU), UA), 1.6), ((GC, (GU, UA), UA), 1.7), ((GC, (GU, UC), UA), 1.6), ((GC, (GU, UG), UA), 1.7), ((GC, (GU, UU), UA), 0.7),
    ((GC, (UA, AA), UA), 1.9), ((GC, (UA, AC), UA), 1.6), ((GC, (UA, AG), UA), 0.6), ((GC, (UA, AU), UA), 1.6), ((GC, (UA, CA), UA), 1.7), ((GC, (UA, CC), UA), 1.7), ((GC, (UA, CG), UA), 1.7), ((GC, (UA, CU), UA), 1.7), ((GC, (UA, GA), UA), 1.3), ((GC, (UA, GC), UA), 1.6), ((GC, (UA, GG), UA), 0.1), ((GC, (UA, GU), UA), 1.6), ((GC, (UA, UA), UA), 1.7), ((GC, (UA, UC), UA), 1.7), ((GC, (UA, UG), UA), 1.7), ((GC, (UA, UU), UA), 0.8),
    ((GC, (UC, AA), UA), 1.9), ((GC, (UC, AC), UA), 1.6), ((GC, (UC, AG), UA), 1.2), ((GC, (UC, AU), UA), 1.6), ((GC, (UC, CA), UA), 1.7), ((GC, (UC, CC), UA), 1.7), ((GC, (UC, CG), UA), 1.7), ((GC, (UC, CU), UA), 1.7), ((GC, (UC, GA), UA), 1.9), ((GC, (UC, GC), UA), 1.6), ((GC, (UC, GG), UA), 0.1), ((GC, (UC, GU), UA), 1.6), ((GC, (UC, UA), UA), 1.7), ((GC, (UC, UC), UA), 1.7), ((GC, (UC, UG), UA), 1.7), ((GC, (UC, UU), UA), 0.8),
    ((GC, (UG, AA), UA), 1.9), ((GC, (UG, AC), UA), 1.6), ((GC, (UG, AG), UA), 0.6), ((GC, (UG, AU), UA), 1.6), ((GC, (UG, CA), UA), 1.7), ((GC, (UG, CC), UA), 1.7), ((GC, (UG, CG), UA), 1.7), ((GC, (UG, CU), UA), 1.7), ((GC, (UG, GA), UA), 1.3), ((GC, (UG, GC), UA), 1.6), ((GC, (UG, GG), UA), 0.1), ((GC, (UG, GU), UA), 1.6), ((GC, (UG, UA), UA), 1.7), ((GC, (UG, UC), UA), 1.7), ((GC, (UG, UG), UA), 1.7), ((GC, (UG, UU), UA), 0.8),
    ((GC, (UU, AA), UA), 1.7), ((GC, (UU, AC), UA), 0.9), ((GC, (UU, AG), UA), -0.1), ((GC, (UU, AU), UA), 0.9), ((GC, (UU, CA), UA), 1.0), ((GC, (UU, CC), UA), 0.9), ((GC, (UU, CG), UA), 1.0), ((GC, (UU, CU), UA), 0.9), ((GC, (UU, GA), UA), 0.6), ((GC, (UU, GC), UA), 0.9), ((GC, (UU, GG), UA), 0.7), ((GC, (UU, GU), UA), 0.9), ((GC, (UU, UA), UA), 1.0), ((GC, (UU, UC), UA), 0.9), ((GC, (UU, UG), UA), 1.0), ((GC, (UU, UU), UA), 0.0),
    // For internal loops between the base pairs "GC" and "UG".
    ((GC, (AA, AA), UG), 2.8), ((GC, (AA, AC), UG), 2.4), ((GC, (AA, AG), UG), 1.6), ((GC, (AA, AU), UG), 2.4), ((GC, (AA, CA), UG), 2.4), ((GC, (AA, CC), UG), 2.4), ((GC, (AA, CG), UG), 2.4), ((GC, (AA, CU), UG), 2.4), ((GC, (AA, GA), UG), 2.0), ((GC, (AA, GC), UG), 2.4), ((GC, (AA, GG), UG), 1.1), ((GC, (AA, GU), UG), 2.4), ((GC, (AA, UA), UG), 2.4), ((GC, (AA, UC), UG), 2.4), ((GC, (AA, UG), UG), 2.4), ((GC, (AA, UU), UG), 3.0),
    ((GC, (AC, AA), UG), 2.5), ((GC, (AC, AC), UG), 2.1), ((GC, (AC, AG), UG), 1.3), ((GC, (AC, AU), UG), 2.1), ((GC, (AC, CA), UG), 2.1), ((GC, (AC, CC), UG), 2.1), ((GC, (AC, CG), UG), 2.1), ((GC, (AC, CU), UG), 2.1), ((GC, (AC, GA), UG), 1.7), ((GC, (AC, GC), UG), 2.1), ((GC, (AC, GG), UG), 0.8), ((GC, (AC, GU), UG), 2.1), ((GC, (AC, UA), UG), 2.1), ((GC, (AC, UC), UG), 2.1), ((GC, (AC, UG), UG), 2.1), ((GC, (AC, UU), UG), 2.1),
    ((GC, (AG, AA), UG), 1.4), ((GC, (AG, AC), UG), 1.0), ((GC, (AG, AG), UG), 0.2), ((GC, (AG, AU), UG), 1.0), ((GC, (AG, CA), UG), 1.0), ((GC, (AG, CC), UG), 1.6), ((GC, (AG, CG), UG), 1.0), ((GC, (AG, CU), UG), 1.6), ((GC, (AG, GA), UG), 0.6), ((GC, (AG, GC), UG), 1.0), ((GC, (AG, GG), UG), 1.0), ((GC, (AG, GU), UG), 1.0), ((GC, (AG, UA), UG), 1.0), ((GC, (AG, UC), UG), 1.6), ((GC, (AG, UG), UG), 1.0), ((GC, (AG, UU), UG), 1.0),
    ((GC, (AU, AA), UG), 2.5), ((GC, (AU, AC), UG), 2.1), ((GC, (AU, AG), UG), 1.3), ((GC, (AU, AU), UG), 2.1), ((GC, (AU, CA), UG), 2.1), ((GC, (AU, CC), UG), 2.1), ((GC, (AU, CG), UG), 2.1), ((GC, (AU, CU), UG), 2.1), ((GC, (AU, GA), UG), 1.7), ((GC, (AU, GC), UG), 2.1), ((GC, (AU, GG), UG), 0.8), ((GC, (AU, GU), UG), 2.1), ((GC, (AU, UA), UG), 2.1), ((GC, (AU, UC), UG), 2.1), ((GC, (AU, UG), UG), 2.1), ((GC, (AU, UU), UG), 2.1),
    ((GC, (CA, AA), UG), 2.5), ((GC, (CA, AC), UG), 2.2), ((GC, (CA, AG), UG), 1.4), ((GC, (CA, AU), UG), 2.2), ((GC, (CA, CA), UG), 2.2), ((GC, (CA, CC), UG), 2.2), ((GC, (CA, CG), UG), 2.2), ((GC, (CA, CU), UG), 2.2), ((GC, (CA, GA), UG), 1.8), ((GC, (CA, GC), UG), 2.2), ((GC, (CA, GG), UG), 0.9), ((GC, (CA, GU), UG), 2.2), ((GC, (CA, UA), UG), 2.2), ((GC, (CA, UC), UG), 2.2), ((GC, (CA, UG), UG), 2.2), ((GC, (CA, UU), UG), 2.2),
    ((GC, (CC, AA), UG), 2.5), ((GC, (CC, AC), UG), 2.2), ((GC, (CC, AG), UG), 2.0), ((GC, (CC, AU), UG), 2.2), ((GC, (CC, CA), UG), 2.2), ((GC, (CC, CC), UG), 2.2), ((GC, (CC, CG), UG), 2.2), ((GC, (CC, CU), UG), 2.2), ((GC, (CC, GA), UG), 2.4), ((GC, (CC, GC), UG), 2.2), ((GC, (CC, GG), UG), 0.9), ((GC, (CC, GU), UG), 2.2), ((GC, (CC, UA), UG), 2.2), ((GC, (CC, UC), UG), 2.2), ((GC, (CC, UG), UG), 2.2), ((GC, (CC, UU), UG), 2.2),
    ((GC, (CG, AA), UG), 2.5), ((GC, (CG, AC), UG), 2.2), ((GC, (CG, AG), UG), 1.4), ((GC, (CG, AU), UG), 2.2), ((GC, (CG, CA), UG), 2.2), ((GC, (CG, CC), UG), 2.2), ((GC, (CG, CG), UG), 2.2), ((GC, (CG, CU), UG), 2.2), ((GC, (CG, GA), UG), 1.8), ((GC, (CG, GC), UG), 2.2), ((GC, (CG, GG), UG), 0.9), ((GC, (CG, GU), UG), 2.2), ((GC, (CG, UA), UG), 2.2), ((GC, (CG, UC), UG), 2.2), ((GC, (CG, UG), UG), 2.2), ((GC, (CG, UU), UG), 2.2),
    ((GC, (CU, AA), UG), 2.6), ((GC, (CU, AC), UG), 2.2), ((GC, (CU, AG), UG), 2.0), ((GC, (CU, AU), UG), 2.2), ((GC, (CU, CA), UG), 2.2), ((GC, (CU, CC), UG), 2.2), ((GC, (CU, CG), UG), 2.2), ((GC, (CU, CU), UG), 2.2), ((GC, (CU, GA), UG), 2.4), ((GC, (CU, GC), UG), 2.2), ((GC, (CU, GG), UG), 0.9), ((GC, (CU, GU), UG), 2.2), ((GC, (CU, UA), UG), 2.2), ((GC, (CU, UC), UG), 2.2), ((GC, (CU, UG), UG), 2.2), ((GC, (CU, UU), UG), 2.2),
    ((GC, (GA, AA), UG), 0.7), ((GC, (GA, AC), UG), 0.4), ((GC, (GA, AG), UG), -0.4), ((GC, (GA, AU), UG), 0.4), ((GC, (GA, CA), UG), 0.4), ((GC, (GA, CC), UG), 1.0), ((GC, (GA, CG), UG), 0.4), ((GC, (GA, CU), UG), 1.0), ((GC, (GA, GA), UG), 0.0), ((GC, (GA, GC), UG), 0.4), ((GC, (GA, GG), UG), 0.4), ((GC, (GA, GU), UG), 0.4), ((GC, (GA, UA), UG), 0.4), ((GC, (GA, UC), UG), 1.0), ((GC, (GA, UG), UG), 0.4), ((GC, (GA, UU), UG), 0.4),
    ((GC, (GC, AA), UG), 2.5), ((GC, (GC, AC), UG), 2.1), ((GC, (GC, AG), UG), 1.3), ((GC, (GC, AU), UG), 2.1), ((GC, (GC, CA), UG), 2.1), ((GC, (GC, CC), UG), 2.1), ((GC, (GC, CG), UG), 2.1), ((GC, (GC, CU), UG), 2.1), ((GC, (GC, GA), UG), 1.7), ((GC, (GC, GC), UG), 2.1), ((GC, (GC, GG), UG), 0.8), ((GC, (GC, GU), UG), 2.1), ((GC, (GC, UA), UG), 2.1), ((GC, (GC, UC), UG), 2.1), ((GC, (GC, UG), UG), 2.1), ((GC, (GC, UU), UG), 2.1),
    ((GC, (GG, AA), UG), 1.1), ((GC, (GG, AC), UG), 0.8), ((GC, (GG, AG), UG), 1.3), ((GC, (GG, AU), UG), 0.8), ((GC, (GG, CA), UG), 0.8), ((GC, (GG, CC), UG), 0.8), ((GC, (GG, CG), UG), 0.8), ((GC, (GG, CU), UG), 0.8), ((GC, (GG, GA), UG), 1.7), ((GC, (GG, GC), UG), 0.8), ((GC, (GG, GG), UG), 2.1), ((GC, (GG, GU), UG), 0.8), ((GC, (GG, UA), UG), 0.8), ((GC, (GG, UC), UG), 0.8), ((GC, (GG, UG), UG), 0.8), ((GC, (GG, UU), UG), 2.1),
    ((GC, (GU, AA), UG), 2.5), ((GC, (GU, AC), UG), 2.1), ((GC, (GU, AG), UG), 1.3), ((GC, (GU, AU), UG), 2.1), ((GC, (GU, CA), UG), 2.1), ((GC, (GU, CC), UG), 2.1), ((GC, (GU, CG), UG), 2.1), ((GC, (GU, CU), UG), 2.1), ((GC, (GU, GA), UG), 1.7), ((GC, (GU, GC), UG), 2.1), ((GC, (GU, GG), UG), 0.8), ((GC, (GU, GU), UG), 2.1), ((GC, (GU, UA), UG), 2.1), ((GC, (GU, UC), UG), 2.1), ((GC, (GU, UG), UG), 2.1), ((GC, (GU, UU), UG), 2.1),
    ((GC, (UA, AA), UG), 2.5), ((GC, (UA, AC), UG), 2.2), ((GC, (UA, AG), UG), 1.4), ((GC, (UA, AU), UG), 2.2), ((GC, (UA, CA), UG), 2.2), ((GC, (UA, CC), UG), 2.2), ((GC, (UA, CG), UG), 2.2), ((GC, (UA, CU), UG), 2.2), ((GC, (UA, GA), UG), 1.8), ((GC, (UA, GC), UG), 2.2), ((GC, (UA, GG), UG), 0.9), ((GC, (UA, GU), UG), 2.2), ((GC, (UA, UA), UG), 2.2), ((GC, (UA, UC), UG), 2.2), ((GC, (UA, UG), UG), 2.2), ((GC, (UA, UU), UG), 2.2),
    ((GC, (UC, AA), UG), 2.5), ((GC, (UC, AC), UG), 2.2), ((GC, (UC, AG), UG), 2.0), ((GC, (UC, AU), UG), 2.2), ((GC, (UC, CA), UG), 2.2), ((GC, (UC, CC), UG), 2.2), ((GC, (UC, CG), UG), 2.2), ((GC, (UC, CU), UG), 2.2), ((GC, (UC, GA), UG), 2.4), ((GC, (UC, GC), UG), 2.2), ((GC, (UC, GG), UG), 0.9), ((GC, (UC, GU), UG), 2.2), ((GC, (UC, UA), UG), 2.2), ((GC, (UC, UC), UG), 2.2), ((GC, (UC, UG), UG), 2.2), ((GC, (UC, UU), UG), 2.2),
    ((GC, (UG, AA), UG), 2.5), ((GC, (UG, AC), UG), 2.2), ((GC, (UG, AG), UG), 1.4), ((GC, (UG, AU), UG), 2.2), ((GC, (UG, CA), UG), 2.2), ((GC, (UG, CC), UG), 2.2), ((GC, (UG, CG), UG), 2.2), ((GC, (UG, CU), UG), 2.2), ((GC, (UG, GA), UG), 1.8), ((GC, (UG, GC), UG), 2.2), ((GC, (UG, GG), UG), 0.9), ((GC, (UG, GU), UG), 2.2), ((GC, (UG, UA), UG), 2.2), ((GC, (UG, UC), UG), 2.2), ((GC, (UG, UG), UG), 2.2), ((GC, (UG, UU), UG), 2.2),
    ((GC, (UU, AA), UG), 2.4), ((GC, (UU, AC), UG), 1.4), ((GC, (UU, AG), UG), 0.6), ((GC, (UU, AU), UG), 1.4), ((GC, (UU, CA), UG), 1.4), ((GC, (UU, CC), UG), 1.4), ((GC, (UU, CG), UG), 1.4), ((GC, (UU, CU), UG), 1.4), ((GC, (UU, GA), UG), 1.0), ((GC, (UU, GC), UG), 1.4), ((GC, (UU, GG), UG), 1.4), ((GC, (UU, GU), UG), 1.4), ((GC, (UU, UA), UG), 1.4), ((GC, (UU, UC), UG), 1.4), ((GC, (UU, UG), UG), 1.4), ((GC, (UU, UU), UG), 1.4),
    // For internal loops between the base pairs "GU" and "AU".
    ((GU, (AA, AA), AU), 3.4), ((GU, (AA, AC), AU), 3.0), ((GU, (AA, AG), AU), 2.4), ((GU, (AA, AU), AU), 3.0), ((GU, (AA, CA), AU), 3.4), ((GU, (AA, CC), AU), 3.4), ((GU, (AA, CG), AU), 3.4), ((GU, (AA, CU), AU), 3.4), ((GU, (AA, GA), AU), 2.5), ((GU, (AA, GC), AU), 3.0), ((GU, (AA, GG), AU), 1.9), ((GU, (AA, GU), AU), 3.0), ((GU, (AA, UA), AU), 3.4), ((GU, (AA, UC), AU), 3.1), ((GU, (AA, UG), AU), 3.4), ((GU, (AA, UU), AU), 3.2),
    ((GU, (AC, AA), AU), 3.1), ((GU, (AC, AC), AU), 2.6), ((GU, (AC, AG), AU), 2.0), ((GU, (AC, AU), AU), 2.6), ((GU, (AC, CA), AU), 3.1), ((GU, (AC, CC), AU), 3.1), ((GU, (AC, CG), AU), 3.1), ((GU, (AC, CU), AU), 3.1), ((GU, (AC, GA), AU), 2.1), ((GU, (AC, GC), AU), 2.6), ((GU, (AC, GG), AU), 1.5), ((GU, (AC, GU), AU), 2.6), ((GU, (AC, UA), AU), 3.1), ((GU, (AC, UC), AU), 2.8), ((GU, (AC, UG), AU), 3.1), ((GU, (AC, UU), AU), 2.2),
    ((GU, (AG, AA), AU), 2.7), ((GU, (AG, AC), AU), 2.2), ((GU, (AG, AG), AU), 1.6), ((GU, (AG, AU), AU), 2.2), ((GU, (AG, CA), AU), 2.7), ((GU, (AG, CC), AU), 3.3), ((GU, (AG, CG), AU), 2.7), ((GU, (AG, CU), AU), 3.3), ((GU, (AG, GA), AU), 1.7), ((GU, (AG, GC), AU), 2.2), ((GU, (AG, GG), AU), 2.4), ((GU, (AG, GU), AU), 2.2), ((GU, (AG, UA), AU), 2.7), ((GU, (AG, UC), AU), 3.0), ((GU, (AG, UG), AU), 2.7), ((GU, (AG, UU), AU), 1.8),
    ((GU, (AU, AA), AU), 3.1), ((GU, (AU, AC), AU), 2.6), ((GU, (AU, AG), AU), 2.0), ((GU, (AU, AU), AU), 2.6), ((GU, (AU, CA), AU), 3.1), ((GU, (AU, CC), AU), 3.1), ((GU, (AU, CG), AU), 3.1), ((GU, (AU, CU), AU), 3.1), ((GU, (AU, GA), AU), 2.1), ((GU, (AU, GC), AU), 2.6), ((GU, (AU, GG), AU), 1.5), ((GU, (AU, GU), AU), 2.6), ((GU, (AU, UA), AU), 3.1), ((GU, (AU, UC), AU), 2.8), ((GU, (AU, UG), AU), 3.1), ((GU, (AU, UU), AU), 2.2),
    ((GU, (CA, AA), AU), 3.1), ((GU, (CA, AC), AU), 2.6), ((GU, (CA, AG), AU), 2.0), ((GU, (CA, AU), AU), 2.6), ((GU, (CA, CA), AU), 3.1), ((GU, (CA, CC), AU), 3.1), ((GU, (CA, CG), AU), 3.1), ((GU, (CA, CU), AU), 3.1), ((GU, (CA, GA), AU), 2.1), ((GU, (CA, GC), AU), 2.6), ((GU, (CA, GG), AU), 1.5), ((GU, (CA, GU), AU), 2.6), ((GU, (CA, UA), AU), 3.1), ((GU, (CA, UC), AU), 2.8), ((GU, (CA, UG), AU), 3.1), ((GU, (CA, UU), AU), 2.2),
    ((GU, (CC, AA), AU), 3.1), ((GU, (CC, AC), AU), 2.6), ((GU, (CC, AG), AU), 2.6), ((GU, (CC, AU), AU), 2.6), ((GU, (CC, CA), AU), 3.1), ((GU, (CC, CC), AU), 3.1), ((GU, (CC, CG), AU), 3.1), ((GU, (CC, CU), AU), 3.1), ((GU, (CC, GA), AU), 2.7), ((GU, (CC, GC), AU), 2.6), ((GU, (CC, GG), AU), 1.5), ((GU, (CC, GU), AU), 2.6), ((GU, (CC, UA), AU), 3.1), ((GU, (CC, UC), AU), 2.8), ((GU, (CC, UG), AU), 3.1), ((GU, (CC, UU), AU), 2.2),
    ((GU, (CG, AA), AU), 3.1), ((GU, (CG, AC), AU), 2.6), ((GU, (CG, AG), AU), 2.0), ((GU, (CG, AU), AU), 2.6), ((GU, (CG, CA), AU), 3.1), ((GU, (CG, CC), AU), 3.1), ((GU, (CG, CG), AU), 3.1), ((GU, (CG, CU), AU), 3.1), ((GU, (CG, GA), AU), 2.1), ((GU, (CG, GC), AU), 2.6), ((GU, (CG, GG), AU), 1.5), ((GU, (CG, GU), AU), 2.6), ((GU, (CG, UA), AU), 3.1), ((GU, (CG, UC), AU), 2.8), ((GU, (CG, UG), AU), 3.1), ((GU, (CG, UU), AU), 2.2),
    ((GU, (CU, AA), AU), 3.1), ((GU, (CU, AC), AU), 2.6), ((GU, (CU, AG), AU), 2.6), ((GU, (CU, AU), AU), 2.6), ((GU, (CU, CA), AU), 3.1), ((GU, (CU, CC), AU), 3.1), ((GU, (CU, CG), AU), 3.1), ((GU, (CU, CU), AU), 3.1), ((GU, (CU, GA), AU), 2.7), ((GU, (CU, GC), AU), 2.6), ((GU, (CU, GG), AU), 1.5), ((GU, (CU, GU), AU), 2.6), ((GU, (CU, UA), AU), 3.1), ((GU, (CU, UC), AU), 2.8), ((GU, (CU, UG), AU), 3.1), ((GU, (CU, UU), AU), 2.2),
    ((GU, (GA, AA), AU), 2.3), ((GU, (GA, AC), AU), 1.8), ((GU, (GA, AG), AU), 1.2), ((GU, (GA, AU), AU), 1.8), ((GU, (GA, CA), AU), 2.3), ((GU, (GA, CC), AU), 2.9), ((GU, (GA, CG), AU), 2.3), ((GU, (GA, CU), AU), 2.9), ((GU, (GA, GA), AU), 1.3), ((GU, (GA, GC), AU), 1.8), ((GU, (GA, GG), AU), 2.0), ((GU, (GA, GU), AU), 1.8), ((GU, (GA, UA), AU), 2.3), ((GU, (GA, UC), AU), 2.6), ((GU, (GA, UG), AU), 2.3), ((GU, (GA, UU), AU), 1.4),
    ((GU, (GC, AA), AU), 3.1), ((GU, (GC, AC), AU), 2.6), ((GU, (GC, AG), AU), 2.0), ((GU, (GC, AU), AU), 2.6), ((GU, (GC, CA), AU), 3.1), ((GU, (GC, CC), AU), 3.1), ((GU, (GC, CG), AU), 3.1), ((GU, (GC, CU), AU), 3.1), ((GU, (GC, GA), AU), 2.1), ((GU, (GC, GC), AU), 2.6), ((GU, (GC, GG), AU), 1.5), ((GU, (GC, GU), AU), 2.6), ((GU, (GC, UA), AU), 3.1), ((GU, (GC, UC), AU), 2.8), ((GU, (GC, UG), AU), 3.1), ((GU, (GC, UU), AU), 2.2),
    ((GU, (GG, AA), AU), 1.8), ((GU, (GG, AC), AU), 1.3), ((GU, (GG, AG), AU), 2.0), ((GU, (GG, AU), AU), 1.3), ((GU, (GG, CA), AU), 1.8), ((GU, (GG, CC), AU), 1.8), ((GU, (GG, CG), AU), 1.8), ((GU, (GG, CU), AU), 1.8), ((GU, (GG, GA), AU), 2.1), ((GU, (GG, GC), AU), 1.3), ((GU, (GG, GG), AU), 2.8), ((GU, (GG, GU), AU), 1.3), ((GU, (GG, UA), AU), 1.8), ((GU, (GG, UC), AU), 1.5), ((GU, (GG, UG), AU), 1.8), ((GU, (GG, UU), AU), 2.2),
    ((GU, (GU, AA), AU), 3.1), ((GU, (GU, AC), AU), 2.6), ((GU, (GU, AG), AU), 2.0), ((GU, (GU, AU), AU), 2.6), ((GU, (GU, CA), AU), 3.1), ((GU, (GU, CC), AU), 3.1), ((GU, (GU, CG), AU), 3.1), ((GU, (GU, CU), AU), 3.1), ((GU, (GU, GA), AU), 2.1), ((GU, (GU, GC), AU), 2.6), ((GU, (GU, GG), AU), 1.5), ((GU, (GU, GU), AU), 2.6), ((GU, (GU, UA), AU), 3.1), ((GU, (GU, UC), AU), 2.8), ((GU, (GU, UG), AU), 3.1), ((GU, (GU, UU), AU), 2.2),
    ((GU, (UA, AA), AU), 3.1), ((GU, (UA, AC), AU), 2.6), ((GU, (UA, AG), AU), 2.0), ((GU, (UA, AU), AU), 2.6), ((GU, (UA, CA), AU), 3.1), ((GU, (UA, CC), AU), 3.1), ((GU, (UA, CG), AU), 3.1), ((GU, (UA, CU), AU), 3.1), ((GU, (UA, GA), AU), 2.1), ((GU, (UA, GC), AU), 2.6), ((GU, (UA, GG), AU), 1.5), ((GU, (UA, GU), AU), 2.6), ((GU, (UA, UA), AU), 3.1), ((GU, (UA, UC), AU), 2.8), ((GU, (UA, UG), AU), 3.1), ((GU, (UA, UU), AU), 2.2),
    ((GU, (UC, AA), AU), 3.1), ((GU, (UC, AC), AU), 2.6), ((GU, (UC, AG), AU), 2.6), ((GU, (UC, AU), AU), 2.6), ((GU, (UC, CA), AU), 3.1), ((GU, (UC, CC), AU), 3.1), ((GU, (UC, CG), AU), 3.1), ((GU, (UC, CU), AU), 3.1), ((GU, (UC, GA), AU), 2.7), ((GU, (UC, GC), AU), 2.6), ((GU, (UC, GG), AU), 1.5), ((GU, (UC, GU), AU), 2.6), ((GU, (UC, UA), AU), 3.1), ((GU, (UC, UC), AU), 2.8), ((GU, (UC, UG), AU), 3.1), ((GU, (UC, UU), AU), 2.2),
    ((GU, (UG, AA), AU), 3.1), ((GU, (UG, AC), AU), 2.6), ((GU, (UG, AG), AU), 2.0), ((GU, (UG, AU), AU), 2.6), ((GU, (UG, CA), AU), 3.1), ((GU, (UG, CC), AU), 3.1), ((GU, (UG, CG), AU), 3.1), ((GU, (UG, CU), AU), 3.1), ((GU, (UG, GA), AU), 2.1), ((GU, (UG, GC), AU), 2.6), ((GU, (UG, GG), AU), 1.5), ((GU, (UG, GU), AU), 2.6), ((GU, (UG, UA), AU), 3.1), ((GU, (UG, UC), AU), 2.8), ((GU, (UG, UG), AU), 3.1), ((GU, (UG, UU), AU), 2.2),
    ((GU, (UU, AA), AU), 3.7), ((GU, (UU, AC), AU), 2.6), ((GU, (UU, AG), AU), 2.0), ((GU, (UU, AU), AU), 2.6), ((GU, (UU, CA), AU), 3.1), ((GU, (UU, CC), AU), 3.1), ((GU, (UU, CG), AU), 3.1), ((GU, (UU, CU), AU), 3.1), ((GU, (UU, GA), AU), 2.1), ((GU, (UU, GC), AU), 2.6), ((GU, (UU, GG), AU), 2.8), ((GU, (UU, GU), AU), 2.6), ((GU, (UU, UA), AU), 3.1), ((GU, (UU, UC), AU), 2.8), ((GU, (UU, UG), AU), 3.1), ((GU, (UU, UU), AU), 2.2),
    // For internal loops between the base pairs "GU" and "CG".
    ((GU, (AA, AA), CG), 2.8), ((GU, (AA, AC), CG), 2.5), ((GU, (AA, AG), CG), 0.7), ((GU, (AA, AU), CG), 2.5), ((GU, (AA, CA), CG), 2.5), ((GU, (AA, CC), CG), 2.5), ((GU, (AA, CG), CG), 2.5), ((GU, (AA, CU), CG), 2.5), ((GU, (AA, GA), CG), 1.4), ((GU, (AA, GC), CG), 2.5), ((GU, (AA, GG), CG), 1.1), ((GU, (AA, GU), CG), 2.5), ((GU, (AA, UA), CG), 2.5), ((GU, (AA, UC), CG), 2.6), ((GU, (AA, UG), CG), 2.5), ((GU, (AA, UU), CG), 2.4),
    ((GU, (AC, AA), CG), 2.4), ((GU, (AC, AC), CG), 2.2), ((GU, (AC, AG), CG), 0.4), ((GU, (AC, AU), CG), 2.2), ((GU, (AC, CA), CG), 2.1), ((GU, (AC, CC), CG), 2.2), ((GU, (AC, CG), CG), 2.1), ((GU, (AC, CU), CG), 2.2), ((GU, (AC, GA), CG), 1.0), ((GU, (AC, GC), CG), 2.2), ((GU, (AC, GG), CG), 0.8), ((GU, (AC, GU), CG), 2.2), ((GU, (AC, UA), CG), 2.1), ((GU, (AC, UC), CG), 2.2), ((GU, (AC, UG), CG), 2.1), ((GU, (AC, UU), CG), 1.4),
    ((GU, (AG, AA), CG), 2.0), ((GU, (AG, AC), CG), 1.8), ((GU, (AG, AG), CG), 0.0), ((GU, (AG, AU), CG), 1.8), ((GU, (AG, CA), CG), 1.7), ((GU, (AG, CC), CG), 2.4), ((GU, (AG, CG), CG), 1.7), ((GU, (AG, CU), CG), 2.4), ((GU, (AG, GA), CG), 0.6), ((GU, (AG, GC), CG), 1.8), ((GU, (AG, GG), CG), 1.7), ((GU, (AG, GU), CG), 1.8), ((GU, (AG, UA), CG), 1.7), ((GU, (AG, UC), CG), 2.4), ((GU, (AG, UG), CG), 1.7), ((GU, (AG, UU), CG), 1.0),
    ((GU, (AU, AA), CG), 2.4), ((GU, (AU, AC), CG), 2.2), ((GU, (AU, AG), CG), 0.4), ((GU, (AU, AU), CG), 2.2), ((GU, (AU, CA), CG), 2.1), ((GU, (AU, CC), CG), 2.2), ((GU, (AU, CG), CG), 2.1), ((GU, (AU, CU), CG), 2.2), ((GU, (AU, GA), CG), 1.0), ((GU, (AU, GC), CG), 2.2), ((GU, (AU, GG), CG), 0.8), ((GU, (AU, GU), CG), 2.2), ((GU, (AU, UA), CG), 2.1), ((GU, (AU, UC), CG), 2.2), ((GU, (AU, UG), CG), 2.1), ((GU, (AU, UU), CG), 1.4),
    ((GU, (CA, AA), CG), 2.4), ((GU, (CA, AC), CG), 2.2), ((GU, (CA, AG), CG), 0.4), ((GU, (CA, AU), CG), 2.2), ((GU, (CA, CA), CG), 2.1), ((GU, (CA, CC), CG), 2.2), ((GU, (CA, CG), CG), 2.1), ((GU, (CA, CU), CG), 2.2), ((GU, (CA, GA), CG), 1.0), ((GU, (CA, GC), CG), 2.2), ((GU, (CA, GG), CG), 0.8), ((GU, (CA, GU), CG), 2.2), ((GU, (CA, UA), CG), 2.1), ((GU, (CA, UC), CG), 2.2), ((GU, (CA, UG), CG), 2.1), ((GU, (CA, UU), CG), 1.4),
    ((GU, (CC, AA), CG), 2.4), ((GU, (CC, AC), CG), 2.2), ((GU, (CC, AG), CG), 1.0), ((GU, (CC, AU), CG), 2.2), ((GU, (CC, CA), CG), 2.1), ((GU, (CC, CC), CG), 2.2), ((GU, (CC, CG), CG), 2.1), ((GU, (CC, CU), CG), 2.2), ((GU, (CC, GA), CG), 1.6), ((GU, (CC, GC), CG), 2.2), ((GU, (CC, GG), CG), 0.8), ((GU, (CC, GU), CG), 2.2), ((GU, (CC, UA), CG), 2.1), ((GU, (CC, UC), CG), 2.2), ((GU, (CC, UG), CG), 2.1), ((GU, (CC, UU), CG), 1.4),
    ((GU, (CG, AA), CG), 2.4), ((GU, (CG, AC), CG), 2.2), ((GU, (CG, AG), CG), 0.4), ((GU, (CG, AU), CG), 2.2), ((GU, (CG, CA), CG), 2.1), ((GU, (CG, CC), CG), 2.2), ((GU, (CG, CG), CG), 2.1), ((GU, (CG, CU), CG), 2.2), ((GU, (CG, GA), CG), 1.0), ((GU, (CG, GC), CG), 2.2), ((GU, (CG, GG), CG), 0.8), ((GU, (CG, GU), CG), 2.2), ((GU, (CG, UA), CG), 2.1), ((GU, (CG, UC), CG), 2.2), ((GU, (CG, UG), CG), 2.1), ((GU, (CG, UU), CG), 1.4),
    ((GU, (CU, AA), CG), 2.4), ((GU, (CU, AC), CG), 2.2), ((GU, (CU, AG), CG), 1.0), ((GU, (CU, AU), CG), 2.2), ((GU, (CU, CA), CG), 2.1), ((GU, (CU, CC), CG), 2.2), ((GU, (CU, CG), CG), 2.1), ((GU, (CU, CU), CG), 2.2), ((GU, (CU, GA), CG), 1.6), ((GU, (CU, GC), CG), 2.2), ((GU, (CU, GG), CG), 0.8), ((GU, (CU, GU), CG), 2.2), ((GU, (CU, UA), CG), 2.1), ((GU, (CU, UC), CG), 2.2), ((GU, (CU, UG), CG), 2.1), ((GU, (CU, UU), CG), 1.4),
    ((GU, (GA, AA), CG), 1.6), ((GU, (GA, AC), CG), 1.4), ((GU, (GA, AG), CG), -0.4), ((GU, (GA, AU), CG), 1.4), ((GU, (GA, CA), CG), 1.3), ((GU, (GA, CC), CG), 2.0), ((GU, (GA, CG), CG), 1.3), ((GU, (GA, CU), CG), 2.0), ((GU, (GA, GA), CG), 0.2), ((GU, (GA, GC), CG), 1.4), ((GU, (GA, GG), CG), 1.3), ((GU, (GA, GU), CG), 1.4), ((GU, (GA, UA), CG), 1.3), ((GU, (GA, UC), CG), 2.0), ((GU, (GA, UG), CG), 1.3), ((GU, (GA, UU), CG), 0.6),
    ((GU, (GC, AA), CG), 2.4), ((GU, (GC, AC), CG), 2.2), ((GU, (GC, AG), CG), 0.4), ((GU, (GC, AU), CG), 2.2), ((GU, (GC, CA), CG), 2.1), ((GU, (GC, CC), CG), 2.2), ((GU, (GC, CG), CG), 2.1), ((GU, (GC, CU), CG), 2.2), ((GU, (GC, GA), CG), 1.0), ((GU, (GC, GC), CG), 2.2), ((GU, (GC, GG), CG), 0.8), ((GU, (GC, GU), CG), 2.2), ((GU, (GC, UA), CG), 2.1), ((GU, (GC, UC), CG), 2.2), ((GU, (GC, UG), CG), 2.1), ((GU, (GC, UU), CG), 1.4),
    ((GU, (GG, AA), CG), 1.1), ((GU, (GG, AC), CG), 0.9), ((GU, (GG, AG), CG), 0.4), ((GU, (GG, AU), CG), 0.9), ((GU, (GG, CA), CG), 0.8), ((GU, (GG, CC), CG), 0.9), ((GU, (GG, CG), CG), 0.8), ((GU, (GG, CU), CG), 0.9), ((GU, (GG, GA), CG), 1.0), ((GU, (GG, GC), CG), 0.9), ((GU, (GG, GG), CG), 2.1), ((GU, (GG, GU), CG), 0.9), ((GU, (GG, UA), CG), 0.8), ((GU, (GG, UC), CG), 0.9), ((GU, (GG, UG), CG), 0.8), ((GU, (GG, UU), CG), 1.4),
    ((GU, (GU, AA), CG), 2.4), ((GU, (GU, AC), CG), 2.2), ((GU, (GU, AG), CG), 0.4), ((GU, (GU, AU), CG), 2.2), ((GU, (GU, CA), CG), 2.1), ((GU, (GU, CC), CG), 2.2), ((GU, (GU, CG), CG), 2.1), ((GU, (GU, CU), CG), 2.2), ((GU, (GU, GA), CG), 1.0), ((GU, (GU, GC), CG), 2.2), ((GU, (GU, GG), CG), 0.8), ((GU, (GU, GU), CG), 2.2), ((GU, (GU, UA), CG), 2.1), ((GU, (GU, UC), CG), 2.2), ((GU, (GU, UG), CG), 2.1), ((GU, (GU, UU), CG), 1.4),
    ((GU, (UA, AA), CG), 2.4), ((GU, (UA, AC), CG), 2.2), ((GU, (UA, AG), CG), 0.4), ((GU, (UA, AU), CG), 2.2), ((GU, (UA, CA), CG), 2.1), ((GU, (UA, CC), CG), 2.2), ((GU, (UA, CG), CG), 2.1), ((GU, (UA, CU), CG), 2.2), ((GU, (UA, GA), CG), 1.0), ((GU, (UA, GC), CG), 2.2), ((GU, (UA, GG), CG), 0.8), ((GU, (UA, GU), CG), 2.2), ((GU, (UA, UA), CG), 2.1), ((GU, (UA, UC), CG), 2.2), ((GU, (UA, UG), CG), 2.1), ((GU, (UA, UU), CG), 1.4),
    ((GU, (UC, AA), CG), 2.4), ((GU, (UC, AC), CG), 2.2), ((GU, (UC, AG), CG), 1.0), ((GU, (UC, AU), CG), 2.2), ((GU, (UC, CA), CG), 2.1), ((GU, (UC, CC), CG), 2.2), ((GU, (UC, CG), CG), 2.1), ((GU, (UC, CU), CG), 2.2), ((GU, (UC, GA), CG), 1.6), ((GU, (UC, GC), CG), 2.2), ((GU, (UC, GG), CG), 0.8), ((GU, (UC, GU), CG), 2.2), ((GU, (UC, UA), CG), 2.1), ((GU, (UC, UC), CG), 2.2), ((GU, (UC, UG), CG), 2.1), ((GU, (UC, UU), CG), 1.4),
    ((GU, (UG, AA), CG), 2.4), ((GU, (UG, AC), CG), 2.2), ((GU, (UG, AG), CG), 0.4), ((GU, (UG, AU), CG), 2.2), ((GU, (UG, CA), CG), 2.1), ((GU, (UG, CC), CG), 2.2), ((GU, (UG, CG), CG), 2.1), ((GU, (UG, CU), CG), 2.2), ((GU, (UG, GA), CG), 1.0), ((GU, (UG, GC), CG), 2.2), ((GU, (UG, GG), CG), 0.8), ((GU, (UG, GU), CG), 2.2), ((GU, (UG, UA), CG), 2.1), ((GU, (UG, UC), CG), 2.2), ((GU, (UG, UG), CG), 2.1), ((GU, (UG, UU), CG), 1.4),
    ((GU, (UU, AA), CG), 3.0), ((GU, (UU, AC), CG), 2.2), ((GU, (UU, AG), CG), 0.4), ((GU, (UU, AU), CG), 2.2), ((GU, (UU, CA), CG), 2.1), ((GU, (UU, CC), CG), 2.2), ((GU, (UU, CG), CG), 2.1), ((GU, (UU, CU), CG), 2.2), ((GU, (UU, GA), CG), 1.0), ((GU, (UU, GC), CG), 2.2), ((GU, (UU, GG), CG), 2.1), ((GU, (UU, GU), CG), 2.2), ((GU, (UU, UA), CG), 2.1), ((GU, (UU, UC), CG), 2.2), ((GU, (UU, UG), CG), 2.1), ((GU, (UU, UU), CG), 1.4),
    // For internal loops between the base pairs "GU" and "GC".
    ((GU, (AA, AA), GC), 2.7), ((GU, (AA, AC), GC), 2.6), ((GU, (AA, AG), GC), 1.7), ((GU, (AA, AU), GC), 2.6), ((GU, (AA, CA), GC), 3.0), ((GU, (AA, CC), GC), 2.9), ((GU, (AA, CG), GC), 3.0), ((GU, (AA, CU), GC), 2.7), ((GU, (AA, GA), GC), 1.7), ((GU, (AA, GC), GC), 2.6), ((GU, (AA, GG), GC), 1.1), ((GU, (AA, GU), GC), 2.6), ((GU, (AA, UA), GC), 3.0), ((GU, (AA, UC), GC), 2.7), ((GU, (AA, UG), GC), 3.0), ((GU, (AA, UU), GC), 2.4),
    ((GU, (AC, AA), GC), 2.3), ((GU, (AC, AC), GC), 2.2), ((GU, (AC, AG), GC), 1.3), ((GU, (AC, AU), GC), 2.2), ((GU, (AC, CA), GC), 2.7), ((GU, (AC, CC), GC), 2.5), ((GU, (AC, CG), GC), 2.7), ((GU, (AC, CU), GC), 2.4), ((GU, (AC, GA), GC), 1.3), ((GU, (AC, GC), GC), 2.2), ((GU, (AC, GG), GC), 0.8), ((GU, (AC, GU), GC), 2.2), ((GU, (AC, UA), GC), 2.7), ((GU, (AC, UC), GC), 2.4), ((GU, (AC, UG), GC), 2.7), ((GU, (AC, UU), GC), 1.5),
    ((GU, (AG, AA), GC), 1.9), ((GU, (AG, AC), GC), 1.8), ((GU, (AG, AG), GC), 0.9), ((GU, (AG, AU), GC), 1.8), ((GU, (AG, CA), GC), 2.3), ((GU, (AG, CC), GC), 2.7), ((GU, (AG, CG), GC), 2.3), ((GU, (AG, CU), GC), 2.6), ((GU, (AG, GA), GC), 0.9), ((GU, (AG, GC), GC), 1.8), ((GU, (AG, GG), GC), 1.7), ((GU, (AG, GU), GC), 1.8), ((GU, (AG, UA), GC), 2.3), ((GU, (AG, UC), GC), 2.6), ((GU, (AG, UG), GC), 2.3), ((GU, (AG, UU), GC), 1.1),
    ((GU, (AU, AA), GC), 2.3), ((GU, (AU, AC), GC), 2.2), ((GU, (AU, AG), GC), 1.3), ((GU, (AU, AU), GC), 2.2), ((GU, (AU, CA), GC), 2.7), ((GU, (AU, CC), GC), 2.5), ((GU, (AU, CG), GC), 2.7), ((GU, (AU, CU), GC), 2.4), ((GU, (AU, GA), GC), 1.3), ((GU, (AU, GC), GC), 2.2), ((GU, (AU, GG), GC), 0.8), ((GU, (AU, GU), GC), 2.2), ((GU, (AU, UA), GC), 2.7), ((GU, (AU, UC), GC), 2.4), ((GU, (AU, UG), GC), 2.7), ((GU, (AU, UU), GC), 1.5),
    ((GU, (CA, AA), GC), 2.3), ((GU, (CA, AC), GC), 2.2), ((GU, (CA, AG), GC), 1.3), ((GU, (CA, AU), GC), 2.2), ((GU, (CA, CA), GC), 2.7), ((GU, (CA, CC), GC), 2.5), ((GU, (CA, CG), GC), 2.7), ((GU, (CA, CU), GC), 2.4), ((GU, (CA, GA), GC), 1.9), ((GU, (CA, GC), GC), 2.2), ((GU, (CA, GG), GC), 0.8), ((GU, (CA, GU), GC), 2.2), ((GU, (CA, UA), GC), 2.7), ((GU, (CA, UC), GC), 2.4), ((GU, (CA, UG), GC), 2.7), ((GU, (CA, UU), GC), 1.5),
    ((GU, (CC, AA), GC), 2.3), ((GU, (CC, AC), GC), 2.2), ((GU, (CC, AG), GC), 1.9), ((GU, (CC, AU), GC), 2.2), ((GU, (CC, CA), GC), 2.7), ((GU, (CC, CC), GC), 2.5), ((GU, (CC, CG), GC), 2.7), ((GU, (CC, CU), GC), 2.4), ((GU, (CC, GA), GC), 1.9), ((GU, (CC, GC), GC), 2.2), ((GU, (CC, GG), GC), 0.8), ((GU, (CC, GU), GC), 2.2), ((GU, (CC, UA), GC), 2.7), ((GU, (CC, UC), GC), 2.4), ((GU, (CC, UG), GC), 2.7), ((GU, (CC, UU), GC), 1.5),
    ((GU, (CG, AA), GC), 2.3), ((GU, (CG, AC), GC), 2.2), ((GU, (CG, AG), GC), 1.3), ((GU, (CG, AU), GC), 2.2), ((GU, (CG, CA), GC), 2.7), ((GU, (CG, CC), GC), 2.5), ((GU, (CG, CG), GC), 2.7), ((GU, (CG, CU), GC), 2.4), ((GU, (CG, GA), GC), 1.3), ((GU, (CG, GC), GC), 2.2), ((GU, (CG, GG), GC), 0.8), ((GU, (CG, GU), GC), 2.2), ((GU, (CG, UA), GC), 2.7), ((GU, (CG, UC), GC), 2.4), ((GU, (CG, UG), GC), 2.7), ((GU, (CG, UU), GC), 1.5),
    ((GU, (CU, AA), GC), 2.3), ((GU, (CU, AC), GC), 2.2), ((GU, (CU, AG), GC), 1.9), ((GU, (CU, AU), GC), 2.2), ((GU, (CU, CA), GC), 2.7), ((GU, (CU, CC), GC), 2.5), ((GU, (CU, CG), GC), 2.7), ((GU, (CU, CU), GC), 2.4), ((GU, (CU, GA), GC), 1.9), ((GU, (CU, GC), GC), 2.2), ((GU, (CU, GG), GC), 0.8), ((GU, (CU, GU), GC), 2.2), ((GU, (CU, UA), GC), 2.7), ((GU, (CU, UC), GC), 2.4), ((GU, (CU, UG), GC), 2.7), ((GU, (CU, UU), GC), 1.5),
    ((GU, (GA, AA), GC), 1.5), ((GU, (GA, AC), GC), 1.4), ((GU, (GA, AG), GC), 0.5), ((GU, (GA, AU), GC), 1.4), ((GU, (GA, CA), GC), 1.9), ((GU, (GA, CC), GC), 2.3), ((GU, (GA, CG), GC), 1.9), ((GU, (GA, CU), GC), 2.2), ((GU, (GA, GA), GC), 0.5), ((GU, (GA, GC), GC), 1.4), ((GU, (GA, GG), GC), 1.3), ((GU, (GA, GU), GC), 1.4), ((GU, (GA, UA), GC), 1.9), ((GU, (GA, UC), GC), 2.2), ((GU, (GA, UG), GC), 1.9), ((GU, (GA, UU), GC), 0.7),
    ((GU, (GC, AA), GC), 2.3), ((GU, (GC, AC), GC), 2.2), ((GU, (GC, AG), GC), 1.3), ((GU, (GC, AU), GC), 2.2), ((GU, (GC, CA), GC), 2.7), ((GU, (GC, CC), GC), 2.5), ((GU, (GC, CG), GC), 2.7), ((GU, (GC, CU), GC), 2.4), ((GU, (GC, GA), GC), 1.3), ((GU, (GC, GC), GC), 2.2), ((GU, (GC, GG), GC), 0.8), ((GU, (GC, GU), GC), 2.2), ((GU, (GC, UA), GC), 2.7), ((GU, (GC, UC), GC), 2.4), ((GU, (GC, UG), GC), 2.7), ((GU, (GC, UU), GC), 1.5),
    ((GU, (GG, AA), GC), 1.0), ((GU, (GG, AC), GC), 0.9), ((GU, (GG, AG), GC), 1.3), ((GU, (GG, AU), GC), 0.9), ((GU, (GG, CA), GC), 1.4), ((GU, (GG, CC), GC), 1.2), ((GU, (GG, CG), GC), 1.4), ((GU, (GG, CU), GC), 1.1), ((GU, (GG, GA), GC), 1.3), ((GU, (GG, GC), GC), 0.9), ((GU, (GG, GG), GC), 2.1), ((GU, (GG, GU), GC), 0.9), ((GU, (GG, UA), GC), 1.4), ((GU, (GG, UC), GC), 1.1), ((GU, (GG, UG), GC), 1.4), ((GU, (GG, UU), GC), 1.5),
    ((GU, (GU, AA), GC), 2.3), ((GU, (GU, AC), GC), 2.2), ((GU, (GU, AG), GC), 1.3), ((GU, (GU, AU), GC), 2.2), ((GU, (GU, CA), GC), 2.7), ((GU, (GU, CC), GC), 2.5), ((GU, (GU, CG), GC), 2.7), ((GU, (GU, CU), GC), 2.4), ((GU, (GU, GA), GC), 1.3), ((GU, (GU, GC), GC), 2.2), ((GU, (GU, GG), GC), 0.8), ((GU, (GU, GU), GC), 2.2), ((GU, (GU, UA), GC), 2.7), ((GU, (GU, UC), GC), 2.4), ((GU, (GU, UG), GC), 2.7), ((GU, (GU, UU), GC), 1.5),
    ((GU, (UA, AA), GC), 2.3), ((GU, (UA, AC), GC), 2.2), ((GU, (UA, AG), GC), 1.3), ((GU, (UA, AU), GC), 2.2), ((GU, (UA, CA), GC), 2.7), ((GU, (UA, CC), GC), 2.5), ((GU, (UA, CG), GC), 2.7), ((GU, (UA, CU), GC), 2.4), ((GU, (UA, GA), GC), 1.3), ((GU, (UA, GC), GC), 2.2), ((GU, (UA, GG), GC), 0.8), ((GU, (UA, GU), GC), 2.2), ((GU, (UA, UA), GC), 2.7), ((GU, (UA, UC), GC), 2.4), ((GU, (UA, UG), GC), 2.7), ((GU, (UA, UU), GC), 1.5),
    ((GU, (UC, AA), GC), 2.3), ((GU, (UC, AC), GC), 2.2), ((GU, (UC, AG), GC), 1.9), ((GU, (UC, AU), GC), 2.2), ((GU, (UC, CA), GC), 2.7), ((GU, (UC, CC), GC), 2.5), ((GU, (UC, CG), GC), 2.7), ((GU, (UC, CU), GC), 2.4), ((GU, (UC, GA), GC), 1.9), ((GU, (UC, GC), GC), 2.2), ((GU, (UC, GG), GC), 0.8), ((GU, (UC, GU), GC), 2.2), ((GU, (UC, UA), GC), 2.7), ((GU, (UC, UC), GC), 2.4), ((GU, (UC, UG), GC), 2.7), ((GU, (UC, UU), GC), 1.5),
    ((GU, (UG, AA), GC), 2.3), ((GU, (UG, AC), GC), 2.2), ((GU, (UG, AG), GC), 1.3), ((GU, (UG, AU), GC), 2.2), ((GU, (UG, CA), GC), 2.7), ((GU, (UG, CC), GC), 2.5), ((GU, (UG, CG), GC), 2.7), ((GU, (UG, CU), GC), 2.4), ((GU, (UG, GA), GC), 1.3), ((GU, (UG, GC), GC), 2.2), ((GU, (UG, GG), GC), 0.8), ((GU, (UG, GU), GC), 2.2), ((GU, (UG, UA), GC), 2.7), ((GU, (UG, UC), GC), 2.4), ((GU, (UG, UG), GC), 2.7), ((GU, (UG, UU), GC), 1.5),
    ((GU, (UU, AA), GC), 2.9), ((GU, (UU, AC), GC), 2.2), ((GU, (UU, AG), GC), 1.3), ((GU, (UU, AU), GC), 2.2), ((GU, (UU, CA), GC), 2.7), ((GU, (UU, CC), GC), 2.5), ((GU, (UU, CG), GC), 2.7), ((GU, (UU, CU), GC), 2.4), ((GU, (UU, GA), GC), 1.3), ((GU, (UU, GC), GC), 2.2), ((GU, (UU, GG), GC), 2.1), ((GU, (UU, GU), GC), 2.2), ((GU, (UU, UA), GC), 2.7), ((GU, (UU, UC), GC), 2.4), ((GU, (UU, UG), GC), 2.7), ((GU, (UU, UU), GC), 1.5),
    // For internal loops between the base pairs "GU" and "GU".
    ((GU, (AA, AA), GU), 3.6), ((GU, (AA, AC), GU), 3.4), ((GU, (AA, AG), GU), 2.2), ((GU, (AA, AU), GU), 3.4), ((GU, (AA, CA), GU), 3.4), ((GU, (AA, CC), GU), 3.4), ((GU, (AA, CG), GU), 3.4), ((GU, (AA, CU), GU), 3.4), ((GU, (AA, GA), GU), 3.7), ((GU, (AA, GC), GU), 3.4), ((GU, (AA, GG), GU), 2.1), ((GU, (AA, GU), GU), 3.4), ((GU, (AA, UA), GU), 3.4), ((GU, (AA, UC), GU), 3.4), ((GU, (AA, UG), GU), 3.4), ((GU, (AA, UU), GU), 4.0),
    ((GU, (AC, AA), GU), 2.7), ((GU, (AC, AC), GU), 3.1), ((GU, (AC, AG), GU), 1.7), ((GU, (AC, AU), GU), 3.1), ((GU, (AC, CA), GU), 3.1), ((GU, (AC, CC), GU), 3.1), ((GU, (AC, CG), GU), 3.1), ((GU, (AC, CU), GU), 3.1), ((GU, (AC, GA), GU), 3.4), ((GU, (AC, GC), GU), 3.1), ((GU, (AC, GG), GU), 1.8), ((GU, (AC, GU), GU), 3.1), ((GU, (AC, UA), GU), 3.1), ((GU, (AC, UC), GU), 3.1), ((GU, (AC, UG), GU), 3.1), ((GU, (AC, UU), GU), 3.1),
    ((GU, (AG, AA), GU), 3.6), ((GU, (AG, AC), GU), 2.7), ((GU, (AG, AG), GU), 1.3), ((GU, (AG, AU), GU), 2.7), ((GU, (AG, CA), GU), 2.7), ((GU, (AG, CC), GU), 3.3), ((GU, (AG, CG), GU), 2.7), ((GU, (AG, CU), GU), 3.3), ((GU, (AG, GA), GU), 3.0), ((GU, (AG, GC), GU), 2.7), ((GU, (AG, GG), GU), 2.7), ((GU, (AG, GU), GU), 2.7), ((GU, (AG, UA), GU), 2.7), ((GU, (AG, UC), GU), 3.3), ((GU, (AG, UG), GU), 2.7), ((GU, (AG, UU), GU), 2.7),
    ((GU, (AU, AA), GU), 2.7), ((GU, (AU, AC), GU), 3.1), ((GU, (AU, AG), GU), 1.7), ((GU, (AU, AU), GU), 3.1), ((GU, (AU, CA), GU), 3.1), ((GU, (AU, CC), GU), 3.1), ((GU, (AU, CG), GU), 3.1), ((GU, (AU, CU), GU), 3.1), ((GU, (AU, GA), GU), 3.4), ((GU, (AU, GC), GU), 3.1), ((GU, (AU, GG), GU), 1.8), ((GU, (AU, GU), GU), 3.1), ((GU, (AU, UA), GU), 3.1), ((GU, (AU, UC), GU), 3.1), ((GU, (AU, UG), GU), 3.1), ((GU, (AU, UU), GU), 3.1),
    ((GU, (CA, AA), GU), 2.7), ((GU, (CA, AC), GU), 3.1), ((GU, (CA, AG), GU), 1.7), ((GU, (CA, AU), GU), 3.1), ((GU, (CA, CA), GU), 3.1), ((GU, (CA, CC), GU), 3.1), ((GU, (CA, CG), GU), 3.1), ((GU, (CA, CU), GU), 3.1), ((GU, (CA, GA), GU), 3.4), ((GU, (CA, GC), GU), 3.1), ((GU, (CA, GG), GU), 1.8), ((GU, (CA, GU), GU), 3.1), ((GU, (CA, UA), GU), 3.1), ((GU, (CA, UC), GU), 3.1), ((GU, (CA, UG), GU), 3.1), ((GU, (CA, UU), GU), 3.1),
    ((GU, (CC, AA), GU), 2.7), ((GU, (CC, AC), GU), 3.1), ((GU, (CC, AG), GU), 2.3), ((GU, (CC, AU), GU), 3.1), ((GU, (CC, CA), GU), 3.1), ((GU, (CC, CC), GU), 3.1), ((GU, (CC, CG), GU), 3.1), ((GU, (CC, CU), GU), 3.1), ((GU, (CC, GA), GU), 4.0), ((GU, (CC, GC), GU), 3.1), ((GU, (CC, GG), GU), 1.8), ((GU, (CC, GU), GU), 3.1), ((GU, (CC, UA), GU), 3.1), ((GU, (CC, UC), GU), 3.1), ((GU, (CC, UG), GU), 3.1), ((GU, (CC, UU), GU), 3.1),
    ((GU, (CG, AA), GU), 2.7), ((GU, (CG, AC), GU), 3.1), ((GU, (CG, AG), GU), 1.7), ((GU, (CG, AU), GU), 3.1), ((GU, (CG, CA), GU), 3.1), ((GU, (CG, CC), GU), 3.1), ((GU, (CG, CG), GU), 3.1), ((GU, (CG, CU), GU), 3.1), ((GU, (CG, GA), GU), 3.4), ((GU, (CG, GC), GU), 3.1), ((GU, (CG, GG), GU), 1.8), ((GU, (CG, GU), GU), 3.1), ((GU, (CG, UA), GU), 3.1), ((GU, (CG, UC), GU), 3.1), ((GU, (CG, UG), GU), 3.1), ((GU, (CG, UU), GU), 3.1),
    ((GU, (CU, AA), GU), 2.7), ((GU, (CU, AC), GU), 3.1), ((GU, (CU, AG), GU), 2.3), ((GU, (CU, AU), GU), 3.1), ((GU, (CU, CA), GU), 3.1), ((GU, (CU, CC), GU), 3.1), ((GU, (CU, CG), GU), 3.1), ((GU, (CU, CU), GU), 3.1), ((GU, (CU, GA), GU), 4.0), ((GU, (CU, GC), GU), 3.1), ((GU, (CU, GG), GU), 1.8), ((GU, (CU, GU), GU), 3.1), ((GU, (CU, UA), GU), 3.1), ((GU, (CU, UC), GU), 3.1), ((GU, (CU, UG), GU), 3.1), ((GU, (CU, UU), GU), 3.1),
    ((GU, (GA, AA), GU), 1.9), ((GU, (GA, AC), GU), 2.3), ((GU, (GA, AG), GU), 0.2), ((GU, (GA, AU), GU), 2.3), ((GU, (GA, CA), GU), 2.3), ((GU, (GA, CC), GU), 2.9), ((GU, (GA, CG), GU), 2.3), ((GU, (GA, CU), GU), 2.9), ((GU, (GA, GA), GU), 2.6), ((GU, (GA, GC), GU), 2.3), ((GU, (GA, GG), GU), 2.3), ((GU, (GA, GU), GU), 2.3), ((GU, (GA, UA), GU), 2.3), ((GU, (GA, UC), GU), 2.9), ((GU, (GA, UG), GU), 2.3), ((GU, (GA, UU), GU), 2.3),
    ((GU, (GC, AA), GU), 2.7), ((GU, (GC, AC), GU), 3.1), ((GU, (GC, AG), GU), 1.7), ((GU, (GC, AU), GU), 3.1), ((GU, (GC, CA), GU), 3.1), ((GU, (GC, CC), GU), 3.1), ((GU, (GC, CG), GU), 3.1), ((GU, (GC, CU), GU), 3.1), ((GU, (GC, GA), GU), 3.4), ((GU, (GC, GC), GU), 3.1), ((GU, (GC, GG), GU), 1.8), ((GU, (GC, GU), GU), 3.1), ((GU, (GC, UA), GU), 3.1), ((GU, (GC, UC), GU), 3.1), ((GU, (GC, UG), GU), 3.1), ((GU, (GC, UU), GU), 3.1),
    ((GU, (GG, AA), GU), 1.4), ((GU, (GG, AC), GU), 1.8), ((GU, (GG, AG), GU), 1.7), ((GU, (GG, AU), GU), 1.8), ((GU, (GG, CA), GU), 1.8), ((GU, (GG, CC), GU), 1.8), ((GU, (GG, CG), GU), 1.8), ((GU, (GG, CU), GU), 1.8), ((GU, (GG, GA), GU), 3.4), ((GU, (GG, GC), GU), 1.8), ((GU, (GG, GG), GU), 3.1), ((GU, (GG, GU), GU), 1.8), ((GU, (GG, UA), GU), 1.8), ((GU, (GG, UC), GU), 1.8), ((GU, (GG, UG), GU), 1.8), ((GU, (GG, UU), GU), 3.1),
    ((GU, (GU, AA), GU), 2.7), ((GU, (GU, AC), GU), 3.1), ((GU, (GU, AG), GU), 1.7), ((GU, (GU, AU), GU), 3.1), ((GU, (GU, CA), GU), 3.1), ((GU, (GU, CC), GU), 3.1), ((GU, (GU, CG), GU), 3.1), ((GU, (GU, CU), GU), 3.1), ((GU, (GU, GA), GU), 3.4), ((GU, (GU, GC), GU), 3.1), ((GU, (GU, GG), GU), 1.8), ((GU, (GU, GU), GU), 3.1), ((GU, (GU, UA), GU), 3.1), ((GU, (GU, UC), GU), 3.1), ((GU, (GU, UG), GU), 3.1), ((GU, (GU, UU), GU), 3.1),
    ((GU, (UA, AA), GU), 2.7), ((GU, (UA, AC), GU), 3.1), ((GU, (UA, AG), GU), 1.7), ((GU, (UA, AU), GU), 3.1), ((GU, (UA, CA), GU), 3.1), ((GU, (UA, CC), GU), 3.1), ((GU, (UA, CG), GU), 3.1), ((GU, (UA, CU), GU), 3.1), ((GU, (UA, GA), GU), 3.4), ((GU, (UA, GC), GU), 3.1), ((GU, (UA, GG), GU), 1.8), ((GU, (UA, GU), GU), 3.1), ((GU, (UA, UA), GU), 3.1), ((GU, (UA, UC), GU), 3.1), ((GU, (UA, UG), GU), 3.1), ((GU, (UA, UU), GU), 3.1),
    ((GU, (UC, AA), GU), 2.7), ((GU, (UC, AC), GU), 3.1), ((GU, (UC, AG), GU), 2.3), ((GU, (UC, AU), GU), 3.1), ((GU, (UC, CA), GU), 3.1), ((GU, (UC, CC), GU), 3.1), ((GU, (UC, CG), GU), 3.1), ((GU, (UC, CU), GU), 3.1), ((GU, (UC, GA), GU), 4.0), ((GU, (UC, GC), GU), 3.1), ((GU, (UC, GG), GU), 1.8), ((GU, (UC, GU), GU), 3.1), ((GU, (UC, UA), GU), 3.1), ((GU, (UC, UC), GU), 3.1), ((GU, (UC, UG), GU), 3.1), ((GU, (UC, UU), GU), 3.1),
    ((GU, (UG, AA), GU), 2.7), ((GU, (UG, AC), GU), 3.1), ((GU, (UG, AG), GU), 1.7), ((GU, (UG, AU), GU), 3.1), ((GU, (UG, CA), GU), 3.1), ((GU, (UG, CC), GU), 3.1), ((GU, (UG, CG), GU), 3.1), ((GU, (UG, CU), GU), 3.1), ((GU, (UG, GA), GU), 3.4), ((GU, (UG, GC), GU), 3.1), ((GU, (UG, GG), GU), 1.8), ((GU, (UG, GU), GU), 3.1), ((GU, (UG, UA), GU), 3.1), ((GU, (UG, UC), GU), 3.1), ((GU, (UG, UG), GU), 3.1), ((GU, (UG, UU), GU), 3.1),
    ((GU, (UU, AA), GU), 3.3), ((GU, (UU, AC), GU), 3.1), ((GU, (UU, AG), GU), 1.7), ((GU, (UU, AU), GU), 3.1), ((GU, (UU, CA), GU), 3.1), ((GU, (UU, CC), GU), 3.1), ((GU, (UU, CG), GU), 3.1), ((GU, (UU, CU), GU), 3.1), ((GU, (UU, GA), GU), 3.4), ((GU, (UU, GC), GU), 3.1), ((GU, (UU, GG), GU), 3.1), ((GU, (UU, GU), GU), 3.1), ((GU, (UU, UA), GU), 3.1), ((GU, (UU, UC), GU), 3.1), ((GU, (UU, UG), GU), 3.1), ((GU, (UU, UU), GU), 3.1),
    // For internal loops between the base pairs "GU" and "UA".
    ((GU, (AA, AA), UA), 3.4), ((GU, (AA, AC), UA), 3.2), ((GU, (AA, AG), UA), 2.2), ((GU, (AA, AU), UA), 3.2), ((GU, (AA, CA), UA), 3.3), ((GU, (AA, CC), UA), 3.2), ((GU, (AA, CG), UA), 3.3), ((GU, (AA, CU), UA), 3.2), ((GU, (AA, GA), UA), 2.9), ((GU, (AA, GC), UA), 3.2), ((GU, (AA, GG), UA), 1.7), ((GU, (AA, GU), UA), 3.2), ((GU, (AA, UA), UA), 3.3), ((GU, (AA, UC), UA), 3.2), ((GU, (AA, UG), UA), 3.3), ((GU, (AA, UU), UA), 2.9),
    ((GU, (AC, AA), UA), 3.1), ((GU, (AC, AC), UA), 2.8), ((GU, (AC, AG), UA), 1.8), ((GU, (AC, AU), UA), 2.8), ((GU, (AC, CA), UA), 2.9), ((GU, (AC, CC), UA), 2.9), ((GU, (AC, CG), UA), 2.9), ((GU, (AC, CU), UA), 2.9), ((GU, (AC, GA), UA), 2.5), ((GU, (AC, GC), UA), 2.8), ((GU, (AC, GG), UA), 1.3), ((GU, (AC, GU), UA), 2.8), ((GU, (AC, UA), UA), 2.9), ((GU, (AC, UC), UA), 2.9), ((GU, (AC, UG), UA), 2.9), ((GU, (AC, UU), UA), 2.0),
    ((GU, (AG, AA), UA), 2.7), ((GU, (AG, AC), UA), 2.4), ((GU, (AG, AG), UA), 1.4), ((GU, (AG, AU), UA), 2.4), ((GU, (AG, CA), UA), 2.5), ((GU, (AG, CC), UA), 3.1), ((GU, (AG, CG), UA), 2.5), ((GU, (AG, CU), UA), 3.1), ((GU, (AG, GA), UA), 2.1), ((GU, (AG, GC), UA), 2.4), ((GU, (AG, GG), UA), 2.2), ((GU, (AG, GU), UA), 2.4), ((GU, (AG, UA), UA), 2.5), ((GU, (AG, UC), UA), 3.1), ((GU, (AG, UG), UA), 2.5), ((GU, (AG, UU), UA), 1.6),
    ((GU, (AU, AA), UA), 3.1), ((GU, (AU, AC), UA), 2.8), ((GU, (AU, AG), UA), 1.8), ((GU, (AU, AU), UA), 2.8), ((GU, (AU, CA), UA), 2.9), ((GU, (AU, CC), UA), 2.9), ((GU, (AU, CG), UA), 2.9), ((GU, (AU, CU), UA), 2.9), ((GU, (AU, GA), UA), 2.5), ((GU, (AU, GC), UA), 2.8), ((GU, (AU, GG), UA), 1.3), ((GU, (AU, GU), UA), 2.8), ((GU, (AU, UA), UA), 2.9), ((GU, (AU, UC), UA), 2.9), ((GU, (AU, UG), UA), 2.9), ((GU, (AU, UU), UA), 2.0),
    ((GU, (CA, AA), UA), 3.1), ((GU, (CA, AC), UA), 2.8), ((GU, (CA, AG), UA), 1.8), ((GU, (CA, AU), UA), 2.8), ((GU, (CA, CA), UA), 2.9), ((GU, (CA, CC), UA), 2.9), ((GU, (CA, CG), UA), 2.9), ((GU, (CA, CU), UA), 2.9), ((GU, (CA, GA), UA), 2.5), ((GU, (CA, GC), UA), 2.8), ((GU, (CA, GG), UA), 1.3), ((GU, (CA, GU), UA), 2.8), ((GU, (CA, UA), UA), 2.9), ((GU, (CA, UC), UA), 2.9), ((GU, (CA, UG), UA), 2.9), ((GU, (CA, UU), UA), 2.0),
    ((GU, (CC, AA), UA), 3.1), ((GU, (CC, AC), UA), 2.8), ((GU, (CC, AG), UA), 2.4), ((GU, (CC, AU), UA), 2.8), ((GU, (CC, CA), UA), 2.9), ((GU, (CC, CC), UA), 2.9), ((GU, (CC, CG), UA), 2.9), ((GU, (CC, CU), UA), 2.9), ((GU, (CC, GA), UA), 3.1), ((GU, (CC, GC), UA), 2.8), ((GU, (CC, GG), UA), 1.3), ((GU, (CC, GU), UA), 2.8), ((GU, (CC, UA), UA), 2.9), ((GU, (CC, UC), UA), 2.9), ((GU, (CC, UG), UA), 2.9), ((GU, (CC, UU), UA), 2.0),
    ((GU, (CG, AA), UA), 3.1), ((GU, (CG, AC), UA), 2.8), ((GU, (CG, AG), UA), 1.8), ((GU, (CG, AU), UA), 2.8), ((GU, (CG, CA), UA), 2.9), ((GU, (CG, CC), UA), 2.9), ((GU, (CG, CG), UA), 2.9), ((GU, (CG, CU), UA), 2.9), ((GU, (CG, GA), UA), 2.5), ((GU, (CG, GC), UA), 2.8), ((GU, (CG, GG), UA), 1.3), ((GU, (CG, GU), UA), 2.8), ((GU, (CG, UA), UA), 2.9), ((GU, (CG, UC), UA), 2.9), ((GU, (CG, UG), UA), 2.9), ((GU, (CG, UU), UA), 2.0),
    ((GU, (CU, AA), UA), 3.1), ((GU, (CU, AC), UA), 2.8), ((GU, (CU, AG), UA), 2.4), ((GU, (CU, AU), UA), 2.8), ((GU, (CU, CA), UA), 2.9), ((GU, (CU, CC), UA), 2.9), ((GU, (CU, CG), UA), 2.9), ((GU, (CU, CU), UA), 2.9), ((GU, (CU, GA), UA), 3.1), ((GU, (CU, GC), UA), 2.8), ((GU, (CU, GG), UA), 1.3), ((GU, (CU, GU), UA), 2.8), ((GU, (CU, UA), UA), 2.9), ((GU, (CU, UC), UA), 2.9), ((GU, (CU, UG), UA), 2.9), ((GU, (CU, UU), UA), 2.0),
    ((GU, (GA, AA), UA), 2.3), ((GU, (GA, AC), UA), 2.0), ((GU, (GA, AG), UA), 1.0), ((GU, (GA, AU), UA), 2.0), ((GU, (GA, CA), UA), 2.1), ((GU, (GA, CC), UA), 2.7), ((GU, (GA, CG), UA), 2.1), ((GU, (GA, CU), UA), 2.7), ((GU, (GA, GA), UA), 1.7), ((GU, (GA, GC), UA), 2.0), ((GU, (GA, GG), UA), 1.8), ((GU, (GA, GU), UA), 2.0), ((GU, (GA, UA), UA), 2.1), ((GU, (GA, UC), UA), 2.7), ((GU, (GA, UG), UA), 2.1), ((GU, (GA, UU), UA), 1.2),
    ((GU, (GC, AA), UA), 3.1), ((GU, (GC, AC), UA), 2.8), ((GU, (GC, AG), UA), 1.8), ((GU, (GC, AU), UA), 2.8), ((GU, (GC, CA), UA), 2.9), ((GU, (GC, CC), UA), 2.9), ((GU, (GC, CG), UA), 2.9), ((GU, (GC, CU), UA), 2.9), ((GU, (GC, GA), UA), 2.5), ((GU, (GC, GC), UA), 2.8), ((GU, (GC, GG), UA), 1.3), ((GU, (GC, GU), UA), 2.8), ((GU, (GC, UA), UA), 2.9), ((GU, (GC, UC), UA), 2.9), ((GU, (GC, UG), UA), 2.9), ((GU, (GC, UU), UA), 2.0),
    ((GU, (GG, AA), UA), 1.8), ((GU, (GG, AC), UA), 1.5), ((GU, (GG, AG), UA), 1.8), ((GU, (GG, AU), UA), 1.5), ((GU, (GG, CA), UA), 1.6), ((GU, (GG, CC), UA), 1.6), ((GU, (GG, CG), UA), 1.6), ((GU, (GG, CU), UA), 1.6), ((GU, (GG, GA), UA), 2.5), ((GU, (GG, GC), UA), 1.5), ((GU, (GG, GG), UA), 2.6), ((GU, (GG, GU), UA), 1.5), ((GU, (GG, UA), UA), 1.6), ((GU, (GG, UC), UA), 1.6), ((GU, (GG, UG), UA), 1.6), ((GU, (GG, UU), UA), 2.0),
    ((GU, (GU, AA), UA), 3.1), ((GU, (GU, AC), UA), 2.8), ((GU, (GU, AG), UA), 1.8), ((GU, (GU, AU), UA), 2.8), ((GU, (GU, CA), UA), 2.9), ((GU, (GU, CC), UA), 2.9), ((GU, (GU, CG), UA), 2.9), ((GU, (GU, CU), UA), 2.9), ((GU, (GU, GA), UA), 2.5), ((GU, (GU, GC), UA), 2.8), ((GU, (GU, GG), UA), 1.3), ((GU, (GU, GU), UA), 2.8), ((GU, (GU, UA), UA), 2.9), ((GU, (GU, UC), UA), 2.9), ((GU, (GU, UG), UA), 2.9), ((GU, (GU, UU), UA), 2.0),
    ((GU, (UA, AA), UA), 3.1), ((GU, (UA, AC), UA), 2.8), ((GU, (UA, AG), UA), 1.8), ((GU, (UA, AU), UA), 2.8), ((GU, (UA, CA), UA), 2.9), ((GU, (UA, CC), UA), 2.9), ((GU, (UA, CG), UA), 2.9), ((GU, (UA, CU), UA), 2.9), ((GU, (UA, GA), UA), 2.5), ((GU, (UA, GC), UA), 2.8), ((GU, (UA, GG), UA), 1.3), ((GU, (UA, GU), UA), 2.8), ((GU, (UA, UA), UA), 2.9), ((GU, (UA, UC), UA), 2.9), ((GU, (UA, UG), UA), 2.9), ((GU, (UA, UU), UA), 2.0),
    ((GU, (UC, AA), UA), 3.1), ((GU, (UC, AC), UA), 2.8), ((GU, (UC, AG), UA), 2.4), ((GU, (UC, AU), UA), 2.8), ((GU, (UC, CA), UA), 2.9), ((GU, (UC, CC), UA), 2.9), ((GU, (UC, CG), UA), 2.9), ((GU, (UC, CU), UA), 2.9), ((GU, (UC, GA), UA), 3.1), ((GU, (UC, GC), UA), 2.8), ((GU, (UC, GG), UA), 1.3), ((GU, (UC, GU), UA), 2.8), ((GU, (UC, UA), UA), 2.9), ((GU, (UC, UC), UA), 2.9), ((GU, (UC, UG), UA), 2.9), ((GU, (UC, UU), UA), 2.0),
    ((GU, (UG, AA), UA), 3.1), ((GU, (UG, AC), UA), 2.8), ((GU, (UG, AG), UA), 1.8), ((GU, (UG, AU), UA), 2.8), ((GU, (UG, CA), UA), 2.9), ((GU, (UG, CC), UA), 2.9), ((GU, (UG, CG), UA), 2.9), ((GU, (UG, CU), UA), 2.9), ((GU, (UG, GA), UA), 2.5), ((GU, (UG, GC), UA), 2.8), ((GU, (UG, GG), UA), 1.3), ((GU, (UG, GU), UA), 2.8), ((GU, (UG, UA), UA), 2.9), ((GU, (UG, UC), UA), 2.9), ((GU, (UG, UG), UA), 2.9), ((GU, (UG, UU), UA), 2.0),
    ((GU, (UU, AA), UA), 3.7), ((GU, (UU, AC), UA), 2.8), ((GU, (UU, AG), UA), 1.8), ((GU, (UU, AU), UA), 2.8), ((GU, (UU, CA), UA), 2.9), ((GU, (UU, CC), UA), 2.9), ((GU, (UU, CG), UA), 2.9), ((GU, (UU, CU), UA), 2.9), ((GU, (UU, GA), UA), 2.5), ((GU, (UU, GC), UA), 2.8), ((GU, (UU, GG), UA), 2.6), ((GU, (UU, GU), UA), 2.8), ((GU, (UU, UA), UA), 2.9), ((GU, (UU, UC), UA), 2.9), ((GU, (UU, UG), UA), 2.9), ((GU, (UU, UU), UA), 2.0),
    // For internal loops between the base pairs "GU" and "UG".
    ((GU, (AA, AA), UG), 4.1), ((GU, (AA, AC), UG), 3.7), ((GU, (AA, AG), UG), 2.9), ((GU, (AA, AU), UG), 3.7), ((GU, (AA, CA), UG), 3.7), ((GU, (AA, CC), UG), 3.7), ((GU, (AA, CG), UG), 3.7), ((GU, (AA, CU), UG), 3.7), ((GU, (AA, GA), UG), 3.3), ((GU, (AA, GC), UG), 3.7), ((GU, (AA, GG), UG), 2.4), ((GU, (AA, GU), UG), 3.7), ((GU, (AA, UA), UG), 3.7), ((GU, (AA, UC), UG), 3.7), ((GU, (AA, UG), UG), 3.7), ((GU, (AA, UU), UG), 4.3),
    ((GU, (AC, AA), UG), 3.7), ((GU, (AC, AC), UG), 3.4), ((GU, (AC, AG), UG), 2.6), ((GU, (AC, AU), UG), 3.4), ((GU, (AC, CA), UG), 3.4), ((GU, (AC, CC), UG), 3.4), ((GU, (AC, CG), UG), 3.4), ((GU, (AC, CU), UG), 3.4), ((GU, (AC, GA), UG), 3.0), ((GU, (AC, GC), UG), 3.4), ((GU, (AC, GG), UG), 2.1), ((GU, (AC, GU), UG), 3.4), ((GU, (AC, UA), UG), 3.4), ((GU, (AC, UC), UG), 3.4), ((GU, (AC, UG), UG), 3.4), ((GU, (AC, UU), UG), 3.4),
    ((GU, (AG, AA), UG), 3.3), ((GU, (AG, AC), UG), 3.0), ((GU, (AG, AG), UG), 2.2), ((GU, (AG, AU), UG), 3.0), ((GU, (AG, CA), UG), 3.0), ((GU, (AG, CC), UG), 3.6), ((GU, (AG, CG), UG), 3.0), ((GU, (AG, CU), UG), 3.6), ((GU, (AG, GA), UG), 2.6), ((GU, (AG, GC), UG), 3.0), ((GU, (AG, GG), UG), 3.0), ((GU, (AG, GU), UG), 3.0), ((GU, (AG, UA), UG), 3.0), ((GU, (AG, UC), UG), 3.6), ((GU, (AG, UG), UG), 3.0), ((GU, (AG, UU), UG), 3.0),
    ((GU, (AU, AA), UG), 3.7), ((GU, (AU, AC), UG), 3.4), ((GU, (AU, AG), UG), 2.6), ((GU, (AU, AU), UG), 3.4), ((GU, (AU, CA), UG), 3.4), ((GU, (AU, CC), UG), 3.4), ((GU, (AU, CG), UG), 3.4), ((GU, (AU, CU), UG), 3.4), ((GU, (AU, GA), UG), 3.0), ((GU, (AU, GC), UG), 3.4), ((GU, (AU, GG), UG), 2.1), ((GU, (AU, GU), UG), 3.4), ((GU, (AU, UA), UG), 3.4), ((GU, (AU, UC), UG), 3.4), ((GU, (AU, UG), UG), 3.4), ((GU, (AU, UU), UG), 3.4),
    ((GU, (CA, AA), UG), 3.7), ((GU, (CA, AC), UG), 3.4), ((GU, (CA, AG), UG), 2.6), ((GU, (CA, AU), UG), 3.4), ((GU, (CA, CA), UG), 3.4), ((GU, (CA, CC), UG), 3.4), ((GU, (CA, CG), UG), 3.4), ((GU, (CA, CU), UG), 3.4), ((GU, (CA, GA), UG), 3.0), ((GU, (CA, GC), UG), 3.4), ((GU, (CA, GG), UG), 2.1), ((GU, (CA, GU), UG), 3.4), ((GU, (CA, UA), UG), 3.4), ((GU, (CA, UC), UG), 3.4), ((GU, (CA, UG), UG), 3.4), ((GU, (CA, UU), UG), 3.4),
    ((GU, (CC, AA), UG), 3.7), ((GU, (CC, AC), UG), 3.4), ((GU, (CC, AG), UG), 3.2), ((GU, (CC, AU), UG), 3.4), ((GU, (CC, CA), UG), 3.4), ((GU, (CC, CC), UG), 3.4), ((GU, (CC, CG), UG), 3.4), ((GU, (CC, CU), UG), 3.4), ((GU, (CC, GA), UG), 3.6), ((GU, (CC, GC), UG), 3.4), ((GU, (CC, GG), UG), 2.1), ((GU, (CC, GU), UG), 3.4), ((GU, (CC, UA), UG), 3.4), ((GU, (CC, UC), UG), 3.4), ((GU, (CC, UG), UG), 3.4), ((GU, (CC, UU), UG), 3.4),
    ((GU, (CG, AA), UG), 3.7), ((GU, (CG, AC), UG), 3.4), ((GU, (CG, AG), UG), 2.6), ((GU, (CG, AU), UG), 3.4), ((GU, (CG, CA), UG), 3.4), ((GU, (CG, CC), UG), 3.4), ((GU, (CG, CG), UG), 3.4), ((GU, (CG, CU), UG), 3.4), ((GU, (CG, GA), UG), 3.0), ((GU, (CG, GC), UG), 3.4), ((GU, (CG, GG), UG), 2.1), ((GU, (CG, GU), UG), 3.4), ((GU, (CG, UA), UG), 3.4), ((GU, (CG, UC), UG), 3.4), ((GU, (CG, UG), UG), 3.4), ((GU, (CG, UU), UG), 3.4),
    ((GU, (CU, AA), UG), 3.7), ((GU, (CU, AC), UG), 3.4), ((GU, (CU, AG), UG), 3.2), ((GU, (CU, AU), UG), 3.4), ((GU, (CU, CA), UG), 3.4), ((GU, (CU, CC), UG), 3.4), ((GU, (CU, CG), UG), 3.4), ((GU, (CU, CU), UG), 3.4), ((GU, (CU, GA), UG), 3.6), ((GU, (CU, GC), UG), 3.4), ((GU, (CU, GG), UG), 2.1), ((GU, (CU, GU), UG), 3.4), ((GU, (CU, UA), UG), 3.4), ((GU, (CU, UC), UG), 3.4), ((GU, (CU, UG), UG), 3.4), ((GU, (CU, UU), UG), 3.4),
    ((GU, (GA, AA), UG), 2.9), ((GU, (GA, AC), UG), 2.6), ((GU, (GA, AG), UG), 1.8), ((GU, (GA, AU), UG), 2.6), ((GU, (GA, CA), UG), 2.6), ((GU, (GA, CC), UG), 3.2), ((GU, (GA, CG), UG), 2.6), ((GU, (GA, CU), UG), 3.2), ((GU, (GA, GA), UG), 2.2), ((GU, (GA, GC), UG), 2.6), ((GU, (GA, GG), UG), 2.6), ((GU, (GA, GU), UG), 2.6), ((GU, (GA, UA), UG), 2.6), ((GU, (GA, UC), UG), 3.2), ((GU, (GA, UG), UG), 2.6), ((GU, (GA, UU), UG), 2.6),
    ((GU, (GC, AA), UG), 3.7), ((GU, (GC, AC), UG), 3.4), ((GU, (GC, AG), UG), 2.6), ((GU, (GC, AU), UG), 3.4), ((GU, (GC, CA), UG), 3.4), ((GU, (GC, CC), UG), 3.4), ((GU, (GC, CG), UG), 3.4), ((GU, (GC, CU), UG), 3.4), ((GU, (GC, GA), UG), 3.0), ((GU, (GC, GC), UG), 3.4), ((GU, (GC, GG), UG), 2.1), ((GU, (GC, GU), UG), 3.4), ((GU, (GC, UA), UG), 3.4), ((GU, (GC, UC), UG), 3.4), ((GU, (GC, UG), UG), 3.4), ((GU, (GC, UU), UG), 3.4),
    ((GU, (GG, AA), UG), 2.4), ((GU, (GG, AC), UG), 2.1), ((GU, (GG, AG), UG), 2.6), ((GU, (GG, AU), UG), 2.1), ((GU, (GG, CA), UG), 2.1), ((GU, (GG, CC), UG), 2.1), ((GU, (GG, CG), UG), 2.1), ((GU, (GG, CU), UG), 2.1), ((GU, (GG, GA), UG), 3.0), ((GU, (GG, GC), UG), 2.1), ((GU, (GG, GG), UG), 3.4), ((GU, (GG, GU), UG), 2.1), ((GU, (GG, UA), UG), 2.1), ((GU, (GG, UC), UG), 2.1), ((GU, (GG, UG), UG), 2.1), ((GU, (GG, UU), UG), 3.4),
    ((GU, (GU, AA), UG), 3.7), ((GU, (GU, AC), UG), 3.4), ((GU, (GU, AG), UG), 2.6), ((GU, (GU, AU), UG), 3.4), ((GU, (GU, CA), UG), 3.4), ((GU, (GU, CC), UG), 3.4), ((GU, (GU, CG), UG), 3.4), ((GU, (GU, CU), UG), 3.4), ((GU, (GU, GA), UG), 3.0), ((GU, (GU, GC), UG), 3.4), ((GU, (GU, GG), UG), 2.1), ((GU, (GU, GU), UG), 3.4), ((GU, (GU, UA), UG), 3.4), ((GU, (GU, UC), UG), 3.4), ((GU, (GU, UG), UG), 3.4), ((GU, (GU, UU), UG), 3.4),
    ((GU, (UA, AA), UG), 3.7), ((GU, (UA, AC), UG), 3.4), ((GU, (UA, AG), UG), 2.6), ((GU, (UA, AU), UG), 3.4), ((GU, (UA, CA), UG), 3.4), ((GU, (UA, CC), UG), 3.4), ((GU, (UA, CG), UG), 3.4), ((GU, (UA, CU), UG), 3.4), ((GU, (UA, GA), UG), 3.0), ((GU, (UA, GC), UG), 3.4), ((GU, (UA, GG), UG), 2.1), ((GU, (UA, GU), UG), 3.4), ((GU, (UA, UA), UG), 3.4), ((GU, (UA, UC), UG), 3.4), ((GU, (UA, UG), UG), 3.4), ((GU, (UA, UU), UG), 3.4),
    ((GU, (UC, AA), UG), 3.7), ((GU, (UC, AC), UG), 3.4), ((GU, (UC, AG), UG), 3.2), ((GU, (UC, AU), UG), 3.4), ((GU, (UC, CA), UG), 3.4), ((GU, (UC, CC), UG), 3.4), ((GU, (UC, CG), UG), 3.4), ((GU, (UC, CU), UG), 3.4), ((GU, (UC, GA), UG), 3.6), ((GU, (UC, GC), UG), 3.4), ((GU, (UC, GG), UG), 2.1), ((GU, (UC, GU), UG), 3.4), ((GU, (UC, UA), UG), 3.4), ((GU, (UC, UC), UG), 3.4), ((GU, (UC, UG), UG), 3.4), ((GU, (UC, UU), UG), 3.4),
    ((GU, (UG, AA), UG), 3.7), ((GU, (UG, AC), UG), 3.4), ((GU, (UG, AG), UG), 2.6), ((GU, (UG, AU), UG), 3.4), ((GU, (UG, CA), UG), 3.4), ((GU, (UG, CC), UG), 3.4), ((GU, (UG, CG), UG), 3.4), ((GU, (UG, CU), UG), 3.4), ((GU, (UG, GA), UG), 3.0), ((GU, (UG, GC), UG), 3.4), ((GU, (UG, GG), UG), 2.1), ((GU, (UG, GU), UG), 3.4), ((GU, (UG, UA), UG), 3.4), ((GU, (UG, UC), UG), 3.4), ((GU, (UG, UG), UG), 3.4), ((GU, (UG, UU), UG), 3.4),
    ((GU, (UU, AA), UG), 4.3), ((GU, (UU, AC), UG), 3.4), ((GU, (UU, AG), UG), 2.6), ((GU, (UU, AU), UG), 3.4), ((GU, (UU, CA), UG), 3.4), ((GU, (UU, CC), UG), 3.4), ((GU, (UU, CG), UG), 3.4), ((GU, (UU, CU), UG), 3.4), ((GU, (UU, GA), UG), 3.0), ((GU, (UU, GC), UG), 3.4), ((GU, (UU, GG), UG), 3.4), ((GU, (UU, GU), UG), 3.4), ((GU, (UU, UA), UG), 3.4), ((GU, (UU, UC), UG), 3.4), ((GU, (UU, UG), UG), 3.4), ((GU, (UU, UU), UG), 3.4),
    // For internal loops between the base pairs "UA" and "AU".
    ((UA, (AA, AA), AU), 2.8), ((UA, (AA, AC), AU), 2.3), ((UA, (AA, AG), AU), 1.7), ((UA, (AA, AU), AU), 2.3), ((UA, (AA, CA), AU), 2.8), ((UA, (AA, CC), AU), 2.8), ((UA, (AA, CG), AU), 2.8), ((UA, (AA, CU), AU), 2.8), ((UA, (AA, GA), AU), 1.8), ((UA, (AA, GC), AU), 2.3), ((UA, (AA, GG), AU), 1.2), ((UA, (AA, GU), AU), 2.3), ((UA, (AA, UA), AU), 2.8), ((UA, (AA, UC), AU), 2.5), ((UA, (AA, UG), AU), 2.8), ((UA, (AA, UU), AU), 2.5),
    ((UA, (AC, AA), AU), 2.8), ((UA, (AC, AC), AU), 2.3), ((UA, (AC, AG), AU), 1.7), ((UA, (AC, AU), AU), 2.3), ((UA, (AC, CA), AU), 2.8), ((UA, (AC, CC), AU), 2.8), ((UA, (AC, CG), AU), 2.8), ((UA, (AC, CU), AU), 2.8), ((UA, (AC, GA), AU), 1.8), ((UA, (AC, GC), AU), 2.3), ((UA, (AC, GG), AU), 1.2), ((UA, (AC, GU), AU), 2.3), ((UA, (AC, UA), AU), 2.8), ((UA, (AC, UC), AU), 2.5), ((UA, (AC, UG), AU), 2.8), ((UA, (AC, UU), AU), 1.9),
    ((UA, (AG, AA), AU), 1.8), ((UA, (AG, AC), AU), 1.4), ((UA, (AG, AG), AU), 0.8), ((UA, (AG, AU), AU), 1.4), ((UA, (AG, CA), AU), 1.8), ((UA, (AG, CC), AU), 2.4), ((UA, (AG, CG), AU), 1.8), ((UA, (AG, CU), AU), 2.4), ((UA, (AG, GA), AU), 0.9), ((UA, (AG, GC), AU), 1.4), ((UA, (AG, GG), AU), 1.6), ((UA, (AG, GU), AU), 1.4), ((UA, (AG, UA), AU), 1.8), ((UA, (AG, UC), AU), 2.1), ((UA, (AG, UG), AU), 1.8), ((UA, (AG, UU), AU), 1.0),
    ((UA, (AU, AA), AU), 2.8), ((UA, (AU, AC), AU), 2.3), ((UA, (AU, AG), AU), 1.7), ((UA, (AU, AU), AU), 2.3), ((UA, (AU, CA), AU), 2.8), ((UA, (AU, CC), AU), 2.8), ((UA, (AU, CG), AU), 2.8), ((UA, (AU, CU), AU), 2.8), ((UA, (AU, GA), AU), 1.8), ((UA, (AU, GC), AU), 2.3), ((UA, (AU, GG), AU), 1.2), ((UA, (AU, GU), AU), 2.3), ((UA, (AU, UA), AU), 2.8), ((UA, (AU, UC), AU), 2.5), ((UA, (AU, UG), AU), 2.8), ((UA, (AU, UU), AU), 1.9),
    ((UA, (CA, AA), AU), 2.3), ((UA, (CA, AC), AU), 1.9), ((UA, (CA, AG), AU), 1.3), ((UA, (CA, AU), AU), 1.9), ((UA, (CA, CA), AU), 2.3), ((UA, (CA, CC), AU), 2.3), ((UA, (CA, CG), AU), 2.3), ((UA, (CA, CU), AU), 2.3), ((UA, (CA, GA), AU), 1.4), ((UA, (CA, GC), AU), 1.9), ((UA, (CA, GG), AU), 0.8), ((UA, (CA, GU), AU), 1.9), ((UA, (CA, UA), AU), 2.3), ((UA, (CA, UC), AU), 2.0), ((UA, (CA, UG), AU), 2.3), ((UA, (CA, UU), AU), 1.5),
    ((UA, (CC, AA), AU), 2.8), ((UA, (CC, AC), AU), 2.3), ((UA, (CC, AG), AU), 2.3), ((UA, (CC, AU), AU), 2.3), ((UA, (CC, CA), AU), 2.8), ((UA, (CC, CC), AU), 2.8), ((UA, (CC, CG), AU), 2.8), ((UA, (CC, CU), AU), 2.8), ((UA, (CC, GA), AU), 2.4), ((UA, (CC, GC), AU), 2.3), ((UA, (CC, GG), AU), 1.2), ((UA, (CC, GU), AU), 2.3), ((UA, (CC, UA), AU), 2.8), ((UA, (CC, UC), AU), 2.5), ((UA, (CC, UG), AU), 2.8), ((UA, (CC, UU), AU), 1.9),
    ((UA, (CG, AA), AU), 2.3), ((UA, (CG, AC), AU), 1.9), ((UA, (CG, AG), AU), 1.3), ((UA, (CG, AU), AU), 1.9), ((UA, (CG, CA), AU), 2.3), ((UA, (CG, CC), AU), 2.3), ((UA, (CG, CG), AU), 2.3), ((UA, (CG, CU), AU), 2.3), ((UA, (CG, GA), AU), 1.4), ((UA, (CG, GC), AU), 1.9), ((UA, (CG, GG), AU), 0.8), ((UA, (CG, GU), AU), 1.9), ((UA, (CG, UA), AU), 2.3), ((UA, (CG, UC), AU), 2.0), ((UA, (CG, UG), AU), 2.3), ((UA, (CG, UU), AU), 1.5),
    ((UA, (CU, AA), AU), 2.5), ((UA, (CU, AC), AU), 2.0), ((UA, (CU, AG), AU), 2.0), ((UA, (CU, AU), AU), 2.0), ((UA, (CU, CA), AU), 2.5), ((UA, (CU, CC), AU), 2.5), ((UA, (CU, CG), AU), 2.5), ((UA, (CU, CU), AU), 2.5), ((UA, (CU, GA), AU), 2.1), ((UA, (CU, GC), AU), 2.0), ((UA, (CU, GG), AU), 0.9), ((UA, (CU, GU), AU), 2.0), ((UA, (CU, UA), AU), 2.5), ((UA, (CU, UC), AU), 2.2), ((UA, (CU, UG), AU), 2.5), ((UA, (CU, UU), AU), 1.6),
    ((UA, (GA, AA), AU), 1.7), ((UA, (GA, AC), AU), 1.3), ((UA, (GA, AG), AU), 0.7), ((UA, (GA, AU), AU), 1.3), ((UA, (GA, CA), AU), 1.7), ((UA, (GA, CC), AU), 2.3), ((UA, (GA, CG), AU), 1.7), ((UA, (GA, CU), AU), 2.3), ((UA, (GA, GA), AU), 0.8), ((UA, (GA, GC), AU), 1.3), ((UA, (GA, GG), AU), 1.5), ((UA, (GA, GU), AU), 1.3), ((UA, (GA, UA), AU), 1.7), ((UA, (GA, UC), AU), 2.0), ((UA, (GA, UG), AU), 1.7), ((UA, (GA, UU), AU), 0.9),
    ((UA, (GC, AA), AU), 2.8), ((UA, (GC, AC), AU), 2.3), ((UA, (GC, AG), AU), 1.7), ((UA, (GC, AU), AU), 2.3), ((UA, (GC, CA), AU), 2.8), ((UA, (GC, CC), AU), 2.8), ((UA, (GC, CG), AU), 2.8), ((UA, (GC, CU), AU), 2.8), ((UA, (GC, GA), AU), 1.8), ((UA, (GC, GC), AU), 2.3), ((UA, (GC, GG), AU), 1.2), ((UA, (GC, GU), AU), 2.3), ((UA, (GC, UA), AU), 2.8), ((UA, (GC, UC), AU), 2.5), ((UA, (GC, UG), AU), 2.8), ((UA, (GC, UU), AU), 1.9),
    ((UA, (GG, AA), AU), 1.2), ((UA, (GG, AC), AU), 0.8), ((UA, (GG, AG), AU), 1.5), ((UA, (GG, AU), AU), 0.8), ((UA, (GG, CA), AU), 1.2), ((UA, (GG, CC), AU), 1.2), ((UA, (GG, CG), AU), 1.2), ((UA, (GG, CU), AU), 1.2), ((UA, (GG, GA), AU), 1.6), ((UA, (GG, GC), AU), 0.8), ((UA, (GG, GG), AU), 2.3), ((UA, (GG, GU), AU), 0.8), ((UA, (GG, UA), AU), 1.2), ((UA, (GG, UC), AU), 0.9), ((UA, (GG, UG), AU), 1.2), ((UA, (GG, UU), AU), 1.7),
    ((UA, (GU, AA), AU), 2.8), ((UA, (GU, AC), AU), 2.3), ((UA, (GU, AG), AU), 1.7), ((UA, (GU, AU), AU), 2.3), ((UA, (GU, CA), AU), 2.8), ((UA, (GU, CC), AU), 2.8), ((UA, (GU, CG), AU), 2.8), ((UA, (GU, CU), AU), 2.8), ((UA, (GU, GA), AU), 1.8), ((UA, (GU, GC), AU), 2.3), ((UA, (GU, GG), AU), 1.2), ((UA, (GU, GU), AU), 2.3), ((UA, (GU, UA), AU), 2.8), ((UA, (GU, UC), AU), 2.5), ((UA, (GU, UG), AU), 2.8), ((UA, (GU, UU), AU), 1.9),
    ((UA, (UA, AA), AU), 2.3), ((UA, (UA, AC), AU), 1.9), ((UA, (UA, AG), AU), 1.3), ((UA, (UA, AU), AU), 1.9), ((UA, (UA, CA), AU), 2.3), ((UA, (UA, CC), AU), 2.3), ((UA, (UA, CG), AU), 2.3), ((UA, (UA, CU), AU), 2.3), ((UA, (UA, GA), AU), 1.4), ((UA, (UA, GC), AU), 1.9), ((UA, (UA, GG), AU), 0.8), ((UA, (UA, GU), AU), 1.9), ((UA, (UA, UA), AU), 2.3), ((UA, (UA, UC), AU), 2.0), ((UA, (UA, UG), AU), 2.3), ((UA, (UA, UU), AU), 1.5),
    ((UA, (UC, AA), AU), 2.8), ((UA, (UC, AC), AU), 2.3), ((UA, (UC, AG), AU), 2.3), ((UA, (UC, AU), AU), 2.3), ((UA, (UC, CA), AU), 2.8), ((UA, (UC, CC), AU), 2.8), ((UA, (UC, CG), AU), 2.8), ((UA, (UC, CU), AU), 2.8), ((UA, (UC, GA), AU), 2.4), ((UA, (UC, GC), AU), 2.3), ((UA, (UC, GG), AU), 1.2), ((UA, (UC, GU), AU), 2.3), ((UA, (UC, UA), AU), 2.8), ((UA, (UC, UC), AU), 2.5), ((UA, (UC, UG), AU), 2.8), ((UA, (UC, UU), AU), 1.9),
    ((UA, (UG, AA), AU), 2.3), ((UA, (UG, AC), AU), 1.9), ((UA, (UG, AG), AU), 1.3), ((UA, (UG, AU), AU), 1.9), ((UA, (UG, CA), AU), 2.3), ((UA, (UG, CC), AU), 2.3), ((UA, (UG, CG), AU), 2.3), ((UA, (UG, CU), AU), 2.3), ((UA, (UG, GA), AU), 1.4), ((UA, (UG, GC), AU), 1.9), ((UA, (UG, GG), AU), 0.8), ((UA, (UG, GU), AU), 1.9), ((UA, (UG, UA), AU), 2.3), ((UA, (UG, UC), AU), 2.0), ((UA, (UG, UG), AU), 2.3), ((UA, (UG, UU), AU), 1.5),
    ((UA, (UU, AA), AU), 2.5), ((UA, (UU, AC), AU), 1.5), ((UA, (UU, AG), AU), 0.9), ((UA, (UU, AU), AU), 1.5), ((UA, (UU, CA), AU), 1.9), ((UA, (UU, CC), AU), 1.9), ((UA, (UU, CG), AU), 1.9), ((UA, (UU, CU), AU), 1.9), ((UA, (UU, GA), AU), 1.0), ((UA, (UU, GC), AU), 1.5), ((UA, (UU, GG), AU), 1.7), ((UA, (UU, GU), AU), 1.5), ((UA, (UU, UA), AU), 1.9), ((UA, (UU, UC), AU), 1.6), ((UA, (UU, UG), AU), 1.9), ((UA, (UU, UU), AU), 1.1),
    // For internal loops between the base pairs "UA" and "CG".
    ((UA, (AA, AA), CG), 2.1), ((UA, (AA, AC), CG), 1.9), ((UA, (AA, AG), CG), 0.1), ((UA, (AA, AU), CG), 1.9), ((UA, (AA, CA), CG), 1.8), ((UA, (AA, CC), CG), 1.9), ((UA, (AA, CG), CG), 1.8), ((UA, (AA, CU), CG), 1.9), ((UA, (AA, GA), CG), 0.7), ((UA, (AA, GC), CG), 1.9), ((UA, (AA, GG), CG), 0.5), ((UA, (AA, GU), CG), 1.9), ((UA, (AA, UA), CG), 1.8), ((UA, (AA, UC), CG), 1.9), ((UA, (AA, UG), CG), 1.8), ((UA, (AA, UU), CG), 1.7),
    ((UA, (AC, AA), CG), 2.1), ((UA, (AC, AC), CG), 1.9), ((UA, (AC, AG), CG), 0.1), ((UA, (AC, AU), CG), 1.9), ((UA, (AC, CA), CG), 1.8), ((UA, (AC, CC), CG), 1.9), ((UA, (AC, CG), CG), 1.8), ((UA, (AC, CU), CG), 1.9), ((UA, (AC, GA), CG), 0.7), ((UA, (AC, GC), CG), 1.9), ((UA, (AC, GG), CG), 0.5), ((UA, (AC, GU), CG), 1.9), ((UA, (AC, UA), CG), 1.8), ((UA, (AC, UC), CG), 1.9), ((UA, (AC, UG), CG), 1.8), ((UA, (AC, UU), CG), 1.1),
    ((UA, (AG, AA), CG), 1.2), ((UA, (AG, AC), CG), 0.9), ((UA, (AG, AG), CG), -0.8), ((UA, (AG, AU), CG), 0.9), ((UA, (AG, CA), CG), 0.9), ((UA, (AG, CC), CG), 1.5), ((UA, (AG, CG), CG), 0.9), ((UA, (AG, CU), CG), 1.5), ((UA, (AG, GA), CG), -0.2), ((UA, (AG, GC), CG), 0.9), ((UA, (AG, GG), CG), 0.8), ((UA, (AG, GU), CG), 0.9), ((UA, (AG, UA), CG), 0.9), ((UA, (AG, UC), CG), 1.6), ((UA, (AG, UG), CG), 0.9), ((UA, (AG, UU), CG), 0.2),
    ((UA, (AU, AA), CG), 2.1), ((UA, (AU, AC), CG), 1.9), ((UA, (AU, AG), CG), 0.1), ((UA, (AU, AU), CG), 1.9), ((UA, (AU, CA), CG), 1.8), ((UA, (AU, CC), CG), 1.9), ((UA, (AU, CG), CG), 1.8), ((UA, (AU, CU), CG), 1.9), ((UA, (AU, GA), CG), 0.7), ((UA, (AU, GC), CG), 1.9), ((UA, (AU, GG), CG), 0.5), ((UA, (AU, GU), CG), 1.9), ((UA, (AU, UA), CG), 1.8), ((UA, (AU, UC), CG), 1.9), ((UA, (AU, UG), CG), 1.8), ((UA, (AU, UU), CG), 1.1),
    ((UA, (CA, AA), CG), 1.7), ((UA, (CA, AC), CG), 1.4), ((UA, (CA, AG), CG), -0.3), ((UA, (CA, AU), CG), 1.4), ((UA, (CA, CA), CG), 1.4), ((UA, (CA, CC), CG), 1.4), ((UA, (CA, CG), CG), 1.4), ((UA, (CA, CU), CG), 1.4), ((UA, (CA, GA), CG), 0.3), ((UA, (CA, GC), CG), 1.4), ((UA, (CA, GG), CG), 0.0), ((UA, (CA, GU), CG), 1.4), ((UA, (CA, UA), CG), 1.4), ((UA, (CA, UC), CG), 1.5), ((UA, (CA, UG), CG), 1.4), ((UA, (CA, UU), CG), 0.7),
    ((UA, (CC, AA), CG), 2.1), ((UA, (CC, AC), CG), 1.9), ((UA, (CC, AG), CG), 0.7), ((UA, (CC, AU), CG), 1.9), ((UA, (CC, CA), CG), 1.8), ((UA, (CC, CC), CG), 1.9), ((UA, (CC, CG), CG), 1.8), ((UA, (CC, CU), CG), 1.9), ((UA, (CC, GA), CG), 1.3), ((UA, (CC, GC), CG), 1.9), ((UA, (CC, GG), CG), 0.5), ((UA, (CC, GU), CG), 1.9), ((UA, (CC, UA), CG), 1.8), ((UA, (CC, UC), CG), 1.9), ((UA, (CC, UG), CG), 1.8), ((UA, (CC, UU), CG), 1.1),
    ((UA, (CG, AA), CG), 1.7), ((UA, (CG, AC), CG), 1.4), ((UA, (CG, AG), CG), -0.3), ((UA, (CG, AU), CG), 1.4), ((UA, (CG, CA), CG), 1.4), ((UA, (CG, CC), CG), 1.4), ((UA, (CG, CG), CG), 1.4), ((UA, (CG, CU), CG), 1.4), ((UA, (CG, GA), CG), 0.3), ((UA, (CG, GC), CG), 1.4), ((UA, (CG, GG), CG), 0.0), ((UA, (CG, GU), CG), 1.4), ((UA, (CG, UA), CG), 1.4), ((UA, (CG, UC), CG), 1.5), ((UA, (CG, UG), CG), 1.4), ((UA, (CG, UU), CG), 0.7),
    ((UA, (CU, AA), CG), 1.8), ((UA, (CU, AC), CG), 1.6), ((UA, (CU, AG), CG), 0.4), ((UA, (CU, AU), CG), 1.6), ((UA, (CU, CA), CG), 1.5), ((UA, (CU, CC), CG), 1.6), ((UA, (CU, CG), CG), 1.5), ((UA, (CU, CU), CG), 1.6), ((UA, (CU, GA), CG), 1.0), ((UA, (CU, GC), CG), 1.6), ((UA, (CU, GG), CG), 0.2), ((UA, (CU, GU), CG), 1.6), ((UA, (CU, UA), CG), 1.5), ((UA, (CU, UC), CG), 1.6), ((UA, (CU, UG), CG), 1.5), ((UA, (CU, UU), CG), 0.8),
    ((UA, (GA, AA), CG), 1.1), ((UA, (GA, AC), CG), 0.8), ((UA, (GA, AG), CG), -0.9), ((UA, (GA, AU), CG), 0.8), ((UA, (GA, CA), CG), 0.8), ((UA, (GA, CC), CG), 1.4), ((UA, (GA, CG), CG), 0.8), ((UA, (GA, CU), CG), 1.4), ((UA, (GA, GA), CG), -0.3), ((UA, (GA, GC), CG), 0.8), ((UA, (GA, GG), CG), 0.7), ((UA, (GA, GU), CG), 0.8), ((UA, (GA, UA), CG), 0.8), ((UA, (GA, UC), CG), 1.5), ((UA, (GA, UG), CG), 0.8), ((UA, (GA, UU), CG), 0.1),
    ((UA, (GC, AA), CG), 2.1), ((UA, (GC, AC), CG), 1.9), ((UA, (GC, AG), CG), 0.1), ((UA, (GC, AU), CG), 1.9), ((UA, (GC, CA), CG), 1.8), ((UA, (GC, CC), CG), 1.9), ((UA, (GC, CG), CG), 1.8), ((UA, (GC, CU), CG), 1.9), ((UA, (GC, GA), CG), 0.7), ((UA, (GC, GC), CG), 1.9), ((UA, (GC, GG), CG), 0.5), ((UA, (GC, GU), CG), 1.9), ((UA, (GC, UA), CG), 1.8), ((UA, (GC, UC), CG), 1.9), ((UA, (GC, UG), CG), 1.8), ((UA, (GC, UU), CG), 1.1),
    ((UA, (GG, AA), CG), 0.6), ((UA, (GG, AC), CG), 0.3), ((UA, (GG, AG), CG), -0.1), ((UA, (GG, AU), CG), 0.3), ((UA, (GG, CA), CG), 0.3), ((UA, (GG, CC), CG), 0.3), ((UA, (GG, CG), CG), 0.3), ((UA, (GG, CU), CG), 0.3), ((UA, (GG, GA), CG), 0.5), ((UA, (GG, GC), CG), 0.3), ((UA, (GG, GG), CG), 1.5), ((UA, (GG, GU), CG), 0.3), ((UA, (GG, UA), CG), 0.3), ((UA, (GG, UC), CG), 0.4), ((UA, (GG, UG), CG), 0.3), ((UA, (GG, UU), CG), 0.9),
    ((UA, (GU, AA), CG), 2.1), ((UA, (GU, AC), CG), 1.9), ((UA, (GU, AG), CG), 0.1), ((UA, (GU, AU), CG), 1.9), ((UA, (GU, CA), CG), 1.8), ((UA, (GU, CC), CG), 1.9), ((UA, (GU, CG), CG), 1.8), ((UA, (GU, CU), CG), 1.9), ((UA, (GU, GA), CG), 0.7), ((UA, (GU, GC), CG), 1.9), ((UA, (GU, GG), CG), 0.5), ((UA, (GU, GU), CG), 1.9), ((UA, (GU, UA), CG), 1.8), ((UA, (GU, UC), CG), 1.9), ((UA, (GU, UG), CG), 1.8), ((UA, (GU, UU), CG), 1.1),
    ((UA, (UA, AA), CG), 1.7), ((UA, (UA, AC), CG), 1.4), ((UA, (UA, AG), CG), -0.3), ((UA, (UA, AU), CG), 1.4), ((UA, (UA, CA), CG), 1.4), ((UA, (UA, CC), CG), 1.4), ((UA, (UA, CG), CG), 1.4), ((UA, (UA, CU), CG), 1.4), ((UA, (UA, GA), CG), 0.3), ((UA, (UA, GC), CG), 1.4), ((UA, (UA, GG), CG), 0.0), ((UA, (UA, GU), CG), 1.4), ((UA, (UA, UA), CG), 1.4), ((UA, (UA, UC), CG), 1.5), ((UA, (UA, UG), CG), 1.4), ((UA, (UA, UU), CG), 0.7),
    ((UA, (UC, AA), CG), 2.1), ((UA, (UC, AC), CG), 1.9), ((UA, (UC, AG), CG), 0.7), ((UA, (UC, AU), CG), 1.9), ((UA, (UC, CA), CG), 1.8), ((UA, (UC, CC), CG), 1.9), ((UA, (UC, CG), CG), 1.8), ((UA, (UC, CU), CG), 1.9), ((UA, (UC, GA), CG), 1.3), ((UA, (UC, GC), CG), 1.9), ((UA, (UC, GG), CG), 0.5), ((UA, (UC, GU), CG), 1.9), ((UA, (UC, UA), CG), 1.8), ((UA, (UC, UC), CG), 1.9), ((UA, (UC, UG), CG), 1.8), ((UA, (UC, UU), CG), 1.1),
    ((UA, (UG, AA), CG), 1.7), ((UA, (UG, AC), CG), 1.4), ((UA, (UG, AG), CG), -0.3), ((UA, (UG, AU), CG), 1.4), ((UA, (UG, CA), CG), 1.4), ((UA, (UG, CC), CG), 1.4), ((UA, (UG, CG), CG), 1.4), ((UA, (UG, CU), CG), 1.4), ((UA, (UG, GA), CG), 0.3), ((UA, (UG, GC), CG), 1.4), ((UA, (UG, GG), CG), 0.0), ((UA, (UG, GU), CG), 1.4), ((UA, (UG, UA), CG), 1.4), ((UA, (UG, UC), CG), 1.5), ((UA, (UG, UG), CG), 1.4), ((UA, (UG, UU), CG), 0.7),
    ((UA, (UU, AA), CG), 1.9), ((UA, (UU, AC), CG), 1.0), ((UA, (UU, AG), CG), -0.7), ((UA, (UU, AU), CG), 1.0), ((UA, (UU, CA), CG), 1.0), ((UA, (UU, CC), CG), 1.0), ((UA, (UU, CG), CG), 1.0), ((UA, (UU, CU), CG), 1.0), ((UA, (UU, GA), CG), -0.1), ((UA, (UU, GC), CG), 1.0), ((UA, (UU, GG), CG), 0.9), ((UA, (UU, GU), CG), 1.0), ((UA, (UU, UA), CG), 1.0), ((UA, (UU, UC), CG), 1.1), ((UA, (UU, UG), CG), 1.0), ((UA, (UU, UU), CG), 0.3),
    // For internal loops between the base pairs "UA" and "GC".
    ((UA, (AA, AA), GC), 2.0), ((UA, (AA, AC), GC), 1.9), ((UA, (AA, AG), GC), 1.0), ((UA, (AA, AU), GC), 1.9), ((UA, (AA, CA), GC), 2.4), ((UA, (AA, CC), GC), 2.2), ((UA, (AA, CG), GC), 2.4), ((UA, (AA, CU), GC), 2.1), ((UA, (AA, GA), GC), 1.0), ((UA, (AA, GC), GC), 1.9), ((UA, (AA, GG), GC), 0.5), ((UA, (AA, GU), GC), 1.9), ((UA, (AA, UA), GC), 2.4), ((UA, (AA, UC), GC), 2.1), ((UA, (AA, UG), GC), 2.4), ((UA, (AA, UU), GC), 1.8),
    ((UA, (AC, AA), GC), 2.0), ((UA, (AC, AC), GC), 1.9), ((UA, (AC, AG), GC), 1.0), ((UA, (AC, AU), GC), 1.9), ((UA, (AC, CA), GC), 2.4), ((UA, (AC, CC), GC), 2.2), ((UA, (AC, CG), GC), 2.4), ((UA, (AC, CU), GC), 2.1), ((UA, (AC, GA), GC), 1.0), ((UA, (AC, GC), GC), 1.9), ((UA, (AC, GG), GC), 0.5), ((UA, (AC, GU), GC), 1.9), ((UA, (AC, UA), GC), 2.4), ((UA, (AC, UC), GC), 2.1), ((UA, (AC, UG), GC), 2.4), ((UA, (AC, UU), GC), 1.2),
    ((UA, (AG, AA), GC), 1.0), ((UA, (AG, AC), GC), 1.0), ((UA, (AG, AG), GC), 0.1), ((UA, (AG, AU), GC), 1.0), ((UA, (AG, CA), GC), 1.4), ((UA, (AG, CC), GC), 1.9), ((UA, (AG, CG), GC), 1.4), ((UA, (AG, CU), GC), 1.7), ((UA, (AG, GA), GC), 0.1), ((UA, (AG, GC), GC), 1.0), ((UA, (AG, GG), GC), 0.8), ((UA, (AG, GU), GC), 1.0), ((UA, (AG, UA), GC), 1.4), ((UA, (AG, UC), GC), 1.7), ((UA, (AG, UG), GC), 1.4), ((UA, (AG, UU), GC), 0.2),
    ((UA, (AU, AA), GC), 2.0), ((UA, (AU, AC), GC), 1.9), ((UA, (AU, AG), GC), 1.0), ((UA, (AU, AU), GC), 1.9), ((UA, (AU, CA), GC), 2.4), ((UA, (AU, CC), GC), 2.2), ((UA, (AU, CG), GC), 2.4), ((UA, (AU, CU), GC), 2.1), ((UA, (AU, GA), GC), 1.0), ((UA, (AU, GC), GC), 1.9), ((UA, (AU, GG), GC), 0.5), ((UA, (AU, GU), GC), 1.9), ((UA, (AU, UA), GC), 2.4), ((UA, (AU, UC), GC), 2.1), ((UA, (AU, UG), GC), 2.4), ((UA, (AU, UU), GC), 1.2),
    ((UA, (CA, AA), GC), 1.5), ((UA, (CA, AC), GC), 1.5), ((UA, (CA, AG), GC), 0.6), ((UA, (CA, AU), GC), 1.5), ((UA, (CA, CA), GC), 1.9), ((UA, (CA, CC), GC), 1.8), ((UA, (CA, CG), GC), 1.9), ((UA, (CA, CU), GC), 1.6), ((UA, (CA, GA), GC), 0.6), ((UA, (CA, GC), GC), 1.5), ((UA, (CA, GG), GC), 0.0), ((UA, (CA, GU), GC), 1.5), ((UA, (CA, UA), GC), 1.9), ((UA, (CA, UC), GC), 1.6), ((UA, (CA, UG), GC), 1.9), ((UA, (CA, UU), GC), 0.7),
    ((UA, (CC, AA), GC), 2.0), ((UA, (CC, AC), GC), 1.9), ((UA, (CC, AG), GC), 1.6), ((UA, (CC, AU), GC), 1.9), ((UA, (CC, CA), GC), 2.4), ((UA, (CC, CC), GC), 2.2), ((UA, (CC, CG), GC), 2.4), ((UA, (CC, CU), GC), 2.1), ((UA, (CC, GA), GC), 1.6), ((UA, (CC, GC), GC), 1.9), ((UA, (CC, GG), GC), 0.5), ((UA, (CC, GU), GC), 1.9), ((UA, (CC, UA), GC), 2.4), ((UA, (CC, UC), GC), 2.1), ((UA, (CC, UG), GC), 2.4), ((UA, (CC, UU), GC), 1.2),
    ((UA, (CG, AA), GC), 1.5), ((UA, (CG, AC), GC), 1.5), ((UA, (CG, AG), GC), 0.6), ((UA, (CG, AU), GC), 1.5), ((UA, (CG, CA), GC), 1.9), ((UA, (CG, CC), GC), 1.8), ((UA, (CG, CG), GC), 1.9), ((UA, (CG, CU), GC), 1.6), ((UA, (CG, GA), GC), 0.6), ((UA, (CG, GC), GC), 1.5), ((UA, (CG, GG), GC), 0.0), ((UA, (CG, GU), GC), 1.5), ((UA, (CG, UA), GC), 1.9), ((UA, (CG, UC), GC), 1.6), ((UA, (CG, UG), GC), 1.9), ((UA, (CG, UU), GC), 0.7),
    ((UA, (CU, AA), GC), 1.7), ((UA, (CU, AC), GC), 1.6), ((UA, (CU, AG), GC), 1.3), ((UA, (CU, AU), GC), 1.6), ((UA, (CU, CA), GC), 2.1), ((UA, (CU, CC), GC), 1.9), ((UA, (CU, CG), GC), 2.1), ((UA, (CU, CU), GC), 1.8), ((UA, (CU, GA), GC), 1.3), ((UA, (CU, GC), GC), 1.6), ((UA, (CU, GG), GC), 0.2), ((UA, (CU, GU), GC), 1.6), ((UA, (CU, UA), GC), 2.1), ((UA, (CU, UC), GC), 1.8), ((UA, (CU, UG), GC), 2.1), ((UA, (CU, UU), GC), 0.9),
    ((UA, (GA, AA), GC), 0.9), ((UA, (GA, AC), GC), 0.9), ((UA, (GA, AG), GC), 0.0), ((UA, (GA, AU), GC), 0.9), ((UA, (GA, CA), GC), 1.3), ((UA, (GA, CC), GC), 1.8), ((UA, (GA, CG), GC), 1.3), ((UA, (GA, CU), GC), 1.6), ((UA, (GA, GA), GC), 0.0), ((UA, (GA, GC), GC), 0.9), ((UA, (GA, GG), GC), 0.7), ((UA, (GA, GU), GC), 0.9), ((UA, (GA, UA), GC), 1.3), ((UA, (GA, UC), GC), 1.6), ((UA, (GA, UG), GC), 1.3), ((UA, (GA, UU), GC), 0.1),
    ((UA, (GC, AA), GC), 2.0), ((UA, (GC, AC), GC), 1.9), ((UA, (GC, AG), GC), 1.0), ((UA, (GC, AU), GC), 1.9), ((UA, (GC, CA), GC), 2.4), ((UA, (GC, CC), GC), 2.2), ((UA, (GC, CG), GC), 2.4), ((UA, (GC, CU), GC), 2.1), ((UA, (GC, GA), GC), 1.0), ((UA, (GC, GC), GC), 1.9), ((UA, (GC, GG), GC), 0.5), ((UA, (GC, GU), GC), 1.9), ((UA, (GC, UA), GC), 2.4), ((UA, (GC, UC), GC), 2.1), ((UA, (GC, UG), GC), 2.4), ((UA, (GC, UU), GC), 1.2),
    ((UA, (GG, AA), GC), 0.4), ((UA, (GG, AC), GC), 0.4), ((UA, (GG, AG), GC), 0.8), ((UA, (GG, AU), GC), 0.4), ((UA, (GG, CA), GC), 0.8), ((UA, (GG, CC), GC), 0.7), ((UA, (GG, CG), GC), 0.8), ((UA, (GG, CU), GC), 0.5), ((UA, (GG, GA), GC), 0.8), ((UA, (GG, GC), GC), 0.4), ((UA, (GG, GG), GC), 1.5), ((UA, (GG, GU), GC), 0.4), ((UA, (GG, UA), GC), 0.8), ((UA, (GG, UC), GC), 0.5), ((UA, (GG, UG), GC), 0.8), ((UA, (GG, UU), GC), 0.9),
    ((UA, (GU, AA), GC), 2.0), ((UA, (GU, AC), GC), 1.9), ((UA, (GU, AG), GC), 1.0), ((UA, (GU, AU), GC), 1.9), ((UA, (GU, CA), GC), 2.4), ((UA, (GU, CC), GC), 2.2), ((UA, (GU, CG), GC), 2.4), ((UA, (GU, CU), GC), 2.1), ((UA, (GU, GA), GC), 1.0), ((UA, (GU, GC), GC), 1.9), ((UA, (GU, GG), GC), 0.5), ((UA, (GU, GU), GC), 1.9), ((UA, (GU, UA), GC), 2.4), ((UA, (GU, UC), GC), 2.1), ((UA, (GU, UG), GC), 2.4), ((UA, (GU, UU), GC), 1.2),
    ((UA, (UA, AA), GC), 1.5), ((UA, (UA, AC), GC), 1.5), ((UA, (UA, AG), GC), 0.6), ((UA, (UA, AU), GC), 1.5), ((UA, (UA, CA), GC), 1.9), ((UA, (UA, CC), GC), 1.8), ((UA, (UA, CG), GC), 1.9), ((UA, (UA, CU), GC), 1.6), ((UA, (UA, GA), GC), 0.6), ((UA, (UA, GC), GC), 1.5), ((UA, (UA, GG), GC), 0.0), ((UA, (UA, GU), GC), 1.5), ((UA, (UA, UA), GC), 1.9), ((UA, (UA, UC), GC), 1.6), ((UA, (UA, UG), GC), 1.9), ((UA, (UA, UU), GC), 0.7),
    ((UA, (UC, AA), GC), 2.0), ((UA, (UC, AC), GC), 1.9), ((UA, (UC, AG), GC), 1.6), ((UA, (UC, AU), GC), 1.9), ((UA, (UC, CA), GC), 2.4), ((UA, (UC, CC), GC), 2.2), ((UA, (UC, CG), GC), 2.4), ((UA, (UC, CU), GC), 2.1), ((UA, (UC, GA), GC), 1.6), ((UA, (UC, GC), GC), 1.9), ((UA, (UC, GG), GC), 0.5), ((UA, (UC, GU), GC), 1.9), ((UA, (UC, UA), GC), 2.4), ((UA, (UC, UC), GC), 2.1), ((UA, (UC, UG), GC), 2.4), ((UA, (UC, UU), GC), 1.2),
    ((UA, (UG, AA), GC), 1.5), ((UA, (UG, AC), GC), 1.5), ((UA, (UG, AG), GC), 0.6), ((UA, (UG, AU), GC), 1.5), ((UA, (UG, CA), GC), 1.9), ((UA, (UG, CC), GC), 1.8), ((UA, (UG, CG), GC), 1.9), ((UA, (UG, CU), GC), 1.6), ((UA, (UG, GA), GC), 0.6), ((UA, (UG, GC), GC), 1.5), ((UA, (UG, GG), GC), 0.0), ((UA, (UG, GU), GC), 1.5), ((UA, (UG, UA), GC), 1.9), ((UA, (UG, UC), GC), 1.6), ((UA, (UG, UG), GC), 1.9), ((UA, (UG, UU), GC), 0.7),
    ((UA, (UU, AA), GC), 1.7), ((UA, (UU, AC), GC), 1.1), ((UA, (UU, AG), GC), 0.2), ((UA, (UU, AU), GC), 1.1), ((UA, (UU, CA), GC), 1.5), ((UA, (UU, CC), GC), 1.4), ((UA, (UU, CG), GC), 1.5), ((UA, (UU, CU), GC), 1.2), ((UA, (UU, GA), GC), 0.2), ((UA, (UU, GC), GC), 1.1), ((UA, (UU, GG), GC), 0.9), ((UA, (UU, GU), GC), 1.1), ((UA, (UU, UA), GC), 1.5), ((UA, (UU, UC), GC), 1.2), ((UA, (UU, UG), GC), 1.5), ((UA, (UU, UU), GC), 0.3),
    // For internal loops between the base pairs "UA" and "GU".
    ((UA, (AA, AA), GU), 2.4), ((UA, (AA, AC), GU), 2.8), ((UA, (AA, AG), GU), 1.4), ((UA, (AA, AU), GU), 2.8), ((UA, (AA, CA), GU), 2.8), ((UA, (AA, CC), GU), 2.8), ((UA, (AA, CG), GU), 2.8), ((UA, (AA, CU), GU), 2.8), ((UA, (AA, GA), GU), 3.1), ((UA, (AA, GC), GU), 2.8), ((UA, (AA, GG), GU), 1.5), ((UA, (AA, GU), GU), 2.8), ((UA, (AA, UA), GU), 2.8), ((UA, (AA, UC), GU), 2.8), ((UA, (AA, UG), GU), 2.8), ((UA, (AA, UU), GU), 3.4),
    ((UA, (AC, AA), GU), 2.4), ((UA, (AC, AC), GU), 2.8), ((UA, (AC, AG), GU), 1.4), ((UA, (AC, AU), GU), 2.8), ((UA, (AC, CA), GU), 2.8), ((UA, (AC, CC), GU), 2.8), ((UA, (AC, CG), GU), 2.8), ((UA, (AC, CU), GU), 2.8), ((UA, (AC, GA), GU), 3.1), ((UA, (AC, GC), GU), 2.8), ((UA, (AC, GG), GU), 1.5), ((UA, (AC, GU), GU), 2.8), ((UA, (AC, UA), GU), 2.8), ((UA, (AC, UC), GU), 2.8), ((UA, (AC, UG), GU), 2.8), ((UA, (AC, UU), GU), 2.8),
    ((UA, (AG, AA), GU), 1.5), ((UA, (AG, AC), GU), 1.8), ((UA, (AG, AG), GU), 0.5), ((UA, (AG, AU), GU), 1.8), ((UA, (AG, CA), GU), 1.8), ((UA, (AG, CC), GU), 2.4), ((UA, (AG, CG), GU), 1.8), ((UA, (AG, CU), GU), 2.4), ((UA, (AG, GA), GU), 2.1), ((UA, (AG, GC), GU), 1.8), ((UA, (AG, GG), GU), 1.8), ((UA, (AG, GU), GU), 1.8), ((UA, (AG, UA), GU), 1.8), ((UA, (AG, UC), GU), 2.4), ((UA, (AG, UG), GU), 1.8), ((UA, (AG, UU), GU), 1.8),
    ((UA, (AU, AA), GU), 2.4), ((UA, (AU, AC), GU), 2.8), ((UA, (AU, AG), GU), 1.4), ((UA, (AU, AU), GU), 2.8), ((UA, (AU, CA), GU), 2.8), ((UA, (AU, CC), GU), 2.8), ((UA, (AU, CG), GU), 2.8), ((UA, (AU, CU), GU), 2.8), ((UA, (AU, GA), GU), 3.1), ((UA, (AU, GC), GU), 2.8), ((UA, (AU, GG), GU), 1.5), ((UA, (AU, GU), GU), 2.8), ((UA, (AU, UA), GU), 2.8), ((UA, (AU, UC), GU), 2.8), ((UA, (AU, UG), GU), 2.8), ((UA, (AU, UU), GU), 2.8),
    ((UA, (CA, AA), GU), 2.0), ((UA, (CA, AC), GU), 2.3), ((UA, (CA, AG), GU), 1.0), ((UA, (CA, AU), GU), 2.3), ((UA, (CA, CA), GU), 2.3), ((UA, (CA, CC), GU), 2.3), ((UA, (CA, CG), GU), 2.3), ((UA, (CA, CU), GU), 2.3), ((UA, (CA, GA), GU), 2.6), ((UA, (CA, GC), GU), 2.3), ((UA, (CA, GG), GU), 1.0), ((UA, (CA, GU), GU), 2.3), ((UA, (CA, UA), GU), 2.3), ((UA, (CA, UC), GU), 2.3), ((UA, (CA, UG), GU), 2.3), ((UA, (CA, UU), GU), 2.3),
    ((UA, (CC, AA), GU), 2.4), ((UA, (CC, AC), GU), 2.8), ((UA, (CC, AG), GU), 2.0), ((UA, (CC, AU), GU), 2.8), ((UA, (CC, CA), GU), 2.8), ((UA, (CC, CC), GU), 2.8), ((UA, (CC, CG), GU), 2.8), ((UA, (CC, CU), GU), 2.8), ((UA, (CC, GA), GU), 3.7), ((UA, (CC, GC), GU), 2.8), ((UA, (CC, GG), GU), 1.5), ((UA, (CC, GU), GU), 2.8), ((UA, (CC, UA), GU), 2.8), ((UA, (CC, UC), GU), 2.8), ((UA, (CC, UG), GU), 2.8), ((UA, (CC, UU), GU), 2.8),
    ((UA, (CG, AA), GU), 2.0), ((UA, (CG, AC), GU), 2.3), ((UA, (CG, AG), GU), 1.0), ((UA, (CG, AU), GU), 2.3), ((UA, (CG, CA), GU), 2.3), ((UA, (CG, CC), GU), 2.3), ((UA, (CG, CG), GU), 2.3), ((UA, (CG, CU), GU), 2.3), ((UA, (CG, GA), GU), 2.6), ((UA, (CG, GC), GU), 2.3), ((UA, (CG, GG), GU), 1.0), ((UA, (CG, GU), GU), 2.3), ((UA, (CG, UA), GU), 2.3), ((UA, (CG, UC), GU), 2.3), ((UA, (CG, UG), GU), 2.3), ((UA, (CG, UU), GU), 2.3),
    ((UA, (CU, AA), GU), 2.1), ((UA, (CU, AC), GU), 2.5), ((UA, (CU, AG), GU), 1.7), ((UA, (CU, AU), GU), 2.5), ((UA, (CU, CA), GU), 2.5), ((UA, (CU, CC), GU), 2.5), ((UA, (CU, CG), GU), 2.5), ((UA, (CU, CU), GU), 2.5), ((UA, (CU, GA), GU), 3.4), ((UA, (CU, GC), GU), 2.5), ((UA, (CU, GG), GU), 1.2), ((UA, (CU, GU), GU), 2.5), ((UA, (CU, UA), GU), 2.5), ((UA, (CU, UC), GU), 2.5), ((UA, (CU, UG), GU), 2.5), ((UA, (CU, UU), GU), 2.5),
    ((UA, (GA, AA), GU), 1.4), ((UA, (GA, AC), GU), 1.7), ((UA, (GA, AG), GU), 0.4), ((UA, (GA, AU), GU), 1.7), ((UA, (GA, CA), GU), 1.7), ((UA, (GA, CC), GU), 2.3), ((UA, (GA, CG), GU), 1.7), ((UA, (GA, CU), GU), 2.3), ((UA, (GA, GA), GU), 2.0), ((UA, (GA, GC), GU), 1.7), ((UA, (GA, GG), GU), 1.7), ((UA, (GA, GU), GU), 1.7), ((UA, (GA, UA), GU), 1.7), ((UA, (GA, UC), GU), 2.3), ((UA, (GA, UG), GU), 1.7), ((UA, (GA, UU), GU), 1.7),
    ((UA, (GC, AA), GU), 2.4), ((UA, (GC, AC), GU), 2.8), ((UA, (GC, AG), GU), 1.4), ((UA, (GC, AU), GU), 2.8), ((UA, (GC, CA), GU), 2.8), ((UA, (GC, CC), GU), 2.8), ((UA, (GC, CG), GU), 2.8), ((UA, (GC, CU), GU), 2.8), ((UA, (GC, GA), GU), 3.1), ((UA, (GC, GC), GU), 2.8), ((UA, (GC, GG), GU), 1.5), ((UA, (GC, GU), GU), 2.8), ((UA, (GC, UA), GU), 2.8), ((UA, (GC, UC), GU), 2.8), ((UA, (GC, UG), GU), 2.8), ((UA, (GC, UU), GU), 2.8),
    ((UA, (GG, AA), GU), 0.9), ((UA, (GG, AC), GU), 1.2), ((UA, (GG, AG), GU), 1.2), ((UA, (GG, AU), GU), 1.2), ((UA, (GG, CA), GU), 1.2), ((UA, (GG, CC), GU), 1.2), ((UA, (GG, CG), GU), 1.2), ((UA, (GG, CU), GU), 1.2), ((UA, (GG, GA), GU), 2.8), ((UA, (GG, GC), GU), 1.2), ((UA, (GG, GG), GU), 2.5), ((UA, (GG, GU), GU), 1.2), ((UA, (GG, UA), GU), 1.2), ((UA, (GG, UC), GU), 1.2), ((UA, (GG, UG), GU), 1.2), ((UA, (GG, UU), GU), 2.5),
    ((UA, (GU, AA), GU), 2.4), ((UA, (GU, AC), GU), 2.8), ((UA, (GU, AG), GU), 1.4), ((UA, (GU, AU), GU), 2.8), ((UA, (GU, CA), GU), 2.8), ((UA, (GU, CC), GU), 2.8), ((UA, (GU, CG), GU), 2.8), ((UA, (GU, CU), GU), 2.8), ((UA, (GU, GA), GU), 3.1), ((UA, (GU, GC), GU), 2.8), ((UA, (GU, GG), GU), 1.5), ((UA, (GU, GU), GU), 2.8), ((UA, (GU, UA), GU), 2.8), ((UA, (GU, UC), GU), 2.8), ((UA, (GU, UG), GU), 2.8), ((UA, (GU, UU), GU), 2.8),
    ((UA, (UA, AA), GU), 2.0), ((UA, (UA, AC), GU), 2.3), ((UA, (UA, AG), GU), 1.0), ((UA, (UA, AU), GU), 2.3), ((UA, (UA, CA), GU), 2.3), ((UA, (UA, CC), GU), 2.3), ((UA, (UA, CG), GU), 2.3), ((UA, (UA, CU), GU), 2.3), ((UA, (UA, GA), GU), 2.6), ((UA, (UA, GC), GU), 2.3), ((UA, (UA, GG), GU), 1.0), ((UA, (UA, GU), GU), 2.3), ((UA, (UA, UA), GU), 2.3), ((UA, (UA, UC), GU), 2.3), ((UA, (UA, UG), GU), 2.3), ((UA, (UA, UU), GU), 2.3),
    ((UA, (UC, AA), GU), 2.4), ((UA, (UC, AC), GU), 2.8), ((UA, (UC, AG), GU), 2.0), ((UA, (UC, AU), GU), 2.8), ((UA, (UC, CA), GU), 2.8), ((UA, (UC, CC), GU), 2.8), ((UA, (UC, CG), GU), 2.8), ((UA, (UC, CU), GU), 2.8), ((UA, (UC, GA), GU), 3.7), ((UA, (UC, GC), GU), 2.8), ((UA, (UC, GG), GU), 1.5), ((UA, (UC, GU), GU), 2.8), ((UA, (UC, UA), GU), 2.8), ((UA, (UC, UC), GU), 2.8), ((UA, (UC, UG), GU), 2.8), ((UA, (UC, UU), GU), 2.8),
    ((UA, (UG, AA), GU), 2.0), ((UA, (UG, AC), GU), 2.3), ((UA, (UG, AG), GU), 1.0), ((UA, (UG, AU), GU), 2.3), ((UA, (UG, CA), GU), 2.3), ((UA, (UG, CC), GU), 2.3), ((UA, (UG, CG), GU), 2.3), ((UA, (UG, CU), GU), 2.3), ((UA, (UG, GA), GU), 2.6), ((UA, (UG, GC), GU), 2.3), ((UA, (UG, GG), GU), 1.0), ((UA, (UG, GU), GU), 2.3), ((UA, (UG, UA), GU), 2.3), ((UA, (UG, UC), GU), 2.3), ((UA, (UG, UG), GU), 2.3), ((UA, (UG, UU), GU), 2.3),
    ((UA, (UU, AA), GU), 2.2), ((UA, (UU, AC), GU), 1.9), ((UA, (UU, AG), GU), 0.6), ((UA, (UU, AU), GU), 1.9), ((UA, (UU, CA), GU), 1.9), ((UA, (UU, CC), GU), 1.9), ((UA, (UU, CG), GU), 1.9), ((UA, (UU, CU), GU), 1.9), ((UA, (UU, GA), GU), 2.2), ((UA, (UU, GC), GU), 1.9), ((UA, (UU, GG), GU), 1.9), ((UA, (UU, GU), GU), 1.9), ((UA, (UU, UA), GU), 1.9), ((UA, (UU, UC), GU), 1.9), ((UA, (UU, UG), GU), 1.9), ((UA, (UU, UU), GU), 1.9),
    // For internal loops between the base pairs "UA" and "UA".
    ((UA, (AA, AA), UA), 2.8), ((UA, (AA, AC), UA), 2.5), ((UA, (AA, AG), UA), 1.5), ((UA, (AA, AU), UA), 2.5), ((UA, (AA, CA), UA), 2.6), ((UA, (AA, CC), UA), 2.6), ((UA, (AA, CG), UA), 2.6), ((UA, (AA, CU), UA), 2.6), ((UA, (AA, GA), UA), 2.2), ((UA, (AA, GC), UA), 2.5), ((UA, (AA, GG), UA), 1.0), ((UA, (AA, GU), UA), 2.5), ((UA, (AA, UA), UA), 2.6), ((UA, (AA, UC), UA), 2.6), ((UA, (AA, UG), UA), 2.6), ((UA, (AA, UU), UA), 2.3),
    ((UA, (AC, AA), UA), 2.8), ((UA, (AC, AC), UA), 2.5), ((UA, (AC, AG), UA), 1.5), ((UA, (AC, AU), UA), 2.5), ((UA, (AC, CA), UA), 2.6), ((UA, (AC, CC), UA), 2.6), ((UA, (AC, CG), UA), 2.6), ((UA, (AC, CU), UA), 2.6), ((UA, (AC, GA), UA), 2.2), ((UA, (AC, GC), UA), 2.5), ((UA, (AC, GG), UA), 1.0), ((UA, (AC, GU), UA), 2.5), ((UA, (AC, UA), UA), 2.6), ((UA, (AC, UC), UA), 2.6), ((UA, (AC, UG), UA), 2.6), ((UA, (AC, UU), UA), 1.7),
    ((UA, (AG, AA), UA), 1.8), ((UA, (AG, AC), UA), 1.6), ((UA, (AG, AG), UA), 0.6), ((UA, (AG, AU), UA), 1.6), ((UA, (AG, CA), UA), 1.7), ((UA, (AG, CC), UA), 2.2), ((UA, (AG, CG), UA), 1.7), ((UA, (AG, CU), UA), 2.2), ((UA, (AG, GA), UA), 1.3), ((UA, (AG, GC), UA), 1.6), ((UA, (AG, GG), UA), 1.4), ((UA, (AG, GU), UA), 1.6), ((UA, (AG, UA), UA), 1.7), ((UA, (AG, UC), UA), 2.2), ((UA, (AG, UG), UA), 1.7), ((UA, (AG, UU), UA), 0.7),
    ((UA, (AU, AA), UA), 2.8), ((UA, (AU, AC), UA), 2.5), ((UA, (AU, AG), UA), 1.5), ((UA, (AU, AU), UA), 2.5), ((UA, (AU, CA), UA), 2.6), ((UA, (AU, CC), UA), 2.6), ((UA, (AU, CG), UA), 2.6), ((UA, (AU, CU), UA), 2.6), ((UA, (AU, GA), UA), 2.2), ((UA, (AU, GC), UA), 2.5), ((UA, (AU, GG), UA), 1.0), ((UA, (AU, GU), UA), 2.5), ((UA, (AU, UA), UA), 2.6), ((UA, (AU, UC), UA), 2.6), ((UA, (AU, UG), UA), 2.6), ((UA, (AU, UU), UA), 1.7),
    ((UA, (CA, AA), UA), 2.3), ((UA, (CA, AC), UA), 2.1), ((UA, (CA, AG), UA), 1.1), ((UA, (CA, AU), UA), 2.1), ((UA, (CA, CA), UA), 2.2), ((UA, (CA, CC), UA), 2.1), ((UA, (CA, CG), UA), 2.2), ((UA, (CA, CU), UA), 2.1), ((UA, (CA, GA), UA), 1.8), ((UA, (CA, GC), UA), 2.1), ((UA, (CA, GG), UA), 0.6), ((UA, (CA, GU), UA), 2.1), ((UA, (CA, UA), UA), 2.2), ((UA, (CA, UC), UA), 2.1), ((UA, (CA, UG), UA), 2.2), ((UA, (CA, UU), UA), 1.2),
    ((UA, (CC, AA), UA), 2.8), ((UA, (CC, AC), UA), 2.5), ((UA, (CC, AG), UA), 2.1), ((UA, (CC, AU), UA), 2.5), ((UA, (CC, CA), UA), 2.6), ((UA, (CC, CC), UA), 2.6), ((UA, (CC, CG), UA), 2.6), ((UA, (CC, CU), UA), 2.6), ((UA, (CC, GA), UA), 2.8), ((UA, (CC, GC), UA), 2.5), ((UA, (CC, GG), UA), 1.0), ((UA, (CC, GU), UA), 2.5), ((UA, (CC, UA), UA), 2.6), ((UA, (CC, UC), UA), 2.6), ((UA, (CC, UG), UA), 2.6), ((UA, (CC, UU), UA), 1.7),
    ((UA, (CG, AA), UA), 2.3), ((UA, (CG, AC), UA), 2.1), ((UA, (CG, AG), UA), 1.1), ((UA, (CG, AU), UA), 2.1), ((UA, (CG, CA), UA), 2.2), ((UA, (CG, CC), UA), 2.1), ((UA, (CG, CG), UA), 2.2), ((UA, (CG, CU), UA), 2.1), ((UA, (CG, GA), UA), 1.8), ((UA, (CG, GC), UA), 2.1), ((UA, (CG, GG), UA), 0.6), ((UA, (CG, GU), UA), 2.1), ((UA, (CG, UA), UA), 2.2), ((UA, (CG, UC), UA), 2.1), ((UA, (CG, UG), UA), 2.2), ((UA, (CG, UU), UA), 1.2),
    ((UA, (CU, AA), UA), 2.5), ((UA, (CU, AC), UA), 2.2), ((UA, (CU, AG), UA), 1.8), ((UA, (CU, AU), UA), 2.2), ((UA, (CU, CA), UA), 2.3), ((UA, (CU, CC), UA), 2.3), ((UA, (CU, CG), UA), 2.3), ((UA, (CU, CU), UA), 2.3), ((UA, (CU, GA), UA), 2.5), ((UA, (CU, GC), UA), 2.2), ((UA, (CU, GG), UA), 0.7), ((UA, (CU, GU), UA), 2.2), ((UA, (CU, UA), UA), 2.3), ((UA, (CU, UC), UA), 2.3), ((UA, (CU, UG), UA), 2.3), ((UA, (CU, UU), UA), 1.4),
    ((UA, (GA, AA), UA), 1.7), ((UA, (GA, AC), UA), 1.5), ((UA, (GA, AG), UA), 0.5), ((UA, (GA, AU), UA), 1.5), ((UA, (GA, CA), UA), 1.6), ((UA, (GA, CC), UA), 2.1), ((UA, (GA, CG), UA), 1.6), ((UA, (GA, CU), UA), 2.1), ((UA, (GA, GA), UA), 1.2), ((UA, (GA, GC), UA), 1.5), ((UA, (GA, GG), UA), 1.3), ((UA, (GA, GU), UA), 1.5), ((UA, (GA, UA), UA), 1.6), ((UA, (GA, UC), UA), 2.1), ((UA, (GA, UG), UA), 1.6), ((UA, (GA, UU), UA), 0.6),
    ((UA, (GC, AA), UA), 2.8), ((UA, (GC, AC), UA), 2.5), ((UA, (GC, AG), UA), 1.5), ((UA, (GC, AU), UA), 2.5), ((UA, (GC, CA), UA), 2.6), ((UA, (GC, CC), UA), 2.6), ((UA, (GC, CG), UA), 2.6), ((UA, (GC, CU), UA), 2.6), ((UA, (GC, GA), UA), 2.2), ((UA, (GC, GC), UA), 2.5), ((UA, (GC, GG), UA), 1.0), ((UA, (GC, GU), UA), 2.5), ((UA, (GC, UA), UA), 2.6), ((UA, (GC, UC), UA), 2.6), ((UA, (GC, UG), UA), 2.6), ((UA, (GC, UU), UA), 1.7),
    ((UA, (GG, AA), UA), 1.2), ((UA, (GG, AC), UA), 1.0), ((UA, (GG, AG), UA), 1.3), ((UA, (GG, AU), UA), 1.0), ((UA, (GG, CA), UA), 1.1), ((UA, (GG, CC), UA), 1.0), ((UA, (GG, CG), UA), 1.1), ((UA, (GG, CU), UA), 1.0), ((UA, (GG, GA), UA), 2.0), ((UA, (GG, GC), UA), 1.0), ((UA, (GG, GG), UA), 2.1), ((UA, (GG, GU), UA), 1.0), ((UA, (GG, UA), UA), 1.1), ((UA, (GG, UC), UA), 1.0), ((UA, (GG, UG), UA), 1.1), ((UA, (GG, UU), UA), 1.4),
    ((UA, (GU, AA), UA), 2.8), ((UA, (GU, AC), UA), 2.5), ((UA, (GU, AG), UA), 1.5), ((UA, (GU, AU), UA), 2.5), ((UA, (GU, CA), UA), 2.6), ((UA, (GU, CC), UA), 2.6), ((UA, (GU, CG), UA), 2.6), ((UA, (GU, CU), UA), 2.6), ((UA, (GU, GA), UA), 2.2), ((UA, (GU, GC), UA), 2.5), ((UA, (GU, GG), UA), 1.0), ((UA, (GU, GU), UA), 2.5), ((UA, (GU, UA), UA), 2.6), ((UA, (GU, UC), UA), 2.6), ((UA, (GU, UG), UA), 2.6), ((UA, (GU, UU), UA), 1.7),
    ((UA, (UA, AA), UA), 2.3), ((UA, (UA, AC), UA), 2.1), ((UA, (UA, AG), UA), 1.1), ((UA, (UA, AU), UA), 2.1), ((UA, (UA, CA), UA), 2.2), ((UA, (UA, CC), UA), 2.1), ((UA, (UA, CG), UA), 2.2), ((UA, (UA, CU), UA), 2.1), ((UA, (UA, GA), UA), 1.8), ((UA, (UA, GC), UA), 2.1), ((UA, (UA, GG), UA), 0.6), ((UA, (UA, GU), UA), 2.1), ((UA, (UA, UA), UA), 2.2), ((UA, (UA, UC), UA), 2.1), ((UA, (UA, UG), UA), 2.2), ((UA, (UA, UU), UA), 1.2),
    ((UA, (UC, AA), UA), 2.8), ((UA, (UC, AC), UA), 2.5), ((UA, (UC, AG), UA), 2.1), ((UA, (UC, AU), UA), 2.5), ((UA, (UC, CA), UA), 2.6), ((UA, (UC, CC), UA), 2.6), ((UA, (UC, CG), UA), 2.6), ((UA, (UC, CU), UA), 2.6), ((UA, (UC, GA), UA), 2.8), ((UA, (UC, GC), UA), 2.5), ((UA, (UC, GG), UA), 1.0), ((UA, (UC, GU), UA), 2.5), ((UA, (UC, UA), UA), 2.6), ((UA, (UC, UC), UA), 2.6), ((UA, (UC, UG), UA), 2.6), ((UA, (UC, UU), UA), 1.7),
    ((UA, (UG, AA), UA), 2.3), ((UA, (UG, AC), UA), 2.1), ((UA, (UG, AG), UA), 1.1), ((UA, (UG, AU), UA), 2.1), ((UA, (UG, CA), UA), 2.2), ((UA, (UG, CC), UA), 2.1), ((UA, (UG, CG), UA), 2.2), ((UA, (UG, CU), UA), 2.1), ((UA, (UG, GA), UA), 1.8), ((UA, (UG, GC), UA), 2.1), ((UA, (UG, GG), UA), 0.6), ((UA, (UG, GU), UA), 2.1), ((UA, (UG, UA), UA), 2.2), ((UA, (UG, UC), UA), 2.1), ((UA, (UG, UG), UA), 2.2), ((UA, (UG, UU), UA), 1.2),
    ((UA, (UU, AA), UA), 2.5), ((UA, (UU, AC), UA), 1.7), ((UA, (UU, AG), UA), 0.7), ((UA, (UU, AU), UA), 1.7), ((UA, (UU, CA), UA), 1.8), ((UA, (UU, CC), UA), 1.7), ((UA, (UU, CG), UA), 1.8), ((UA, (UU, CU), UA), 1.7), ((UA, (UU, GA), UA), 1.4), ((UA, (UU, GC), UA), 1.7), ((UA, (UU, GG), UA), 1.5), ((UA, (UU, GU), UA), 1.7), ((UA, (UU, UA), UA), 1.8), ((UA, (UU, UC), UA), 1.7), ((UA, (UU, UG), UA), 1.8), ((UA, (UU, UU), UA), 0.8),
    // For internal loops between the base pairs "UA" and "UG".
    ((UA, (AA, AA), UG), 3.4), ((UA, (AA, AC), UG), 3.1), ((UA, (AA, AG), UG), 2.3), ((UA, (AA, AU), UG), 3.1), ((UA, (AA, CA), UG), 3.1), ((UA, (AA, CC), UG), 3.1), ((UA, (AA, CG), UG), 3.1), ((UA, (AA, CU), UG), 3.1), ((UA, (AA, GA), UG), 2.7), ((UA, (AA, GC), UG), 3.1), ((UA, (AA, GG), UG), 1.8), ((UA, (AA, GU), UG), 3.1), ((UA, (AA, UA), UG), 3.1), ((UA, (AA, UC), UG), 3.1), ((UA, (AA, UG), UG), 3.1), ((UA, (AA, UU), UG), 3.7),
    ((UA, (AC, AA), UG), 3.4), ((UA, (AC, AC), UG), 3.1), ((UA, (AC, AG), UG), 2.3), ((UA, (AC, AU), UG), 3.1), ((UA, (AC, CA), UG), 3.1), ((UA, (AC, CC), UG), 3.1), ((UA, (AC, CG), UG), 3.1), ((UA, (AC, CU), UG), 3.1), ((UA, (AC, GA), UG), 2.7), ((UA, (AC, GC), UG), 3.1), ((UA, (AC, GG), UG), 1.8), ((UA, (AC, GU), UG), 3.1), ((UA, (AC, UA), UG), 3.1), ((UA, (AC, UC), UG), 3.1), ((UA, (AC, UG), UG), 3.1), ((UA, (AC, UU), UG), 3.1),
    ((UA, (AG, AA), UG), 2.5), ((UA, (AG, AC), UG), 2.1), ((UA, (AG, AG), UG), 1.3), ((UA, (AG, AU), UG), 2.1), ((UA, (AG, CA), UG), 2.1), ((UA, (AG, CC), UG), 2.7), ((UA, (AG, CG), UG), 2.1), ((UA, (AG, CU), UG), 2.7), ((UA, (AG, GA), UG), 1.7), ((UA, (AG, GC), UG), 2.1), ((UA, (AG, GG), UG), 2.1), ((UA, (AG, GU), UG), 2.1), ((UA, (AG, UA), UG), 2.1), ((UA, (AG, UC), UG), 2.7), ((UA, (AG, UG), UG), 2.1), ((UA, (AG, UU), UG), 2.1),
    ((UA, (AU, AA), UG), 3.4), ((UA, (AU, AC), UG), 3.1), ((UA, (AU, AG), UG), 2.3), ((UA, (AU, AU), UG), 3.1), ((UA, (AU, CA), UG), 3.1), ((UA, (AU, CC), UG), 3.1), ((UA, (AU, CG), UG), 3.1), ((UA, (AU, CU), UG), 3.1), ((UA, (AU, GA), UG), 2.7), ((UA, (AU, GC), UG), 3.1), ((UA, (AU, GG), UG), 1.8), ((UA, (AU, GU), UG), 3.1), ((UA, (AU, UA), UG), 3.1), ((UA, (AU, UC), UG), 3.1), ((UA, (AU, UG), UG), 3.1), ((UA, (AU, UU), UG), 3.1),
    ((UA, (CA, AA), UG), 3.0), ((UA, (CA, AC), UG), 2.6), ((UA, (CA, AG), UG), 1.8), ((UA, (CA, AU), UG), 2.6), ((UA, (CA, CA), UG), 2.6), ((UA, (CA, CC), UG), 2.6), ((UA, (CA, CG), UG), 2.6), ((UA, (CA, CU), UG), 2.6), ((UA, (CA, GA), UG), 2.2), ((UA, (CA, GC), UG), 2.6), ((UA, (CA, GG), UG), 1.3), ((UA, (CA, GU), UG), 2.6), ((UA, (CA, UA), UG), 2.6), ((UA, (CA, UC), UG), 2.6), ((UA, (CA, UG), UG), 2.6), ((UA, (CA, UU), UG), 2.6),
    ((UA, (CC, AA), UG), 3.4), ((UA, (CC, AC), UG), 3.1), ((UA, (CC, AG), UG), 2.9), ((UA, (CC, AU), UG), 3.1), ((UA, (CC, CA), UG), 3.1), ((UA, (CC, CC), UG), 3.1), ((UA, (CC, CG), UG), 3.1), ((UA, (CC, CU), UG), 3.1), ((UA, (CC, GA), UG), 3.3), ((UA, (CC, GC), UG), 3.1), ((UA, (CC, GG), UG), 1.8), ((UA, (CC, GU), UG), 3.1), ((UA, (CC, UA), UG), 3.1), ((UA, (CC, UC), UG), 3.1), ((UA, (CC, UG), UG), 3.1), ((UA, (CC, UU), UG), 3.1),
    ((UA, (CG, AA), UG), 3.0), ((UA, (CG, AC), UG), 2.6), ((UA, (CG, AG), UG), 1.8), ((UA, (CG, AU), UG), 2.6), ((UA, (CG, CA), UG), 2.6), ((UA, (CG, CC), UG), 2.6), ((UA, (CG, CG), UG), 2.6), ((UA, (CG, CU), UG), 2.6), ((UA, (CG, GA), UG), 2.2), ((UA, (CG, GC), UG), 2.6), ((UA, (CG, GG), UG), 1.3), ((UA, (CG, GU), UG), 2.6), ((UA, (CG, UA), UG), 2.6), ((UA, (CG, UC), UG), 2.6), ((UA, (CG, UG), UG), 2.6), ((UA, (CG, UU), UG), 2.6),
    ((UA, (CU, AA), UG), 3.1), ((UA, (CU, AC), UG), 2.8), ((UA, (CU, AG), UG), 2.6), ((UA, (CU, AU), UG), 2.8), ((UA, (CU, CA), UG), 2.8), ((UA, (CU, CC), UG), 2.8), ((UA, (CU, CG), UG), 2.8), ((UA, (CU, CU), UG), 2.8), ((UA, (CU, GA), UG), 3.0), ((UA, (CU, GC), UG), 2.8), ((UA, (CU, GG), UG), 1.5), ((UA, (CU, GU), UG), 2.8), ((UA, (CU, UA), UG), 2.8), ((UA, (CU, UC), UG), 2.8), ((UA, (CU, UG), UG), 2.8), ((UA, (CU, UU), UG), 2.8),
    ((UA, (GA, AA), UG), 2.4), ((UA, (GA, AC), UG), 2.0), ((UA, (GA, AG), UG), 1.2), ((UA, (GA, AU), UG), 2.0), ((UA, (GA, CA), UG), 2.0), ((UA, (GA, CC), UG), 2.6), ((UA, (GA, CG), UG), 2.0), ((UA, (GA, CU), UG), 2.6), ((UA, (GA, GA), UG), 1.6), ((UA, (GA, GC), UG), 2.0), ((UA, (GA, GG), UG), 2.0), ((UA, (GA, GU), UG), 2.0), ((UA, (GA, UA), UG), 2.0), ((UA, (GA, UC), UG), 2.6), ((UA, (GA, UG), UG), 2.0), ((UA, (GA, UU), UG), 2.0),
    ((UA, (GC, AA), UG), 3.4), ((UA, (GC, AC), UG), 3.1), ((UA, (GC, AG), UG), 2.3), ((UA, (GC, AU), UG), 3.1), ((UA, (GC, CA), UG), 3.1), ((UA, (GC, CC), UG), 3.1), ((UA, (GC, CG), UG), 3.1), ((UA, (GC, CU), UG), 3.1), ((UA, (GC, GA), UG), 2.7), ((UA, (GC, GC), UG), 3.1), ((UA, (GC, GG), UG), 1.8), ((UA, (GC, GU), UG), 3.1), ((UA, (GC, UA), UG), 3.1), ((UA, (GC, UC), UG), 3.1), ((UA, (GC, UG), UG), 3.1), ((UA, (GC, UU), UG), 3.1),
    ((UA, (GG, AA), UG), 1.9), ((UA, (GG, AC), UG), 1.5), ((UA, (GG, AG), UG), 2.0), ((UA, (GG, AU), UG), 1.5), ((UA, (GG, CA), UG), 1.5), ((UA, (GG, CC), UG), 1.5), ((UA, (GG, CG), UG), 1.5), ((UA, (GG, CU), UG), 1.5), ((UA, (GG, GA), UG), 2.4), ((UA, (GG, GC), UG), 1.5), ((UA, (GG, GG), UG), 2.8), ((UA, (GG, GU), UG), 1.5), ((UA, (GG, UA), UG), 1.5), ((UA, (GG, UC), UG), 1.5), ((UA, (GG, UG), UG), 1.5), ((UA, (GG, UU), UG), 2.8),
    ((UA, (GU, AA), UG), 3.4), ((UA, (GU, AC), UG), 3.1), ((UA, (GU, AG), UG), 2.3), ((UA, (GU, AU), UG), 3.1), ((UA, (GU, CA), UG), 3.1), ((UA, (GU, CC), UG), 3.1), ((UA, (GU, CG), UG), 3.1), ((UA, (GU, CU), UG), 3.1), ((UA, (GU, GA), UG), 2.7), ((UA, (GU, GC), UG), 3.1), ((UA, (GU, GG), UG), 1.8), ((UA, (GU, GU), UG), 3.1), ((UA, (GU, UA), UG), 3.1), ((UA, (GU, UC), UG), 3.1), ((UA, (GU, UG), UG), 3.1), ((UA, (GU, UU), UG), 3.1),
    ((UA, (UA, AA), UG), 3.0), ((UA, (UA, AC), UG), 2.6), ((UA, (UA, AG), UG), 1.8), ((UA, (UA, AU), UG), 2.6), ((UA, (UA, CA), UG), 2.6), ((UA, (UA, CC), UG), 2.6), ((UA, (UA, CG), UG), 2.6), ((UA, (UA, CU), UG), 2.6), ((UA, (UA, GA), UG), 2.2), ((UA, (UA, GC), UG), 2.6), ((UA, (UA, GG), UG), 1.3), ((UA, (UA, GU), UG), 2.6), ((UA, (UA, UA), UG), 2.6), ((UA, (UA, UC), UG), 2.6), ((UA, (UA, UG), UG), 2.6), ((UA, (UA, UU), UG), 2.6),
    ((UA, (UC, AA), UG), 3.4), ((UA, (UC, AC), UG), 3.1), ((UA, (UC, AG), UG), 2.9), ((UA, (UC, AU), UG), 3.1), ((UA, (UC, CA), UG), 3.1), ((UA, (UC, CC), UG), 3.1), ((UA, (UC, CG), UG), 3.1), ((UA, (UC, CU), UG), 3.1), ((UA, (UC, GA), UG), 3.3), ((UA, (UC, GC), UG), 3.1), ((UA, (UC, GG), UG), 1.8), ((UA, (UC, GU), UG), 3.1), ((UA, (UC, UA), UG), 3.1), ((UA, (UC, UC), UG), 3.1), ((UA, (UC, UG), UG), 3.1), ((UA, (UC, UU), UG), 3.1),
    ((UA, (UG, AA), UG), 3.0), ((UA, (UG, AC), UG), 2.6), ((UA, (UG, AG), UG), 1.8), ((UA, (UG, AU), UG), 2.6), ((UA, (UG, CA), UG), 2.6), ((UA, (UG, CC), UG), 2.6), ((UA, (UG, CG), UG), 2.6), ((UA, (UG, CU), UG), 2.6), ((UA, (UG, GA), UG), 2.2), ((UA, (UG, GC), UG), 2.6), ((UA, (UG, GG), UG), 1.3), ((UA, (UG, GU), UG), 2.6), ((UA, (UG, UA), UG), 2.6), ((UA, (UG, UC), UG), 2.6), ((UA, (UG, UG), UG), 2.6), ((UA, (UG, UU), UG), 2.6),
    ((UA, (UU, AA), UG), 3.2), ((UA, (UU, AC), UG), 2.2), ((UA, (UU, AG), UG), 1.4), ((UA, (UU, AU), UG), 2.2), ((UA, (UU, CA), UG), 2.2), ((UA, (UU, CC), UG), 2.2), ((UA, (UU, CG), UG), 2.2), ((UA, (UU, CU), UG), 2.2), ((UA, (UU, GA), UG), 1.8), ((UA, (UU, GC), UG), 2.2), ((UA, (UU, GG), UG), 2.2), ((UA, (UU, GU), UG), 2.2), ((UA, (UU, UA), UG), 2.2), ((UA, (UU, UC), UG), 2.2), ((UA, (UU, UG), UG), 2.2), ((UA, (UU, UU), UG), 2.2),
    // For internal loops between the base pairs "UG" and "AU".
    ((UG, (AA, AA), AU), 2.4), ((UG, (AA, AC), AU), 2.0), ((UG, (AA, AG), AU), 1.4), ((UG, (AA, AU), AU), 2.0), ((UG, (AA, CA), AU), 2.4), ((UG, (AA, CC), AU), 2.4), ((UG, (AA, CG), AU), 2.4), ((UG, (AA, CU), AU), 2.4), ((UG, (AA, GA), AU), 1.5), ((UG, (AA, GC), AU), 2.0), ((UG, (AA, GG), AU), 0.9), ((UG, (AA, GU), AU), 2.0), ((UG, (AA, UA), AU), 2.4), ((UG, (AA, UC), AU), 2.1), ((UG, (AA, UG), AU), 2.4), ((UG, (AA, UU), AU), 2.2),
    ((UG, (AC, AA), AU), 2.8), ((UG, (AC, AC), AU), 2.3), ((UG, (AC, AG), AU), 1.7), ((UG, (AC, AU), AU), 2.3), ((UG, (AC, CA), AU), 2.8), ((UG, (AC, CC), AU), 2.8), ((UG, (AC, CG), AU), 2.8), ((UG, (AC, CU), AU), 2.8), ((UG, (AC, GA), AU), 1.8), ((UG, (AC, GC), AU), 2.3), ((UG, (AC, GG), AU), 1.2), ((UG, (AC, GU), AU), 2.3), ((UG, (AC, UA), AU), 2.8), ((UG, (AC, UC), AU), 2.5), ((UG, (AC, UG), AU), 2.8), ((UG, (AC, UU), AU), 1.9),
    ((UG, (AG, AA), AU), 3.1), ((UG, (AG, AC), AU), 2.6), ((UG, (AG, AG), AU), 2.0), ((UG, (AG, AU), AU), 2.6), ((UG, (AG, CA), AU), 3.1), ((UG, (AG, CC), AU), 3.7), ((UG, (AG, CG), AU), 3.1), ((UG, (AG, CU), AU), 3.7), ((UG, (AG, GA), AU), 2.1), ((UG, (AG, GC), AU), 2.6), ((UG, (AG, GG), AU), 2.8), ((UG, (AG, GU), AU), 2.6), ((UG, (AG, UA), AU), 3.1), ((UG, (AG, UC), AU), 3.4), ((UG, (AG, UG), AU), 3.1), ((UG, (AG, UU), AU), 2.2),
    ((UG, (AU, AA), AU), 2.8), ((UG, (AU, AC), AU), 2.3), ((UG, (AU, AG), AU), 1.7), ((UG, (AU, AU), AU), 2.3), ((UG, (AU, CA), AU), 2.8), ((UG, (AU, CC), AU), 2.8), ((UG, (AU, CG), AU), 2.8), ((UG, (AU, CU), AU), 2.8), ((UG, (AU, GA), AU), 1.8), ((UG, (AU, GC), AU), 2.3), ((UG, (AU, GG), AU), 1.2), ((UG, (AU, GU), AU), 2.3), ((UG, (AU, UA), AU), 2.8), ((UG, (AU, UC), AU), 2.5), ((UG, (AU, UG), AU), 2.8), ((UG, (AU, UU), AU), 1.9),
    ((UG, (CA, AA), AU), 2.8), ((UG, (CA, AC), AU), 2.3), ((UG, (CA, AG), AU), 1.7), ((UG, (CA, AU), AU), 2.3), ((UG, (CA, CA), AU), 2.8), ((UG, (CA, CC), AU), 2.8), ((UG, (CA, CG), AU), 2.8), ((UG, (CA, CU), AU), 2.8), ((UG, (CA, GA), AU), 1.8), ((UG, (CA, GC), AU), 2.3), ((UG, (CA, GG), AU), 1.2), ((UG, (CA, GU), AU), 2.3), ((UG, (CA, UA), AU), 2.8), ((UG, (CA, UC), AU), 2.5), ((UG, (CA, UG), AU), 2.8), ((UG, (CA, UU), AU), 1.9),
    ((UG, (CC, AA), AU), 2.8), ((UG, (CC, AC), AU), 2.3), ((UG, (CC, AG), AU), 2.3), ((UG, (CC, AU), AU), 2.3), ((UG, (CC, CA), AU), 2.8), ((UG, (CC, CC), AU), 2.8), ((UG, (CC, CG), AU), 2.8), ((UG, (CC, CU), AU), 2.8), ((UG, (CC, GA), AU), 2.4), ((UG, (CC, GC), AU), 2.3), ((UG, (CC, GG), AU), 1.2), ((UG, (CC, GU), AU), 2.3), ((UG, (CC, UA), AU), 2.8), ((UG, (CC, UC), AU), 2.5), ((UG, (CC, UG), AU), 2.8), ((UG, (CC, UU), AU), 1.9),
    ((UG, (CG, AA), AU), 2.8), ((UG, (CG, AC), AU), 2.3), ((UG, (CG, AG), AU), 1.7), ((UG, (CG, AU), AU), 2.3), ((UG, (CG, CA), AU), 2.8), ((UG, (CG, CC), AU), 2.8), ((UG, (CG, CG), AU), 2.8), ((UG, (CG, CU), AU), 2.8), ((UG, (CG, GA), AU), 1.8), ((UG, (CG, GC), AU), 2.3), ((UG, (CG, GG), AU), 1.2), ((UG, (CG, GU), AU), 2.3), ((UG, (CG, UA), AU), 2.8), ((UG, (CG, UC), AU), 2.5), ((UG, (CG, UG), AU), 2.8), ((UG, (CG, UU), AU), 1.9),
    ((UG, (CU, AA), AU), 2.8), ((UG, (CU, AC), AU), 2.3), ((UG, (CU, AG), AU), 2.3), ((UG, (CU, AU), AU), 2.3), ((UG, (CU, CA), AU), 2.8), ((UG, (CU, CC), AU), 2.8), ((UG, (CU, CG), AU), 2.8), ((UG, (CU, CU), AU), 2.8), ((UG, (CU, GA), AU), 2.4), ((UG, (CU, GC), AU), 2.3), ((UG, (CU, GG), AU), 1.2), ((UG, (CU, GU), AU), 2.3), ((UG, (CU, UA), AU), 2.8), ((UG, (CU, UC), AU), 2.5), ((UG, (CU, UG), AU), 2.8), ((UG, (CU, UU), AU), 1.9),
    ((UG, (GA, AA), AU), 1.4), ((UG, (GA, AC), AU), 1.0), ((UG, (GA, AG), AU), 0.4), ((UG, (GA, AU), AU), 1.0), ((UG, (GA, CA), AU), 1.4), ((UG, (GA, CC), AU), 2.0), ((UG, (GA, CG), AU), 1.4), ((UG, (GA, CU), AU), 2.0), ((UG, (GA, GA), AU), 0.5), ((UG, (GA, GC), AU), 1.0), ((UG, (GA, GG), AU), 1.2), ((UG, (GA, GU), AU), 1.0), ((UG, (GA, UA), AU), 1.4), ((UG, (GA, UC), AU), 1.7), ((UG, (GA, UG), AU), 1.4), ((UG, (GA, UU), AU), 0.6),
    ((UG, (GC, AA), AU), 2.8), ((UG, (GC, AC), AU), 2.3), ((UG, (GC, AG), AU), 1.7), ((UG, (GC, AU), AU), 2.3), ((UG, (GC, CA), AU), 2.8), ((UG, (GC, CC), AU), 2.8), ((UG, (GC, CG), AU), 2.8), ((UG, (GC, CU), AU), 2.8), ((UG, (GC, GA), AU), 1.8), ((UG, (GC, GC), AU), 2.3), ((UG, (GC, GG), AU), 1.2), ((UG, (GC, GU), AU), 2.3), ((UG, (GC, UA), AU), 2.8), ((UG, (GC, UC), AU), 2.5), ((UG, (GC, UG), AU), 2.8), ((UG, (GC, UU), AU), 1.9),
    ((UG, (GG, AA), AU), 1.5), ((UG, (GG, AC), AU), 1.0), ((UG, (GG, AG), AU), 1.7), ((UG, (GG, AU), AU), 1.0), ((UG, (GG, CA), AU), 1.5), ((UG, (GG, CC), AU), 1.5), ((UG, (GG, CG), AU), 1.5), ((UG, (GG, CU), AU), 1.5), ((UG, (GG, GA), AU), 1.8), ((UG, (GG, GC), AU), 1.0), ((UG, (GG, GG), AU), 2.5), ((UG, (GG, GU), AU), 1.0), ((UG, (GG, UA), AU), 1.5), ((UG, (GG, UC), AU), 1.2), ((UG, (GG, UG), AU), 1.5), ((UG, (GG, UU), AU), 1.9),
    ((UG, (GU, AA), AU), 2.8), ((UG, (GU, AC), AU), 2.3), ((UG, (GU, AG), AU), 1.7), ((UG, (GU, AU), AU), 2.3), ((UG, (GU, CA), AU), 2.8), ((UG, (GU, CC), AU), 2.8), ((UG, (GU, CG), AU), 2.8), ((UG, (GU, CU), AU), 2.8), ((UG, (GU, GA), AU), 1.8), ((UG, (GU, GC), AU), 2.3), ((UG, (GU, GG), AU), 1.2), ((UG, (GU, GU), AU), 2.3), ((UG, (GU, UA), AU), 2.8), ((UG, (GU, UC), AU), 2.5), ((UG, (GU, UG), AU), 2.8), ((UG, (GU, UU), AU), 1.9),
    ((UG, (UA, AA), AU), 2.8), ((UG, (UA, AC), AU), 2.3), ((UG, (UA, AG), AU), 1.7), ((UG, (UA, AU), AU), 2.3), ((UG, (UA, CA), AU), 2.8), ((UG, (UA, CC), AU), 2.8), ((UG, (UA, CG), AU), 2.8), ((UG, (UA, CU), AU), 2.8), ((UG, (UA, GA), AU), 1.8), ((UG, (UA, GC), AU), 2.3), ((UG, (UA, GG), AU), 1.2), ((UG, (UA, GU), AU), 2.3), ((UG, (UA, UA), AU), 2.8), ((UG, (UA, UC), AU), 2.5), ((UG, (UA, UG), AU), 2.8), ((UG, (UA, UU), AU), 1.9),
    ((UG, (UC, AA), AU), 2.8), ((UG, (UC, AC), AU), 2.3), ((UG, (UC, AG), AU), 2.3), ((UG, (UC, AU), AU), 2.3), ((UG, (UC, CA), AU), 2.8), ((UG, (UC, CC), AU), 2.8), ((UG, (UC, CG), AU), 2.8), ((UG, (UC, CU), AU), 2.8), ((UG, (UC, GA), AU), 2.4), ((UG, (UC, GC), AU), 2.3), ((UG, (UC, GG), AU), 1.2), ((UG, (UC, GU), AU), 2.3), ((UG, (UC, UA), AU), 2.8), ((UG, (UC, UC), AU), 2.5), ((UG, (UC, UG), AU), 2.8), ((UG, (UC, UU), AU), 1.9),
    ((UG, (UG, AA), AU), 2.8), ((UG, (UG, AC), AU), 2.3), ((UG, (UG, AG), AU), 1.7), ((UG, (UG, AU), AU), 2.3), ((UG, (UG, CA), AU), 2.8), ((UG, (UG, CC), AU), 2.8), ((UG, (UG, CG), AU), 2.8), ((UG, (UG, CU), AU), 2.8), ((UG, (UG, GA), AU), 1.8), ((UG, (UG, GC), AU), 2.3), ((UG, (UG, GG), AU), 1.2), ((UG, (UG, GU), AU), 2.3), ((UG, (UG, UA), AU), 2.8), ((UG, (UG, UC), AU), 2.5), ((UG, (UG, UG), AU), 2.8), ((UG, (UG, UU), AU), 1.9),
    ((UG, (UU, AA), AU), 3.4), ((UG, (UU, AC), AU), 2.3), ((UG, (UU, AG), AU), 1.7), ((UG, (UU, AU), AU), 2.3), ((UG, (UU, CA), AU), 2.8), ((UG, (UU, CC), AU), 2.8), ((UG, (UU, CG), AU), 2.8), ((UG, (UU, CU), AU), 2.8), ((UG, (UU, GA), AU), 1.8), ((UG, (UU, GC), AU), 2.3), ((UG, (UU, GG), AU), 2.5), ((UG, (UU, GU), AU), 2.3), ((UG, (UU, UA), AU), 2.8), ((UG, (UU, UC), AU), 2.5), ((UG, (UU, UG), AU), 2.8), ((UG, (UU, UU), AU), 1.9),
    // For internal loops between the base pairs "UG" and "CG".
    ((UG, (AA, AA), CG), 1.9), ((UG, (AA, AC), CG), 1.5), ((UG, (AA, AG), CG), -0.2), ((UG, (AA, AU), CG), 1.5), ((UG, (AA, CA), CG), 1.5), ((UG, (AA, CC), CG), 1.5), ((UG, (AA, CG), CG), 1.5), ((UG, (AA, CU), CG), 1.5), ((UG, (AA, GA), CG), 0.4), ((UG, (AA, GC), CG), 1.5), ((UG, (AA, GG), CG), 0.1), ((UG, (AA, GU), CG), 1.5), ((UG, (AA, UA), CG), 1.5), ((UG, (AA, UC), CG), 1.6), ((UG, (AA, UG), CG), 1.5), ((UG, (AA, UU), CG), 1.4),
    ((UG, (AC, AA), CG), 2.1), ((UG, (AC, AC), CG), 1.9), ((UG, (AC, AG), CG), 0.1), ((UG, (AC, AU), CG), 1.9), ((UG, (AC, CA), CG), 1.8), ((UG, (AC, CC), CG), 1.9), ((UG, (AC, CG), CG), 1.8), ((UG, (AC, CU), CG), 1.9), ((UG, (AC, GA), CG), 0.7), ((UG, (AC, GC), CG), 1.9), ((UG, (AC, GG), CG), 0.5), ((UG, (AC, GU), CG), 1.9), ((UG, (AC, UA), CG), 1.8), ((UG, (AC, UC), CG), 1.9), ((UG, (AC, UG), CG), 1.8), ((UG, (AC, UU), CG), 1.1),
    ((UG, (AG, AA), CG), 2.4), ((UG, (AG, AC), CG), 2.2), ((UG, (AG, AG), CG), 0.4), ((UG, (AG, AU), CG), 2.2), ((UG, (AG, CA), CG), 2.1), ((UG, (AG, CC), CG), 2.8), ((UG, (AG, CG), CG), 2.1), ((UG, (AG, CU), CG), 2.8), ((UG, (AG, GA), CG), 1.0), ((UG, (AG, GC), CG), 2.2), ((UG, (AG, GG), CG), 2.1), ((UG, (AG, GU), CG), 2.2), ((UG, (AG, UA), CG), 2.1), ((UG, (AG, UC), CG), 2.8), ((UG, (AG, UG), CG), 2.1), ((UG, (AG, UU), CG), 1.4),
    ((UG, (AU, AA), CG), 2.1), ((UG, (AU, AC), CG), 1.9), ((UG, (AU, AG), CG), 0.1), ((UG, (AU, AU), CG), 1.9), ((UG, (AU, CA), CG), 1.8), ((UG, (AU, CC), CG), 1.9), ((UG, (AU, CG), CG), 1.8), ((UG, (AU, CU), CG), 1.9), ((UG, (AU, GA), CG), 0.7), ((UG, (AU, GC), CG), 1.9), ((UG, (AU, GG), CG), 0.5), ((UG, (AU, GU), CG), 1.9), ((UG, (AU, UA), CG), 1.8), ((UG, (AU, UC), CG), 1.9), ((UG, (AU, UG), CG), 1.8), ((UG, (AU, UU), CG), 1.1),
    ((UG, (CA, AA), CG), 2.1), ((UG, (CA, AC), CG), 1.9), ((UG, (CA, AG), CG), 0.1), ((UG, (CA, AU), CG), 1.9), ((UG, (CA, CA), CG), 1.8), ((UG, (CA, CC), CG), 1.9), ((UG, (CA, CG), CG), 1.8), ((UG, (CA, CU), CG), 1.9), ((UG, (CA, GA), CG), 0.7), ((UG, (CA, GC), CG), 1.9), ((UG, (CA, GG), CG), 0.5), ((UG, (CA, GU), CG), 1.9), ((UG, (CA, UA), CG), 1.8), ((UG, (CA, UC), CG), 1.9), ((UG, (CA, UG), CG), 1.8), ((UG, (CA, UU), CG), 1.1),
    ((UG, (CC, AA), CG), 2.1), ((UG, (CC, AC), CG), 1.9), ((UG, (CC, AG), CG), 0.7), ((UG, (CC, AU), CG), 1.9), ((UG, (CC, CA), CG), 1.8), ((UG, (CC, CC), CG), 1.9), ((UG, (CC, CG), CG), 1.8), ((UG, (CC, CU), CG), 1.9), ((UG, (CC, GA), CG), 1.3), ((UG, (CC, GC), CG), 1.9), ((UG, (CC, GG), CG), 0.5), ((UG, (CC, GU), CG), 1.9), ((UG, (CC, UA), CG), 1.8), ((UG, (CC, UC), CG), 1.9), ((UG, (CC, UG), CG), 1.8), ((UG, (CC, UU), CG), 1.1),
    ((UG, (CG, AA), CG), 2.1), ((UG, (CG, AC), CG), 1.9), ((UG, (CG, AG), CG), 0.1), ((UG, (CG, AU), CG), 1.9), ((UG, (CG, CA), CG), 1.8), ((UG, (CG, CC), CG), 1.9), ((UG, (CG, CG), CG), 1.8), ((UG, (CG, CU), CG), 1.9), ((UG, (CG, GA), CG), 0.7), ((UG, (CG, GC), CG), 1.9), ((UG, (CG, GG), CG), 0.5), ((UG, (CG, GU), CG), 1.9), ((UG, (CG, UA), CG), 1.8), ((UG, (CG, UC), CG), 1.9), ((UG, (CG, UG), CG), 1.8), ((UG, (CG, UU), CG), 1.1),
    ((UG, (CU, AA), CG), 2.1), ((UG, (CU, AC), CG), 1.9), ((UG, (CU, AG), CG), 0.7), ((UG, (CU, AU), CG), 1.9), ((UG, (CU, CA), CG), 1.8), ((UG, (CU, CC), CG), 1.9), ((UG, (CU, CG), CG), 1.8), ((UG, (CU, CU), CG), 1.9), ((UG, (CU, GA), CG), 1.3), ((UG, (CU, GC), CG), 1.9), ((UG, (CU, GG), CG), 0.5), ((UG, (CU, GU), CG), 1.9), ((UG, (CU, UA), CG), 1.8), ((UG, (CU, UC), CG), 1.9), ((UG, (CU, UG), CG), 1.8), ((UG, (CU, UU), CG), 1.1),
    ((UG, (GA, AA), CG), 0.8), ((UG, (GA, AC), CG), 0.5), ((UG, (GA, AG), CG), -1.2), ((UG, (GA, AU), CG), 0.5), ((UG, (GA, CA), CG), 0.5), ((UG, (GA, CC), CG), 1.1), ((UG, (GA, CG), CG), 0.5), ((UG, (GA, CU), CG), 1.1), ((UG, (GA, GA), CG), -0.6), ((UG, (GA, GC), CG), 0.5), ((UG, (GA, GG), CG), 0.4), ((UG, (GA, GU), CG), 0.5), ((UG, (GA, UA), CG), 0.5), ((UG, (GA, UC), CG), 1.2), ((UG, (GA, UG), CG), 0.5), ((UG, (GA, UU), CG), -0.2),
    ((UG, (GC, AA), CG), 2.1), ((UG, (GC, AC), CG), 1.9), ((UG, (GC, AG), CG), 0.1), ((UG, (GC, AU), CG), 1.9), ((UG, (GC, CA), CG), 1.8), ((UG, (GC, CC), CG), 1.9), ((UG, (GC, CG), CG), 1.8), ((UG, (GC, CU), CG), 1.9), ((UG, (GC, GA), CG), 0.7), ((UG, (GC, GC), CG), 1.9), ((UG, (GC, GG), CG), 0.5), ((UG, (GC, GU), CG), 1.9), ((UG, (GC, UA), CG), 1.8), ((UG, (GC, UC), CG), 1.9), ((UG, (GC, UG), CG), 1.8), ((UG, (GC, UU), CG), 1.1),
    ((UG, (GG, AA), CG), 0.8), ((UG, (GG, AC), CG), 0.6), ((UG, (GG, AG), CG), 0.1), ((UG, (GG, AU), CG), 0.6), ((UG, (GG, CA), CG), 0.5), ((UG, (GG, CC), CG), 0.6), ((UG, (GG, CG), CG), 0.5), ((UG, (GG, CU), CG), 0.6), ((UG, (GG, GA), CG), 0.7), ((UG, (GG, GC), CG), 0.6), ((UG, (GG, GG), CG), 1.8), ((UG, (GG, GU), CG), 0.6), ((UG, (GG, UA), CG), 0.5), ((UG, (GG, UC), CG), 0.6), ((UG, (GG, UG), CG), 0.5), ((UG, (GG, UU), CG), 1.1),
    ((UG, (GU, AA), CG), 2.1), ((UG, (GU, AC), CG), 1.9), ((UG, (GU, AG), CG), 0.1), ((UG, (GU, AU), CG), 1.9), ((UG, (GU, CA), CG), 1.8), ((UG, (GU, CC), CG), 1.9), ((UG, (GU, CG), CG), 1.8), ((UG, (GU, CU), CG), 1.9), ((UG, (GU, GA), CG), 0.7), ((UG, (GU, GC), CG), 1.9), ((UG, (GU, GG), CG), 0.5), ((UG, (GU, GU), CG), 1.9), ((UG, (GU, UA), CG), 1.8), ((UG, (GU, UC), CG), 1.9), ((UG, (GU, UG), CG), 1.8), ((UG, (GU, UU), CG), 1.1),
    ((UG, (UA, AA), CG), 2.1), ((UG, (UA, AC), CG), 1.9), ((UG, (UA, AG), CG), 0.1), ((UG, (UA, AU), CG), 1.9), ((UG, (UA, CA), CG), 1.8), ((UG, (UA, CC), CG), 1.9), ((UG, (UA, CG), CG), 1.8), ((UG, (UA, CU), CG), 1.9), ((UG, (UA, GA), CG), 0.7), ((UG, (UA, GC), CG), 1.9), ((UG, (UA, GG), CG), 0.5), ((UG, (UA, GU), CG), 1.9), ((UG, (UA, UA), CG), 1.8), ((UG, (UA, UC), CG), 1.9), ((UG, (UA, UG), CG), 1.8), ((UG, (UA, UU), CG), 1.1),
    ((UG, (UC, AA), CG), 2.1), ((UG, (UC, AC), CG), 1.9), ((UG, (UC, AG), CG), 0.7), ((UG, (UC, AU), CG), 1.9), ((UG, (UC, CA), CG), 1.8), ((UG, (UC, CC), CG), 1.9), ((UG, (UC, CG), CG), 1.8), ((UG, (UC, CU), CG), 1.9), ((UG, (UC, GA), CG), 1.3), ((UG, (UC, GC), CG), 1.9), ((UG, (UC, GG), CG), 0.5), ((UG, (UC, GU), CG), 1.9), ((UG, (UC, UA), CG), 1.8), ((UG, (UC, UC), CG), 1.9), ((UG, (UC, UG), CG), 1.8), ((UG, (UC, UU), CG), 1.1),
    ((UG, (UG, AA), CG), 2.1), ((UG, (UG, AC), CG), 1.9), ((UG, (UG, AG), CG), 0.1), ((UG, (UG, AU), CG), 1.9), ((UG, (UG, CA), CG), 1.8), ((UG, (UG, CC), CG), 1.9), ((UG, (UG, CG), CG), 1.8), ((UG, (UG, CU), CG), 1.9), ((UG, (UG, GA), CG), 0.7), ((UG, (UG, GC), CG), 1.9), ((UG, (UG, GG), CG), 0.5), ((UG, (UG, GU), CG), 1.9), ((UG, (UG, UA), CG), 1.8), ((UG, (UG, UC), CG), 1.9), ((UG, (UG, UG), CG), 1.8), ((UG, (UG, UU), CG), 1.1),
    ((UG, (UU, AA), CG), 2.7), ((UG, (UU, AC), CG), 1.9), ((UG, (UU, AG), CG), 0.1), ((UG, (UU, AU), CG), 1.9), ((UG, (UU, CA), CG), 1.8), ((UG, (UU, CC), CG), 1.9), ((UG, (UU, CG), CG), 1.8), ((UG, (UU, CU), CG), 1.9), ((UG, (UU, GA), CG), 0.7), ((UG, (UU, GC), CG), 1.9), ((UG, (UU, GG), CG), 1.8), ((UG, (UU, GU), CG), 1.9), ((UG, (UU, UA), CG), 1.8), ((UG, (UU, UC), CG), 1.9), ((UG, (UU, UG), CG), 1.8), ((UG, (UU, UU), CG), 1.1),
    // For internal loops between the base pairs "UG" and "GC".
    ((UG, (AA, AA), GC), 1.6), ((UG, (AA, AC), GC), 1.6), ((UG, (AA, AG), GC), 0.7), ((UG, (AA, AU), GC), 1.6), ((UG, (AA, CA), GC), 2.0), ((UG, (AA, CC), GC), 1.9), ((UG, (AA, CG), GC), 2.0), ((UG, (AA, CU), GC), 1.7), ((UG, (AA, GA), GC), 0.7), ((UG, (AA, GC), GC), 1.6), ((UG, (AA, GG), GC), 0.1), ((UG, (AA, GU), GC), 1.6), ((UG, (AA, UA), GC), 2.0), ((UG, (AA, UC), GC), 1.7), ((UG, (AA, UG), GC), 2.0), ((UG, (AA, UU), GC), 1.4),
    ((UG, (AC, AA), GC), 2.0), ((UG, (AC, AC), GC), 1.9), ((UG, (AC, AG), GC), 1.0), ((UG, (AC, AU), GC), 1.9), ((UG, (AC, CA), GC), 2.4), ((UG, (AC, CC), GC), 2.2), ((UG, (AC, CG), GC), 2.4), ((UG, (AC, CU), GC), 2.1), ((UG, (AC, GA), GC), 1.0), ((UG, (AC, GC), GC), 1.9), ((UG, (AC, GG), GC), 0.5), ((UG, (AC, GU), GC), 1.9), ((UG, (AC, UA), GC), 2.4), ((UG, (AC, UC), GC), 2.1), ((UG, (AC, UG), GC), 2.4), ((UG, (AC, UU), GC), 1.2),
    ((UG, (AG, AA), GC), 2.3), ((UG, (AG, AC), GC), 2.2), ((UG, (AG, AG), GC), 1.3), ((UG, (AG, AU), GC), 2.2), ((UG, (AG, CA), GC), 2.7), ((UG, (AG, CC), GC), 3.1), ((UG, (AG, CG), GC), 2.7), ((UG, (AG, CU), GC), 3.0), ((UG, (AG, GA), GC), 1.3), ((UG, (AG, GC), GC), 2.2), ((UG, (AG, GG), GC), 2.1), ((UG, (AG, GU), GC), 2.2), ((UG, (AG, UA), GC), 2.7), ((UG, (AG, UC), GC), 3.0), ((UG, (AG, UG), GC), 2.7), ((UG, (AG, UU), GC), 1.5),
    ((UG, (AU, AA), GC), 2.0), ((UG, (AU, AC), GC), 1.9), ((UG, (AU, AG), GC), 1.0), ((UG, (AU, AU), GC), 1.9), ((UG, (AU, CA), GC), 2.4), ((UG, (AU, CC), GC), 2.2), ((UG, (AU, CG), GC), 2.4), ((UG, (AU, CU), GC), 2.1), ((UG, (AU, GA), GC), 1.0), ((UG, (AU, GC), GC), 1.9), ((UG, (AU, GG), GC), 0.5), ((UG, (AU, GU), GC), 1.9), ((UG, (AU, UA), GC), 2.4), ((UG, (AU, UC), GC), 2.1), ((UG, (AU, UG), GC), 2.4), ((UG, (AU, UU), GC), 1.2),
    ((UG, (CA, AA), GC), 2.0), ((UG, (CA, AC), GC), 1.9), ((UG, (CA, AG), GC), 1.0), ((UG, (CA, AU), GC), 1.9), ((UG, (CA, CA), GC), 2.4), ((UG, (CA, CC), GC), 2.2), ((UG, (CA, CG), GC), 2.4), ((UG, (CA, CU), GC), 2.1), ((UG, (CA, GA), GC), 1.0), ((UG, (CA, GC), GC), 1.9), ((UG, (CA, GG), GC), 0.5), ((UG, (CA, GU), GC), 1.9), ((UG, (CA, UA), GC), 2.4), ((UG, (CA, UC), GC), 2.1), ((UG, (CA, UG), GC), 2.4), ((UG, (CA, UU), GC), 1.2),
    ((UG, (CC, AA), GC), 2.0), ((UG, (CC, AC), GC), 1.9), ((UG, (CC, AG), GC), 1.6), ((UG, (CC, AU), GC), 1.9), ((UG, (CC, CA), GC), 2.4), ((UG, (CC, CC), GC), 2.2), ((UG, (CC, CG), GC), 2.4), ((UG, (CC, CU), GC), 2.1), ((UG, (CC, GA), GC), 1.6), ((UG, (CC, GC), GC), 1.9), ((UG, (CC, GG), GC), 0.5), ((UG, (CC, GU), GC), 1.9), ((UG, (CC, UA), GC), 2.4), ((UG, (CC, UC), GC), 2.1), ((UG, (CC, UG), GC), 2.4), ((UG, (CC, UU), GC), 1.2),
    ((UG, (CG, AA), GC), 2.0), ((UG, (CG, AC), GC), 1.9), ((UG, (CG, AG), GC), 1.0), ((UG, (CG, AU), GC), 1.9), ((UG, (CG, CA), GC), 2.4), ((UG, (CG, CC), GC), 2.2), ((UG, (CG, CG), GC), 2.4), ((UG, (CG, CU), GC), 2.1), ((UG, (CG, GA), GC), 1.0), ((UG, (CG, GC), GC), 1.9), ((UG, (CG, GG), GC), 0.5), ((UG, (CG, GU), GC), 1.9), ((UG, (CG, UA), GC), 2.4), ((UG, (CG, UC), GC), 2.1), ((UG, (CG, UG), GC), 2.4), ((UG, (CG, UU), GC), 1.2),
    ((UG, (CU, AA), GC), 2.0), ((UG, (CU, AC), GC), 1.9), ((UG, (CU, AG), GC), 1.6), ((UG, (CU, AU), GC), 1.9), ((UG, (CU, CA), GC), 2.4), ((UG, (CU, CC), GC), 2.2), ((UG, (CU, CG), GC), 2.4), ((UG, (CU, CU), GC), 2.1), ((UG, (CU, GA), GC), 1.6), ((UG, (CU, GC), GC), 1.9), ((UG, (CU, GG), GC), 0.5), ((UG, (CU, GU), GC), 1.9), ((UG, (CU, UA), GC), 2.4), ((UG, (CU, UC), GC), 2.1), ((UG, (CU, UG), GC), 2.4), ((UG, (CU, UU), GC), 1.2),
    ((UG, (GA, AA), GC), 0.6), ((UG, (GA, AC), GC), 0.6), ((UG, (GA, AG), GC), -0.3), ((UG, (GA, AU), GC), 0.6), ((UG, (GA, CA), GC), 1.0), ((UG, (GA, CC), GC), 1.5), ((UG, (GA, CG), GC), 1.0), ((UG, (GA, CU), GC), 1.3), ((UG, (GA, GA), GC), -0.3), ((UG, (GA, GC), GC), 0.6), ((UG, (GA, GG), GC), 0.4), ((UG, (GA, GU), GC), 0.6), ((UG, (GA, UA), GC), 1.0), ((UG, (GA, UC), GC), 1.3), ((UG, (GA, UG), GC), 1.0), ((UG, (GA, UU), GC), -0.1),
    ((UG, (GC, AA), GC), 2.0), ((UG, (GC, AC), GC), 1.9), ((UG, (GC, AG), GC), 1.0), ((UG, (GC, AU), GC), 1.9), ((UG, (GC, CA), GC), 2.4), ((UG, (GC, CC), GC), 2.2), ((UG, (GC, CG), GC), 2.4), ((UG, (GC, CU), GC), 2.1), ((UG, (GC, GA), GC), 1.0), ((UG, (GC, GC), GC), 1.9), ((UG, (GC, GG), GC), 0.5), ((UG, (GC, GU), GC), 1.9), ((UG, (GC, UA), GC), 2.4), ((UG, (GC, UC), GC), 2.1), ((UG, (GC, UG), GC), 2.4), ((UG, (GC, UU), GC), 1.2),
    ((UG, (GG, AA), GC), 0.7), ((UG, (GG, AC), GC), 0.6), ((UG, (GG, AG), GC), 1.0), ((UG, (GG, AU), GC), 0.6), ((UG, (GG, CA), GC), 1.1), ((UG, (GG, CC), GC), 0.9), ((UG, (GG, CG), GC), 1.1), ((UG, (GG, CU), GC), 0.8), ((UG, (GG, GA), GC), 1.0), ((UG, (GG, GC), GC), 0.6), ((UG, (GG, GG), GC), 1.8), ((UG, (GG, GU), GC), 0.6), ((UG, (GG, UA), GC), 1.1), ((UG, (GG, UC), GC), 0.8), ((UG, (GG, UG), GC), 1.1), ((UG, (GG, UU), GC), 1.2),
    ((UG, (GU, AA), GC), 2.0), ((UG, (GU, AC), GC), 1.9), ((UG, (GU, AG), GC), 1.0), ((UG, (GU, AU), GC), 1.9), ((UG, (GU, CA), GC), 2.4), ((UG, (GU, CC), GC), 2.2), ((UG, (GU, CG), GC), 2.4), ((UG, (GU, CU), GC), 2.1), ((UG, (GU, GA), GC), 1.0), ((UG, (GU, GC), GC), 1.9), ((UG, (GU, GG), GC), 0.5), ((UG, (GU, GU), GC), 1.9), ((UG, (GU, UA), GC), 2.4), ((UG, (GU, UC), GC), 2.1), ((UG, (GU, UG), GC), 2.4), ((UG, (GU, UU), GC), 1.2),
    ((UG, (UA, AA), GC), 2.0), ((UG, (UA, AC), GC), 1.9), ((UG, (UA, AG), GC), 1.0), ((UG, (UA, AU), GC), 1.9), ((UG, (UA, CA), GC), 2.4), ((UG, (UA, CC), GC), 2.2), ((UG, (UA, CG), GC), 2.4), ((UG, (UA, CU), GC), 2.1), ((UG, (UA, GA), GC), 1.0), ((UG, (UA, GC), GC), 1.9), ((UG, (UA, GG), GC), 0.5), ((UG, (UA, GU), GC), 1.9), ((UG, (UA, UA), GC), 2.4), ((UG, (UA, UC), GC), 2.1), ((UG, (UA, UG), GC), 2.4), ((UG, (UA, UU), GC), 1.2),
    ((UG, (UC, AA), GC), 2.0), ((UG, (UC, AC), GC), 1.9), ((UG, (UC, AG), GC), 1.6), ((UG, (UC, AU), GC), 1.9), ((UG, (UC, CA), GC), 2.4), ((UG, (UC, CC), GC), 2.2), ((UG, (UC, CG), GC), 2.4), ((UG, (UC, CU), GC), 2.1), ((UG, (UC, GA), GC), 1.6), ((UG, (UC, GC), GC), 1.9), ((UG, (UC, GG), GC), 0.5), ((UG, (UC, GU), GC), 1.9), ((UG, (UC, UA), GC), 2.4), ((UG, (UC, UC), GC), 2.1), ((UG, (UC, UG), GC), 2.4), ((UG, (UC, UU), GC), 1.2),
    ((UG, (UG, AA), GC), 2.0), ((UG, (UG, AC), GC), 1.9), ((UG, (UG, AG), GC), 1.0), ((UG, (UG, AU), GC), 1.9), ((UG, (UG, CA), GC), 2.4), ((UG, (UG, CC), GC), 2.2), ((UG, (UG, CG), GC), 2.4), ((UG, (UG, CU), GC), 2.1), ((UG, (UG, GA), GC), 1.0), ((UG, (UG, GC), GC), 1.9), ((UG, (UG, GG), GC), 0.5), ((UG, (UG, GU), GC), 1.9), ((UG, (UG, UA), GC), 2.4), ((UG, (UG, UC), GC), 2.1), ((UG, (UG, UG), GC), 2.4), ((UG, (UG, UU), GC), 1.2),
    ((UG, (UU, AA), GC), 2.6), ((UG, (UU, AC), GC), 1.9), ((UG, (UU, AG), GC), 1.0), ((UG, (UU, AU), GC), 1.9), ((UG, (UU, CA), GC), 2.4), ((UG, (UU, CC), GC), 2.2), ((UG, (UU, CG), GC), 2.4), ((UG, (UU, CU), GC), 2.1), ((UG, (UU, GA), GC), 1.0), ((UG, (UU, GC), GC), 1.9), ((UG, (UU, GG), GC), 1.8), ((UG, (UU, GU), GC), 1.9), ((UG, (UU, UA), GC), 2.4), ((UG, (UU, UC), GC), 2.1), ((UG, (UU, UG), GC), 2.4), ((UG, (UU, UU), GC), 1.2),
    // For internal loops between the base pairs "UG" and "GU".
    ((UG, (AA, AA), GU), 2.1), ((UG, (AA, AC), GU), 2.4), ((UG, (AA, AG), GU), 1.1), ((UG, (AA, AU), GU), 2.4), ((UG, (AA, CA), GU), 2.4), ((UG, (AA, CC), GU), 2.4), ((UG, (AA, CG), GU), 2.4), ((UG, (AA, CU), GU), 2.4), ((UG, (AA, GA), GU), 2.7), ((UG, (AA, GC), GU), 2.4), ((UG, (AA, GG), GU), 1.1), ((UG, (AA, GU), GU), 2.4), ((UG, (AA, UA), GU), 2.4), ((UG, (AA, UC), GU), 2.4), ((UG, (AA, UG), GU), 2.4), ((UG, (AA, UU), GU), 3.0),
    ((UG, (AC, AA), GU), 2.4), ((UG, (AC, AC), GU), 2.8), ((UG, (AC, AG), GU), 1.4), ((UG, (AC, AU), GU), 2.8), ((UG, (AC, CA), GU), 2.8), ((UG, (AC, CC), GU), 2.8), ((UG, (AC, CG), GU), 2.8), ((UG, (AC, CU), GU), 2.8), ((UG, (AC, GA), GU), 3.1), ((UG, (AC, GC), GU), 2.8), ((UG, (AC, GG), GU), 1.5), ((UG, (AC, GU), GU), 2.8), ((UG, (AC, UA), GU), 2.8), ((UG, (AC, UC), GU), 2.8), ((UG, (AC, UG), GU), 2.8), ((UG, (AC, UU), GU), 2.8),
    ((UG, (AG, AA), GU), 2.7), ((UG, (AG, AC), GU), 3.1), ((UG, (AG, AG), GU), 1.7), ((UG, (AG, AU), GU), 3.1), ((UG, (AG, CA), GU), 3.1), ((UG, (AG, CC), GU), 3.7), ((UG, (AG, CG), GU), 3.1), ((UG, (AG, CU), GU), 3.7), ((UG, (AG, GA), GU), 3.4), ((UG, (AG, GC), GU), 3.1), ((UG, (AG, GG), GU), 3.1), ((UG, (AG, GU), GU), 3.1), ((UG, (AG, UA), GU), 3.1), ((UG, (AG, UC), GU), 3.7), ((UG, (AG, UG), GU), 3.1), ((UG, (AG, UU), GU), 3.1),
    ((UG, (AU, AA), GU), 2.4), ((UG, (AU, AC), GU), 2.8), ((UG, (AU, AG), GU), 1.4), ((UG, (AU, AU), GU), 2.8), ((UG, (AU, CA), GU), 2.8), ((UG, (AU, CC), GU), 2.8), ((UG, (AU, CG), GU), 2.8), ((UG, (AU, CU), GU), 2.8), ((UG, (AU, GA), GU), 3.1), ((UG, (AU, GC), GU), 2.8), ((UG, (AU, GG), GU), 1.5), ((UG, (AU, GU), GU), 2.8), ((UG, (AU, UA), GU), 2.8), ((UG, (AU, UC), GU), 2.8), ((UG, (AU, UG), GU), 2.8), ((UG, (AU, UU), GU), 2.8),
    ((UG, (CA, AA), GU), 2.4), ((UG, (CA, AC), GU), 2.8), ((UG, (CA, AG), GU), 1.4), ((UG, (CA, AU), GU), 2.8), ((UG, (CA, CA), GU), 2.8), ((UG, (CA, CC), GU), 2.8), ((UG, (CA, CG), GU), 2.8), ((UG, (CA, CU), GU), 2.8), ((UG, (CA, GA), GU), 3.1), ((UG, (CA, GC), GU), 2.8), ((UG, (CA, GG), GU), 1.5), ((UG, (CA, GU), GU), 2.8), ((UG, (CA, UA), GU), 2.8), ((UG, (CA, UC), GU), 2.8), ((UG, (CA, UG), GU), 2.8), ((UG, (CA, UU), GU), 2.8),
    ((UG, (CC, AA), GU), 2.4), ((UG, (CC, AC), GU), 2.8), ((UG, (CC, AG), GU), 2.0), ((UG, (CC, AU), GU), 2.8), ((UG, (CC, CA), GU), 2.8), ((UG, (CC, CC), GU), 2.8), ((UG, (CC, CG), GU), 2.8), ((UG, (CC, CU), GU), 2.8), ((UG, (CC, GA), GU), 3.7), ((UG, (CC, GC), GU), 2.8), ((UG, (CC, GG), GU), 1.5), ((UG, (CC, GU), GU), 2.8), ((UG, (CC, UA), GU), 2.8), ((UG, (CC, UC), GU), 2.8), ((UG, (CC, UG), GU), 2.8), ((UG, (CC, UU), GU), 2.8),
    ((UG, (CG, AA), GU), 2.4), ((UG, (CG, AC), GU), 2.8), ((UG, (CG, AG), GU), 1.4), ((UG, (CG, AU), GU), 2.8), ((UG, (CG, CA), GU), 2.8), ((UG, (CG, CC), GU), 2.8), ((UG, (CG, CG), GU), 2.8), ((UG, (CG, CU), GU), 2.8), ((UG, (CG, GA), GU), 3.1), ((UG, (CG, GC), GU), 2.8), ((UG, (CG, GG), GU), 1.5), ((UG, (CG, GU), GU), 2.8), ((UG, (CG, UA), GU), 2.8), ((UG, (CG, UC), GU), 2.8), ((UG, (CG, UG), GU), 2.8), ((UG, (CG, UU), GU), 2.8),
    ((UG, (CU, AA), GU), 2.4), ((UG, (CU, AC), GU), 2.8), ((UG, (CU, AG), GU), 2.0), ((UG, (CU, AU), GU), 2.8), ((UG, (CU, CA), GU), 2.8), ((UG, (CU, CC), GU), 2.8), ((UG, (CU, CG), GU), 2.8), ((UG, (CU, CU), GU), 2.8), ((UG, (CU, GA), GU), 3.7), ((UG, (CU, GC), GU), 2.8), ((UG, (CU, GG), GU), 1.5), ((UG, (CU, GU), GU), 2.8), ((UG, (CU, UA), GU), 2.8), ((UG, (CU, UC), GU), 2.8), ((UG, (CU, UG), GU), 2.8), ((UG, (CU, UU), GU), 2.8),
    ((UG, (GA, AA), GU), 1.1), ((UG, (GA, AC), GU), 1.4), ((UG, (GA, AG), GU), 0.1), ((UG, (GA, AU), GU), 1.4), ((UG, (GA, CA), GU), 1.4), ((UG, (GA, CC), GU), 2.0), ((UG, (GA, CG), GU), 1.4), ((UG, (GA, CU), GU), 2.0), ((UG, (GA, GA), GU), 1.7), ((UG, (GA, GC), GU), 1.4), ((UG, (GA, GG), GU), 1.4), ((UG, (GA, GU), GU), 1.4), ((UG, (GA, UA), GU), 1.4), ((UG, (GA, UC), GU), 2.0), ((UG, (GA, UG), GU), 1.4), ((UG, (GA, UU), GU), 1.4),
    ((UG, (GC, AA), GU), 2.4), ((UG, (GC, AC), GU), 2.8), ((UG, (GC, AG), GU), 1.4), ((UG, (GC, AU), GU), 2.8), ((UG, (GC, CA), GU), 2.8), ((UG, (GC, CC), GU), 2.8), ((UG, (GC, CG), GU), 2.8), ((UG, (GC, CU), GU), 2.8), ((UG, (GC, GA), GU), 3.1), ((UG, (GC, GC), GU), 2.8), ((UG, (GC, GG), GU), 1.5), ((UG, (GC, GU), GU), 2.8), ((UG, (GC, UA), GU), 2.8), ((UG, (GC, UC), GU), 2.8), ((UG, (GC, UG), GU), 2.8), ((UG, (GC, UU), GU), 2.8),
    ((UG, (GG, AA), GU), 1.1), ((UG, (GG, AC), GU), 1.5), ((UG, (GG, AG), GU), 1.4), ((UG, (GG, AU), GU), 1.5), ((UG, (GG, CA), GU), 1.5), ((UG, (GG, CC), GU), 1.5), ((UG, (GG, CG), GU), 1.5), ((UG, (GG, CU), GU), 1.5), ((UG, (GG, GA), GU), 3.1), ((UG, (GG, GC), GU), 1.5), ((UG, (GG, GG), GU), 2.8), ((UG, (GG, GU), GU), 1.5), ((UG, (GG, UA), GU), 1.5), ((UG, (GG, UC), GU), 1.5), ((UG, (GG, UG), GU), 1.5), ((UG, (GG, UU), GU), 2.8),
    ((UG, (GU, AA), GU), 2.4), ((UG, (GU, AC), GU), 2.8), ((UG, (GU, AG), GU), 1.4), ((UG, (GU, AU), GU), 2.8), ((UG, (GU, CA), GU), 2.8), ((UG, (GU, CC), GU), 2.8), ((UG, (GU, CG), GU), 2.8), ((UG, (GU, CU), GU), 2.8), ((UG, (GU, GA), GU), 3.1), ((UG, (GU, GC), GU), 2.8), ((UG, (GU, GG), GU), 1.5), ((UG, (GU, GU), GU), 2.8), ((UG, (GU, UA), GU), 2.8), ((UG, (GU, UC), GU), 2.8), ((UG, (GU, UG), GU), 2.8), ((UG, (GU, UU), GU), 2.8),
    ((UG, (UA, AA), GU), 2.4), ((UG, (UA, AC), GU), 2.8), ((UG, (UA, AG), GU), 1.4), ((UG, (UA, AU), GU), 2.8), ((UG, (UA, CA), GU), 2.8), ((UG, (UA, CC), GU), 2.8), ((UG, (UA, CG), GU), 2.8), ((UG, (UA, CU), GU), 2.8), ((UG, (UA, GA), GU), 3.1), ((UG, (UA, GC), GU), 2.8), ((UG, (UA, GG), GU), 1.5), ((UG, (UA, GU), GU), 2.8), ((UG, (UA, UA), GU), 2.8), ((UG, (UA, UC), GU), 2.8), ((UG, (UA, UG), GU), 2.8), ((UG, (UA, UU), GU), 2.8),
    ((UG, (UC, AA), GU), 2.4), ((UG, (UC, AC), GU), 2.8), ((UG, (UC, AG), GU), 2.0), ((UG, (UC, AU), GU), 2.8), ((UG, (UC, CA), GU), 2.8), ((UG, (UC, CC), GU), 2.8), ((UG, (UC, CG), GU), 2.8), ((UG, (UC, CU), GU), 2.8), ((UG, (UC, GA), GU), 3.7), ((UG, (UC, GC), GU), 2.8), ((UG, (UC, GG), GU), 1.5), ((UG, (UC, GU), GU), 2.8), ((UG, (UC, UA), GU), 2.8), ((UG, (UC, UC), GU), 2.8), ((UG, (UC, UG), GU), 2.8), ((UG, (UC, UU), GU), 2.8),
    ((UG, (UG, AA), GU), 2.4), ((UG, (UG, AC), GU), 2.8), ((UG, (UG, AG), GU), 1.4), ((UG, (UG, AU), GU), 2.8), ((UG, (UG, CA), GU), 2.8), ((UG, (UG, CC), GU), 2.8), ((UG, (UG, CG), GU), 2.8), ((UG, (UG, CU), GU), 2.8), ((UG, (UG, GA), GU), 3.1), ((UG, (UG, GC), GU), 2.8), ((UG, (UG, GG), GU), 1.5), ((UG, (UG, GU), GU), 2.8), ((UG, (UG, UA), GU), 2.8), ((UG, (UG, UC), GU), 2.8), ((UG, (UG, UG), GU), 2.8), ((UG, (UG, UU), GU), 2.8),
    ((UG, (UU, AA), GU), 3.0), ((UG, (UU, AC), GU), 2.8), ((UG, (UU, AG), GU), 1.4), ((UG, (UU, AU), GU), 2.8), ((UG, (UU, CA), GU), 2.8), ((UG, (UU, CC), GU), 2.8), ((UG, (UU, CG), GU), 2.8), ((UG, (UU, CU), GU), 2.8), ((UG, (UU, GA), GU), 3.1), ((UG, (UU, GC), GU), 2.8), ((UG, (UU, GG), GU), 2.8), ((UG, (UU, GU), GU), 2.8), ((UG, (UU, UA), GU), 2.8), ((UG, (UU, UC), GU), 2.8), ((UG, (UU, UG), GU), 2.8), ((UG, (UU, UU), GU), 2.8),
    // For internal loops between the base pairs "UG" and "UA".
    ((UG, (AA, AA), UA), 2.4), ((UG, (AA, AC), UA), 2.2), ((UG, (AA, AG), UA), 1.2), ((UG, (AA, AU), UA), 2.2), ((UG, (AA, CA), UA), 2.3), ((UG, (AA, CC), UA), 2.2), ((UG, (AA, CG), UA), 2.3), ((UG, (AA, CU), UA), 2.2), ((UG, (AA, GA), UA), 1.9), ((UG, (AA, GC), UA), 2.2), ((UG, (AA, GG), UA), 0.7), ((UG, (AA, GU), UA), 2.2), ((UG, (AA, UA), UA), 2.3), ((UG, (AA, UC), UA), 2.2), ((UG, (AA, UG), UA), 2.3), ((UG, (AA, UU), UA), 1.9),
    ((UG, (AC, AA), UA), 2.8), ((UG, (AC, AC), UA), 2.5), ((UG, (AC, AG), UA), 1.5), ((UG, (AC, AU), UA), 2.5), ((UG, (AC, CA), UA), 2.6), ((UG, (AC, CC), UA), 2.6), ((UG, (AC, CG), UA), 2.6), ((UG, (AC, CU), UA), 2.6), ((UG, (AC, GA), UA), 2.2), ((UG, (AC, GC), UA), 2.5), ((UG, (AC, GG), UA), 1.0), ((UG, (AC, GU), UA), 2.5), ((UG, (AC, UA), UA), 2.6), ((UG, (AC, UC), UA), 2.6), ((UG, (AC, UG), UA), 2.6), ((UG, (AC, UU), UA), 1.7),
    ((UG, (AG, AA), UA), 3.1), ((UG, (AG, AC), UA), 2.8), ((UG, (AG, AG), UA), 1.8), ((UG, (AG, AU), UA), 2.8), ((UG, (AG, CA), UA), 2.9), ((UG, (AG, CC), UA), 3.5), ((UG, (AG, CG), UA), 2.9), ((UG, (AG, CU), UA), 3.5), ((UG, (AG, GA), UA), 2.5), ((UG, (AG, GC), UA), 2.8), ((UG, (AG, GG), UA), 2.6), ((UG, (AG, GU), UA), 2.8), ((UG, (AG, UA), UA), 2.9), ((UG, (AG, UC), UA), 3.5), ((UG, (AG, UG), UA), 2.9), ((UG, (AG, UU), UA), 2.0),
    ((UG, (AU, AA), UA), 2.8), ((UG, (AU, AC), UA), 2.5), ((UG, (AU, AG), UA), 1.5), ((UG, (AU, AU), UA), 2.5), ((UG, (AU, CA), UA), 2.6), ((UG, (AU, CC), UA), 2.6), ((UG, (AU, CG), UA), 2.6), ((UG, (AU, CU), UA), 2.6), ((UG, (AU, GA), UA), 2.2), ((UG, (AU, GC), UA), 2.5), ((UG, (AU, GG), UA), 1.0), ((UG, (AU, GU), UA), 2.5), ((UG, (AU, UA), UA), 2.6), ((UG, (AU, UC), UA), 2.6), ((UG, (AU, UG), UA), 2.6), ((UG, (AU, UU), UA), 1.7),
    ((UG, (CA, AA), UA), 2.8), ((UG, (CA, AC), UA), 2.5), ((UG, (CA, AG), UA), 1.5), ((UG, (CA, AU), UA), 2.5), ((UG, (CA, CA), UA), 2.6), ((UG, (CA, CC), UA), 2.6), ((UG, (CA, CG), UA), 2.6), ((UG, (CA, CU), UA), 2.6), ((UG, (CA, GA), UA), 2.2), ((UG, (CA, GC), UA), 2.5), ((UG, (CA, GG), UA), 1.0), ((UG, (CA, GU), UA), 2.5), ((UG, (CA, UA), UA), 2.6), ((UG, (CA, UC), UA), 2.6), ((UG, (CA, UG), UA), 2.6), ((UG, (CA, UU), UA), 1.7),
    ((UG, (CC, AA), UA), 2.8), ((UG, (CC, AC), UA), 2.5), ((UG, (CC, AG), UA), 2.1), ((UG, (CC, AU), UA), 2.5), ((UG, (CC, CA), UA), 2.6), ((UG, (CC, CC), UA), 2.6), ((UG, (CC, CG), UA), 2.6), ((UG, (CC, CU), UA), 2.6), ((UG, (CC, GA), UA), 2.8), ((UG, (CC, GC), UA), 2.5), ((UG, (CC, GG), UA), 1.0), ((UG, (CC, GU), UA), 2.5), ((UG, (CC, UA), UA), 2.6), ((UG, (CC, UC), UA), 2.6), ((UG, (CC, UG), UA), 2.6), ((UG, (CC, UU), UA), 1.7),
    ((UG, (CG, AA), UA), 2.8), ((UG, (CG, AC), UA), 2.5), ((UG, (CG, AG), UA), 1.5), ((UG, (CG, AU), UA), 2.5), ((UG, (CG, CA), UA), 2.6), ((UG, (CG, CC), UA), 2.6), ((UG, (CG, CG), UA), 2.6), ((UG, (CG, CU), UA), 2.6), ((UG, (CG, GA), UA), 2.2), ((UG, (CG, GC), UA), 2.5), ((UG, (CG, GG), UA), 1.0), ((UG, (CG, GU), UA), 2.5), ((UG, (CG, UA), UA), 2.6), ((UG, (CG, UC), UA), 2.6), ((UG, (CG, UG), UA), 2.6), ((UG, (CG, UU), UA), 1.7),
    ((UG, (CU, AA), UA), 2.8), ((UG, (CU, AC), UA), 2.5), ((UG, (CU, AG), UA), 2.1), ((UG, (CU, AU), UA), 2.5), ((UG, (CU, CA), UA), 2.6), ((UG, (CU, CC), UA), 2.6), ((UG, (CU, CG), UA), 2.6), ((UG, (CU, CU), UA), 2.6), ((UG, (CU, GA), UA), 2.8), ((UG, (CU, GC), UA), 2.5), ((UG, (CU, GG), UA), 1.0), ((UG, (CU, GU), UA), 2.5), ((UG, (CU, UA), UA), 2.6), ((UG, (CU, UC), UA), 2.6), ((UG, (CU, UG), UA), 2.6), ((UG, (CU, UU), UA), 1.7),
    ((UG, (GA, AA), UA), 1.4), ((UG, (GA, AC), UA), 1.2), ((UG, (GA, AG), UA), 0.2), ((UG, (GA, AU), UA), 1.2), ((UG, (GA, CA), UA), 1.3), ((UG, (GA, CC), UA), 1.8), ((UG, (GA, CG), UA), 1.3), ((UG, (GA, CU), UA), 1.8), ((UG, (GA, GA), UA), 0.9), ((UG, (GA, GC), UA), 1.2), ((UG, (GA, GG), UA), 1.0), ((UG, (GA, GU), UA), 1.2), ((UG, (GA, UA), UA), 1.3), ((UG, (GA, UC), UA), 1.8), ((UG, (GA, UG), UA), 1.3), ((UG, (GA, UU), UA), 0.3),
    ((UG, (GC, AA), UA), 2.8), ((UG, (GC, AC), UA), 2.5), ((UG, (GC, AG), UA), 1.5), ((UG, (GC, AU), UA), 2.5), ((UG, (GC, CA), UA), 2.6), ((UG, (GC, CC), UA), 2.6), ((UG, (GC, CG), UA), 2.6), ((UG, (GC, CU), UA), 2.6), ((UG, (GC, GA), UA), 2.2), ((UG, (GC, GC), UA), 2.5), ((UG, (GC, GG), UA), 1.0), ((UG, (GC, GU), UA), 2.5), ((UG, (GC, UA), UA), 2.6), ((UG, (GC, UC), UA), 2.6), ((UG, (GC, UG), UA), 2.6), ((UG, (GC, UU), UA), 1.7),
    ((UG, (GG, AA), UA), 1.5), ((UG, (GG, AC), UA), 1.2), ((UG, (GG, AG), UA), 1.5), ((UG, (GG, AU), UA), 1.2), ((UG, (GG, CA), UA), 1.3), ((UG, (GG, CC), UA), 1.3), ((UG, (GG, CG), UA), 1.3), ((UG, (GG, CU), UA), 1.3), ((UG, (GG, GA), UA), 2.2), ((UG, (GG, GC), UA), 1.2), ((UG, (GG, GG), UA), 2.3), ((UG, (GG, GU), UA), 1.2), ((UG, (GG, UA), UA), 1.3), ((UG, (GG, UC), UA), 1.3), ((UG, (GG, UG), UA), 1.3), ((UG, (GG, UU), UA), 1.7),
    ((UG, (GU, AA), UA), 2.8), ((UG, (GU, AC), UA), 2.5), ((UG, (GU, AG), UA), 1.5), ((UG, (GU, AU), UA), 2.5), ((UG, (GU, CA), UA), 2.6), ((UG, (GU, CC), UA), 2.6), ((UG, (GU, CG), UA), 2.6), ((UG, (GU, CU), UA), 2.6), ((UG, (GU, GA), UA), 2.2), ((UG, (GU, GC), UA), 2.5), ((UG, (GU, GG), UA), 1.0), ((UG, (GU, GU), UA), 2.5), ((UG, (GU, UA), UA), 2.6), ((UG, (GU, UC), UA), 2.6), ((UG, (GU, UG), UA), 2.6), ((UG, (GU, UU), UA), 1.7),
    ((UG, (UA, AA), UA), 2.8), ((UG, (UA, AC), UA), 2.5), ((UG, (UA, AG), UA), 1.5), ((UG, (UA, AU), UA), 2.5), ((UG, (UA, CA), UA), 2.6), ((UG, (UA, CC), UA), 2.6), ((UG, (UA, CG), UA), 2.6), ((UG, (UA, CU), UA), 2.6), ((UG, (UA, GA), UA), 2.2), ((UG, (UA, GC), UA), 2.5), ((UG, (UA, GG), UA), 1.0), ((UG, (UA, GU), UA), 2.5), ((UG, (UA, UA), UA), 2.6), ((UG, (UA, UC), UA), 2.6), ((UG, (UA, UG), UA), 2.6), ((UG, (UA, UU), UA), 1.7),
    ((UG, (UC, AA), UA), 2.8), ((UG, (UC, AC), UA), 2.5), ((UG, (UC, AG), UA), 2.1), ((UG, (UC, AU), UA), 2.5), ((UG, (UC, CA), UA), 2.6), ((UG, (UC, CC), UA), 2.6), ((UG, (UC, CG), UA), 2.6), ((UG, (UC, CU), UA), 2.6), ((UG, (UC, GA), UA), 2.8), ((UG, (UC, GC), UA), 2.5), ((UG, (UC, GG), UA), 1.0), ((UG, (UC, GU), UA), 2.5), ((UG, (UC, UA), UA), 2.6), ((UG, (UC, UC), UA), 2.6), ((UG, (UC, UG), UA), 2.6), ((UG, (UC, UU), UA), 1.7),
    ((UG, (UG, AA), UA), 2.8), ((UG, (UG, AC), UA), 2.5), ((UG, (UG, AG), UA), 1.5), ((UG, (UG, AU), UA), 2.5), ((UG, (UG, CA), UA), 2.6), ((UG, (UG, CC), UA), 2.6), ((UG, (UG, CG), UA), 2.6), ((UG, (UG, CU), UA), 2.6), ((UG, (UG, GA), UA), 2.2), ((UG, (UG, GC), UA), 2.5), ((UG, (UG, GG), UA), 1.0), ((UG, (UG, GU), UA), 2.5), ((UG, (UG, UA), UA), 2.6), ((UG, (UG, UC), UA), 2.6), ((UG, (UG, UG), UA), 2.6), ((UG, (UG, UU), UA), 1.7),
    ((UG, (UU, AA), UA), 3.4), ((UG, (UU, AC), UA), 2.5), ((UG, (UU, AG), UA), 1.5), ((UG, (UU, AU), UA), 2.5), ((UG, (UU, CA), UA), 2.6), ((UG, (UU, CC), UA), 2.6), ((UG, (UU, CG), UA), 2.6), ((UG, (UU, CU), UA), 2.6), ((UG, (UU, GA), UA), 2.2), ((UG, (UU, GC), UA), 2.5), ((UG, (UU, GG), UA), 2.3), ((UG, (UU, GU), UA), 2.5), ((UG, (UU, UA), UA), 2.6), ((UG, (UU, UC), UA), 2.6), ((UG, (UU, UG), UA), 2.6), ((UG, (UU, UU), UA), 1.7),
    // For internal loops between the base pairs "UG" and "UG".
    ((UG, (AA, AA), UG), 3.6), ((UG, (AA, AC), UG), 2.7), ((UG, (AA, AG), UG), 1.9), ((UG, (AA, AU), UG), 2.7), ((UG, (AA, CA), UG), 2.7), ((UG, (AA, CC), UG), 2.7), ((UG, (AA, CG), UG), 2.7), ((UG, (AA, CU), UG), 2.7), ((UG, (AA, GA), UG), 3.6), ((UG, (AA, GC), UG), 2.7), ((UG, (AA, GG), UG), 1.4), ((UG, (AA, GU), UG), 2.7), ((UG, (AA, UA), UG), 2.7), ((UG, (AA, UC), UG), 2.7), ((UG, (AA, UG), UG), 2.7), ((UG, (AA, UU), UG), 3.3),
    ((UG, (AC, AA), UG), 3.4), ((UG, (AC, AC), UG), 3.1), ((UG, (AC, AG), UG), 2.3), ((UG, (AC, AU), UG), 3.1), ((UG, (AC, CA), UG), 3.1), ((UG, (AC, CC), UG), 3.1), ((UG, (AC, CG), UG), 3.1), ((UG, (AC, CU), UG), 3.1), ((UG, (AC, GA), UG), 2.7), ((UG, (AC, GC), UG), 3.1), ((UG, (AC, GG), UG), 1.8), ((UG, (AC, GU), UG), 3.1), ((UG, (AC, UA), UG), 3.1), ((UG, (AC, UC), UG), 3.1), ((UG, (AC, UG), UG), 3.1), ((UG, (AC, UU), UG), 3.1),
    ((UG, (AG, AA), UG), 3.7), ((UG, (AG, AC), UG), 3.4), ((UG, (AG, AG), UG), 2.6), ((UG, (AG, AU), UG), 3.4), ((UG, (AG, CA), UG), 3.4), ((UG, (AG, CC), UG), 4.0), ((UG, (AG, CG), UG), 3.4), ((UG, (AG, CU), UG), 4.0), ((UG, (AG, GA), UG), 3.0), ((UG, (AG, GC), UG), 3.4), ((UG, (AG, GG), UG), 3.4), ((UG, (AG, GU), UG), 3.4), ((UG, (AG, UA), UG), 3.4), ((UG, (AG, UC), UG), 4.0), ((UG, (AG, UG), UG), 3.4), ((UG, (AG, UU), UG), 3.4),
    ((UG, (AU, AA), UG), 3.4), ((UG, (AU, AC), UG), 3.1), ((UG, (AU, AG), UG), 2.3), ((UG, (AU, AU), UG), 3.1), ((UG, (AU, CA), UG), 3.1), ((UG, (AU, CC), UG), 3.1), ((UG, (AU, CG), UG), 3.1), ((UG, (AU, CU), UG), 3.1), ((UG, (AU, GA), UG), 2.7), ((UG, (AU, GC), UG), 3.1), ((UG, (AU, GG), UG), 1.8), ((UG, (AU, GU), UG), 3.1), ((UG, (AU, UA), UG), 3.1), ((UG, (AU, UC), UG), 3.1), ((UG, (AU, UG), UG), 3.1), ((UG, (AU, UU), UG), 3.1),
    ((UG, (CA, AA), UG), 3.4), ((UG, (CA, AC), UG), 3.1), ((UG, (CA, AG), UG), 2.3), ((UG, (CA, AU), UG), 3.1), ((UG, (CA, CA), UG), 3.1), ((UG, (CA, CC), UG), 3.1), ((UG, (CA, CG), UG), 3.1), ((UG, (CA, CU), UG), 3.1), ((UG, (CA, GA), UG), 2.7), ((UG, (CA, GC), UG), 3.1), ((UG, (CA, GG), UG), 1.8), ((UG, (CA, GU), UG), 3.1), ((UG, (CA, UA), UG), 3.1), ((UG, (CA, UC), UG), 3.1), ((UG, (CA, UG), UG), 3.1), ((UG, (CA, UU), UG), 3.1),
    ((UG, (CC, AA), UG), 3.4), ((UG, (CC, AC), UG), 3.1), ((UG, (CC, AG), UG), 2.9), ((UG, (CC, AU), UG), 3.1), ((UG, (CC, CA), UG), 3.1), ((UG, (CC, CC), UG), 3.1), ((UG, (CC, CG), UG), 3.1), ((UG, (CC, CU), UG), 3.1), ((UG, (CC, GA), UG), 3.3), ((UG, (CC, GC), UG), 3.1), ((UG, (CC, GG), UG), 1.8), ((UG, (CC, GU), UG), 3.1), ((UG, (CC, UA), UG), 3.1), ((UG, (CC, UC), UG), 3.1), ((UG, (CC, UG), UG), 3.1), ((UG, (CC, UU), UG), 3.1),
    ((UG, (CG, AA), UG), 3.4), ((UG, (CG, AC), UG), 3.1), ((UG, (CG, AG), UG), 2.3), ((UG, (CG, AU), UG), 3.1), ((UG, (CG, CA), UG), 3.1), ((UG, (CG, CC), UG), 3.1), ((UG, (CG, CG), UG), 3.1), ((UG, (CG, CU), UG), 3.1), ((UG, (CG, GA), UG), 2.7), ((UG, (CG, GC), UG), 3.1), ((UG, (CG, GG), UG), 1.8), ((UG, (CG, GU), UG), 3.1), ((UG, (CG, UA), UG), 3.1), ((UG, (CG, UC), UG), 3.1), ((UG, (CG, UG), UG), 3.1), ((UG, (CG, UU), UG), 3.1),
    ((UG, (CU, AA), UG), 3.4), ((UG, (CU, AC), UG), 3.1), ((UG, (CU, AG), UG), 2.9), ((UG, (CU, AU), UG), 3.1), ((UG, (CU, CA), UG), 3.1), ((UG, (CU, CC), UG), 3.1), ((UG, (CU, CG), UG), 3.1), ((UG, (CU, CU), UG), 3.1), ((UG, (CU, GA), UG), 3.3), ((UG, (CU, GC), UG), 3.1), ((UG, (CU, GG), UG), 1.8), ((UG, (CU, GU), UG), 3.1), ((UG, (CU, UA), UG), 3.1), ((UG, (CU, UC), UG), 3.1), ((UG, (CU, UG), UG), 3.1), ((UG, (CU, UU), UG), 3.1),
    ((UG, (GA, AA), UG), 2.2), ((UG, (GA, AC), UG), 1.7), ((UG, (GA, AG), UG), 0.2), ((UG, (GA, AU), UG), 1.7), ((UG, (GA, CA), UG), 1.7), ((UG, (GA, CC), UG), 2.3), ((UG, (GA, CG), UG), 1.7), ((UG, (GA, CU), UG), 2.3), ((UG, (GA, GA), UG), 1.3), ((UG, (GA, GC), UG), 1.7), ((UG, (GA, GG), UG), 1.7), ((UG, (GA, GU), UG), 1.7), ((UG, (GA, UA), UG), 1.7), ((UG, (GA, UC), UG), 2.3), ((UG, (GA, UG), UG), 1.7), ((UG, (GA, UU), UG), 1.7),
    ((UG, (GC, AA), UG), 3.4), ((UG, (GC, AC), UG), 3.1), ((UG, (GC, AG), UG), 2.3), ((UG, (GC, AU), UG), 3.1), ((UG, (GC, CA), UG), 3.1), ((UG, (GC, CC), UG), 3.1), ((UG, (GC, CG), UG), 3.1), ((UG, (GC, CU), UG), 3.1), ((UG, (GC, GA), UG), 2.7), ((UG, (GC, GC), UG), 3.1), ((UG, (GC, GG), UG), 1.8), ((UG, (GC, GU), UG), 3.1), ((UG, (GC, UA), UG), 3.1), ((UG, (GC, UC), UG), 3.1), ((UG, (GC, UG), UG), 3.1), ((UG, (GC, UU), UG), 3.1),
    ((UG, (GG, AA), UG), 2.1), ((UG, (GG, AC), UG), 1.8), ((UG, (GG, AG), UG), 2.3), ((UG, (GG, AU), UG), 1.8), ((UG, (GG, CA), UG), 1.8), ((UG, (GG, CC), UG), 1.8), ((UG, (GG, CG), UG), 1.8), ((UG, (GG, CU), UG), 1.8), ((UG, (GG, GA), UG), 2.7), ((UG, (GG, GC), UG), 1.8), ((UG, (GG, GG), UG), 3.1), ((UG, (GG, GU), UG), 1.8), ((UG, (GG, UA), UG), 1.8), ((UG, (GG, UC), UG), 1.8), ((UG, (GG, UG), UG), 1.8), ((UG, (GG, UU), UG), 3.1),
    ((UG, (GU, AA), UG), 3.4), ((UG, (GU, AC), UG), 3.1), ((UG, (GU, AG), UG), 2.3), ((UG, (GU, AU), UG), 3.1), ((UG, (GU, CA), UG), 3.1), ((UG, (GU, CC), UG), 3.1), ((UG, (GU, CG), UG), 3.1), ((UG, (GU, CU), UG), 3.1), ((UG, (GU, GA), UG), 2.7), ((UG, (GU, GC), UG), 3.1), ((UG, (GU, GG), UG), 1.8), ((UG, (GU, GU), UG), 3.1), ((UG, (GU, UA), UG), 3.1), ((UG, (GU, UC), UG), 3.1), ((UG, (GU, UG), UG), 3.1), ((UG, (GU, UU), UG), 3.1),
    ((UG, (UA, AA), UG), 3.4), ((UG, (UA, AC), UG), 3.1), ((UG, (UA, AG), UG), 2.3), ((UG, (UA, AU), UG), 3.1), ((UG, (UA, CA), UG), 3.1), ((UG, (UA, CC), UG), 3.1), ((UG, (UA, CG), UG), 3.1), ((UG, (UA, CU), UG), 3.1), ((UG, (UA, GA), UG), 2.7), ((UG, (UA, GC), UG), 3.1), ((UG, (UA, GG), UG), 1.8), ((UG, (UA, GU), UG), 3.1), ((UG, (UA, UA), UG), 3.1), ((UG, (UA, UC), UG), 3.1), ((UG, (UA, UG), UG), 3.1), ((UG, (UA, UU), UG), 3.1),
    ((UG, (UC, AA), UG), 3.4), ((UG, (UC, AC), UG), 3.1), ((UG, (UC, AG), UG), 2.9), ((UG, (UC, AU), UG), 3.1), ((UG, (UC, CA), UG), 3.1), ((UG, (UC, CC), UG), 3.1), ((UG, (UC, CG), UG), 3.1), ((UG, (UC, CU), UG), 3.1), ((UG, (UC, GA), UG), 3.3), ((UG, (UC, GC), UG), 3.1), ((UG, (UC, GG), UG), 1.8), ((UG, (UC, GU), UG), 3.1), ((UG, (UC, UA), UG), 3.1), ((UG, (UC, UC), UG), 3.1), ((UG, (UC, UG), UG), 3.1), ((UG, (UC, UU), UG), 3.1),
    ((UG, (UG, AA), UG), 3.4), ((UG, (UG, AC), UG), 3.1), ((UG, (UG, AG), UG), 2.3), ((UG, (UG, AU), UG), 3.1), ((UG, (UG, CA), UG), 3.1), ((UG, (UG, CC), UG), 3.1), ((UG, (UG, CG), UG), 3.1), ((UG, (UG, CU), UG), 3.1), ((UG, (UG, GA), UG), 2.7), ((UG, (UG, GC), UG), 3.1), ((UG, (UG, GG), UG), 1.8), ((UG, (UG, GU), UG), 3.1), ((UG, (UG, UA), UG), 3.1), ((UG, (UG, UC), UG), 3.1), ((UG, (UG, UG), UG), 3.1), ((UG, (UG, UU), UG), 3.1),
    ((UG, (UU, AA), UG), 4.0), ((UG, (UU, AC), UG), 3.1), ((UG, (UU, AG), UG), 2.3), ((UG, (UU, AU), UG), 3.1), ((UG, (UU, CA), UG), 3.1), ((UG, (UU, CC), UG), 3.1), ((UG, (UU, CG), UG), 3.1), ((UG, (UU, CU), UG), 3.1), ((UG, (UU, GA), UG), 2.7), ((UG, (UU, GC), UG), 3.1), ((UG, (UU, GG), UG), 3.1), ((UG, (UU, GU), UG), 3.1), ((UG, (UU, UA), UG), 3.1), ((UG, (UU, UC), UG), 3.1), ((UG, (UU, UG), UG), 3.1), ((UG, (UU, UU), UG), 3.1),
  ].iter() {
    TWO_VS_2_IL_DELTA_FES[(x.0).0][(x.0).1][((x.1).0).0][((x.1).0).1][((x.1).1).0][((x.1).1).1][(x.2).0][(x.2).1] = scale(y);
  }
  buf += &format!("pub const TWO_VS_2_IL_DELTA_FES: TwoVs2IlDeltaFes = {:?};\n", &TWO_VS_2_IL_DELTA_FES);

  let mut STACK_DELTA_FES = [[[[NEG_INF; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
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
  ].iter() {
    STACK_DELTA_FES[(x.0).0][(x.0).1][(x.1).0][(x.1).1] = scale(y);
  }
  buf += &format!("pub const STACK_DELTA_FES: StackDeltaFes = {:?};\n", &STACK_DELTA_FES);
  let mut HL_TM_DELTA_FES = [[[[NEG_INF; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
  for &(x, y) in [
    // For the base pair "AU" against which another base pair is stacked.
    ((AU, AA), -0.3),  ((AU, AC), -0.5),  ((AU, AG), -0.3),  ((AU, AU), -0.5),
    ((AU, CA), -0.1),  ((AU, CC), -0.2),  ((AU, CG), -0.1),  ((AU, CU), -0.2),
    ((AU, GA), -1.2),  ((AU, GC), -0.5), ((AU, GG), -1.1),  ((AU, GU), -0.5),
    ((AU, UA), -0.1),  ((AU, UC), -0.3),  ((AU, UG), -0.1), ((AU, UU), -1.2),
    // For the base pair "CG" against which another base pair is stacked.
    ((CG, AA), -1.5), ((CG, AC), -1.5), ((CG, AG), -1.4), ((CG, AU), -1.5),
    ((CG, CA), -1.0), ((CG, CC), -1.1), ((CG, CG), -1.0), ((CG, CU), -0.8),
    ((CG, GA), -2.3), ((CG, GC), -1.5), ((CG, GG), -2.4), ((CG, GU), -1.5),
    ((CG, UA), -1.0), ((CG, UC), -1.4), ((CG, UG), -1.0), ((CG, UU), -2.1),
    // For the base pair "GC" against which another base pair is stacked.
    ((GC, AA), -1.1), ((GC, AC), -1.5), ((GC, AG), -1.3), ((GC, AU), -1.5),
    ((GC, CA), -1.1),  ((GC, CC), -0.7), ((GC, CG), -1.1),  ((GC, CU), -0.5),
    ((GC, GA), -2.5), ((GC, GC), -1.5), ((GC, GG), -2.2), ((GC, GU), -1.5),
    ((GC, UA), -1.1), ((GC, UC), -1.0), ((GC, UG), -1.1), ((GC, UU), -1.6),
    // For the base pair "GU" against which another base pair is stacked.
    ((GU, AA), 0.2),  ((GU, AC), -0.5),  ((GU, AG), -0.3),  ((GU, AU), -0.5),
    ((GU, CA), -0.1),  ((GU, CC), -0.2),  ((GU, CG), -0.1),  ((GU, CU), -0.2),
    ((GU, GA), -1.0),  ((GU, GC), -0.5), ((GU, GG), -1.1),  ((GU, GU), -0.5),
    ((GU, UA), -0.1),  ((GU, UC), -0.3),  ((GU, UG), -0.1), ((GU, UU), -1.0),
    // For the base pair "UG" against which another base pair is stacked.
    ((UG, AA), -0.5),  ((UG, AC), -0.3),  ((UG, AG), -0.6),  ((UG, AU), -0.3),
    ((UG, CA), -0.2),  ((UG, CC), -0.1),  ((UG, CG), -0.2),    ((UG, CU), 0.0),
    ((UG, GA), -0.9),  ((UG, GC), -0.3), ((UG, GG), -1.1),  ((UG, GU), -0.3),
    ((UG, UA), -0.2),  ((UG, UC), -0.1),  ((UG, UG), -0.2),  ((UG, UU), -0.9),
    // For the base pair "UA" against which another base pair is stacked.
    ((UA, AA), -0.5),  ((UA, AC), -0.3),  ((UA, AG), -0.5),  ((UA, AU), -0.3),
    ((UA, CA), -0.2),  ((UA, CC), -0.1),  ((UA, CG), -0.2),    ((UA, CU), 0.0),
    ((UA, GA), -1.5),  ((UA, GC), -0.3), ((UA, GG), -1.5),  ((UA, GU), -0.3),
    ((UA, UA), -0.2),  ((UA, UC), -0.1),  ((UA, UG), -0.2),  ((UA, UU), -0.9),
  ].iter() {
    HL_TM_DELTA_FES[(x.0).0][(x.0).1][(x.1).0][(x.1).1] = scale(y);
  }
  buf += &format!("pub const HL_TM_DELTA_FES: HlTmDeltaFes = {:?};\n", &HL_TM_DELTA_FES);

  let mut IL_TM_DELTA_FES = [[[[NEG_INF; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
  for &(x, y) in [
    // For the base pair "AU" against which another base pair is stacked.
    ((AU, AA), 0.7),  ((AU, AC), 0.7),  ((AU, AG), -0.1),  ((AU, AU), 0.7),
    ((AU, CA), 0.7),  ((AU, CC), 0.7),  ((AU, CG), 0.7),  ((AU, CU), 0.7),
    ((AU, GA), -0.3),  ((AU, GC), 0.7), ((AU, GG), -0.3),  ((AU, GU), 0.7),
    ((AU, UA), 0.7),  ((AU, UC), 0.7),  ((AU, UG), 0.7), ((AU, UU), 0.1),
    // For the base pair "CG" against which another base pair is stacked.
    ((CG, AA), 0.0), ((CG, AC), 0.0), ((CG, AG), -0.8), ((CG, AU), 0.0),
    ((CG, CA), 0.0), ((CG, CC), 0.0), ((CG, CG), 0.0), ((CG, CU), 0.0),
    ((CG, GA), -1.0), ((CG, GC), 0.0), ((CG, GG), -1.0), ((CG, GU), 0.0),
    ((CG, UA), 0.0), ((CG, UC), 0.0), ((CG, UG), 0.0), ((CG, UU), -0.6),
    // For the base pair "GC" against which another base pair is stacked.
    ((GC, AA), 0.0), ((GC, AC), 0.0), ((GC, AG), -0.8), ((GC, AU), 0.0),
    ((GC, CA), 0.0),  ((GC, CC), 0.0), ((GC, CG), 0.0),  ((GC, CU), 0.0),
    ((GC, GA), -1.0), ((GC, GC), 0.0), ((GC, GG), -1.0), ((GC, GU), 0.0),
    ((GC, UA), 0.0), ((GC, UC), 0.0), ((GC, UG), 0.0), ((GC, UU), -0.6),
    // For the base pair "GU" against which another base pair is stacked.
    ((GU, AA), 0.7),  ((GU, AC), 0.7),  ((GU, AG), -0.1),  ((GU, AU), 0.7),
    ((GU, CA), 0.7),  ((GU, CC), 0.7),  ((GU, CG), 0.7),  ((GU, CU), 0.7),
    ((GU, GA), -0.3),  ((GU, GC), 0.7), ((GU, GG), -0.3),  ((GU, GU), 0.7),
    ((GU, UA), 0.7),  ((GU, UC), 0.7),  ((GU, UG), 0.7), ((GU, UU), 0.1),
    // For the base pair "UG" against which another base pair is stacked.
    ((UG, AA), 0.7),  ((UG, AC), 0.7),  ((UG, AG), -0.1),  ((UG, AU), 0.7),
    ((UG, CA), 0.7),  ((UG, CC), 0.7),  ((UG, CG), 0.7),    ((UG, CU), 0.7),
    ((UG, GA), -0.3),  ((UG, GC), 0.7), ((UG, GG), -0.3),  ((UG, GU), 0.7),
    ((UG, UA), 0.7),  ((UG, UC), 0.7),  ((UG, UG), 0.7),  ((UG, UU), 0.1),
    // For the base pair "UA" against which another base pair is stacked.
    ((UA, AA), 0.7),  ((UA, AC), 0.7),  ((UA, AG), -0.1),  ((UA, AU), 0.7),
    ((UA, CA), 0.7),  ((UA, CC), 0.7),  ((UA, CG), 0.7),    ((UA, CU), 0.7),
    ((UA, GA), -0.3),  ((UA, GC), 0.7), ((UA, GG), -0.3),  ((UA, GU), 0.7),
    ((UA, UA), 0.7),  ((UA, UC), 0.7),  ((UA, UG), 0.7),  ((UA, UU), 0.1),
  ].iter() {
    IL_TM_DELTA_FES[(x.0).0][(x.0).1][(x.1).0][(x.1).1] = scale(y);
  }
  buf += &format!("pub const IL_TM_DELTA_FES: IlTmDeltaFes = {:?};\n", &IL_TM_DELTA_FES);

  let mut ONE_VS_MANY_IL_TM_DELTA_FES = [[[[NEG_INF; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
  for &(x, y) in [
    // For the base pair "AU" against which another base pair is stacked.
    ((AU, AA), 0.7),  ((AU, AC), 0.7),  ((AU, AG), 0.7),  ((AU, AU), 0.7),
    ((AU, CA), 0.7),  ((AU, CC), 0.7),  ((AU, CG), 0.7),  ((AU, CU), 0.7),
    ((AU, GA), 0.7),  ((AU, GC), 0.7), ((AU, GG), 0.7),  ((AU, GU), 0.7),
    ((AU, UA), 0.7),  ((AU, UC), 0.7),  ((AU, UG), 0.7), ((AU, UU), 0.7),
    // For the base pair "CG" against which another base pair is stacked.
    ((CG, AA), 0.0), ((CG, AC), 0.0), ((CG, AG), 0.0), ((CG, AU), 0.0),
    ((CG, CA), 0.0), ((CG, CC), 0.0), ((CG, CG), 0.0), ((CG, CU), 0.0),
    ((CG, GA), 0.0), ((CG, GC), 0.0), ((CG, GG), 0.0), ((CG, GU), 0.0),
    ((CG, UA), 0.0), ((CG, UC), 0.0), ((CG, UG), 0.0), ((CG, UU), 0.0),
    // For the base pair "GC" against which another base pair is stacked.
    ((GC, AA), 0.0), ((GC, AC), 0.0), ((GC, AG), 0.0), ((GC, AU), 0.0),
    ((GC, CA), 0.0),  ((GC, CC), 0.0), ((GC, CG), 0.0),  ((GC, CU), 0.0),
    ((GC, GA), 0.0), ((GC, GC), 0.0), ((GC, GG), 0.0), ((GC, GU), 0.0),
    ((GC, UA), 0.0), ((GC, UC), 0.0), ((GC, UG), 0.0), ((GC, UU), 0.0),
    // For the base pair "GU" against which another base pair is stacked.
    ((GU, AA), 0.7),  ((GU, AC), 0.7),  ((GU, AG), 0.7),  ((GU, AU), 0.7),
    ((GU, CA), 0.7),  ((GU, CC), 0.7),  ((GU, CG), 0.7),  ((GU, CU), 0.7),
    ((GU, GA), 0.7),  ((GU, GC), 0.7), ((GU, GG), 0.7),  ((GU, GU), 0.7),
    ((GU, UA), 0.7),  ((GU, UC), 0.7),  ((GU, UG), 0.7), ((GU, UU), 0.7),
    // For the base pair "UG" against which another base pair is stacked.
    ((UG, AA), 0.7),  ((UG, AC), 0.7),  ((UG, AG), 0.7),  ((UG, AU), 0.7),
    ((UG, CA), 0.7),  ((UG, CC), 0.7),  ((UG, CG), 0.7),    ((UG, CU), 0.7),
    ((UG, GA), 0.7),  ((UG, GC), 0.7), ((UG, GG), 0.7),  ((UG, GU), 0.7),
    ((UG, UA), 0.7),  ((UG, UC), 0.7),  ((UG, UG), 0.7),  ((UG, UU), 0.7),
    // For the base pair "UA" against which another base pair is stacked.
    ((UA, AA), 0.7),  ((UA, AC), 0.7),  ((UA, AG), 0.7),  ((UA, AU), 0.7),
    ((UA, CA), 0.7),  ((UA, CC), 0.7),  ((UA, CG), 0.7),    ((UA, CU), 0.7),
    ((UA, GA), 0.7),  ((UA, GC), 0.7), ((UA, GG), 0.7),  ((UA, GU), 0.7),
    ((UA, UA), 0.7),  ((UA, UC), 0.7),  ((UA, UG), 0.7),  ((UA, UU), 0.7),
  ].iter() {
    ONE_VS_MANY_IL_TM_DELTA_FES[(x.0).0][(x.0).1][(x.1).0][(x.1).1] = scale(y);
  }
  buf += &format!("pub const ONE_VS_MANY_IL_TM_DELTA_FES: IlTmDeltaFes = {:?};\n", &ONE_VS_MANY_IL_TM_DELTA_FES);

  let mut TWO_VS_3_IL_TM_DELTA_FES = [[[[NEG_INF; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
  for &(x, y) in [
    // For the base pair "AU" against which another base pair is stacked.
    ((AU, AA), 0.7),  ((AU, AC), 0.7),  ((AU, AG), 0.7),  ((AU, AU), 0.7),
    ((AU, CA), 0.7),  ((AU, CC), 0.7),  ((AU, CG), 0.7),  ((AU, CU), 0.7),
    ((AU, GA), 0.4),  ((AU, GC), 0.7), ((AU, GG), 0.0),  ((AU, GU), 0.7),
    ((AU, UA), 0.7),  ((AU, UC), 0.7),  ((AU, UG), 0.7), ((AU, UU), 0.4),
    // For the base pair "CG" against which another base pair is stacked.
    ((CG, AA), 0.0), ((CG, AC), 0.0), ((CG, AG), -0.5), ((CG, AU), 0.0),
    ((CG, CA), 0.0), ((CG, CC), 0.0), ((CG, CG), 0.0), ((CG, CU), 0.0),
    ((CG, GA), -1.1), ((CG, GC), 0.0), ((CG, GG), 0.7), ((CG, GU), 0.0),
    ((CG, UA), 0.0), ((CG, UC), 0.0), ((CG, UG), 0.0), ((CG, UU), -0.3),
    // For the base pair "GC" against which another base pair is stacked.
    ((GC, AA), 0.0), ((GC, AC), 0.0), ((GC, AG), 0.0), ((GC, AU), 0.0),
    ((GC, CA), 0.0),  ((GC, CC), 0.0), ((GC, CG), 0.0),  ((GC, CU), 0.0),
    ((GC, GA), -1.2), ((GC, GC), 0.0), ((GC, GG), -0.7), ((GC, GU), 0.0),
    ((GC, UA), 0.0), ((GC, UC), 0.0), ((GC, UG), 0.0), ((GC, UU), -0.3),
    // For the base pair "GU" against which another base pair is stacked.
    ((GU, AA), 0.7),  ((GU, AC), 0.7),  ((GU, AG), 0.7),  ((GU, AU), 0.7),
    ((GU, CA), 0.7),  ((GU, CC), 0.7),  ((GU, CG), 0.7),  ((GU, CU), 0.7),
    ((GU, GA), -0.4),  ((GU, GC), 0.7), ((GU, GG), 0.0),  ((GU, GU), 0.7),
    ((GU, UA), 0.7),  ((GU, UC), 0.7),  ((GU, UG), 0.7), ((GU, UU), 0.4),
    // For the base pair "UG" against which another base pair is stacked.
    ((UG, AA), 0.7),  ((UG, AC), 0.7),  ((UG, AG), 0.7),  ((UG, AU), 0.7),
    ((UG, CA), 0.7),  ((UG, CC), 0.7),  ((UG, CG), 0.7),    ((UG, CU), 0.7),
    ((UG, GA), -0.4),  ((UG, GC), 0.7), ((UG, GG), 0.0),  ((UG, GU), 0.7),
    ((UG, UA), 0.7),  ((UG, UC), 0.7),  ((UG, UG), 0.7),  ((UG, UU), 0.4),
    // For the base pair "UA" against which another base pair is stacked.
    ((UA, AA), 0.7),  ((UA, AC), 0.7),  ((UA, AG), 0.7),  ((UA, AU), 0.7),
    ((UA, CA), 0.7),  ((UA, CC), 0.7),  ((UA, CG), 0.7),    ((UA, CU), 0.7),
    ((UA, GA), -0.4),  ((UA, GC), 0.7), ((UA, GG), 0.0),  ((UA, GU), 0.7),
    ((UA, UA), 0.7),  ((UA, UC), 0.7),  ((UA, UG), 0.7),  ((UA, UU), 0.4),
  ].iter() {
    TWO_VS_3_IL_TM_DELTA_FES[(x.0).0][(x.0).1][(x.1).0][(x.1).1] = scale(y);
  }
  buf += &format!("pub const TWO_VS_3_IL_TM_DELTA_FES: IlTmDeltaFes = {:?};\n", &TWO_VS_3_IL_TM_DELTA_FES);

  let mut ML_TM_DELTA_FES = [[[[NEG_INF; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
  for &(x, y) in [
    // For the base pair "AU" against which another base pair is stacked.
    ((AU, AA), -0.8),  ((AU, AC), -1.0),  ((AU, AG), -0.8),  ((AU, AU), -1.0),
    ((AU, CA), -0.6),  ((AU, CC), -0.7),  ((AU, CG), -0.6),  ((AU, CU), -0.7),
    ((AU, GA), -0.8),  ((AU, GC), -1.0), ((AU, GG), -0.8),  ((AU, GU), -1.0),
    ((AU, UA), -0.6),  ((AU, UC), -0.8),  ((AU, UG), -0.6), ((AU, UU), -0.8),
    // For the base pair "CG" against which another base pair is stacked.
    ((CG, AA), -1.5), ((CG, AC), -1.5), ((CG, AG), -1.4), ((CG, AU), -1.5),
    ((CG, CA), -1.0), ((CG, CC), -1.1), ((CG, CG), -1.0), ((CG, CU), -0.8),
    ((CG, GA), -1.4), ((CG, GC), -1.5), ((CG, GG), -1.6), ((CG, GU), -1.5),
    ((CG, UA), -1.0), ((CG, UC), -1.4), ((CG, UG), -1.0), ((CG, UU), -1.2),
    // For the base pair "GC" against which another base pair is stacked.
    ((GC, AA), -1.1), ((GC, AC), -1.5), ((GC, AG), -1.3), ((GC, AU), -1.5),
    ((GC, CA), -1.1),  ((GC, CC), -0.7), ((GC, CG), -1.1),  ((GC, CU), -0.5),
    ((GC, GA), -1.6), ((GC, GC), -1.5), ((GC, GG), -1.4), ((GC, GU), -1.5),
    ((GC, UA), -1.1), ((GC, UC), -1.0), ((GC, UG), -1.1), ((GC, UU), -0.7),
    // For the base pair "GU" against which another base pair is stacked.
    ((GU, AA), -0.3),  ((GU, AC), -1.0),  ((GU, AG), -0.8),  ((GU, AU), -1.0),
    ((GU, CA), -0.6),  ((GU, CC), -0.7),  ((GU, CG), -0.6),  ((GU, CU), -0.7),
    ((GU, GA), -0.6),  ((GU, GC), -1.0), ((GU, GG), -0.8),  ((GU, GU), -1.0),
    ((GU, UA), -0.6),  ((GU, UC), -0.8),  ((GU, UG), -0.6), ((GU, UU), -0.6),
    // For the base pair "UG" against which another base pair is stacked.
    ((UG, AA), -1.0),  ((UG, AC), -0.8),  ((UG, AG), -1.1),  ((UG, AU), -0.8),
    ((UG, CA), -0.7),  ((UG, CC), -0.6),  ((UG, CG), -0.7),    ((UG, CU), -0.5),
    ((UG, GA), -0.5),  ((UG, GC), -0.8), ((UG, GG), -0.8),  ((UG, GU), -0.8),
    ((UG, UA), -0.7),  ((UG, UC), -0.6),  ((UG, UG), -0.7),  ((UG, UU), -0.5),
    // For the base pair "UA" against which another base pair is stacked.
    ((UA, AA), -1.0),  ((UA, AC), -0.8),  ((UA, AG), -1.1),  ((UA, AU), -0.8),
    ((UA, CA), -0.7),  ((UA, CC), -0.6),  ((UA, CG), -0.7),    ((UA, CU), -0.5),
    ((UA, GA), -1.1),  ((UA, GC), -0.8), ((UA, GG), -1.2),  ((UA, GU), -0.8),
    ((UA, UA), -0.7),  ((UA, UC), -0.6),  ((UA, UG), -0.7),  ((UA, UU), -0.5),
  ].iter() {
    ML_TM_DELTA_FES[(x.0).0][(x.0).1][(x.1).0][(x.1).1] = scale(y);
  }
  buf += &format!("pub const ML_TM_DELTA_FES: MlTmDeltaFes = {:?};\n", &ML_TM_DELTA_FES);
  let _ = writer_2_output_file.write_all(buf.as_bytes());
}
