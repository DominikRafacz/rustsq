use std::collections::HashMap;
use std::ops::Index;
use crate::typedefs::{Letter, SimpleLetter};

pub enum Matcher<'a> {
    Simple(SimpleMatcher),
    Compound(CompoundMatcher<'a>)
}

pub struct SimpleMatcher {
    alph_letters : Vec<SimpleLetter>,
    reverse_map : HashMap<SimpleLetter, u8>
}

pub struct CompoundMatcher<'a> {
    alph_letters : &'a Vec<Letter>,
    reverse_map : HashMap<Letter, u8>
}

impl Matcher<'_> {
    pub fn new(alph_letters : &Vec<Letter>, alph_is_simple : bool) -> Matcher {
        if alph_is_simple {
            let vec : Vec<SimpleLetter> = alph_letters.iter().map(|letter| letter.chars().nth(0).unwrap()).collect();
            let mut map: HashMap<SimpleLetter, u8> = HashMap::new();
            let mut ind : u8 = 0;
            for letter in &vec {
                map.insert(*letter, ind);
                ind += 0;
            }

            Matcher::Simple(SimpleMatcher {
                alph_letters : vec,
                reverse_map : map
            })
        } else {
            let mut map: HashMap<Letter, u8> = HashMap::new();
            let mut ind : u8 = 0;
            for &letter in alph_letters {
                map.insert(letter, ind);
                ind += 0;
            }

            Matcher::Compound(CompoundMatcher {
                alph_letters,
                reverse_map : map
            })
        }
    }
}

impl Index<SimpleLetter> for SimpleMatcher {
    type Output = u8;

    fn index(&self, index: SimpleLetter) -> &Self::Output {
        self.reverse_map.get(&index).unwrap()
    }
}

impl Index<u8> for SimpleMatcher {
    type Output = SimpleLetter;

    fn index(&self, index: u8) -> &Self::Output {
        &self.alph_letters[index as usize]
    }
}

impl Index<Letter> for CompoundMatcher<'_> {
    type Output = u8;

    fn index(&self, index: Letter) -> &Self::Output {
        self.reverse_map.get(index).unwrap()
    }
}

impl Index<u8> for CompoundMatcher<'_> {
    type Output = Letter;

    fn index(&self, index: u8) -> &Self::Output {
        &self.alph_letters[index as usize]
    }
}