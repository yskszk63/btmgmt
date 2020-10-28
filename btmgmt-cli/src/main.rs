use std::collections::HashSet;
use std::str::FromStr;

use btmgmt::client::Client;
use btmgmt::command;
use btmgmt::event::Event;
use clap::Clap;
use tokio::stream::StreamExt;

fn length(len: usize) -> impl FnMut(&str) -> Result<(), anyhow::Error> {
    move |s| {
        if s.len() != len {
            anyhow::bail!("invalid length {} != {}", len, s.len())
        }
        Ok(())
    }
}

#[derive(Debug, Clap)]
struct Opt {
    #[clap(short, long, default_value = "0")]
    index: u16,

    #[clap(short, long)]
    listen: bool,

    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Clap)]
enum Command {
    Version,

    SupportedCommands,

    #[clap(aliases=&["c"])]
    Controller {
        #[clap(subcommand)]
        command: Option<ControllerCommand>,
    },

    #[clap(aliases=&["k"])]
    Key {
        #[clap(subcommand)]
        command: KeyCommand,
    },

    #[clap(aliases=&["conn"])]
    Connection {
        #[clap(subcommand)]
        command: Option<ConnectionCommand>,
    },

    #[clap(aliases=&["discov"])]
    Discovery {
        #[clap(subcommand)]
        command: DiscoveryCommand,
    },

    #[clap(aliases=&["conf"])]
    Configuration {
        #[clap(subcommand)]
        command: ConfigurationCommand,
    },

    #[clap(aliases=&["adv"])]
    Advertise {
        #[clap(subcommand)]
        command: AdvertiseCommand,
    },

    #[clap(aliases=&["dev"])]
    Device {
        #[clap(subcommand)]
        command: DeviceCommand,
    },

    Oob {
        #[clap(subcommand)]
        command: OobCommand,
    },
}

// TODO pin code reply
// TODO pair device / confirm / passkey
// TODO oob

#[derive(Debug, Clap)]
enum ControllerCommand {
    Show,

    Ls {
        #[clap(long, short)]
        extended: bool,
    },

    Power {
        flag: OnOff,
    },

    Discoverable {
        flag: Discoerable,

        #[clap(short, long, default_value = "0")]
        timeout: u16,
    },

    Connectable {
        flag: OnOff,
    },

    FastConnectable {
        flag: OnOff,
    },

    Bondable {
        flag: OnOff,
    },

    LinkSecurity {
        flag: OnOff,
    },

    Ssp {
        flag: OnOff,
    },

    Hs {
        flag: OnOff,
    },

    Le {
        flag: OnOff,
    },

    Bredr {
        flag: OnOff,
    },

    Cod {
        major: u8,
        minor: u8,
    },

    Name {
        name: btmgmt::Name,
        short_name: Option<btmgmt::ShortName>,
    },

    Uuid {
        #[clap(subcommand)]
        command: UuidCommand,
    },

    Advertising {
        flag: OnOff,
        #[clap(short, long)]
        connectable: bool,
    },
}

impl Default for ControllerCommand {
    fn default() -> Self {
        Self::Show
    }
}

impl ControllerCommand {
    async fn proc(&self, client: &Client, index: u16) -> anyhow::Result<()> {
        match self {
            Self::Show => {
                let reply = client
                    .call(index, command::ReadControllerInformation::new())
                    .await?;
                println!("address: {}", reply.address());
                println!("bluetooth version: {}", reply.bluetooth_version());
                println!("manufacture: {}", reply.manufacturer());
                println!("supported settings: {:?}", reply.supported_settings());
                println!("current settings: {:?}", reply.current_settings());
                println!("class of device: {}", reply.class_of_device());
                println!("name: {}", reply.name().to_string_lossy());
                println!("short name: {}", reply.short_name().to_string_lossy());
            }

            Self::Ls { extended } => {
                if !extended {
                    let reply = client
                        .call(None, command::ReadControllerIndexList::new())
                        .await?;
                    for c in reply {
                        println!("{}", u16::from(c));
                    }
                } else {
                    let reply = client
                        .call(None, command::ReadExtendedControllerIndexList::new())
                        .await?;
                    for (index, typ, bus) in reply {
                        println!("{} {:?} {:?}", u16::from(index), typ, bus);
                    }
                }
            }

            Self::Power { flag } => {
                let powered = matches!(flag, OnOff::On);
                let reply = client
                    .call(index, command::SetPowered::new(powered))
                    .await?;
                println!("{:?}", reply.current_settings());
            }

            Self::Discoverable { flag, timeout } => {
                let flag = match flag {
                    Discoerable::On => btmgmt::Discoverable::General,
                    Discoerable::Off => btmgmt::Discoverable::Disable,
                    Discoerable::Limited => btmgmt::Discoverable::Limited,
                };
                let reply = client
                    .call(index, command::SetDiscoverable::new(flag, *timeout))
                    .await?;
                println!("OK {:?}", reply);
            }

            Self::Connectable { flag } => {
                let flag = matches!(flag, OnOff::On);
                let reply = client
                    .call(index, command::SetConnectable::new(flag))
                    .await?;
                println!("OK {:?}", reply);
            }

            Self::FastConnectable { flag } => {
                let flag = matches!(flag, OnOff::On);
                let reply = client
                    .call(index, command::SetFastConnectable::new(flag))
                    .await?;
                println!("OK {:?}", reply);
            }

            Self::Bondable { flag } => {
                let flag = matches!(flag, OnOff::On);
                let reply = client.call(index, command::SetBondable::new(flag)).await?;
                println!("OK {:?}", reply);
            }

            Self::LinkSecurity { flag } => {
                let flag = matches!(flag, OnOff::On);
                let reply = client
                    .call(index, command::SetLinkSecurity::new(flag))
                    .await?;
                println!("OK {:?}", reply);
            }

            Self::Ssp { flag } => {
                let flag = matches!(flag, OnOff::On);
                let reply = client
                    .call(index, command::SetSecureSimplePairing::new(flag))
                    .await?;
                println!("OK {:?}", reply);
            }

            Self::Hs { flag } => {
                let flag = matches!(flag, OnOff::On);
                let reply = client.call(index, command::SetHighSpeed::new(flag)).await?;
                println!("OK {:?}", reply);
            }

            Self::Le { flag } => {
                let flag = matches!(flag, OnOff::On);
                let reply = client.call(index, command::SetLowEnergy::new(flag)).await?;
                println!("OK {:?}", reply);
            }

            Self::Bredr { flag } => {
                let flag = matches!(flag, OnOff::On);
                let reply = client.call(index, command::SetBrEdr::new(flag)).await?;
                println!("OK {:?}", reply);
            }

            Self::Cod { major, minor } => {
                let reply = client
                    .call(index, command::SetDeviceClass::new(*major, *minor))
                    .await?;
                println!("{}", reply.class_of_device());
            }

            Self::Name { name, short_name } => {
                let reply = client
                    .call(
                        index,
                        command::SetLocalName::new(
                            name.clone(),
                            short_name
                                .clone()
                                .unwrap_or_else(|| btmgmt::ShortName::new("").unwrap()),
                        ),
                    )
                    .await?;
                println!("{}", reply.name().to_string_lossy());
                println!("{}", reply.short_name().to_string_lossy());
            }

            Self::Uuid { command } => match command {
                UuidCommand::Add { val, svc_hint } => {
                    let reply = client
                        .call(index, command::AddUuid::new(*val, *svc_hint))
                        .await?;
                    println!("{}", reply.class_of_device());
                }

                UuidCommand::Remove { val } => {
                    let reply = client.call(index, command::RemoveUuid::new(*val)).await?;
                    println!("{}", reply.class_of_device());
                }
            },

            Self::Advertising { flag, connectable } => {
                let flag = match flag {
                    OnOff::On if *connectable => btmgmt::Advertising::Connectable,
                    OnOff::On => btmgmt::Advertising::Enable,
                    OnOff::Off => btmgmt::Advertising::Disable,
                };
                let reply = client
                    .call(index, command::SetAdvertising::new(flag))
                    .await?;
                println!("{:?}", reply.current_settings());
            }
        };
        Ok(())
    }
}

#[derive(Debug, Clap)]
enum UuidCommand {
    Add { val: btmgmt::Uuid, svc_hint: u8 },

    Remove { val: btmgmt::Uuid },
}

#[derive(Debug, Clap)]
enum KeyCommand {
    Link, // TODO
    Ltk,
}

#[derive(Debug, Clap)]
enum ConnectionCommand {
    Ls,

    Disconnect {
        address: btmgmt::Address,
        address_type: AddressType,
    },
}

impl Default for ConnectionCommand {
    fn default() -> Self {
        Self::Ls
    }
}

impl ConnectionCommand {
    async fn proc(&self, client: &Client, index: u16) -> anyhow::Result<()> {
        match self {
            ConnectionCommand::Ls => {
                let reply = client.call(index, command::GetConnections::new()).await?;
                for (addr, addr_type) in &reply {
                    println!("{} {:?}", addr, addr_type);
                }
            }

            ConnectionCommand::Disconnect {
                address,
                address_type,
            } => {
                let reply = client
                    .call(
                        index,
                        command::Disconnect::new(address.clone(), address_type.0.clone()),
                    )
                    .await?;
                println!("{}", reply.address());
                println!("{:?}", reply.address_type());
            }
        };
        Ok(())
    }
}

#[derive(Debug, Clap)]
enum DiscoveryCommand {
    Start {
        #[clap(long, short)]
        bredr: bool,
        #[clap(long, short)]
        le: bool,
        #[clap(long, short = 'L')]
        limited: bool,
        #[clap(long, short, conflicts_with = "limited")]
        rssi: Option<u8>,
        #[clap(long, short, conflicts_with = "limited")]
        uuid: Vec<btmgmt::Uuid>,
        #[clap(long, short)]
        wait: bool,
    },

    Stop {
        #[clap(long, short)]
        bredr: bool,
        #[clap(long, short)]
        le: bool,
    },
}

impl DiscoveryCommand {
    async fn proc(&self, client: &Client, index: u16) -> anyhow::Result<()> {
        match self {
            DiscoveryCommand::Start {
                bredr,
                le,
                limited,
                rssi,
                uuid,
                wait,
            } => {
                let mut addr_type = HashSet::new();
                if *bredr || (!bredr && !le) {
                    addr_type.insert(btmgmt::AddressType::BrEdr);
                }
                if *le || (!bredr && !le) {
                    addr_type.insert(btmgmt::AddressType::LePublic);
                    addr_type.insert(btmgmt::AddressType::LeRandom);
                }

                if !limited {
                    let reply = client
                        .call(index, command::StartDiscovery::new(addr_type))
                        .await?;
                    println!("{:?}", reply.address_type());
                } else if rssi.is_some() || !uuid.is_empty() {
                    let rssi = rssi.unwrap_or(127);
                    let mut uuid = uuid.clone();
                    if uuid.is_empty() {
                        uuid.push(btmgmt::Uuid::default());
                    }
                    let reply = client
                        .call(
                            index,
                            command::StartServiceDiscovery::new(addr_type, rssi, uuid),
                        )
                        .await?;
                    println!("{:?}", reply.address_type());
                } else {
                    let reply = client
                        .call(index, command::StartLimitedDiscovery::new(addr_type))
                        .await?;
                    println!("{:?}", reply.address_type());
                }

                if *wait {
                    let mut events = client.events().await;
                    while let Some((_, event)) = events.next().await {
                        match event {
                            Event::Discovering(discov) if !*discov.discovering() => break, // FIXME check index addrt
                            _ => {}
                        }
                    }
                }
            }

            DiscoveryCommand::Stop { bredr, le } => {
                let mut addr_type = HashSet::new();
                if *bredr || (!bredr && !le) {
                    addr_type.insert(btmgmt::AddressType::BrEdr);
                }
                if *le || (!bredr && !le) {
                    addr_type.insert(btmgmt::AddressType::LePublic);
                    addr_type.insert(btmgmt::AddressType::LeRandom);
                }
                let reply = client
                    .call(index, command::StopDiscovery::new(addr_type))
                    .await?;
                println!("{:?}", reply.address_type());
            }
        };
        Ok(())
    }
}

#[derive(Debug, Clap)]
enum ConfigurationCommand {
    #[clap(aliases=&["sys"])]
    System {
        #[clap(subcommand)]
        command: SystemConfigurationCommand,
    },
    #[clap(aliases=&["rt"])]
    Runtime {
        #[clap(subcommand)]
        command: RuntimeConfigurationCommand,
    },
}

impl ConfigurationCommand {
    async fn proc(&self, client: &Client, index: u16) -> anyhow::Result<()> {
        match self {
            Self::System { command } => command.proc(client, index).await?,
            Self::Runtime { command } => command.proc(client, index).await?,
        }
        Ok(())
    }
}

#[derive(Debug, Clap)]
enum SystemConfigurationCommand {
    Get,

    Set {
        #[clap(long)]
        adv_min_interval: Option<u16>,
        #[clap(long)]
        adv_max_interval: Option<u16>,
    },
}

impl SystemConfigurationCommand {
    async fn proc(&self, client: &Client, index: u16) -> anyhow::Result<()> {
        match self {
            Self::Get => {
                let reply = client
                    .call(index, command::ReadDefaultSystemConfiguration::new())
                    .await?;
                for item in reply {
                    println!("{:?} {}", item.for_type(), item.value_as_u16().unwrap());
                }
            }

            Self::Set {
                adv_min_interval,
                adv_max_interval,
            } => {
                use btmgmt::SystemConfigurationParameter::*;
                let mut req = vec![];
                if let Some(val) = adv_min_interval {
                    req.push(LEAdvertisementMinInterval(*val));
                }
                if let Some(val) = adv_max_interval {
                    req.push(LEAdvertisementMaxInterval(*val));
                }
                let reply = client
                    .call(
                        index,
                        req.into_iter()
                            .collect::<command::SetDefaultSystemConfiguration>(),
                    )
                    .await?;
                println!("OK {:?}", reply);
            }
        };
        Ok(())
    }
}

#[derive(Debug, Clap)]
enum RuntimeConfigurationCommand {
    Get,

    Set {},
}

impl RuntimeConfigurationCommand {
    async fn proc(&self, client: &Client, index: u16) -> anyhow::Result<()> {
        match self {
            Self::Get => {
                let reply = client
                    .call(index, command::ReadDefaultRuntimeConfiguration::new())
                    .await?;
                for item in reply {
                    println!("{:?} {}", item.for_type(), item.value_as_u16().unwrap());
                }
            }

            Self::Set {} => todo!(),
        };
        Ok(())
    }
}

#[derive(Debug, Clap)]
enum AdvertiseCommand {
    Features,

    Add {
        #[clap(long, short, default_value = "1")]
        instance: u8,
        #[clap(long, short, default_value = "0")]
        duration: u16,
        #[clap(long, short, default_value = "0")]
        timeout: u16,
        #[clap(long, short)]
        adv_data: Option<HexBinary>,
        #[clap(long, short)]
        scan_resp: Option<HexBinary>,
        // TODO flags
    },

    Remove {
        #[clap(long, short, default_value = "1")]
        instance: u8,
    },

    Monitor {
        #[clap(subcommand)]
        command: AdvertiseMonitorCommand,
    },
}

impl AdvertiseCommand {
    async fn proc(&self, client: &Client, index: u16) -> anyhow::Result<()> {
        match self {
            Self::Features => {
                let reply = client
                    .call(index, command::ReadAdvertisingFeature::new())
                    .await?;
                println!("supported flags: {:?}", reply.supported_flags());
                println!("max adv data len: {}", reply.max_adv_data_len());
                println!("max scan resp len: {}", reply.max_scan_resp_len());
                println!("max instances: {}", reply.max_instances());
                println!("instances:");
                for n in reply.instances() {
                    println!("{:?}", n);
                }
            }

            Self::Add {
                instance,
                duration,
                timeout,
                adv_data,
                scan_resp,
            } => {
                let adv_data = adv_data.clone().unwrap_or_default();
                let scan_resp = scan_resp.clone().unwrap_or_default();
                let flags = btmgmt::AdvertisingFlag::empty();
                let reply = client
                    .call(
                        index,
                        command::AddAdvertising::new(
                            (*instance).into(),
                            flags,
                            *duration,
                            *timeout,
                            adv_data.0,
                            scan_resp.0,
                        ),
                    )
                    .await?;
                println!("OK {:?}", reply);
            }

            Self::Remove { instance } => {
                let reply = client
                    .call(index, command::RemoveAdvertising::new((*instance).into()))
                    .await?;
                println!("OK {:?}", reply);
            }

            Self::Monitor { command } => command.proc(client, index).await?,
        }
        Ok(())
    }
}

#[derive(Clap, Debug)]
enum AdvertiseMonitorCommand {
    Add {
        #[clap(short, long)]
        addr_type: Vec<u8>,
        #[clap(short, long)]
        offset: Vec<u8>,
        #[clap(short, long)]
        value: Vec<HexBinary>,
    },

    Remove {
        #[clap(short, long)]
        handle: u16,
    },
}

impl AdvertiseMonitorCommand {
    async fn proc(&self, client: &Client, index: u16) -> anyhow::Result<()> {
        match self {
            Self::Add {
                addr_type,
                offset,
                value,
            } => {
                let patterns = addr_type
                    .iter()
                    .zip(offset.iter())
                    .zip(value.iter())
                    .map(|((t, o), v)| btmgmt::AdvertisementPattern::new(*t, *o, &v.0))
                    .collect::<command::AddAdvertisementPatternsMonitor>();
                let reply = client.call(index, patterns).await?;
                println!("{:?}", reply.monitor_handle());
            }

            Self::Remove { handle } => {
                let reply = client
                    .call(
                        index,
                        command::RemoveAdvertisementPatternsMonitor::new((*handle).into()),
                    )
                    .await?;
                println!("{:?}", reply.monitor_handle());
            }
        };
        Ok(())
    }
}

#[derive(Debug, Clap)]
enum DeviceCommand {
    Add {
        #[clap(long, short)]
        address: btmgmt::Address,

        #[clap(long, short, conflicts_with_all=&["le", "random"])]
        bredr: bool,

        #[clap(long, short, conflicts_with = "bredr")]
        le: bool,

        #[clap(long, short, conflicts_with = "bredr")]
        random: bool,

        #[clap(long, short = 'A', conflicts_with = "background")]
        autoconnect: bool,

        #[clap(long, short = 'B', conflicts_with = "autoconnect")]
        background: bool,
    },

    Remove {
        #[clap(long, short)]
        address: btmgmt::Address,

        #[clap(long, short, conflicts_with_all=&["le", "random"])]
        bredr: bool,

        #[clap(long, short, conflicts_with = "bredr")]
        le: bool,

        #[clap(long, short, conflicts_with = "bredr")]
        random: bool,
    },

    Block {
        #[clap(long, short)]
        address: btmgmt::Address,

        #[clap(long, short, conflicts_with_all=&["le", "random"])]
        bredr: bool,

        #[clap(long, short, conflicts_with = "bredr")]
        le: bool,

        #[clap(long, short, conflicts_with = "bredr")]
        random: bool,
    },

    Unblock {
        #[clap(long, short)]
        address: btmgmt::Address,

        #[clap(long, short, conflicts_with_all=&["le", "random"])]
        bredr: bool,

        #[clap(long, short, conflicts_with = "bredr")]
        le: bool,

        #[clap(long, short, conflicts_with = "bredr")]
        random: bool,
    },

    Pair {
        #[clap(long, short)]
        address: btmgmt::Address,

        #[clap(long, short, conflicts_with_all=&["le", "random"])]
        bredr: bool,

        #[clap(long, short, conflicts_with = "bredr")]
        le: bool,

        #[clap(long, short, conflicts_with = "bredr")]
        random: bool,

        #[clap(long, conflicts_with_all=&["display-yesno", "keyboard-only", "no-input-no-output", "keyboard-display"])]
        display_only: bool,

        #[clap(long, conflicts_with_all=&["display-only", "keyboard-only", "no-input-no-output", "keyboard-display"])]
        display_yesno: bool,

        #[clap(long, conflicts_with_all=&["display-only", "display-yesno", "no-input-no-output", "keyboard-display"])]
        keyboard_only: bool,

        #[clap(long, conflicts_with_all=&["display-only", "display-yesno", "keyboard-only", "keyboard-display"])]
        no_input_no_output: bool,

        #[clap(long, conflicts_with_all=&["display-only", "display-yesno", "keyboard-only", "no-input-no-output"])]
        keyboard_display: bool,
    },

    CancelPair {
        #[clap(long, short)]
        address: btmgmt::Address,

        #[clap(long, short, conflicts_with_all=&["le", "random"])]
        bredr: bool,

        #[clap(long, short, conflicts_with = "bredr")]
        le: bool,

        #[clap(long, short, conflicts_with = "bredr")]
        random: bool,
    },

    Unpair {
        #[clap(long, short)]
        address: btmgmt::Address,

        #[clap(long, short, conflicts_with_all=&["le", "random"])]
        bredr: bool,

        #[clap(long, short, conflicts_with = "bredr")]
        le: bool,

        #[clap(long, short, conflicts_with = "bredr")]
        random: bool,

        #[clap(long, short)]
        disconnect: bool,
    },
}

impl DeviceCommand {
    async fn proc(&self, client: &Client, index: u16) -> anyhow::Result<()> {
        match self {
            Self::Add {
                address,
                bredr,
                le,
                random,
                autoconnect,
                background,
            } => {
                let addr_type = match (bredr, le, random) {
                    (true, false, false) | (false, false, false) => btmgmt::AddressType::BrEdr,
                    (false, true, false) => btmgmt::AddressType::LePublic,
                    (false, false, true) | (false, true, true) => btmgmt::AddressType::LeRandom,
                    _ => unreachable!(),
                };
                let action = match (autoconnect, background) {
                    (false, false) => btmgmt::Action::Allow,
                    (true, false) => btmgmt::Action::AutoConnect,
                    (false, true) => btmgmt::Action::Background,
                    _ => unreachable!(),
                };
                let reply = client
                    .call(
                        index,
                        command::AddDevice::new(address.clone(), addr_type, action),
                    )
                    .await?;
                println!("OK {:?}", reply);
            }

            Self::Remove {
                address,
                bredr,
                le,
                random,
            } => {
                let addr_type = match (bredr, le, random) {
                    (true, false, false) | (false, false, false) => btmgmt::AddressType::BrEdr,
                    (false, true, false) => btmgmt::AddressType::LePublic,
                    (false, false, true) | (false, true, true) => btmgmt::AddressType::LeRandom,
                    _ => unreachable!(),
                };
                let reply = client
                    .call(
                        index,
                        command::RemoveDevice::new(address.clone(), addr_type),
                    )
                    .await?;
                println!("OK {:?}", reply);
            }

            Self::Block {
                address,
                bredr,
                le,
                random,
            } => {
                let addr_type = match (bredr, le, random) {
                    (true, false, false) | (false, false, false) => btmgmt::AddressType::BrEdr,
                    (false, true, false) => btmgmt::AddressType::LePublic,
                    (false, false, true) | (false, true, true) => btmgmt::AddressType::LeRandom,
                    _ => unreachable!(),
                };
                let reply = client
                    .call(index, command::BlockDevice::new(address.clone(), addr_type))
                    .await?;
                println!("OK {:?}", reply);
            }

            Self::Unblock {
                address,
                bredr,
                le,
                random,
            } => {
                let addr_type = match (bredr, le, random) {
                    (true, false, false) | (false, false, false) => btmgmt::AddressType::BrEdr,
                    (false, true, false) => btmgmt::AddressType::LePublic,
                    (false, false, true) | (false, true, true) => btmgmt::AddressType::LeRandom,
                    _ => unreachable!(),
                };
                let reply = client
                    .call(
                        index,
                        command::UnblockDevice::new(address.clone(), addr_type),
                    )
                    .await?;
                println!("OK {:?}", reply);
            }

            Self::Pair {
                address,
                bredr,
                le,
                random,
                display_only,
                display_yesno,
                keyboard_only,
                no_input_no_output,
                keyboard_display,
            } => {
                let addr_type = match (bredr, le, random) {
                    (true, false, false) | (false, false, false) => btmgmt::AddressType::BrEdr,
                    (false, true, false) => btmgmt::AddressType::LePublic,
                    (false, false, true) | (false, true, true) => btmgmt::AddressType::LeRandom,
                    _ => unreachable!(),
                };
                let capability = match (
                    display_only,
                    display_yesno,
                    keyboard_only,
                    no_input_no_output,
                    keyboard_display,
                ) {
                    (false, false, false, false, false) => btmgmt::IoCapability::NoInputNoOutput,
                    (true, false, false, false, false) => btmgmt::IoCapability::DisplayOnly,
                    (false, true, false, false, false) => btmgmt::IoCapability::DisplayYesNo,
                    (false, false, true, false, false) => btmgmt::IoCapability::KeyboardOnly,
                    (false, false, false, true, false) => btmgmt::IoCapability::NoInputNoOutput,
                    (false, false, false, false, true) => btmgmt::IoCapability::KeyboardDisplay,
                    _ => unreachable!(),
                };
                let reply = client
                    .call(
                        index,
                        command::PairDevice::new(address.clone(), addr_type, capability),
                    )
                    .await?;
                println!("OK {:?}", reply);
            }

            Self::CancelPair {
                address,
                bredr,
                le,
                random,
            } => {
                let addr_type = match (bredr, le, random) {
                    (true, false, false) | (false, false, false) => btmgmt::AddressType::BrEdr,
                    (false, true, false) => btmgmt::AddressType::LePublic,
                    (false, false, true) | (false, true, true) => btmgmt::AddressType::LeRandom,
                    _ => unreachable!(),
                };
                let reply = client
                    .call(
                        index,
                        command::CancelPairDevice::new(address.clone(), addr_type),
                    )
                    .await?;
                println!("OK {:?}", reply);
            }

            Self::Unpair {
                address,
                bredr,
                le,
                random,
                disconnect,
            } => {
                let addr_type = match (bredr, le, random) {
                    (true, false, false) | (false, false, false) => btmgmt::AddressType::BrEdr,
                    (false, true, false) => btmgmt::AddressType::LePublic,
                    (false, false, true) | (false, true, true) => btmgmt::AddressType::LeRandom,
                    _ => unreachable!(),
                };
                let reply = client
                    .call(
                        index,
                        command::UnpairDevice::new(address.clone(), addr_type, *disconnect),
                    )
                    .await?;
                println!("OK {:?}", reply);
            }
        };
        Ok(())
    }
}

#[derive(Debug, Clap)]
enum OobCommand {
    Add {
        #[clap(long, short)]
        address: btmgmt::Address,

        #[clap(long, short, conflicts_with_all=&["le", "random"])]
        bredr: bool,

        #[clap(long, short, conflicts_with = "bredr")]
        le: bool,

        #[clap(long, short, conflicts_with = "bredr")]
        random: bool,

        #[clap(long, validator=length(32))]
        hash192: HexBinary,

        #[clap(long, validator=length(32))]
        randomizer192: HexBinary,

        #[clap(long, validator=length(32))]
        hash256: Option<HexBinary>,

        #[clap(long, validator=length(32))]
        randomizer256: Option<HexBinary>,
    },

    Remove {
        #[clap(long, short)]
        address: btmgmt::Address,

        #[clap(long, short, conflicts_with_all=&["le", "random"])]
        bredr: bool,

        #[clap(long, short, conflicts_with = "bredr")]
        le: bool,

        #[clap(long, short, conflicts_with = "bredr")]
        random: bool,
    },

    Read {
        #[clap(long, short)]
        extended: bool,

        #[clap(long, short, conflicts_with = "le", requires = "extended")]
        bredr: bool,

        #[clap(long, short, conflicts_with = "bredr", requires = "extended")]
        le: bool,
    },
}

impl OobCommand {
    async fn proc(&self, client: &Client, index: u16) -> anyhow::Result<()> {
        match self {
            Self::Add {
                address,
                bredr,
                le,
                random,
                hash192,
                randomizer192,
                hash256,
                randomizer256,
            } => {
                let into_array = |b: Vec<_>| {
                    let mut v = [0; 16];
                    v.copy_from_slice(&b);
                    v
                };
                let addr_type = match (bredr, le, random) {
                    (true, false, false) | (false, false, false) => btmgmt::AddressType::BrEdr,
                    (false, true, false) => btmgmt::AddressType::LePublic,
                    (false, false, true) | (false, true, true) => btmgmt::AddressType::LeRandom,
                    _ => unreachable!(),
                };
                let hash192 = into_array(hash192.0.clone());
                let randomizer192 = into_array(randomizer192.0.clone());
                let hash256 = hash256.as_ref().map(|b| into_array(b.0.clone()));
                let randomizer256 = randomizer256.as_ref().map(|b| into_array(b.0.clone()));
                let reply = client
                    .call(
                        index,
                        command::AddRemoteOutOfBandData::new(
                            address.clone(),
                            addr_type,
                            hash192,
                            randomizer192,
                            hash256,
                            randomizer256,
                        ),
                    )
                    .await?;
                println!("OK {:?}", reply);
            }

            Self::Remove {
                address,
                bredr,
                le,
                random,
            } => {
                let addr_type = match (bredr, le, random) {
                    (true, false, false) | (false, false, false) => btmgmt::AddressType::BrEdr,
                    (false, true, false) => btmgmt::AddressType::LePublic,
                    (false, false, true) | (false, true, true) => btmgmt::AddressType::LeRandom,
                    _ => unreachable!(),
                };
                let reply = client
                    .call(
                        index,
                        command::RemoveRemoteOutOfBandData::new(address.clone(), addr_type),
                    )
                    .await?;
                println!("OK {:?}", reply);
            }

            Self::Read {
                extended,
                bredr,
                le,
            } => {
                if *extended {
                    let addr_type = match (bredr, le) {
                        (false, false) | (true, false) => {
                            vec![btmgmt::AddressType::BrEdr].into_iter().collect()
                        }
                        (false, true) => {
                            vec![btmgmt::AddressType::LePublic, btmgmt::AddressType::LeRandom]
                                .into_iter()
                                .collect()
                        }
                        _ => unreachable!(),
                    };
                    let reply = client
                        .call(
                            index,
                            command::ReadLocalOutOfBandExtendedData::new(addr_type),
                        )
                        .await?;
                    println!("OK {:?}", reply);
                } else {
                    let reply = client
                        .call(index, command::ReadLocalOutOfBandData::new())
                        .await?;
                    println!("OK {:?}", reply);
                }
            }
        };
        Ok(())
    }
}

#[derive(Debug)]
enum OnOff {
    On,
    Off,
}

impl FromStr for OnOff {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            v => Err(v.into()),
        }
    }
}

#[derive(Debug)]
enum Discoerable {
    On,
    Off,
    Limited,
}

impl FromStr for Discoerable {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            "limited" => Ok(Self::Limited),
            v => Err(v.into()),
        }
    }
}

#[derive(Debug)]
struct AddressType(btmgmt::AddressType);

impl FromStr for AddressType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bredr" => Ok(Self(btmgmt::AddressType::BrEdr)),
            "le_public" => Ok(Self(btmgmt::AddressType::LePublic)),
            "le_random" => Ok(Self(btmgmt::AddressType::LeRandom)),
            v => Err(v.into()),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct HexBinary(Vec<u8>);

impl FromStr for HexBinary {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = (0..s.len())
            .map(|i| u8::from_str_radix(&s[i..i + 1], 16))
            .collect::<Result<Vec<u8>, _>>()?;
        Ok(Self(v))
    }
}

/*
#[derive(Debug)]
struct CoD(btmgmt::ClassOfDevice);

impl FromStr for CoD {
type Err = String;
fn from_str(s: &str) -> Result<Self, Self::Err> {
let maybe_values = [s.get(0..2), s.get(2..4), s.get(4..6)]
.iter()
.map(|e| e.and_then(|v| u8::from_str(v).ok()))
.collect::<Option<Vec<_>>>();
if let Some(&[i1, i2, i3]) = maybe_values.as_deref() {
Ok(Self([i1, i2, i3].into()))
} else {
Err("invalid format".into())
}
}
}
*/

fn handle_event(index: btmgmt::ControllerIndex, event: Event) {
    println!("{:?} {:?}", index, event);
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let opt = Opt::parse();

    let index = opt.index;
    let listen = opt.listen || opt.command.is_none();

    let (client, handle) = Client::open()?;

    let mut events = client.events().await;
    tokio::spawn(async move {
        while let Some((index, event)) = events.next().await {
            handle_event(index, event);
        }
    });

    if let Some(command) = opt.command {
        match command {
            Command::Version => {
                let reply = client
                    .call(None, command::ReadManagementVersionInformation::new())
                    .await?;
                println!("{}.{}", reply.version(), reply.revision());
            }

            Command::SupportedCommands => {
                let reply = client
                    .call(None, command::ReadManagementSupportedCommands::new())
                    .await?;
                println!("commands");
                for command in reply.commands() {
                    println!("  {:?}", command);
                }
                println!("events");
                for event in reply.events() {
                    println!("  {:?}", event);
                }
            }

            Command::Controller { command } => {
                command.unwrap_or_default().proc(&client, index).await?
            }
            Command::Key { .. } => todo!(),
            Command::Connection { command } => {
                command.unwrap_or_default().proc(&client, index).await?
            }
            Command::Discovery { command } => command.proc(&client, index).await?,
            Command::Configuration { command } => command.proc(&client, index).await?,
            Command::Advertise { command } => command.proc(&client, index).await?,
            Command::Device { command } => command.proc(&client, index).await?,
            Command::Oob { command } => command.proc(&client, index).await?,
        };
    }

    if listen {
        std::mem::forget(client);
    } else {
        drop(client);
    }

    handle.await??;

    Ok(())
}
