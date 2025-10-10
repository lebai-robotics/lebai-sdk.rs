use cmod::Result;
use core::time::Duration;
use futures_util::future::{Either, select};
use mdns_sd::{ScopedIp, ServiceDaemon, ServiceEvent};
use proto::lebai::multi_devices::DeviceInfo;

const SERVICE_NAME: &str = "_lebai._tcp.local.";

pub async fn discover_devices(time: f64) -> Result<Vec<DeviceInfo>> {
    let mdns = ServiceDaemon::new().map_err(|e| e.to_string())?;
    let receiver = mdns.browse(SERVICE_NAME).map_err(|e| e.to_string())?;
    let mut devices: Vec<DeviceInfo> = Vec::new();
    let mut task_wait = futures_timer::Delay::new(Duration::from_millis((time * 1000.0) as u64));
    let mut task_recv = receiver.recv_async();
    while let Either::Left((res, fut_wait)) = select(task_recv, task_wait).await {
        let res = if let Ok(res) = res { res } else { break };
        task_wait = fut_wait;
        task_recv = receiver.recv_async();

        match res {
            ServiceEvent::ServiceResolved(record) => {
                let name = if let Some(x) = record.get_fullname().split('.').next() {
                    x.to_owned()
                } else {
                    continue;
                };
                let mac = if let Some(x) = record.get_property_val_str("mac") {
                    x.to_owned()
                } else {
                    String::new()
                };
                let mut ip = None;
                for addr in record.get_addresses().iter() {
                    if let ScopedIp::V4(addr) = addr {
                        if addr.addr().is_private() && addr.addr().octets() != [10, 20, 17, 1] {
                            ip = Some(addr.addr().to_string());
                            break;
                        }
                    }
                }
                let ip = if let Some(ip) = ip { ip } else { continue };

                if let Some(device) = devices.iter_mut().find(|x| x.name == name) {
                    device.ip = ip;
                } else {
                    devices.push(DeviceInfo { name, mac, ip, online: true })
                }
            }
            ServiceEvent::SearchStopped(_) => {}
            _ => {}
        }
    }
    devices.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(devices)
}
