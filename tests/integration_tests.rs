use sh::error::HomeError;
use sh::home::{Home, SmartHome};
use sh::make_room;
use sh::report::Report;
use sh::room::{Room, SmartRoom};
use sh::smart_device::{CelsiusThermometer, Device, PowerSocket, Socket, Thermometer};
use std::collections::HashMap;

#[test]
fn test_make_power_socket() {
    let socket = PowerSocket::<f32>::new(100.0, 20.0);
    assert_eq!(socket.get_state(), sh::smart_device::SocketState::OFF);
}

#[test]
fn test_make_thermometer() {
    let mut thermometer = CelsiusThermometer::<f32>::new(22.0, -5.0, 8.0);
    let temperature = thermometer.get_temperature();
    assert!((17.0..=30.0).contains(&temperature));
}

#[test]
fn test_make_room() {
    let devices: HashMap<String, Device<f32>> = HashMap::new();
    let room = SmartRoom::<f32>::new(devices);
    assert!(room.get_device("nonexistent").is_none());
}

#[test]
fn test_make_home() {
    let rooms: HashMap<String, SmartRoom<f32>> = HashMap::new();
    let home = SmartHome::<f32>::new(rooms);
    assert!(home.get_room("nonexistent").is_none());
}

#[test]
fn test_make_room_macro() {
    let socket = PowerSocket::<f32>::new(100.0, 20.0);
    let thermometer = CelsiusThermometer::<f32>::new(22.0, -5.0, 8.0);
    let room = make_room!(
        "socket" => socket,
        "thermometer" => thermometer
    );
    assert!(room.get_device("socket").is_some());
    assert!(room.get_device("thermometer").is_some());
}

#[test]
fn test_add_room() {
    let mut home = SmartHome::<f32>::new(HashMap::new());
    let room = make_room!("socket" => PowerSocket::<f32>::new(100.0, 20.0));

    home.add_room("living".to_string(), room);
    assert!(home.get_room("living").is_some());
}

#[test]
fn test_remove_room() {
    let mut home = SmartHome::<f32>::new(HashMap::new());
    let room = make_room!("socket" => PowerSocket::<f32>::new(100.0, 20.0));

    home.add_room("living".to_string(), room);
    home.remove_room("living");

    assert!(home.get_room("living").is_none());
}

#[test]
fn test_add_device() {
    let new_socket = PowerSocket::<f32>::new(200.0, 30.0);
    let mut room = make_room!("socket" => PowerSocket::<f32>::new(100.0, 20.0));

    room.add_device("new_socket".to_string(), new_socket.into());

    assert!(room.get_device("new_socket").is_some());
}

#[test]
fn test_remove_device() {
    let mut room = make_room!(
        "socket" => PowerSocket::<f32>::new(100.0, 20.0),
        "thermometer" => CelsiusThermometer::<f32>::new(22.0, -5.0, 8.0)
    );

    room.remove_device("thermometer");
    assert!(room.get_device("thermometer").is_none());
    assert!(room.get_device("socket").is_some());
}

#[test]
fn test_get_device_room_not_found() {
    let home = SmartHome::<f32>::new(HashMap::new());
    let result = home.get_device("nonexistent_room", "device");

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), HomeError::RoomNotFound(_)));
}

#[test]
fn test_get_device_device_not_found() {
    let mut home = SmartHome::<f32>::new(HashMap::new());
    let room = make_room!("socket" => PowerSocket::<f32>::new(100.0, 20.0));

    home.add_room("living".to_string(), room);
    let result = home.get_device("living", "nonexistent_device");

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), HomeError::DeviceNotFound(_)));
}

#[test]
fn test_device_report() {
    let socket = PowerSocket::<f32>::new(100.0, 20.0);
    let report = socket.report();

    assert!(report.contains("Socket"));
    assert!(report.contains("100"));
    assert!(report.contains("20"));
}

#[test]
fn test_room_report() {
    let room = make_room!("socket" => PowerSocket::<f32>::new(100.0, 20.0));
    let report = room.report();

    assert!(report.contains("Device"));
    assert!(report.contains("socket"));
}

#[test]
fn test_from_power_socket_to_device() {
    let socket = PowerSocket::<f32>::new(100.0, 20.0);
    let device: Device<f32> = socket.into();

    match device {
        Device::Socket(_) => {}
        _ => panic!("Expected Device::Socket"),
    }
}

#[test]
fn test_from_thermometer_to_device() {
    let thermometer = CelsiusThermometer::<f32>::new(22.0, -5.0, 8.0);
    let device: Device<f32> = thermometer.into();

    match device {
        Device::Thermometer(_) => {}
        _ => panic!("Expected Device::Thermometer"),
    }
}
