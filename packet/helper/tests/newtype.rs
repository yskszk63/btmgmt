use btmgmt_packet_helper::helper::Newtype;

#[derive(Newtype)]
struct MyNewType(u8);

fn main() {
    let mut v = MyNewType::from(1);
    assert_eq!(1, *v);
    *v = 2;
    assert_eq!(2, *v);
}
