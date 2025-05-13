use tokio::process;

use crate::config::Config;
use crate::parse::Cli;
use crate::platform::PlatformOps;
use std::collections::HashSet;
use std::{collections::HashMap, fs::File, io::BufReader, str::from_utf8};

pub struct PlatfromWl {
    cli: Cli,
    cli: Cli,
    master_url: String,
    worker_url: String,
    upload_url: String,
    // gen_demos: HashSet<String>,
    config: Config,
}

impl PlatfromWl {
    pub fn new(cli: &Cli, config: Config) -> Self {
    pub fn new(cli: &Cli, config: Config) -> Self {
        let mut res = Self {
            cli: cli.clone(),
            cli: cli.clone(),
            master_url: "".to_owned(),
            worker_url: "".to_owned(),
            upload_url: "".to_owned(),
            // gen_demos: HashSet::new(),
            config: config.clone(),
        };

        let file = File::open(cli.cluster_config()).unwrap();
        let reader = BufReader::new(file);
        let config: HashMap<String, HashMap<String, serde_yaml::Value>> =
            serde_yaml::from_reader(reader).unwrap();


        let file = File::open(cli.cluster_config()).unwrap();
        let reader = BufReader::new(file);
        let config: HashMap<String, HashMap<String, serde_yaml::Value>> =
            serde_yaml::from_reader(reader).unwrap();

        for (_, conf) in config {
            if conf.contains_key("is_master") {
                res.master_url = format!(
                    "http://{}",
                    "http://{}",
                    conf.get("ip").unwrap().as_str().unwrap().to_owned()
                );
            } else {
                res.worker_url = format!(
                    "http://{}",
                    "http://{}",
                    conf.get("ip").unwrap().as_str().unwrap().to_owned()
                );
            }
        }

        // 设置 upload_url（根据您的需求调整）
        res.upload_url = format!("{}/upload", res.master_url);

        res
    }
}

impl PlatformOps for PlatfromWl {
    async fn prepare_apps_bin(&self, apps: Vec<String>, config: &Config) {
        let model_apps: Vec<String> = apps
            .clone()
            .into_iter()
            .filter(|app| config.models.contains_key(app))
            .collect();

        for app in model_apps {
    async fn prepare_apps_bin(&self, apps: Vec<String>, config: &Config) {
        let model_apps: Vec<String> = apps
            .clone()
            .into_iter()
            .filter(|app| config.models.contains_key(app))
            .collect();

        for app in model_apps {
            let res = process::Command::new("python3")
                .args(&["../demos/scripts/1.gen_waverless_app.py", &app])
                .args(&["../demos/scripts/1.gen_waverless_app.py", &app])
                .status()
                .await
                .expect(&format!("Failed to gen demo {}", app));
            assert!(res.success(), "Failed to gen demo app: {}", app);
            // self.gen_demos.insert(demo.to_owned());
                .expect(&format!("Failed to gen demo {}", app));
            assert!(res.success(), "Failed to gen demo app: {}", app);
            // self.gen_demos.insert(demo.to_owned());
        }
    }

    fn cli(&self) -> &Cli {
        &self.cli
    }
    async fn remove_all_fn(&self) {}
    async fn upload_fn(&mut self, demo: &str, rename_sub: &str) {
        // if !self.gen_demos.contains(demo) {

        // }
    }

    fn cli(&self) -> &Cli {
        &self.cli
    }
    async fn remove_all_fn(&self) {}
    async fn upload_fn(&mut self, demo: &str, rename_sub: &str) {
        // if !self.gen_demos.contains(demo) {

        // }
        process::Command::new("python3")
            .args(&[
                "../middlewares/waverless/3.add_func.py",
                demo,
                rename_sub,
                &self.cli().cluster_config(),
            ])
            .args(&[
                "../middlewares/waverless/3.add_func.py",
                demo,
                rename_sub,
                &self.cli().cluster_config(),
            ])
            .status()
            .await
            .expect(&format!("Failed to add func {} as {}", demo, rename_sub));
    }
    async fn call_fn(
        &self,
        app: &str,
        func: &str,
        arg_json_value: &serde_json::Value,
        // _big_data: &Option<Vec<String>>,
    ) -> String {
        println!("{}/{}/{}", self.master_url, app, func);

        let res = reqwest::Client::new()
            .post(format!("{}/{}/{}", self.master_url, app, func))
            .body(serde_json::to_string(&arg_json_value).unwrap())
            .send()
            .await
            .unwrap_or_else(|e| panic!("err: {:?}", e))
            .text()
            .await;
        res.unwrap()
    }
    async fn write_data(&self, key: &str, data: &[u8]) {
        let client = reqwest::Client::new();
        let upload_endpoint = format!("{}/data_upload", self.master_url);

        // 使用 multipart 格式上传数据
        let part = reqwest::multipart::Part::bytes(data.to_vec()).file_name("data"); // 可选：为上传的数据提供文件名

        let form = reqwest::multipart::Form::new()
            .text("key", key.to_string())
            .part("file", part);

        match client.post(&upload_endpoint).multipart(form).send().await {
            Ok(response) => {
                if !response.status().is_success() {
                    eprintln!("上传失败，状态码: {}", response.status());
                    if let Ok(text) = response.text().await {
                        eprintln!("错误信息: {}", text);
                    }
                }
            }
            Err(e) => {
                eprintln!("上传请求发送失败: {}", e);
            }
        }
    }
}
