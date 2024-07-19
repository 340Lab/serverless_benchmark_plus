use super::Cli;
use base64::encode;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::{format, Debug};
use std::fs::{self, File};
use std::path::Path;
use std::process::{self, Command as Process};
use std::time::{SystemTime, UNIX_EPOCH};

use super::PlatformOpsBind;
use crate::{Metric, PlatformOps, SpecTarget, BUCKET};

#[derive(Default)]
pub struct WordCount(Option<PlatformOpsBind>);

impl WordCount {
    fn prepare_texts(&self, seed: &str) {
        // mkdir
        if fs::metadata("word_count").is_err() {
            let _ = fs::create_dir_all("word_count");
        }

        // 检查文件是否存在，如果不存在则生成
        if fs::metadata("word_count/random_words.txt").is_err() {
            // 使用种子初始化随机数生成器
            let mut rng = rand::rngs::StdRng::seed_from_u64(hash_str(seed));
            let text = generate_random_text(&mut rng, 15000000);
            if let Err(err) = fs::write("word_count/random_words.txt", &text) {
                println!("Error writing file: {}", err);
            }
        }
    }
}

impl SpecTarget for WordCount {
    fn set_platform(&mut self, platform: PlatformOpsBind) {
        self.0 = Some(platform);
    }

    fn get_platform(&mut self) -> &mut PlatformOpsBind {
        self.0.as_mut().unwrap()
    }

    async fn prepare_once(&mut self, seed: String, cli: Cli) {
        self.get_platform().remove_all_fn().await;
        self.get_platform().upload_fn("word_count", "").await;
        self.prepare_texts(&seed);
    }

    async fn call_once(&mut self, cli: Cli) -> Metric {
        
        let file_path = "word_count/random_words.txt";

        // 读取文件内容
        let text = fs::read(file_path).expect("Failed to read file");

        // 上传文件到存储桶
        BUCKET.put_object("random_words.txt", &text)
            .await
            .expect("Failed to upload file to bucket");
            
        let arg = Args {
            text_s3_path: "random_words.txt".to_string(),
        };

        let start_call_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64;
        let output = self
            .get_platform()
            .call_fn("word_count", "count", &serde_json::to_value(arg).unwrap())
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

        let slice_keys: Vec<String> = res
        .as_object()
        .unwrap()
        .iter()
        .filter_map(|(key, _value)| {
            if key.starts_with("slice_wordcount_slice") {
                Some(key.clone())
            } else {
                None
            }
        })
        .collect();

        // Fetch objects from the bucket for each slice key
        for key in slice_keys {
            let slice_path = res.get(&key).unwrap().as_str().unwrap();

            let slice_content:  Vec<u8> = BUCKET.get_object(slice_path)
                .await
                .expect("Failed to get slice from bucket")
                .to_vec();

            let file_name = Path::new(slice_path).file_name().unwrap().to_str().unwrap();
            let local_file_path = format!("word_count/{}", file_name);
        
            // Write slice content to local file
            std::fs::write(&local_file_path, &slice_content).expect("Failed to write slice to file");
           
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
        self.prepare_texts(&seed);
    }

    async fn call_first_call(&mut self, cli: Cli) {
        let mut metrics = vec![];
        for _ in 0..20 {
            self.get_platform().upload_fn("word_count", "").await;
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
    text_s3_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Resp {
    result_txt: String,
}

// 将字符串转换为u64哈希值
fn hash_str(s: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

fn generate_random_text<R: Rng>(rng: &mut R, length: usize) -> Vec<u8> {
    let mut text = Vec::with_capacity(length);
    let chars: &[u8] = b"abcdefghijklmnopqrstuvwxyz ";

    for i in 0..length {
        if i > 0 && i % 80 == 0 {
            // 每80个字符插入一个换行符
            text.push(b'\n');
        } else {
            text.push(*chars.choose(rng).unwrap());
        }
    }

    text
}

