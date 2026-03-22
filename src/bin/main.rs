use sh::home::{Home, SmartHome};
use sh::room::{Room, SmartRoom};
use sh::smart_device::{
    CelsiusThermometer, Device, PowerSocket, PowerSocketTurnOffVisitor, Socket, Thermometer,
};

fn main() {
    let mut power_socket_living = PowerSocket::<f32>::new(100.0, 20.0);
    let power_socket_kitchen = PowerSocket::<f32>::new(200.0, 30.0);
    let thermometer_living = CelsiusThermometer::<f32>::new(22.0, 5.0, 22.0);

    power_socket_living.turn_on();

    let living_room = SmartRoom::new(vec![
        Box::new(power_socket_living) as Box<dyn Device<f32>>,
        Box::new(thermometer_living) as Box<dyn Device<f32>>,
    ]);
    let kitchen_room = SmartRoom::new(vec![Box::new(power_socket_kitchen) as Box<dyn Device<f32>>]);

    let mut home = SmartHome::new(vec![living_room, kitchen_room]);

    println!("==== Initial state ====");
    home.print_report();

    let mut turn_off_visitor = PowerSocketTurnOffVisitor;
    home.get_room_mut(0)
        .get_device_mut(0)
        .accept(&mut turn_off_visitor);

    println!("\n==== After socket turn off state ====");
    home.print_report();
}
