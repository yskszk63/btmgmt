use btmgmt_packet_helper::pack::{Pack, Unpack};

#[derive(Pack, Unpack, Debug, PartialEq, Eq)]
pub struct Tuple(u16, bool);

fn main() {
    fn assert<A: Pack + Unpack>() {}
    assert::<Tuple>();

    let mut b = vec![];
    let v = Tuple(0x0123, true);
    v.pack(&mut b).unwrap();
    assert_eq!(b, &[0x23, 0x01, 0x01]);

    let v2 = Tuple::unpack(&mut &b[..]).unwrap();
    assert_eq!(v, v2);
}
