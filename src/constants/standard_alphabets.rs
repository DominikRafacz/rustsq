use crate::typedefs::{Letter, SqType};
use crate::typedefs::SqType::*;

const AMI_BSC_LETTERS : &'static [Letter] = &["A", "C", "D", "E", "F", "G", "H", "I", "K", "L", "M",
    "N", "P", "Q", "R", "S", "T", "V", "W", "Y", "-", "*"];
const AMI_EXT_LETTERS : &'static [Letter] = &["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K",
    "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "-", "*"];
const DNA_BSC_LETTERS : &'static [Letter] = &["A", "C", "G", "T", "-"];
const DNA_EXT_LETTERS : &'static [Letter] = &["A", "C", "G", "T", "W", "S", "M", "K", "R", "Y", "B",
    "D", "H", "V", "N", "-"];
const RNA_BSC_LETTERS : &'static [Letter] = &["A", "C", "G", "U", "-"];
const RNA_EXT_LETTERS : &'static [Letter] = &["A", "C", "G", "U", "W", "S", "M", "K", "R", "Y", "B",
    "D", "H", "V", "N", "-"];

pub(crate) fn match_letters(sqtype: &SqType) -> Option<&'static [Letter]> {
    match sqtype {
        AmiBsc => Some(AMI_BSC_LETTERS),
        AmiExt => Some(AMI_EXT_LETTERS),
        DNABsc => Some(DNA_BSC_LETTERS),
        DNAExt => Some(DNA_EXT_LETTERS),
        RNABsc => Some(RNA_BSC_LETTERS),
        RNAExt => Some(RNA_EXT_LETTERS),
        _ => None
    }
}
