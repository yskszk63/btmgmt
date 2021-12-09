#![doc(html_root_url = "https://docs.rs/btmgmt/0.3.0-alpha.4")]
//! Linux bluetooth mgmt API client.
//!
//! see [bluez docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
//!
//! ## Dependencies
//!
//! ```toml
//! [dependencies]
//! btmgmt = "0.3.0-alpha.4"
//! ```
//!
//! ## Example
//!
//! ```no_run
//! use btmgmt::Client;
//! use btmgmt::command::ReadManagementSupportedCommands;
//! use futures::StreamExt;
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() {
//!     // (management client, run loop handle)
//!     let client = Client::open().unwrap();
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
//!     let reply = client.call(None, ReadManagementSupportedCommands).await.unwrap();
//!     for command in reply.commands() {
//!         // do stuff
//!     }
//!     for event in reply.events() {
//!         // do stuff
//!     }
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
//!   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
//! * MIT license
//!   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
//! at your option.
//!
//! ## Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
//! dual licensed as above, without any additional terms or conditions.!
pub use btmgmt_packet as packet;
pub use client::Client;
pub use packet::{command, event};
pub mod client;
mod sock;
