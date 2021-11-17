use crate::typedefs::SequenceData;

pub struct Sequence {
    data : SequenceData
}

impl From<SequenceData> for Sequence {
    fn from(data: SequenceData) -> Self {
        Sequence {
            data
        }
    }
}