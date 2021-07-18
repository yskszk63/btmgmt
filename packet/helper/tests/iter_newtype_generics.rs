use std::array::IntoIter;

use btmgmt_packet_helper::helper::IterNewtype;

#[derive(IterNewtype)]
struct MyNewType<T>(Vec<T>);

fn main() {
    let mut v = IntoIter::new([0, 1, 2]).collect::<MyNewType<u8>>();
    for a in v.iter() {
        assert!(*a == 0 || *a == 1 || *a == 2);
    }
    for a in v.iter_mut() {
        *a += 1;
    }
    for a in v {
        assert!(a == 1 || a == 2 || a == 3);
    }
}
