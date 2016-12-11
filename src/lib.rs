mod native;

use std::ffi::CString;
use std::io::{self, ErrorKind};

enum InstanceState {
    Disconnected,
    Connected,
    LoggedIn,
    InWorld,
}

pub struct Sdk {
    instance: native::VpInstance,
    pub username: String,
    pub botname: String,
    pub world: String,
    state: InstanceState,
}

impl Sdk {
    pub fn create(host: &str, port: i32) -> Result<Sdk, io::Error> {
        let host = CString::new(host)?;

        unsafe {
            let rc = native::vp_init(native::VPSDK_VERSION);
            if rc != 0 {
                return Err(io::Error::new(ErrorKind::AddrNotAvailable, format!("Could not initialize API. (reason {})", rc).as_str()));
            }

            let mut sdk = Sdk { instance: native::vp_create(std::ptr::null()),
                username: String::new(),
                botname: String::new(),
                world: String::new(),
                state: InstanceState::Disconnected
            };

            if sdk.instance == std::ptr::null_mut()  {
                return Err(io::Error::new(ErrorKind::InvalidData, "Could not create VP Instance."));
            }

            let rc = native::vp_connect_universe(sdk.instance, host.as_ptr(), port);
            if rc != 0 {
                return Err(io::Error::new(ErrorKind::AddrNotAvailable, "Could not connect to the universe."));
            }

            sdk.state = InstanceState::Connected;

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
            self.state = InstanceState::LoggedIn;
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
            self.state = InstanceState::InWorld;
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

    pub fn tick(&mut self, max_wait_ms: i32) -> Result<i32, io::Error> {
        let rc = unsafe {
            native::vp_wait(self.instance, max_wait_ms)
        };

        if rc != 0 {
            Err(io::Error::new(ErrorKind::Interrupted, "Could not process events."))
        } else {
            Ok(rc)
        }
    }

    pub fn say(&mut self, message: &str) -> Result<i32, io::Error> {
        let message = CString::new(message)?;
        let rc = unsafe {
            native::vp_say(self.instance, message.as_ptr())
        };

        if rc != 0 {
            Err(io::Error::new(ErrorKind::InvalidData, "Could not say anything."))
        } else {
            Ok(rc)
        }
    }
}

impl Drop for Sdk {
    fn drop(&mut self) {
        match self.state {
            InstanceState::InWorld => {
                unsafe {
                    native::vp_leave(self.instance);
                }
            },
            _ => {}
        }

        unsafe {
            native::vp_destroy(self.instance);
        }
    }
}