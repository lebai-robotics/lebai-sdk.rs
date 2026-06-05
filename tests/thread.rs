use lebai_sdk::RobotPort;

#[tokio::test]
async fn test() {
    println!("version: {}", lebai_sdk::version().unwrap());
    let lebai = lebai_sdk::connect("47.116.126.158".into(), Some(RobotPort::Simu(true)), None).await.unwrap();
    println!("{:?}", lebai.get_kin_data().await.unwrap());
    for i in 0..10 {
        let lebai_bak = lebai.clone();
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
            rt.block_on(async {
                let lebai = lebai_bak;
                loop {
                    println!("{i}: {:?}", lebai.get_kin_data().await.is_ok());
                }
            });
        });
    }
    core::future::pending().await
}
