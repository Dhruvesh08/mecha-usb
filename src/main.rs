use usb::{UsbPort, UsbTrait};


mod usb;
fn main() {
    let mut usb_port = UsbPort {
        role_path: String::new(),
    };

    // Set the USB port device path

    let path = "/sys/class/udc/ci_hdrc.0/device/role".to_string();
    usb_port.set_device(path).expect("Failed to set USB device");
    
    // Now you can use the USB port methods as required
    match usb_port.check_mode() {
        Ok(mode) => println!("Current mode: {}", mode),
        Err(err) => println!("Error checking mode: {}", err),
    }
    
    // Set the USB port mode to host
    if let Err(err) = usb_port.set_host_mode() {
        println!("Error setting host mode: {}", err);
    }
    
    //sleep for 5 seconds
    std::thread::sleep(std::time::Duration::from_secs(5));
    
    // Set the USB port mode to gadget
    if let Err(err) = usb_port.set_gadget_mode() {
        println!("Error setting gadget mode: {}", err);
    }


    // do same for other usb port
    let mut usb_port = UsbPort {
        role_path: String::new(),
    };

    // Set the USB port device path

    let path = "/sys/class/udc/ci_hdrc.1/device/role".to_string();
    usb_port.set_device(path).expect("Failed to set USB device");
    
    // Now you can use the USB port methods as required
    match usb_port.check_mode() {
        Ok(mode) => println!("Current mode: {}", mode),
        Err(err) => println!("Error checking mode: {}", err),
    }
    
    // Set the USB port mode to host
    if let Err(err) = usb_port.set_host_mode() {
        println!("Error setting host mode: {}", err);
    }
    
    //sleep for 5 seconds
    std::thread::sleep(std::time::Duration::from_secs(5));
    
    // Set the USB port mode to gadget
    if let Err(err) = usb_port.set_gadget_mode() {
        println!("Error setting gadget mode: {}", err);
    }
    
}
