pub mod io;
pub mod kinematic;
pub mod led;
pub mod modbus;
pub mod posture;
pub mod task;

pub mod google {
    pub mod protobuf {
        pub use pbjson_types::*;
    }
}
pub mod lebai {
    include!(concat!(env!("OUT_DIR"), "/lebai.rs"));
    include!(concat!(env!("OUT_DIR"), "/lebai.serde.rs"));
    include!(concat!(env!("OUT_DIR"), "/lebai.jsonrpc.rs"));
    pub mod cmp {
        include!(concat!(env!("OUT_DIR"), "/lebai.cmp.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.cmp.serde.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.cmp.jsonrpc.rs"));
    }
    pub mod db {
        include!(concat!(env!("OUT_DIR"), "/lebai.db.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.db.serde.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.db.jsonrpc.rs"));
    }
    pub mod file {
        include!(concat!(env!("OUT_DIR"), "/lebai.file.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.file.serde.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.file.jsonrpc.rs"));
    }
    pub mod posture {
        include!(concat!(env!("OUT_DIR"), "/lebai.posture.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.posture.serde.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.posture.jsonrpc.rs"));
    }
    pub mod motor {
        include!(concat!(env!("OUT_DIR"), "/lebai.motor.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.motor.serde.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.motor.jsonrpc.rs"));
    }
    pub mod system {
        include!(concat!(env!("OUT_DIR"), "/lebai.system.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.system.serde.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.system.jsonrpc.rs"));
    }
    pub mod storage {
        include!(concat!(env!("OUT_DIR"), "/lebai.storage.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.storage.serde.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.storage.jsonrpc.rs"));
    }
    pub mod structure {
        include!(concat!(env!("OUT_DIR"), "/lebai.structure.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.structure.serde.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.structure.jsonrpc.rs"));
    }
    pub mod dynamic {
        include!(concat!(env!("OUT_DIR"), "/lebai.dynamic.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.dynamic.serde.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.dynamic.jsonrpc.rs"));
    }
    pub mod kinematic {
        include!(concat!(env!("OUT_DIR"), "/lebai.kinematic.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.kinematic.serde.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.kinematic.jsonrpc.rs"));
    }
    pub mod motion {
        include!(concat!(env!("OUT_DIR"), "/lebai.motion.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.motion.serde.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.motion.jsonrpc.rs"));
    }
    pub mod safety {
        include!(concat!(env!("OUT_DIR"), "/lebai.safety.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.safety.serde.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.safety.jsonrpc.rs"));
    }
    pub mod task {
        include!(concat!(env!("OUT_DIR"), "/lebai.task.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.task.serde.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.task.jsonrpc.rs"));
    }
    pub mod trigger {
        include!(concat!(env!("OUT_DIR"), "/lebai.trigger.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.trigger.serde.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.trigger.jsonrpc.rs"));
    }
    pub mod signal {
        include!(concat!(env!("OUT_DIR"), "/lebai.signal.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.signal.serde.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.signal.jsonrpc.rs"));
    }
    pub mod io {
        include!(concat!(env!("OUT_DIR"), "/lebai.io.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.io.serde.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.io.jsonrpc.rs"));
    }
    pub mod serial {
        include!(concat!(env!("OUT_DIR"), "/lebai.serial.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.serial.serde.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.serial.jsonrpc.rs"));
    }
    pub mod modbus {
        include!(concat!(env!("OUT_DIR"), "/lebai.modbus.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.modbus.serde.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.modbus.jsonrpc.rs"));
    }
    pub mod led {
        include!(concat!(env!("OUT_DIR"), "/lebai.led.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.led.serde.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.led.jsonrpc.rs"));
    }
    pub mod claw {
        include!(concat!(env!("OUT_DIR"), "/lebai.claw.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.claw.serde.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.claw.jsonrpc.rs"));
    }
    pub mod backup {
        include!(concat!(env!("OUT_DIR"), "/lebai.backup.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.backup.serde.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.backup.jsonrpc.rs"));
    }
    pub mod hardware {
        include!(concat!(env!("OUT_DIR"), "/lebai.hardware.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.hardware.serde.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.hardware.jsonrpc.rs"));
    }
    pub mod multi_devices {
        include!(concat!(env!("OUT_DIR"), "/lebai.multi_devices.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.multi_devices.serde.rs"));
        include!(concat!(env!("OUT_DIR"), "/lebai.multi_devices.jsonrpc.rs"));
    }
}
