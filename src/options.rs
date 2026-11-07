//! Options model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Default,
    Essid,
    Nickname,
    Bitrate,
    TxPower,
    Commit,
}

#[derive(Debug, Clone)]
pub struct IwconfigOptions {
    pub mode: Mode,
    pub interface: Option<String>,
}

impl Default for IwconfigOptions {
    fn default() -> Self { Self { mode: Mode::Default, interface: None } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opts() { assert_eq!(IwconfigOptions::default().mode, Mode::Default); }
}
