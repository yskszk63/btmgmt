use btmgmt_packet_helper::pack::{Pack, Unpack};

#[derive(Pack, Unpack, Debug, PartialEq, Eq)]
#[pack(u16)]
pub enum Enum {
    A = 0x0F,
    B = 0xF0,
}

fn main() {
    fn assert<A: Pack + Unpack>() {}
    assert::<Enum>();

    let mut b = vec![];
    let v = Enum::A;
    v.pack(&mut b).unwrap();
    assert_eq!(b, &[0x0F, 0x00]);

    let v2 = Enum::unpack(&mut &b[..]).unwrap();
    assert_eq!(v, v2);
}
