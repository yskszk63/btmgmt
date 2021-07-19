use std::array::IntoIter;
use std::collections::HashSet;

use btmgmt_packet_helper::helper::IterNewtype;

#[derive(IterNewtype)]
#[iter_newtype(item = u8, into_iter = ::std::collections::hash_set::IntoIter<u8>, no_iter_mut)]
struct MyNewType(HashSet<u8>);

fn main() {
    let v = IntoIter::new([0, 1, 2]).collect::<MyNewType>();
    for a in v.iter() {
        assert!(*a == 0 || *a == 1 || *a == 2);
    }
    for a in v {
        assert!(a == 0 || a == 1 || a == 2);
    }
}
