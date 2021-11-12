mod typedefs;
mod alphabet;
mod constants;
mod utilities;
mod proto_sequence;
mod sequence;

#[cfg(test)]
mod tests {
    use crate::alphabet::Alphabet;
    use crate::proto_sequence::ProtoSequence;
    use crate::typedefs::SqType;
    use crate::typedefs::SqType::AmiBsc;
    use crate::utilities::matcher;

    #[test]
    fn create_alphabet_from_vector() {
        Alphabet::new(vec!["A", "C", "T", "G", "-"]);
    }

    #[test]
    fn create_alphabet_from_type() {
        Alphabet::from_sqtype(AmiBsc).unwrap();
    }

    #[test]
    fn get_matcher() {
        Alphabet::new(vec!["A"]).matcher();
    }

    #[test]
    fn construct_proto_sequences() {
        ProtoSequence::from("CATGATCGATACAGTG");
    }
}
