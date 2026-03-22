use num_traits::FromPrimitive;
use std::fmt::Debug;

use crate::smart_device::{CelsiusThermometer, PowerSocket, Socket, SocketState, Thermometer};
use crate::utils::trait_alias::RandomNumber;

pub trait DeviceVisitor<T: RandomNumber> {
    fn visit_socket(&mut self, socket: &mut PowerSocket<T>);
    fn visit_thermometer(&mut self, thermometer: &mut CelsiusThermometer<T>);
}

pub trait PrintVisitor {
    #[must_use]
    fn new(device_index: usize) -> Self;
}

pub struct PowerSocketTurnOffVisitor;
pub struct PrintStateVisitor {
    device_index: usize,
}

impl<T: RandomNumber + FromPrimitive> DeviceVisitor<T> for PowerSocketTurnOffVisitor {
    fn visit_socket(&mut self, socket: &mut PowerSocket<T>) {
        socket.turn_off();
    }

    fn visit_thermometer(&mut self, _thermometer: &mut CelsiusThermometer<T>) {
        // NOTE: Empty
    }
}

impl PrintVisitor for PrintStateVisitor {
    fn new(device_index: usize) -> Self {
        Self { device_index }
    }
}

impl<T: RandomNumber + FromPrimitive + Debug> DeviceVisitor<T> for PrintStateVisitor {
    fn visit_socket(&mut self, socket: &mut PowerSocket<T>) {
        let state = socket.get_state();
        let power = socket.get_power();

        println!(
            "Device {} (socket): state = {}, power = {:?}",
            self.device_index,
            match state {
                SocketState::ON => "ON",
                SocketState::OFF => "OFF",
            },
            power
        );
    }

    fn visit_thermometer(&mut self, thermometer: &mut CelsiusThermometer<T>) {
        let temperature = thermometer.get_temperature();

        println!(
            "Device {} (thermometer): temperature = {:?}",
            self.device_index, temperature
        );
    }
}

pub trait Device<T: RandomNumber> {
    fn accept(&mut self, visitor: &mut dyn DeviceVisitor<T>);
}

impl<TemperatureT: RandomNumber + FromPrimitive + Debug> Device<TemperatureT>
    for CelsiusThermometer<TemperatureT>
{
    fn accept(&mut self, visitor: &mut dyn DeviceVisitor<TemperatureT>) {
        visitor.visit_thermometer(self);
    }
}

impl<PowerT: RandomNumber + FromPrimitive + Debug> Device<PowerT> for PowerSocket<PowerT> {
    fn accept(&mut self, visitor: &mut dyn DeviceVisitor<PowerT>) {
        visitor.visit_socket(self);
    }
}
