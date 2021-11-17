use crate::alphabet::Alphabet;
use crate::pack;
use crate::sequence::Sequence;
use crate::typedefs::{ProtoSequenceData, SequenceData};
use crate::utilities::calc;

pub struct ProtoSequence {
    data : ProtoSequenceData
}

impl ProtoSequence {
    pub(crate) fn pack(&self, alph : &Alphabet) -> Sequence {
        let len = calc::packed_len(self.data.len(), &alph);
        let mut seq: SequenceData = vec![0; len];
        pack::pack(&self.data, &mut seq, &alph);
        Sequence::from(seq)
    }
}

impl From<ProtoSequenceData> for ProtoSequence {
    fn from(data: ProtoSequenceData) -> Self {
        ProtoSequence {
            data
        }
    }
}

impl From<&str> for ProtoSequence {
    fn from(data: &str) -> Self {
        ProtoSequence {
            data : data.into()
        }
    }
}