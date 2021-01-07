extern crate rna_ss_params;

use rna_ss_params::utils::*;

fn main() {
  let output_file_path = Path::new("./src/compiled_free_energy_params_contra.rs");
  let mut writer_2_output_file = BufWriter::new(File::create(&output_file_path).unwrap());
  let mut buf = format!("use utils::*;\n");
  let mut base_pair_fes: ContraBasePairFes = [[0.; NUM_OF_BASES]; NUM_OF_BASES];
  for &(x, y) in [
    (AU, 0.59791199), (UA, 0.59791199), (CG, 1.544290641), (GC, 1.544290641), (GU, -0.01304754992), (UG, -0.01304754992),
  ].iter() {
    base_pair_fes[x.0][x.1] = y;
  }
  buf += &format!("pub const CONTRA_BASE_PAIR_FES: ContraBasePairFes = {:?};\n", &base_pair_fes);
  let mut terminal_mismatch_fes: ContraTerminalMismatchFes = [[[[0.; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
  for &(x, y) in [
    ((AU, AA), -0.184546064),
    ((AU, AC), -0.1181844187),
    ((AU, AG), -0.4461469607),
    ((AU, AU), -0.6175254495),
    ((AU, CA), 0.004788458708),
    ((AU, CC), 0.08319395146),
    ((AU, CG), -0.2249479995),
    ((AU, CU), -0.3981327204),
    ((AU, GA), 0.5191110288),
    ((AU, GC), -0.3524119307),
    ((AU, GG), -0.4056429433),
    ((AU, GU), -0.7733932162),
    ((AU, UA), -0.01574403519),
    ((AU, UC), 0.268570042),
    ((AU, UG), -0.0934388741),
    ((AU, UU), 0.3373711531),
    ((CG, AA), 0.08386423535),
    ((CG, AC), -0.2520716816),
    ((CG, AG), -0.6711841881),
    ((CG, AU), -0.3816350028),
    ((CG, CA), 0.1117852189),
    ((CG, CC), -0.1704393624),
    ((CG, CG), -0.2179987732),
    ((CG, CU), -0.459267635),
    ((CG, GA), 0.8520640313),
    ((CG, GC), -0.9332488517),
    ((CG, GG), -0.3289551692),
    ((CG, GU), -0.7778822056),
    ((CG, UA), -0.2422339958),
    ((CG, UC), -0.03780509247),
    ((CG, UG), -0.4322334143),
    ((CG, UU), -0.2419976114),
    ((GC, AA), -0.1703136025),
    ((GC, AC), -0.09154056357),
    ((GC, AG), -0.2522413002),
    ((GC, AU), -0.8520314799),
    ((GC, CA), 0.04763224188),
    ((GC, CC), -0.2428654283),
    ((GC, CG), -0.2079275061),
    ((GC, CU), -0.1874270053),
    ((GC, GA), 0.6540033983),
    ((GC, GC), -0.7823988605),
    ((GC, GG), 0.1995898255),
    ((GC, GU), -0.4432169392),
    ((GC, UA), -0.1736921762),
    ((GC, UC), 0.288494362),
    ((GC, UG), -0.01638238057),
    ((GC, UU), 0.6757988971),
    ((GU, AA), -0.4871607613),
    ((GU, AC), 0.1105031953),
    ((GU, AG), 0.363373916),
    ((GU, AU), -0.6193199348),
    ((GU, CA), 0.3451056056),
    ((GU, CC), 0.0314944976),
    ((GU, CG), -0.3799172956),
    ((GU, CU), -0.03222973182),
    ((GU, GA), 0.4948638637),
    ((GU, GC), -0.2821952552),
    ((GU, GG), -0.2702227211),
    ((GU, GU), -0.06658395291),
    ((GU, UA), -0.4306154451),
    ((GU, UC), -0.09497863465),
    ((GU, UG), -0.3130794485),
    ((GU, UU), -0.2283242981),
    ((UA, AA), 0.0115363879),
    ((UA, AC), -0.3923408221),
    ((UA, AG), 0.05661063599),
    ((UA, AU), -0.1251485388),
    ((UA, CA), -0.06545074758),
    ((UA, CC), -0.3167200568),
    ((UA, CG), 0.002258383981),
    ((UA, CU), -0.422217724),
    ((UA, GA), 0.5458416646),
    ((UA, GC), -0.2085887954),
    ((UA, GG), -0.1971766062),
    ((UA, GU), -0.4722410132),
    ((UA, UA), -0.1779642496),
    ((UA, UC), 0.1643454344),
    ((UA, UG), -0.5005617032),
    ((UA, UU), 0.1333867679),
    ((UG, AA), 0.1218741278),
    ((UG, AC), 0.1990260141),
    ((UG, AG), 0.04681893928),
    ((UG, AU), 0.3256264491),
    ((UG, CA), 0.1186812326),
    ((UG, CC), -0.1851065102),
    ((UG, CG), -0.04311512683),
    ((UG, CU), -0.6150608139),
    ((UG, GA), 0.754933218),
    ((UG, GC), -0.3150708483),
    ((UG, GG), 0.1569582926),
    ((UG, GU), -0.514970007),
    ((UG, UA), -0.2926246029),
    ((UG, UC), 0.1373068149),
    ((UG, UG), -0.05422333363),
    ((UG, UU), 0.03086776921),
  ].iter() {
    terminal_mismatch_fes[(x.0).0][(x.0).1][(x.1).0][(x.1).1] = y;
  }

  buf += &format!("pub const CONTRA_TERMINAL_MISMATCH_FES: ContraTerminalMismatchFes = {:?};\n", &terminal_mismatch_fes);
  let mut stack_fes: ContraStackFes = [[[[0.; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
  for &(x, y) in [
    ((AU, AU), 0.1482005248),
    ((AU, CG), 0.4343497127),
    ((AU, GC), 0.7079642577),
    ((AU, GU), -0.1010777582),
    ((AU, UA), 0.243256656),
    ((AU, UG), 0.1623654243),
    ((CG, AU), 0.4878707793),
    ((CG, CG), 0.8481320247),
    ((CG, GC), 0.4784248478),
    ((CG, GU), -0.1811268205),
    ((CG, UG), 0.4849351028),
    ((GC, AU), 0.5551785831),
    ((GC, CG), 0.5008324248),
    ((GC, GU), 0.2165962476),
    ((GC, UG), 0.4864603589),
    ((GU, AU), -0.04665365028),
    ((GU, GU), 0.1833447295),
    ((GU, UG), -0.2858970755),
    ((UA, AU), 0.3897593783),
    ((UA, GU), -0.1157333764),
    ((UG, GU), 0.120296538),
  ].iter() {
    stack_fes[(x.0).0][(x.0).1][(x.1).0][(x.1).1] = y;
    stack_fes[(x.1).1][(x.1).0][(x.0).1][(x.0).0] = y;
  }

  buf += &format!("pub const CONTRA_STACK_FES: ContraStackFes = {:?};\n", &stack_fes);
  let mut helix_closing_fes: ContraHelixClosingFes = [[0.; NUM_OF_BASES]; NUM_OF_BASES];
  for &(x, y) in [
    (AU, -0.9770893163),
    (CG, -0.4574650937),
    (GC, -0.8265995623),
    (GU, -1.051678928),
    (UA, -0.9246140521),
    (UG, -0.3698708172),
  ].iter() {
    helix_closing_fes[x.0][x.1] = y;
  }
  buf += &format!("pub const CONTRA_HELIX_CLOSING_FES: ContraHelixClosingFes = {:?};\n", &helix_closing_fes);

  let mut left_dangle_fes: ContraDangleFes = [[[0.; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
  for &(x, y) in &[
    // For the base pair "AU" against which a base is stacked.
    ((AU, A), -0.1251037681), ((AU, C), 0.0441606708), ((AU, G), -0.02541879082), ((AU, U), 0.00785098466), 
    // For the base pair "CG" against which a base is stacked.
    ((CG, A), 0.07224381372), ((CG, C), 0.05279281874), ((CG, G), 0.1009554299), ((CG, U), -0.1515059013), 
    // For the base pair "GC" against which a base is stacked.
    ((GC, A), -0.1829535099), ((GC, C), 0.03393000394), ((GC, G), 0.1335339061), ((GC, U), -0.1604274506), 
    // For the base pair "GU" against which a base is stacked.
    ((GU, A), -0.06517511341), ((GU, C), -0.04250882422), ((GU, G), 0.02875971806), ((GU, U), -0.04359727428), 
    // For the base pair "UA" against which a base is stacked.
    ((UA, A), -0.03373847659), ((UA, C), -0.005070324324), ((UA, G), -0.1186861149), ((UA, U), -0.01162357727), 
    // For the base pair "UG" against which a base is stacked.
    ((UG, A), -0.08047139148), ((UG, C), 0.001608000669), ((UG, G), 0.1016272216), ((UG, U), -0.09200842832), 
  ] {
    left_dangle_fes[(x.0).0][(x.0).1][x.1] = y;
  }
  buf += &format!("pub const CONTRA_LEFT_DANGLE_FES: ContraDangleFes = {:?};\n", &left_dangle_fes);

  let mut right_dangle_fes: ContraDangleFes = [[[0.; NUM_OF_BASES]; NUM_OF_BASES]; NUM_OF_BASES];
  for &(x, y) in &[
    // For the base pair "AU" against which a base is stacked.
    ((AU, A), 0.03232578201), ((AU, C), -0.09096819493), ((AU, G), -0.0740750973), ((AU, U), -0.01621157379), 
    // For the base pair "CG" against which a base is stacked.
    ((CG, A), 0.2133964379), ((CG, C), -0.06234810991), ((CG, G), -0.07008531041), ((CG, U), -0.2141912285), 
    // For the base pair "GC" against which a base is stacked.
    ((GC, A), 0.01581957549), ((GC, C), 0.005644320058), ((GC, G), -0.00943297687), ((GC, U), -0.2597793095), 
    // For the base pair "GU" against which a base is stacked.
    ((GU, A), -0.04480271781), ((GU, C), -0.07321213002), ((GU, G), 0.01270494867), ((GU, U), -0.05717033985), 
    // For the base pair "UA" against which a base is stacked.
    ((UA, A), -0.1631918513), ((UA, C), 0.06769304994), ((UA, G), -0.08789074414), ((UA, U), -0.05525570007), 
    // For the base pair "UG" against which a base is stacked.
    ((UG, A), 0.04105458185), ((UG, C), -0.008136642572), ((UG, G), -0.03808592022), ((UG, U), -0.08629373429), 
  ] {
    right_dangle_fes[(x.0).0][(x.0).1][x.1] = y;
  }

  buf += &format!("pub const CONTRA_RIGHT_DANGLE_FES: ContraDangleFes = {:?};\n", &right_dangle_fes);
  let mut hl_length_fes_at_least = [0.; CONTRA_MAX_LOOP_LEN + 1];
  let mut hl_length_fes = [0.; CONTRA_MAX_LOOP_LEN + 1];
  let mut sum = 0.;
  for (i, &x) in [
    -5.993180158,
    -3.108105762,
    0.4168976347,
    2.205419066,
    1.926749692,
    -0.5873245329,
    -0.0827571778,
    0.5783889844,
    -0.7220883372,
    -0.1725874624,
    -0.3025089867,
    -0.0296315939,
    -0.9268995948,
    -0.03157753978,
    -0.1022472101,
    0.1901407346,
    -0.09280909826,
    0.1690448408,
    -0.08172566471,
    -0.3445939031,
    -0.109150294,
    -0.2903523693,
    -0.3393713667,
    -0.1915364117,
    -0.05019209379,
    -0.03874620924,
    0.04751470752,
    0.06744321926,
    0.09721875726,
    0.1673131733,
    0.2329937249,
  ].iter().enumerate() {
    sum += x;
    hl_length_fes_at_least[i] = x;
    hl_length_fes[i] = sum;
  }
  buf += &format!("pub const CONTRA_HL_LENGTH_FES_AT_LEAST: ContraHlLengthFes = {:?};\n", &hl_length_fes_at_least);
  buf += &format!("pub const CONTRA_HL_LENGTH_FES: ContraHlLengthFes = {:?};\n", &hl_length_fes);
  let mut bl_length_fes_at_least = [0.; CONTRA_MAX_LOOP_LEN];
  let mut bl_length_fes = [0.; CONTRA_MAX_LOOP_LEN];
  sum = 0.;
  for (i, &x) in [
    -2.399548472,
    -0.8945183117,
    -0.9088550909,
    -0.8412474755,
    -0.4365479343,
    -0.5699187801,
    0.2002834224,
    0.7538761358,
    -0.6045045455,
    -0.7200948098,
    -0.5136721921,
    -0.3614726679,
    -0.2614454392,
    -0.1593926893,
    -0.08624668281,
    -0.03107090996,
    -0.01097222032,
    0.03001220283,
    0.04759123789,
    -0.04296172065,
    -0.01791899662,
    -0.07800551522,
    -0.0709932643,
    -0.05767952896,
    -0.04633794681,
    -0.03559420456,
    -0.02674934394,
    -0.01818957972,
    -0.01052300732,
    -0.005153626846,
  ].iter().enumerate() {
    sum += x;
    bl_length_fes_at_least[i] = x;
    bl_length_fes[i] = sum;
  }
  buf += &format!("pub const CONTRA_BL_LENGTH_FES_AT_LEAST: ContraBlLengthFes = {:?};\n", &bl_length_fes_at_least);
  buf += &format!("pub const CONTRA_BL_LENGTH_FES: ContraBlLengthFes = {:?};\n", &bl_length_fes);

  let mut il_length_fes_at_least = [0.; CONTRA_MAX_LOOP_LEN - 1];
  let mut il_length_fes = [0.; CONTRA_MAX_LOOP_LEN - 1];
  sum = 0.;
  for (i, &x) in [
    -0.429061443,
    -0.3532111501,
    -0.3963797535,
    -0.3111199175,
    -0.2551945472,
    -0.05149116898,
    -0.04319002407,
    0.001985489485,
    -0.1761513136,
    -0.2639686207,
    -0.3460613577,
    -0.2926603079,
    -0.03624250307,
    -0.1199953761,
    -0.04354771926,
    -0.08209293135,
    -0.007113226038,
    0.02354824852,
    0.03066973571,
    -0.06618241094,
    -0.1316092383,
    -0.1407995514,
    -0.06600291862,
    -0.07779204744,
    -0.05084201265,
    -0.04139875601,
    0.003276583405,
    0.00592458284,
    0.006875738004,
  ].iter().enumerate() {
    sum += x;
    il_length_fes_at_least[i] = x;
    il_length_fes[i] = sum;
  }
  buf += &format!("pub const CONTRA_IL_LENGTH_FES_AT_LEAST: ContraIlLengthFes = {:?};\n", &il_length_fes_at_least);
  buf += &format!("pub const CONTRA_IL_LENGTH_FES: ContraIlLengthFes = {:?};\n", &il_length_fes);

  let mut il_symm_length_fes_at_least = [0.; CONTRA_MAX_LOOP_LEN / 2];
  let mut il_symm_length_fes = [0.; CONTRA_MAX_LOOP_LEN / 2];
  sum = 0.;
  for (i, &x) in [
    -0.5467082599,
    -0.3854701647,
    -0.2588466401,
    -0.2340836745,
    0.1450577765,
    -0.6562932515,
    -0.3021088369,
    -0.03032275267,
    -0.3517944058,
    -0.2159132506,
    -0.1228270454,
    -0.1552208595,
    -0.08541120743,
    -0.04592109799,
    -0.02232234236,
  ].iter().enumerate() {
    sum += x;
    il_symm_length_fes_at_least[i] = x;
    il_symm_length_fes[i] = sum;
  }
  buf += &format!("pub const CONTRA_IL_SYMM_LENGTH_FES_AT_LEAST: ContraIlSymmLengthFes = {:?};\n", &il_symm_length_fes_at_least);
  buf += &format!("pub const CONTRA_IL_SYMM_LENGTH_FES: ContraIlSymmLengthFes = {:?};\n", &il_symm_length_fes);

  let mut il_asymm_length_fes_at_least = [0.; CONTRA_MAX_LOOP_LEN - 2];
  let mut il_asymm_length_fes = [0.; CONTRA_MAX_LOOP_LEN - 2];
  sum = 0.;
  for (i, &x) in [
    -2.105646719,
    -0.5520140431,
    -0.577070767,
    -0.6136667847,
    -0.3057156841,
    -0.1155052001,
    -0.2105612231,
    -0.314574313,
    -0.3148961681,
    -0.09018189492,
    -0.2200026794,
    -0.1406483243,
    -0.2162411259,
    -0.1725531435,
    -0.1558911866,
    -0.1040858663,
    -0.06967684228,
    -0.04105977494,
    -0.01570624316,
    0.01382000639,
    0.04131988563,
    0.0359418595,
    0.02822186282,
    0.01636585874,
    0.02550056175,
    0.03348032793,
    0.03971924412,
    -0.002545113932,
  ].iter().enumerate() {
    sum += x;
    il_asymm_length_fes_at_least[i] = x;
    il_asymm_length_fes[i] = sum;
  }
  buf += &format!("pub const CONTRA_IL_ASYMM_LENGTH_FES_AT_LEAST: ContraIlAsymmLengthFes = {:?};\n", &il_asymm_length_fes_at_least);
  buf += &format!("pub const CONTRA_IL_ASYMM_LENGTH_FES: ContraIlAsymmLengthFes = {:?};\n", &il_asymm_length_fes);
  let _ = writer_2_output_file.write_all(buf.as_bytes());
}
