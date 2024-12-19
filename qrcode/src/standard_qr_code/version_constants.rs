use std::u16;

use crate::input::ErrorLevel;

use super::qr_struct::ErrorBlockInfo;

/// delivers a vector containing tuples which contain
/// (version number, codewords available, error block information)
pub fn get_error_block_info() -> Vec<(u8, u16, Vec<(ErrorLevel, Vec<ErrorBlockInfo>)>)> {
    let all_error_info: Vec<(u8, u16, Vec<(ErrorLevel, Vec<ErrorBlockInfo>)>)> = vec![
        (
            1,
            26,
            vec![
                (ErrorLevel::L, vec![ErrorBlockInfo::new(1, 26, 19)]),
                (ErrorLevel::M, vec![ErrorBlockInfo::new(1, 26, 16)]),
                (ErrorLevel::Q, vec![ErrorBlockInfo::new(1, 26, 13)]),
                (ErrorLevel::H, vec![ErrorBlockInfo::new(1, 26, 9)]),
            ],
        ),
        (
            2,
            44,
            vec![
                (ErrorLevel::L, vec![ErrorBlockInfo::new(1, 44, 34)]),
                (ErrorLevel::M, vec![ErrorBlockInfo::new(1, 44, 28)]),
                (ErrorLevel::Q, vec![ErrorBlockInfo::new(1, 44, 22)]),
                (ErrorLevel::H, vec![ErrorBlockInfo::new(1, 44, 16)]),
            ],
        ),
        (
            3,
            70,
            vec![
                (ErrorLevel::L, vec![ErrorBlockInfo::new(1, 70, 55)]),
                (ErrorLevel::M, vec![ErrorBlockInfo::new(1, 70, 44)]),
                (ErrorLevel::Q, vec![ErrorBlockInfo::new(2, 35, 17)]),
                (ErrorLevel::H, vec![ErrorBlockInfo::new(2, 35, 13)]),
            ],
        ),
        (
            4,
            100,
            vec![
                (ErrorLevel::L, vec![ErrorBlockInfo::new(1, 100, 80)]),
                (ErrorLevel::M, vec![ErrorBlockInfo::new(2, 50, 32)]),
                (ErrorLevel::Q, vec![ErrorBlockInfo::new(2, 50, 24)]),
                (ErrorLevel::H, vec![ErrorBlockInfo::new(4, 25, 9)]),
            ],
        ),
        (
            5,
            134,
            vec![
                (ErrorLevel::L, vec![ErrorBlockInfo::new(1, 134, 108)]),
                (ErrorLevel::M, vec![ErrorBlockInfo::new(2, 67, 43)]),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(2, 33, 15),
                        ErrorBlockInfo::new(2, 34, 16),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(2, 33, 11),
                        ErrorBlockInfo::new(2, 34, 12),
                    ],
                ),
            ],
        ),
        (
            6,
            172,
            vec![
                (ErrorLevel::L, vec![ErrorBlockInfo::new(2, 86, 68)]),
                (ErrorLevel::M, vec![ErrorBlockInfo::new(4, 43, 27)]),
                (ErrorLevel::Q, vec![ErrorBlockInfo::new(4, 43, 19)]),
                (ErrorLevel::H, vec![ErrorBlockInfo::new(4, 43, 15)]),
            ],
        ),
        (
            7,
            196,
            vec![
                (ErrorLevel::L, vec![ErrorBlockInfo::new(2, 98, 78)]),
                (ErrorLevel::M, vec![ErrorBlockInfo::new(4, 49, 31)]),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(2, 32, 14),
                        ErrorBlockInfo::new(4, 33, 15),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(4, 39, 13),
                        ErrorBlockInfo::new(1, 40, 14),
                    ],
                ),
            ],
        ),
        (
            8,
            242,
            vec![
                (ErrorLevel::L, vec![ErrorBlockInfo::new(2, 121, 97)]),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(2, 60, 38),
                        ErrorBlockInfo::new(2, 61, 39),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(4, 40, 18),
                        ErrorBlockInfo::new(2, 41, 19),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(4, 40, 14),
                        ErrorBlockInfo::new(2, 41, 15),
                    ],
                ),
            ],
        ),
        (
            9,
            292,
            vec![
                (ErrorLevel::L, vec![ErrorBlockInfo::new(2, 146, 116)]),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(3, 58, 36),
                        ErrorBlockInfo::new(2, 59, 37),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(4, 36, 16),
                        ErrorBlockInfo::new(4, 37, 17),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(4, 36, 12),
                        ErrorBlockInfo::new(4, 37, 13),
                    ],
                ),
            ],
        ),
        (
            10,
            346,
            vec![
                (
                    ErrorLevel::L,
                    vec![
                        ErrorBlockInfo::new(2, 86, 68),
                        ErrorBlockInfo::new(2, 87, 69),
                    ],
                ),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(4, 69, 43),
                        ErrorBlockInfo::new(1, 70, 44),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(6, 43, 19),
                        ErrorBlockInfo::new(2, 44, 20),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(6, 43, 15),
                        ErrorBlockInfo::new(2, 44, 16),
                    ],
                ),
            ],
        ),
        (
            11,
            404,
            vec![
                (ErrorLevel::L, vec![ErrorBlockInfo::new(4, 101, 81)]),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(1, 80, 50),
                        ErrorBlockInfo::new(4, 81, 51),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(4, 50, 22),
                        ErrorBlockInfo::new(4, 51, 23),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(3, 36, 12),
                        ErrorBlockInfo::new(8, 37, 13),
                    ],
                ),
            ],
        ),
        (
            12,
            466,
            vec![
                (
                    ErrorLevel::L,
                    vec![
                        ErrorBlockInfo::new(2, 116, 92),
                        ErrorBlockInfo::new(2, 117, 93),
                    ],
                ),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(6, 58, 36),
                        ErrorBlockInfo::new(2, 59, 37),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(4, 46, 20),
                        ErrorBlockInfo::new(6, 47, 21),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(7, 42, 14),
                        ErrorBlockInfo::new(4, 43, 15),
                    ],
                ),
            ],
        ),
        (
            13,
            532,
            vec![
                (ErrorLevel::L, vec![ErrorBlockInfo::new(4, 133, 107)]),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(8, 59, 37),
                        ErrorBlockInfo::new(1, 60, 38),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(8, 44, 20),
                        ErrorBlockInfo::new(4, 45, 21),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(12, 33, 11),
                        ErrorBlockInfo::new(4, 34, 12),
                    ],
                ),
            ],
        ),
        (
            14,
            581,
            vec![
                (
                    ErrorLevel::L,
                    vec![
                        ErrorBlockInfo::new(3, 145, 115),
                        ErrorBlockInfo::new(1, 146, 116),
                    ],
                ),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(4, 64, 40),
                        ErrorBlockInfo::new(5, 65, 41),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(11, 36, 16),
                        ErrorBlockInfo::new(5, 37, 17),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(11, 36, 12),
                        ErrorBlockInfo::new(5, 37, 13),
                    ],
                ),
            ],
        ),
        (
            15,
            655,
            vec![
                (
                    ErrorLevel::L,
                    vec![
                        ErrorBlockInfo::new(5, 109, 87),
                        ErrorBlockInfo::new(1, 110, 88),
                    ],
                ),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(5, 65, 41),
                        ErrorBlockInfo::new(5, 66, 42),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(5, 54, 24),
                        ErrorBlockInfo::new(7, 55, 25),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(11, 36, 12),
                        ErrorBlockInfo::new(7, 37, 13),
                    ],
                ),
            ],
        ),
        (
            16,
            733,
            vec![
                (
                    ErrorLevel::L,
                    vec![
                        ErrorBlockInfo::new(5, 122, 98),
                        ErrorBlockInfo::new(1, 123, 99),
                    ],
                ),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(7, 73, 45),
                        ErrorBlockInfo::new(3, 74, 46),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(15, 43, 19),
                        ErrorBlockInfo::new(2, 44, 20),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(3, 45, 15),
                        ErrorBlockInfo::new(13, 46, 16),
                    ],
                ),
            ],
        ),
        (
            17,
            815,
            vec![
                (
                    ErrorLevel::L,
                    vec![
                        ErrorBlockInfo::new(1, 135, 107),
                        ErrorBlockInfo::new(5, 136, 108),
                    ],
                ),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(10, 74, 46),
                        ErrorBlockInfo::new(1, 75, 47),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(1, 50, 22),
                        ErrorBlockInfo::new(15, 51, 23),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(2, 42, 14),
                        ErrorBlockInfo::new(17, 43, 15),
                    ],
                ),
            ],
        ),
        (
            18,
            901,
            vec![
                (
                    ErrorLevel::L,
                    vec![
                        ErrorBlockInfo::new(5, 150, 120),
                        ErrorBlockInfo::new(1, 151, 121),
                    ],
                ),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(9, 69, 43),
                        ErrorBlockInfo::new(4, 70, 44),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(17, 50, 22),
                        ErrorBlockInfo::new(1, 51, 23),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(2, 42, 14),
                        ErrorBlockInfo::new(19, 43, 15),
                    ],
                ),
            ],
        ),
        (
            19,
            991,
            vec![
                (
                    ErrorLevel::L,
                    vec![
                        ErrorBlockInfo::new(3, 141, 113),
                        ErrorBlockInfo::new(4, 142, 114),
                    ],
                ),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(3, 70, 44),
                        ErrorBlockInfo::new(11, 71, 45),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(17, 47, 21),
                        ErrorBlockInfo::new(4, 48, 22),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(9, 39, 13),
                        ErrorBlockInfo::new(16, 40, 14),
                    ],
                ),
            ],
        ),
        (
            20,
            1085,
            vec![
                (
                    ErrorLevel::L,
                    vec![
                        ErrorBlockInfo::new(3, 135, 107),
                        ErrorBlockInfo::new(5, 136, 108),
                    ],
                ),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(3, 67, 41),
                        ErrorBlockInfo::new(13, 68, 42),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(15, 54, 24),
                        ErrorBlockInfo::new(5, 55, 25),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(15, 43, 15),
                        ErrorBlockInfo::new(10, 44, 16),
                    ],
                ),
            ],
        ),
        (
            21,
            1156,
            vec![
                (
                    ErrorLevel::L,
                    vec![
                        ErrorBlockInfo::new(4, 144, 116),
                        ErrorBlockInfo::new(4, 145, 117),
                    ],
                ),
                (ErrorLevel::M, vec![ErrorBlockInfo::new(17, 68, 42)]),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(17, 50, 22),
                        ErrorBlockInfo::new(6, 51, 23),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(19, 46, 16),
                        ErrorBlockInfo::new(6, 47, 17),
                    ],
                ),
            ],
        ),
        (
            22,
            1258,
            vec![
                (
                    ErrorLevel::L,
                    vec![
                        ErrorBlockInfo::new(2, 139, 111),
                        ErrorBlockInfo::new(7, 140, 112),
                    ],
                ),
                (ErrorLevel::M, vec![ErrorBlockInfo::new(17, 74, 46)]),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(7, 54, 24),
                        ErrorBlockInfo::new(16, 55, 25),
                    ],
                ),
                (ErrorLevel::H, vec![ErrorBlockInfo::new(34, 37, 13)]),
            ],
        ),
        (
            23,
            1364,
            vec![
                (
                    ErrorLevel::L,
                    vec![
                        ErrorBlockInfo::new(4, 151, 121),
                        ErrorBlockInfo::new(5, 152, 122),
                    ],
                ),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(4, 75, 47),
                        ErrorBlockInfo::new(14, 76, 48),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(11, 54, 24),
                        ErrorBlockInfo::new(14, 55, 25),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(16, 45, 15),
                        ErrorBlockInfo::new(14, 46, 16),
                    ],
                ),
            ],
        ),
        (
            24,
            1474,
            vec![
                (
                    ErrorLevel::L,
                    vec![
                        ErrorBlockInfo::new(6, 147, 117),
                        ErrorBlockInfo::new(4, 148, 118),
                    ],
                ),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(6, 73, 45),
                        ErrorBlockInfo::new(14, 74, 46),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(11, 54, 24),
                        ErrorBlockInfo::new(16, 55, 25),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(30, 46, 16),
                        ErrorBlockInfo::new(2, 47, 17),
                    ],
                ),
            ],
        ),
        (
            25,
            1588,
            vec![
                (
                    ErrorLevel::L,
                    vec![
                        ErrorBlockInfo::new(8, 132, 106),
                        ErrorBlockInfo::new(4, 133, 107),
                    ],
                ),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(8, 75, 47),
                        ErrorBlockInfo::new(13, 76, 48),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(7, 54, 24),
                        ErrorBlockInfo::new(22, 55, 25),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(22, 45, 15),
                        ErrorBlockInfo::new(13, 46, 16),
                    ],
                ),
            ],
        ),
        (
            26,
            1706,
            vec![
                (
                    ErrorLevel::L,
                    vec![
                        ErrorBlockInfo::new(10, 142, 114),
                        ErrorBlockInfo::new(2, 143, 115),
                    ],
                ),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(19, 74, 46),
                        ErrorBlockInfo::new(4, 75, 47),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(28, 50, 22),
                        ErrorBlockInfo::new(6, 51, 23),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(33, 46, 16),
                        ErrorBlockInfo::new(4, 47, 17),
                    ],
                ),
            ],
        ),
        (
            27,
            1828,
            vec![
                (
                    ErrorLevel::L,
                    vec![
                        ErrorBlockInfo::new(8, 152, 122),
                        ErrorBlockInfo::new(4, 153, 123),
                    ],
                ),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(22, 73, 45),
                        ErrorBlockInfo::new(3, 74, 46),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(8, 53, 23),
                        ErrorBlockInfo::new(26, 54, 24),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(12, 45, 15),
                        ErrorBlockInfo::new(28, 46, 16),
                    ],
                ),
            ],
        ),
        (
            28,
            1921,
            vec![
                (
                    ErrorLevel::L,
                    vec![
                        ErrorBlockInfo::new(3, 147, 117),
                        ErrorBlockInfo::new(10, 148, 118),
                    ],
                ),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(3, 73, 45),
                        ErrorBlockInfo::new(23, 74, 46),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(4, 54, 24),
                        ErrorBlockInfo::new(31, 55, 25),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(11, 45, 15),
                        ErrorBlockInfo::new(31, 46, 16),
                    ],
                ),
            ],
        ),
        (
            29,
            2051,
            vec![
                (
                    ErrorLevel::L,
                    vec![
                        ErrorBlockInfo::new(7, 146, 116),
                        ErrorBlockInfo::new(7, 147, 117),
                    ],
                ),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(21, 73, 45),
                        ErrorBlockInfo::new(7, 74, 46),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(1, 53, 23),
                        ErrorBlockInfo::new(37, 54, 24),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(19, 45, 15),
                        ErrorBlockInfo::new(26, 46, 16),
                    ],
                ),
            ],
        ),
        (
            30,
            2185,
            vec![
                (
                    ErrorLevel::L,
                    vec![
                        ErrorBlockInfo::new(5, 145, 115),
                        ErrorBlockInfo::new(10, 146, 116),
                    ],
                ),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(19, 75, 47),
                        ErrorBlockInfo::new(10, 76, 48),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(15, 54, 24),
                        ErrorBlockInfo::new(25, 55, 25),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(23, 45, 15),
                        ErrorBlockInfo::new(25, 46, 16),
                    ],
                ),
            ],
        ),
        (
            31,
            2323,
            vec![
                (
                    ErrorLevel::L,
                    vec![
                        ErrorBlockInfo::new(13, 145, 115),
                        ErrorBlockInfo::new(3, 146, 116),
                    ],
                ),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(2, 74, 46),
                        ErrorBlockInfo::new(29, 75, 47),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(42, 54, 24),
                        ErrorBlockInfo::new(1, 55, 25),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(23, 45, 15),
                        ErrorBlockInfo::new(28, 46, 16),
                    ],
                ),
            ],
        ),
        (
            32,
            2465,
            vec![
                (ErrorLevel::L, vec![ErrorBlockInfo::new(17, 145, 115)]),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(10, 74, 46),
                        ErrorBlockInfo::new(23, 75, 47),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(10, 54, 24),
                        ErrorBlockInfo::new(35, 55, 25),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(19, 45, 15),
                        ErrorBlockInfo::new(35, 46, 16),
                    ],
                ),
            ],
        ),
        (
            33,
            2611,
            vec![
                (
                    ErrorLevel::L,
                    vec![
                        ErrorBlockInfo::new(17, 145, 115),
                        ErrorBlockInfo::new(1, 146, 116),
                    ],
                ),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(14, 74, 46),
                        ErrorBlockInfo::new(21, 75, 47),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(29, 54, 24),
                        ErrorBlockInfo::new(19, 55, 25),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(11, 45, 15),
                        ErrorBlockInfo::new(46, 46, 16),
                    ],
                ),
            ],
        ),
        (
            34,
            2761,
            vec![
                (
                    ErrorLevel::L,
                    vec![
                        ErrorBlockInfo::new(13, 145, 115),
                        ErrorBlockInfo::new(6, 146, 116),
                    ],
                ),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(14, 74, 46),
                        ErrorBlockInfo::new(23, 75, 47),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(44, 54, 24),
                        ErrorBlockInfo::new(7, 55, 25),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(59, 46, 16),
                        ErrorBlockInfo::new(1, 47, 17),
                    ],
                ),
            ],
        ),
        (
            35,
            2876,
            vec![
                (
                    ErrorLevel::L,
                    vec![
                        ErrorBlockInfo::new(12, 151, 121),
                        ErrorBlockInfo::new(7, 152, 122),
                    ],
                ),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(12, 75, 47),
                        ErrorBlockInfo::new(26, 76, 48),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(39, 54, 24),
                        ErrorBlockInfo::new(14, 55, 25),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(22, 45, 15),
                        ErrorBlockInfo::new(41, 46, 16),
                    ],
                ),
            ],
        ),
        (
            36,
            3034,
            vec![
                (
                    ErrorLevel::L,
                    vec![
                        ErrorBlockInfo::new(6, 151, 121),
                        ErrorBlockInfo::new(14, 152, 122),
                    ],
                ),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(6, 75, 47),
                        ErrorBlockInfo::new(34, 76, 48),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(46, 54, 24),
                        ErrorBlockInfo::new(10, 55, 25),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(2, 45, 15),
                        ErrorBlockInfo::new(64, 46, 16),
                    ],
                ),
            ],
        ),
        (
            37,
            3196,
            vec![
                (
                    ErrorLevel::L,
                    vec![
                        ErrorBlockInfo::new(17, 152, 122),
                        ErrorBlockInfo::new(4, 153, 123),
                    ],
                ),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(29, 74, 46),
                        ErrorBlockInfo::new(14, 75, 47),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(49, 54, 24),
                        ErrorBlockInfo::new(10, 55, 25),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(24, 45, 15),
                        ErrorBlockInfo::new(46, 46, 16),
                    ],
                ),
            ],
        ),
        (
            38,
            3362,
            vec![
                (
                    ErrorLevel::L,
                    vec![
                        ErrorBlockInfo::new(4, 152, 122),
                        ErrorBlockInfo::new(18, 153, 123),
                    ],
                ),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(13, 74, 46),
                        ErrorBlockInfo::new(32, 75, 47),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(48, 54, 24),
                        ErrorBlockInfo::new(14, 55, 25),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(42, 45, 15),
                        ErrorBlockInfo::new(32, 46, 16),
                    ],
                ),
            ],
        ),
        (
            39,
            3532,
            vec![
                (
                    ErrorLevel::L,
                    vec![
                        ErrorBlockInfo::new(20, 147, 117),
                        ErrorBlockInfo::new(4, 148, 118),
                    ],
                ),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(40, 75, 47),
                        ErrorBlockInfo::new(7, 76, 48),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(43, 54, 24),
                        ErrorBlockInfo::new(22, 55, 25),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(10, 45, 15),
                        ErrorBlockInfo::new(67, 46, 16),
                    ],
                ),
            ],
        ),
        (
            40,
            3706,
            vec![
                (
                    ErrorLevel::L,
                    vec![
                        ErrorBlockInfo::new(19, 148, 118),
                        ErrorBlockInfo::new(6, 149, 119),
                    ],
                ),
                (
                    ErrorLevel::M,
                    vec![
                        ErrorBlockInfo::new(18, 75, 47),
                        ErrorBlockInfo::new(31, 76, 48),
                    ],
                ),
                (
                    ErrorLevel::Q,
                    vec![
                        ErrorBlockInfo::new(34, 54, 24),
                        ErrorBlockInfo::new(34, 55, 25),
                    ],
                ),
                (
                    ErrorLevel::H,
                    vec![
                        ErrorBlockInfo::new(20, 45, 15),
                        ErrorBlockInfo::new(61, 46, 16),
                    ],
                ),
            ],
        ),
    ];
    all_error_info
}

/// function that returns a tuple which contains number of alignment patterns
/// and their centre coordinates
pub fn alignment_pattern_data(version: u8) -> (u8, Vec<u8>) {
    match version {
        1 => (0, vec![]),
        2 => (1, vec![6, 18]),
        3 => (1, vec![6, 22]),
        4 => (1, vec![6, 26]),
        5 => (1, vec![6, 30]),
        6 => (1, vec![6, 34]),
        7 => (6, vec![6, 22, 38]),
        8 => (6, vec![6, 24, 42]),
        9 => (6, vec![6, 26, 46]),
        10 => (6, vec![6, 28, 50]),
        11 => (6, vec![6, 30, 54]),
        12 => (6, vec![6, 32, 58]),
        13 => (6, vec![6, 34, 62]),
        14 => (13, vec![6, 26, 46, 66]),
        15 => (13, vec![6, 26, 48, 70]),
        16 => (13, vec![6, 26, 50, 74]),
        17 => (13, vec![6, 30, 54, 78]),
        18 => (13, vec![6, 30, 56, 82]),
        19 => (13, vec![6, 30, 58, 86]),
        20 => (13, vec![6, 34, 62, 90]),
        21 => (22, vec![6, 28, 50, 72, 94]),
        22 => (22, vec![6, 26, 50, 74, 98]),
        23 => (22, vec![6, 30, 54, 78, 102]),
        24 => (22, vec![6, 28, 54, 80, 106]),
        25 => (22, vec![6, 32, 58, 84, 110]),
        26 => (22, vec![6, 30, 58, 86, 114]),
        27 => (22, vec![6, 34, 62, 90, 118]),
        28 => (33, vec![6, 26, 50, 74, 98, 122]),
        29 => (33, vec![6, 30, 54, 78, 102, 126]),
        30 => (33, vec![6, 26, 52, 78, 104, 130]),
        31 => (33, vec![6, 30, 56, 82, 108, 134]),
        32 => (33, vec![6, 34, 60, 86, 112, 138]),
        33 => (33, vec![6, 30, 58, 86, 114, 142]),
        34 => (33, vec![6, 34, 62, 90, 118, 146]),
        35 => (46, vec![6, 30, 54, 78, 102, 126, 150]),
        36 => (46, vec![6, 24, 50, 76, 102, 128, 154]),
        37 => (46, vec![6, 28, 54, 80, 106, 132, 158]),
        38 => (46, vec![6, 32, 58, 84, 110, 136, 162]),
        39 => (46, vec![6, 26, 54, 82, 110, 138, 166]),
        40 => (46, vec![6, 30, 58, 86, 114, 142, 170]),
        _ => {
            eprintln!("version number {}", version);
            panic!();
        }
    }
}

/// contains all bit streams for all versions
pub fn version_info(version: u8) -> u32 {
    match version {
        7 => 0x07C94,
        8 => 0x085BC,
        9 => 0x09A99,
        10 => 0x0A4D3,
        11 => 0x0BBF6,
        12 => 0x0C762,
        13 => 0x0D847,
        14 => 0x0E60D,
        15 => 0x0F928,
        16 => 0x10B78,
        17 => 0x1145D,
        18 => 0x12A17,
        19 => 0x13532,
        20 => 0x149A6,
        21 => 0x15683,
        22 => 0x168C9,
        23 => 0x177EC,
        24 => 0x18EC4,
        25 => 0x191E1,
        26 => 0x1AFAB,
        27 => 0x1B08E,
        28 => 0x1CC1A,
        29 => 0x1D33F,
        30 => 0x1ED75,
        31 => 0x1F250,
        32 => 0x209D5,
        33 => 0x216F0,
        34 => 0x228BA,
        35 => 0x2379F,
        36 => 0x24B0B,
        37 => 0x2542E,
        38 => 0x26A64,
        39 => 0x27541,
        40 => 0x28C69,
        _ => panic!("invlaid version handed to version information {}", version),
    }
}

/// valid formant information bit sequences
pub fn information_sequences(data: u8) -> u16 {
    match data {
        0 => 0x5412,
        1 => 0x5125,
        2 => 0x5E7C,
        3 => 0x5B4B,
        4 => 0x45F9,
        5 => 0x40CE,
        6 => 0x4F97,
        7 => 0x4AA0,
        8 => 0x77C4,
        9 => 0x72F3,
        10 => 0x7DAA,
        11 => 0x789D,
        12 => 0x662F,
        13 => 0x6318,
        14 => 0x6C41,
        15 => 0x6976,
        16 => 0x1689,
        17 => 0x13BE,
        18 => 0x1CE7,
        19 => 0x19D0,
        20 => 0x0762,
        21 => 0x0255,
        22 => 0x0D0C,
        23 => 0x083B,
        24 => 0x355F,
        25 => 0x3068,
        26 => 0x3F31,
        27 => 0x3A06,
        28 => 0x24B4,
        29 => 0x2183,
        30 => 0x2EDA,
        31 => 0x2BED,
        _ => panic!(
            "an invalid value given for information bit sequences {}",
            data
        ),
    }
}

#[test]
#[cfg(test)]
fn sanitycheck_version_information() {
    let info: Vec<(u8, u16, Vec<(ErrorLevel, Vec<ErrorBlockInfo>)>)> = get_error_block_info();
    assert_eq!(info.len(), 40);
    let mut version: i16 = 0;
    let mut size: u16 = 0;
    for loop_version in info.clone() {
        // check that the numbers go up by one
        if loop_version.0 as i16 != version + 1 {
            panic!();
        } else {
            version = loop_version.0 as i16;
        }
        // check that size goes up
        if loop_version.1 <= size {
            println!(
                "last version number was {} the current one is {}",
                size, loop_version.1
            );
            panic!();
        } else {
            size = loop_version.1
        }
        // make shure that the blocks summed up have the size of the version
        for memory_case in loop_version.2.clone() {
            let mut current_memory: u16 = 0;
            for block in memory_case.1.iter() {
                let total_length = block.num_data_bytes + block.num_error_bytes;
                current_memory += block.num_block as u16 * total_length as u16;
                assert_eq!(total_length, block.num_data_bytes + block.num_error_bytes);
            }
            if current_memory != loop_version.1 {
                println!(
                    "version {} has a size of {} bytes but the {:?} block takes up {} bytes",
                    version, loop_version.1, memory_case.0, current_memory
                );
                panic!();
            }
            // assert that there can't be more than two error blocks per version
            assert!(memory_case.1.len() <= 2 && memory_case.1.len() > 0);
        }
    }
}

#[test]
#[cfg(test)]
// if there are more than tree rows/columns of alignment patterns
// there sould be spaced out equally from one another
fn sanity_check_alignment_pattern() {
    for version in 1..=40 {
        eprintln!("version: {}", version);
        let version_data: (u8, Vec<u8>) = alignment_pattern_data(version);
        if version_data.1.len() >= 3 {
            if version_data.1.len() == 3 {
                let difference = version_data.1[1] - version_data.1[0];
                for index in 0..version_data.1.len() - 1 {
                    assert_eq!(
                        difference,
                        version_data.1[index + 1] - version_data.1[index]
                    );
                }
            } else {
                let difference = version_data.1[2] - version_data.1[1];
                for index in 1..version_data.1.len() - 1 {
                    assert_eq!(
                        difference,
                        version_data.1[index + 1] - version_data.1[index]
                    );
                }
            }
        }
    }
}
