use btmgmt_packet_helper::commands;
use btmgmt_packet_helper::pack::{Pack, Unpack};

#[commands(name = Commands, trait = Command, codes = CommandCode)]
mod commands {
    use super::*;

    #[derive(Debug, Pack)]
    #[command(code = 0x0001, reply = MyCommandReply)]
    pub struct  MyCommand {
        pub f1: u16,
    }

    #[derive(Debug, Unpack, PartialEq, Eq)]
    pub struct MyCommandReply;
}

fn main() {
    use commands::*;

    fn assert_command<C>() where C: Command {}
    assert_command::<MyCommand>();

    assert_eq!(CommandCode::MyCommand, <MyCommand as Command>::CODE);

    let b = vec![];
    let r = <MyCommand as Command>::Reply::unpack(&mut &b[..]).unwrap();

    assert_eq!(MyCommandReply, r);
    assert!(matches!(Commands::from(MyCommand { f1: 0 }), Commands::MyCommand(MyCommand { f1: 0 })));

    assert_eq!(Commands::from(MyCommand { f1: 0 }).code(), CommandCode::MyCommand);

    let mut b = vec![];
    Commands::from(MyCommand { f1: 0 }).pack_inner(&mut b).unwrap();
    assert_eq!(b, &[0x00, 0x00]);
}
