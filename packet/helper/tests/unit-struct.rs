use btmgmt_packet_helper::pack::{Pack, Unpack};

#[derive(Pack, Unpack, Debug, PartialEq, Eq)]
pub struct Unit;

fn main() {
    fn assert<A: Pack + Unpack>() {}
    assert::<Unit>();

    let mut b = vec![];
    let v = Unit;
    v.pack(&mut b).unwrap();
    assert!(b.is_empty());

    let v2 = Unit::unpack(&mut &b[..]).unwrap();
    assert_eq!(v, v2);
}
