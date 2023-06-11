mod usb;
use usb::UsbPort;

fn main() {
    let usb_port = UsbPort::new(0);

    match usb_port.check_mode() {
        Ok(mode_str) => println!("Current USB port mode: {:?}", mode_str),
        Err(err) => eprintln!("Error checking USB port mode: {}", err),
    }

    match usb_port.set_host_mode() {
        Ok(()) => println!("USB port set to host mode"),
        Err(err) => eprintln!("Error setting USB port to host mode: {}", err),
    }

    match usb_port.set_gadget_mode() {
        Ok(()) => println!("USB port set to gadget mode"),
        Err(err) => eprintln!("Error setting USB port to gadget mode: {}", err),
    }
}
