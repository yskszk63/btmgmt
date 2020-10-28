use std::ops::Deref;

use super::*;
use crate::packet::{RuntimeConfigurationParameter, SystemConfigurationParameter};

impl IntoIterator for DefaultSystemConfigurationChanged {
    type Item = SystemConfigurationParameter;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.parameters.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a DefaultSystemConfigurationChanged {
    type Item = &'a SystemConfigurationParameter;
    type IntoIter = std::slice::Iter<'a, SystemConfigurationParameter>;
    fn into_iter(self) -> Self::IntoIter {
        self.parameters.0.iter()
    }
}

impl Deref for DefaultSystemConfigurationChanged {
    type Target = [SystemConfigurationParameter];
    fn deref(&self) -> &Self::Target {
        &self.parameters.0
    }
}

impl IntoIterator for DefaultRuntimeConfigurationChanged {
    type Item = RuntimeConfigurationParameter;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.parameters.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a DefaultRuntimeConfigurationChanged {
    type Item = &'a RuntimeConfigurationParameter;
    type IntoIter = std::slice::Iter<'a, RuntimeConfigurationParameter>;
    fn into_iter(self) -> Self::IntoIter {
        self.parameters.0.iter()
    }
}

impl Deref for DefaultRuntimeConfigurationChanged {
    type Target = [RuntimeConfigurationParameter];
    fn deref(&self) -> &Self::Target {
        &self.parameters.0
    }
}
