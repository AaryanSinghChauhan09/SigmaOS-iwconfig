//! Iwconfig model — configure wireless network interfaces.

#[derive(Debug, Clone)]
pub struct WirelessInfo {
    pub essid: String,
    pub mode: String,
    pub frequency: f64,
    pub access_point: String,
    pub bit_rate: f64,
    pub tx_power: u32,
    pub signal_level: i32,
    pub noise_level: i32,
}

#[derive(Debug, Clone)]
pub struct Iwconfig {
    pub show_all: bool,
}

impl Default for Iwconfig {
    fn default() -> Self { Self { show_all: false } }
}

impl Iwconfig {
    pub fn new() -> Self { Self::default() }

    pub fn get_info(&self, _iface: &str) -> WirelessInfo {
        WirelessInfo { essid: "MyNetwork".into(), mode: "Managed".into(), frequency: 2.437, access_point: "00:11:22:33:44:55".into(), bit_rate: 54.0, tx_power: 20, signal_level: -50, noise_level: -90 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_info() {
        let i = Iwconfig::new();
        assert_eq!(i.get_info("wlan0").essid, "MyNetwork");
    }
}
