use super::Robot;
use cmod::Result;
use proto::lebai::io::*;

impl Robot {
    pub(crate) async fn set_do(&self, device: IoDevice, pin: u32, value: u32) -> Result<()> {
        let req = SetDoPinRequest {
            device: device.into(),
            pin: pin.into(),
            value: value.into(),
        };
        let _ = self.c.set_do(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn get_do(&self, device: IoDevice, pin: u32) -> Result<u32> {
        let req = GetDioPinRequest {
            device: device.into(),
            pin: pin.into(),
        };
        let resp = self.c.get_do(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp.value)
    }
    pub(crate) async fn get_dos(&self, device: IoDevice, pin: u32, num: u32) -> Result<Vec<u32>> {
        let req = GetDioPinsRequest {
            device: device.into(),
            pin: pin.into(),
            count: num.into(),
        };
        let resp = self.c.get_dos(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp.values)
    }
    pub(crate) async fn get_di(&self, device: IoDevice, pin: u32) -> Result<u32> {
        let req = GetDioPinRequest {
            device: device.into(),
            pin: pin.into(),
        };
        let resp = self.c.get_di(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp.value)
    }
    pub(crate) async fn get_dis(&self, device: IoDevice, pin: u32, num: u32) -> Result<Vec<u32>> {
        let req = GetDioPinsRequest {
            device: device.into(),
            pin: pin.into(),
            count: num.into(),
        };
        let resp = self.c.get_dis(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp.values)
    }
    pub(crate) async fn set_ao(&self, device: IoDevice, pin: u32, value: u32) -> Result<()> {
        let req = SetAoPinRequest {
            device: device.into(),
            pin: pin.into(),
            value: value.into(),
        };
        let _ = self.c.set_ao(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn get_ao(&self, device: IoDevice, pin: u32) -> Result<f64> {
        let req = GetAioPinRequest {
            device: device.into(),
            pin: pin.into(),
        };
        let resp = self.c.get_ao(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp.value)
    }
    pub(crate) async fn get_aos(&self, device: IoDevice, pin: u32, num: u32) -> Result<Vec<f64>> {
        let req = GetAioPinsRequest {
            device: device.into(),
            pin: pin.into(),
            count: num.into(),
        };
        let resp = self.c.get_aos(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp.values)
    }
    pub(crate) async fn get_ai(&self, device: IoDevice, pin: u32) -> Result<f64> {
        let req = GetAioPinRequest {
            device: device.into(),
            pin: pin.into(),
        };
        let resp = self.c.get_ai(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp.value)
    }
    pub(crate) async fn get_ais(&self, device: IoDevice, pin: u32, num: u32) -> Result<Vec<f64>> {
        let req = GetAioPinsRequest {
            device: device.into(),
            pin: pin.into(),
            count: num.into(),
        };
        let resp = self.c.get_ais(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp.values)
    }
}
