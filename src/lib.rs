mod native;

use std::ffi::CString;
use std::io::{self, ErrorKind};

pub struct Sdk {
    instance: native::VpInstance,
    pub username: String,
    pub botname: String,
    pub world: String,
}

impl Sdk {
    pub fn create(host: &str, port: i32) -> Result<Sdk, io::Error> {
        let host = CString::new(host)?;

        unsafe {
            let sdk = Sdk { instance: native::vp_create(), username: String::new(), botname: String::new(), world: String::new() };

            let rc = native::vp_connect_universe(sdk.instance, host.as_ptr(), port);
            if rc != 0 {
                return Err(io::Error::new(ErrorKind::AddrNotAvailable, "Could not connect to the universe."))
            }

            Ok(sdk)
        }
    }

    pub fn login(&mut self, username: &str, password: &str, botname: &str) -> Result<i32, io::Error> {
        self.username = String::from(username);
        self.botname = String::from(botname);

        let username = CString::new(username)?;
        let password = CString::new(password)?;
        let botname = CString::new(botname)?;
        let rc = unsafe {
            native::vp_login(self.instance, username.as_ptr(), password.as_ptr(), botname.as_ptr())
        };

        if rc != 0 {
            Err(io::Error::new(ErrorKind::PermissionDenied, "Could not login."))
        } else {
            Ok(rc)
        }
    }

    pub fn enter(&mut self, worldname: &str) -> Result<i32, io::Error> {
        self.world = String::from(worldname);

        let worldname = CString::new(worldname)?;
        let rc = unsafe {
            native::vp_enter(self.instance, worldname.as_ptr())
        };

        if rc != 0 {
            Err(io::Error::new(ErrorKind::InvalidData, "Could not enter world."))
        } else {
            Ok(rc)
        }
    }

    pub fn update(&mut self) -> Result<i32, io::Error> {
        let rc = unsafe {
            native::vp_state_change(self.instance)
        };

        if rc != 0 {
            Err(io::Error::new(ErrorKind::Other, "Could not update state."))
        } else {
            Ok(rc)
        }
    }
}