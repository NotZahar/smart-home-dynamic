use num_traits::FromPrimitive;
use std::collections::HashMap;
use std::fmt;

use crate::error::HomeError;
use crate::report::Report;
use crate::room::{Room, SmartRoom};
use crate::smart_device::Device;
use crate::utils::trait_alias::RandomNumber;

pub trait Home<T: RandomNumber> {
    #[must_use]
    fn new(rooms: HashMap<String, SmartRoom<T>>) -> Self;

    #[must_use]
    fn get_room(&self, key: &str) -> Option<&SmartRoom<T>>;

    #[must_use]
    fn get_room_mut(&mut self, key: &str) -> Option<&mut SmartRoom<T>>;

    fn add_room(&mut self, key: String, room: SmartRoom<T>);

    fn remove_room(&mut self, key: &str);

    fn get_device(&self, room_key: &str, device_key: &str) -> Result<&Device<T>, HomeError>;

    fn get_device_mut(
        &mut self,
        room_key: &str,
        device_key: &str,
    ) -> Result<&mut Device<T>, HomeError>;
}

#[derive(Debug)]
pub struct SmartHome<T: RandomNumber> {
    rooms: HashMap<String, SmartRoom<T>>,
}

impl<T: RandomNumber + FromPrimitive + fmt::Debug> Home<T> for SmartHome<T> {
    fn new(rooms: HashMap<String, SmartRoom<T>>) -> Self {
        SmartHome { rooms }
    }

    fn get_room(&self, key: &str) -> Option<&SmartRoom<T>> {
        self.rooms.get(key)
    }

    fn get_room_mut(&mut self, key: &str) -> Option<&mut SmartRoom<T>> {
        self.rooms.get_mut(key)
    }

    fn add_room(&mut self, key: String, room: SmartRoom<T>) {
        self.rooms.insert(key, room);
    }

    fn remove_room(&mut self, key: &str) {
        self.rooms.remove(key);
    }

    fn get_device(&self, room_key: &str, device_key: &str) -> Result<&Device<T>, HomeError> {
        let room = self
            .rooms
            .get(room_key)
            .ok_or_else(|| HomeError::RoomNotFound(room_key.to_string()))?;
        room.get_device(device_key)
            .ok_or_else(|| HomeError::DeviceNotFound(device_key.to_string()))
    }

    fn get_device_mut(
        &mut self,
        room_key: &str,
        device_key: &str,
    ) -> Result<&mut Device<T>, HomeError> {
        let room = self
            .rooms
            .get_mut(room_key)
            .ok_or_else(|| HomeError::RoomNotFound(room_key.to_string()))?;
        room.get_device_mut(device_key)
            .ok_or_else(|| HomeError::DeviceNotFound(device_key.to_string()))
    }
}

impl<T: RandomNumber + FromPrimitive + fmt::Debug> Report for SmartHome<T> {
    fn report(&self) -> String {
        let mut result = String::from("Home:\n");
        for (name, room) in &self.rooms {
            result.push_str(&format!(" - Room '{}':\n", name));
            for line in room.report().lines() {
                result.push_str(&format!("    - {}\n", line));
            }
        }
        result
    }
}
