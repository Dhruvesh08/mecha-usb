use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

enum UsbPortMode {
    Host,
    Gadget,
}

pub struct UsbPort {
    role_path: String,
}

impl UsbPort {
    pub fn new(port_number: u32) -> UsbPort {
        let role_path = format!("/sys/class/udc/ci_hdrc.{}/device/role", port_number);
        UsbPort { role_path }
    }

    pub fn check_mode(&self) -> Result<String, std::io::Error> {
        let mut file = File::open(&self.role_path)?;
        let mut mode = String::new();
        file.read_to_string(&mut mode)?;
        match mode.trim() {
            "host" => Ok("host".to_string()),
            "gadget" => Ok("gadget".to_string()),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid mode",
            )),
        }
    }

    pub fn set_host_mode(&self) -> Result<(), std::io::Error> {
        self.set_mode(UsbPortMode::Host)
    }

    pub fn set_gadget_mode(&self) -> Result<(), std::io::Error> {
        self.set_mode(UsbPortMode::Gadget)
    }

    fn set_mode(&self, mode: UsbPortMode) -> Result<(), std::io::Error> {
        let mode_str = match mode {
            UsbPortMode::Host => "host",
            UsbPortMode::Gadget => "gadget",
        };
        let mut file = OpenOptions::new().write(true).open(&self.role_path)?;
        file.write_all(mode_str.as_bytes())?;
        Ok(())
    }
}
