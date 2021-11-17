pub type Asize = u8;

pub type Letter = &'static str;
pub type SimpleLetter = char;

pub(crate) type ProtoSequenceData = String;
pub(crate) type SequenceData = Vec<ElemPacked>;

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