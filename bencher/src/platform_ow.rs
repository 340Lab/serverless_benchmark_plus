use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::BufReader,
    str::from_utf8,
};
use tokio::process::{self, Command};

use crate::{config::Config, parse::Cli, platform::PlatformOps};

pub struct PlatfromOw {
    pub gen_demos: HashSet<String>,
    cli: Cli,
    config: Config,
    cli: Cli,
    config: Config,
}

impl PlatfromOw {
    pub fn new(cli: &Cli, config: &Config) -> Self {
        Self {
impl PlatfromOw {
    pub fn new(cli: &Cli, config: &Config) -> Self {
        Self {
            gen_demos: HashSet::new(),
            cli: cli.clone(),
            config: config.clone(),
            cli: cli.clone(),
            config: config.clone(),
        }
    }
}

// impl Default for PlatfromOw {
//     fn default() -> Self {
//         PlatfromOw {
//             gen_demos: HashSet::new(),
//         }
//     }
// }

// impl Default for PlatfromOw {
//     fn default() -> Self {
//         PlatfromOw {
//             gen_demos: HashSet::new(),
//         }
//     }
// }

impl PlatformOps for PlatfromOw {
    async fn prepare_apps_bin(&self, apps: Vec<String>, config: &Config) {
        process::Command::new("python3")
            .args(&["../middlewares/openwhisk/7.clean_all_fns.py"])
            .status()
            .await
            .expect("Failed to clean openwhisk fns");

        for app in apps {
            let mut cmd2 = process::Command::new("python3");
            cmd2.args(&["../middlewares/openwhisk/8.add_func.py", &app, &app]);
            tracing::info!("run cmd: {:?}", cmd2);
            let res = cmd2
                .status()
                .await
                .expect(&format!("Failed to add func {}", app));
            assert!(res.success());
        }
    }

    fn cli(&self) -> &Cli {
        &self.cli
    }
    async fn remove_all_fn(&self) {
        process::Command::new("python3")
            .args(&["../middlewares/openwhisk/7.clean_all_fns.py"])
            .status()
            .await
            .expect("Failed to clean openwhisk fns");
    }
    async fn upload_fn(&mut self, demo: &str, rename_sub: &str) {
        if !self.gen_demos.contains(demo) {
            let res = process::Command::new("python3")
                .args(&["../demos/scripts/1.gen_ow_app.py", demo])
                .status()
                .await
                .expect(&format!("Failed to add func {} as {}", demo, rename_sub));
            assert!(res.success());
            self.gen_demos.insert(demo.to_owned());
        }
        let mut cmd2 = process::Command::new("python3");
        cmd2.args(&["../middlewares/openwhisk/8.add_func.py", demo, rename_sub]);
        tracing::info!("run cmd: {:?}", cmd2);
        let res = cmd2
            .status()
            .await
            .expect(&format!("Failed to add func {} as {}", demo, rename_sub));
        assert!(res.success());
    }
    async fn call_fn(
        &self,
        app: &str,
        func: &str,
        arg_json_value: &serde_json::Value,
        // big_data: &Option<Vec<String>>,
        // fn_details: &FnDetails,
    ) -> String {
        let mut p = Command::new("wsk");
        let appfunc = format!("{}_{}", app, func);
        let args = vec!["action", "invoke", "--result", &appfunc];
        p.args(&args);

        for (k, v) in arg_json_value.as_object().unwrap() {
            p.arg("--param");
            p.arg(k);
            p.arg(&match v {
                serde_json::Value::String(s) => s.to_owned(),
                _ => v.to_string(),
            });
        }

        tracing::info!("run cmd: {:?}", p);

        let res = p
            .output()
            .await
            .expect("Failed to execute wsk action invoke");

        if res.status.success() {
            from_utf8(&res.stdout).unwrap().to_owned()
        } else {
            from_utf8(&res.stderr).unwrap().to_owned()
        }
    }
    async fn write_data(&self, key: &str, data: &[u8]) {
        panic!("openwhisk doesn't support write data");
    }
}
