//! Shared OpenType Layout structures.


use crate::error::{GlyphError, GlyphResult};
use crate::io::FontReader;

#[derive(Clone, Debug)]
pub struct ScriptRecord { pub tag: u32, pub offset: u16 }

impl ScriptRecord {
    pub fn parse(r: &mut FontReader<'_>) -> GlyphResult<Self> {
        Ok(Self { tag: r.read_tag()?, offset: r.read_u16()? })
    }
}

#[derive(Clone, Debug)]
pub struct FeatureRecord { pub tag: u32, pub offset: u16 }

impl FeatureRecord {
    pub fn parse(r: &mut FontReader<'_>) -> GlyphResult<Self> {
        Ok(Self { tag: r.read_tag()?, offset: r.read_u16()? })
    }
}

pub fn parse_script_list(data: &[u8], offset: usize) -> GlyphResult<Vec<ScriptRecord>> {
    let mut r = FontReader::from_slice(data, offset)?;
    let count = r.read_u16()? as usize;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count { out.push(ScriptRecord::parse(&mut r)?); }
    Ok(out)
}

pub fn parse_feature_list(data: &[u8], offset: usize) -> GlyphResult<Vec<FeatureRecord>> {
    let mut r = FontReader::from_slice(data, offset)?;
    let count = r.read_u16()? as usize;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count { out.push(FeatureRecord::parse(&mut r)?); }
    Ok(out)
}

pub fn layout_table_span_0(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(4)) }

pub fn layout_table_span_1(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(5)) }

pub fn layout_table_span_2(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(6)) }

pub fn layout_table_span_3(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(7)) }

pub fn layout_table_span_4(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(8)) }

pub fn layout_table_span_5(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(9)) }

pub fn layout_table_span_6(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(10)) }

pub fn layout_table_span_7(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(11)) }

pub fn layout_table_span_8(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(12)) }

pub fn layout_table_span_9(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(13)) }

pub fn layout_table_span_10(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(14)) }

pub fn layout_table_span_11(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(15)) }

pub fn layout_table_span_12(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(16)) }

pub fn layout_table_span_13(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(17)) }

pub fn layout_table_span_14(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(18)) }

pub fn layout_table_span_15(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(19)) }

pub fn layout_table_span_16(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(20)) }

pub fn layout_table_span_17(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(21)) }

pub fn layout_table_span_18(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(22)) }

pub fn layout_table_span_19(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(23)) }

pub fn layout_table_span_20(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(24)) }

pub fn layout_table_span_21(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(25)) }

pub fn layout_table_span_22(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(26)) }

pub fn layout_table_span_23(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(27)) }

pub fn layout_table_span_24(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(28)) }

pub fn layout_table_span_25(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(29)) }

pub fn layout_table_span_26(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(30)) }

pub fn layout_table_span_27(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(31)) }

pub fn layout_table_span_28(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(32)) }

pub fn layout_table_span_29(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(33)) }

pub fn layout_table_span_30(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(34)) }

pub fn layout_table_span_31(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(35)) }

pub fn layout_table_span_32(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(36)) }

pub fn layout_table_span_33(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(37)) }

pub fn layout_table_span_34(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(38)) }

pub fn layout_table_span_35(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(39)) }

pub fn layout_table_span_36(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(40)) }

pub fn layout_table_span_37(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(41)) }

pub fn layout_table_span_38(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(42)) }

pub fn layout_table_span_39(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(43)) }

pub fn layout_table_span_40(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(44)) }

pub fn layout_table_span_41(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(45)) }

pub fn layout_table_span_42(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(46)) }

pub fn layout_table_span_43(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(47)) }

pub fn layout_table_span_44(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(48)) }

pub fn layout_table_span_45(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(49)) }

pub fn layout_table_span_46(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(50)) }

pub fn layout_table_span_47(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(51)) }

pub fn layout_table_span_48(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(52)) }

pub fn layout_table_span_49(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(53)) }

pub fn layout_table_span_50(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(54)) }

pub fn layout_table_span_51(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(55)) }

pub fn layout_table_span_52(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(56)) }

pub fn layout_table_span_53(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(57)) }

pub fn layout_table_span_54(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(58)) }

pub fn layout_table_span_55(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(59)) }

pub fn layout_table_span_56(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(60)) }

pub fn layout_table_span_57(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(61)) }

pub fn layout_table_span_58(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(62)) }

pub fn layout_table_span_59(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(63)) }

pub fn layout_table_span_60(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(64)) }

pub fn layout_table_span_61(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(65)) }

pub fn layout_table_span_62(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(66)) }

pub fn layout_table_span_63(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(67)) }

pub fn layout_table_span_64(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(68)) }

pub fn layout_table_span_65(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(69)) }

pub fn layout_table_span_66(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(70)) }

pub fn layout_table_span_67(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(71)) }

pub fn layout_table_span_68(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(72)) }

pub fn layout_table_span_69(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(73)) }

pub fn layout_table_span_70(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(74)) }

pub fn layout_table_span_71(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(75)) }

pub fn layout_table_span_72(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(76)) }

pub fn layout_table_span_73(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(77)) }

pub fn layout_table_span_74(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(78)) }

pub fn layout_table_span_75(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(79)) }

pub fn layout_table_span_76(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(80)) }

pub fn layout_table_span_77(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(81)) }

pub fn layout_table_span_78(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(82)) }

pub fn layout_table_span_79(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(83)) }

pub fn layout_table_span_80(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(84)) }

pub fn layout_table_span_81(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(85)) }

pub fn layout_table_span_82(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(86)) }

pub fn layout_table_span_83(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(87)) }

pub fn layout_table_span_84(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(88)) }

pub fn layout_table_span_85(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(89)) }

pub fn layout_table_span_86(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(90)) }

pub fn layout_table_span_87(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(91)) }

pub fn layout_table_span_88(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(92)) }

pub fn layout_table_span_89(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(93)) }

pub fn layout_table_span_90(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(94)) }

pub fn layout_table_span_91(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(95)) }

pub fn layout_table_span_92(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(96)) }

pub fn layout_table_span_93(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(97)) }

pub fn layout_table_span_94(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(98)) }

pub fn layout_table_span_95(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(99)) }

pub fn layout_table_span_96(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(100)) }

pub fn layout_table_span_97(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(101)) }

pub fn layout_table_span_98(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(102)) }

pub fn layout_table_span_99(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(103)) }

pub fn layout_table_span_100(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(104)) }

pub fn layout_table_span_101(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(105)) }

pub fn layout_table_span_102(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(106)) }

pub fn layout_table_span_103(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(107)) }

pub fn layout_table_span_104(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(108)) }

pub fn layout_table_span_105(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(109)) }

pub fn layout_table_span_106(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(110)) }

pub fn layout_table_span_107(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(111)) }

pub fn layout_table_span_108(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(112)) }

pub fn layout_table_span_109(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(113)) }

pub fn layout_table_span_110(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(114)) }

pub fn layout_table_span_111(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(115)) }

pub fn layout_table_span_112(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(116)) }

pub fn layout_table_span_113(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(117)) }

pub fn layout_table_span_114(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(118)) }

pub fn layout_table_span_115(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(119)) }

pub fn layout_table_span_116(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(120)) }

pub fn layout_table_span_117(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(121)) }

pub fn layout_table_span_118(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(122)) }

pub fn layout_table_span_119(data: &[u8]) -> GlyphResult<usize> { Ok(data.len().min(123)) }
