//! mgmt API events.
use getset::Getters;

use btmgmt_packet_helper::events;

use super::*;
pub use imp::*;

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
    pub struct DeviceConnected {
        address: super::WrappedAddress,
        address_type: super::InternalAddressType,
        #[getset(get = "pub")]
        flags: super::DeviceConnectFlags,
        #[getset(get = "pub")]
        eir_data: super::VariableLengthBytes,
    }

    impl DeviceConnected {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// Device Disconnected Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x000C)]
    pub struct DeviceDisconnect {
        address: super::WrappedAddress,
        address_type: super::InternalAddressType,
        #[getset(get = "pub")]
        reason: super::DeviceDisconnectReason,
    }

    impl DeviceDisconnect {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// Connect Failed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x000D)]
    pub struct ConnectFailed {
        address: super::WrappedAddress,
        address_type: super::InternalAddressType,
        #[getset(get = "pub")]
        status: super::ErrorCode,
    }

    impl ConnectFailed {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// PIN Code Request Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x000E)]
    pub struct PinCodeRequest {
        address: super::WrappedAddress,
        address_type: super::InternalAddressType,
        #[getset(get = "pub")]
        secure: bool,
    }

    impl PinCodeRequest {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// User Confirmation Request Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x000F)]
    pub struct UserConfirmationRequest {
        address: super::WrappedAddress,
        address_type: super::InternalAddressType,
        #[getset(get = "pub")]
        confirm_hint: super::ConfirmHint,
        #[getset(get = "pub")]
        value: [u8; 4],
    }

    impl UserConfirmationRequest {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// User Passkey Request Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x0010)]
    pub struct UserPasskeyRequest {
        address: super::WrappedAddress,
        address_type: super::InternalAddressType,
    }

    impl UserPasskeyRequest {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// Authentication Failed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x0011)]
    pub struct AuthenticationFailed {
        address: super::WrappedAddress,
        address_type: super::InternalAddressType,
        #[getset(get = "pub")]
        status: super::ErrorCode,
    }

    impl AuthenticationFailed {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// Device Found Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x0012)]
    pub struct DeviceFound {
        address: super::WrappedAddress,
        address_type: super::InternalAddressType,
        #[getset(get = "pub")]
        rssi: u8,
        #[getset(get = "pub")]
        flags: super::DeviceConnectFlags,
        #[getset(get = "pub")]
        eir_data: super::VariableLengthBytes,
    }

    impl DeviceFound {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
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
    #[derive(Debug, Clone, Unpack)]
    #[event(0x0014)]
    pub struct DeviceBlocked {
        address: super::WrappedAddress,
        address_type: super::InternalAddressType,
    }

    impl DeviceBlocked {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// Device Unblocked Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x0015)]
    pub struct DeviceUnblocked {
        address: super::WrappedAddress,
        address_type: super::InternalAddressType,
    }

    impl DeviceUnblocked {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// Device Unpaired Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x0016)]
    pub struct DeviceUnpaired {
        address: super::WrappedAddress,
        address_type: super::InternalAddressType,
    }

    impl DeviceUnpaired {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// Passkey Notify Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x0017)]
    pub struct PasskeyNotify {
        address: super::WrappedAddress,
        address_type: super::InternalAddressType,
        #[getset(get = "pub")]
        passkey: u32,
        #[getset(get = "pub")]
        entered: u8,
    }

    impl PasskeyNotify {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// New Identity Resolving Key Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x0018)]
    pub struct NewIdentityResolvingKey {
        #[getset(get = "pub")]
        store_hint: bool,
        random_address: super::WrappedAddress,
        #[getset(get = "pub")]
        key: super::IdentityResolvingKey,
    }

    impl NewIdentityResolvingKey {
        pub fn address(&self) -> Address {
            self.random_address.0.clone().to_le_random_addr()
        }
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
    pub struct DeviceAdded {
        address: super::WrappedAddress,
        address_type: super::InternalAddressType,
        #[getset(get = "pub")]
        action: super::Action,
    }

    impl DeviceAdded {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// Device Removed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack)]
    #[event(0x001B)]
    pub struct DeviceRemoved {
        address: super::WrappedAddress,
        address_type: super::InternalAddressType,
    }

    impl DeviceRemoved {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }

    /// New Connection Parameter Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x001C)]
    pub struct NewConnectionParameter {
        address: super::WrappedAddress,
        address_type: super::InternalAddressType,
        #[getset(get = "pub")]
        min_connection_interval: u16,
        #[getset(get = "pub")]
        max_connection_interval: u16,
        #[getset(get = "pub")]
        connection_latency: u16,
        #[getset(get = "pub")]
        supervision_timeout: u16,
    }

    impl NewConnectionParameter {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
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
    pub struct DefaultSystemConfigurationChanged(
        super::Remaining<super::SystemConfigurationParameter>,
    );

    /// Default Runtime Configuration Changed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, IterNewtype)]
    #[event(0x0029)]
    pub struct DefaultRuntimeConfigurationChanged(
        super::Remaining<super::RuntimeConfigurationParameter>,
    );

    /// Device Flags Changed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Unpack, Getters)]
    #[event(0x002A)]
    pub struct DeviceFlagsChanged {
        address: super::WrappedAddress,
        address_type: super::InternalAddressType,
        #[getset(get = "pub")]
        supported_flags: super::DeviceFlags,
        #[getset(get = "pub")]
        current_flags: super::DeviceFlags,
    }

    impl DeviceFlagsChanged {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
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
    pub struct ControllerResume {
        #[getset(get = "pub")]
        wake_reason: super::WakeReason,
        address: super::WrappedAddress,
        address_type: super::InternalAddressType,
    }

    impl ControllerResume {
        pub fn address(&self) -> Address {
            join(&self.address_type, &self.address)
        }
    }
}

#[doc(hidden)]
pub fn unpack_events<R>(read: &mut R) -> pack::Result<(ControllerIndex, Event)>
where
    R: io::Read,
{
    let code = EventCode::unpack(read)?;
    let index = ControllerIndex::unpack(read)?;

    let data = <Vec<u8>>::unpack(read)?;
    let events = Event::unpack_inner(code, &mut &data[..])?;

    Ok((index, events))
}
