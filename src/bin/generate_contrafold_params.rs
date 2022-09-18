extern crate rna_ss_params;

use rna_ss_params::utils::*;
use std::env;

fn main() {
  let args = env::args().collect::<Args>();
  let program_name = args[0].clone();
  let mut opts = Options::new();
  opts.reqopt(
    "i",
    "input_file_path",
    "A CONTRAfold parameter file path",
    "STR",
  );
  opts.optflag("h", "help", "Print a help menu");
  let matches = match opts.parse(&args[1..]) {
    Ok(opt) => opt,
    Err(failure) => {
      print_program_usage(&program_name, &opts);
      panic!("{}", failure.to_string())
    }
  };
  if matches.opt_present("h") {
    print_program_usage(&program_name, &opts);
    return;
  }
  let input_file_path = matches.opt_str("i").unwrap();
  let input_file_path = Path::new(&input_file_path);
  let mut base_pair_fes: ContraBasePairFes = [[0.; NUM_OF_BASES]; NUM_OF_BASES];
  let mut terminal_mismatch_fes: ContraTerminalMismatchFes =
    [[[[0.; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
  let mut stack_fes: ContraStackFes =
    [[[[0.; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
  let mut helix_closing_fes: ContraHelixClosingFes = [[0.; NUM_OF_BASES]; NUM_OF_BASES];
  let mut left_dangle_fes: ContraDangleFes = [[[0.; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
  let mut right_dangle_fes: ContraDangleFes = [[[0.; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
  let mut hl_length_fes_at_least = [0.; CONTRA_MAX_LOOP_LEN + 1];
  let mut hl_length_fes = [0.; CONTRA_MAX_LOOP_LEN + 1];
  let mut sum_4_hl_length_fes = 0.;
  let mut bl_length_fes_at_least = [0.; CONTRA_MAX_LOOP_LEN];
  let mut bl_length_fes = [0.; CONTRA_MAX_LOOP_LEN];
  let mut sum_4_bl_length_fes = 0.;
  let mut bl_0x1_fes: ContraBl0x1Fes = [0.; NUM_OF_BASES];
  let mut il_length_fes_at_least = [0.; CONTRA_MAX_LOOP_LEN - 1];
  let mut il_length_fes = [0.; CONTRA_MAX_LOOP_LEN - 1];
  let mut sum_4_il_length_fes = 0.;
  let mut il_symm_length_fes_at_least = [0.; CONTRA_MAX_LOOP_LEN / 2];
  let mut il_symm_length_fes = [0.; CONTRA_MAX_LOOP_LEN / 2];
  let mut sum_4_il_symm_length_fes = 0.;
  let mut il_asymm_length_fes_at_least = [0.; CONTRA_MAX_LOOP_LEN - 2];
  let mut il_asymm_length_fes = [0.; CONTRA_MAX_LOOP_LEN - 2];
  let mut sum_4_il_asymm_length_fes = 0.;
  let mut il_explicit_fes: ContraIlExplicitFes =
    [[0.; CONTRA_MAX_IL_EXPLICIT_LEN]; CONTRA_MAX_IL_EXPLICIT_LEN];
  let mut il_1x1_fes: ContraIl1x1Fes = [[0.; NUM_OF_BASES]; NUM_OF_BASES];
  let mut multi_base: FreeEnergy = 0.;
  let mut multi_unpaired: FreeEnergy = 0.;
  let mut multi_paired: FreeEnergy = 0.;
  let mut external_unpaired: FreeEnergy = 0.;
  let mut external_paired: FreeEnergy = 0.;
  let reader_2_input_file_path = BufReader::new(File::open(input_file_path).unwrap());
  for line in reader_2_input_file_path.lines() {
    let line = line.unwrap();
    let mut sublines = line.split_whitespace();
    let feature_name = sublines.next().unwrap();
    let feature_weight = sublines.next().unwrap().parse().unwrap();
    if feature_name.starts_with("base_pair_") {
      let prefix = "base_pair_";
      let prefix_len = prefix.len();
      let suffix = feature_name.split_at(prefix_len).1;
      let suffix_chars = suffix.chars().map(|x| x as u8).collect::<Vec<u8>>();
      let base_pair = (convert_char(suffix_chars[0]), convert_char(suffix_chars[1]));
      base_pair_fes[base_pair.0][base_pair.1] = feature_weight;
      base_pair_fes[base_pair.1][base_pair.0] = feature_weight;
    } else if feature_name.starts_with("terminal_mismatch_") {
      let prefix = "terminal_mismatch_";
      let prefix_len = prefix.len();
      let suffix = feature_name.split_at(prefix_len).1;
      let suffix_chars = suffix.chars().map(|x| x as u8).collect::<Vec<u8>>();
      let base_quadruple = (
        convert_char(suffix_chars[0]),
        convert_char(suffix_chars[1]),
        convert_char(suffix_chars[2]),
        convert_char(suffix_chars[3]),
      );
      terminal_mismatch_fes[base_quadruple.0][base_quadruple.1][base_quadruple.2]
        [base_quadruple.3] = feature_weight;
    } else if feature_name.starts_with("hairpin_length_at_least_") {
      let prefix = "hairpin_length_at_least_";
      let prefix_len = prefix.len();
      let suffix: usize = feature_name.split_at(prefix_len).1.parse().unwrap();
      sum_4_hl_length_fes += feature_weight;
      hl_length_fes_at_least[suffix] = feature_weight;
      hl_length_fes[suffix] = sum_4_hl_length_fes;
    } else if feature_name.starts_with("bulge_length_at_least_") {
      let prefix = "bulge_length_at_least_";
      let prefix_len = prefix.len();
      let suffix: usize = feature_name.split_at(prefix_len).1.parse().unwrap();
      sum_4_bl_length_fes += feature_weight;
      bl_length_fes_at_least[suffix - 1] = feature_weight;
      bl_length_fes[suffix - 1] = sum_4_bl_length_fes;
    } else if feature_name.starts_with("bulge_0x1_nucleotides_") {
      let prefix = "bulge_0x1_nucleotides_";
      let prefix_len = prefix.len();
      let suffix = feature_name.split_at(prefix_len).1;
      let suffix_chars = suffix.chars().map(|x| x as u8).collect::<Vec<u8>>();
      let base = convert_char(suffix_chars[0]);
      bl_0x1_fes[base] = feature_weight;
    } else if feature_name.starts_with("internal_length_at_least_") {
      let prefix = "internal_length_at_least_";
      let prefix_len = prefix.len();
      let suffix: usize = feature_name.split_at(prefix_len).1.parse().unwrap();
      sum_4_il_length_fes += feature_weight;
      il_length_fes_at_least[suffix - 2] = feature_weight;
      il_length_fes[suffix - 2] = sum_4_il_length_fes;
    } else if feature_name.starts_with("internal_symmetric_length_at_least_") {
      let prefix = "internal_symmetric_length_at_least_";
      let prefix_len = prefix.len();
      let suffix: usize = feature_name.split_at(prefix_len).1.parse().unwrap();
      sum_4_il_symm_length_fes += feature_weight;
      il_symm_length_fes_at_least[suffix - 1] = feature_weight;
      il_symm_length_fes[suffix - 1] = sum_4_il_symm_length_fes;
    } else if feature_name.starts_with("internal_asymmetry_at_least_") {
      let prefix = "internal_asymmetry_at_least_";
      let prefix_len = prefix.len();
      let suffix: usize = feature_name.split_at(prefix_len).1.parse().unwrap();
      sum_4_il_asymm_length_fes += feature_weight;
      il_asymm_length_fes_at_least[suffix - 1] = feature_weight;
      il_asymm_length_fes[suffix - 1] = sum_4_il_asymm_length_fes;
    } else if feature_name.starts_with("internal_explicit_") {
      let prefix = "internal_explicit_";
      let prefix_len = prefix.len();
      let suffix = feature_name.split_at(prefix_len).1;
      let len_pair: (usize, usize) = (suffix[0..1].parse().unwrap(), suffix[2..3].parse().unwrap());
      il_explicit_fes[len_pair.0 - 1][len_pair.1 - 1] = feature_weight;
      il_explicit_fes[len_pair.1 - 1][len_pair.0 - 1] = feature_weight;
    } else if feature_name.starts_with("internal_1x1_nucleotides_") {
      let prefix = "internal_1x1_nucleotides_";
      let prefix_len = prefix.len();
      let suffix = feature_name.split_at(prefix_len).1;
      let suffix_chars = suffix.chars().map(|x| x as u8).collect::<Vec<u8>>();
      let base_pair = (convert_char(suffix_chars[0]), convert_char(suffix_chars[1]));
      il_1x1_fes[base_pair.0][base_pair.1] = feature_weight;
      il_1x1_fes[base_pair.1][base_pair.0] = feature_weight;
    } else if feature_name.starts_with("helix_stacking_") {
      let prefix = "helix_stacking_";
      let prefix_len = prefix.len();
      let suffix = feature_name.split_at(prefix_len).1;
      let suffix_chars = suffix.chars().map(|x| x as u8).collect::<Vec<u8>>();
      let base_quadruple = (
        convert_char(suffix_chars[0]),
        convert_char(suffix_chars[1]),
        convert_char(suffix_chars[2]),
        convert_char(suffix_chars[3]),
      );
      stack_fes[base_quadruple.0][base_quadruple.1][base_quadruple.2][base_quadruple.3] =
        feature_weight;
      stack_fes[base_quadruple.3][base_quadruple.2][base_quadruple.1][base_quadruple.0] =
        feature_weight;
    } else if feature_name.starts_with("helix_closing_") {
      let prefix = "helix_closing_";
      let prefix_len = prefix.len();
      let suffix = feature_name.split_at(prefix_len).1;
      let suffix_chars = suffix.chars().map(|x| x as u8).collect::<Vec<u8>>();
      let base_pair = (convert_char(suffix_chars[0]), convert_char(suffix_chars[1]));
      helix_closing_fes[base_pair.0][base_pair.1] = feature_weight;
    } else if feature_name.starts_with("dangle_left_") {
      let prefix = "dangle_left_";
      let prefix_len = prefix.len();
      let suffix = feature_name.split_at(prefix_len).1;
      let suffix_chars = suffix.chars().map(|x| x as u8).collect::<Vec<u8>>();
      let base_triple = (
        convert_char(suffix_chars[0]),
        convert_char(suffix_chars[1]),
        convert_char(suffix_chars[2]),
      );
      left_dangle_fes[base_triple.0][base_triple.1][base_triple.2] = feature_weight;
    } else if feature_name.starts_with("dangle_right_") {
      let prefix = "dangle_right_";
      let prefix_len = prefix.len();
      let suffix = feature_name.split_at(prefix_len).1;
      let suffix_chars = suffix.chars().map(|x| x as u8).collect::<Vec<u8>>();
      let base_triple = (
        convert_char(suffix_chars[0]),
        convert_char(suffix_chars[1]),
        convert_char(suffix_chars[2]),
      );
      right_dangle_fes[base_triple.0][base_triple.1][base_triple.2] = feature_weight;
    } else if feature_name.starts_with("multi_base") {
      multi_base = feature_weight;
    } else if feature_name.starts_with("multi_unpaired") {
      multi_unpaired = feature_weight;
    } else if feature_name.starts_with("multi_paired") {
      multi_paired = feature_weight;
    } else if feature_name.starts_with("external_unpaired") {
      external_unpaired = feature_weight;
    } else if feature_name.starts_with("external_paired") {
      external_paired = feature_weight;
    } else {
      println!("{}: {}", feature_name, feature_weight);
      assert!(false);
    }
  }
  let output_file_path = Path::new("./src/compiled_free_energy_params_contra.rs");
  let mut writer_2_output_file = BufWriter::new(File::create(&output_file_path).unwrap());
  let mut buf = format!("use utils::*;\n");
  buf += &format!(
    "pub const CONTRA_BASE_PAIR_FES: ContraBasePairFes = {:?};\n",
    &base_pair_fes
  );
  buf += &format!(
    "pub const CONTRA_TERMINAL_MISMATCH_FES: ContraTerminalMismatchFes = {:?};\n",
    &terminal_mismatch_fes
  );
  buf += &format!(
    "pub const CONTRA_STACK_FES: ContraStackFes = {:?};\n",
    &stack_fes
  );
  buf += &format!(
    "pub const CONTRA_HELIX_CLOSING_FES: ContraHelixClosingFes = {:?};\n",
    &helix_closing_fes
  );
  buf += &format!(
    "pub const CONTRA_LEFT_DANGLE_FES: ContraDangleFes = {:?};\n",
    &left_dangle_fes
  );
  buf += &format!(
    "pub const CONTRA_RIGHT_DANGLE_FES: ContraDangleFes = {:?};\n",
    &right_dangle_fes
  );
  buf += &format!(
    "pub const CONTRA_HL_LENGTH_FES_AT_LEAST: ContraHlLengthFes = {:?};\n",
    &hl_length_fes_at_least
  );
  buf += &format!(
    "pub const CONTRA_HL_LENGTH_FES: ContraHlLengthFes = {:?};\n",
    &hl_length_fes
  );
  buf += &format!(
    "pub const CONTRA_BL_LENGTH_FES_AT_LEAST: ContraBlLengthFes = {:?};\n",
    &bl_length_fes_at_least
  );
  buf += &format!(
    "pub const CONTRA_BL_LENGTH_FES: ContraBlLengthFes = {:?};\n",
    &bl_length_fes
  );
  buf += &format!(
    "pub const CONTRA_BL_0X1_FES: ContraBl0x1Fes = {:?};\n",
    &bl_0x1_fes
  );
  buf += &format!(
    "pub const CONTRA_IL_LENGTH_FES_AT_LEAST: ContraIlLengthFes = {:?};\n",
    &il_length_fes_at_least
  );
  buf += &format!(
    "pub const CONTRA_IL_LENGTH_FES: ContraIlLengthFes = {:?};\n",
    &il_length_fes
  );
  buf += &format!(
    "pub const CONTRA_IL_SYMM_LENGTH_FES_AT_LEAST: ContraIlSymmLengthFes = {:?};\n",
    &il_symm_length_fes_at_least
  );
  buf += &format!(
    "pub const CONTRA_IL_SYMM_LENGTH_FES: ContraIlSymmLengthFes = {:?};\n",
    &il_symm_length_fes
  );
  buf += &format!(
    "pub const CONTRA_IL_ASYMM_LENGTH_FES_AT_LEAST: ContraIlAsymmLengthFes = {:?};\n",
    &il_asymm_length_fes_at_least
  );
  buf += &format!(
    "pub const CONTRA_IL_ASYMM_LENGTH_FES: ContraIlAsymmLengthFes = {:?};\n",
    &il_asymm_length_fes
  );
  buf += &format!(
    "pub const CONTRA_IL_EXPLICIT_FES: ContraIlExplicitFes = {:?};\n",
    &il_explicit_fes
  );
  buf += &format!(
    "pub const CONTRA_IL_1X1_FES: ContraIl1x1Fes = {:?};\n",
    &il_1x1_fes
  );
  buf += &format!(
    "pub const CONTRA_ML_BASE_FE: FreeEnergy = {};\n",
    multi_base
  );
  buf += &format!(
    "pub const CONTRA_ML_UNPAIRED_FE: FreeEnergy = {};\n",
    multi_unpaired
  );
  buf += &format!(
    "pub const CONTRA_ML_PAIRED_FE: FreeEnergy = {};\n",
    multi_paired
  );
  buf += &format!(
    "pub const CONTRA_EL_UNPAIRED_FE: FreeEnergy = {};\n",
    external_unpaired
  );
  buf += &format!(
    "pub const CONTRA_EL_PAIRED_FE: FreeEnergy = {};\n",
    external_paired
  );
  let _ = writer_2_output_file.write_all(buf.as_bytes());
}
