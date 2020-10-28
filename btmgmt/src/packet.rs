use std::collections::HashSet;
use std::convert::{TryFrom, TryInto};
use std::ffi::{CString, NulError};
use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

use bytes::{Buf, Bytes, BytesMut};
use derive_builder::Builder;
use derive_new::new as New;
use getset::Getters;
pub use uuid::Uuid;

use crate::pack::{self, Error as UnpackError, Pack, Unpack};

pub mod command;
pub mod event;

packable_enum! {
    #[derive(Debug, Clone)]
    pub enum ErrorCode: u8 {
        Success => 0x00,
        UnknownCommand => 0x01,
        NotConnected => 0x02,
        Failed => 0x03,
        ConnectFailed => 0x04,
        AuthenticationFailed => 0x05,
        NotPaired => 0x06,
        NoResources => 0x07,
        Timeout => 0x08,
        AlreadyConnected => 0x09,
        Busy => 0x0A,
        Rejected => 0x0B,
        NotSupported => 0x0C,
        InvalidParameters => 0x0D,
        Disconnected => 0x0E,
        NotPowered => 0x0F,
        Cancelled => 0x10,
        InvalidIndex => 0x11,
        RfKilled => 0x12,
        AlreadyPaired => 0x13,
        PermissionDenied => 0x14,
    }
}

impl ErrorCode {
    pub(crate) fn success(&self) -> bool {
        matches!(self, Self::Success)
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
        matches!(self, Self::NonController)
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
    fn pack(self, buf: &mut BytesMut) {
        u16::from(self).pack(buf)
    }
}

impl Unpack for ControllerIndex {
    fn unpack<B: Buf>(buf: &mut B) -> pack::Result<Self> {
        u16::unpack(buf).map(Into::into)
    }
}

#[derive(Debug)]
pub struct CommandsEvents {
    commands: Vec<command::CommandCode>,
    events: Vec<event::EventCode>,
}

impl CommandsEvents {
    fn commands(&self) -> &[command::CommandCode] {
        &self.commands
    }

    fn events(&self) -> &[event::EventCode] {
        &self.events
    }
}

impl Pack for CommandsEvents {
    fn pack(self, buf: &mut BytesMut) {
        (self.commands.len() as u16).pack(buf);
        (self.events.len() as u16).pack(buf);
        for item in self.commands {
            item.pack(buf);
        }
        for item in self.events {
            item.pack(buf);
        }
    }
}

impl Unpack for CommandsEvents {
    fn unpack<B: Buf>(buf: &mut B) -> Result<Self, UnpackError> {
        let num_commands = u16::unpack(buf)?;
        let num_events = u16::unpack(buf)?;
        let commands = (0..num_commands)
            .map(|_| Unpack::unpack(buf))
            .collect::<Result<_, _>>()?;
        let events = (0..num_events)
            .map(|_| Unpack::unpack(buf))
            .collect::<Result<_, _>>()?;
        Ok(Self { commands, events })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("unable to parse addrss")]
pub struct ParseAddressError;

packable_newtype! {
    #[derive(Debug, Clone)]
    pub struct Address([u8; 6]);
}

impl FromStr for Address {
    type Err = ParseAddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut b = s
            .splitn(6, ':')
            .map(|v| u8::from_str_radix(v, 16))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| ParseAddressError)?;
        if b.len() != 6 {
            return Err(ParseAddressError);
        }
        b.reverse();

        let mut v = [0; 6];
        v.copy_from_slice(&b);
        Ok(Address(v))
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.0.iter().rev();
        write!(f, "{:02X}", iter.next().unwrap())?;
        for v in iter {
            write!(f, ":")?;
            write!(f, "{:02X}", v)?;
        }
        Ok(())
    }
}

packable_enum! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum AddressType: u8 {
        BrEdr => 0,
        LePublic => 1,
        LeRandom => 2,
    }
}

impl Pack for HashSet<AddressType> {
    fn pack(self, buf: &mut BytesMut) {
        let mut v = 0u8;
        for item in self {
            v |= match item {
                AddressType::BrEdr => 1 << 0,
                AddressType::LePublic => 1 << 1,
                AddressType::LeRandom => 1 << 2,
            };
        }
        v.pack(buf);
    }
}

impl Unpack for HashSet<AddressType> {
    fn unpack<B: Buf>(buf: &mut B) -> Result<Self, UnpackError> {
        let v = u8::unpack(buf)?;
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

        Ok(r)
    }
}

packable_flags! {
    pub struct settings::Settings: u32 {
        const Powered = 0;
        const Connectable = 1;
        const FastConnectable = 2;
        const Discoverable = 3;
        const Bondable = 4;
        const LinkLevelSecurity = 5;
        const SecureSimplePairing = 6;
        const BasicRateEnhancedDataRate = 7;
        const HighSpeed = 8;
        const LowEnergy = 9;
        const Advertising = 10;
        const SecureConnections = 11;
        const DebugKeys = 12;
        const Privacy = 13;
        const ControllerConfiguration = 14;
        const StaticAddress = 15;
        const PhyConfiguration = 16;
        const WidebandSpeech = 17;
    }
}

packable_newtype! {
    #[derive(Debug, Clone)]
    pub struct ClassOfDevice([u8; 3]);
}

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

packable_newtype! {
    #[derive(Clone)]
    pub struct Name(Box<[u8; Self::LEN]>);
}

impl Name {
    const LEN: usize = 249;

    pub fn new<T: Into<Vec<u8>>>(t: T) -> Result<Self, NameError> {
        let s = CString::new(t)?;
        let b = s.as_bytes_with_nul();
        if b.len() > Self::LEN {
            return Err(LengthTooLong(Self::LEN, b.len()).into());
        }

        let mut v = [0; Self::LEN];
        (&mut v[..b.len()]).copy_from_slice(b);
        Ok(Self(Box::new(v)))
    }

    pub fn to_string_lossy(&self) -> String {
        let b = self.0.split(|b| b == &0).next().unwrap_or(b"");
        CString::new(b).unwrap().to_string_lossy().to_string()
    }
}

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b = self.0.split(|b| b == &0).next().unwrap_or(b"");
        CString::new(b).unwrap().fmt(f)
    }
}

impl FromStr for Name {
    type Err = NameError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

packable_newtype! {
    #[derive(Clone)]
    pub struct ShortName([u8; Self::LEN]);
}

impl ShortName {
    const LEN: usize = 11;

    pub fn new<T: Into<Vec<u8>>>(t: T) -> Result<Self, NameError> {
        let s = CString::new(t)?;
        let b = s.as_bytes_with_nul();
        if b.len() > Self::LEN {
            return Err(LengthTooLong(Self::LEN, b.len()).into());
        }

        let mut v = [0; Self::LEN];
        (&mut v[..b.len()]).copy_from_slice(b);
        Ok(Self(v))
    }

    pub fn to_string_lossy(&self) -> String {
        let b = self.0.split(|b| b == &0).next().unwrap_or(b"");
        CString::new(b).unwrap().to_string_lossy().to_string()
    }
}

impl fmt::Debug for ShortName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b = self.0.split(|b| b == &0).next().unwrap_or(b"");
        CString::new(b).unwrap().fmt(f)
    }
}

impl FromStr for ShortName {
    type Err = NameError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

packable_enum! {
    #[derive(Debug)]
    pub enum Discoverable: u8 {
        Disable => 0x00,
        General => 0x01,
        Limited => 0x02,
    }
}

impl Pack for Uuid {
    fn pack(self, buf: &mut BytesMut) {
        self.to_u128_le().pack(buf);
    }
}

impl Unpack for Uuid {
    fn unpack<B: Buf>(buf: &mut B) -> Result<Self, UnpackError> {
        Ok(Self::from_u128_le(Unpack::unpack(buf)?))
    }
}

packable_enum! {
    #[derive(Debug, Clone)]
    pub enum LinkKeyType: u8 {
        Combinationkey => 0x00,
        LocalUnitkey => 0x01,
        RemoteUnitkey => 0x02,
        DebugCombinationkey => 0x03,
        UnauthenticatedCombinationkeyfromP192 => 0x04,
        AuthenticatedCombinationkeyfromP192 => 0x05,
        ChangedCombinationkey => 0x06,
        UnauthenticatedCombinationkeyfromP256 => 0x07,
        AuthenticatedCombinationkeyfromP256 => 0x08,
    }
}

packable_struct! {
    #[derive(Debug, Clone, New, Getters)]
    #[get="pub"]
    pub struct LinkKey {
        address: Address,
        address_type: AddressType,
        key_type: LinkKeyType,
        value: [u8; 16],
        pin_length: u8,
    }
}

packable_enum! {
    #[derive(Debug, Clone)]
    pub enum LongTermKeyType: u8 {
        UnauthenticatedKey => 0x00,
        AuthenticatedKey => 0x01,
    }
}

packable_struct! {
    #[derive(Debug, Clone, Builder, Getters)]
    #[get="pub"]
    pub struct LongTermKey {
        address: Address,
        address_type: AddressType,
        key_type: LongTermKeyType,
        master: bool,
        encryption_size: u8,
        encryption_diversifier: u16,
        random_number: [u8; 8],
        value: [u8; 16],
    }
}

packable_struct! {
    #[derive(Debug, Clone, New, Getters)]
    #[get="pub"]
    pub struct IdentityResolvingKey {
        address: Address,
        address_type: AddressType,
        value: [u8; 16],
    }
}

packable_enum! {
    #[derive(Debug)]
    pub enum IoCapability: u8 {
        DisplayOnly => 0,
        DisplayYesNo => 1,
        KeyboardOnly => 2,
        NoInputNoOutput => 3,
        KeyboardDisplay => 4,
    }
}

packable_enum! {
    #[derive(Debug)]
    pub enum DeviceIdSource: u16 {
        DisableDeviceId => 0x0000,
        BluetoothSig => 0x0001,
        UsbImplementersForum => 0x0002,
    }
}

packable_enum! {
    #[derive(Debug)]
    pub enum Advertising: u8 {
        Disable => 0x00,
        Enable => 0x01,
        Connectable => 0x02,
    }
}

packable_enum! {
    #[derive(Debug)]
    pub enum SecureConnections: u8 {
        Disable => 0x00,
        Enable => 0x01,
        Only => 0x02,
    }
}

packable_enum! {
    #[derive(Debug)]
    pub enum DebugKeys: u8 {
        Disable => 0x00,
        Enable => 0x01,
        ForEach => 0x02,
    }
}

packable_enum! {
    #[derive(Debug)]
    pub enum Privacy: u8 {
        Disable => 0x00,
        Enable => 0x01,
        Limited => 0x02,
    }
}

packable_enum! {
    #[derive(Debug, Clone)]
    pub enum Action: u8 {
        Background => 0,
        Allow => 1,
        AutoConnect => 2,
    }
}

packable_struct! {
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub struct ConnectionParameter {
        address: Address,
        address_type: AddressType,
        min_connection_interval: u16,
        max_connection_interval: u16,
        connection_latency: u16,
        supervision_timeout: u16,
    }
}

packable_flags! {
    pub struct controller_configuration_option::ControllerConfigurationOption: u32 {
        const ExternalConfiguration = 0;
        const BluetoothPublicAddressConfiguration = 1;
    }
}

#[derive(Debug, Clone)]
pub struct VariableLengthBytes<L = u16>(Bytes, PhantomData<L>);

impl Pack for VariableLengthBytes<u16> {
    fn pack(self, buf: &mut BytesMut) {
        (self.0.len() as u16).pack(buf);
        self.0.pack(buf);
    }
}

impl Unpack for VariableLengthBytes<u16> {
    fn unpack<B: Buf>(buf: &mut B) -> Result<Self, UnpackError> {
        let len = u16::unpack(buf)? as usize;
        if buf.remaining() < len {
            return Err(UnpackError::Overflow);
        }

        Ok(Self(buf.copy_to_bytes(len), PhantomData))
    }
}

impl Pack for VariableLengthBytes<u8> {
    fn pack(self, buf: &mut BytesMut) {
        (self.0.len() as u8).pack(buf);
        self.0.pack(buf);
    }
}

impl Unpack for VariableLengthBytes<u8> {
    fn unpack<B: Buf>(buf: &mut B) -> Result<Self, UnpackError> {
        let len = u8::unpack(buf)? as usize;
        if buf.remaining() < len {
            return Err(UnpackError::Overflow);
        }

        Ok(Self(buf.copy_to_bytes(len), PhantomData))
    }
}

impl<L> From<u16> for VariableLengthBytes<L> {
    fn from(v: u16) -> Self {
        Self(v.to_le_bytes().to_vec().into(), PhantomData)
    }
}

impl<L> TryFrom<VariableLengthBytes<L>> for u16 {
    type Error = (); // FIXME
    fn try_from(value: VariableLengthBytes<L>) -> Result<Self, Self::Error> {
        match *value.0.as_ref() {
            [i1, i2] => Ok(u16::from_le_bytes([i1, i2])),
            _ => Err(()),
        }
    }
}

packable_enum! {
    #[derive(Debug, Clone)]
    pub enum ControllerType: u8 {
        PrimaryController => 0x00,
        UnconfiguredController => 0x01,
        AlternateMacPhyController => 0x02,
    }
}

packable_enum! {
    #[derive(Debug, Clone)]
    pub enum ControllerBus: u8 {
        Virtual => 0x00,
        Usb => 0x01,
        Pcmcia => 0x02,
        Uart => 0x03,
        Rs232 => 0x04,
        Pci => 0x05,
        Sdio => 0x06,
        Spi => 0x07,
        I2c => 0x08,
        Smd => 0x09,
        Virtio => 0x0A,
    }
}

packable_flags! {
    pub struct advertising_flag::AdvertisingFlag: u32 {
        const SwitchIntoConnectableMode = 0;
        const AdvertiseAsDiscoverable = 1;
        const AdvertiseAsLimitedDiscoverable = 2;
        const AddFlagsFieldToAdvData = 3;
        const AddTxPowerFieldToAdvData = 4;
        const AddAppearanceFieldToScanResp = 5;
        const AddLocalNameInScanResp = 6;
        const SecondaryChannelWithLe1M = 7;
        const SecondaryChannelWithLe2M = 8;
        const SecondaryChannelWithLeCoded = 9;
    }
}

packable_newtype! {
    #[derive(Debug, Clone)]
    pub struct AdvertiseInstance(u8);
}

impl From<u8> for AdvertiseInstance {
    fn from(v: u8) -> Self {
        Self(v)
    }
}

#[derive(Debug)]
pub struct AdvertiseInstances(Vec<AdvertiseInstance>);

impl Pack for AdvertiseInstances {
    fn pack(self, buf: &mut BytesMut) {
        (self.0.len() as u8).pack(buf);
        for item in self.0 {
            item.pack(buf);
        }
    }
}

impl Unpack for AdvertiseInstances {
    fn unpack<B: Buf>(buf: &mut B) -> Result<Self, UnpackError> {
        let len = u8::unpack(buf)? as usize;
        let v = (0..len)
            .map(|_| Unpack::unpack(buf))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(v))
    }
}

impl std::iter::IntoIterator for AdvertiseInstances {
    type Item = AdvertiseInstance;
    type IntoIter = std::vec::IntoIter<AdvertiseInstance>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
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
pub struct AdvDataScanResp(Bytes, Bytes);

impl Pack for AdvDataScanResp {
    fn pack(self, buf: &mut BytesMut) {
        (self.0.len() as u8).pack(buf);
        (self.1.len() as u8).pack(buf);
        buf.extend_from_slice(&self.0);
        buf.extend_from_slice(&self.1);
    }
}

impl Unpack for AdvDataScanResp {
    fn unpack<B: Buf>(buf: &mut B) -> Result<Self, UnpackError> {
        let adv_data_len = u8::unpack(buf)? as usize;
        let scan_resp_len = u8::unpack(buf)? as usize;
        if buf.remaining() < adv_data_len + scan_resp_len {
            return Err(UnpackError::Overflow);
        }
        Ok(Self(
            buf.copy_to_bytes(adv_data_len),
            buf.copy_to_bytes(scan_resp_len),
        ))
    }
}

impl<A, B> From<(A, B)> for AdvDataScanResp
where
    A: Into<Bytes>,
    B: Into<Bytes>,
{
    fn from((a, b): (A, B)) -> Self {
        Self(a.into(), b.into())
    }
}

packable_flags! {
    pub struct phys::Phys: u32 {
        const Br1M1Slot = 0;
        const Br1M3Slot = 1;
        const Br1M5Slot = 2;
        const Edr2M1Slot = 3;
        const Edr2M3Slot = 4;
        const Edr2M5Slot = 5;
        const Edr3M1Slot = 6;
        const Edr3M3Slot = 7;
        const Edr3M5Slot = 8;
        const Le1MTx = 9;
        const Le1MRx = 10;
        const Le2MTx = 11;
        const Le2MRx = 12;
        const LeCodedTx = 13;
        const LeCodedRx = 14;
    }
}

packable_enum! {
    #[derive(Debug)]
    pub enum BlockedKeyType: u8 {
        LinkKey => 0x00,
        LongTermKey => 0x01,
        IdentityResolvingKey => 0x02,
    }
}

packable_struct! {
    #[derive(Debug, New, Getters)]
    #[get="pub"]
    pub struct BlockedKey {
        key_type: BlockedKeyType,
        value: [u8; 16],
    }
}

packable_flags! {
    pub struct feature_flags::FeatureFlags: u32 {
        const FeatureActive = 0;
        const CauseChangeInSupportedSettings = 1;
    }
}

packable_enum! {
    #[derive(Debug)]
    pub enum FeatureAction: u8 {
        Disable => 0x00,
        Enable => 0x01,
    }
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
            #[allow(unreachable_code, unused_variables)]
            fn pack(self, buf: &mut BytesMut) {
                let (t, v) = match self {
                    $(Self::$vname(v) => ($vcode, VariableLengthBytes::from(v)),)*
                };
                <u16 as Pack>::pack(t, buf);
                <VariableLengthBytes<u8> as Pack>::pack(v, buf);
            }
        }

        impl Unpack for $name {
            #[allow(unreachable_code, unused_variables)]
            fn unpack<B: Buf>(buf: &mut B) -> Result<Self, UnpackError> {
                let t = u16::unpack(buf)?;
                let v = VariableLengthBytes::<u8>::unpack(buf)?;
                Ok(match t {
                    $($vcode => Self::$vname(v.try_into().map_err(|_| UnpackError::Unexpected(format!("unexpected data")))?),)*
                    t => return Err(UnpackError::Unexpected(format!("unknown type {}", t))),
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

#[derive(Debug, Clone)]
pub struct Remaining<T>(Vec<T>);

impl<T> Pack for Remaining<T>
where
    T: Pack,
{
    fn pack(self, buf: &mut BytesMut) {
        for item in self.0 {
            item.pack(buf);
        }
    }
}

impl<T> Unpack for Remaining<T>
where
    T: Unpack,
{
    fn unpack<B: Buf>(buf: &mut B) -> Result<Self, UnpackError> {
        let mut v = vec![];
        loop {
            if !buf.has_remaining() {
                return Ok(Self(v));
            }

            v.push(Unpack::unpack(buf)?);
        }
    }
}

packable_flags! {
    pub struct device_flags::DeviceFlags: u32 {
        const RemoteWakeupEnabled = 0;
    }
}

packable_flags! {
    pub struct advertisement_monitor_features::AdvertisementMonitorFeatures: u32 {
        const AdvertisementContentMonitoringBasedOnPatternsWithLogicalOr = 0;
    }
}

packable_newtype! {
    #[derive(Debug, Clone)]
    pub struct AdvertisementMonitorHandle(u16);
}

impl From<u16> for AdvertisementMonitorHandle {
    fn from(v: u16) -> Self {
        Self(v)
    }
}

packable_struct! {
    #[derive(Debug, Getters)]
    pub struct AdvertisementPattern {
        #[get="pub"]
        ad_type: u8,
        #[get="pub"]
        offset: u8,
        length: u8,
        value: [u8; 31],
    }
}

impl AdvertisementPattern {
    pub fn new(ad_type: u8, offset: u8, value: &[u8]) -> Self {
        let length = value.len() as u8;
        let mut v = [0; 31];
        v[0..value.len()].copy_from_slice(value);
        Self {
            ad_type,
            offset,
            length,
            value: v,
        }
    }
}

packable_flags! {
    pub struct device_connect_flags::DeviceConnectFlags: u32 {
        const ConfirmName = 0;
        const LegacyPairing = 1;
        const NotConnectable = 2;
    }
}

packable_enum! {
    #[derive(Debug, Clone)]
    pub enum DeviceDisconnectReason: u8 {
        Unspecified => 0,
        ConnectionTimeout => 1,
        ConnectionTerminatedByLocalHost => 2,
        ConnectionTerminatedByRemoteHost => 3,
        ConnectionTerminatedDueToAuthenticationFailure => 4,
        ConnectionTerminatedByLocalHostForSuspend => 5,
    }
}

packable_enum! {
    #[derive(Debug, Clone)]
    pub enum ConfirmHint: u8 {
        Full => 0,
        Simple => 1,
    }
}

packable_enum! {
    #[derive(Debug, Clone)]
    pub enum SignatureResolvingKeyType: u8 {
        UnauthenticatedLocalCsrk => 0x00,
        UnauthenticatedRemoteCsrk => 0x01,
        AuthenticatedLocalCsrk => 0x02,
        AuthenticatedRemoteCsrk => 0x03,
    }
}

packable_struct! {
    #[derive(Debug, Clone, New, Getters)]
    #[get="pub"]
    pub struct SignatureResolvingKey {
        address: Address,
        address_type: AddressType,
        typ: SignatureResolvingKeyType,
        value: [u8; 16],
    }
}

packable_enum! {
    #[derive(Debug, Clone)]
    pub enum SuspendState: u8 {
        Running => 0,
        DisconnectedAndNotScanning => 1,
        PageScanAndOrPassiveScanning => 2,
    }
}

packable_enum! {
    #[derive(Debug, Clone)]
    pub enum WakeReason: u8 {
        ResumeFromNonBluetoothWakeSource => 0,
        WakeDueToUnexpectedEvent => 1,
        RemoteWakeDueToPeerDeviceConnection => 2,
    }
}
