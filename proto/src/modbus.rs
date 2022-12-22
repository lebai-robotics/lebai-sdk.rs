pub use crate::lebai::modbus::ModbusKind;

impl From<&str> for ModbusKind {
    fn from(s: &str) -> Self {
        match s {
            "ROBOT" => ModbusKind::Robot,
            "FLANGE" => ModbusKind::Flange,
            "EXTRA" => ModbusKind::Extra,
            "MODBUS_FLANGE" => ModbusKind::ModbusFlange,
            "MODBUS_RTU" => ModbusKind::ModbusRtu,
            "MODBUS_TCP" => ModbusKind::ModbusTcp,
            _ => ModbusKind::ModbusFlange,
        }
    }
}
