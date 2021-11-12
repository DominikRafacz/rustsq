pub struct ProtoSequence {
    data : String
}

impl From<String> for ProtoSequence {
    fn from(data: String) -> Self {
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