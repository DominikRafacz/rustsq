use std::ops::Index;
use crate::typedefs::Letter;

pub enum Matcher {
    Simple(SimpleMatcher),
    Compound(CompoundMatcher)
}

pub struct SimpleMatcher {

}

pub struct CompoundMatcher {

}

impl Index<Letter> for SimpleMatcher {
    type Output = u8;

    fn index(&self, index: Letter) -> &Self::Output {
        todo!()
    }
}

impl Index<u8> for CompoundMatcher {
    type Output = Letter;

    fn index(&self, index: u8) -> &Self::Output {
        todo!()
    }
}