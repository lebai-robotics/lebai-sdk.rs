use lebai_sdk::RobotPort;
use std::time::{Duration, SystemTime};

#[tokio::test]
async fn test() {
    println!("version: {}", lebai_sdk::version().unwrap());
    let lebai = lebai_sdk::connect("192.168.2.101".into(), Some(RobotPort::Simu(true)), None)
        .await
        .unwrap();
    println!("{:?}", lebai.get_kin_data().await.unwrap());
    for i in 0..10 {
        let lebai_bak = lebai.clone();
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
            rt.block_on(async {
                let lebai = lebai_bak;
                let mut start_time = SystemTime::now();
                let mut num = 0;
                loop {
                    if let Err(e) = lebai.get_kin_data().await {
                        println!("{i}: {:?}", e);
                    }
                    num += 1;
                    let now = SystemTime::now();
                    if now.duration_since(start_time).unwrap() > Duration::from_secs(1) {
                        start_time = now;
                        println!("{i}: {:?}", num);
                    }
                }
            });
        });
    }
    core::future::pending().await
}
