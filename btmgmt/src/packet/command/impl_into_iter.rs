use super::*;
use crate::packet::{
    Address, AddressType, ControllerBus, ControllerType, FeatureFlags,
    RuntimeConfigurationParameter, SystemConfigurationParameter, Uuid,
};

impl IntoIterator for ReadControllerIndexListReply {
    type Item = ControllerIndex;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl<'a> IntoIterator for &'a ReadControllerIndexListReply {
    type Item = &'a ControllerIndex;
    type IntoIter = std::slice::Iter<'a, ControllerIndex>;
    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}

impl Deref for ReadControllerIndexListReply {
    type Target = [ControllerIndex];
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl IntoIterator for GetConnectionsReply {
    type Item = (Address, AddressType);
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl<'a> IntoIterator for &'a GetConnectionsReply {
    type Item = &'a (Address, AddressType);
    type IntoIter = std::slice::Iter<'a, (Address, AddressType)>;
    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}

impl Deref for GetConnectionsReply {
    type Target = [(Address, AddressType)];
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl IntoIterator for ReadDefaultSystemConfigurationReply {
    type Item = SystemConfigurationParameter;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.values.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a ReadDefaultSystemConfigurationReply {
    type Item = &'a SystemConfigurationParameter;
    type IntoIter = std::slice::Iter<'a, SystemConfigurationParameter>;
    fn into_iter(self) -> Self::IntoIter {
        (&self.values.0).iter()
    }
}

impl Deref for ReadDefaultSystemConfigurationReply {
    type Target = [SystemConfigurationParameter];
    fn deref(&self) -> &Self::Target {
        &self.values.0
    }
}

impl IntoIterator for ReadDefaultRuntimeConfigurationReply {
    type Item = RuntimeConfigurationParameter;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.values.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a ReadDefaultRuntimeConfigurationReply {
    type Item = &'a RuntimeConfigurationParameter;
    type IntoIter = std::slice::Iter<'a, RuntimeConfigurationParameter>;
    fn into_iter(self) -> Self::IntoIter {
        (&self.values.0).iter()
    }
}

impl Deref for ReadDefaultRuntimeConfigurationReply {
    type Target = [RuntimeConfigurationParameter];
    fn deref(&self) -> &Self::Target {
        &self.values.0
    }
}

impl IntoIterator for ReadExtendedControllerIndexListReply {
    type Item = (ControllerIndex, ControllerType, ControllerBus);
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl<'a> IntoIterator for &'a ReadExtendedControllerIndexListReply {
    type Item = &'a (ControllerIndex, ControllerType, ControllerBus);
    type IntoIter = std::slice::Iter<'a, (ControllerIndex, ControllerType, ControllerBus)>;
    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}

impl Deref for ReadExtendedControllerIndexListReply {
    type Target = [(ControllerIndex, ControllerType, ControllerBus)];
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl IntoIterator for ReadExperimentalFeaturesInformationReply {
    type Item = (Uuid, FeatureFlags);
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.features.into_iter()
    }
}

impl<'a> IntoIterator for &'a ReadExperimentalFeaturesInformationReply {
    type Item = &'a (Uuid, FeatureFlags);
    type IntoIter = std::slice::Iter<'a, (Uuid, FeatureFlags)>;
    fn into_iter(self) -> Self::IntoIter {
        self.features.iter()
    }
}

impl Deref for ReadExperimentalFeaturesInformationReply {
    type Target = [(Uuid, FeatureFlags)];
    fn deref(&self) -> &Self::Target {
        &self.features
    }
}

impl IntoIterator for ReadUnconfiguredControllerIndexListReply {
    type Item = ControllerIndex;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl<'a> IntoIterator for &'a ReadUnconfiguredControllerIndexListReply {
    type Item = &'a ControllerIndex;
    type IntoIter = std::slice::Iter<'a, ControllerIndex>;
    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}

impl Deref for ReadUnconfiguredControllerIndexListReply {
    type Target = [ControllerIndex];
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}
