///! #处理app界面传递过来的数据。用于保存密码等信息
use tokio::{
    io::AsyncWriteExt, 
    sync::mpsc, 
    BufWriter};
use tokio::fs::OpenOptions;
use crate::app::KeyInfo;


pub async fn receive_app_data(mut rx: mpsc::Receiver<KeyInfo>) {
    let file = match OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .open("./key_info.toml").await {
                            Ok(file) => file,
                            Err(e) => panic!("打开或创建文件key_info.toml 失败：{:?}", e),
    };

    let mut write = BufWriter::new(file);
    while let Some(message) = rx.recv().await {
        let serialized = serde_json::to_string(&message).unwrap();
        if let Err(e) = write.write((serialized + "\n").as_bytes()).await {
            eprintln!("写入数据失败: {:?}", e);
            break;
        }
        if let Err(e) = write.flush().await {
            eprintln!("刷新缓冲区失败: {:?}", e);
            break;
        }
        println!("Received: {:?}", message);
    }
}