use std::fmt::Display;

use serde::Deserialize;

pub mod requests;
pub mod responses;

pub struct DnsStatus {
    pub auto: bool,
    pub dns1: String,
    pub dns2: String,
    pub dns3: String,
}

pub struct WanStatus {
    pub id: String,
    pub mode: WanMode,
    pub gateway_v4: String,
    pub gateway_v6: String,
    pub dns_v4: DnsStatus,
    pub dns_v6: DnsStatus,
}

pub enum WanMode {
    PPPoE,
    DHCP,
    Static,
}

pub struct InvalidWanMode;

impl TryFrom<usize> for WanMode {
    type Error = InvalidWanMode;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DHCP),
            1 => Ok(Self::Static),
            2 => Ok(Self::PPPoE),
            _ => Err(InvalidWanMode),
        }
    }
}

#[derive(Deserialize)]
pub struct LanStatus {
    pub id: String,
    pub ip4: String,
    pub ip6_count: usize,
    pub ip6_list: Vec<String>,
    pub mac: String,
    pub mtu: usize,
    pub netmask: String,
}

#[derive(Deserialize)]
pub struct Device {
    pub id: String,
    pub model: String,
    pub uptime: usize,
    pub fw_version: String,
}

#[derive(Deserialize)]
pub struct ConnectedDevice {
    pub hostname: String,
    pub id: String,
    pub ip: String,
    pub mac: String,
}

impl Display for ConnectedDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = format!(
            "<===
            <hostname> {}
            [id] {}
            [ip] {}
            [mac] {}
            ===>",
            (if self.hostname.is_empty() {
                "UNKNOWN"
            } else {
                &self.hostname
            }),
            self.id,
            self.ip,
            self.mac
        );

        write!(f, "{text}")
    }
}
