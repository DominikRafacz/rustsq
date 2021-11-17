use std::slice::IterMut;
use std::str::Chars;
use crate::alphabet::Alphabet;
use crate::utilities::matcher::{Matcher, SimpleMatcher};
use crate::typedefs::*;

type State = u8;

pub fn pack(proto : &ProtoSequenceData, seq : &mut SequenceData, alph : &Alphabet) {
    let matcher = alph.matcher();
    match matcher {
        Matcher::Simple(matcher) => {
            let mut remaining_letters = proto.len();
            if remaining_letters == 0 { return; }

            let mut proto_iter = proto.chars();
            let mut seq_iter = seq.iter_mut();

            while remaining_letters >= 8 {
                pack_full_octet(&matcher, &mut proto_iter, &mut seq_iter);
                remaining_letters -= 8;
            }

            pack_incomplete_octet(&matcher, &mut proto_iter, &mut seq_iter, remaining_letters);
        }
        Matcher::Compound(_) => {}
    }
}

fn pack_full_octet(matcher: &SimpleMatcher, proto_iter: &mut Chars, seq_iter: &mut IterMut<ElemPacked>) {
    // here we are sure that seq_iters has at least 3 more items and proto_iter has at least 8,
    // so we can unwrap them without worries

    let first_byte = seq_iter.next().unwrap();
    let second_byte = seq_iter.next().unwrap();
    let third_byte = seq_iter.next().unwrap();

    *first_byte = matcher[proto_iter.next().unwrap()];
    *first_byte |= matcher[proto_iter.next().unwrap()] << 3;

    let mut tmp = matcher[proto_iter.next().unwrap()];

    *first_byte |= tmp << 6;
    *second_byte = tmp >> 2;
    *second_byte |= matcher[proto_iter.next().unwrap()] << 1;
    *second_byte |= matcher[proto_iter.next().unwrap()] << 4;

    tmp = matcher[proto_iter.next().unwrap()];

    *second_byte |= tmp << 7;
    *third_byte = tmp >> 1;
    *third_byte |= matcher[proto_iter.next().unwrap()] << 2;
    *third_byte |= matcher[proto_iter.next().unwrap()] << 5;
}

fn pack_incomplete_octet(matcher: &SimpleMatcher, proto_iter: &mut Chars, seq_iter: &mut IterMut<ElemPacked>, remaining_letters: usize) {

}
