//! mgmt API commands
use btmgmt_packet_helper::commands;
use btmgmt_packet_helper::pack::{Pack, Unpack};

pub use imp::*;
use super::*;

// Management API Command
#[commands(name = Commands, trait = Command, codes = CommandCode)]
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
    #[derive(Debug, Unpack)]
    pub struct ReadManagementVersionInformationReply {
        pub version: u8,
        pub revision: u16,
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
    #[derive(Debug, Unpack)]
    pub struct ReadControllerInformationReply {
        pub address: super::Address,
        pub bluetooth_version: u8,
        pub manufacturer: u16,
        pub supported_settings: super::Settings,
        pub current_settings: super::Settings,
        pub class_of_device: super::ClassOfDevice,
        pub name: super::Name,
        pub short_name: super::ShortName,
    }

    /// Set Powered Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype)]
    #[command(code = 0x0005, reply = SetPoweredReply)]
    pub struct SetPowered(bool);

    /// Reply for [`SetPowered`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetPoweredReply(super::Settings);

    /// Set Discoverable Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0006, reply = SetDiscoverableReply)]
    pub struct SetDiscoverable {
        pub discoverable: super::Discoverable,
        pub timeout: u16,
    }

    /// Reply for [`SetDiscoverable`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetDiscoverableReply(super::Settings);

    /// Set Connectable Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype)]
    #[command(code = 0x0007, reply = SetConnectableReply)]
    pub struct SetConnectable(bool);

    /// Reply for [`SetConnectable`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetConnectableReply(super::Settings);

    /// Set Fast Connectable Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype)]
    #[command(code = 0x0008, reply = SetFastConnectableReply)]
    pub struct SetFastConnectable(bool);

    /// Reply for [`SetFastConnectable`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetFastConnectableReply(super::Settings);

    /// Set Bondable Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype)]
    #[command(code = 0x0009, reply = SetBondableReply)]
    pub struct SetBondable(bool);

    /// Reply for [`SetBondable`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetBondableReply(super::Settings);

    /// Set Link Security Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype)]
    #[command(code = 0x000A, reply = SetLinkSecurityReply)]
    pub struct SetLinkSecurity(bool);

    /// Reply for [`SetLinkSecurity`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetLinkSecurityReply(super::Settings);

    /// Set Secure Simple Pairing Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype)]
    #[command(code = 0x000B, reply = SetSecureSimplePairingReply)]
    pub struct SetSecureSimplePairing(bool);

    /// Reply for [`SetSecureSimplePairing`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetSecureSimplePairingReply(super::Settings);

    /// Set High Speed Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype)]
    #[command(code = 0x000C, reply = SetHighSpeedReply)]
    pub struct SetHighSpeed(bool);

    /// Reply for [`SetHighSpeed`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetHighSpeedReply(super::Settings);

    /// Set Low Energy Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype)]
    #[command(code = 0x000D, reply = SetLowEnergyReply)]
    pub struct SetLowEnergy(bool);

    /// Reply for [`SetLowEnergy`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetLowEnergyReply(super::Settings);

    /// Set Device Class Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x000E, reply = SetDeviceClassReply)]
    pub struct SetDeviceClass {
        pub major_class: u8,
        pub minor_class: u8,
    }

    /// Reply for [`SetDeviceClass`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetDeviceClassReply(super::ClassOfDevice);

    /// Set Local Name Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x000F, reply = SetLocalNameReply)]
    pub struct SetLocalName {
        pub name: super::Name,
        pub short_name: super::ShortName,
    }

    /// Reply for [`SetLocalName`]
    #[derive(Debug, Unpack)]
    pub struct SetLocalNameReply {
        pub name: super::Name,
        pub short_name: super::ShortName,
    }

    /// Add UUID Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0010, reply = AddUuidReply)]
    pub struct AddUuid {
        pub uuid: super::Uuid,
        pub svc_hint: u8,
    }

    /// Reply for [`AddUuid`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct AddUuidReply(super::ClassOfDevice);

    /// Remove UUID Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0011, reply = RemoveUuidReply)]
    pub struct RemoveUuid {
        pub uuid: super::Uuid,
    }

    /// Reply for [`RemoveUuid`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct RemoveUuidReply(super::ClassOfDevice);

    /// Load Link Keys Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0012, reply = LoadLinkKeysReply)]
    pub struct LoadLinkKeys {
        pub debug_keys: bool,
        pub keys: Vec<super::LinkKey>,
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
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// Reply for [`Disconnect`]
    #[derive(Debug, Unpack)]
    pub struct DisconnectReply {
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// Get Connections Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0015, reply = GetConnectionsReply)]
    pub struct GetConnections;

    /// Reply for [`GetConnections`]
    #[derive(Debug, Unpack, IterNewtype)]
    pub struct GetConnectionsReply(Vec<(super::Address, super::AddressType)>);

    /// PIN Code Reply Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0016, reply = PinCodeReplyReply)]
    pub struct PinCodeReply {
        pub address: super::Address,
        pub address_type: super::AddressType,
        pub pin_length: u8,
        pub pin_code: [u8; 16],
    }

    /// Reply for [`PinCodeReply`]
    #[derive(Debug, Unpack)]
    pub struct PinCodeReplyReply {
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// PIN Code Negative Reply Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0017, reply = PinCodeNegativeReplyReply)]
    pub struct PinCodeNegativeReply {
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// Reply for [`PinCodeNegativeReply`]
    #[derive(Debug, Unpack)]
    pub struct PinCodeNegativeReplyReply {
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// Set IO Capability Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype)]
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
        pub address: super::Address,
        pub address_type: super::AddressType,
        pub io_capability: super::IoCapability,
    }

    /// Reply for [`PairDevice`]
    #[derive(Debug, Unpack)]
    pub struct PairDeviceReply {
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// Cancel Pair Device Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x001A, reply = CancelPairDeviceReply)]
    pub struct CancelPairDevice {
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// Reply for [`CancelPairDevice`]
    #[derive(Debug, Unpack)]
    pub struct CancelPairDeviceReply {
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// Unpair Device Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x001B, reply = UnpairDeviceReply)]
    pub struct UnpairDevice {
        pub address: super::Address,
        pub address_type: super::AddressType,
        pub disconnect: bool,
    }

    /// Reply for [`UnpairDevice`]
    #[derive(Debug, Unpack)]
    pub struct UnpairDeviceReply {
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// User Confirmation Reply Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x001C, reply = UserConfirmationReplyReply)]
    pub struct UserConfirmationReply {
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// Reply for [`UserConfirmationReply`]
    #[derive(Debug, Unpack)]
    pub struct UserConfirmationReplyReply {
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// User Confirmation Negative Reply Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x001D, reply = UserConfirmationNegativeReplyReply)]
    pub struct UserConfirmationNegativeReply {
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// Reply for [`UserConfirmationNegativeReply`]
    #[derive(Debug, Unpack)]
    pub struct UserConfirmationNegativeReplyReply {
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// User Passkey Reply Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x001E, reply = UserPasskeyReplyReply)]
    pub struct UserPasskeyReply {
        pub address: super::Address,
        pub address_type: super::AddressType,
        pub passkey: u32,
    }

    /// Reply for [`UserPasskeyReply`]
    #[derive(Debug, Unpack)]
    pub struct UserPasskeyReplyReply {
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// User Passkey Negative Reply Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x001F, reply = UserPasskeyNegativeReplyReply)]
    pub struct UserPasskeyNegativeReply {
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// Reply for [`UserPasskeyNegativeReply`]
    #[derive(Debug, Unpack)]
    pub struct UserPasskeyNegativeReplyReply {
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// Read Local Out Of Band Data Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0020, reply = ReadLocalOutOfBandDataReply)]
    pub struct ReadLocalOutOfBandData;

    /// Reply for [`ReadLocalOutOfBandData`]
    #[derive(Debug, Unpack)]
    pub struct ReadLocalOutOfBandDataReply {
        pub hash192: [u8; 16],
        pub randomizer192: [u8; 16],
        pub hash256: Option<[u8; 16]>,
        pub randomizer256: Option<[u8; 16]>,
    }

    /// Add Remote Out Of Band Data Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0021, reply = AddRemoteOutOfBandDataReply)]
    pub struct AddRemoteOutOfBandData {
        pub address: super::Address,
        pub address_type: super::AddressType,
        pub hash192: [u8; 16],
        pub randomizer192: [u8; 16],
        pub hash256: Option<[u8; 16]>,
        pub randomizer256: Option<[u8; 16]>,
    }

    /// Reply for [`AddRemoteOutOfBandData`]
    #[derive(Debug, Unpack)]
    pub struct AddRemoteOutOfBandDataReply {
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// Remove Remote Out Of Band Data Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0022, reply = RemoveRemoteOutOfBandDataReply)]
    pub struct RemoveRemoteOutOfBandData {
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// Reply for [`RemoveRemoteOutOfBandData`]
    #[derive(Debug, Unpack)]
    pub struct RemoveRemoteOutOfBandDataReply {
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// Start Discovery Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype)]
    #[command(code = 0x0023, reply = StartDiscoveryReply)]
    pub struct StartDiscovery(super::AddressTypes);

    /// Reply for [`StartDiscovery`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct StartDiscoveryReply(super::AddressTypes);

    /// Stop Discovery Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype)]
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
        pub address: super::Address,
        pub address_type: super::AddressType,
        pub name_known: bool,
    }

    /// Reply for [`ConfirmName`]
    #[derive(Debug, Unpack)]
    pub struct ConfirmNameReply {
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// Block Device Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0026, reply = BlockDeviceReply)]
    pub struct BlockDevice {
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// Reply for [`BlockDevice`]
    #[derive(Debug, Unpack)]
    pub struct BlockDeviceReply {
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// Unblock Device Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0027, reply = UnblockDeviceReply)]
    pub struct UnblockDevice {
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// Reply for [`UnblockDevice`]
    #[derive(Debug, Unpack)]
    pub struct UnblockDeviceReply {
        pub address: super::Address,
        pub address_type: super::AddressType,
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
    #[derive(Debug, Pack, Newtype)]
    #[command(code = 0x0029, reply = SetAdvertisingReply)]
    pub struct SetAdvertising(super::Advertising);

    /// Reply for [`SetAdvertising`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetAdvertisingReply(super::Settings);

    /// Set BR/EDR Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype)]
    #[command(code = 0x002A, reply = SetBrEdrReply)]
    pub struct SetBrEdr(bool);

    /// Reply for [`SetBrEdr`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetBrEdrReply(super::Settings);

    /// Read Management Version Information Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype)]
    #[command(code = 0x002B, reply = SetStaticAddressReply)]
    pub struct SetStaticAddress(super::Address);

    /// Reply for [`SetStaticAddress`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetStaticAddressReply(super::Settings);

    /// Set Scan Parameters Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x002C, reply = SetScanParametersReply)]
    pub struct SetScanParameters {
        pub interval: u16,
        pub window: u16,
    }

    /// Reply for [`SetScanParameters`]
    #[derive(Debug, Unpack)]
    pub struct SetScanParametersReply;

    /// Set Secure Connections Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype)]
    #[command(code = 0x002D, reply = SetSecureConnectionsReply)]
    pub struct SetSecureConnections(super::SecureConnections);

    /// Reply for [`SetSecureConnections`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetSecureConnectionsReply(super::Settings);

    /// Set Debug Keys Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype)]
    #[command(code = 0x002E, reply = SetDebugKeysReply)]
    pub struct SetDebugKeys(super::DebugKeys);

    /// Reply for [`SetDebugKeys`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetDebugKeysReply(super::Settings);

    /// Set Privacy Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x002F, reply = SetPrivacyReply)]
    pub struct SetPrivacy {
        pub privacy: super::Privacy,
        pub identity_resolving_key: [u8; 16],
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
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// Reply for [`GetConnectionInformation`]
    #[derive(Debug, Unpack)]
    pub struct GetConnectionInformationReply {
        pub address: super::Address,
        pub address_type: super::AddressType,
        pub rssi: u8,
        pub tx_power: u8,
        pub max_tx_power: u8,
    }

    /// Get Clock Information Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0032, reply = GetClockInformationReply)]
    pub struct GetClockInformation {
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// Reply for [`GetClockInformation`]
    #[derive(Debug, Unpack)]
    pub struct GetClockInformationReply {
        pub address: super::Address,
        pub address_type: super::AddressType,
        pub local_clock: u32,
        pub piconet_clock: u32,
        pub accuracy: u16,
    }

    /// Add Device Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0033, reply = AddDeviceReply)]
    pub struct AddDevice {
        pub address: super::Address,
        pub address_type: super::AddressType,
        pub action: super::Action,
    }

    /// Reply for [`AddDevice`]
    #[derive(Debug, Unpack)]
    pub struct AddDeviceReply {
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// Remove Device Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0034, reply = RemoveDeviceReply)]
    pub struct RemoveDevice {
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// Reply for [`RemoveDevice`]
    #[derive(Debug, Unpack)]
    pub struct RemoveDeviceReply {
        pub address: super::Address,
        pub address_type: super::AddressType,
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
    #[derive(Debug, Unpack)]
    pub struct ReadControllerConfigurationInformationReply {
        pub manufacture: u16,
        pub supported_options: super::ControllerConfigurationOption,
        pub missing_options: super::ControllerConfigurationOption,
    }

    /// Set External Configuration Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype)]
    #[command(code = 0x0038, reply = SetExternalConfigurationReply)]
    pub struct SetExternalConfiguration(bool);

    /// Reply for [`SetExternalConfiguration`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetExternalConfigurationReply(super::ControllerConfigurationOption);

    /// Set Public Address Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype)]
    #[command(code = 0x0039, reply = SetPublicAddressReply)]
    pub struct SetPublicAddress(super::Address);

    /// Reply for [`SetPublicAddress`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct SetPublicAddressReply(super::ControllerConfigurationOption);

    /// Start Service Discovery Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x003A, reply = StartServiceDiscoveryReply)]
    pub struct StartServiceDiscovery {
        pub address_type: super::AddressTypes,
        pub rssi_threshold: u8,
        pub uuids: Vec<super::Uuid>,
    }

    /// Reply for [`StartServiceDiscovery`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct StartServiceDiscoveryReply(super::AddressTypes);

    /// Read Local Out Of Band Extended Data Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype)]
    #[command(code = 0x003B, reply = ReadLocalOutOfBandExtendedDataReply)]
    pub struct ReadLocalOutOfBandExtendedData(super::AddressTypes);

    /// Reply for [`ReadLocalOutOfBandExtendedData`]
    #[derive(Debug, Unpack)]
    pub struct ReadLocalOutOfBandExtendedDataReply {
        pub address_type: super::AddressTypes,
        pub eir_data: super::VariableLengthBytes,
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
    pub struct ReadExtendedControllerIndexListReply(Vec<(ControllerIndex, super::ControllerType, super::ControllerBus)>);

    /// Read Advertising Features Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x003D, reply = ReadAdvertisingFeatureReply)]
    pub struct ReadAdvertisingFeature;

    /// Reply for [`ReadAdvertisingFeature`]
    #[derive(Debug, Unpack)]
    pub struct ReadAdvertisingFeatureReply {
        pub supported_flags: super::AdvertisingFlag,
        pub max_adv_data_len: u8,
        pub max_scan_resp_len: u8,
        pub max_instances: u8,
        pub instances: super::AdvertiseInstances,
    }

    /// Add Advertising Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x003E, reply = AddAdvertisingReply)]
    pub struct AddAdvertising {
        pub instance: super::AdvertiseInstance,
        pub flags: super::AdvertisingFlag,
        pub duration: u16,
        pub timeout: u16,
        pub adv_data_scan_resp: super::AdvDataScanResp,
    }

    /// Reply for [`AddAdvertising`]
    #[derive(Debug, Unpack)]
    pub struct AddAdvertisingReply {
        pub instance: super::AdvertiseInstance,
    }

    /// Remove Advertising Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype)]
    #[command(code = 0x003F, reply = RemoveAdvertisingReply)]
    pub struct RemoveAdvertising(super::AdvertiseInstance);

    /// Reply for [`RemoveAdvertising`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct RemoveAdvertisingReply(super::AdvertiseInstance);

    /// Get Advertising Size Information Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0040, reply = GetAdvertisingSizeInformationReply)]
    pub struct GetAdvertisingSizeInformation {
        pub instance: super::AdvertiseInstance,
        pub flags: super::AdvertisingFlag,
    }

    /// Reply for [`GetAdvertisingSizeInformation`]
    #[derive(Debug, Unpack)]
    pub struct GetAdvertisingSizeInformationReply {
        pub instance: super::AdvertiseInstance,
        pub flags: super::AdvertisingFlag,
        pub max_adv_data_len: u8,
        pub max_scan_resp_len: u8,
    }

    /// Start Limited Discovery Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype)]
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
    #[derive(Debug, Unpack)]
    pub struct ReadExtendedControllerInformationReply {
        pub address: super::Address,
        pub bluetooth_version: u8,
        pub manufacturer: u16,
        pub supported_settings: super::Settings,
        pub current_settings: super::Settings,
        pub eir_data: super::VariableLengthBytes,
    }

    /// Set Appearance Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype)]
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
    #[derive(Debug, Unpack)]
    pub struct GetPhyConfigurationReply {
        pub supported_phys: super::Phys,
        pub configurable_phys: super::Phys,
        pub selected_phys: super::Phys,
    }

    /// Set PHY Configuration Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, Newtype)]
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
    #[derive(Debug, Pack, Newtype)]
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
    #[derive(Debug, Pack)]
    #[command(code = 0x004A, reply = SetExperimentalFeatureReply)]
    pub struct SetExperimentalFeature {
        pub uuid: super::Uuid,
        pub action: super::FeatureAction,
    }

    /// Reply for [`SetExperimentalFeature`]
    #[derive(Debug, Unpack)]
    pub struct SetExperimentalFeatureReply {
        pub uuid: super::Uuid,
        pub flags: super::FeatureFlags,
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
    pub struct ReadDefaultSystemConfigurationReply(super::Remaining<super::SystemConfigurationParameter>);

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
    pub struct ReadDefaultRuntimeConfigurationReply(super::Remaining<super::RuntimeConfigurationParameter>);

    /// Read Management Version Information Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack, IterNewtype)]
    #[command(code = 0x004E, reply = SetDefaultRuntimeConfigurationReply)]
    pub struct SetDefaultRuntimeConfiguration(super::Remaining<super::RuntimeConfigurationParameter>);

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
        pub addrss: super::Address,
        pub address_type: super::AddressType,
    }

    /// Reply for [`GetDeviceFlag`]
    #[derive(Debug, Unpack)]
    pub struct GetDeviceFlagReply {
        pub addrss: super::Address,
        pub address_type: super::AddressType,
        pub supported_flags: super::DeviceFlags,
        pub current_flags: super::DeviceFlags,
    }

    /// Set Device Flags Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0050, reply = SetDeviceFlagReply)]
    pub struct SetDeviceFlag {
        pub addrss: super::Address,
        pub address_type: super::AddressType,
        pub current_flags: super::DeviceFlags,
    }

    /// Reply for [`SetDeviceFlag`]
    #[derive(Debug, Unpack)]
    pub struct SetDeviceFlagReply {
        pub addrss: super::Address,
        pub address_type: super::AddressType,
    }

    /// Read Advertisement Monitor Features Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Pack)]
    #[command(code = 0x0051, reply = ReadAdvertisementMonitorFeaturesReply)]
    pub struct ReadAdvertisementMonitorFeatures;

    /// Reply for [`ReadAdvertisementMonitorFeatures`]
    #[derive(Debug, Unpack)]
    pub struct ReadAdvertisementMonitorFeaturesReply {
        pub supported_features: super::AdvertisementMonitorFeatures,
        pub enabled_features: super::AdvertisementMonitorFeatures,
        pub max_num_handle: u16,
        pub max_num_pattern: u8,
        pub handles: Vec<super::AdvertisementMonitorHandle>,
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
    #[derive(Debug, Pack, Newtype)]
    #[command(code = 0x0053, reply = RemoveAdvertisementPatternsMonitorReply)]
    pub struct RemoveAdvertisementPatternsMonitor(super::AdvertisementMonitorHandle);

    /// Reply for [`RemoveAdvertisementPatternsMonitor`]
    #[derive(Debug, Unpack, Newtype)]
    pub struct RemoveAdvertisementPatternsMonitorReply(super::AdvertisementMonitorHandle);

}

#[doc(hidden)]
pub fn pack_command<W>(index: &ControllerIndex, command: &Commands, write: &mut W) -> pack::Result<()> where W: io::Write {
    let mut buf = vec![];
    command.pack_inner(&mut buf)?;

    command.code().pack(write)?;
    index.pack(write)?;
    (buf.len() as u16).pack(write)?;
    write.write_all(&buf)?;

    Ok(())
}
