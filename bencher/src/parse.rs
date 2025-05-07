use core::panic;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use clap::arg;
use clap::value_parser;
use clap::{command, Command};
use clap::{Parser, Subcommand};
use serde_yaml::Value;

#[derive(Parser, Clone)]
#[command(version, about, long_about = None)]
pub struct Cli {
    // #[arg(action = clap::ArgAction::Count)]
    pub app_fn: String,

    #[arg(long, action = clap::ArgAction::Count)]
    pub prepare: u8,

    #[arg(long, action = clap::ArgAction::Count)]
    pub with_ow: u8,

    #[arg(long, action = clap::ArgAction::Count)]
    pub with_wl: u8,

    #[arg(long, action = clap::ArgAction::Count)]
    pub bench_mode: u8,

    // create many function copy and collect the average cold start
    #[arg(long, action = clap::ArgAction::Count)]
    pub first_call_mode: u8,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct FnDetails {
    pub args: Option<HashMap<String, Value>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct AppFnEntry {
    pub prepare_data: Vec<String>,
    pub prepare_scripts: Vec<String>,
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
pub type AppFnEntries = HashMap<String, AppFnEntry>;

impl Cli {
    pub fn app(&self) -> Option<String> {
        if self.app_fn.find("/").is_none() {
            return None;
        }
        return Some(self.app_fn.split("/").next().unwrap().to_owned());
    }
    pub fn func(&self) -> Option<String> {
        if self.app_fn.find("/").is_none() {
            return None;
        }
        let mut iter = self.app_fn.split("/");
        iter.next();
        return Some(iter.next().unwrap().to_owned());
    }
    pub fn func_details(&self) -> FnDetails {
        let app = self.app().unwrap_or_else(|| {
            panic!("missing app name, cur input is {}", self.app_fn);
        });
        let func = self.func().unwrap_or_else(|| {
            panic!("missing fn name, cur input is {}", self.app_fn);
        });

        //read 'app_fn_entries.yaml'
        let f = File::open("app_fn_entries.yaml").expect("open app_fn_entries.yaml failed");
        let freader = BufReader::new(f);
        let app_fn_entries: AppFnEntries = serde_yaml::from_reader(freader).unwrap_or_else(|e| {
            panic!("parse 'app_fn_entries.yaml' failed with {:?}", e);
        });

        let f = app_fn_entries
            .get(&app)
            .unwrap_or_else(|| panic!("app '{}' is not in app_fn_entries.yaml", app))
            .fns
            .get(&func)
            .unwrap_or_else(|| panic!("function '{}' is not in app '{}' in app_fn_entries.yaml", func, app));
        f.clone()
    }
    pub fn check_app_fn(&self) -> &Self {
        let app = self.app().unwrap_or_else(|| {
            panic!("missing app name, cur input is {}", self.app_fn);
        });
        let func = self.func().unwrap_or_else(|| {
            panic!("missing fn name, cur input is {}", self.app_fn);
        });

        //read 'app_fn_entries.yaml'
        let f = File::open("app_fn_entries.yaml").expect("open app_fn_entries.yaml failed");
        let freader = BufReader::new(f);
        let app_fn_entries: AppFnEntries = serde_yaml::from_reader(freader).unwrap_or_else(|e| {
            panic!("parse 'app_fn_entries.yaml' failed with {:?}", e);
        });

        let _ = app_fn_entries
            .get(&app)
            .unwrap_or_else(|| panic!("app '{}' is not in app_fn_entries.yaml", app))
            .fns
            .get(&func)
            .unwrap_or_else(|| panic!("function '{}' is not in app '{}' in app_fn_entries.yaml", func, app));
        self
    }
    pub fn check_platform(&self) -> &Self {
        assert!(
            !(self.with_ow > 0 && self.with_wl > 0),
            "Cannot run with both OpenWhisk and Waverless"
        );
        self
    }

    pub fn check_mode(&self) -> &Self {
        assert!(
            self.bench_mode + self.first_call_mode <= 1,
            "Cannot test multiple modes at one time {}",
            self.bench_mode + self.first_call_mode
        );
        self
    }
}
