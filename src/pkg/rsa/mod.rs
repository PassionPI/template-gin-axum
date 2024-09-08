use anyhow::{Context, Result};
use base64::prelude::*;
use rsa::{
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding},
    Oaep, RsaPrivateKey, RsaPublicKey,
};
use sha2::Sha256;

use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
    str,
};

const PEM_DIR: &'static str = "pem";
const PUBLIC_NAME: &'static str = "public.pem";
const PRIVATE_NAME: &'static str = "private.pem";

fn get_private_path(private_dir: &str) -> String {
    format!("{}/{}/{}", private_dir, PEM_DIR, PRIVATE_NAME)
}

fn get_public_path(private_dir: &str) -> String {
    format!("{}/{}/{}", private_dir, PEM_DIR, PUBLIC_NAME)
}

fn read_pem_file(file_path: &str) -> String {
    // 打开文件
    let mut file = File::open(file_path).expect("Failed to open file");

    // 创建一个字符串来存储文件内容
    let mut contents = String::new();

    // 读取文件内容到字符串中
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    contents
}

fn write_pem_file(file_path: &str, content: &str) {
    let dir = Path::new(file_path)
        .parent()
        .expect("No parent directory found");

    fs::create_dir_all(dir).expect("Failed to create directory");

    let mut file = File::create(file_path).expect("Failed to create file");

    file.write_all(content.as_bytes())
        .expect("Failed to write private key");
}

#[derive(Clone)]
pub struct Rsa {
    private_key: RsaPrivateKey,
    public_key: RsaPublicKey,
    public_pem: String,
}

impl Rsa {
    pub fn new(private_dir: &str) -> Self {
        let private_path = get_private_path(private_dir);
        let public_path = get_public_path(private_dir);

        if fs::metadata(&private_path).is_ok() == false {
            println!("Generate pem files.");

            let mut rng = rand::thread_rng();

            let bits = 2048;
            let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a key");
            let public_key = RsaPublicKey::from(&private_key);
            let private_pem = private_key
                .to_pkcs8_pem(LineEnding::LF)
                .expect("Failed to write private key");
            let public_pem = public_key
                .to_public_key_pem(LineEnding::LF)
                .expect("Failed to write public key");

            write_pem_file(&private_path, &private_pem);
            write_pem_file(&public_path, &public_pem);
        } else {
            println!("Pem files already exist.");
        }

        let private_pem = read_pem_file(&private_path);
        let public_pem = read_pem_file(&public_path);

        let private_key =
            RsaPrivateKey::from_pkcs8_pem(&private_pem).expect("Failed to parse private_key file");
        let public_key = RsaPublicKey::from_public_key_pem(&public_pem)
            .expect("Failed to parse public_key file");

        let instance = Self {
            private_key,
            public_key,
            public_pem,
        };

        instance.assert();

        instance
    }
    fn assert(&self) {
        let enc = self.encrypt("hello".as_bytes());
        let dec = self
            .decrypt(&enc)
            .expect("Failed to decrypt, check private pem file");
        assert!(dec == "hello".as_bytes());
    }
    pub fn get_public_pem(&self) -> &str {
        &self.public_pem
    }
    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let padding = Oaep::new::<Sha256>();
        self.public_key
            .encrypt(&mut rng, padding, &data[..])
            .expect("Failed to encrypt")
    }
    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let padding = Oaep::new::<Sha256>();
        Ok(self
            .private_key
            .decrypt(padding, data)
            .context("Fail to decrypt &[u8]")?)
    }
    pub fn decrypt_base64(&self, data: &str) -> Result<Vec<u8>> {
        let u8 = &BASE64_STANDARD
            .decode(data)
            .context("Failed to decode base64")?;
        Ok(self.decrypt(u8)?)
    }
}
