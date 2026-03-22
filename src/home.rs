use num_traits::FromPrimitive;
use std::fmt::Debug;

use crate::{
    room::{Room, SmartRoom},
    utils::trait_alias::RandomNumber,
};

pub trait Home<T> {
    #[must_use]
    fn new(rooms: Vec<SmartRoom<T>>) -> Self;

    #[must_use]
    fn get_room(&self, index: usize) -> &SmartRoom<T>;

    #[must_use]
    fn get_room_mut(&mut self, index: usize) -> &mut SmartRoom<T>;

    fn print_report(&mut self);
}

pub struct SmartHome<T> {
    rooms: Vec<SmartRoom<T>>,
}

impl<T: RandomNumber + FromPrimitive + Debug> Home<T> for SmartHome<T> {
    fn new(rooms: Vec<SmartRoom<T>>) -> Self {
        SmartHome { rooms }
    }

    fn get_room(&self, index: usize) -> &SmartRoom<T> {
        &self.rooms[index]
    }

    fn get_room_mut(&mut self, index: usize) -> &mut SmartRoom<T> {
        &mut self.rooms[index]
    }

    fn print_report(&mut self) {
        for (room_index, room) in self.rooms.iter_mut().enumerate() {
            println!("Room {}:", room_index);
            room.print_report();
        }
    }
}
