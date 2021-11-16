use crate::typedefs::Asize;

pub(crate) fn asize(alph_len: usize) -> Asize {
    ((alph_len + 2) as f32).log2().ceil() as Asize
}