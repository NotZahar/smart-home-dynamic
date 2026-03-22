use num_traits::FromPrimitive;
use std::fmt::Debug;

use crate::smart_device::{Device, PrintStateVisitor, PrintVisitor};
use crate::utils::trait_alias::RandomNumber;

pub trait Room<T> {
    #[must_use]
    fn new(devices: Vec<Box<dyn Device<T>>>) -> Self;

    #[must_use]
    fn get_device(&self, index: usize) -> &dyn Device<T>;

    #[must_use]
    fn get_device_mut(&mut self, index: usize) -> &mut dyn Device<T>;

    fn print_report(&mut self);
}

pub struct SmartRoom<T> {
    devices: Vec<Box<dyn Device<T>>>,
}

impl<T: RandomNumber + FromPrimitive + Debug> Room<T> for SmartRoom<T> {
    fn new(devices: Vec<Box<dyn Device<T>>>) -> Self {
        SmartRoom { devices }
    }

    fn get_device(&self, index: usize) -> &dyn Device<T> {
        self.devices[index].as_ref()
    }

    fn get_device_mut(&mut self, index: usize) -> &mut dyn Device<T> {
        self.devices[index].as_mut()
    }

    fn print_report(&mut self) {
        for (device_index, device) in self.devices.iter_mut().enumerate() {
            let mut visitor = PrintStateVisitor::new(device_index);
            device.accept(&mut visitor);
        }
    }
}
