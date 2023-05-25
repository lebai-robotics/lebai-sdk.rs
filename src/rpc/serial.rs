use super::Robot;
use cmod::Result;
use proto::lebai::serial::*;

impl Robot {
    pub(crate) async fn set_serial_baud_rate(&self, device: String, baud_rate: u32) -> Result<()> {
        let req = SetSerialBaudRateRequest { device, baud_rate };
        let _ = self.c.set_serial_baud_rate(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn set_serial_parity(&self, device: String, parity: Parity) -> Result<()> {
        let req = SetSerialParityRequest {
            device,
            parity: parity as i32,
        };
        let _ = self.c.set_serial_parity(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn write_serial(&self, device: String, data: Vec<u8>) -> Result<()> {
        let data = data.into_iter().map(|x| x as u32).collect();
        let req = WriteSerialRequest { device, data };
        let _ = self.c.write_serial(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn read_serial(&self, device: String, len: u32) -> Result<Vec<u8>> {
        let req = ReadSerialRequest { device, len };
        let resp = self.c.read_serial(Some(req)).await.map_err(|e| e.to_string())?;
        let data = resp.data.into_iter().map(|x| x as u8).collect();
        Ok(data)
    }
}
