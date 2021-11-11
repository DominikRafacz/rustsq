use crate::typedefs::*;
use crate::constants::standard_alphabets;
use crate::utilities::matcher;

pub struct Alphabet {
    sqtype: SqType,
    letters : Vec<Letter>
}

impl Alphabet {
    pub fn new(letters : Vec<Letter>) -> Alphabet {
        Alphabet {
            sqtype : SqType::Atp,
            letters
        }
    }

    pub fn from_sqtype(sqtype : SqType) -> Option<Alphabet> {
        match standard_alphabets::match_letters(&sqtype) {
            Some(letters) => Some(
                Alphabet {
                    sqtype,
                    letters : Vec::from(letters)
                }
            ),
            None => None
        }
    }

    pub fn matcher(&self) -> matcher::Matcher {
        matcher::Matcher::new(&self.letters, self.is_simple())
    }

    fn is_simple(&self) -> bool {
        !self.letters.iter().any(|letter| letter.len() > 1)
    }
}
