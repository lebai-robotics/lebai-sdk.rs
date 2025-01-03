use super::Robot;
use cmod::Result;
use proto::lebai::io::*;

impl Robot {
    pub(crate) async fn set_dio_mode(&self, device: String, pin: u32, mode: DigitalMode) -> Result<()> {
        let req = SetDioModeRequest {
            device: IoDevice::from(&*device).into(),
            pin,
            mode: mode.into(),
        };
        let _ = self.c.set_dio_mode(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn get_dio_mode(&self, device: String, pin: u32) -> Result<DigitalMode> {
        let req = GetDioModeRequest {
            device: IoDevice::from(&*device).into(),
            pin,
        };
        let resp = self.c.get_dio_mode(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp.mode())
    }
    pub(crate) async fn set_do(&self, device: String, pin: u32, value: u32) -> Result<()> {
        let req = SetDoPinRequest {
            device: IoDevice::from(&*device).into(),
            pin,
            value,
        };
        let _ = self.c.set_do(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn get_do(&self, device: String, pin: u32) -> Result<u32> {
        let req = GetDioPinRequest {
            device: IoDevice::from(&*device).into(),
            pin,
        };
        let resp = self.c.get_do(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp.value)
    }
    pub(crate) async fn get_dos(&self, device: String, pin: u32, num: u32) -> Result<Vec<u32>> {
        let req = GetDioPinsRequest {
            device: IoDevice::from(&*device).into(),
            pin,
            count: num,
        };
        let resp = self.c.get_dos(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp.values)
    }
    pub(crate) async fn get_di(&self, device: String, pin: u32) -> Result<u32> {
        let req = GetDioPinRequest {
            device: IoDevice::from(&*device).into(),
            pin,
        };
        let resp = self.c.get_di(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp.value)
    }
    pub(crate) async fn get_dis(&self, device: String, pin: u32, num: u32) -> Result<Vec<u32>> {
        let req = GetDioPinsRequest {
            device: IoDevice::from(&*device).into(),
            pin,
            count: num,
        };
        let resp = self.c.get_dis(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp.values)
    }
    pub(crate) async fn set_ao(&self, device: String, pin: u32, value: f64) -> Result<()> {
        let req = SetAoPinRequest {
            device: IoDevice::from(&*device).into(),
            pin,
            value,
        };
        let _ = self.c.set_ao(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn get_ao(&self, device: String, pin: u32) -> Result<f64> {
        let req = GetAioPinRequest {
            device: IoDevice::from(&*device).into(),
            pin,
        };
        let resp = self.c.get_ao(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp.value)
    }
    pub(crate) async fn get_aos(&self, device: String, pin: u32, num: u32) -> Result<Vec<f64>> {
        let req = GetAioPinsRequest {
            device: IoDevice::from(&*device).into(),
            pin,
            count: num,
        };
        let resp = self.c.get_aos(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp.values)
    }
    pub(crate) async fn get_ai(&self, device: String, pin: u32) -> Result<f64> {
        let req = GetAioPinRequest {
            device: IoDevice::from(&*device).into(),
            pin,
        };
        let resp = self.c.get_ai(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp.value)
    }
    pub(crate) async fn get_ais(&self, device: String, pin: u32, num: u32) -> Result<Vec<f64>> {
        let req = GetAioPinsRequest {
            device: IoDevice::from(&*device).into(),
            pin,
            count: num,
        };
        let resp = self.c.get_ais(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp.values)
    }
}
