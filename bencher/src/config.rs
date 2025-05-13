use serde_yaml::Value;
use std::{collections::HashMap, fs::File, io::BufReader};

#[derive(serde::Deserialize, Debug, Clone)]
pub struct FnDetails {
    pub args: Option<HashMap<String, Value>>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct AppFnEntry {
    pub prepare_data: Vec<String>,
    pub prepare_scripts: Vec<String>,
    pub fns: HashMap<String, FnDetails>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct AppFnReplica {
    pub source: String,
    pub fns: HashMap<String, FnDetails>,
}

// app:
//   prepare_data:
//   - file1
//   prepare_scripts:
//   - script1
//   fns:
//     fn1:
//       args:
// pub type AppFnEntries = HashMap<String, AppFnEntry>;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct MinioConfig {
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
    pub compose_path: String,
}

impl MinioConfig {
    pub fn one_line(&self) -> String {
        format!("{},{},{}", self.endpoint, self.access_key, self.secret_key)
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Config {
    pub models: HashMap<String, AppFnEntry>,
    pub replicas: HashMap<String, AppFnReplica>,
    pub benchlist: HashMap<String, ()>,
    pub minio: MinioConfig,
}

impl Config {
    // todo: add replica support
    pub fn get(&self, app: &str) -> Option<&AppFnEntry> {
        self.models.get(app)
    }

    pub fn get_fn_details(&self, app: &str, func: &str) -> Option<FnDetails> {
        let mut fndetail = if self.models.contains_key(app) {
            self.models.get(app).unwrap().fns.get(func).cloned()
        } else if let Some(replica) = self.replicas.get(app) {
            // replica args will cover model args
            let source = replica.source.clone();
            let source_fn_details = self.get_fn_details(&source, func);
            if let Some(mut source_fn_details) = source_fn_details {
                // cover by replica args
                for (key, value) in replica.fns.get(func).unwrap().args.as_ref().unwrap() {
                    source_fn_details
                        .args
                        .as_mut()
                        .unwrap()
                        .insert(key.clone(), value.clone());
                }
                Some(source_fn_details)
            } else {
                None
            }
        } else {
            None
        };
        fndetail.map(|mut fndetail| {
            let args = fndetail.args.as_mut().unwrap();
            if let Some(Value::Bool(true)) = args.get("use_minio") {
                args.insert("use_minio".to_owned(), Value::from(self.minio.one_line()));
            } else {
                // remove use_minio
                args.remove("use_minio");
            }

            fndetail
        })
    }
}

pub fn load_config() -> Config {
    let f = File::open("app_fn_entries.yaml").expect("open app_fn_entries.yaml failed");
    let freader = BufReader::new(f);
    let app_fn_entries: Config = serde_yaml::from_reader(freader).unwrap_or_else(|e| {
        panic!("parse 'app_fn_entries.yaml' failed with {:?}", e);
    });
    app_fn_entries
}
