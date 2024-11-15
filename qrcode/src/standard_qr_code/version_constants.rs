use std::u16;

use crate::input::ErrorLevel;

use super::qr_struct::ErrorBlockInfo;

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
            for block in memory_case.1 {
                current_memory += block.num_block as u16 * block.total_block_len as u16;
            }
            if current_memory != loop_version.1 {
                println!(
                    "version {} has a size of {} bytes but the {:?} block takes up {} bytes",
                    version, loop_version.1, memory_case.0, current_memory
                );
                panic!();
            }
        }
    }
}
