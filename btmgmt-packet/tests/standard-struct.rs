use btmgmt_packet::pack::{Pack, Unpack};

#[derive(Pack, Unpack, Debug, PartialEq, Eq)]
pub struct Standard {
    f1: u16,
    f2: bool,
}

fn main() {
    fn assert<A: Pack + Unpack>() {}
    assert::<Standard>();

    let mut b = vec![];
    let v = Standard { f1: 0x0123, f2: true };
    v.pack(&mut b).unwrap();
    assert_eq!(b, &[0x23, 0x01, 0x01]);

    let v2 = Standard::unpack(&mut &b[..]).unwrap();
    assert_eq!(v, v2);
}
