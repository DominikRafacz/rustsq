mod typedefs;
mod alphabet;
mod constants;

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
}
