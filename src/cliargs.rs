use crate::ip::IP;
use std::{env, fmt::Display};

pub struct CliArgs {
    target: IP,
    network: bool,
    broadcast: bool,
    mask: bool,
    hostmin: bool,
    hostmax: bool,
    count: bool,
}

impl CliArgs {
    pub fn from_args() -> Result<CliArgs, Box<dyn std::error::Error>> {
        let mut cliargs = CliArgs {
            target: IP::IPV4 {
                prefix: 0,
                mask: 32,
            },
            network: false,
            broadcast: false,
            mask: false,
            hostmin: false,
            hostmax: false,
            count: false,
        };

        for argument in env::args().skip(1) {
            match argument.as_str() {
                "-n" | "--network" => cliargs.network = true,
                "-b" | "--broadcast" => cliargs.broadcast = true,
                "-m" | "--mask" => cliargs.mask = true,
                "-i" | "--hostmin" => cliargs.hostmin = true,
                "-a" | "--hostmax" => cliargs.hostmax = true,
                "-c" | "--count" => cliargs.count = true,
                unknown if unknown.starts_with('-') => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Flag inconnu : {}", unknown),
                    )));
                }
                _ => {
                    cliargs.target = IP::from_str(&argument).map_err(|e| {
                        Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, e))
                    })?;
                }
            }
        }
        Ok(cliargs)
    }

    pub fn get_network(&self) -> bool {
        self.network
    }

    pub fn get_broadcast(&self) -> bool {
        self.broadcast
    }

    pub fn get_mask(&self) -> bool {
        self.mask
    }

    pub fn get_hostmin(&self) -> bool {
        self.hostmin
    }

    pub fn get_hostmax(&self) -> bool {
        self.hostmax
    }

    pub fn get_count(&self) -> bool {
        self.count
    }

    pub fn get_target(&self) -> IP {
        self.target
    }
}
impl Display for CliArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CliArgs {{ target: {}, network: {}, broadcast: {}, mask: {}, hostmin: {}, hostmax: {}, count: {} }}",
            self.target,
            self.network,
            self.broadcast,
            self.mask,
            self.hostmin,
            self.hostmax,
            self.count
        )
    }
}
