mod common_prepare;
mod config;
mod data_api;
mod fs;
mod metric;
mod minio;
mod mode_bench;
mod mode_call_once;
mod mode_first_call;
mod mode_prepare;
mod parse;
mod parse_platform;
mod parse_test_mode;
mod platform;
mod platform_ow;
mod platform_wl;
mod prometheus;
mod test_call_once;
mod util;
// mod reponse;

use async_trait::async_trait;

use clap::Parser;
use config::Config;
use config::FnDetails;
use enum_dispatch::enum_dispatch;
use goose::prelude::*;
use parse::Cli;
use platform::PlatformOps;
use platform::PlatformOpsBind;
use s3::creds::Credentials;
use s3::Bucket;
use s3::BucketConfiguration;
use s3::Region;
use serde::Deserialize;
use serde::Serialize;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use std::sync::mpsc;
use std::time::Duration;
use tokio::sync::oneshot;
use tokio::sync::Mutex;
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

fn is_bench_mode(cli: &Cli) -> bool {
    cli.bench_mode > 0
}

fn is_first_call_mode(cli: &Cli) -> bool {
    cli.first_call_mode > 0
}

fn is_prepare_mode(cli: &Cli) -> bool {
    cli.prepare > 0
}

fn is_once_mode(cli: &Cli) -> bool {
    !is_bench_mode(cli) && !is_first_call_mode(cli)
}
// #[enum_dispatch(SpecTarget)]
// enum SpecTargetBind {
//     ImgResize(demo_img_resize::ImgResize),
//     WordCount(demo_word_count::WordCount),
//     Parallel(demo_parallel::Parallel),
//     Sequential(demo_sequential::Sequential),
// }

/// unit: ms
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Metric {
    pub start_call_time: u64,
    pub req_arrive_time: u64,
    pub bf_exec_time: u64,
    pub recover_begin_time: u64,
    pub fn_start_time: u64,
    pub fn_end_time: u64,
    pub receive_resp_time: u64,
}

impl Metric {
    // println!(
    //     "\ntotal request latency: {}",
    //     receive_resp_time - start_call_ms
    // );

    // println!("- req trans time: {}", req_arrive_time - start_call_ms);
    // println!("- app verify time: {}", bf_exec_time - req_arrive_time);
    // println!("- cold start time: {}", recover_begin_time - bf_exec_time);
    // println!("- cold start time2: {}", fn_start_ms - recover_begin_time);
    // println!("- exec time:{}", fn_end_ms - fn_start_ms);

    fn get_total_req(&self) -> u64 {
        self.receive_resp_time - self.start_call_time
    }
    fn get_req_trans_time(&self) -> u64 {
        self.req_arrive_time - self.start_call_time
    }
    fn get_app_verify_time(&self) -> u64 {
        self.bf_exec_time - self.req_arrive_time
    }
    fn get_cold_start_time(&self) -> u64 {
        self.recover_begin_time - self.bf_exec_time
    }
    fn get_cold_start_time2(&self) -> u64 {
        self.fn_start_time - self.recover_begin_time
    }
    fn get_exec_time(&self) -> u64 {
        self.fn_end_time - self.fn_start_time
    }
}

// #[enum_dispatch]
// trait SpecTarget: Send + 'static {
//     fn app(&self) -> App;
//     fn set_platform(&mut self, platform: PlatformOpsBind);
//     fn get_platform(&mut self) -> &mut PlatformOpsBind;

//     async fn prepare_once(&mut self, _seed: String, _cli: Cli) {
//         unimplemented!()
//     }
//     async fn call_once(&mut self, _cli: Cli) -> Metric {
//         unimplemented!()
//     }
//     async fn prepare_bench(&mut self, _seed: String, _cli: Cli) {}
//     async fn call_bench(&mut self, _cli: Cli) {
//         unimplemented!()
//     }
//     async fn prepare_first_call(&mut self, _seed: String, _cli: Cli) {
//         unimplemented!()
//     }
//     async fn call_first_call(&mut self, _cli: Cli) {
//         unimplemented!()
//     }
// }

// pub trait PlatformOpsExt: PlatformOps {
//     fn config_path_string(&self) -> String {
//         self.cli().config_path()
//     }
// }
// pub struct CallRes {
//     out: String,
//     err: String,
// }

pub fn start_tracing() {
    let my_filter = tracing_subscriber::filter::filter_fn(|v| {
        // println!("{}", v.module_path().unwrap());
        // println!("{}", v.name());
        // if v.module_path().unwrap().contains("quinn_proto") {
        //     return false;
        // }

        // if v.module_path().unwrap().contains("qp2p::wire_msg") {
        //     return false;
        // }

        // println!("{}", v.target());
        if let Some(mp) = v.module_path() {
            if mp.contains("async_raft") {
                return false;
            }
            if mp.contains("hyper") {
                return false;
            }
        }

        // if v.module_path().unwrap().contains("less::network::p2p") {
        //     return false;
        // }

        // v.level() == &tracing::Level::ERROR
        //     || v.level() == &tracing::Level::WARN
        //     || v.level() == &tracing::Level::INFO
        v.level() != &tracing::Level::TRACE
        // v.level() == &tracing::Level::INFO
        // true
    });
    let my_layer = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry()
        .with(my_layer.with_filter(my_filter))
        .init();
}

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    // don't go thouph proxy when performance
    std::env::remove_var("http_proxy");
    std::env::remove_var("https_proxy");

    start_tracing();

    // tracing::debug!(
    //     "bencher running at dir {}",
    //     std::env::current_dir().unwrap()
    // );
    // debug abs running dir

    tracing::debug!(
        "bencher running at dir {}",
        std::env::current_dir().unwrap().display()
    );

    let cli = Cli::parse();
    cli.check_app_fn().check_platform().check_mode();

    let config = config::load_config();

    minio::init_bucket(&config.minio).await;

    let seed = "helloworld";
    tracing::debug!("Preparing paltform >>>");
    let mut platform = if cli.with_ow > 0 {
        PlatformOpsBind::from(platform_ow::PlatfromOw::new(&cli, &config))
    } else if cli.with_wl > 0 {
        PlatformOpsBind::from(platform_wl::PlatfromWl::new(&cli, config.clone()))
    } else {
        panic!("no platform specified, please specify by --with-ow or --with-wl");
    };

    tracing::debug!("dispatching mode >>>");
    fn print_mode(mode: &str, preparing: bool) {
        tracing::debug!("===========================");
        tracing::debug!("Current mode is {mode}");
        tracing::debug!("Preparing: {preparing}");
        tracing::debug!("===========================");
    }
    if is_prepare_mode(&cli) {
        common_prepare::prepare_data(cli.target_apps(), &config).await;
        platform.prepare_apps_bin(cli.target_apps(), &config).await;
    }

    if is_bench_mode(&cli) {
        print_mode("bench", is_bench_mode(&cli));
        unimplemented!();
        // target.prepare_bench(seed.to_owned(), cli.clone()).await;
        // target.call_bench(cli).await;
    } else if is_first_call_mode(&cli) {
        print_mode("first_call", is_bench_mode(&cli));
        if is_prepare_mode(&cli) {
            mode_first_call::prepare(&mut platform, &config, cli.clone()).await;
        } else {
            mode_first_call::call(&mut platform, cli, &config).await;
        }
    } else if is_once_mode(&cli) {
        print_mode("first_call", is_bench_mode(&cli));
        if is_prepare_mode(&cli) {
            mode_call_once::prepare(&mut platform, seed.to_owned(), cli.clone()).await;
        } else {
            mode_call_once::call(&mut platform, cli, &config).await;
        }
    } else {
        panic!("unreachable")
    }

    Ok(())
}
