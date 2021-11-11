use crate::typedefs::*;
use crate::constants::standard_alphabets;

pub struct Alphabet {
    sqtype: SqType,
    letters : Vec<Letter>,
}

impl Alphabet {
    pub fn new(letters : Vec<Letter>) -> Alphabet {
        Alphabet {
            sqtype: SqType::Atp,
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
}
