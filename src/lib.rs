#[macro_export]
macro_rules! make_room {
    ( $($device_name:expr => $device:expr),* $(,)? ) => {
        {
            use std::collections::HashMap;
            let mut devices = HashMap::new();
            $(
                devices.insert($device_name.to_string(), $device.into());
            )*
            SmartRoom::new(devices)
        }
    };
}

mod utils;

pub mod error;
pub mod home;
pub mod report;
pub mod room;
pub mod smart_device;
