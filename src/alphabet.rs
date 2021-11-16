use crate::typedefs::*;
use crate::constants::standard_alphabets;
use crate::utilities::{matcher, calc};

pub struct Alphabet {
    sqtype: SqType,
    letters: Vec<Letter>,
    asize: Asize
}

impl Alphabet {
    pub fn new(letters: Vec<Letter>) -> Alphabet {
        let asize = calc::asize(letters.len());
        Alphabet {
            sqtype: SqType::Atp,
            letters,
            asize
        }
    }

    pub fn from_sqtype(sqtype: SqType) -> Option<Alphabet> {
        match standard_alphabets::match_letters(&sqtype) {
            Some(letters) => Some(
                Alphabet {
                    sqtype,
                    letters: Vec::from(letters),
                    asize: calc::asize(letters.len())
                }
            ),
            None => None
        }
    }

    pub fn asize(&self) -> Asize {
        self.asize
    }

    pub fn matcher(&self) -> matcher::Matcher {
        matcher::Matcher::new(&self.letters, self.is_simple())
    }

    fn is_simple(&self) -> bool {
        !self.letters.iter().any(|letter| letter.len() > 1)
    }
}
