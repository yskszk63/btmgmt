#![doc(html_root_url = "https://docs.rs/btmgmt/0.1.0")]
//! Linux bluetooth mgmt API client.
//!
//! see [bluez docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
//!
//! ## Dependencies
//!
//! ```toml
//! [dependencies]
//! btmgmt = "0.1.0"
//! ```
//!
//! ## Example
//!
//! ```no_run
//! use btmgmt::Client;
//! use btmgmt::command::ReadManagementSupportedCommands;
//! use tokio_stream::StreamExt;
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() {
//!     // (management client, run loop handle)
//!     let (client, handle) = Client::open().unwrap();
//!     let handle = tokio::spawn(handle);
//!
//!     let mut events = client.events().await;
//!     tokio::spawn(async move {
//!         while let Some((index, event)) = events.next().await {
//!             match event {
//!                 // do staff
//!#                _ => {}
//!             }
//!         }
//!     });
//!
//!     let reply = client.call(None, ReadManagementSupportedCommands::new()).await.unwrap();
//!     for command in reply.commands() {
//!         // do stuff
//!     }
//!     for event in reply.events() {
//!         // do stuff
//!     }
//!
//!     drop(client); // may be run loop exit.
//!     handle.await.unwrap();
//! }
//! ```
//!
//! ## Command line client
//!
//! ```bash
//! $ cargo install btmgmt-cli
//! ...
//! $ btmgmt-cli version
//! 1.18
//! $
//! ```
//!
//! Many operations require privileges.
//!
//! ## License
//!
//! Licensed under either of
//! * Apache License, Version 2.0
//!   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
//! * MIT license
//!   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
//! at your option.
//!
//! ## Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
//! dual licensed as above, without any additional terms or conditions.!
use btmgmt_packet as packet;

pub use client::Client;
pub use packet::{
    pack, command, event, Action, Address, AddressType, AdvertiseInstance, AdvertisementMonitorFeatures,
    AdvertisementMonitorHandle, AdvertisementPattern, Advertising, AdvertisingFlag, BlockedKey,
    ClassOfDevice, ConfirmHint, ConnectionParameter, ControllerBus, ControllerConfigurationOption,
    ControllerIndex, ControllerType, DeviceConnectFlags, DeviceDisconnectReason, DeviceFlags,
    Discoverable, ErrorCode, FeatureAction, FeatureFlags, IdentityResolvingKey, IoCapability,
    LinkKey, LinkKeyType, LongTermKey, LongTermKeyType, Name, NameError, Phys,
    Privacy, RuntimeConfigurationParameter, RuntimeConfigurationParameterType, SecureConnections,
    Settings, ShortName, SignatureResolvingKey, SuspendState, SystemConfigurationParameter,
    SystemConfigurationParameterType, Uuid, WakeReason,
};

mod sock;
pub mod client;
