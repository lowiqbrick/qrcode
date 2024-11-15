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
                (ErrorLevel::H, vec![ErrorBlockInfo::new(4, 243, 15)]),
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
    ];
    all_error_info
}
