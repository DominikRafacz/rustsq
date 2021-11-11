mod typedefs;
mod alphabet;

#[cfg(test)]
mod tests {
    use crate::alphabet::SimpleAlphabet;

    #[test]
    fn create_alphabet_from_vector() {
        SimpleAlphabet::new(vec!['A', 'C', 'T', 'G', '-']);
    }

    #[test]
    fn index_alphabet_with_number() {
        let alph = SimpleAlphabet::new(vec!['A', 'C', 'T', 'G', '-']);

        assert_eq!(alph[0], 'A');
        assert_eq!(alph[4], '-');
    }

    #[test]
    fn index_alphabet_with_letter() {
        let alph = SimpleAlphabet::new(vec!['A', 'C', 'T', 'G', '-']);

        assert_eq!(alph['A'], 0);
        assert_eq!(alph['C'], 1);
    }
}
