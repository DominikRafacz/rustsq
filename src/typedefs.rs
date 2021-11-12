pub type Letter = &'static str;
pub type SimpleLetter = char;

pub type ElemPacked = u8;

pub enum SqType {
    AmiExt,
    AmiBsc,
    DNAExt,
    DNABsc,
    RNAExt,
    RNABsc,
    Unt,
    Atp,
    Enc
}