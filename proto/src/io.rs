pub use crate::lebai::io::IoDevice;

impl From<&str> for IoDevice {
    fn from(s: &str) -> Self {
        match s {
            "ROBOT" => IoDevice::Robot,
            "FLANGE" => IoDevice::Flange,
            "EXTRA" => IoDevice::Extra,
            "SHOULDER" => IoDevice::Shoulder,
            _ => IoDevice::FlangeBtn,
        }
    }
}

impl From<String> for IoDevice {
    fn from(s: String) -> Self {
        match s.as_str() {
            "ROBOT" => IoDevice::Robot,
            "FLANGE" => IoDevice::Flange,
            "EXTRA" => IoDevice::Extra,
            "SHOULDER" => IoDevice::Shoulder,
            _ => IoDevice::FlangeBtn,
        }
    }
}
