use std::iter::{Extend, FromIterator};

use super::*;
use crate::packet::{
    AdvertisementPattern, BlockedKey, ConnectionParameter, IdentityResolvingKey, LinkKey,
    LongTermKey, Remaining, RuntimeConfigurationParameter, SystemConfigurationParameter,
};

impl FromIterator<SystemConfigurationParameter> for SetDefaultSystemConfiguration {
    fn from_iter<T: IntoIterator<Item = SystemConfigurationParameter>>(iter: T) -> Self {
        Self {
            values: Remaining(iter.into_iter().collect()),
        }
    }
}

impl Extend<SystemConfigurationParameter> for SetDefaultSystemConfiguration {
    fn extend<T: IntoIterator<Item = SystemConfigurationParameter>>(&mut self, iter: T) {
        self.values.0.extend(iter)
    }
}

impl FromIterator<RuntimeConfigurationParameter> for SetDefaultRuntimeConfiguration {
    fn from_iter<T: IntoIterator<Item = RuntimeConfigurationParameter>>(iter: T) -> Self {
        Self {
            values: Remaining(iter.into_iter().collect()),
        }
    }
}

impl Extend<RuntimeConfigurationParameter> for SetDefaultRuntimeConfiguration {
    fn extend<T: IntoIterator<Item = RuntimeConfigurationParameter>>(&mut self, iter: T) {
        self.values.0.extend(iter)
    }
}

impl FromIterator<AdvertisementPattern> for AddAdvertisementPatternsMonitor {
    fn from_iter<T: IntoIterator<Item = AdvertisementPattern>>(iter: T) -> Self {
        Self {
            patterns: iter.into_iter().collect(),
        }
    }
}

impl Extend<AdvertisementPattern> for AddAdvertisementPatternsMonitor {
    fn extend<T: IntoIterator<Item = AdvertisementPattern>>(&mut self, iter: T) {
        self.patterns.extend(iter)
    }
}

impl FromIterator<BlockedKey> for LoadBlockedKeys {
    fn from_iter<T: IntoIterator<Item = BlockedKey>>(iter: T) -> Self {
        Self {
            keys: iter.into_iter().collect(),
        }
    }
}

impl Extend<BlockedKey> for LoadBlockedKeys {
    fn extend<T: IntoIterator<Item = BlockedKey>>(&mut self, iter: T) {
        self.keys.extend(iter)
    }
}

impl FromIterator<ConnectionParameter> for LoadConnectionParameters {
    fn from_iter<T: IntoIterator<Item = ConnectionParameter>>(iter: T) -> Self {
        Self {
            params: iter.into_iter().collect(),
        }
    }
}

impl Extend<ConnectionParameter> for LoadConnectionParameters {
    fn extend<T: IntoIterator<Item = ConnectionParameter>>(&mut self, iter: T) {
        self.params.extend(iter)
    }
}

impl FromIterator<IdentityResolvingKey> for LoadIdentityResolvingKeys {
    fn from_iter<T: IntoIterator<Item = IdentityResolvingKey>>(iter: T) -> Self {
        Self {
            keys: iter.into_iter().collect(),
        }
    }
}

impl Extend<IdentityResolvingKey> for LoadIdentityResolvingKeys {
    fn extend<T: IntoIterator<Item = IdentityResolvingKey>>(&mut self, iter: T) {
        self.keys.extend(iter)
    }
}

impl FromIterator<LinkKey> for LoadLinkKeys {
    fn from_iter<T: IntoIterator<Item = LinkKey>>(iter: T) -> Self {
        Self {
            debug_keys: false,
            keys: iter.into_iter().collect(),
        }
    }
}

impl Extend<LinkKey> for LoadLinkKeys {
    fn extend<T: IntoIterator<Item = LinkKey>>(&mut self, iter: T) {
        self.keys.extend(iter)
    }
}

impl FromIterator<LongTermKey> for LoadLongTermKey {
    fn from_iter<T: IntoIterator<Item = LongTermKey>>(iter: T) -> Self {
        Self {
            keys: iter.into_iter().collect(),
        }
    }
}

impl Extend<LongTermKey> for LoadLongTermKey {
    fn extend<T: IntoIterator<Item = LongTermKey>>(&mut self, iter: T) {
        self.keys.extend(iter)
    }
}
