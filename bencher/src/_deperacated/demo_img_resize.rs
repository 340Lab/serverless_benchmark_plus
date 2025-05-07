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
use crate::metric::Recorder;
use crate::parse_app::App;
use crate::parse_platform::Platform;
use crate::parse_test_mode::TestMode;
use crate::{prometheus, Metric, PlatformOps, SpecTarget, BUCKET};

#[derive(Default)]
pub struct ImgResize(Option<PlatformOpsBind>);

impl ImgResize {
    fn prepare_img(&self, seed: &str) {
        // mkdir
        if fs::metadata("img_resize").is_ok() {
            return;
        }
        let _ = fs::create_dir_all("img_resize");

        // 使用种子初始化随机数生成器
        let mut rng = rand::rngs::StdRng::seed_from_u64(hash_str(seed));

        for i in 0..100 {
            // 生成随机宽度和高度
            let width = rng.gen_range(100..1000);
            let height = rng.gen_range(100..1000);

            // 创建空白图片
            let mut img: RgbImage = ImageBuffer::new(width, height);

            // 填充图片的每个像素
            for x in 0..width {
                for y in 0..height {
                    let pixel = img.get_pixel_mut(x, y);
                    *pixel = image::Rgb([rng.gen(), rng.gen(), rng.gen()]);
                }
            }

            img.save_with_format(
                format!("img_resize/image_{}.jpg", i),
                image::ImageFormat::Jpeg,
            )
            .unwrap();
        }
    }
}

impl SpecTarget for ImgResize {
    fn app(&self) -> App {
        App::ImgResize
    }
    fn set_platform(&mut self, platform: PlatformOpsBind) {
        self.0 = Some(platform);
    }
    fn get_platform(&mut self) -> &mut PlatformOpsBind {
        self.0.as_mut().unwrap()
    }
    async fn prepare_once(&mut self, seed: String, cli: Cli) {
        self.get_platform().remove_all_fn().await;
        self.get_platform().upload_fn("img_resize", "").await;
        self.prepare_img(&seed);
    }

    async fn call_once(&mut self, cli: Cli) -> Metric {
        // read image from file
        let img = fs::read("img_resize/image_0.jpg").unwrap();

        BUCKET
            // .lock()
            // .await
            .put_object(format!("image_{}.jpg", 0), &img)
            .await
            .unwrap();

        let arg = Args {
            image_s3_path: format!("image_{}.jpg", 0),
            target_width: 50,
            target_height: 50,
        };

        let start_call_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64;
        let output = self
            .get_platform()
            .call_fn("img_resize", "resize", &serde_json::to_value(arg).unwrap())
            .await;
        // tracing::info!("debug output {}", output);
        let res: serde_json::Value = serde_json::from_str(&output).unwrap_or_else(|e| {
            tracing::error!("failed to parse json: {}", e);
            panic!("output is not json: '{}'", output);
        });

        let mut req_arrive_time = res
            .get("req_arrive_time")
            .map(|v| v.as_u64().unwrap())
            .unwrap_or(0);

        let mut bf_exec_time = res
            .get("bf_exec_time")
            .map(|v| v.as_u64().unwrap())
            .unwrap_or(0);

        let mut recover_begin_time = res
            .get("recover_begin_time")
            .map(|v| v.as_u64().unwrap())
            .unwrap_or(0);

        let fn_start_ms = res.get("fn_start_time").unwrap().as_u64().unwrap();
        {
            if req_arrive_time == 0 {
                req_arrive_time = fn_start_ms;
            }
            if bf_exec_time == 0 {
                bf_exec_time = fn_start_ms;
            }
            if recover_begin_time == 0 {
                recover_begin_time = fn_start_ms;
            }
        }

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

        let res: Resp = serde_json::from_str(&output).expect("Failed to parse JSON response");
        let res = BUCKET.get_object(&res.resized_image).await.unwrap();
        std::fs::write("resized_image.jpg", res.as_slice()).unwrap();

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
        self.prepare_img(&seed);
    }

    async fn call_first_call(&mut self, cli: Cli) {
        let mut recorder = Recorder::new(
            self.app().to_string(),
            TestMode::from(&cli),
            Platform::from(&cli),
        );

        let mut metrics = vec![];
        for _ in 0..20 {
            self.get_platform().upload_fn("img_resize", "").await;
            let m = self.call_once(cli.clone()).await;
            recorder.record(m.clone());
            // prometheus::upload_fn_call_metric("img_resize", &m).await;
            metrics.push(m);
        }
        recorder.persist();

        println!("Average metrics:");
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
    image_s3_path: String,
    target_width: i32,
    target_height: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Resp {
    resized_image: String,
}

// #[derive(Debug)]
// enum CallRes<R: Debug> {
//     ProcessOut(std::process::Output),
//     Struct(R),
// }

// pub async fn test(user: &mut GooseUser) -> TransactionResult {
//     let _goose_metrics = user.get("").await?;

//     Ok(())
// }

// 将字符串转换为u64哈希值
fn hash_str(s: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}
