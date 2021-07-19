//! mgmt API events.
use getset::Getters;

use btmgmt_packet_helper::events;

pub use imp::*;
use super::*;

/// Management API Events
#[events(name = Event, codes = EventCode)]
mod imp {
    use super::*;

    /// Command Complete Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x0001)]
    #[getset(get = "pub")]
    pub struct CommandComplete {
        opcode: crate::command::CommandCode,
        status: ErrorCode,
        data: Box<[u8]>,
    }

    /// Command Status Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x0002)]
    #[getset(get = "pub")]
    pub struct CommandStatus {
        pub opcode: crate::command::CommandCode,
        pub status: ErrorCode,
    }

    /// Controller Error Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Newtype)]
    #[event(0x0003)]
    pub struct ControllerError(ErrorCode);

    /// Index Added Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x0004)]
    pub struct IndexAdded;

    /// Index Removed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x0005)]
    pub struct IndexRemoved;

    /// New Settings Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Newtype)]
    #[event(0x0006)]
    pub struct NewSettings(super::Settings);

    /// Class Of Device Changed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Newtype)]
    #[event(0x0007)]
    pub struct ClassOfDeviceChanged(super::ClassOfDevice);

    /// Local Name Changed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x0008)]
    #[getset(get = "pub")]
    pub struct LocalNameChanged {
        name: super::Name,
        short_name: super::ShortName,
    }


    /// New Link Key Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x0009)]
    #[getset(get = "pub")]
    pub struct NewLinkKey {
        store_hint: bool,
        key: super::LinkKey,
    }

    /// New Long Term Key Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x000A)]
    #[getset(get = "pub")]
    pub struct NewLongTermKey {
        store_hint: bool,
        key: super::LongTermKey,
    }

    /// Device Connected Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x000B)]
    #[getset(get = "pub")]
    pub struct DeviceConnected {
        address: super::Address,
        address_type: super::AddressType,
        flags: super::DeviceConnectFlags,
        eir_data: super::VariableLengthBytes,
    }

    /// Device Disconnected Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x000C)]
    #[getset(get = "pub")]
    pub struct DeviceDisconnect {
        address: super::Address,
        address_type: super::AddressType,
        reason: super::DeviceDisconnectReason,
    }

    /// Connect Failed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x000D)]
    #[getset(get = "pub")]
    pub struct ConnectFailed {
        address: super::Address,
        address_type: super::AddressType,
        status: super::ErrorCode,
    }

    /// PIN Code Request Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x000E)]
    #[getset(get = "pub")]
    pub struct PinCodeRequest {
        address: super::Address,
        address_type: super::AddressType,
        secure: bool,
    }

    /// User Confirmation Request Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x000F)]
    #[getset(get = "pub")]
    pub struct UserConfirmationRequest {
        address: super::Address,
        address_type: super::AddressType,
        confirm_hint: super::ConfirmHint,
        value: [u8; 4],
    }

    /// User Passkey Request Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x0010)]
    #[getset(get = "pub")]
    pub struct UserPasskeyRequest {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Authentication Failed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x0011)]
    #[getset(get = "pub")]
    pub struct AuthenticationFailed {
        address: super::Address,
        address_type: super::AddressType,
        status: super::ErrorCode,
    }

    /// Device Found Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x0012)]
    #[getset(get = "pub")]
    pub struct DeviceFound {
        address: super::Address,
        address_type: super::AddressType,
        rssi: u8,
        flags: super::DeviceConnectFlags,
        eir_data: super::VariableLengthBytes,
    }

    /// Discovering Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x0013)]
    #[getset(get = "pub")]
    pub struct Discovering {
        address_type: super::AddressTypes,
        discovering: bool,
    }

    /// Device Blocked Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x0014)]
    #[getset(get = "pub")]
    pub struct DeviceBlocked {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Device Unblocked Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x0015)]
    #[getset(get = "pub")]
    pub struct DeviceUnblocked {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Device Unpaired Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x0016)]
    #[getset(get = "pub")]
    pub struct DeviceUnpaired {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Passkey Notify Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x0017)]
    #[getset(get = "pub")]
    pub struct PasskeyNotify {
        address: super::Address,
        address_type: super::AddressType,
        passkey: u32,
        entered: u8,
    }

    /// New Identity Resolving Key Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x0018)]
    #[getset(get = "pub")]
    pub struct NewIdentityResolvingKey {
        store_hint: bool,
        random_address: super::Address,
        key: super::IdentityResolvingKey,
    }

    /// New Signature Resolving Key Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x0019)]
    #[getset(get = "pub")]
    pub struct NewSignatureResolvingKey {
        store_hint: bool,
        key: super::SignatureResolvingKey,
    }

    /// Device Added Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x001A)]
    #[getset(get = "pub")]
    pub struct DeviceAdded {
        address: super::Address,
        address_type: super::AddressType,
        action: super::Action,
    }

    /// Device Removed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x001B)]
    #[getset(get = "pub")]
    pub struct DeviceRemoved {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// New Connection Parameter Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x001C)]
    #[getset(get = "pub")]
    pub struct NewConnectionParameter {
        address: super::Address,
        address_type: super::AddressType,
        min_connection_interval: u16,
        max_connection_interval: u16,
        connection_latency: u16,
        supervision_timeout: u16,
    }

    /// Unconfigured Index Added Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x001D)]
    pub struct UnconfiguredIndexAdded;

    /// Unconfigured Index Removed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x001E)]
    pub struct UnconfiguredIndexRemoved;

    /// New Configuration Options Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Newtype)]
    #[event(0x001F)]
    pub struct NewConfigurationOptions(super::ControllerConfigurationOption);

    /// Extended Index Added Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x0020)]
    #[getset(get = "pub")]
    pub struct ExtendedIndexAdded {
        controller_type: super::ControllerType,
        controller_bus: super::ControllerBus,
    }

    /// Extended Index Removed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x0021)]
    #[getset(get = "pub")]
    pub struct ExtendedIndexRemoved {
        controller_type: super::ControllerType,
        controller_bus: super::ControllerBus,
    }

    /// Local Out Of Band Extended Data Updated Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x0022)]
    #[getset(get = "pub")]
    pub struct LocalOutOfBandExtendedDataUpdate {
        address_type: super::AddressTypes,
        eir_data: super::VariableLengthBytes,
    }

    /// Advertising Added Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Newtype)]
    #[event(0x0023)]
    pub struct AdvertisingAdded(super::AdvertiseInstance);

    /// Advertising Removed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Newtype)]
    #[event(0x0024)]
    pub struct AdvertisingRemoved(super::AdvertiseInstance);

    /// Extended Controller Information Changed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Newtype)]
    #[event(0x0025)]
    pub struct ExtendedControllerInformationChanged(super::VariableLengthBytes);

    /// PHY Configuration Changed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Newtype)]
    #[event(0x0026)]
    pub struct PhyConfigurationChanged(super::Phys);

    /// Experimental Feature Changed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x0027)]
    #[getset(get = "pub")]
    pub struct ExperimentalFeatureChanged {
        uuid: super::Uuid,
        flags: super::FeatureFlags,
    }

    /// Default System Configuration Changed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, IterNewtype)]
    #[event(0x0028)]
    pub struct DefaultSystemConfigurationChanged(super::Remaining<super::SystemConfigurationParameter>);

    /// Default Runtime Configuration Changed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, IterNewtype)]
    #[event(0x0029)]
    pub struct DefaultRuntimeConfigurationChanged(super::Remaining<super::RuntimeConfigurationParameter>);

    /// Device Flags Changed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x002A)]
    #[getset(get = "pub")]
    pub struct DeviceFlagsChanged {
        address: super::Address,
        address_type: super::AddressType,
        supported_flags: super::DeviceFlags,
        current_flags: super::DeviceFlags,
    }

    /// Advertisement Monitor Added Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Newtype)]
    #[event(0x002B)]
    pub struct AdvertisementMonitorAdded(super::AdvertisementMonitorHandle);

    /// Advertisement Monitor Removed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Newtype)]
    #[event(0x002C)]
    pub struct AdvertisementMonitorRemoved(super::AdvertisementMonitorHandle);

    /// Controller Suspend Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Newtype)]
    #[event(0x002D)]
    pub struct ControllerSuspend(super::SuspendState);

    /// Controller Resume Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x002E)]
    #[getset(get = "pub")]
    pub struct ControllerResume {
        wake_reason: super::WakeReason,
        address: super::Address,
        address_type: super::AddressType,
    }

}

#[doc(hidden)]
pub fn unpack_events<R>(read: &mut R) -> pack::Result<(ControllerIndex, Event)> where R: io::Read {
    let code = EventCode::unpack(read)?;
    let index = ControllerIndex::unpack(read)?;

    let data = <Vec<u8>>::unpack(read)?;
    let events = Event::unpack_inner(code, &mut &data[..])?;

    Ok((index, events))
}
