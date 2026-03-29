use num_traits::FromPrimitive;
use std::collections::HashMap;
use std::fmt;

use crate::report::Report;
use crate::smart_device::Device;
use crate::utils::trait_alias::RandomNumber;

pub trait Room<T: RandomNumber> {
    #[must_use]
    fn new(devices: HashMap<String, Device<T>>) -> Self;

    #[must_use]
    fn get_device(&self, key: &str) -> Option<&Device<T>>;

    #[must_use]
    fn get_device_mut(&mut self, key: &str) -> Option<&mut Device<T>>;

    fn add_device(&mut self, key: String, device: Device<T>);

    fn remove_device(&mut self, key: &str);
}

#[derive(Debug)]
pub struct SmartRoom<T: RandomNumber> {
    devices: HashMap<String, Device<T>>,
}

impl<T: RandomNumber + FromPrimitive + fmt::Debug> Room<T> for SmartRoom<T> {
    fn new(devices: HashMap<String, Device<T>>) -> Self {
        SmartRoom { devices }
    }

    fn get_device(&self, key: &str) -> Option<&Device<T>> {
        self.devices.get(key)
    }

    fn get_device_mut(&mut self, key: &str) -> Option<&mut Device<T>> {
        self.devices.get_mut(key)
    }

    fn add_device(&mut self, key: String, device: Device<T>) {
        self.devices.insert(key, device);
    }

    fn remove_device(&mut self, key: &str) {
        self.devices.remove(key);
    }
}

impl<T: RandomNumber + fmt::Debug> Report for SmartRoom<T> {
    fn report(&self) -> String {
        let mut result = String::from("");
        for (device_name, device) in &self.devices {
            result.push_str(&format!("Device '{}': {}\n", device_name, device.report()));
        }
        result
    }
}
