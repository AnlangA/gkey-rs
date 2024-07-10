///! #处理app界面传递过来的数据。用于保存密码等信息
use tokio::{
    io::{BufWriter,AsyncWriteExt}, 
    sync::mpsc};
use crate::app::KeyInfo;
use tokio::fs::{self,OpenOptions};
use toml::{Value, Table};

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
        let mut table_main = toml::value::Table::new();
        let mut table = toml::value::Table::new();
        table.insert("账户".to_string(), toml::Value::String(message.get_key_name()));
        table.insert("密码".to_string(), toml::Value::String(message.get_key()));
        table_main.insert(message.get_key_remark(), toml::Value::Table(table));

        if let Ok(mut new_toml_str) = toml::to_string(&table_main){
            //换行
            new_toml_str = new_toml_str + "\n";
            if let Err(e) = write.write(new_toml_str.as_bytes()).await{
                eprintln!("{:?}",e);
            }
        }
        if let Err(e) = write.flush().await{
            eprintln!("{:?}",e);
        }
    }
}