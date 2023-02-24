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
  let mut basepair_scores: BasepairScores = [[0.; NUM_BASES]; NUM_BASES];
  let mut terminal_mismatch_scores: TerminalMismatchScores =
    [[[[0.; NUM_BASES]; NUM_BASES]; NUM_BASES]; NUM_BASES];
  let mut stack_scores: StackScores =
    [[[[0.; NUM_BASES]; NUM_BASES]; NUM_BASES]; NUM_BASES];
  let mut helix_close_scores: HelixCloseScores = [[0.; NUM_BASES]; NUM_BASES];
  let mut dangling_scores_left: DanglingScores = [[[0.; NUM_BASES]; NUM_BASES]; NUM_BASES];
  let mut dangling_scores_right: DanglingScores = [[[0.; NUM_BASES]; NUM_BASES]; NUM_BASES];
  let mut hairpin_scores_len_atleast = [0.; MAX_LOOP_LEN + 1];
  let mut hairpin_scores_len = [0.; MAX_LOOP_LEN + 1];
  let mut sum_score_hairpin_len = 0.;
  let mut bulge_scores_len_atleast = [0.; MAX_LOOP_LEN];
  let mut bulge_scores_len = [0.; MAX_LOOP_LEN];
  let mut sum_score_bulge_len = 0.;
  let mut bulge_scores_0x1: BulgeScores0x1 = [0.; NUM_BASES];
  let mut interior_scores_len_atleast = [0.; MAX_LOOP_LEN - 1];
  let mut interior_scores_len = [0.; MAX_LOOP_LEN - 1];
  let mut sum_score_interior_len = 0.;
  let mut interior_scores_symmetric_atleast = [0.; MAX_LOOP_LEN / 2];
  let mut interior_scores_symmetric = [0.; MAX_LOOP_LEN / 2];
  let mut sum_score_interior_symmetric = 0.;
  let mut interior_scores_asymmetric_atleast = [0.; MAX_LOOP_LEN - 2];
  let mut interior_scores_asymmetric = [0.; MAX_LOOP_LEN - 2];
  let mut sum_score_interior_asymmetric = 0.;
  let mut interior_scores_explicit: InteriorScoresExplicit =
    [[0.; MAX_INTERIOR_EXPLICIT]; MAX_INTERIOR_EXPLICIT];
  let mut interior_scores_1x1: InteriorScores1x1Contra = [[0.; NUM_BASES]; NUM_BASES];
  let mut multibranch_score_base: Score = 0.;
  let mut multibranch_score_unpair: Score = 0.;
  let mut multibranch_score_basepair: Score = 0.;
  let mut external_score_unpair: Score = 0.;
  let mut external_score_basepair: Score = 0.;
  let reader = BufReader::new(File::open(input_file_path).unwrap());
  for line in reader.lines() {
    let line = line.unwrap();
    let mut sublines = line.split_whitespace();
    let feature_name = sublines.next().unwrap();
    let feature_score = sublines.next().unwrap().parse().unwrap();
    if feature_name.starts_with("base_pair_") {
      let prefix = "base_pair_";
      let prefix_len = prefix.len();
      let suffix = feature_name.split_at(prefix_len).1;
      let suffix_chars = suffix.chars().map(|x| x as u8).collect::<Vec<u8>>();
      let base_pair = (char2base(suffix_chars[0]), char2base(suffix_chars[1]));
      basepair_scores[base_pair.0][base_pair.1] = feature_score;
      basepair_scores[base_pair.1][base_pair.0] = feature_score;
    } else if feature_name.starts_with("terminal_mismatch_") {
      let prefix = "terminal_mismatch_";
      let prefix_len = prefix.len();
      let suffix = feature_name.split_at(prefix_len).1;
      let suffix_chars = suffix.chars().map(|x| x as u8).collect::<Vec<u8>>();
      let base_quad = (
        char2base(suffix_chars[0]),
        char2base(suffix_chars[1]),
        char2base(suffix_chars[2]),
        char2base(suffix_chars[3]),
      );
      terminal_mismatch_scores[base_quad.0][base_quad.1][base_quad.2]
        [base_quad.3] = feature_score;
    } else if feature_name.starts_with("hairpin_length_at_least_") {
      let prefix = "hairpin_length_at_least_";
      let prefix_len = prefix.len();
      let suffix: usize = feature_name.split_at(prefix_len).1.parse().unwrap();
      sum_score_hairpin_len += feature_score;
      hairpin_scores_len_atleast[suffix] = feature_score;
      hairpin_scores_len[suffix] = sum_score_hairpin_len;
    } else if feature_name.starts_with("bulge_length_at_least_") {
      let prefix = "bulge_length_at_least_";
      let prefix_len = prefix.len();
      let suffix: usize = feature_name.split_at(prefix_len).1.parse().unwrap();
      sum_score_bulge_len += feature_score;
      bulge_scores_len_atleast[suffix - 1] = feature_score;
      bulge_scores_len[suffix - 1] = sum_score_bulge_len;
    } else if feature_name.starts_with("bulge_0x1_nucleotides_") {
      let prefix = "bulge_0x1_nucleotides_";
      let prefix_len = prefix.len();
      let suffix = feature_name.split_at(prefix_len).1;
      let suffix_chars = suffix.chars().map(|x| x as u8).collect::<Vec<u8>>();
      let base = char2base(suffix_chars[0]);
      bulge_scores_0x1[base] = feature_score;
    } else if feature_name.starts_with("internal_length_at_least_") {
      let prefix = "internal_length_at_least_";
      let prefix_len = prefix.len();
      let suffix: usize = feature_name.split_at(prefix_len).1.parse().unwrap();
      sum_score_interior_len += feature_score;
      interior_scores_len_atleast[suffix - 2] = feature_score;
      interior_scores_len[suffix - 2] = sum_score_interior_len;
    } else if feature_name.starts_with("internal_symmetric_length_at_least_") {
      let prefix = "internal_symmetric_length_at_least_";
      let prefix_len = prefix.len();
      let suffix: usize = feature_name.split_at(prefix_len).1.parse().unwrap();
      sum_score_interior_symmetric += feature_score;
      interior_scores_symmetric_atleast[suffix - 1] = feature_score;
      interior_scores_symmetric[suffix - 1] = sum_score_interior_symmetric;
    } else if feature_name.starts_with("internal_asymmetry_at_least_") {
      let prefix = "internal_asymmetry_at_least_";
      let prefix_len = prefix.len();
      let suffix: usize = feature_name.split_at(prefix_len).1.parse().unwrap();
      sum_score_interior_asymmetric += feature_score;
      interior_scores_asymmetric_atleast[suffix - 1] = feature_score;
      interior_scores_asymmetric[suffix - 1] = sum_score_interior_asymmetric;
    } else if feature_name.starts_with("internal_explicit_") {
      let prefix = "internal_explicit_";
      let prefix_len = prefix.len();
      let suffix = feature_name.split_at(prefix_len).1;
      let len_pair: (usize, usize) = (suffix[0..1].parse().unwrap(), suffix[2..3].parse().unwrap());
      interior_scores_explicit[len_pair.0 - 1][len_pair.1 - 1] = feature_score;
      interior_scores_explicit[len_pair.1 - 1][len_pair.0 - 1] = feature_score;
    } else if feature_name.starts_with("internal_1x1_nucleotides_") {
      let prefix = "internal_1x1_nucleotides_";
      let prefix_len = prefix.len();
      let suffix = feature_name.split_at(prefix_len).1;
      let suffix_chars = suffix.chars().map(|x| x as u8).collect::<Vec<u8>>();
      let base_pair = (char2base(suffix_chars[0]), char2base(suffix_chars[1]));
      interior_scores_1x1[base_pair.0][base_pair.1] = feature_score;
      interior_scores_1x1[base_pair.1][base_pair.0] = feature_score;
    } else if feature_name.starts_with("helix_stacking_") {
      let prefix = "helix_stacking_";
      let prefix_len = prefix.len();
      let suffix = feature_name.split_at(prefix_len).1;
      let suffix_chars = suffix.chars().map(|x| x as u8).collect::<Vec<u8>>();
      let base_quad = (
        char2base(suffix_chars[0]),
        char2base(suffix_chars[1]),
        char2base(suffix_chars[2]),
        char2base(suffix_chars[3]),
      );
      stack_scores[base_quad.0][base_quad.1][base_quad.2][base_quad.3] =
        feature_score;
      stack_scores[base_quad.3][base_quad.2][base_quad.1][base_quad.0] =
        feature_score;
    } else if feature_name.starts_with("helix_closing_") {
      let prefix = "helix_closing_";
      let prefix_len = prefix.len();
      let suffix = feature_name.split_at(prefix_len).1;
      let suffix_chars = suffix.chars().map(|x| x as u8).collect::<Vec<u8>>();
      let base_pair = (char2base(suffix_chars[0]), char2base(suffix_chars[1]));
      helix_close_scores[base_pair.0][base_pair.1] = feature_score;
    } else if feature_name.starts_with("dangle_left_") {
      let prefix = "dangle_left_";
      let prefix_len = prefix.len();
      let suffix = feature_name.split_at(prefix_len).1;
      let suffix_chars = suffix.chars().map(|x| x as u8).collect::<Vec<u8>>();
      let base_triple = (
        char2base(suffix_chars[0]),
        char2base(suffix_chars[1]),
        char2base(suffix_chars[2]),
      );
      dangling_scores_left[base_triple.0][base_triple.1][base_triple.2] = feature_score;
    } else if feature_name.starts_with("dangle_right_") {
      let prefix = "dangle_right_";
      let prefix_len = prefix.len();
      let suffix = feature_name.split_at(prefix_len).1;
      let suffix_chars = suffix.chars().map(|x| x as u8).collect::<Vec<u8>>();
      let base_triple = (
        char2base(suffix_chars[0]),
        char2base(suffix_chars[1]),
        char2base(suffix_chars[2]),
      );
      dangling_scores_right[base_triple.0][base_triple.1][base_triple.2] = feature_score;
    } else if feature_name.starts_with("multi_base") {
      multibranch_score_base = feature_score;
    } else if feature_name.starts_with("multi_unpaired") {
      multibranch_score_unpair = feature_score;
    } else if feature_name.starts_with("multi_paired") {
      multibranch_score_basepair = feature_score;
    } else if feature_name.starts_with("external_unpaired") {
      external_score_unpair = feature_score;
    } else if feature_name.starts_with("external_paired") {
      external_score_basepair = feature_score;
    } else {
      println!("{}: {}", feature_name, feature_score);
      panic!();
    }
  }
  let output_file_path = Path::new("../src/compiled_scores_contra.rs");
  let mut writer = BufWriter::new(File::create(output_file_path).unwrap());
  let mut buf = "use utils::*;\n".to_string();
  buf += &format!(
    "pub const BASEPAIR_SCORES: BasepairScores = {:?};\n",
    &basepair_scores
  );
  buf += &format!(
    "pub const TERMINAL_MISMATCH_SCORES_CONTRA: TerminalMismatchScores = {:?};\n",
    &terminal_mismatch_scores
  );
  buf += &format!(
    "pub const STACK_SCORES_CONTRA: StackScores = {:?};\n",
    &stack_scores
  );
  buf += &format!(
    "pub const HELIX_CLOSE_SCORES: HelixCloseScores = {:?};\n",
    &helix_close_scores
  );
  buf += &format!(
    "pub const DANGLING_SCORES_LEFT: DanglingScores = {:?};\n",
    &dangling_scores_left
  );
  buf += &format!(
    "pub const DANGLING_SCORES_RIGHT: DanglingScores = {:?};\n",
    &dangling_scores_right
  );
  buf += &format!(
    "pub const HAIRPIN_SCORES_LEN_ATLEAST: HairpinScoresLen = {:?};\n",
    &hairpin_scores_len_atleast
  );
  buf += &format!(
    "pub const HAIRPIN_SCORES_LEN: HairpinScoresLen = {:?};\n",
    &hairpin_scores_len
  );
  buf += &format!(
    "pub const BULGE_SCORES_LEN_ATLEAST: BulgeScoresLen = {:?};\n",
    &bulge_scores_len_atleast
  );
  buf += &format!(
    "pub const BULGE_SCORES_LEN: BulgeScoresLen = {:?};\n",
    &bulge_scores_len
  );
  buf += &format!(
    "pub const BULGE_SCORES_0X1: BulgeScores0x1 = {:?};\n",
    &bulge_scores_0x1
  );
  buf += &format!(
    "pub const INTERIOR_SCORES_LEN_ATLEAST: InteriorScoresLen = {:?};\n",
    &interior_scores_len_atleast
  );
  buf += &format!(
    "pub const INTERIOR_SCORES_LEN: InteriorScoresLen = {:?};\n",
    &interior_scores_len
  );
  buf += &format!(
    "pub const INTERIOR_SCORES_SYMMETRIC_ATLEAST: InteriorScoresSymmetric = {:?};\n",
    &interior_scores_symmetric_atleast
  );
  buf += &format!(
    "pub const INTERIOR_SCORES_SYMMETRIC: InteriorScoresSymmetric = {:?};\n",
    &interior_scores_symmetric
  );
  buf += &format!(
    "pub const INTERIOR_SCORES_ASYMMETRIC_ATLEAST: InteriorScoresAsymmetric = {:?};\n",
    &interior_scores_asymmetric_atleast
  );
  buf += &format!(
    "pub const INTERIOR_SCORES_ASYMMETRIC: InteriorScoresAsymmetric = {:?};\n",
    &interior_scores_asymmetric
  );
  buf += &format!(
    "pub const INTERIOR_SCORES_EXPLICIT: InteriorScoresExplicit = {:?};\n",
    &interior_scores_explicit
  );
  buf += &format!(
    "pub const INTERIOR_SCORES_1X1_CONTRA: InteriorScores1x1Contra = {:?};\n",
    &interior_scores_1x1
  );
  buf += &format!(
    "pub const MULTIBRANCH_SCORE_BASE: Score = {};\n",
    multibranch_score_base
  );
  buf += &format!(
    "pub const MULTIBRANCH_SCORE_UNPAIR: Score = {};\n",
    multibranch_score_unpair
  );
  buf += &format!(
    "pub const MULTIBRANCH_SCORE_BASEPAIR: Score = {};\n",
    multibranch_score_basepair
  );
  buf += &format!(
    "pub const EXTERNAL_SCORE_UNPAIR: Score = {};\n",
    external_score_unpair
  );
  buf += &format!(
    "pub const EXTERNAL_SCORE_BASEPAIR: Score = {};\n",
    external_score_basepair
  );
  let _ = writer.write_all(buf.as_bytes());
}
