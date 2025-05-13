use std::{
    collections::HashMap,
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

use clap::Args;
use serde_yaml::Value;

use crate::{config::Config, parse::Cli, Metric, PlatformOps, PlatformOpsBind};

pub async fn prepare(platform: &mut PlatformOpsBind, seed: String, cli: Cli) {
    platform.remove_all_fn().await;
    platform.upload_fn(&cli.app().unwrap(), "").await;
    // self.prepare_img(&seed);
}

pub async fn call(platform: &mut PlatformOpsBind, cli: Cli, config: &Config) -> Metric {
    // read image from file
    // let img = fs::read("img_resize/image_0.jpg").unwrap();
    let app = cli.app().unwrap();
    let func = cli.func().unwrap();
    // BUCKET
    //     // .lock()
    //     // .await
    //     .put_object(format!("image_{}.jpg", 0), &img)
    //     .await
    //     .unwrap();

    let fndetail = config.get_fn_details(&app, &func).unwrap();
    // let args = cli.func_details().args;
    // let arg = Args {
    //     image_s3_path: format!("image_{}.jpg", 0),
    //     target_width: 50,
    //     target_height: 50,
    // };

    let start_call_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64;
    // let output = platform
    //     .call_fn("img_resize", "resize", &serde_json::to_value(args).unwrap())
    //     .await;
    let output = platform
        .call_fn(
            &cli.app().unwrap(),
            &cli.func().unwrap(),
            &serde_json::to_value(fndetail.args).unwrap(),
        )
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

    // |
    println!("debug output: {:?}", output);
    println!(
        "\ntotal request latency: {} ms",
        receive_resp_time - start_call_ms
    );

    println!("- req trans time: {}", req_arrive_time - start_call_ms);
    // if recover_begin_time<=req_arrive_time{

    // }
    // 系统调用函数时刻 - 请求到达系统
    println!("- app verify time: {}", bf_exec_time - req_arrive_time);
    // 开始冷启动时刻
    println!(
        "- cold start time: {}",
        if recover_begin_time > bf_exec_time {
            recover_begin_time - bf_exec_time
        } else {
            0
        }
    );

    // 冷启动和请求到达系统谁更新
    println!(
        "- cold start time2: {}",
        fn_start_ms - recover_begin_time.max(req_arrive_time)
    );

    println!("- exec time:{}", fn_end_ms - fn_start_ms);
    if fn_end_ms > receive_resp_time {
        println!(
            "- system time is not synced, lag with {} ms",
            fn_end_ms - receive_resp_time
        );
    } else {
        println!("- receive resp time: {}", receive_resp_time - fn_end_ms);
    }

    // let res: Resp = serde_json::from_str(&output).expect("Failed to parse JSON response");
    // let res = BUCKET.get_object(&res.resized_image).await.unwrap();
    // std::fs::write("resized_image.jpg", res.as_slice()).unwrap();

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
