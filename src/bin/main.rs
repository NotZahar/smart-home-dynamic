use sh::error::HomeError;
use sh::home::{Home, SmartHome};
use sh::make_room;
use sh::report::Report;
use sh::room::{Room, SmartRoom};
use sh::smart_device::{CelsiusThermometer, PowerSocket, Socket, Thermometer};

fn print_report(item: &impl Report, title: &str) {
    println!("=== {} ===", title);
    println!("{}", item.report());
}

fn main() -> Result<(), HomeError> {
    let mut socket_living = PowerSocket::<f32>::new(100.0, 20.0);
    let socket_kitchen = PowerSocket::<f32>::new(200.0, 30.0);
    let thermometer_living = CelsiusThermometer::<f32>::new(22.0, -5.0, 8.0);

    socket_living.turn_on();

    let room_living = make_room!(
        "socket" => socket_living,
        "thermometer" => thermometer_living
    );
    let room_kitchen = make_room!(
        "socket" => socket_kitchen
    );

    let mut rooms = std::collections::HashMap::new();
    rooms.insert("living".to_string(), room_living);
    rooms.insert("kitchen".to_string(), room_kitchen);

    let mut home = SmartHome::new(rooms);
    print_report(&home, "Initial home state");

    let socket_bathroom = PowerSocket::<f32>::new(50.0, 10.0);
    let room_bathroom = make_room!("socket" => socket_bathroom);
    home.add_room("bathroom".to_string(), room_bathroom);
    print_report(&home, "After adding bathroom");

    if let Some(room) = home.get_room_mut("living") {
        room.remove_device("thermometer")
    }
    print_report(&home, "After removing thermometer from living room");

    let socket_new = PowerSocket::<f32>::new(300.0, 40.0);
    if let Some(room) = home.get_room_mut("living") {
        room.add_device("new_socket".to_string(), socket_new.into());
    }
    print_report(&home, "After adding new socket to living room");

    home.remove_room("bathroom");
    print_report(&home, "After removing bathroom room");

    println!("=== Error handling ===");
    match home.get_device("living", "thermometer") {
        Ok(device) => println!("Found device: {:?}", device),
        Err(error) => println!("Error: {}", error),
    }
    match home.get_device("bathroom", "nonexistent") {
        Ok(device) => println!("Found device: {:?}", device),
        Err(error) => println!("Error: {}", error),
    }
    match home.get_device("nonexistent", "socket") {
        Ok(device) => println!("Found device: {:?}", device),
        Err(error) => println!("Error: {}", error),
    }

    println!();
    if let Ok(device) = home.get_device("living", "socket") {
        print_report(device, "Report for a single device");
    }

    println!();
    if let Some(room) = home.get_room("kitchen") {
        print_report(room, "Report for a single room");
    }

    Ok(())
}
