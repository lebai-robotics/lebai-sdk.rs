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
