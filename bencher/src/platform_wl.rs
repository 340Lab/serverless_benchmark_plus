use tokio::process;

use crate::config::Config;
use crate::parse::Cli;
use crate::PlatformOps;
use std::collections::HashSet;
use std::{collections::HashMap, fs::File, io::BufReader, str::from_utf8};

pub struct PlatfromWl {
    cli: Cli,
    master_url: String,
    worker_url: String,
    // gen_demos: HashSet<String>,
    config: Config,
}

impl PlatfromWl {
    pub fn new(cli: &Cli, config: Config) -> Self {
        let mut res = Self {
            cli: cli.clone(),
            master_url: "".to_owned(),
            worker_url: "".to_owned(),
            // gen_demos: HashSet::new(),
            config: config.clone(),
        };

        let file = File::open(cli.cluster_config()).unwrap();
        let reader = BufReader::new(file);
        let config: HashMap<String, HashMap<String, serde_yaml::Value>> =
            serde_yaml::from_reader(reader).unwrap();

        for (_, conf) in config {
            if conf.contains_key("is_master") {
                res.master_url = format!(
                    "http://{}",
                    conf.get("ip").unwrap().as_str().unwrap().to_owned()
                );
            } else {
                res.worker_url = format!(
                    "http://{}",
                    conf.get("ip").unwrap().as_str().unwrap().to_owned()
                );
            }
        }
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
            let res = process::Command::new("python3")
                .args(&["../demos/scripts/1.gen_waverless_app.py", &app])
                .status()
                .await
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
        process::Command::new("python3")
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
    async fn call_fn(&self, app: &str, func: &str, arg_json_value: &serde_json::Value) -> String {
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
}
