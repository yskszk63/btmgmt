#![doc(html_root_url = "https://docs.rs/btmgmt-packet/0.2.5")]
#![allow(non_upper_case_globals)]
//! Linux bluetooth mgmt API packet structures.
//!
//! see [bluez docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
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

use std::collections::HashSet;
use std::convert::{TryFrom, TryInto};
use std::ffi::{CString, NulError};
use std::fmt;
use std::io;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

pub use bdaddr;
use bitflags::bitflags;
use derive_new::new as New;
use getset::Getters;

use btmgmt_packet_helper as helper;
use helper::helper::{IterNewtype, Newtype};
#[doc(hidden)]
pub use helper::pack::{self, Pack, Unpack};

pub mod command;
pub mod event;

fn split(addr: bdaddr::Address) -> (Address, AddressType) {
    let address_type = match &addr {
        crate::bdaddr::Address::BrEdr(..) => AddressType::BrEdr,
        crate::bdaddr::Address::LePublic(..) => AddressType::LePublic,
        crate::bdaddr::Address::LeRandom(..) => AddressType::LeRandom,
    };
    let address = Address(addr.into_bd_addr());
    (address, address_type)
}

fn join(ty: &AddressType, addr: &Address) -> bdaddr::Address {
    match ty {
        AddressType::BrEdr => addr.0.clone().to_br_edr_addr(),
        AddressType::LePublic => addr.0.clone().to_le_public_addr(),
        AddressType::LeRandom => addr.0.clone().to_le_random_addr(),
    }
}

#[derive(Debug, Clone, Newtype, New)]
struct Address(bdaddr::BdAddr);

impl FromStr for Address {
    type Err = <bdaddr::BdAddr as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(FromStr::from_str(s)?))
    }
}

impl Pack for Address {
    fn pack<W>(&self, write: &mut W) -> pack::Result<()>
    where
        W: io::Write,
    {
        <[u8; 6]>::pack(&self.0.clone().into(), write)
    }
}

impl Unpack for Address {
    fn unpack<R>(read: &mut R) -> pack::Result<Self>
    where
        R: io::Read,
    {
        <[u8; 6]>::unpack(read).map(|a| Self(a.into()))
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Pack, Unpack)]
#[pack(u8)]
pub enum ErrorCode {
    Success = 0x00,
    UnknownCommand = 0x01,
    NotConnected = 0x02,
    Failed = 0x03,
    ConnectFailed = 0x04,
    AuthenticationFailed = 0x05,
    NotPaired = 0x06,
    NoResources = 0x07,
    Timeout = 0x08,
    AlreadyConnected = 0x09,
    Busy = 0x0A,
    Rejected = 0x0B,
    NotSupported = 0x0C,
    InvalidParameters = 0x0D,
    Disconnected = 0x0E,
    NotPowered = 0x0F,
    Cancelled = 0x10,
    InvalidIndex = 0x11,
    RfKilled = 0x12,
    AlreadyPaired = 0x13,
    PermissionDenied = 0x14,
}

impl ErrorCode {
    pub fn success(&self) -> bool {
        self == &Self::Success
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let m = match self {
            Self::Success => "Success (0x00)",
            Self::UnknownCommand => "Unknown Command (0x01)",
            Self::NotConnected => "Not Connected (0x02)",
            Self::Failed => "Failed (0x03)",
            Self::ConnectFailed => "Connect Failed (0x04)",
            Self::AuthenticationFailed => "Authentication Failed (0x05)",
            Self::NotPaired => "Not Paired (0x06)",
            Self::NoResources => "No Resources (0x07)",
            Self::Timeout => "Timeout (0x08)",
            Self::AlreadyConnected => "Already Connected (0x09)",
            Self::Busy => "Busy (0x0A)",
            Self::Rejected => "Rejected (0x0B)",
            Self::NotSupported => "Not Supported (0x0C)",
            Self::InvalidParameters => "Invalid Parameters (0x0D)",
            Self::Disconnected => "Disconnected (0x0E)",
            Self::NotPowered => "Not Powered (0x0F)",
            Self::Cancelled => "Cancelled (0x10)",
            Self::InvalidIndex => "Invalid Index (0x11)",
            Self::RfKilled => "RFKilled (0x12)",
            Self::AlreadyPaired => "Already Paired (0x13)",
            Self::PermissionDenied => "Permission Denied (0x14)",
        };
        write!(f, "{}", m)
    }
}

/// Controller Index
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ControllerIndex {
    /// Controller index
    ControllerId(u16),
    /// Not related to any controller
    NonController,
}

impl ControllerIndex {
    /// true if [`Self::NonController`]
    pub fn is_non(&self) -> bool {
        self == &Self::NonController
    }
}

impl From<u16> for ControllerIndex {
    fn from(v: u16) -> Self {
        if v == 0xFFFF {
            Self::NonController
        } else {
            Self::ControllerId(v)
        }
    }
}

impl From<Option<u16>> for ControllerIndex {
    fn from(v: Option<u16>) -> Self {
        match v {
            Some(v) if v != 0xFFFF => ControllerIndex::ControllerId(v),
            _ => ControllerIndex::NonController,
        }
    }
}

impl From<ControllerIndex> for u16 {
    fn from(v: ControllerIndex) -> Self {
        match v {
            ControllerIndex::ControllerId(v) => v,
            ControllerIndex::NonController => 0xFFFF,
        }
    }
}

impl Pack for ControllerIndex {
    fn pack<W>(&self, write: &mut W) -> pack::Result<()>
    where
        W: io::Write,
    {
        u16::from(self.clone()).pack(write)?;
        Ok(())
    }
}

impl Unpack for ControllerIndex {
    fn unpack<R>(read: &mut R) -> pack::Result<Self>
    where
        R: io::Read,
    {
        u16::unpack(read).map(Into::into)
    }
}

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct CommandsEvents {
    commands: Vec<command::CommandCode>,
    events: Vec<event::EventCode>,
}

impl Pack for CommandsEvents {
    fn pack<W>(&self, write: &mut W) -> pack::Result<()>
    where
        W: io::Write,
    {
        (self.commands.len() as u16).pack(write)?;
        (self.events.len() as u16).pack(write)?;
        for item in &self.commands {
            item.pack(write)?;
        }
        for item in &self.events {
            item.pack(write)?;
        }
        Ok(())
    }
}

impl Unpack for CommandsEvents {
    fn unpack<R>(read: &mut R) -> pack::Result<Self>
    where
        R: io::Read,
    {
        let num_commands = u16::unpack(read)?;
        let num_events = u16::unpack(read)?;
        let commands = (0..num_commands)
            .map(|_| Unpack::unpack(read))
            .collect::<Result<_, _>>()?;
        let events = (0..num_events)
            .map(|_| Unpack::unpack(read))
            .collect::<Result<_, _>>()?;
        Ok(Self { commands, events })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Pack, Unpack)]
#[pack(u8)]
pub enum AddressType {
    BrEdr = 0,
    LePublic = 1,
    LeRandom = 2,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, IterNewtype)]
#[iter_newtype(item = AddressType, into_iter = ::std::collections::hash_set::IntoIter<AddressType>, no_iter_mut)]
pub struct AddressTypes(HashSet<AddressType>);

impl Pack for AddressTypes {
    fn pack<W>(&self, write: &mut W) -> pack::Result<()>
    where
        W: io::Write,
    {
        let mut v = 0u8;
        for item in &self.0 {
            v |= match item {
                AddressType::BrEdr => 1 << 0,
                AddressType::LePublic => 1 << 1,
                AddressType::LeRandom => 1 << 2,
            };
        }
        v.pack(write)
    }
}

impl Unpack for AddressTypes {
    fn unpack<R>(read: &mut R) -> pack::Result<Self>
    where
        R: io::Read,
    {
        let v = u8::unpack(read)?;
        let mut r = HashSet::new();

        if v & 1 << 0 != 0 {
            r.insert(AddressType::BrEdr);
        }
        if v & 1 << 1 != 0 {
            r.insert(AddressType::LePublic);
        }
        if v & 1 << 2 != 0 {
            r.insert(AddressType::LeRandom);
        }

        Ok(Self(r))
    }
}

bitflags! {
    #[derive(Pack, Unpack)]
    pub struct Settings: u32 {
        const Powered = 1 << 0;
        const Connectable = 1 << 1;
        const FastConnectable = 1 << 2;
        const Discoverable = 1 << 3;
        const Bondable = 1 << 4;
        const LinkLevelSecurity = 1 << 5;
        const SecureSimplePairing = 1 << 6;
        const BasicRateEnhancedDataRate = 1 << 7;
        const HighSpeed = 1 << 8;
        const LowEnergy = 1 << 9;
        const Advertising = 1 << 10;
        const SecureConnections = 1 << 11;
        const DebugKeys = 1 << 12;
        const Privacy = 1 << 13;
        const ControllerConfiguration = 1 << 14;
        const StaticAddress = 1 << 15;
        const PhyConfiguration = 1 << 16;
        const WidebandSpeech = 1 << 17;
    }
}

#[derive(Debug, Clone, Pack, Unpack)]
pub struct ClassOfDevice([u8; 3]);

impl From<[u8; 3]> for ClassOfDevice {
    fn from(v: [u8; 3]) -> Self {
        Self(v) // FIXME reverse?
    }
}

impl fmt::Display for ClassOfDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02X}{:02X}{:02X}", self.0[0], self.0[1], self.0[2]) // FIXME reverse?
    }
}

#[derive(Debug, thiserror::Error)]
#[error("length too long {0:} < {1:}")]
pub struct LengthTooLong(usize, usize);

#[derive(Debug, thiserror::Error)]
pub enum NameError {
    #[error(transparent)]
    LengthTooLong(#[from] LengthTooLong),

    #[error(transparent)]
    NulError(#[from] NulError),
}

#[derive(Clone, Pack, Unpack)]
pub struct FixedLengthName<const N: usize>(Box<[u8; N]>);

impl<const N: usize> FixedLengthName<N> {
    pub fn new<T: Into<Vec<u8>>>(t: T) -> Result<Self, NameError> {
        let s = CString::new(t)?;
        let b = s.as_bytes_with_nul();
        if b.len() > N {
            return Err(LengthTooLong(N, b.len()).into());
        }

        let mut v = [0; N];
        (&mut v[..b.len()]).copy_from_slice(b);
        Ok(Self(Box::new(v)))
    }

    pub fn to_string_lossy(&self) -> String {
        let b = self.0.split(|b| b == &0).next().unwrap_or(b"");
        CString::new(b).unwrap().to_string_lossy().to_string()
    }
}

impl<const N: usize> fmt::Debug for FixedLengthName<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b = self.0.split(|b| b == &0).next().unwrap_or(b"");
        CString::new(b).unwrap().fmt(f)
    }
}

impl<const N: usize> FromStr for FixedLengthName<N> {
    type Err = NameError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

pub type Name = FixedLengthName<249>;
pub type ShortName = FixedLengthName<11>;

#[derive(Debug, Pack, Unpack)]
#[pack(u8)]
pub enum Discoverable {
    Disable = 0x00,
    General = 0x01,
    Limited = 0x02,
}

#[derive(Debug, Clone, Default, Newtype, New)]
pub struct Uuid(uuid::Uuid);

impl Pack for Uuid {
    fn pack<W>(&self, read: &mut W) -> pack::Result<()>
    where
        W: io::Write,
    {
        self.0.to_u128_le().pack(read)?;
        Ok(())
    }
}

impl Unpack for Uuid {
    fn unpack<R>(read: &mut R) -> pack::Result<Self>
    where
        R: io::Read,
    {
        Ok(Self(uuid::Uuid::from_u128_le(Unpack::unpack(read)?)))
    }
}

impl FromStr for Uuid {
    type Err = <uuid::Uuid as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(FromStr::from_str(s)?))
    }
}

#[derive(Debug, Clone, Pack, Unpack)]
#[pack(u8)]
pub enum LinkKeyType {
    Combinationkey = 0x00,
    LocalUnitkey = 0x01,
    RemoteUnitkey = 0x02,
    DebugCombinationkey = 0x03,
    UnauthenticatedCombinationkeyfromP192 = 0x04,
    AuthenticatedCombinationkeyfromP192 = 0x05,
    ChangedCombinationkey = 0x06,
    UnauthenticatedCombinationkeyfromP256 = 0x07,
    AuthenticatedCombinationkeyfromP256 = 0x08,
}

#[derive(Debug, Clone, Pack, Unpack, Getters)]
pub struct LinkKey {
    address: Address,
    address_type: AddressType,
    #[getset(get = "pub")]
    key_type: LinkKeyType,
    #[getset(get = "pub")]
    value: [u8; 16],
    #[getset(get = "pub")]
    pin_length: u8,
}

impl LinkKey {
    pub fn new(
        addr: bdaddr::Address,
        key_type: LinkKeyType,
        value: [u8; 16],
        pin_length: u8,
    ) -> Self {
        let (address, address_type) = split(addr);
        Self {
            address,
            address_type,
            key_type,
            value,
            pin_length,
        }
    }

    pub fn address(&self) -> &crate::bdaddr::BdAddr {
        &self.address.0
    }
}

#[derive(Debug, Clone, Pack, Unpack)]
#[pack(u8)]
pub enum LongTermKeyType {
    UnauthenticatedKey = 0x00,
    AuthenticatedKey = 0x01,
    UnauthenticatedP256Key = 0x02,
    AuthenticatedP256Key = 0x03,
    DebugKeyP256 = 0x04,
}

#[derive(Debug, thiserror::Error)]
#[error("uninitialized field: {0:}")]
pub struct LongTermKeyBuilderError(&'static str);

#[derive(Clone, Default)]
pub struct LongTermKeyBuilder {
    address: Option<bdaddr::Address>,
    key_type: Option<LongTermKeyType>,
    master: Option<bool>,
    encryption_size: Option<u8>,
    encryption_diversifier: Option<u16>,
    random_number: Option<[u8; 8]>,
    value: Option<[u8; 16]>,
}

impl LongTermKeyBuilder {
    pub fn address(&mut self, addr: bdaddr::Address) -> &mut Self {
        self.address = Some(addr);
        self
    }
    pub fn key_type(&mut self, key_type: LongTermKeyType) -> &mut Self {
        self.key_type = Some(key_type);
        self
    }
    pub fn master(&mut self, master: bool) -> &mut Self {
        self.master = Some(master);
        self
    }
    pub fn encryption_size(&mut self, encryption_size: u8) -> &mut Self {
        self.encryption_size = Some(encryption_size);
        self
    }
    pub fn encryption_diversifier(&mut self, encryption_diversifier: u16) -> &mut Self {
        self.encryption_diversifier = Some(encryption_diversifier);
        self
    }
    pub fn random_number(&mut self, random_number: [u8; 8]) -> &mut Self {
        self.random_number = Some(random_number);
        self
    }
    pub fn value(&mut self, value: [u8; 16]) -> &mut Self {
        self.value = Some(value);
        self
    }
    pub fn build(&self) -> Result<LongTermKey, LongTermKeyBuilderError> {
        let address = if let Some(address) = &self.address {
            address.clone()
        } else {
            return Err(LongTermKeyBuilderError("address"));
        };
        let key_type = if let Some(key_type) = &self.key_type {
            key_type.clone()
        } else {
            return Err(LongTermKeyBuilderError("key_type"));
        };
        let master = if let Some(master) = self.master {
            master
        } else {
            return Err(LongTermKeyBuilderError("master"));
        };
        let encryption_size = if let Some(encryption_size) = self.encryption_size {
            encryption_size
        } else {
            return Err(LongTermKeyBuilderError("encryption_size"));
        };
        let encryption_diversifier =
            if let Some(encryption_diversifier) = self.encryption_diversifier {
                encryption_diversifier
            } else {
                return Err(LongTermKeyBuilderError("encryption_diversifier"));
            };
        let random_number = if let Some(random_number) = self.random_number {
            random_number
        } else {
            return Err(LongTermKeyBuilderError("random_number"));
        };
        let value = if let Some(value) = self.value {
            value
        } else {
            return Err(LongTermKeyBuilderError("value"));
        };

        let (address, address_type) = split(address);
        Ok(LongTermKey {
            address,
            address_type,
            key_type,
            master,
            encryption_size,
            encryption_diversifier,
            random_number,
            value,
        })
    }
}

#[derive(Debug, Clone, Pack, Unpack, Getters)]
pub struct LongTermKey {
    address: Address,
    address_type: AddressType,
    #[getset(get = "pub")]
    key_type: LongTermKeyType,
    #[getset(get = "pub")]
    master: bool,
    #[getset(get = "pub")]
    encryption_size: u8,
    #[getset(get = "pub")]
    encryption_diversifier: u16,
    #[getset(get = "pub")]
    random_number: [u8; 8],
    #[getset(get = "pub")]
    value: [u8; 16],
}

impl LongTermKey {
    pub fn address(&self) -> &crate::bdaddr::BdAddr {
        &self.address.0
    }
}

#[derive(Debug, Clone, Pack, Unpack, Getters)]
pub struct IdentityResolvingKey {
    address: Address,
    address_type: AddressType,
    #[getset(get = "pub")]
    value: [u8; 16],
}

impl IdentityResolvingKey {
    pub fn new(addr: bdaddr::Address, value: [u8; 16]) -> Self {
        let (address, address_type) = split(addr);
        Self {
            address,
            address_type,
            value,
        }
    }

    pub fn address(&self) -> &crate::bdaddr::BdAddr {
        &self.address.0
    }
}

#[derive(Debug, Pack, Unpack)]
#[pack(u8)]
pub enum IoCapability {
    DisplayOnly = 0,
    DisplayYesNo = 1,
    KeyboardOnly = 2,
    NoInputNoOutput = 3,
    KeyboardDisplay = 4,
}

#[derive(Debug, Pack, Unpack)]
#[pack(u16)]
pub enum DeviceIdSource {
    DisableDeviceId = 0x0000,
    BluetoothSig = 0x0001,
    UsbImplementersForum = 0x0002,
}

#[derive(Debug, Pack, Unpack)]
#[pack(u8)]
pub enum Advertising {
    Disable = 0x00,
    Enable = 0x01,
    Connectable = 0x02,
}

#[derive(Debug, Pack, Unpack)]
#[pack(u8)]
pub enum SecureConnections {
    Disable = 0x00,
    Enable = 0x01,
    Only = 0x02,
}

#[derive(Debug, Pack, Unpack)]
#[pack(u8)]
pub enum DebugKeys {
    Disable = 0x00,
    Enable = 0x01,
    ForEach = 0x02,
}

#[derive(Debug, Pack, Unpack)]
#[pack(u8)]
pub enum Privacy {
    Disable = 0x00,
    Enable = 0x01,
    Limited = 0x02,
}

#[derive(Debug, Clone, Pack, Unpack)]
#[pack(u8)]
pub enum Action {
    Background = 0,
    Allow = 1,
    AutoConnect = 2,
}

#[derive(Debug, Pack, Unpack, Getters)]
pub struct ConnectionParameter {
    address: Address,
    address_type: AddressType,
    #[getset(get = "pub")]
    min_connection_interval: u16,
    #[getset(get = "pub")]
    max_connection_interval: u16,
    #[getset(get = "pub")]
    connection_latency: u16,
    #[getset(get = "pub")]
    supervision_timeout: u16,
}

impl ConnectionParameter {
    pub fn new(
        addr: bdaddr::Address,
        min_connection_interval: u16,
        max_connection_interval: u16,
        connection_latency: u16,
        supervision_timeout: u16,
    ) -> Self {
        let (address, address_type) = split(addr);
        Self {
            address,
            address_type,
            min_connection_interval,
            max_connection_interval,
            connection_latency,
            supervision_timeout,
        }
    }

    pub fn address(&self) -> bdaddr::Address {
        join(&self.address_type, &self.address)
    }
}

bitflags! {
    #[derive(Pack, Unpack)]
    pub struct ControllerConfigurationOption: u32 {
        const ExternalConfiguration = 1 << 0;
        const BluetoothPublicAddressConfiguration = 1 << 1;
    }
}

#[derive(Debug, Clone)]
pub struct VariableLengthBytes<L = u16>(Box<[u8]>, PhantomData<L>);

impl Pack for VariableLengthBytes<u16> {
    fn pack<W>(&self, write: &mut W) -> pack::Result<()>
    where
        W: io::Write,
    {
        (self.0.len() as u16).pack(write)?;
        self.0.pack(write)?;
        Ok(())
    }
}

impl Unpack for VariableLengthBytes<u16> {
    fn unpack<R>(read: &mut R) -> pack::Result<Self>
    where
        R: io::Read,
    {
        let len = u16::unpack(read)? as usize;
        let mut read = <&mut R as io::Read>::take(read, len as u64);
        let buf = <Box<[u8]>>::unpack(&mut read)?;
        if buf.len() != len {
            return Err(io::Error::new(io::ErrorKind::Other, "few bytes.").into());
        }

        Ok(Self(buf, PhantomData))
    }
}

impl Pack for VariableLengthBytes<u8> {
    fn pack<W>(&self, write: &mut W) -> pack::Result<()>
    where
        W: io::Write,
    {
        (self.0.len() as u8).pack(write)?;
        self.0.pack(write)?;
        Ok(())
    }
}

impl Unpack for VariableLengthBytes<u8> {
    fn unpack<R>(read: &mut R) -> pack::Result<Self>
    where
        R: io::Read,
    {
        let len = u8::unpack(read)? as usize;
        let mut read = <&mut R as io::Read>::take(read, len as u64);
        let buf = <Box<[u8]>>::unpack(&mut read)?;
        if buf.len() != len {
            return Err(io::Error::new(io::ErrorKind::Other, "few bytes.").into());
        }

        Ok(Self(buf, PhantomData))
    }
}

// ?
impl<L> From<u16> for VariableLengthBytes<L> {
    fn from(v: u16) -> Self {
        Self(v.to_le_bytes().to_vec().into(), PhantomData)
    }
}

// ?
impl<L> TryFrom<VariableLengthBytes<L>> for u16 {
    type Error = (); // FIXME
    fn try_from(value: VariableLengthBytes<L>) -> Result<Self, Self::Error> {
        match *value.0.as_ref() {
            [i1, i2] => Ok(u16::from_le_bytes([i1, i2])),
            _ => Err(()),
        }
    }
}

impl<L> AsRef<[u8]> for VariableLengthBytes<L> {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<L> AsMut<[u8]> for VariableLengthBytes<L> {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl<L> Deref for VariableLengthBytes<L> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<L> DerefMut for VariableLengthBytes<L> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, Pack, Unpack)]
#[pack(u8)]
pub enum ControllerType {
    PrimaryController = 0x00,
    UnconfiguredController = 0x01,
    AlternateMacPhyController = 0x02,
}

#[derive(Debug, Clone, Pack, Unpack)]
#[pack(u8)]
pub enum ControllerBus {
    Virtual = 0x00,
    Usb = 0x01,
    Pcmcia = 0x02,
    Uart = 0x03,
    Rs232 = 0x04,
    Pci = 0x05,
    Sdio = 0x06,
    Spi = 0x07,
    I2c = 0x08,
    Smd = 0x09,
    Virtio = 0x0A,
}

bitflags! {
    #[derive(Pack, Unpack)]
    pub struct AdvertisingFlag: u32 {
        const SwitchIntoConnectableMode = 1 << 0;
        const AdvertiseAsDiscoverable = 1 << 1;
        const AdvertiseAsLimitedDiscoverable = 1 << 2;
        const AddFlagsFieldToAdvData = 1 << 3;
        const AddTxPowerFieldToAdvData = 1 << 4;
        const AddAppearanceFieldToScanResp = 1 << 5;
        const AddLocalNameInScanResp = 1 << 6;
        const SecondaryChannelWithLe1M = 1 << 7;
        const SecondaryChannelWithLe2M = 1 << 8;
        const SecondaryChannelWithLeCoded = 1 << 9;
    }
}

#[derive(Debug, Clone, Pack, Unpack, Newtype, New)]
pub struct AdvertiseInstance(u8);

#[derive(Debug, IterNewtype)]
pub struct AdvertiseInstances(Vec<AdvertiseInstance>);

impl Pack for AdvertiseInstances {
    fn pack<W>(&self, write: &mut W) -> pack::Result<()>
    where
        W: io::Write,
    {
        (self.0.len() as u8).pack(write)?;
        for item in &self.0 {
            item.pack(write)?;
        }
        Ok(())
    }
}

impl Unpack for AdvertiseInstances {
    fn unpack<R>(read: &mut R) -> pack::Result<Self>
    where
        R: io::Read,
    {
        let len = u8::unpack(read)? as usize;
        let v = (0..len)
            .map(|_| Unpack::unpack(read))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(v))
    }
}

impl<'a> std::iter::IntoIterator for &'a AdvertiseInstances {
    type Item = &'a AdvertiseInstance;
    type IntoIter = std::slice::Iter<'a, AdvertiseInstance>;
    fn into_iter(self) -> Self::IntoIter {
        (&self.0).iter()
    }
}

#[derive(Debug)]
pub struct AdvDataScanResp(Box<[u8]>, Box<[u8]>);

impl AdvDataScanResp {
    pub fn new<A, S>(advdata: A, scanresp: S) -> Self
    where
        A: AsRef<[u8]>,
        S: AsRef<[u8]>,
    {
        let advdata = advdata.as_ref();
        let scanresp = scanresp.as_ref();
        Self(advdata.into(), scanresp.into())
    }
}

impl Pack for AdvDataScanResp {
    fn pack<W>(&self, write: &mut W) -> pack::Result<()>
    where
        W: io::Write,
    {
        (self.0.len() as u8).pack(write)?;
        (self.1.len() as u8).pack(write)?;
        self.0.pack(write)?;
        self.1.pack(write)?;
        Ok(())
    }
}

impl Unpack for AdvDataScanResp {
    fn unpack<R>(read: &mut R) -> pack::Result<Self>
    where
        R: io::Read,
    {
        let adv_data_len = u8::unpack(read)? as usize;
        let scan_resp_len = u8::unpack(read)? as usize;

        let mut read = <&mut R as io::Read>::take(read, adv_data_len as u64);
        let adv_data = <Box<[u8]>>::unpack(&mut read)?;
        let read = read.into_inner();
        if adv_data.len() != adv_data_len {
            return Err(io::Error::new(io::ErrorKind::Other, "unexpected length.").into());
        }

        let mut read = <&mut R as io::Read>::take(read, adv_data_len as u64);
        let scan_resp = <Box<[u8]>>::unpack(&mut read)?;
        if scan_resp.len() != scan_resp_len {
            return Err(io::Error::new(io::ErrorKind::Other, "unexpected length.").into());
        }

        Ok(Self(adv_data, scan_resp))
    }
}

impl<A, B> From<(A, B)> for AdvDataScanResp
where
    A: Into<Box<[u8]>>,
    B: Into<Box<[u8]>>,
{
    fn from((a, b): (A, B)) -> Self {
        Self(a.into(), b.into())
    }
}

bitflags! {
    #[derive(Pack, Unpack)]
    pub struct Phys: u32 {
        const Br1M1Slot = 1 << 0;
        const Br1M3Slot = 1 << 1;
        const Br1M5Slot = 1 << 2;
        const Edr2M1Slot = 1 << 3;
        const Edr2M3Slot = 1 << 4;
        const Edr2M5Slot = 1 << 5;
        const Edr3M1Slot = 1 << 6;
        const Edr3M3Slot = 1 << 7;
        const Edr3M5Slot = 1 << 8;
        const Le1MTx = 1 << 9;
        const Le1MRx = 1 << 10;
        const Le2MTx = 1 << 11;
        const Le2MRx = 1 << 12;
        const LeCodedTx = 1 << 13;
        const LeCodedRx = 1 << 14;
    }
}

#[derive(Debug, Pack, Unpack)]
#[pack(u8)]
pub enum BlockedKeyType {
    LinkKey = 0x00,
    LongTermKey = 0x01,
    IdentityResolvingKey = 0x02,
}

#[derive(Debug, Pack, Unpack, New, Getters)]
#[getset(get = "pub")]
pub struct BlockedKey {
    key_type: BlockedKeyType,
    value: [u8; 16],
}

bitflags! {
    #[derive(Pack, Unpack)]
    pub struct FeatureFlags: u32 {
        const FeatureActive = 1 << 0;
        const CauseChangeInSupportedSettings = 1 << 1;
    }
}

#[derive(Debug, Pack, Unpack)]
#[pack(u8)]
pub enum FeatureAction {
    Disable = 0x00,
    Enable = 0x01,
}

macro_rules! configuration_parameter {
    (
        $(#[$attrs:meta])*
        $vis:vis enum $name:ident: $typename:ident {
            $( $vname:ident($vty:ty) => $vcode:literal, )*
        }
    ) => {
        $(#[$attrs])*
        $vis enum $name {
            $( $vname($vty), )*
        }

        #[derive(Debug, Clone)]
        $vis enum $typename {
            $( $vname, )*
        }

        impl $name {
            #[allow(unreachable_patterns)]
            pub fn for_type(&self) -> $typename {
                match self {
                    $( Self::$vname(..) => $typename::$vname,)*
                    _ => unreachable!(),
                }
            }

            #[allow(unreachable_patterns)]
            pub fn value_as_u16(&self) -> Option<u16> {
                match self {
                    $( Self::$vname(v) => Some(*v),)* // FIXME
                    _ => unreachable!(),
                }
            }
        }

        impl Pack for $name {
            #[allow(unreachable_code, unused_variables, unreachable_patterns)]
            fn pack<W>(&self, write: &mut W) -> pack::Result<()> where W: io::Write {
                let (t, v) = match self {
                    $(Self::$vname(v) => ($vcode, VariableLengthBytes::from(*v)),)*
                    _ => unreachable!(),
                };
                <u16 as Pack>::pack(&t, write)?;
                <VariableLengthBytes<u8> as Pack>::pack(&v, write)?;
                Ok(())
            }
        }

        impl Unpack for $name {
            #[allow(unreachable_code, unused_variables)]
            fn unpack<R>(read: &mut R) -> pack::Result<Self> where R: io::Read {
                let t = u16::unpack(read)?;
                let v = VariableLengthBytes::<u8>::unpack(read)?;
                Ok(match t {
                    $($vcode => Self::$vname(v.try_into().map_err(|_| pack::Error::Io(io::Error::new(io::ErrorKind::Other, "unexpected data")))?),)*
                    t => return Err(io::Error::new(io::ErrorKind::Other, format!("unknown type {}", t)).into()),
                })
            }
        }
    }
}

configuration_parameter! {
    #[derive(Debug, Clone)]
    pub enum SystemConfigurationParameter: SystemConfigurationParameterType {
        BrEdrPageScanType(u16) => 0x0000,
        BrEdrPageScanInterval(u16) => 0x0001,
        BrEdrPageScanWindow(u16) => 0x0002,
        BrEdrInquiryScanType(u16) => 0x0003,
        BrEdrInquiryScanInterval(u16) => 0x0004,
        BrEdrInquiryScanWindow(u16) => 0x0005,
        BrEdrLinkSupervisionTimeout(u16) => 0x0006,
        BrEdrPageTimeout(u16) => 0x0007,
        BrEdrMinSniffInterval(u16) => 0x0008,
        BrEdrMaxSniffInterval(u16) => 0x0009,
        LEAdvertisementMinInterval(u16) => 0x000a,
        LEAdvertisementMaxInterval(u16) => 0x000b,
        LEMultiAdvertisementRotationInterval(u16) => 0x000c,
        LEScanningIntervalforautoconnect(u16) => 0x000d,
        LEScanningWindowforautoconnect(u16) => 0x000e,
        LEScanningIntervalforwakescenarios(u16) => 0x000f,
        LEScanningWindowforwakescenarios(u16) => 0x0010,
        LEScanningIntervalfordiscovery(u16) => 0x0011,
        LEScanningWindowfordiscovery(u16) => 0x0012,
        LEScanningIntervalforadvmonitoring(u16) => 0x0013,
        LEScanningWindowforadvmonitoring(u16) => 0x0014,
        LEScanningIntervalforconnect(u16) => 0x0015,
        LEScanningWindowforconnect(u16) => 0x0016,
        LEMinConnectionInterval(u16) => 0x0017,
        LEMaxConnectionInterval(u16) => 0x0018,
        LEConnectionLatency(u16) => 0x0019,
        LEConnectionSupervisionTimeout(u16) => 0x001a,
        LEAutoconnectTimeout(u16) => 0x001b,
    }
}

configuration_parameter! {
    #[derive(Debug, Clone)]
    pub enum RuntimeConfigurationParameter: RuntimeConfigurationParameterType {
    }
}

#[derive(Debug, Clone, IterNewtype)]
pub struct Remaining<T>(Vec<T>);

impl<T> Pack for Remaining<T>
where
    T: Pack,
{
    fn pack<W>(&self, write: &mut W) -> pack::Result<()>
    where
        W: io::Write,
    {
        for item in &self.0 {
            item.pack(write)?;
        }
        Ok(())
    }
}

impl<T> Unpack for Remaining<T>
where
    T: Unpack,
{
    fn unpack<R>(read: &mut R) -> pack::Result<Self>
    where
        R: io::Read,
    {
        let mut v = vec![];
        loop {
            match Unpack::unpack(read) {
                Ok(item) => v.push(item),
                Err(pack::Error::NoDataAvailable) => return Ok(Self(v)),
                Err(err) => return Err(err),
            }
        }
    }
}

bitflags! {
    #[derive(Pack, Unpack)]
    pub struct DeviceFlags: u32 {
        const RemoteWakeupEnabled = 1 << 0;
    }
}

bitflags! {
    #[derive(Pack, Unpack)]
    pub struct AdvertisementMonitorFeatures: u32 {
        const AdvertisementContentMonitoringBasedOnPatternsWithLogicalOr = 1 << 0;
    }
}

#[derive(Debug, Clone, Pack, Unpack, Newtype, New)]
pub struct AdvertisementMonitorHandle(u16);

#[derive(Debug, Pack, Unpack, Getters)]
#[getset(get = "pub")]
pub struct AdvertisementPattern {
    ad_type: u8,
    offset: u8,
    length: u8,
    value: [u8; 31],
}

impl AdvertisementPattern {
    pub fn new<V>(ad_type: u8, offset: u8, value: V) -> Self
    where
        V: AsRef<[u8]>,
    {
        let v = value.as_ref();
        let length = v.len() as u8;

        let mut value = [0; 31];
        value[0..v.len()].copy_from_slice(v);
        Self {
            ad_type,
            offset,
            length,
            value,
        }
    }
}

bitflags! {
    #[derive(Pack, Unpack)]
    pub struct DeviceConnectFlags: u32 {
        const ConfirmName = 1 << 0;
        const LegacyPairing = 1 << 1;
        const NotConnectable = 1 << 2;
    }
}

#[derive(Debug, Clone, Pack, Unpack)]
#[pack(u8)]
pub enum DeviceDisconnectReason {
    Unspecified = 0,
    ConnectionTimeout = 1,
    ConnectionTerminatedByLocalHost = 2,
    ConnectionTerminatedByRemoteHost = 3,
    ConnectionTerminatedDueToAuthenticationFailure = 4,
    ConnectionTerminatedByLocalHostForSuspend = 5,
}

#[derive(Debug, Clone, Pack, Unpack)]
#[pack(u8)]
pub enum ConfirmHint {
    Full = 0,
    Simple = 1,
}

#[derive(Debug, Clone, Pack, Unpack)]
#[pack(u8)]
pub enum SignatureResolvingKeyType {
    UnauthenticatedLocalCsrk = 0x00,
    UnauthenticatedRemoteCsrk = 0x01,
    AuthenticatedLocalCsrk = 0x02,
    AuthenticatedRemoteCsrk = 0x03,
}

#[derive(Debug, Clone, Pack, Unpack, Getters)]
pub struct SignatureResolvingKey {
    address: Address,
    address_type: AddressType,
    #[getset(get = "pub")]
    typ: SignatureResolvingKeyType,
    #[getset(get = "pub")]
    value: [u8; 16],
}

impl SignatureResolvingKey {
    pub fn new(addr: bdaddr::Address, typ: SignatureResolvingKeyType, value: [u8; 16]) -> Self {
        let (address, address_type) = split(addr);
        Self {
            address,
            address_type,
            typ,
            value,
        }
    }

    pub fn address(&self) -> bdaddr::Address {
        join(&self.address_type, &self.address)
    }
}

#[derive(Debug, Clone, Pack, Unpack)]
#[pack(u8)]
pub enum SuspendState {
    Running = 0,
    DisconnectedAndNotScanning = 1,
    PageScanAndOrPassiveScanning = 2,
}

#[derive(Debug, Clone, Pack, Unpack)]
#[pack(u8)]
pub enum WakeReason {
    ResumeFromNonBluetoothWakeSource = 0,
    WakeDueToUnexpectedEvent = 1,
    RemoteWakeDueToPeerDeviceConnection = 2,
}
