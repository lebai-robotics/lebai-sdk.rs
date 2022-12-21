pub use crate::lebai::modbus::ModbusKind;

impl From<&str> for ModbusKind {
    fn from(s: &str) -> Self {
        match s {
            "ROBOT" => ModbusKind::Robot,
            "FLANGE" => ModbusKind::Flange,
            "EXTRA" => ModbusKind::Extra,
            "MODBUSFLANGE" => ModbusKind::ModbusFlange,
            "MODBUSRTU" => ModbusKind::ModbusRtu,
            _ => ModbusKind::ModbusTcp,
        }
    }
}

impl From<String> for ModbusKind {
    fn from(s: String) -> Self {
        match s.as_str() {
            "ROBOT" => ModbusKind::Robot,
            "FLANGE" => ModbusKind::Flange,
            "EXTRA" => ModbusKind::Extra,
            "MODBUSFLANGE" => ModbusKind::ModbusFlange,
            "MODBUSRTU" => ModbusKind::ModbusRtu,
            _ => ModbusKind::ModbusTcp,
        }
    }
}
