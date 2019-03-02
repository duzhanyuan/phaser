mod ports;
mod domain;

use crate::scanner::{PortModule, HostModule};

pub use ports::Ports;
pub use domain::whois::Whois;

pub fn get_port_modules() -> Vec<Box<dyn PortModule>> {
    return vec!(

    );
}

pub fn get_host_modules() -> Vec<Box<dyn HostModule>> {
    return vec!(
        Box::new(Whois{}),
    );
}
