use super::Robot;
use cmod::Result;
use proto::lebai::modbus::*;

impl Robot {
    pub(crate) async fn disconnect_modbus(&self, device: String) -> Result<()> {
        let req = DisconnectModbusRequest { device };
        let _ = self.c.disconnect_modbus(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn set_modbus_timeout(&self, device: String, timeout: u32) -> Result<()> {
        let req = SetModbusTimeoutRequest { device, timeout };
        let _ = self.c.set_modbus_timeout(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn set_modbus_retry(&self, device: String, retry: u32) -> Result<()> {
        let req = SetModbusRetryRequest { device, retry };
        let _ = self.c.set_modbus_retry(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn write_single_coil(&self, device: String, pin: String, value: bool) -> Result<()> {
        let req = SetCoilRequest { device, pin, value };
        let _ = self.c.write_single_coil(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn write_multiple_coils(&self, device: String, pin: String, values: Vec<bool>) -> Result<()> {
        let req = SetCoilsRequest { device, pin, values };
        let _ = self.c.write_multiple_coils(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn read_coils(&self, device: String, pin: String, count: u32) -> Result<Vec<bool>> {
        let req = GetCoilsRequest { device, pin, count };
        let resp = self.c.read_coils(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp.values)
    }
    pub(crate) async fn read_discrete_inputs(&self, device: String, pin: String, count: u32) -> Result<Vec<bool>> {
        let req = GetCoilsRequest { device, pin, count };
        let resp = self.c.read_discrete_inputs(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp.values)
    }
    pub(crate) async fn write_single_register(&self, device: String, pin: String, value: u32) -> Result<()> {
        let req = SetRegisterRequest { device, pin, value };
        let _ = self.c.write_single_register(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn write_multiple_registers(&self, device: String, pin: String, values: Vec<u32>) -> Result<()> {
        let req = SetRegistersRequest { device, pin, values };
        let _ = self.c.write_multiple_registers(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn read_input_registers(&self, device: String, pin: String, count: u32) -> Result<Vec<u32>> {
        let req = GetRegistersRequest { device, pin, count };
        let resp = self.c.read_input_registers(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp.values)
    }
    pub(crate) async fn read_holding_registers(&self, device: String, pin: String, count: u32) -> Result<Vec<u32>> {
        let req = GetRegistersRequest { device, pin, count };
        let resp = self.c.read_holding_registers(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp.values)
    }
}
