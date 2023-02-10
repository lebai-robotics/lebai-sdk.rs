use cmod::Result;
use core::time::Duration;
use futures_util::future::{select, Either};
use mdns_sd::{ServiceDaemon, ServiceEvent};
use proto::lebai::multi_devices::DeviceInfo;

const SERVICE_NAME: &'static str = "_lebai._tcp.local.";

pub async fn discover_devices(time: u32) -> Result<Vec<DeviceInfo>> {
    let _rt = crate::runtime::RT.enter();
    let mdns = ServiceDaemon::new().map_err(|e| e.to_string())?;
    let receiver = mdns.browse(&SERVICE_NAME).map_err(|e| e.to_string())?;
    let mut devices: Vec<DeviceInfo> = Vec::new();
    let mut task_wait = futures_timer::Delay::new(Duration::from_secs(time as u64));
    let mut task_recv = receiver.recv_async();
    loop {
        match select(task_recv, task_wait).await {
            Either::Left((res, fut_wait)) => {
                let res = if let Ok(res) = res { res } else { break };
                task_wait = fut_wait;
                task_recv = receiver.recv_async();

                match res {
                    ServiceEvent::ServiceResolved(record) => {
                        let name = if let Some(x) = record.get_fullname().split(".").nth(0) {
                            x.to_owned()
                        } else {
                            continue;
                        };
                        let mut ip = None;
                        for addr in record.get_addresses().iter() {
                            if addr.is_private() {
                                ip = Some(addr.to_string());
                                break;
                            }
                        }
                        let ip = if let Some(ip) = ip { ip } else { continue };

                        if let Some(device) = devices.iter_mut().find(|x| x.name == name) {
                            device.ip = ip;
                        } else {
                            devices.push(DeviceInfo {
                                name,
                                mac: String::new(),
                                ip,
                                online: true,
                            })
                        }
                    }
                    _ => {}
                }
            }
            Either::Right(_) => break,
        }
    }
    Ok(devices)
}
