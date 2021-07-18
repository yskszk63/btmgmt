//! mgmt API events.
use btmgmt_packet_helper::events;

pub use imp::*;
use super::*;

/// Management API Events
#[events(name = Events, codes = EventCode)]
mod imp {
    use super::*;

    /// Command Complete Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x0001)]
    pub struct CommandComplete {
        pub opcode: crate::command::CommandCode,
        pub status: ErrorCode,
        pub data: Box<[u8]>,
    }

    /// Command Status Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x0002)]
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
    #[derive(Debug, Clone, Unpack)]
    #[event(0x0008)]
    pub struct LocalNameChanged {
        pub name: super::Name,
        pub short_name: super::ShortName,
    }


    /// New Link Key Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x0009)]
    pub struct NewLinkKey {
        pub store_hint: bool,
        pub key: super::LinkKey,
    }

    /// New Long Term Key Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x000A)]
    pub struct NewLongTermKey {
        pub store_hint: bool,
        pub key: super::LongTermKey,
    }

    /// Device Connected Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x000B)]
    pub struct DeviceConnected {
        pub address: super::Address,
        pub address_type: super::AddressType,
        pub flags: super::DeviceConnectFlags,
        pub eir_data: super::VariableLengthBytes,
    }

    /// Device Disconnected Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x000C)]
    pub struct DeviceDisconnect {
        pub address: super::Address,
        pub address_type: super::AddressType,
        pub reason: super::DeviceDisconnectReason,
    }

    /// Connect Failed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x000D)]
    pub struct ConnectFailed {
        pub address: super::Address,
        pub address_type: super::AddressType,
        pub status: super::ErrorCode,
    }

    /// PIN Code Request Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x000E)]
    pub struct PinCodeRequest {
        pub address: super::Address,
        pub address_type: super::AddressType,
        pub secure: bool,
    }

    /// User Confirmation Request Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x000F)]
    pub struct UserConfirmationRequest {
        pub address: super::Address,
        pub address_type: super::AddressType,
        pub confirm_hint: super::ConfirmHint,
        pub value: [u8; 4],
    }

    /// User Passkey Request Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x0010)]
    pub struct UserPasskeyRequest {
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// Authentication Failed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x0011)]
    pub struct AuthenticationFailed {
        pub address: super::Address,
        pub address_type: super::AddressType,
        pub status: super::ErrorCode,
    }

    /// Device Found Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x0012)]
    pub struct DeviceFound {
        pub address: super::Address,
        pub address_type: super::AddressType,
        pub rssi: u8,
        pub flags: super::DeviceConnectFlags,
        pub eir_data: super::VariableLengthBytes,
    }

    /// Discovering Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x0013)]
    pub struct Discovering {
        pub address_type: super::AddressTypes,
        pub discovering: bool,
    }

    /// Device Blocked Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x0014)]
    pub struct DeviceBlocked {
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// Device Unblocked Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x0015)]
    pub struct DeviceUnblocked {
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// Device Unpaired Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x0016)]
    pub struct DeviceUnpaired {
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// Passkey Notify Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x0017)]
    pub struct PasskeyNotify {
        pub address: super::Address,
        pub address_type: super::AddressType,
        pub passkey: u32,
        pub entered: u8,
    }

    /// New Identity Resolving Key Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x0018)]
    pub struct NewIdentityResolvingKey {
        pub store_hint: bool,
        pub random_address: super::Address,
        pub key: super::IdentityResolvingKey,
    }

    /// New Signature Resolving Key Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x0019)]
    pub struct NewSignatureResolvingKey {
        pub store_hint: bool,
        pub key: super::SignatureResolvingKey,
    }

    /// Device Added Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x001A)]
    pub struct DeviceAdded {
        pub address: super::Address,
        pub address_type: super::AddressType,
        pub action: super::Action,
    }

    /// Device Removed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x001B)]
    pub struct DeviceRemoved {
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

    /// New Connection Parameter Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x001C)]
    pub struct NewConnectionParameter {
        pub address: super::Address,
        pub address_type: super::AddressType,
        pub min_connection_interval: u16,
        pub max_connection_interval: u16,
        pub connection_latency: u16,
        pub supervision_timeout: u16,
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
    #[derive(Debug, Clone, Unpack)]
    #[event(0x0020)]
    pub struct ExtendedIndexAdded {
        pub controller_type: super::ControllerType,
        pub controller_bus: super::ControllerBus,
    }

    /// Extended Index Removed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x0021)]
    pub struct ExtendedIndexRemoved {
        pub controller_type: super::ControllerType,
        pub controller_bus: super::ControllerBus,
    }

    /// Local Out Of Band Extended Data Updated Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x0022)]
    pub struct LocalOutOfBandExtendedDataUpdate {
        pub address_type: super::AddressTypes,
        pub eir_data: super::VariableLengthBytes,
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
    #[derive(Debug, Clone, Unpack)]
    #[event(0x0027)]
    pub struct ExperimentalFeatureChanged {
        pub uuid: super::Uuid,
        pub flags: super::FeatureFlags,
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
    #[derive(Debug, Clone, Unpack)]
    #[event(0x002A)]
    pub struct DeviceFlagsChanged {
        pub address: super::Address,
        pub address_type: super::AddressType,
        pub supported_flags: super::DeviceFlags,
        pub current_flags: super::DeviceFlags,
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
    #[derive(Debug, Clone, Unpack)]
    #[event(0x002E)]
    pub struct ControllerResume {
        pub wake_reason: super::WakeReason,
        pub address: super::Address,
        pub address_type: super::AddressType,
    }

}

#[doc(hidden)]
pub fn unpack_events<R>(read: &mut R) -> pack::Result<(ControllerIndex, Events)> where R: io::Read {
    let code = EventCode::unpack(read)?;
    let index = ControllerIndex::unpack(read)?;

    let data = <Vec<u8>>::unpack(read)?;
    let events = Events::unpack_inner(code, &mut &data[..])?;

    Ok((index, events))
}
