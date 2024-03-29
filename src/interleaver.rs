use fastfec::interleaver::qpp::QppInterleaver;

pub const fn new(length: usize) -> Option<QppInterleaver> {
    match length {
        128 => Some(QppInterleaver::new(128, 7, 16)),
        136 => Some(QppInterleaver::new(136, 121, 102)),
        144 => Some(QppInterleaver::new(144, 5, 12)),
        152 => Some(QppInterleaver::new(152, 17, 114)),
        160 => Some(QppInterleaver::new(160, 9, 20)),
        168 => Some(QppInterleaver::new(168, 5, 42)),
        176 => Some(QppInterleaver::new(176, 109, 132)),
        184 => Some(QppInterleaver::new(184, 11, 46)),
        192 => Some(QppInterleaver::new(192, 23, 48)),
        200 => Some(QppInterleaver::new(200, 3, 20)),
        208 => Some(QppInterleaver::new(208, 25, 52)),
        216 => Some(QppInterleaver::new(216, 5, 18)),
        224 => Some(QppInterleaver::new(224, 13, 28)),
        232 => Some(QppInterleaver::new(232, 15, 58)),
        240 => Some(QppInterleaver::new(240, 7, 30)),
        248 => Some(QppInterleaver::new(248, 91, 186)),
        256 => Some(QppInterleaver::new(256, 15, 32)),
        264 => Some(QppInterleaver::new(264, 17, 66)),
        272 => Some(QppInterleaver::new(272, 11, 68)),
        280 => Some(QppInterleaver::new(280, 17, 70)),
        288 => Some(QppInterleaver::new(288, 7, 24)),
        296 => Some(QppInterleaver::new(296, 39, 222)),
        304 => Some(QppInterleaver::new(304, 9, 38)),
        312 => Some(QppInterleaver::new(312, 115, 78)),
        320 => Some(QppInterleaver::new(320, 19, 40)),
        328 => Some(QppInterleaver::new(328, 125, 246)),
        336 => Some(QppInterleaver::new(336, 5, 42)),
        344 => Some(QppInterleaver::new(344, 21, 86)),
        352 => Some(QppInterleaver::new(352, 21, 88)),
        360 => Some(QppInterleaver::new(360, 11, 30)),
        368 => Some(QppInterleaver::new(368, 11, 46)),
        376 => Some(QppInterleaver::new(376, 23, 94)),
        384 => Some(QppInterleaver::new(384, 35, 120)),
        392 => Some(QppInterleaver::new(392, 25, 98)),
        400 => Some(QppInterleaver::new(400, 7, 40)),
        408 => Some(QppInterleaver::new(408, 25, 102)),
        416 => Some(QppInterleaver::new(416, 25, 104)),
        424 => Some(QppInterleaver::new(424, 27, 106)),
        432 => Some(QppInterleaver::new(432, 7, 24)),
        440 => Some(QppInterleaver::new(440, 27, 110)),
        448 => Some(QppInterleaver::new(448, 13, 28)),
        456 => Some(QppInterleaver::new(456, 173, 342)),
        464 => Some(QppInterleaver::new(464, 15, 58)),
        472 => Some(QppInterleaver::new(472, 57, 118)),
        480 => Some(QppInterleaver::new(480, 29, 60)),
        488 => Some(QppInterleaver::new(488, 31, 122)),
        496 => Some(QppInterleaver::new(496, 15, 62)),
        504 => Some(QppInterleaver::new(504, 13, 42)),
        512 => Some(QppInterleaver::new(512, 15, 32)),
        520 => Some(QppInterleaver::new(520, 21, 130)),
        528 => Some(QppInterleaver::new(528, 13, 132)),
        536 => Some(QppInterleaver::new(536, 23, 134)),
        544 => Some(QppInterleaver::new(544, 9, 34)),
        552 => Some(QppInterleaver::new(552, 17, 138)),
        560 => Some(QppInterleaver::new(560, 17, 70)),
        568 => Some(QppInterleaver::new(568, 23, 142)),
        576 => Some(QppInterleaver::new(576, 7, 24)),
        584 => Some(QppInterleaver::new(584, 25, 146)),
        592 => Some(QppInterleaver::new(592, 25, 148)),
        600 => Some(QppInterleaver::new(600, 11, 60)),
        608 => Some(QppInterleaver::new(608, 37, 152)),
        616 => Some(QppInterleaver::new(616, 25, 154)),
        624 => Some(QppInterleaver::new(624, 19, 78)),
        632 => Some(QppInterleaver::new(632, 27, 158)),
        640 => Some(QppInterleaver::new(640, 19, 40)),
        648 => Some(QppInterleaver::new(648, 11, 36)),
        656 => Some(QppInterleaver::new(656, 21, 82)),
        664 => Some(QppInterleaver::new(664, 27, 166)),
        672 => Some(QppInterleaver::new(672, 41, 84)),
        680 => Some(QppInterleaver::new(680, 29, 170)),
        688 => Some(QppInterleaver::new(688, 29, 172)),
        696 => Some(QppInterleaver::new(696, 83, 174)),
        704 => Some(QppInterleaver::new(704, 43, 88)),
        712 => Some(QppInterleaver::new(712, 29, 178)),
        720 => Some(QppInterleaver::new(720, 11, 30)),
        728 => Some(QppInterleaver::new(728, 31, 182)),
        736 => Some(QppInterleaver::new(736, 45, 92)),
        744 => Some(QppInterleaver::new(744, 23, 186)),
        752 => Some(QppInterleaver::new(752, 23, 94)),
        760 => Some(QppInterleaver::new(760, 31, 190)),
        768 => Some(QppInterleaver::new(768, 23, 48)),
        776 => Some(QppInterleaver::new(776, 33, 194)),
        784 => Some(QppInterleaver::new(784, 13, 28)),
        792 => Some(QppInterleaver::new(792, 17, 66)),
        800 => Some(QppInterleaver::new(800, 33, 200)),
        808 => Some(QppInterleaver::new(808, 33, 202)),
        816 => Some(QppInterleaver::new(816, 25, 102)),
        824 => Some(QppInterleaver::new(824, 35, 206)),
        832 => Some(QppInterleaver::new(832, 51, 104)),
        840 => Some(QppInterleaver::new(840, 79, 210)),
        848 => Some(QppInterleaver::new(848, 27, 106)),
        856 => Some(QppInterleaver::new(856, 35, 214)),
        864 => Some(QppInterleaver::new(864, 17, 48)),
        872 => Some(QppInterleaver::new(872, 37, 218)),
        880 => Some(QppInterleaver::new(880, 27, 110)),
        888 => Some(QppInterleaver::new(888, 115, 222)),
        896 => Some(QppInterleaver::new(896, 27, 56)),
        904 => Some(QppInterleaver::new(904, 37, 226)),
        912 => Some(QppInterleaver::new(912, 37, 228)),
        920 => Some(QppInterleaver::new(920, 39, 230)),
        928 => Some(QppInterleaver::new(928, 57, 116)),
        936 => Some(QppInterleaver::new(936, 53, 78)),
        944 => Some(QppInterleaver::new(944, 29, 118)),
        952 => Some(QppInterleaver::new(952, 39, 238)),
        960 => Some(QppInterleaver::new(960, 41, 240)),
        968 => Some(QppInterleaver::new(968, 41, 242)),
        976 => Some(QppInterleaver::new(976, 31, 122)),
        984 => Some(QppInterleaver::new(984, 31, 246)),
        992 => Some(QppInterleaver::new(992, 61, 124)),
        1000 => Some(QppInterleaver::new(1000, 19, 100)),
        1008 => Some(QppInterleaver::new(1008, 13, 42)),
        1016 => Some(QppInterleaver::new(1016, 43, 254)),
        1024 => Some(QppInterleaver::new(1024, 31, 64)),
        1032 => Some(QppInterleaver::new(1032, 97, 258)),
        1040 => Some(QppInterleaver::new(1040, 33, 130)),
        1048 => Some(QppInterleaver::new(1048, 43, 262)),
        1056 => Some(QppInterleaver::new(1056, 43, 264)),
        1064 => Some(QppInterleaver::new(1064, 33, 266)),
        1072 => Some(QppInterleaver::new(1072, 33, 134)),
        1080 => Some(QppInterleaver::new(1080, 19, 60)),
        1088 => Some(QppInterleaver::new(1088, 33, 68)),
        1096 => Some(QppInterleaver::new(1096, 45, 274)),
        1104 => Some(QppInterleaver::new(1104, 35, 138)),
        1112 => Some(QppInterleaver::new(1112, 35, 278)),
        1120 => Some(QppInterleaver::new(1120, 69, 140)),
        1128 => Some(QppInterleaver::new(1128, 35, 282)),
        1136 => Some(QppInterleaver::new(1136, 35, 142)),
        1144 => Some(QppInterleaver::new(1144, 47, 286)),
        1152 => Some(QppInterleaver::new(1152, 23, 48)),
        1160 => Some(QppInterleaver::new(1160, 49, 290)),
        1168 => Some(QppInterleaver::new(1168, 37, 146)),
        1176 => Some(QppInterleaver::new(1176, 11, 84)),
        1184 => Some(QppInterleaver::new(1184, 143, 296)),
        1192 => Some(QppInterleaver::new(1192, 37, 298)),
        1200 => Some(QppInterleaver::new(1200, 23, 120)),
        1208 => Some(QppInterleaver::new(1208, 37, 302)),
        1216 => Some(QppInterleaver::new(1216, 37, 76)),
        1224 => Some(QppInterleaver::new(1224, 67, 102)),
        1232 => Some(QppInterleaver::new(1232, 39, 154)),
        1240 => Some(QppInterleaver::new(1240, 53, 310)),
        1248 => Some(QppInterleaver::new(1248, 77, 156)),
        1256 => Some(QppInterleaver::new(1256, 51, 314)),
        1264 => Some(QppInterleaver::new(1264, 39, 158)),
        1272 => Some(QppInterleaver::new(1272, 119, 318)),
        1280 => Some(QppInterleaver::new(1280, 39, 80)),
        1288 => Some(QppInterleaver::new(1288, 41, 322)),
        1296 => Some(QppInterleaver::new(1296, 23, 72)),
        1304 => Some(QppInterleaver::new(1304, 53, 326)),
        1312 => Some(QppInterleaver::new(1312, 27, 164)),
        1320 => Some(QppInterleaver::new(1320, 41, 330)),
        1328 => Some(QppInterleaver::new(1328, 41, 166)),
        1336 => Some(QppInterleaver::new(1336, 41, 334)),
        1344 => Some(QppInterleaver::new(1344, 41, 84)),
        1352 => Some(QppInterleaver::new(1352, 43, 338)),
        1360 => Some(QppInterleaver::new(1360, 43, 170)),
        1368 => Some(QppInterleaver::new(1368, 77, 114)),
        1376 => Some(QppInterleaver::new(1376, 29, 172)),
        1384 => Some(QppInterleaver::new(1384, 217, 346)),
        1392 => Some(QppInterleaver::new(1392, 43, 174)),
        1400 => Some(QppInterleaver::new(1400, 13, 70)),
        1408 => Some(QppInterleaver::new(1408, 21, 44)),
        1416 => Some(QppInterleaver::new(1416, 35, 354)),
        1424 => Some(QppInterleaver::new(1424, 45, 178)),
        1432 => Some(QppInterleaver::new(1432, 135, 358)),
        1440 => Some(QppInterleaver::new(1440, 29, 60)),
        1448 => Some(QppInterleaver::new(1448, 227, 362)),
        1456 => Some(QppInterleaver::new(1456, 45, 182)),
        1464 => Some(QppInterleaver::new(1464, 37, 366)),
        1472 => Some(QppInterleaver::new(1472, 45, 368)),
        1480 => Some(QppInterleaver::new(1480, 47, 370)),
        1488 => Some(QppInterleaver::new(1488, 47, 186)),
        1496 => Some(QppInterleaver::new(1496, 141, 374)),
        1504 => Some(QppInterleaver::new(1504, 23, 94)),
        1512 => Some(QppInterleaver::new(1512, 29, 84)),
        1520 => Some(QppInterleaver::new(1520, 47, 190)),
        1528 => Some(QppInterleaver::new(1528, 47, 382)),
        1536 => Some(QppInterleaver::new(1536, 47, 96)),
        1544 => Some(QppInterleaver::new(1544, 49, 386)),
        1552 => Some(QppInterleaver::new(1552, 49, 194)),
        1560 => Some(QppInterleaver::new(1560, 49, 390)),
        1568 => Some(QppInterleaver::new(1568, 15, 112)),
        1576 => Some(QppInterleaver::new(1576, 67, 394)),
        1584 => Some(QppInterleaver::new(1584, 47, 132)),
        1592 => Some(QppInterleaver::new(1592, 49, 398)),
        1600 => Some(QppInterleaver::new(1600, 17, 80)),
        1608 => Some(QppInterleaver::new(1608, 103, 402)),
        1616 => Some(QppInterleaver::new(1616, 51, 202)),
        1624 => Some(QppInterleaver::new(1624, 69, 406)),
        1632 => Some(QppInterleaver::new(1632, 35, 204)),
        1640 => Some(QppInterleaver::new(1640, 67, 410)),
        1648 => Some(QppInterleaver::new(1648, 51, 206)),
        1656 => Some(QppInterleaver::new(1656, 91, 138)),
        1664 => Some(QppInterleaver::new(1664, 25, 52)),
        1672 => Some(QppInterleaver::new(1672, 53, 418)),
        1680 => Some(QppInterleaver::new(1680, 53, 210)),
        1688 => Some(QppInterleaver::new(1688, 69, 422)),
        1696 => Some(QppInterleaver::new(1696, 27, 106)),
        1704 => Some(QppInterleaver::new(1704, 43, 426)),
        1712 => Some(QppInterleaver::new(1712, 53, 214)),
        1720 => Some(QppInterleaver::new(1720, 53, 430)),
        1728 => Some(QppInterleaver::new(1728, 31, 288)),
        1736 => Some(QppInterleaver::new(1736, 55, 434)),
        1744 => Some(QppInterleaver::new(1744, 55, 218)),
        1752 => Some(QppInterleaver::new(1752, 107, 438)),
        1760 => Some(QppInterleaver::new(1760, 37, 220)),
        1768 => Some(QppInterleaver::new(1768, 75, 442)),
        1776 => Some(QppInterleaver::new(1776, 55, 222)),
        1784 => Some(QppInterleaver::new(1784, 55, 446)),
        1792 => Some(QppInterleaver::new(1792, 27, 56)),
        1800 => Some(QppInterleaver::new(1800, 17, 90)),
        1808 => Some(QppInterleaver::new(1808, 57, 226)),
        1816 => Some(QppInterleaver::new(1816, 77, 454)),
        1824 => Some(QppInterleaver::new(1824, 37, 228)),
        1832 => Some(QppInterleaver::new(1832, 75, 458)),
        1840 => Some(QppInterleaver::new(1840, 57, 230)),
        1848 => Some(QppInterleaver::new(1848, 323, 462)),
        1856 => Some(QppInterleaver::new(1856, 57, 232)),
        1864 => Some(QppInterleaver::new(1864, 59, 466)),
        1872 => Some(QppInterleaver::new(1872, 17, 156)),
        1880 => Some(QppInterleaver::new(1880, 77, 470)),
        1888 => Some(QppInterleaver::new(1888, 29, 118)),
        1896 => Some(QppInterleaver::new(1896, 47, 474)),
        1904 => Some(QppInterleaver::new(1904, 59, 238)),
        1912 => Some(QppInterleaver::new(1912, 59, 478)),
        1920 => Some(QppInterleaver::new(1920, 29, 60)),
        1928 => Some(QppInterleaver::new(1928, 61, 482)),
        1936 => Some(QppInterleaver::new(1936, 61, 242)),
        1944 => Some(QppInterleaver::new(1944, 35, 108)),
        1952 => Some(QppInterleaver::new(1952, 31, 122)),
        1960 => Some(QppInterleaver::new(1960, 19, 140)),
        1968 => Some(QppInterleaver::new(1968, 61, 246)),
        1976 => Some(QppInterleaver::new(1976, 49, 494)),
        1984 => Some(QppInterleaver::new(1984, 15, 62)),
        1992 => Some(QppInterleaver::new(1992, 127, 498)),
        2000 => Some(QppInterleaver::new(2000, 19, 100)),
        2008 => Some(QppInterleaver::new(2008, 85, 502)),
        2016 => Some(QppInterleaver::new(2016, 41, 84)),
        2024 => Some(QppInterleaver::new(2024, 51, 506)),
        2032 => Some(QppInterleaver::new(2032, 63, 254)),
        2040 => Some(QppInterleaver::new(2040, 43, 510)),
        _ => None,
    }
}
