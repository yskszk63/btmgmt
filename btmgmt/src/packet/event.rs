//! mgmt API events.
use std::collections::HashSet;

use bytes::{Buf, Bytes};
use getset::Getters;

use super::{ControllerIndex, ErrorCode};
use crate::pack::{Error as UnpackError, Result, Unpack};

mod impl_into_iter;

macro_rules! event {
    (
        $(
            $(#[$attrs:meta])*
            $vis:vis event $name:ident : $code:literal {
                $(
                    $(#[$fattrs:meta])*
                    $fvis:vis $fname:ident : $fty:ty,
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

            impl $name {
                pub const CODE: EventCode = EventCode::$name;
            }
        )*

        packable_enum! {
            /// Event Code
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            pub enum EventCode: u16 {
                $($name => $code,)*
            }
        }

        /// Management API Event
        #[derive(Debug, Clone)]
        #[non_exhaustive]
        pub enum Event {
            $($name($name),)*
            Unknown(u16, Bytes),
        }

        fn unpack_event<B: Buf>(code: EventCode, buf: &mut B) -> Result<Event> {
            Ok(match code {
                $($name::CODE => Event::$name(<$name as Unpack>::unpack(buf)?),)*
            })
        }
    }
}

event! {
    /// Command Complete Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone)]
    pub event CommandComplete: 0x0001 {
        pub(crate) opcode: crate::packet::command::CommandCode,
        pub(crate) status: ErrorCode,
        pub(crate) data: Bytes,
    }

    /// Command Status Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone)]
    pub event CommandStatus: 0x0002 {
        pub(crate) opcode: crate::packet::command::CommandCode,
        pub(crate) status: ErrorCode,
    }

    /// Controller Error Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event ControllerError: 0x0003 {
        status: ErrorCode,
    }

    /// Index Added Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone)]
    pub event IndexAdded: 0x0004 {
    }

    /// Index Removed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone)]
    pub event IndexRemoved: 0x0005 {
    }

    /// New Settings Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event NewSettings: 0x0006 {
        current_settings: super::Settings,
    }

    /// Class Of Device Changed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event ClassOfDeviceChanged: 0x0007 {
        class_of_device: super::ClassOfDevice,
    }

    /// Local Name Changed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event LocalNameChanged: 0x0008 {
        name: super::Name,
        short_name: super::ShortName,
    }

    /// New Link Key Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event NewLinkKey: 0x0009 {
        store_hint: bool,
        key: super::LinkKey,
    }

    /// New Long Term Key Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event NewLongTermKey: 0x000A {
        store_hint: bool,
        key: super::LongTermKey,
    }

    /// Device Connected Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    pub event DeviceConnected: 0x000B {
        #[get="pub"]
        address: super::Address,
        #[get="pub"]
        address_type: super::AddressType,
        #[get="pub"]
        flags: super::DeviceConnectFlags,
        eir_data: super::VariableLengthBytes,
    }

    /// Device Disconnected Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event DeviceDisconnect: 0x000C {
        address: super::Address,
        address_type: super::AddressType,
        reason: super::DeviceDisconnectReason,
    }

    /// Connect Failed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event ConnectFailed: 0x000D {
        address: super::Address,
        address_type: super::AddressType,
        status: super::ErrorCode,
    }

    /// PIN Code Request Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event PinCodeRequest: 0x000E {
        address: super::Address,
        address_type: super::AddressType,
        secure: bool,
    }

    /// User Confirmation Request Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event UserConfirmationRequest: 0x000F {
        address: super::Address,
        address_type: super::AddressType,
        confirm_hint: super::ConfirmHint,
        value: [u8; 4],
    }

    /// User Passkey Request Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event UserPasskey: 0x0010 {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Authentication Failed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event AuthenticationFailed: 0x0011 {
        address: super::Address,
        address_type: super::AddressType,
        status: super::ErrorCode,
    }

    /// Device Found Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    pub event DeviceFound: 0x0012 {
        #[get="pub"]
        address: super::Address,
        #[get="pub"]
        address_type: super::AddressType,
        #[get="pub"]
        rssi: u8,
        #[get="pub"]
        flags: super::DeviceConnectFlags,
        eir_data: super::VariableLengthBytes,
    }

    /// Discovering Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event Discovering: 0x0013 {
        address_type: HashSet<super::AddressType>,
        discovering: bool,
    }

    /// Device Blocked Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event DeviceBlocked: 0x0014 {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Device Unblocked Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event DeviceUnblocked: 0x0015 {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Device Unpaired Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event DeviceUnpaired: 0x0016 {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// Passkey Notify Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event PasskeyNotify: 0x0017 {
        address: super::Address,
        address_type: super::AddressType,
        passkey: [u8; 4],
        entered: u8,
    }

    /// New Identity Resolving Key Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event NewIdentityResolvingKey: 0x0018 {
        store_hint: bool,
        random_address: super::Address,
        key: super::IdentityResolvingKey,
    }

    /// New Signature Resolving Key Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event NewSignatureResolvingKey: 0x0019 {
        store_hint: bool,
        key: super::SignatureResolvingKey,
    }

    /// Device Added Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event DeviceAdded: 0x001A {
        address: super::Address,
        address_type: super::AddressType,
        action: super::Action,
    }

    /// Device Removed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event DeviceRemoved: 0x001B {
        address: super::Address,
        address_type: super::AddressType,
    }

    /// New Connection Parameter Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event NewConnectionParameter: 0x001C {
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
    #[derive(Debug, Clone)]
    pub event UnconfiguredIndexAdded: 0x001D {
    }

    /// Unconfigured Index Removed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone)]
    pub event UnconfiguredIndexRemoved: 0x001E {
    }

    /// New Configuration Options Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event NewConfigurationOptions: 0x001F {
        missing_options: super::ControllerConfigurationOption,
    }

    /// Extended Index Added Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event ExtendedIndexAdded: 0x0020 {
        controller_type: super::ControllerType,
        controller_bus: super::ControllerBus,
    }

    /// Extended Index Removed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event ExtendedIndexRemoved: 0x0021 {
        controller_type: super::ControllerType,
        controller_bus: super::ControllerBus,
    }

    /// Local Out Of Band Extended Data Updated Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    pub event LocalOutOfBandExtendedDataUpdate: 0x0022 {
        #[get="pub"]
        address_type: HashSet<super::AddressType>,
        eir_data: super::VariableLengthBytes,
    }

    /// Advertising Added Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event AdvertisingAdded: 0x0023 {
        instance: super::AdvertiseInstance,
    }

    /// Advertising Removed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event AdvertisingRemoved: 0x0024 {
        instance: super::AdvertiseInstance,
    }

    /// Extended Controller Information Changed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone)]
    pub event ExtendedControllerInformationChanged: 0x0025 {
        eir_data: super::VariableLengthBytes,
    }

    /// PHY Configuration Changed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event PhyConfigurationChanged: 0x0026 {
        selected_phys: super::Phys,
    }

    /// Experimental Feature Changed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event ExperimentalFeatureChanged: 0x0027 {
        uuid: super::Uuid,
        flags: super::FeatureFlags,
    }

    /// Default System Configuration Changed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone)]
    pub event DefaultSystemConfigurationChanged: 0x0028 {
        parameters: super::Remaining<super::SystemConfigurationParameter>,
    }

    /// Default Runtime Configuration Changed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone)]
    pub event DefaultRuntimeConfigurationChanged: 0x0029 {
        parameters: super::Remaining<super::RuntimeConfigurationParameter>,
    }

    /// Device Flags Changed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event DeviceFlagsChanged: 0x002A {
        address: super::Address,
        address_type: super::AddressType,
        supported_flags: super::DeviceFlags,
        current_flags: super::DeviceFlags,
    }

    /// Advertisement Monitor Added Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event AdvertisementMonitorAdded: 0x002B {
        monitor_handle: super::AdvertisementMonitorHandle,
    }

    /// Advertisement Monitor Removed Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event AdvertisementMonitorRemoved: 0x002C {
        monitor_handle: super::AdvertisementMonitorHandle,
    }

    /// Controller Suspend Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event ControllerSuspend: 0x002D {
        suspend_state: super::SuspendState,
    }

    /// Controller Resume Event
    ///
    /// see [bluez
    /// docs/mgmt-api.txt](https://git.kernel.org/pub/scm/bluetooth/bluez.git/plain/doc/mgmt-api.txt)
    #[derive(Debug, Clone, Getters)]
    #[get="pub"]
    pub event ControllerResume: 0x002E {
        wake_reason: super::WakeReason,
        address: super::Address,
        address_type: super::AddressType,
    }

}

impl DeviceConnected {
    pub fn eir_data(&self) -> &[u8] {
        &self.eir_data.0
    }
}

impl DeviceFound {
    pub fn eir_data(&self) -> &[u8] {
        &self.eir_data.0
    }
}

impl ExtendedControllerInformationChanged {
    pub fn eir_data(&self) -> &[u8] {
        &self.eir_data.0
    }
}

impl LocalOutOfBandExtendedDataUpdate {
    pub fn eir_data(&self) -> &[u8] {
        &self.eir_data.0
    }
}

impl Unpack for (ControllerIndex, Event) {
    fn unpack<B: Buf>(buf: &mut B) -> Result<Self> {
        let maybe_code = EventCode::unpack(buf);
        if let Err(UnpackError::Overflow) = maybe_code {
            return Err(UnpackError::Overflow);
        };
        let index = ControllerIndex::unpack(buf)?;

        let len = u16::unpack(buf)? as usize;
        let mut content = buf.take(len);
        let event = unpack_event(maybe_code.unwrap(), &mut content)?; // FIXME
        content.advance(content.remaining());
        Ok((index, event))
    }
}
