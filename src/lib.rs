use log::{debug, info, warn};
use network_interface::{NetworkInterface, NetworkInterfaceConfig};
use std::process::Command;

pub struct HealthCheck {
    application: String,
    app_log_path: String,
}

impl HealthCheck {
    pub fn new(application: String, app_log_path: String) -> HealthCheck {
        HealthCheck {
            application,
            app_log_path,
        }
    }

    pub fn app_name(&self) -> &String {
        &self.application
    }

    pub fn check_application(&self) -> bool {
        *self.check_jps_output() && *self.check_log_file()
    }

    fn check_jps_output(&self) -> &bool {
        let cmd = "jps";
        let output = Command::new(cmd).output();
        let cmd_py = ["ps", "-ex"];
        let output_py = Command::new(cmd_py[0]).arg(cmd_py[1]).output();
        let jps_out = match output {
            Ok(contents) => String::from_utf8(contents.stdout).unwrap_or_else(|_| String::new()),
            Err(_) => String::new(),
        };

        let py_out = match output_py {
            Ok(contents) => String::from_utf8(contents.stdout).unwrap_or_else(|_| String::new()),
            Err(_) => String::new(),
        };

        debug!("Searching for {}", &self.application);
        if jps_out.contains(&self.application) || py_out.contains(&self.application) {
            &true
        } else {
            debug!("{} not in JPS or ps -ex", &self.application);
            &false
        }
    }

    fn check_log_file(&self) -> &bool {
        debug!("Checking log file at {}", &self.app_log_path);
        let cmd_args = ["tail", "-100", &self.app_log_path];
        debug!("Command: {} {} {}", cmd_args[0], cmd_args[1], cmd_args[2]);
        let output = Command::new(cmd_args[0])
            .arg(cmd_args[1])
            .arg(cmd_args[2])
            .output();

        match output {
            Ok(contents) => {
                if !&contents.stdout.is_empty() {
                    let content_string = String::from_utf8(contents.stdout).unwrap();
                    if content_string.contains("Out Of Memory") {
                        warn!("Log file check for {} FAILED", &self.app_log_path);
                        &false
                    } else {
                        info!("Log file check for {} PASSED", &self.app_log_path);
                        &true
                    };
                }
                warn!("Failed to find {}, check fails", &self.app_log_path);
                &false
            }
            Err(_) => {
                warn!("Failed to find {}, check fails.", &self.app_log_path);
                &false
            }
        }
    }
}

pub fn get_eth0_ip() -> String {
    let netis = NetworkInterface::show().unwrap();
    let network_name = String::from("eth0");
    for network in netis {
        if network.name == network_name {
            let network_address = network.addr;
            match network_address {
                Some(addr) => {
                    let network_eth0 = addr.ip().to_string();
                    return network_eth0;
                }
                None => {
                    let local_network = String::from("127.0.0.1");
                    return local_network;
                }
            }
        }
    }
    String::from("127.0.0.1")
}
