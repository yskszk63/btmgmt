//! mgmt API commands
use std::io;

use derive_new::new as New;
use getset::Getters;

use btmgmt_packet_helper::commands;
use btmgmt_packet_helper::pack::{Pack, Unpack};

use super::*;
pub use imp::*;

// Management API Command
#[commands(name = Command, trait = CommandRequest, codes = CommandCode)]
mod imp {
    use super::*;

    /// Read Management Version Information Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Default, Pack)]
    #[command(code = 0x0001, reply = ReadManagementVersionInformationReply)]
    pub struct ReadManagementVersionInformation;

    /// Reply for [`ReadManagementVersionInformation`]
    #[derive(Debug, Unpack, Getters)]
    #[getset(get = "pub")]
    pub struct ReadManagementVersionInformationReply {
        version: u8,
        revision: u16,
    }

    /// Read Management Supported Commands Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Default, Pack)]
    #[command(code = 0x0002, reply = ReadManagementSupportedCommandsReply)]
    pub struct ReadManagementSupportedCommands;

    /// Reply for [`ReadManagementSupportedCommands`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct ReadManagementSupportedCommandsReply(super::CommandsEvents);

    /// Read Controller Index List Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Default, Pack)]
    #[command(code = 0x0003, reply = ReadControllerIndexListReply)]
    pub struct ReadControllerIndexList;

    /// Reply for [`ReadControllerIndexList`]
    #[derive(Debug, Unpack, IterNewtype)]
    pub struct ReadControllerIndexListReply(Vec<ControllerIndex>);

    /// Read Controller Information Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Default, Pack)]
    #[command(code = 0x0004, reply = ReadControllerInformationReply)]
    pub struct ReadControllerInformation;

    /// Reply for [`ReadControllerInformation`]
    #[derive(Debug, Unpack, Getters)]
    pub struct ReadControllerInformationReply {
        address: super::WrappedAddress,
        #[getset(get = "pub")]
        bluetooth_version: u8,
        #[getset(get = "pub")]
        manufacturer: u16,
        #[getset(get = "pub")]
        supported_settings: super::Settings,
        #[getset(get = "pub")]
        current_settings: super::Settings,
        #[getset(get = "pub")]
        class_of_device: super::ClassOfDevice,
        #[getset(get = "pub")]
        name: super::Name,
        #[getset(get = "pub")]
        short_name: super::ShortName,
    }

    impl ReadControllerInformationReply {
        pub fn address(&self) -> &BdAddr {
            &self.address.0
        }
    }

    /// Set Powered Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype, New)]
    #[command(code = 0x0005, reply = SetPoweredReply)]
    pub struct SetPowered(bool);

    /// Reply for [`SetPowered`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetPoweredReply(super::Settings);

    /// Set Discoverable Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, New)]
    #[command(code = 0x0006, reply = SetDiscoverableReply)]
    pub struct SetDiscoverable {
        discoverable: super::Discoverable,
        timeout: u16,
    }

    /// Reply for [`SetDiscoverable`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetDiscoverableReply(super::Settings);

    /// Set Connectable Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype, New)]
    #[command(code = 0x0007, reply = SetConnectableReply)]
    pub struct SetConnectable(bool);

    /// Reply for [`SetConnectable`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetConnectableReply(super::Settings);

    /// Set Fast Connectable Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype, New)]
    #[command(code = 0x0008, reply = SetFastConnectableReply)]
    pub struct SetFastConnectable(bool);

    /// Reply for [`SetFastConnectable`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetFastConnectableReply(super::Settings);

    /// Set Bondable Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype, New)]
    #[command(code = 0x0009, reply = SetBondableReply)]
    pub struct SetBondable(bool);

    /// Reply for [`SetBondable`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetBondableReply(super::Settings);

    /// Set Link Security Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype, New)]
    #[command(code = 0x000A, reply = SetLinkSecurityReply)]
    pub struct SetLinkSecurity(bool);

    /// Reply for [`SetLinkSecurity`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetLinkSecurityReply(super::Settings);

    /// Set Secure Simple Pairing Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype, New)]
    #[command(code = 0x000B, reply = SetSecureSimplePairingReply)]
    pub struct SetSecureSimplePairing(bool);

    /// Reply for [`SetSecureSimplePairing`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetSecureSimplePairingReply(super::Settings);

    /// Set High Speed Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype, New)]
    #[command(code = 0x000C, reply = SetHighSpeedReply)]
    pub struct SetHighSpeed(bool);

    /// Reply for [`SetHighSpeed`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetHighSpeedReply(super::Settings);

    /// Set Low Energy Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype, New)]
    #[command(code = 0x000D, reply = SetLowEnergyReply)]
    pub struct SetLowEnergy(bool);

    /// Reply for [`SetLowEnergy`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetLowEnergyReply(super::Settings);

    /// Set Device Class Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, New)]
    #[command(code = 0x000E, reply = SetDeviceClassReply)]
    pub struct SetDeviceClass {
        major_class: u8,
        minor_class: u8,
    }

    /// Reply for [`SetDeviceClass`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetDeviceClassReply(super::ClassOfDevice);

    /// Set Local Name Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, New)]
    #[command(code = 0x000F, reply = SetLocalNameReply)]
    pub struct SetLocalName {
        name: super::Name,
        short_name: super::ShortName,
    }

    /// Reply for [`SetLocalName`]
    #[derive(Debug, Unpack, Getters)]
    #[getset(get = "pub")]
    pub struct SetLocalNameReply {
        name: super::Name,
        short_name: super::ShortName,
    }

    /// Add UUID Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, New)]
    #[command(code = 0x0010, reply = AddUuidReply)]
    pub struct AddUuid {
        uuid: super::Uuid,
        svc_hint: u8,
    }

    /// Reply for [`AddUuid`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct AddUuidReply(super::ClassOfDevice);

    /// Remove UUID Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype, New)]
    #[command(code = 0x0011, reply = RemoveUuidReply)]
    pub struct RemoveUuid(super::Uuid);

    /// Reply for [`RemoveUuid`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct RemoveUuidReply(super::ClassOfDevice);

    /// Load Link Keys Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, New)]
    #[command(code = 0x0012, reply = LoadLinkKeysReply)]
    pub struct LoadLinkKeys {
        debug_keys: bool,
        keys: Vec<super::LinkKey>,
    }

    /// Reply for [`LoadLinkKeys`]
    #[derive(Debug, Unpack)]
    pub struct LoadLinkKeysReply;

    /// Load Long Term Keys Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, IterNewtype)]
    #[command(code = 0x0013, reply = LoadLongTermKeyReply)]
    pub struct LoadLongTermKey(Vec<super::LongTermKey>);

    /// Reply for [`LoadLongTermKey`]
    #[derive(Debug, Unpack)]
    pub struct LoadLongTermKeyReply;

    /// Disconnect Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0014, reply = DisconnectReply)]
    pub struct Disconnect {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl Disconnect {
        pub fn new(addr: Address) -> Self {
            let (address, address_type) = super::split(addr);
            Self {
                address,
                address_type,
            }
        }
    }

    /// Reply for [`Disconnect`]
    #[derive(Debug, Unpack)]
    pub struct DisconnectReply {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl DisconnectReply {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// Get Connections Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0015, reply = GetConnectionsReply)]
    pub struct GetConnections;

    /// Reply for [`GetConnections`]
    #[derive(Debug, IterNewtype)]
    pub struct GetConnectionsReply(Vec<Address>);

    impl Unpack for GetConnectionsReply {
        fn unpack<R>(read: &mut R) -> crate::pack::Result<Self>
        where
            R: io::Read,
        {
            let inner = Vec::<(super::WrappedAddress, super::AddressType)>::unpack(read)?;
            let inner = inner
                .into_iter()
                .map(|(addr, ty)| join(&ty, &addr))
                .collect::<Vec<_>>();
            Ok(Self(inner))
        }
    }

    /// PIN Code Reply Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0016, reply = PinCodeReplyReply)]
    pub struct PinCodeReply {
        address: super::WrappedAddress,
        address_type: super::AddressType,
        pin_length: u8,
        pin_code: [u8; 16],
    }

    impl PinCodeReply {
        pub fn new(addr: Address, pin_length: u8, pin_code: [u8; 16]) -> Self {
            let (address, address_type) = split(addr);
            Self {
                address,
                address_type,
                pin_length,
                pin_code,
            }
        }
    }

    /// Reply for [`PinCodeReply`]
    #[derive(Debug, Unpack)]
    pub struct PinCodeReplyReply {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl PinCodeReplyReply {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// PIN Code Negative Reply Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0017, reply = PinCodeNegativeReplyReply)]
    pub struct PinCodeNegativeReply {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl PinCodeNegativeReply {
        pub fn new(addr: Address) -> Self {
            let (address, address_type) = split(addr);
            Self {
                address,
                address_type,
            }
        }
    }

    /// Reply for [`PinCodeNegativeReply`]
    #[derive(Debug, Unpack)]
    pub struct PinCodeNegativeReplyReply {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl PinCodeNegativeReplyReply {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// Set IO Capability Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype, New)]
    #[command(code = 0x0018, reply = SetIoCapabilityReply)]
    pub struct SetIoCapability(super::IoCapability);

    /// Reply for [`SetIoCapability`]
    #[derive(Debug, Unpack)]
    pub struct SetIoCapabilityReply;

    /// Read Management Version Information Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0019, reply = PairDeviceReply)]
    pub struct PairDevice {
        address: super::WrappedAddress,
        address_type: super::AddressType,
        io_capability: super::IoCapability,
    }

    impl PairDevice {
        pub fn new(addr: Address, io_capability: super::IoCapability) -> Self {
            let (address, address_type) = split(addr);
            Self {
                address,
                address_type,
                io_capability,
            }
        }
    }

    /// Reply for [`PairDevice`]
    #[derive(Debug, Unpack)]
    pub struct PairDeviceReply {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl PairDeviceReply {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// Cancel Pair Device Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x001A, reply = CancelPairDeviceReply)]
    pub struct CancelPairDevice {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl CancelPairDevice {
        pub fn new(addr: Address) -> Self {
            let (address, address_type) = split(addr);
            Self {
                address,
                address_type,
            }
        }
    }

    /// Reply for [`CancelPairDevice`]
    #[derive(Debug, Unpack)]
    pub struct CancelPairDeviceReply {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl CancelPairDeviceReply {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// Unpair Device Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x001B, reply = UnpairDeviceReply)]
    pub struct UnpairDevice {
        address: super::WrappedAddress,
        address_type: super::AddressType,
        disconnect: bool,
    }

    impl UnpairDevice {
        pub fn new(addr: Address, disconnect: bool) -> Self {
            let (address, address_type) = split(addr);
            Self {
                address,
                address_type,
                disconnect,
            }
        }
    }

    /// Reply for [`UnpairDevice`]
    #[derive(Debug, Unpack)]
    pub struct UnpairDeviceReply {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl UnpairDeviceReply {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// User Confirmation Reply Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x001C, reply = UserConfirmationReplyReply)]
    pub struct UserConfirmationReply {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl UserConfirmationReply {
        pub fn new(addr: Address) -> Self {
            let (address, address_type) = split(addr);
            Self {
                address,
                address_type,
            }
        }
    }

    /// Reply for [`UserConfirmationReply`]
    #[derive(Debug, Unpack)]
    pub struct UserConfirmationReplyReply {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl UserConfirmationReplyReply {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// User Confirmation Negative Reply Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x001D, reply = UserConfirmationNegativeReplyReply)]
    pub struct UserConfirmationNegativeReply {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl UserConfirmationNegativeReply {
        pub fn new(addr: Address) -> Self {
            let (address, address_type) = split(addr);
            Self {
                address,
                address_type,
            }
        }
    }

    /// Reply for [`UserConfirmationNegativeReply`]
    #[derive(Debug, Unpack)]
    pub struct UserConfirmationNegativeReplyReply {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl UserConfirmationNegativeReplyReply {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// User Passkey Reply Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x001E, reply = UserPasskeyReplyReply)]
    pub struct UserPasskeyReply {
        address: super::WrappedAddress,
        address_type: super::AddressType,
        passkey: u32,
    }

    impl UserPasskeyReply {
        pub fn new(addr: Address, passkey: u32) -> Self {
            let (address, address_type) = split(addr);
            Self {
                address,
                address_type,
                passkey,
            }
        }
    }

    /// Reply for [`UserPasskeyReply`]
    #[derive(Debug, Unpack)]
    pub struct UserPasskeyReplyReply {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl UserPasskeyReplyReply {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// User Passkey Negative Reply Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x001F, reply = UserPasskeyNegativeReplyReply)]
    pub struct UserPasskeyNegativeReply {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl UserPasskeyNegativeReply {
        pub fn new(addr: Address) -> Self {
            let (address, address_type) = split(addr);
            Self {
                address,
                address_type,
            }
        }
    }

    /// Reply for [`UserPasskeyNegativeReply`]
    #[derive(Debug, Unpack)]
    pub struct UserPasskeyNegativeReplyReply {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl UserPasskeyNegativeReplyReply {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// Read Local Out Of Band Data Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0020, reply = ReadLocalOutOfBandDataReply)]
    pub struct ReadLocalOutOfBandData;

    /// Reply for [`ReadLocalOutOfBandData`]
    #[derive(Debug, Unpack, Getters)]
    #[getset(get = "pub")]
    pub struct ReadLocalOutOfBandDataReply {
        hash192: [u8; 16],
        randomizer192: [u8; 16],
        hash256: Option<[u8; 16]>,
        randomizer256: Option<[u8; 16]>,
    }

    /// Add Remote Out Of Band Data Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0021, reply = AddRemoteOutOfBandDataReply)]
    pub struct AddRemoteOutOfBandData {
        address: super::WrappedAddress,
        address_type: super::AddressType,
        hash192: [u8; 16],
        randomizer192: [u8; 16],
        hash256: Option<[u8; 16]>,
        randomizer256: Option<[u8; 16]>,
    }

    impl AddRemoteOutOfBandData {
        pub fn new(
            addr: Address,
            hash192: [u8; 16],
            randomizer192: [u8; 16],
            hash256: Option<[u8; 16]>,
            randomizer256: Option<[u8; 16]>,
        ) -> Self {
            let (address, address_type) = split(addr);
            Self {
                address,
                address_type,
                hash192,
                randomizer192,
                hash256,
                randomizer256,
            }
        }
    }

    /// Reply for [`AddRemoteOutOfBandData`]
    #[derive(Debug, Unpack)]
    pub struct AddRemoteOutOfBandDataReply {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl AddRemoteOutOfBandDataReply {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// Remove Remote Out Of Band Data Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0022, reply = RemoveRemoteOutOfBandDataReply)]
    pub struct RemoveRemoteOutOfBandData {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl RemoveRemoteOutOfBandData {
        pub fn new(addr: Address) -> Self {
            let (address, address_type) = split(addr);
            Self {
                address,
                address_type,
            }
        }
    }

    /// Reply for [`RemoveRemoteOutOfBandData`]
    #[derive(Debug, Unpack)]
    pub struct RemoveRemoteOutOfBandDataReply {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl RemoveRemoteOutOfBandDataReply {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// Start Discovery Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype, New)]
    #[command(code = 0x0023, reply = StartDiscoveryReply)]
    pub struct StartDiscovery(super::AddressTypes);

    /// Reply for [`StartDiscovery`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct StartDiscoveryReply(super::AddressTypes);

    /// Stop Discovery Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype, New)]
    #[command(code = 0x0024, reply = StopDiscoveryReply)]
    pub struct StopDiscovery(super::AddressTypes);

    /// Reply for [`StopDiscovery`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct StopDiscoveryReply(super::AddressTypes);

    /// Confirm Name Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0025, reply = ConfirmNameReply)]
    pub struct ConfirmName {
        address: super::WrappedAddress,
        address_type: super::AddressType,
        name_known: bool,
    }

    impl ConfirmName {
        pub fn new(addr: Address, name_known: bool) -> Self {
            let (address, address_type) = split(addr);
            Self {
                address,
                address_type,
                name_known,
            }
        }
    }

    /// Reply for [`ConfirmName`]
    #[derive(Debug, Unpack)]
    pub struct ConfirmNameReply {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl ConfirmNameReply {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// Block Device Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0026, reply = BlockDeviceReply)]
    pub struct BlockDevice {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl BlockDevice {
        pub fn new(addr: Address) -> Self {
            let (address, address_type) = split(addr);
            Self {
                address,
                address_type,
            }
        }
    }

    /// Reply for [`BlockDevice`]
    #[derive(Debug, Unpack)]
    pub struct BlockDeviceReply {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl BlockDeviceReply {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// Unblock Device Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0027, reply = UnblockDeviceReply)]
    pub struct UnblockDevice {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl UnblockDevice {
        pub fn new(addr: Address) -> Self {
            let (address, address_type) = split(addr);
            Self {
                address,
                address_type,
            }
        }
    }

    /// Reply for [`UnblockDevice`]
    #[derive(Debug, Unpack)]
    pub struct UnblockDeviceReply {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl UnblockDeviceReply {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// Set Device ID Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0028, reply = SetDeviceIdReply)]
    pub struct SetDeviceId {
        pub source: super::DeviceIdSource,
        pub vendor: u16,
        pub product: u16,
        pub version: u16,
    }

    /// Reply for [`SetDeviceId`]
    #[derive(Debug, Unpack)]
    pub struct SetDeviceIdReply;

    /// Set Advertising Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype, New)]
    #[command(code = 0x0029, reply = SetAdvertisingReply)]
    pub struct SetAdvertising(super::Advertising);

    /// Reply for [`SetAdvertising`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetAdvertisingReply(super::Settings);

    /// Set BR/EDR Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype, New)]
    #[command(code = 0x002A, reply = SetBrEdrReply)]
    pub struct SetBrEdr(bool);

    /// Reply for [`SetBrEdr`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetBrEdrReply(super::Settings);

    /// Read Management Version Information Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x002B, reply = SetStaticAddressReply)]
    pub struct SetStaticAddress(super::WrappedAddress);

    impl SetStaticAddress {
        pub fn new(addr: bdaddr::StaticDeviceAddress) -> Self {
            let addr = bdaddr::RandomDeviceAddress::from(addr);
            let addr = Address::from(addr).into_bd_addr();
            Self(super::WrappedAddress(addr))
        }
    }

    /// Reply for [`SetStaticAddress`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetStaticAddressReply(super::Settings);

    /// Set Scan Parameters Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, New)]
    #[command(code = 0x002C, reply = SetScanParametersReply)]
    pub struct SetScanParameters {
        interval: u16,
        window: u16,
    }

    /// Reply for [`SetScanParameters`]
    #[derive(Debug, Unpack)]
    pub struct SetScanParametersReply;

    /// Set Secure Connections Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype, New)]
    #[command(code = 0x002D, reply = SetSecureConnectionsReply)]
    pub struct SetSecureConnections(super::SecureConnections);

    /// Reply for [`SetSecureConnections`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetSecureConnectionsReply(super::Settings);

    /// Set Debug Keys Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype, New)]
    #[command(code = 0x002E, reply = SetDebugKeysReply)]
    pub struct SetDebugKeys(super::DebugKeys);

    /// Reply for [`SetDebugKeys`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetDebugKeysReply(super::Settings);

    /// Set Privacy Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, New)]
    #[command(code = 0x002F, reply = SetPrivacyReply)]
    pub struct SetPrivacy {
        privacy: super::Privacy,
        identity_resolving_key: [u8; 16],
    }

    /// Reply for [`SetPrivacy`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetPrivacyReply(super::Settings);

    /// Load Identity Resolving Keys Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, IterNewtype)]
    #[command(code = 0x0030, reply = LoadIdentityResolvingKeysReply)]
    pub struct LoadIdentityResolvingKeys(Vec<super::IdentityResolvingKey>);

    /// Reply for [`LoadIdentityResolvingKeys`]
    #[derive(Debug, Unpack)]
    pub struct LoadIdentityResolvingKeysReply;

    /// Get Connection Information Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0031, reply = GetConnectionInformationReply)]
    pub struct GetConnectionInformation {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl GetConnectionInformation {
        pub fn new(addr: Address) -> Self {
            let (address, address_type) = split(addr);
            Self {
                address,
                address_type,
            }
        }
    }

    /// Reply for [`GetConnectionInformation`]
    #[derive(Debug, Unpack, Getters)]
    pub struct GetConnectionInformationReply {
        address: super::WrappedAddress,
        address_type: super::AddressType,
        #[getset(get = "pub")]
        rssi: u8,
        #[getset(get = "pub")]
        tx_power: u8,
        #[getset(get = "pub")]
        max_tx_power: u8,
    }

    impl GetConnectionInformationReply {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// Get Clock Information Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0032, reply = GetClockInformationReply)]
    pub struct GetClockInformation {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl GetClockInformation {
        pub fn new(addr: Address) -> Self {
            let (address, address_type) = split(addr);
            Self {
                address,
                address_type,
            }
        }
    }

    /// Reply for [`GetClockInformation`]
    #[derive(Debug, Unpack, Getters)]
    pub struct GetClockInformationReply {
        address: super::WrappedAddress,
        address_type: super::AddressType,
        #[getset(get = "pub")]
        local_clock: u32,
        #[getset(get = "pub")]
        piconet_clock: u32,
        #[getset(get = "pub")]
        accuracy: u16,
    }

    impl GetClockInformationReply {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// Add Device Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0033, reply = AddDeviceReply)]
    pub struct AddDevice {
        address: super::WrappedAddress,
        address_type: super::AddressType,
        action: super::Action,
    }

    impl AddDevice {
        pub fn new(addr: Address, action: super::Action) -> Self {
            let (address, address_type) = split(addr);
            Self {
                address,
                address_type,
                action,
            }
        }
    }

    /// Reply for [`AddDevice`]
    #[derive(Debug, Unpack)]
    pub struct AddDeviceReply {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl AddDeviceReply {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// Remove Device Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0034, reply = RemoveDeviceReply)]
    pub struct RemoveDevice {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl RemoveDevice {
        pub fn new(addr: Address) -> Self {
            let (address, address_type) = split(addr);
            Self {
                address,
                address_type,
            }
        }
    }

    /// Reply for [`RemoveDevice`]
    #[derive(Debug, Unpack)]
    pub struct RemoveDeviceReply {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl RemoveDeviceReply {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// Load Connection Parameters Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, IterNewtype)]
    #[command(code = 0x0035, reply = LoadConnectionParametersReply)]
    pub struct LoadConnectionParameters(Vec<super::ConnectionParameter>);

    /// Reply for [`LoadConnectionParameters`]
    #[derive(Debug, Unpack)]
    pub struct LoadConnectionParametersReply;

    /// Read Unconfigured Controller Index List Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0036, reply = ReadUnconfiguredControllerIndexListReply)]
    pub struct ReadUnconfiguredControllerIndexList;

    /// Reply for [`ReadUnconfiguredControllerIndexList`]
    #[derive(Debug, Unpack, IterNewtype)]
    pub struct ReadUnconfiguredControllerIndexListReply(Vec<ControllerIndex>);

    /// Read Controller Configuration Information Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0037, reply = ReadControllerConfigurationInformationReply)]
    pub struct ReadControllerConfigurationInformation;

    /// Reply for [`ReadControllerConfigurationInformation`]
    #[derive(Debug, Unpack, Getters)]
    #[getset(get = "pub")]
    pub struct ReadControllerConfigurationInformationReply {
        manufacture: u16,
        supported_options: super::ControllerConfigurationOption,
        missing_options: super::ControllerConfigurationOption,
    }

    /// Set External Configuration Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype, New)]
    #[command(code = 0x0038, reply = SetExternalConfigurationReply)]
    pub struct SetExternalConfiguration(bool);

    /// Reply for [`SetExternalConfiguration`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetExternalConfigurationReply(super::ControllerConfigurationOption);

    /// Set Public Address Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0039, reply = SetPublicAddressReply)]
    pub struct SetPublicAddress(super::WrappedAddress);

    impl SetPublicAddress {
        pub fn new(addr: bdaddr::PublicDeviceAddress) -> Self {
            Self(super::WrappedAddress(Address::from(addr).into_bd_addr()))
        }
    }

    /// Reply for [`SetPublicAddress`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetPublicAddressReply(super::ControllerConfigurationOption);

    /// Start Service Discovery Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, New)]
    #[command(code = 0x003A, reply = StartServiceDiscoveryReply)]
    pub struct StartServiceDiscovery {
        address_type: super::AddressTypes,
        rssi_threshold: u8,
        uuids: Vec<super::Uuid>,
    }

    /// Reply for [`StartServiceDiscovery`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct StartServiceDiscoveryReply(super::AddressTypes);

    /// Read Local Out Of Band Extended Data Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype, New)]
    #[command(code = 0x003B, reply = ReadLocalOutOfBandExtendedDataReply)]
    pub struct ReadLocalOutOfBandExtendedData(super::AddressTypes);

    /// Reply for [`ReadLocalOutOfBandExtendedData`]
    #[derive(Debug, Unpack, Getters)]
    #[getset(get = "pub")]
    pub struct ReadLocalOutOfBandExtendedDataReply {
        address_type: super::AddressTypes,
        eir_data: super::VariableLengthBytes,
    }

    /// Read Management Version Information Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x003C, reply = ReadExtendedControllerIndexListReply)]
    pub struct ReadExtendedControllerIndexList;

    /// Reply for [`ReadExtendedControllerIndexList`]
    #[derive(Debug, Unpack, IterNewtype)]
    pub struct ReadExtendedControllerIndexListReply(
        Vec<(ControllerIndex, super::ControllerType, super::ControllerBus)>,
    );

    /// Read Advertising Features Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x003D, reply = ReadAdvertisingFeatureReply)]
    pub struct ReadAdvertisingFeature;

    /// Reply for [`ReadAdvertisingFeature`]
    #[derive(Debug, Unpack, Getters)]
    #[getset(get = "pub")]
    pub struct ReadAdvertisingFeatureReply {
        supported_flags: super::AdvertisingFlag,
        max_adv_data_len: u8,
        max_scan_resp_len: u8,
        max_instances: u8,
        instances: super::AdvertiseInstances,
    }

    /// Add Advertising Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, New)]
    #[command(code = 0x003E, reply = AddAdvertisingReply)]
    pub struct AddAdvertising {
        instance: super::AdvertiseInstance,
        flags: super::AdvertisingFlag,
        duration: u16,
        timeout: u16,
        adv_data_scan_resp: super::AdvDataScanResp,
    }

    /// Reply for [`AddAdvertising`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct AddAdvertisingReply(super::AdvertiseInstance);

    /// Remove Advertising Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype, New)]
    #[command(code = 0x003F, reply = RemoveAdvertisingReply)]
    pub struct RemoveAdvertising(super::AdvertiseInstance);

    /// Reply for [`RemoveAdvertising`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct RemoveAdvertisingReply(super::AdvertiseInstance);

    /// Get Advertising Size Information Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, New)]
    #[command(code = 0x0040, reply = GetAdvertisingSizeInformationReply)]
    pub struct GetAdvertisingSizeInformation {
        instance: super::AdvertiseInstance,
        flags: super::AdvertisingFlag,
    }

    /// Reply for [`GetAdvertisingSizeInformation`]
    #[derive(Debug, Unpack, Getters)]
    #[getset(get = "pub")]
    pub struct GetAdvertisingSizeInformationReply {
        instance: super::AdvertiseInstance,
        flags: super::AdvertisingFlag,
        max_adv_data_len: u8,
        max_scan_resp_len: u8,
    }

    /// Start Limited Discovery Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype, New)]
    #[command(code = 0x0041, reply = StartLimitedDiscoveryReply)]
    pub struct StartLimitedDiscovery(super::AddressTypes);

    /// Reply for [`StartLimitedDiscovery`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct StartLimitedDiscoveryReply(super::AddressTypes);

    /// Read Extended Controller Information Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0042, reply = ReadExtendedControllerInformationReply)]
    pub struct ReadExtendedControllerInformation;

    /// Reply for [`ReadExtendedControllerInformation`]
    #[derive(Debug, Unpack, Getters)]
    pub struct ReadExtendedControllerInformationReply {
        address: super::WrappedAddress,
        #[getset(get = "pub")]
        bluetooth_version: u8,
        #[getset(get = "pub")]
        manufacturer: u16,
        #[getset(get = "pub")]
        supported_settings: super::Settings,
        #[getset(get = "pub")]
        current_settings: super::Settings,
        #[getset(get = "pub")]
        eir_data: super::VariableLengthBytes,
    }

    impl ReadExtendedControllerInformationReply {
        pub fn address(&self) -> &BdAddr {
            &self.address.0
        }
    }

    /// Set Appearance Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype, New)]
    #[command(code = 0x0043, reply = SetApperanceReply)]
    pub struct SetApperance(u16);

    /// Reply for [`SetApperance`]
    #[derive(Debug, Unpack)]
    pub struct SetApperanceReply;

    /// Get PHY Configuration Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    pub struct GetPhyConfiguration;

    /// Reply for [`GetPhyConfiguration`]
    #[derive(Debug, Unpack, Getters)]
    #[getset(get = "pub")]
    pub struct GetPhyConfigurationReply {
        supported_phys: super::Phys,
        configurable_phys: super::Phys,
        selected_phys: super::Phys,
    }

    /// Set PHY Configuration Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype, New)]
    #[command(code = 0x0045, reply = SetPhyConfigurationReply)]
    pub struct SetPhyConfiguration(super::Phys);

    /// Reply for [`SetPhyConfiguration`]
    #[derive(Debug, Unpack)]
    pub struct SetPhyConfigurationReply;

    /// Load Blocked Keys Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, IterNewtype)]
    #[command(code = 0x0046, reply = LoadBlockedKeysReply)]
    pub struct LoadBlockedKeys(Vec<super::BlockedKey>);

    /// Reply for [`LoadBlockedKeys`]
    #[derive(Debug, Unpack)]
    pub struct LoadBlockedKeysReply;

    /// Set Wideband Speech Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype, New)]
    #[command(code = 0x0047, reply = SetWidbandSpeechReply)]
    pub struct SetWidbandSpeech(bool);

    /// Reply for [`SetWidbandSpeech`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetWidbandSpeechReply(super::Settings);

    /// Read Security Information Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0048, reply = ReadSecurityInformationReply)]
    pub struct ReadSecurityInformation;

    /// Reply for [`ReadSecurityInformation`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct ReadSecurityInformationReply(super::VariableLengthBytes);

    /// Read Experimental Features Information Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0049, reply = ReadExperimentalFeaturesInformationReply)]
    pub struct ReadExperimentalFeaturesInformation;

    /// Reply for [`ReadExperimentalFeaturesInformation`]
    #[derive(Debug, Unpack, IterNewtype)]
    pub struct ReadExperimentalFeaturesInformationReply(Vec<(super::Uuid, super::FeatureFlags)>);

    /// Set Experimental Feature Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, New)]
    #[command(code = 0x004A, reply = SetExperimentalFeatureReply)]
    pub struct SetExperimentalFeature {
        uuid: super::Uuid,
        action: super::FeatureAction,
    }

    /// Reply for [`SetExperimentalFeature`]
    #[derive(Debug, Unpack, Getters)]
    #[getset(get = "pub")]
    pub struct SetExperimentalFeatureReply {
        uuid: super::Uuid,
        flags: super::FeatureFlags,
    }

    /// Read Default System Configuration Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x004B, reply = ReadDefaultSystemConfigurationReply)]
    pub struct ReadDefaultSystemConfiguration;

    /// Reply for [`ReadDefaultSystemConfiguration`]
    #[derive(Debug, Unpack, IterNewtype)]
    pub struct ReadDefaultSystemConfigurationReply(
        super::Remaining<super::SystemConfigurationParameter>,
    );

    /// Set Default System Configuration Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, IterNewtype)]
    #[command(code = 0x004C, reply = SetDefaultSystemConfigurationReply)]
    pub struct SetDefaultSystemConfiguration(super::Remaining<super::SystemConfigurationParameter>);

    /// Reply for [`SetDefaultSystemConfiguration`]
    #[derive(Debug, Unpack)]
    pub struct SetDefaultSystemConfigurationReply;

    /// Read Default Runtime Configuration Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x004D, reply = ReadDefaultRuntimeConfigurationReply)]
    pub struct ReadDefaultRuntimeConfiguration;

    /// Reply for [`ReadDefaultRuntimeConfiguration`]
    #[derive(Debug, Unpack, IterNewtype)]
    pub struct ReadDefaultRuntimeConfigurationReply(
        super::Remaining<super::RuntimeConfigurationParameter>,
    );

    /// Read Management Version Information Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, IterNewtype)]
    #[command(code = 0x004E, reply = SetDefaultRuntimeConfigurationReply)]
    pub struct SetDefaultRuntimeConfiguration(
        super::Remaining<super::RuntimeConfigurationParameter>,
    );

    /// Reply for [`SetDefaultRuntimeConfiguration`]
    #[derive(Debug, Unpack)]
    pub struct SetDefaultRuntimeConfigurationReply;

    /// Get Device Flags Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x004F, reply = GetDeviceFlagReply)]
    pub struct GetDeviceFlag {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl GetDeviceFlag {
        pub fn new(addr: Address) -> Self {
            let (address, address_type) = super::split(addr);
            Self {
                address,
                address_type,
            }
        }
    }

    /// Reply for [`GetDeviceFlag`]
    #[derive(Debug, Unpack, Getters)]
    pub struct GetDeviceFlagReply {
        address: super::WrappedAddress,
        address_type: super::AddressType,
        #[getset(get = "pub")]
        supported_flags: super::DeviceFlags,
        #[getset(get = "pub")]
        current_flags: super::DeviceFlags,
    }

    impl GetDeviceFlagReply {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// Set Device Flags Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0050, reply = SetDeviceFlagReply)]
    pub struct SetDeviceFlag {
        address: super::WrappedAddress, // TODO typo
        address_type: super::AddressType,
        current_flags: super::DeviceFlags,
    }

    impl SetDeviceFlag {
        pub fn new(addr: Address, current_flags: super::DeviceFlags) -> Self {
            let (address, address_type) = super::split(addr);
            Self {
                address,
                address_type,
                current_flags,
            }
        }
    }

    /// Reply for [`SetDeviceFlag`]
    #[derive(Debug, Unpack)]
    pub struct SetDeviceFlagReply {
        address: super::WrappedAddress,
        address_type: super::AddressType,
    }

    impl SetDeviceFlagReply {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// Read Advertisement Monitor Features Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0051, reply = ReadAdvertisementMonitorFeaturesReply)]
    pub struct ReadAdvertisementMonitorFeatures;

    /// Reply for [`ReadAdvertisementMonitorFeatures`]
    #[derive(Debug, Unpack, Getters)]
    #[getset(get = "pub")]
    pub struct ReadAdvertisementMonitorFeaturesReply {
        supported_features: super::AdvertisementMonitorFeatures,
        enabled_features: super::AdvertisementMonitorFeatures,
        max_num_handle: u16,
        max_num_pattern: u8,
        handles: Vec<super::AdvertisementMonitorHandle>,
    }

    /// Add Advertisement Patterns Monitor Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, IterNewtype)]
    #[command(code = 0x0052, reply = AddAdvertisementPatternsMonitorReply)]
    pub struct AddAdvertisementPatternsMonitor(Vec<super::AdvertisementPattern>);

    /// Reply for [`AddAdvertisementPatternsMonitor`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct AddAdvertisementPatternsMonitorReply(super::AdvertisementMonitorHandle);

    /// Remove Advertisement Monitor Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype, New)]
    #[command(code = 0x0053, reply = RemoveAdvertisementPatternsMonitorReply)]
    pub struct RemoveAdvertisementPatternsMonitor(super::AdvertisementMonitorHandle);

    /// Reply for [`RemoveAdvertisementPatternsMonitor`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct RemoveAdvertisementPatternsMonitorReply(super::AdvertisementMonitorHandle);
}

#[doc(hidden)]
pub fn pack_command<W>(
    index: &ControllerIndex,
    command: &Command,
    write: &mut W,
) -> pack::Result<()>
where
    W: io::Write,
{
    use smallvec::SmallVec;

    let mut buf = SmallVec::<[u8; 64]>::new();
    command.pack_inner(&mut buf)?;

    command.code().pack(write)?;
    index.pack(write)?;
    (buf.len() as u16).pack(write)?;
    write.write_all(&buf)?;

    Ok(())
}
