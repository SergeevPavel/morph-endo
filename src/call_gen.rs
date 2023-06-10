

// Pat: [Open, Search("IFPICFPPCCC"), Open, Search("IFPICFPPCCC"), Close, Close]
// Tmp: [Ref(num=0, prot_lvl=0), I, C, I, I, C, I, C, C, I, I, C, P, I, I, C, I, C, C, C, C, C, P, Ref(num=1, prot_lvl=0)]

use crate::interpreter::{template::TItem, pattern::PItem, dna::{Base, Dna}, template, pattern};
use crate::interpreter::literals::asnat;
use crate::interpreter::pattern::pattern;

fn search(s: &str) -> PItem {
    let s = s.chars().map(|c| Base::from_char(c).unwrap()).collect();
    PItem::Search { s }
}

fn call_gen_prefix(offset: i32, len: i32) -> Vec<Base> {
    use PItem::*;
    use TItem::*;
    use Base::*;
    let p = vec![Open,
                 search("IFPICFPPCCC"),
                 Open,
                 search("IFPICFPPCCC"),
                 Close,
                 Close];

    let mut t = Vec::new();
    t.push(Ref { n: 0, l: 0 });
    t.extend(asnat(offset as usize).iter().map(|b| TBase(*b)));
    t.extend(asnat(len as usize).iter().map(|b| TBase(*b)));
    t.push(Ref { n: 1, l: 0 });


    let mut result = Vec::new();
    result.extend_from_slice(&pattern::encode(&p));
    result.extend_from_slice(&template::encode(&t));
    return result;
}

crate::entry_point!("call_gen_prefix", call_gen_prefix_main);
fn call_gen_prefix_main() {
    // let offset = 0x000510; let len = 0x000018;
    let offset = 0x3c870e; let len = 0x00372b;
    let pref: String = call_gen_prefix(offset, len).iter().map(|b| b.to_char()).collect();
    println!("{:?}", pref);
}