use btmgmt_packet_helper::events;
use btmgmt_packet_helper::pack::Unpack;

/// comments.
#[events(name = Events, codes = EventCode)]
mod events {
    use super::*;

    #[derive(Debug, Clone, Unpack)]
    #[event(0x0001)]
    pub struct  MyEvent {
        pub f1: u16,
    }
}

fn main() {
    use events::*;

    assert_eq!(EventCode::MyEvent, MyEvent::CODE);

    assert!(matches!(Events::from(MyEvent { f1: 0 }), Events::MyEvent(MyEvent { f1: 0 })));
}
