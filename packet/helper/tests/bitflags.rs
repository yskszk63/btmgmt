use btmgmt_packet_helper::pack::{Pack, Unpack};

bitflags::bitflags! {
    #[derive(Pack, Unpack)]
    pub struct Bitflags: u16 {
        const A = 0;
        const B = 1;
    }
}

fn main() {
    fn assert<A: Pack + Unpack>() {}
    assert::<Bitflags>();

    let mut b = vec![];
    let v = Bitflags::A | Bitflags::B;
    v.pack(&mut b).unwrap();
    assert_eq!(b, &[0x01, 0x00]);
    assert_eq!(&v.bits().to_le_bytes()[..], &[0x01, 0x00]);

    let v2 = Bitflags::unpack(&mut &b[..]).unwrap();
    assert_eq!(v, v2);
}
