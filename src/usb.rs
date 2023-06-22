use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Error};

pub enum UsbPortMode {
    Host,
    Gadget,
}

pub struct UsbPort {
   pub role_path: String,
}

pub trait UsbTrait {
    fn set_device(&mut self, path: String) -> Result<(), Error>;
    fn check_mode(&self) -> Result<String, Error>;
    fn set_mode(&self, mode: UsbPortMode) -> Result<(), Error>;
    fn set_gadget_mode(&self) -> Result<(), Error>;
    fn set_host_mode(&self) -> Result<(), Error>;
}

impl UsbTrait for UsbPort {
    fn set_device(&mut self, path: String) -> Result<(), Error> {
        self.role_path = path;
        Ok(())
    }

    fn check_mode(&self) -> Result<String, Error> {
        let mut file = File::open(&self.role_path)?;
        let mut mode = String::new();
        file.read_to_string(&mut mode)?;
        match mode.trim() {
            "host" => Ok("host".to_string()),
            "gadget" => Ok("gadget".to_string()),
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid mode",
            )),
        }
    }

    fn set_mode(&self, mode: UsbPortMode) -> Result<(), Error> {
        let mode_str = match mode {
            UsbPortMode::Host => "host",
            UsbPortMode::Gadget => "gadget",
        };
        let mut file = OpenOptions::new().write(true).open(&self.role_path)?;
        file.write_all(mode_str.as_bytes())?;
        Ok(())
    }

    fn set_gadget_mode(&self) -> Result<(), Error> {
        self.set_mode(UsbPortMode::Gadget)
    }

    fn set_host_mode(&self) -> Result<(), Error> {
        self.set_mode(UsbPortMode::Host)
    }
}
