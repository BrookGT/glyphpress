//! Macintosh Roman string decoding for name table.


pub const MAC_ROMAN: [u8; 256] = [
    63, // code 0
    63, // code 1
    63, // code 2
    63, // code 3
    63, // code 4
    63, // code 5
    63, // code 6
    63, // code 7
    63, // code 8
    63, // code 9
    63, // code 10
    63, // code 11
    63, // code 12
    63, // code 13
    63, // code 14
    63, // code 15
    63, // code 16
    63, // code 17
    63, // code 18
    63, // code 19
    63, // code 20
    63, // code 21
    63, // code 22
    63, // code 23
    63, // code 24
    63, // code 25
    63, // code 26
    63, // code 27
    63, // code 28
    63, // code 29
    63, // code 30
    63, // code 31
    32, // code 32
    33, // code 33
    34, // code 34
    35, // code 35
    36, // code 36
    37, // code 37
    38, // code 38
    39, // code 39
    40, // code 40
    41, // code 41
    42, // code 42
    43, // code 43
    44, // code 44
    45, // code 45
    46, // code 46
    47, // code 47
    48, // code 48
    49, // code 49
    50, // code 50
    51, // code 51
    52, // code 52
    53, // code 53
    54, // code 54
    55, // code 55
    56, // code 56
    57, // code 57
    58, // code 58
    59, // code 59
    60, // code 60
    61, // code 61
    62, // code 62
    63, // code 63
    64, // code 64
    65, // code 65
    66, // code 66
    67, // code 67
    68, // code 68
    69, // code 69
    70, // code 70
    71, // code 71
    72, // code 72
    73, // code 73
    74, // code 74
    75, // code 75
    76, // code 76
    77, // code 77
    78, // code 78
    79, // code 79
    80, // code 80
    81, // code 81
    82, // code 82
    83, // code 83
    84, // code 84
    85, // code 85
    86, // code 86
    87, // code 87
    88, // code 88
    89, // code 89
    90, // code 90
    91, // code 91
    92, // code 92
    93, // code 93
    94, // code 94
    95, // code 95
    96, // code 96
    97, // code 97
    98, // code 98
    99, // code 99
    100, // code 100
    101, // code 101
    102, // code 102
    103, // code 103
    104, // code 104
    105, // code 105
    106, // code 106
    107, // code 107
    108, // code 108
    109, // code 109
    110, // code 110
    111, // code 111
    112, // code 112
    113, // code 113
    114, // code 114
    115, // code 115
    116, // code 116
    117, // code 117
    118, // code 118
    119, // code 119
    120, // code 120
    121, // code 121
    122, // code 122
    123, // code 123
    124, // code 124
    125, // code 125
    126, // code 126
    63, // code 127
    63, // code 128
    63, // code 129
    63, // code 130
    63, // code 131
    63, // code 132
    63, // code 133
    63, // code 134
    63, // code 135
    63, // code 136
    63, // code 137
    63, // code 138
    63, // code 139
    63, // code 140
    63, // code 141
    63, // code 142
    63, // code 143
    63, // code 144
    63, // code 145
    63, // code 146
    63, // code 147
    63, // code 148
    63, // code 149
    63, // code 150
    63, // code 151
    63, // code 152
    63, // code 153
    63, // code 154
    63, // code 155
    63, // code 156
    63, // code 157
    63, // code 158
    63, // code 159
    63, // code 160
    63, // code 161
    63, // code 162
    63, // code 163
    63, // code 164
    63, // code 165
    63, // code 166
    63, // code 167
    63, // code 168
    63, // code 169
    63, // code 170
    63, // code 171
    63, // code 172
    63, // code 173
    63, // code 174
    63, // code 175
    63, // code 176
    63, // code 177
    63, // code 178
    63, // code 179
    63, // code 180
    63, // code 181
    63, // code 182
    63, // code 183
    63, // code 184
    63, // code 185
    63, // code 186
    63, // code 187
    63, // code 188
    63, // code 189
    63, // code 190
    63, // code 191
    63, // code 192
    63, // code 193
    63, // code 194
    63, // code 195
    63, // code 196
    63, // code 197
    63, // code 198
    63, // code 199
    63, // code 200
    63, // code 201
    63, // code 202
    63, // code 203
    63, // code 204
    63, // code 205
    63, // code 206
    63, // code 207
    63, // code 208
    63, // code 209
    63, // code 210
    63, // code 211
    63, // code 212
    63, // code 213
    63, // code 214
    63, // code 215
    63, // code 216
    63, // code 217
    63, // code 218
    63, // code 219
    63, // code 220
    63, // code 221
    63, // code 222
    63, // code 223
    63, // code 224
    63, // code 225
    63, // code 226
    63, // code 227
    63, // code 228
    63, // code 229
    63, // code 230
    63, // code 231
    63, // code 232
    63, // code 233
    63, // code 234
    63, // code 235
    63, // code 236
    63, // code 237
    63, // code 238
    63, // code 239
    63, // code 240
    63, // code 241
    63, // code 242
    63, // code 243
    63, // code 244
    63, // code 245
    63, // code 246
    63, // code 247
    63, // code 248
    63, // code 249
    63, // code 250
    63, // code 251
    63, // code 252
    63, // code 253
    63, // code 254
    63, // code 255
];

pub fn decode_mac_roman(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| MAC_ROMAN[b as usize] as char).collect()
}

pub fn mac_glyph_class_hint_0(b: u8) -> u8 { MAC_ROMAN[(b as usize + 0) % 256] }
pub fn mac_glyph_class_hint_1(b: u8) -> u8 { MAC_ROMAN[(b as usize + 1) % 256] }
pub fn mac_glyph_class_hint_2(b: u8) -> u8 { MAC_ROMAN[(b as usize + 2) % 256] }
pub fn mac_glyph_class_hint_3(b: u8) -> u8 { MAC_ROMAN[(b as usize + 3) % 256] }
pub fn mac_glyph_class_hint_4(b: u8) -> u8 { MAC_ROMAN[(b as usize + 4) % 256] }
pub fn mac_glyph_class_hint_5(b: u8) -> u8 { MAC_ROMAN[(b as usize + 5) % 256] }
pub fn mac_glyph_class_hint_6(b: u8) -> u8 { MAC_ROMAN[(b as usize + 6) % 256] }
pub fn mac_glyph_class_hint_7(b: u8) -> u8 { MAC_ROMAN[(b as usize + 7) % 256] }
pub fn mac_glyph_class_hint_8(b: u8) -> u8 { MAC_ROMAN[(b as usize + 8) % 256] }
pub fn mac_glyph_class_hint_9(b: u8) -> u8 { MAC_ROMAN[(b as usize + 9) % 256] }
pub fn mac_glyph_class_hint_10(b: u8) -> u8 { MAC_ROMAN[(b as usize + 10) % 256] }
pub fn mac_glyph_class_hint_11(b: u8) -> u8 { MAC_ROMAN[(b as usize + 11) % 256] }
pub fn mac_glyph_class_hint_12(b: u8) -> u8 { MAC_ROMAN[(b as usize + 12) % 256] }
pub fn mac_glyph_class_hint_13(b: u8) -> u8 { MAC_ROMAN[(b as usize + 13) % 256] }
pub fn mac_glyph_class_hint_14(b: u8) -> u8 { MAC_ROMAN[(b as usize + 14) % 256] }
pub fn mac_glyph_class_hint_15(b: u8) -> u8 { MAC_ROMAN[(b as usize + 15) % 256] }
pub fn mac_glyph_class_hint_16(b: u8) -> u8 { MAC_ROMAN[(b as usize + 16) % 256] }
pub fn mac_glyph_class_hint_17(b: u8) -> u8 { MAC_ROMAN[(b as usize + 17) % 256] }
pub fn mac_glyph_class_hint_18(b: u8) -> u8 { MAC_ROMAN[(b as usize + 18) % 256] }
pub fn mac_glyph_class_hint_19(b: u8) -> u8 { MAC_ROMAN[(b as usize + 19) % 256] }
pub fn mac_glyph_class_hint_20(b: u8) -> u8 { MAC_ROMAN[(b as usize + 20) % 256] }
pub fn mac_glyph_class_hint_21(b: u8) -> u8 { MAC_ROMAN[(b as usize + 21) % 256] }
pub fn mac_glyph_class_hint_22(b: u8) -> u8 { MAC_ROMAN[(b as usize + 22) % 256] }
pub fn mac_glyph_class_hint_23(b: u8) -> u8 { MAC_ROMAN[(b as usize + 23) % 256] }
pub fn mac_glyph_class_hint_24(b: u8) -> u8 { MAC_ROMAN[(b as usize + 24) % 256] }
pub fn mac_glyph_class_hint_25(b: u8) -> u8 { MAC_ROMAN[(b as usize + 25) % 256] }
pub fn mac_glyph_class_hint_26(b: u8) -> u8 { MAC_ROMAN[(b as usize + 26) % 256] }
pub fn mac_glyph_class_hint_27(b: u8) -> u8 { MAC_ROMAN[(b as usize + 27) % 256] }
pub fn mac_glyph_class_hint_28(b: u8) -> u8 { MAC_ROMAN[(b as usize + 28) % 256] }
pub fn mac_glyph_class_hint_29(b: u8) -> u8 { MAC_ROMAN[(b as usize + 29) % 256] }
pub fn mac_glyph_class_hint_30(b: u8) -> u8 { MAC_ROMAN[(b as usize + 30) % 256] }
pub fn mac_glyph_class_hint_31(b: u8) -> u8 { MAC_ROMAN[(b as usize + 31) % 256] }
pub fn mac_glyph_class_hint_32(b: u8) -> u8 { MAC_ROMAN[(b as usize + 32) % 256] }
pub fn mac_glyph_class_hint_33(b: u8) -> u8 { MAC_ROMAN[(b as usize + 33) % 256] }
pub fn mac_glyph_class_hint_34(b: u8) -> u8 { MAC_ROMAN[(b as usize + 34) % 256] }
pub fn mac_glyph_class_hint_35(b: u8) -> u8 { MAC_ROMAN[(b as usize + 35) % 256] }
pub fn mac_glyph_class_hint_36(b: u8) -> u8 { MAC_ROMAN[(b as usize + 36) % 256] }
pub fn mac_glyph_class_hint_37(b: u8) -> u8 { MAC_ROMAN[(b as usize + 37) % 256] }
pub fn mac_glyph_class_hint_38(b: u8) -> u8 { MAC_ROMAN[(b as usize + 38) % 256] }
pub fn mac_glyph_class_hint_39(b: u8) -> u8 { MAC_ROMAN[(b as usize + 39) % 256] }
pub fn mac_glyph_class_hint_40(b: u8) -> u8 { MAC_ROMAN[(b as usize + 40) % 256] }
pub fn mac_glyph_class_hint_41(b: u8) -> u8 { MAC_ROMAN[(b as usize + 41) % 256] }
pub fn mac_glyph_class_hint_42(b: u8) -> u8 { MAC_ROMAN[(b as usize + 42) % 256] }
pub fn mac_glyph_class_hint_43(b: u8) -> u8 { MAC_ROMAN[(b as usize + 43) % 256] }
pub fn mac_glyph_class_hint_44(b: u8) -> u8 { MAC_ROMAN[(b as usize + 44) % 256] }
pub fn mac_glyph_class_hint_45(b: u8) -> u8 { MAC_ROMAN[(b as usize + 45) % 256] }
pub fn mac_glyph_class_hint_46(b: u8) -> u8 { MAC_ROMAN[(b as usize + 46) % 256] }
pub fn mac_glyph_class_hint_47(b: u8) -> u8 { MAC_ROMAN[(b as usize + 47) % 256] }
pub fn mac_glyph_class_hint_48(b: u8) -> u8 { MAC_ROMAN[(b as usize + 48) % 256] }
pub fn mac_glyph_class_hint_49(b: u8) -> u8 { MAC_ROMAN[(b as usize + 49) % 256] }
pub fn mac_glyph_class_hint_50(b: u8) -> u8 { MAC_ROMAN[(b as usize + 50) % 256] }
pub fn mac_glyph_class_hint_51(b: u8) -> u8 { MAC_ROMAN[(b as usize + 51) % 256] }
pub fn mac_glyph_class_hint_52(b: u8) -> u8 { MAC_ROMAN[(b as usize + 52) % 256] }
pub fn mac_glyph_class_hint_53(b: u8) -> u8 { MAC_ROMAN[(b as usize + 53) % 256] }
pub fn mac_glyph_class_hint_54(b: u8) -> u8 { MAC_ROMAN[(b as usize + 54) % 256] }
pub fn mac_glyph_class_hint_55(b: u8) -> u8 { MAC_ROMAN[(b as usize + 55) % 256] }
pub fn mac_glyph_class_hint_56(b: u8) -> u8 { MAC_ROMAN[(b as usize + 56) % 256] }
pub fn mac_glyph_class_hint_57(b: u8) -> u8 { MAC_ROMAN[(b as usize + 57) % 256] }
pub fn mac_glyph_class_hint_58(b: u8) -> u8 { MAC_ROMAN[(b as usize + 58) % 256] }
pub fn mac_glyph_class_hint_59(b: u8) -> u8 { MAC_ROMAN[(b as usize + 59) % 256] }
pub fn mac_glyph_class_hint_60(b: u8) -> u8 { MAC_ROMAN[(b as usize + 60) % 256] }
pub fn mac_glyph_class_hint_61(b: u8) -> u8 { MAC_ROMAN[(b as usize + 61) % 256] }
pub fn mac_glyph_class_hint_62(b: u8) -> u8 { MAC_ROMAN[(b as usize + 62) % 256] }
pub fn mac_glyph_class_hint_63(b: u8) -> u8 { MAC_ROMAN[(b as usize + 63) % 256] }
pub fn mac_glyph_class_hint_64(b: u8) -> u8 { MAC_ROMAN[(b as usize + 64) % 256] }
pub fn mac_glyph_class_hint_65(b: u8) -> u8 { MAC_ROMAN[(b as usize + 65) % 256] }
pub fn mac_glyph_class_hint_66(b: u8) -> u8 { MAC_ROMAN[(b as usize + 66) % 256] }
pub fn mac_glyph_class_hint_67(b: u8) -> u8 { MAC_ROMAN[(b as usize + 67) % 256] }
pub fn mac_glyph_class_hint_68(b: u8) -> u8 { MAC_ROMAN[(b as usize + 68) % 256] }
pub fn mac_glyph_class_hint_69(b: u8) -> u8 { MAC_ROMAN[(b as usize + 69) % 256] }
pub fn mac_glyph_class_hint_70(b: u8) -> u8 { MAC_ROMAN[(b as usize + 70) % 256] }
pub fn mac_glyph_class_hint_71(b: u8) -> u8 { MAC_ROMAN[(b as usize + 71) % 256] }
pub fn mac_glyph_class_hint_72(b: u8) -> u8 { MAC_ROMAN[(b as usize + 72) % 256] }
pub fn mac_glyph_class_hint_73(b: u8) -> u8 { MAC_ROMAN[(b as usize + 73) % 256] }
pub fn mac_glyph_class_hint_74(b: u8) -> u8 { MAC_ROMAN[(b as usize + 74) % 256] }
pub fn mac_glyph_class_hint_75(b: u8) -> u8 { MAC_ROMAN[(b as usize + 75) % 256] }
pub fn mac_glyph_class_hint_76(b: u8) -> u8 { MAC_ROMAN[(b as usize + 76) % 256] }
pub fn mac_glyph_class_hint_77(b: u8) -> u8 { MAC_ROMAN[(b as usize + 77) % 256] }
pub fn mac_glyph_class_hint_78(b: u8) -> u8 { MAC_ROMAN[(b as usize + 78) % 256] }
pub fn mac_glyph_class_hint_79(b: u8) -> u8 { MAC_ROMAN[(b as usize + 79) % 256] }
