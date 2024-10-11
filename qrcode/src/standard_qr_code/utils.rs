use std::process::exit;

/// Function returns the version of a qr code big enough
/// to encode the number of codewords (bytes) that are handed to it.
/// It also returns the maximum of codewords this version can fit. The first
/// value is the version and the second one is the number of codewords.
pub fn get_verison_info(text_len: usize) -> (usize, usize) {
    match text_len {
        0..=26 => (1, 26),
        27..=44 => (2, 44),
        45..=70 => (3, 70),
        71..=100 => (4, 100),
        101..=134 => (5, 134),
        135..=172 => (6, 172),
        173..=196 => (7, 196),
        197..=242 => (8, 242),
        243..=292 => (9, 292),
        293..=346 => (10, 346),
        347..=404 => (11, 404),
        405..=466 => (12, 466),
        467..=532 => (13, 532),
        533..=581 => (14, 581),
        582..=655 => (15, 655),
        656..=733 => (16, 733),
        734..=815 => (17, 815),
        816..=901 => (18, 901),
        902..=991 => (19, 991),
        992..=1085 => (20, 1085),
        1086..=1156 => (21, 1156),
        1157..=1258 => (22, 1258),
        1259..=1364 => (23, 1364),
        1365..=1474 => (24, 1474),
        1475..=1588 => (25, 1588),
        1589..=1706 => (26, 1706),
        1707..=1828 => (27, 1828),
        1829..=1921 => (28, 1921),
        1922..=2051 => (29, 2051),
        2052..=2185 => (30, 2185),
        2186..=2323 => (31, 2323),
        2324..=2465 => (32, 2465),
        2466..=2611 => (33, 2611),
        2612..=2761 => (34, 2761),
        2762..=2876 => (35, 2876),
        2877..=3034 => (36, 3034),
        3035..=3196 => (37, 3196),
        3197..=3362 => (38, 3362),
        3363..=3532 => (39, 3532),
        3533..=3706 => (40, 3706),
        _ => {
            eprintln!("lext length {} is too long to be encoded", text_len);
            exit(-1)
        }
    }
}
