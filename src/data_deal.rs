///! #处理app界面传递过来的数据。用于保存密码等信息
use tokio::{
    io::{BufWriter,AsyncWriteExt}, 
    sync::mpsc};
use crate::app::KeyInfo;
use tokio::fs::{self,OpenOptions};
use toml::{Value, Table};
use ring::{
    error::Unspecified,
    rand::*,
    aead::*,
};

pub enum KeyRingEn {
    Encryption(String),
    Disencryption(String),
}

pub enum KeyRingDis {
    EncryptionRep(String),
    DisencryptionRep(String),
}

struct CounterNonceSequence(u32);

impl NonceSequence for CounterNonceSequence {
    // called once for each seal operation
    fn advance(&mut self) -> Result<Nonce, Unspecified> {
        let mut nonce_bytes = vec![0; NONCE_LEN];

        let bytes = self.0.to_be_bytes();
        nonce_bytes[8..].copy_from_slice(&bytes);
        println!("nonce_bytes = {}", hex::encode(&nonce_bytes));

        self.0 += 1; // advance the counter
        Nonce::try_assume_unique_for_key(&nonce_bytes)
    }
}

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
        let mut table_main = Table::new();
        let mut table = Table::new();
        table.insert("账户".to_string(), Value::String(message.get_key_name()));
        table.insert("密码".to_string(), Value::String(message.get_key()));
        table_main.insert(message.get_key_remark(), Value::Table(table));

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

pub async fn key_info_deal(mut rx: mpsc::Receiver<KeyRingEn>, mut tx: mpsc::Sender<KeyRingDis>){
    while let Some(meg) = rx.recv().await{
        
        match meg {
            KeyRingEn::Encryption(msg) =>{
                println!("{}", msg);
                let file = match fs::read_to_string("key_info.toml").await{
                    Ok(file) => file,
                    Err(_) => {
                        let _ = tx.send(KeyRingDis::EncryptionRep(String::from("找不到文件"))).await;
                        continue;
                    }
                };
                //对文件进行加密
                let mut data = file.as_bytes().to_vec();
                let rand = SystemRandom::new();
                // Generate a new symmetric encryption key
                let mut key_bytes = vec![0; AES_256_GCM.key_len()];
                rand.fill(&mut key_bytes).expect("随机数生成失败");
                println!("key_bytes = {}", hex::encode(&key_bytes));
                let unbound_key = UnboundKey::new(&AES_256_GCM, &key_bytes).expect("unbound_key生成失败");
                let nonce_sequence = CounterNonceSequence(1);
                let mut sealing_key = SealingKey::new(unbound_key, nonce_sequence);
                let associated_data = Aad::from(b"additional public data");
                let tag = sealing_key.seal_in_place_separate_tag(associated_data, &mut data).expect("tag生成失败");
                let cypher_text_with_tag = [&data, tag.as_ref()].concat();
                let en_string = cypher_text_with_tag.iter()
                                    .map(|&num| num.to_string())
                                    .collect::<Vec<String>>()
                                    .join("");
                let _ = tx.send(KeyRingDis::EncryptionRep(en_string)).await;
                let _ = fs::write("key_info_en.txt", data).await;

            }
            KeyRingEn::Disencryption(_) =>{

            }
        }
    }
}