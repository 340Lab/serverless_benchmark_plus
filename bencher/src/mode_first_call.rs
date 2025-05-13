use crate::{
    config::Config, metric::Recorder, mode_call_once, parse::Cli, parse_platform::Platform,
    parse_test_mode::TestMode, PlatformOps, PlatformOpsBind,
};

pub async fn prepare(platform: &mut PlatformOpsBind, config: &Config, cli: Cli) {
    platform.remove_all_fn().await;
    platform.prepare_apps_bin(cli.target_apps(), config).await;
    // cli.prepare_img(&seed);
}

pub async fn call(platform: &mut PlatformOpsBind, cli: Cli, config: &Config) {
    let app = cli.app().expect("first call mode must have app name");
    let func = cli.func().expect("first call mode must have func name");

    let mut recorder = Recorder::new(app.to_string(), TestMode::from(&cli), Platform::from(&cli));

    let mut metrics = vec![];
    for _ in 0..20 {
        platform.upload_fn("simple_demo", "").await;
        let m = mode_call_once::call(platform, cli.clone(), config).await; //self.call_once(cli.clone()).await;
        recorder.record(m.clone());
        // prometheus::upload_fn_call_metric("simple_demo", &m).await;
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
        metrics.iter().map(|v| v.get_req_trans_time()).sum::<u64>() as f32 / metrics.len() as f32
    );

    println!(
        "- app verify time: {}",
        metrics.iter().map(|v| v.get_app_verify_time()).sum::<u64>() as f32 / metrics.len() as f32
    );

    println!(
        "- cold start time: {}",
        metrics.iter().map(|v| v.get_cold_start_time()).sum::<u64>() as f32 / metrics.len() as f32
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
