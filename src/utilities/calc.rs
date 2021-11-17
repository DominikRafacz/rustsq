use crate::alphabet::Alphabet;
use crate::typedefs::Asize;

pub(crate) fn asize(alph_len: usize) -> Asize {
    ((alph_len + 2) as f32).log2().ceil() as Asize
}

pub(crate) fn packed_len(proto_data_len: usize, alph: &Alphabet) -> usize {
    (proto_data_len as f32 / alph.asize() as f32).ceil() as usize
}