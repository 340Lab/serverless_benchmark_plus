use super::Cli;
use base64::encode;
use image::io::Reader as ImageReader;
use image::{ImageBuffer, RgbImage};
use rand::{Rng, SeedableRng};
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::{format, Debug};
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;
use std::process::{self, Command as Process};
use std::time::{SystemTime, UNIX_EPOCH};

use super::PlatformOpsBind;
use crate::parse_app::App;
use crate::{Metric, PlatformOps, SpecTarget, BUCKET};

#[derive(Default)]
pub struct Sequential(Option<PlatformOpsBind>);

impl Sequential {}

impl SpecTarget for Sequential {
    fn app(&self) -> App {
        App::Sequential
    }
    fn set_platform(&mut self, platform: PlatformOpsBind) {
        self.0 = Some(platform);
    }
    fn get_platform(&mut self) -> &mut PlatformOpsBind {
        self.0.as_mut().unwrap()
    }
    async fn prepare_once(&mut self, seed: String, cli: Cli) {
        self.get_platform().remove_all_fn().await;
        self.get_platform()
            .upload_fn("parallel_composition", "")
            .await;
    }

    async fn call_once(&mut self, cli: Cli) -> Metric {
        let arg = Args { loopTime: 10000000 };

        let start_call_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64;
        let output = self
            .get_platform()
            .call_fn(
                "parallel_composition",
                "sequential",
                &serde_json::to_value(arg).unwrap(),
            )
            .await;

        let res: serde_json::Value = serde_json::from_str(&output).unwrap();
        let req_arrive_time = res.get("req_arrive_time").unwrap().as_u64().unwrap();
        let bf_exec_time = res.get("bf_exec_time").unwrap().as_u64().unwrap();
        let recover_begin_time = res.get("recover_begin_time").unwrap().as_u64().unwrap();
        let fn_start_ms = res.get("fn_start_time").unwrap().as_u64().unwrap();
        let fn_end_ms = res.get("fn_end_time").unwrap().as_u64().unwrap();
        let receive_resp_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64;

        println!("debug output: {:?}", output);
        println!(
            "\ntotal request latency: {}",
            receive_resp_time - start_call_ms
        );

        println!("- req trans time: {}", req_arrive_time - start_call_ms);
        println!("- app verify time: {}", bf_exec_time - req_arrive_time);
        println!("- cold start time: {}", recover_begin_time - bf_exec_time);
        println!("- cold start time2: {}", fn_start_ms - recover_begin_time);
        println!("- exec time:{}", fn_end_ms - fn_start_ms);
        if fn_end_ms > receive_resp_time {
            println!(
                "- system time is not synced, lag with {} ms",
                fn_end_ms - receive_resp_time
            );
        } else {
            println!("- receive resp time: {}", receive_resp_time - fn_end_ms);
        }

        Metric {
            start_call_time: start_call_ms,
            req_arrive_time,
            bf_exec_time,
            recover_begin_time,
            fn_start_time: fn_start_ms,
            fn_end_time: fn_end_ms,
            receive_resp_time,
        }
    }

    async fn prepare_first_call(&mut self, seed: String, cli: Cli) {
        self.get_platform().remove_all_fn().await;
    }

    async fn call_first_call(&mut self, cli: Cli) {
        let mut metrics = vec![];
        for _ in 0..20 {
            self.get_platform()
                .upload_fn("parallel_composition", "")
                .await;
            metrics.push(self.call_once(cli.clone()).await);
        }

        println!(
            "\ntotal request latency: {}",
            metrics.iter().map(|v| v.get_total_req()).sum::<u64>() as f32 / metrics.len() as f32
        );

        println!(
            "- req trans time: {}",
            metrics.iter().map(|v| v.get_req_trans_time()).sum::<u64>() as f32
                / metrics.len() as f32
        );

        println!(
            "- app verify time: {}",
            metrics.iter().map(|v| v.get_app_verify_time()).sum::<u64>() as f32
                / metrics.len() as f32
        );

        println!(
            "- cold start time: {}",
            metrics.iter().map(|v| v.get_cold_start_time()).sum::<u64>() as f32
                / metrics.len() as f32
        );

        println!(
            "- cold start time2: {}",
            metrics
                .iter()
                .map(|v| v.get_cold_start_time2())
                .sum::<u64>() as f32
                / metrics.len() as f32
        );

        println!(
            "- exec time: {}",
            metrics.iter().map(|v| v.get_exec_time()).sum::<u64>() as f32 / metrics.len() as f32
        );
        // println!("- app verify time: {}", bf_exec_time - req_arrive_time);
        // println!("- cold start time: {}", recover_begin_time - bf_exec_time);
        // println!("- cold start time2: {}", fn_start_ms - recover_begin_time);
        // println!("- exec time:{}", fn_end_ms - fn_start_ms);
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Args {
    loopTime: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Resp {
    resized_image: String,
}
