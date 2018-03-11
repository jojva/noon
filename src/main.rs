extern crate libusb;

fn main() {
    let context = libusb::Context::new().unwrap();
    let mut handle = None;
    println!("Browsing USB devices to find a VM116...");
    for mut device in context.devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        println!("Bus {:03} Device {:03} ID {:04x}:{:04x}",
            device.bus_number(),
            device.address(),
            device_desc.vendor_id(),
            device_desc.product_id());

        // VM116 device id is 10cf:8062
        if device_desc.vendor_id() == 0x10cf && device_desc.product_id() == 0x8062 {
            handle = match device.open() {
                Ok(handle) => Some(handle),
                Err(_) => {
                    println!("VM116 found but could not open it. :(");
                    return;
                }
            }
        }
    }
    match handle {
        Some(_) => println!("VM116 found! :)"),
        None => println!("VM116 not found. :(")
    }
}
