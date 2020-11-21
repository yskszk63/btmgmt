//! mgmt API commands.
use std::collections::HashSet;
use std::ops::Deref;

use bytes::BytesMut;
use derive_new::new as New;
use getset::{Getters, Setters};

use super::ControllerIndex;
use crate::pack::{Pack, Unpack};

mod impl_from_iter;
mod impl_into_iter;

macro_rules! command {
    (
        $(
            $(#[$attrs:meta])*
            $vis:vis command $name:ident : $code:literal {
                $(
                    $(#[$fattrs:meta])*
                    $fvis:vis $fname:ident : $fty:ty,
                )*
            }

            $(#[$rattrs:meta])*
            $rvis:vis reply $rname:ident {
                $(
                    $(#[$rfattrs:meta])*
                    $rfvis:vis $rfname:ident : $rfty:ty,
                )*
            }
        )*
    ) => {
        $(
            packable_struct! {
                $(#[$attrs])*
                $vis struct $name {
                    $(
                        $(#[$fattrs])*
                        $fvis $fname: $fty,
                    )*
                }
            }

            packable_struct! {
                $(#[$rattrs])*
                $rvis struct $rname {
                    $(
                        $(#[$rfattrs])*
                        $rfvis $rfname: $rfty,
                    )*
                }
            }

            impl Command for $name {
                const CODE: CommandCode = CommandCode::$name;
                type Reply = $rname;
            }

        )*

        packable_enum! {
            /// Command Code
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            pub enum CommandCode: u16 {
                $( $name => $code,)*
            }
        }
    }
}

command! {

    /// Read Management Version Information Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Default, New)]
    pub command ReadManagementVersionInformation: 0x0001 {
    }

    /// Reply for [`ReadManagementVersionInformation`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply ReadManagementVersionInformationReply {
        version: u8,
        revision: u16,
    }

    /// Read Management Supported Commands Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Default, New)]
    pub command ReadManagementSupportedCommands: 0x0002 {
    }

    /// Reply for [`ReadManagementSupportedCommands`]
    #[derive(Debug)]
    pub reply ReadManagementSupportedCommandsReply {
        value: super::CommandsEvents,
    }

    /// Read Controller Index List Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Default, New)]
    pub command ReadControllerIndexList: 0x0003 {
    }

    /// Reply for [`ReadControllerIndexList`]
    #[derive(Debug)]
    pub reply ReadControllerIndexListReply {
        values: Vec<ControllerIndex>,
    }

    /// Read Controller Information Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Default, New)]
    pub command ReadControllerInformation: 0x0004 {
    }

    /// Reply for [`ReadControllerInformation`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply ReadControllerInformationReply {
        address: super::Address,
        bluetooth_version: u8,
        manufacturer: u16,
        supported_settings: super::Settings,
        current_settings: super::Settings,
        class_of_device: super::ClassOfDevice,
        name: super::Name,
        short_name: super::ShortName,
    }

    /// Set Powered Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command SetPowered: 0x0005 {
        powered: bool,
    }

    /// Reply for [`SetPowered`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply SetPoweredReply {
        current_settings: super::Settings,
    }

    /// Set Discoverable Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command SetDiscoverable: 0x0006 {
        discoverable: super::Discoverable,
        timeout: u16,
    }

    /// Reply for [`SetDiscoverable`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply SetDiscoverableReply {
        current_settings: super::Settings,
    }

    /// Set Connectable Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command SetConnectable: 0x0007 {
        connectable: bool,
    }

    /// Reply for [`SetConnectable`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply SetConnectableReply {
        current_settings: super::Settings,
    }

    /// Set Fast Connectable Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command SetFastConnectable: 0x0008 {
        enable: bool,
    }

    /// Reply for [`SetFastConnectable`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply SetFastConnectableReply {
        current_settings: super::Settings,
    }

    /// Set Bondable Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command SetBondable: 0x0009 {
        bondable: bool,
    }

    /// Reply for [`SetBondable`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply SetBondableReply {
        current_settings: super::Settings,
    }

    /// Set Link Security Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command SetLinkSecurity: 0x000A {
        link_security: bool,
    }

    /// Reply for [`SetLinkSecurity`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply SetLinkSecurityReply {
        current_settings: super::Settings,
    }

    /// Set Secure Simple Pairing Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command SetSecureSimplePairing: 0x000B {
        secure_simple_pairing: bool,
    }

    /// Reply for [`SetSecureSimplePairing`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply SetSecureSimplePairingReply {
        current_settings: super::Settings,
    }

    /// Set High Speed Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command SetHighSpeed: 0x000C {
        high_speed: bool,
    }

    /// Reply for [`SetHighSpeed`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply SetHighSpeedReply {
        current_settings: super::Settings,
    }

    /// Set Low Energy Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command SetLowEnergy: 0x000D {
        low_energy: bool,
    }

    /// Reply for [`SetLowEnergy`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply SetLowEnergyReply {
        current_settings: super::Settings,
    }

    /// Set Device Class Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command SetDeviceClass: 0x000E {
        major_class: u8,
        minor_class: u8,
    }

    /// Reply for [`SetDeviceClass`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply SetDeviceClassReply {
        class_of_device: super::ClassOfDevice,
    }

    /// Set Local Name Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command SetLocalName: 0x000F {
        name: super::Name,
        short_name: super::ShortName,
    }

    /// Reply for [`SetLocalName`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply SetLocalNameReply {
        name: super::Name,
        short_name: super::ShortName,
    }

    /// Add UUID Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command AddUuid: 0x0010 {
        uuid: super::Uuid,
        svc_hint: u8,
    }

    /// Reply for [`AddUuid`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply AddUuidReply {
        class_of_device: super::ClassOfDevice,
    }

    /// Remove UUID Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command RemoveUuid: 0x0011 {
        uuid: super::Uuid,
    }

    /// Reply for [`RemoveUuid`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply RemoveUuidReply {
        class_of_device: super::ClassOfDevice,
    }

    /// Load Link Keys Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Setters)]
    pub command LoadLinkKeys: 0x0012 {
        #[set="pub"]
        debug_keys: bool,
        keys: Vec<super::LinkKey>,
    }

    /// Reply for [`LoadLinkKeys`]
    #[derive(Debug)]
    pub reply LoadLinkKeysReply {
    }

    /// Load Long Term Keys Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug)]
    pub command LoadLongTermKey: 0x0013 {
        keys: Vec<super::LongTermKey>,
    }

    /// Reply for [`LoadLongTermKey`]
    #[derive(Debug)]
    pub reply LoadLongTermKeyReply {
    }

    /// Disconnect Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command Disconnect: 0x0014 {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Reply for [`Disconnect`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply DisconnectReply {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Get Connections Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Default, New)]
    pub command GetConnections: 0x0015 {
    }

    /// Reply for [`GetConnections`]
    #[derive(Debug)]
    pub reply GetConnectionsReply {
        values: Vec<(super::Address, super::AddressType)>,
    }

    /// PIN Code Reply Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug)]
    pub command PinCodeReply: 0x0016 {
        address: super::Address,
        address_type: super::AddressType,
        pin_length: u8,
        pin_code: [u8; 16],
    }

    /// Reply for [`PinCodeReply`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply PinCodeReplyReply {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// PIN Code Negative Reply Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command PinCodeNegativeReply: 0x0017 {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Reply for [`PinCodeNegativeReply`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply PinCodeNegativeReplyReply {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Set IO Capability Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command SetIoCapability: 0x0018 {
        io_capability: super::IoCapability,
    }

    /// Reply for [`SetIoCapability`]
    #[derive(Debug)]
    pub reply SetIoCapabilityReply {
    }

    /// Read Management Version Information Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command PairDevice: 0x0019 {
        address: super::Address,
        address_type: super::AddressType,
        io_capability: super::IoCapability,
    }

    /// Reply for [`PairDevice`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply PairDeviceReply {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Cancel Pair Device Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command CancelPairDevice: 0x001A {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Reply for [`CancelPairDevice`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply CancelPairDeviceReply {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Unpair Device Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command UnpairDevice: 0x001B {
        address: super::Address,
        address_type: super::AddressType,
        disconnect: bool,
    }

    /// Reply for [`UnpairDevice`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply UnpairDeviceReply {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// User Confirmation Reply Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command UserConfirmationReply: 0x001C {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Reply for [`UserConfirmationReply`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply UserConfirmationReplyReply {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// User Confirmation Negative Reply Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command UserConfirmationNegativeReply: 0x001D {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Reply for [`UserConfirmationNegativeReply`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply UserConfirmationNegativeReplyReply {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// User Passkey Reply Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command UserPasskeyReply: 0x001E {
        address: super::Address,
        address_type: super::AddressType,
        passkey: u32,
    }

    /// Reply for [`UserPasskeyReply`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply UserPasskeyReplyReply {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// User Passkey Negative Reply Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command UserPasskeyNegativeReply: 0x001F {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Reply for [`UserPasskeyNegativeReply`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply UserPasskeyNegativeReplyReply {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Read Local Out Of Band Data Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Default, New)]
    pub command ReadLocalOutOfBandData: 0x0020 {
    }

    /// Reply for [`ReadLocalOutOfBandData`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply ReadLocalOutOfBandDataReply {
        hash192: [u8; 16],
        randomizer192: [u8; 16],
        hash256: Option<[u8; 16]>,
        randomizer256: Option<[u8; 16]>,
    }

    /// Add Remote Out Of Band Data Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command AddRemoteOutOfBandData: 0x0021 {
        address: super::Address,
        address_type: super::AddressType,
        hash192: [u8; 16],
        randomizer192: [u8; 16],
        hash256: Option<[u8; 16]>,
        randomizer256: Option<[u8; 16]>,
    }

    /// Reply for [`AddRemoteOutOfBandData`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply AddRemoteOutOfBandDataReply {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Remove Remote Out Of Band Data Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command RemoveRemoteOutOfBandData: 0x0022 {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Reply for [`RemoveRemoteOutOfBandData`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply RemoveRemoteOutOfBandDataReply {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Start Discovery Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command StartDiscovery: 0x0023 {
        address_type: HashSet<super::AddressType>,
    }

    /// Reply for [`StartDiscovery`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply StartDiscoveryReply {
        address_type: HashSet<super::AddressType>,
    }

    /// Stop Discovery Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command StopDiscovery: 0x0024 {
        address_type: HashSet<super::AddressType>,
    }

    /// Reply for [`StopDiscovery`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply StopDiscoveryReply {
        address_type: HashSet<super::AddressType>,
    }

    /// Confirm Name Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command ConfirmName: 0x0025 {
        address: super::Address,
        address_type: super::AddressType,
        name_known: bool,
    }

    /// Reply for [`ConfirmName`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply ConfirmNameReply {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Block Device Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command BlockDevice: 0x0026 {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Reply for [`BlockDevice`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply BlockDeviceReply {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Unblock Device Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command UnblockDevice: 0x0027 {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Reply for [`UnblockDevice`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply UnblockDeviceReply {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Set Device ID Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command SetDeviceId: 0x0028 {
        source: super::DeviceIdSource,
        vendor: u16,
        product: u16,
        version: u16,
    }

    /// Reply for [`SetDeviceId`]
    #[derive(Debug)]
    pub reply SetDeviceIdReply {
    }

    /// Set Advertising Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command SetAdvertising: 0x0029 {
        advertising: super::Advertising,
    }

    /// Reply for [`SetAdvertising`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply SetAdvertisingReply {
        current_settings: super::Settings,
    }

    /// Set BR/EDR Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command SetBrEdr: 0x002A {
        br_edr: bool,
    }

    /// Reply for [`SetBrEdr`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply SetBrEdrReply {
        current_settings: super::Settings,
    }

    /// Read Management Version Information Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command SetStaticAddress: 0x002B {
        address: super::Address,
    }

    /// Reply for [`SetStaticAddress`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply SetStaticAddressReply {
        current_settings: super::Settings,
    }

    /// Set Scan Parameters Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command SetScanParameters: 0x002C {
        interval: u16,
        window: u16,
    }

    /// Reply for [`SetScanParameters`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply SetScanParametersReply {
    }

    /// Set Secure Connections Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command SetSecureConnections: 0x002D {
        secure_connections: super::SecureConnections,
    }

    /// Reply for [`SetSecureConnections`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply SetSecureConnectionsReply {
        current_settings: super::Settings,
    }

    /// Set Debug Keys Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command SetDebugKeys: 0x002E {
        debug_keys: super::DebugKeys,
    }

    /// Reply for [`SetDebugKeys`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply SetDebugKeysReply {
        current_settings: super::Settings,
    }

    /// Set Privacy Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command SetPrivacy: 0x002F {
        privacy: super::Privacy,
        identity_resolving_key: [u8; 16],
    }

    /// Reply for [`SetPrivacy`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply SetPrivacyReply {
        current_settings: super::Settings,
    }

    /// Load Identity Resolving Keys Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug)]
    pub command LoadIdentityResolvingKeys: 0x0030 {
        keys: Vec<super::IdentityResolvingKey>,
    }

    /// Reply for [`LoadIdentityResolvingKeys`]
    #[derive(Debug)]
    pub reply LoadIdentityResolvingKeysReply {
    }

    /// Get Connection Information Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command GetConnectionInformation: 0x0031 {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Reply for [`GetConnectionInformation`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply GetConnectionInformationReply {
        address: super::Address,
        address_type: super::AddressType,
        rssi: u8,
        tx_power: u8,
        max_tx_power: u8,
    }

    /// Get Clock Information Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command GetClockInformation: 0x0032 {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Reply for [`GetClockInformation`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply GetClockInformationReply {
        address: super::Address,
        address_type: super::AddressType,
        local_clock: u32,
        piconet_clock: u32,
        accuracy: u16,
    }

    /// Add Device Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command AddDevice: 0x0033 {
        address: super::Address,
        address_type: super::AddressType,
        action: super::Action,
    }

    /// Reply for [`AddDevice`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply AddDeviceReply {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Remove Device Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command RemoveDevice: 0x0034 {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Reply for [`RemoveDevice`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply RemoveDeviceReply {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Load Connection Parameters Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug)]
    pub command LoadConnectionParameters: 0x0035 {
        params: Vec<super::ConnectionParameter>,
    }

    /// Reply for [`LoadConnectionParameters`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply LoadConnectionParametersReply {
    }

    /// Read Unconfigured Controller Index List Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Default, New)]
    pub command ReadUnconfiguredControllerIndexList: 0x0036 {
    }

    /// Reply for [`ReadUnconfiguredControllerIndexList`]
    #[derive(Debug)]
    pub reply ReadUnconfiguredControllerIndexListReply {
        values: Vec<ControllerIndex>,
    }

    /// Read Controller Configuration Information Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Default, New)]
    pub command ReadControllerConfigurationInformation: 0x0037 {
    }

    /// Reply for [`ReadControllerConfigurationInformation`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply ReadControllerConfigurationInformationReply {
        manufacture: u16,
        supported_options: super::ControllerConfigurationOption,
        missing_options: super::ControllerConfigurationOption,
    }

    /// Set External Configuration Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command SetExternalConfiguration: 0x0038 {
        configuration: bool,
    }

    /// Reply for [`SetExternalConfiguration`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply SetExternalConfigurationReply {
        missing_options: super::ControllerConfigurationOption,
    }

    /// Set Public Address Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command SetPublicAddress: 0x0039 {
        address: super::Address,
    }

    /// Reply for [`SetPublicAddress`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply SetPublicAddressReply {
        missing_options: super::ControllerConfigurationOption,
    }

    /// Start Service Discovery Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug)]
    pub command StartServiceDiscovery: 0x003A {
        address_type: HashSet<super::AddressType>,
        rssi_threshold: u8,
        uuids: Vec<super::Uuid>,
    }

    /// Reply for [`StartServiceDiscovery`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply StartServiceDiscoveryReply {
        address_type: HashSet<super::AddressType>,
    }

    /// Read Local Out Of Band Extended Data Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command ReadLocalOutOfBandExtendedData: 0x003B {
        address_type: HashSet<super::AddressType>,
    }

    /// Reply for [`ReadLocalOutOfBandExtendedData`]
    #[derive(Debug, Getters)]
    pub reply ReadLocalOutOfBandExtendedDataReply {
        #[get="pub"]
        address_type: HashSet<super::AddressType>,
        eir_data: super::VariableLengthBytes,
    }

    /// Read Management Version Information Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Default, New)]
    pub command ReadExtendedControllerIndexList: 0x003C {
    }

    /// Reply for [`ReadExtendedControllerIndexList`]
    #[derive(Debug)]
    pub reply ReadExtendedControllerIndexListReply {
        values: Vec<(ControllerIndex, super::ControllerType, super::ControllerBus)>,
    }

    /// Read Advertising Features Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Default, New)]
    pub command ReadAdvertisingFeature: 0x003D {
    }

    /// Reply for [`ReadAdvertisingFeature`]
    #[derive(Debug, Getters)]
    pub reply ReadAdvertisingFeatureReply {
        #[get="pub"]
        supported_flags: super::AdvertisingFlag,
        #[get="pub"]
        max_adv_data_len: u8,
        #[get="pub"]
        max_scan_resp_len: u8,
        #[get="pub"]
        max_instances: u8,
        instances: super::AdvertiseInstances,
    }

    /// Add Advertising Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug)]
    pub command AddAdvertising: 0x003E {
        instance: super::AdvertiseInstance,
        flags: super::AdvertisingFlag,
        duration: u16,
        timeout: u16,
        adv_data_scan_resp: super::AdvDataScanResp,
    }

    /// Reply for [`AddAdvertising`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply AddAdvertisingReply {
        instance: super::AdvertiseInstance,
    }

    /// Remove Advertising Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command RemoveAdvertising: 0x003F {
        instance: super::AdvertiseInstance,
    }

    /// Reply for [`RemoveAdvertising`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply RemoveAdvertisingReply {
        instance: super::AdvertiseInstance,
    }

    /// Get Advertising Size Information Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command GetAdvertisingSizeInformation: 0x0040 {
        instance: super::AdvertiseInstance,
        flags: super::AdvertisingFlag,
    }

    /// Reply for [`GetAdvertisingSizeInformation`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply GetAdvertisingSizeInformationReply {
        instance: super::AdvertiseInstance,
        flags: super::AdvertisingFlag,
        max_adv_data_len: u8,
        max_scan_resp_len: u8,
    }

    /// Start Limited Discovery Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command StartLimitedDiscovery: 0x0041 {
        address_type: HashSet<super::AddressType>,
    }

    /// Reply for [`StartLimitedDiscovery`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply StartLimitedDiscoveryReply {
        address_type: HashSet<super::AddressType>,
    }

    /// Read Extended Controller Information Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Default, New)]
    pub command ReadExtendedControllerInformation: 0x0042 {
    }

    /// Reply for [`ReadExtendedControllerInformation`]
    #[derive(Debug, Getters)]
    pub reply ReadExtendedControllerInformationReply {
        #[get="pub"]
        address: super::Address,
        #[get="pub"]
        bluetooth_version: u8,
        #[get="pub"]
        manufacturer: u16,
        #[get="pub"]
        supported_settings: super::Settings,
        #[get="pub"]
        current_settings: super::Settings,
        eir_data: super::VariableLengthBytes,
    }

    /// Set Appearance Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Default, New)]
    pub command SetApperance: 0x0043 {
        appearance: u16,
    }

    /// Reply for [`SetApperance`]
    #[derive(Debug)]
    pub reply SetApperanceReply {
    }

    /// Get PHY Configuration Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Default, New)]
    pub command GetPhyConfiguration: 0x0044 {
    }

    /// Reply for [`GetPhyConfiguration`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply GetPhyConfigurationReply {
        supported_phys: super::Phys,
        configurable_phys: super::Phys,
        selected_phys: super::Phys,
    }

    /// Set PHY Configuration Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command SetPhyConfiguration: 0x0045 {
        selected_phys: super::Phys,
    }

    /// Reply for [`SetPhyConfiguration`]
    #[derive(Debug)]
    pub reply SetPhyConfigurationReply {
    }

    /// Load Blocked Keys Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug)]
    pub command LoadBlockedKeys: 0x0046 {
        keys: Vec<super::BlockedKey>,
    }

    /// Reply for [`LoadBlockedKeys`]
    #[derive(Debug)]
    pub reply LoadBlockedKeysReply {
    }

    /// Set Wideband Speech Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command SetWidbandSpeech: 0x0047 {
        widband_speech: bool,
    }

    /// Reply for [`SetWidbandSpeech`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply SetWidbandSpeechReply {
        current_settings: super::Settings,
    }

    /// Read Security Information Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Default, New)]
    pub command ReadSecurityInformation: 0x0048 {
    }

    /// Reply for [`ReadSecurityInformation`]
    #[derive(Debug)]
    pub reply ReadSecurityInformationReply {
        security_data: super::VariableLengthBytes,
    }

    /// Read Experimental Features Information Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Default, New)]
    pub command ReadExperimentalFeaturesInformation: 0x0049 {
    }

    /// Reply for [`ReadExperimentalFeaturesInformation`]
    #[derive(Debug)]
    pub reply ReadExperimentalFeaturesInformationReply {
        features: Vec<(super::Uuid, super::FeatureFlags)>,
    }

    /// Set Experimental Feature Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command SetExperimentalFeature: 0x004A {
        uuid: super::Uuid,
        action: super::FeatureAction,
    }

    /// Reply for [`SetExperimentalFeature`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply SetExperimentalFeatureReply {
        uuid: super::Uuid,
        flags: super::FeatureFlags,
    }

    /// Read Default System Configuration Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Default, New)]
    pub command ReadDefaultSystemConfiguration: 0x004B {
    }

    /// Reply for [`ReadDefaultSystemConfiguration`]
    #[derive(Debug)]
    pub reply ReadDefaultSystemConfigurationReply {
        values: super::Remaining<super::SystemConfigurationParameter>,
    }

    /// Set Default System Configuration Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug)]
    pub command SetDefaultSystemConfiguration: 0x004C {
        values: super::Remaining<super::SystemConfigurationParameter>,
    }

    /// Reply for [`SetDefaultSystemConfiguration`]
    #[derive(Debug)]
    pub reply SetDefaultSystemConfigurationReply {
    }

    /// Read Default Runtime Configuration Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Default, New)]
    pub command ReadDefaultRuntimeConfiguration: 0x004D {
    }

    /// Reply for [`ReadDefaultRuntimeConfiguration`]
    #[derive(Debug)]
    pub reply ReadDefaultRuntimeConfigurationReply {
        values: super::Remaining<super::RuntimeConfigurationParameter>,
    }

    /// Read Management Version Information Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug)]
    pub command SetDefaultRuntimeConfiguration: 0x004E {
        values: super::Remaining<super::RuntimeConfigurationParameter>,
    }

    /// Reply for [`SetDefaultRuntimeConfiguration`]
    #[derive(Debug)]
    pub reply SetDefaultRuntimeConfigurationReply {
    }

    /// Get Device Flags Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command GetDeviceFlag: 0x004F {
        addrss: super::Address,
        address_type: super::AddressType,
    }

    /// Reply for [`GetDeviceFlag`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply GetDeviceFlagReply {
        addrss: super::Address,
        address_type: super::AddressType,
        supported_flags: super::DeviceFlags,
        current_flags: super::DeviceFlags,
    }

    /// Set Device Flags Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command SetDeviceFlag: 0x0050 {
        addrss: super::Address,
        address_type: super::AddressType,
        current_flags: super::DeviceFlags,
    }

    /// Reply for [`SetDeviceFlag`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply SetDeviceFlagReply {
        addrss: super::Address,
        address_type: super::AddressType,
    }

    /// Read Advertisement Monitor Features Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Default, New)]
    pub command ReadAdvertisementMonitorFeatures: 0x0051 {
    }

    /// Reply for [`ReadAdvertisementMonitorFeatures`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply ReadAdvertisementMonitorFeaturesReply {
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
    #[derive(Debug)]
    pub command AddAdvertisementPatternsMonitor: 0x0052 {
        patterns: Vec<super::AdvertisementPattern>,
    }

    /// Reply for [`AddAdvertisementPatternsMonitor`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply AddAdvertisementPatternsMonitorReply {
        monitor_handle: super::AdvertisementMonitorHandle,
    }

    /// Remove Advertisement Monitor Command
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, New)]
    pub command RemoveAdvertisementPatternsMonitor: 0x0053 {
        monitor_handle: super::AdvertisementMonitorHandle,
    }

    /// Reply for [`RemoveAdvertisementPatternsMonitor`]
    #[derive(Debug, Getters)]
    #[get="pub"]
    pub reply RemoveAdvertisementPatternsMonitorReply {
        monitor_handle: super::AdvertisementMonitorHandle,
    }

}

impl ReadManagementSupportedCommandsReply {
    pub fn commands(&self) -> &[CommandCode] {
        &self.value.commands()
    }

    pub fn events(&self) -> &[crate::packet::event::EventCode] {
        &self.value.events()
    }
}

impl AddAdvertising {
    pub fn new<A, B>(
        instance: super::AdvertiseInstance,
        flags: super::AdvertisingFlag,
        duration: u16,
        timeout: u16,
        adv_data: A,
        scan_resp: B,
    ) -> Self
    where
        A: Into<Vec<u8>>,
        B: Into<Vec<u8>>,
    {
        Self {
            instance,
            flags,
            duration,
            timeout,
            adv_data_scan_resp: (adv_data.into(), scan_resp.into()).into(),
        }
    }
}

impl PinCodeReply {
    pub fn new(address: super::Address, address_type: super::AddressType, pin_code: &[u8]) -> Self {
        let pin_length = pin_code.len() as u8; // FIXME check length
        let mut code = [0; 16];
        code[0..pin_code.len()].copy_from_slice(&pin_code);
        Self {
            address,
            address_type,
            pin_length,
            pin_code: code,
        }
    }
}

impl ReadAdvertisingFeatureReply {
    pub fn instances(&self) -> &[super::AdvertiseInstance] {
        &self.instances.0
    }
}

impl ReadExtendedControllerInformationReply {
    pub fn eir_data(&self) -> &[u8] {
        &self.eir_data.0
    }
}

impl ReadLocalOutOfBandExtendedDataReply {
    pub fn eir_data(&self) -> &[u8] {
        &self.eir_data.0
    }
}

impl ReadSecurityInformationReply {
    pub fn security_data(&self) -> &[u8] {
        &self.security_data.0
    }
}

impl StartServiceDiscovery {
    pub fn new<T: std::iter::IntoIterator<Item = super::Uuid>>(
        address_type: HashSet<super::AddressType>,
        rssi_threshold: u8,
        uuids: T,
    ) -> Self {
        Self {
            address_type,
            rssi_threshold,
            uuids: uuids.into_iter().collect(),
        }
    }
}

/// Management API Command
pub trait Command: Pack {
    const CODE: CommandCode;
    type Reply: Unpack;
}

#[derive(Debug)]
pub(crate) struct CommandInternal<P> {
    index: ControllerIndex,
    parameter: P,
}

impl<P> From<(ControllerIndex, P)> for CommandInternal<P>
where
    P: Command,
{
    fn from((index, parameter): (ControllerIndex, P)) -> Self {
        Self { index, parameter }
    }
}

impl<P> Pack for CommandInternal<P>
where
    P: Command,
{
    fn pack(self, buf: &mut BytesMut) {
        let Self { index, parameter } = self;

        P::CODE.pack(buf);
        index.pack(buf);

        buf.reserve(buf.len() + 2);
        let mut content = buf.split_off(buf.len() + 2);
        parameter.pack(&mut content);

        <BytesMut as bytes::BufMut>::put_u16_le(buf, content.len() as u16);
        buf.unsplit(content);
    }
}
