mod typedefs;
mod alphabet;
mod constants;
mod utilities;

#[cfg(test)]
mod tests {
    use crate::alphabet::Alphabet;
    use crate::typedefs::SqType::AmiBsc;

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
}
