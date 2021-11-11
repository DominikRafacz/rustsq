use std::collections::HashMap;
use std::ops::Index;
use crate::typedefs::*;

pub struct SimpleAlphabet {
    letters : Vec<SimpleLetter>,
    reverse_map : HashMap<SimpleLetter, u8>
}

impl SimpleAlphabet {
    pub fn new(letters : Vec<SimpleLetter>) -> SimpleAlphabet {
        let mut reverse_map : HashMap<SimpleLetter, u8> = HashMap::new();
        let mut index : u8 = 0;

        for letter in &letters {
            reverse_map.insert(*letter, index);
            index += 1;
        }

        SimpleAlphabet {
            letters,
            reverse_map
        }
    }
}

impl Index<u8> for SimpleAlphabet {
    type Output = SimpleLetter;

    fn index(&self, index: u8) -> &Self::Output {
        &self.letters[index as usize]
    }
}

impl Index<SimpleLetter> for SimpleAlphabet {
    type Output = u8;

    fn index(&self, index: SimpleLetter) -> &Self::Output {
        self.reverse_map.get(&index).unwrap()
    }
}