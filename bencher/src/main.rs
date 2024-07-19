mod demo_img_resize;
mod demo_word_count;
mod demo_parallel;
mod demo_sequential;
mod platform_ow;
mod platform_wl;

use async_trait::async_trait;
use clap::arg;
use clap::value_parser;
use clap::{command, Command};
use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;
use goose::prelude::*;
use s3::creds::Credentials;
use s3::Bucket;
use s3::BucketConfiguration;
use s3::Region;
use std::path::PathBuf;
use std::process;
use std::sync::mpsc;
use std::time::Duration;
use tokio::sync::oneshot;
use tokio::sync::Mutex;

#[derive(Parser, Clone)]
#[command(version, about, long_about = None)]
struct Cli {
    // #[arg(action = clap::ArgAction::Count)]
    #[arg(long, action = clap::ArgAction::Count)]
    img_resize: u8,

    #[arg(long, action = clap::ArgAction::Count)]
    word_count: u8,

    #[arg(long, action = clap::ArgAction::Count)]
    parallel: u8,

    #[arg(long, action = clap::ArgAction::Count)]
    sequential: u8,

    #[arg(long, action = clap::ArgAction::Count)]
    with_ow: u8,

    #[arg(long, action = clap::ArgAction::Count)]
    with_wl: u8,

    #[arg(long, action = clap::ArgAction::Count)]
    bench_mode: u8,

    // create many function copy and collect the average cold start
    #[arg(long, action = clap::ArgAction::Count)]
    first_call_mode: u8,
}

lazy_static::lazy_static! {
    pub static ref BUCKET:Bucket={
        let(tx,rx)=mpsc::channel();
        tokio::spawn(async move{
            let bucket_name="serverless-bench";
            let region=Region::Custom {
                region: "eu-central-1".to_owned(),
                endpoint: "http://192.168.31.96:9009".to_owned(),
            };
            let credentials= Credentials {
                access_key: Some("minioadmin".to_owned()),
                secret_key: Some("minioadmin123".to_owned()),
                security_token: None,
                session_token: None,
                expiration: None,
            };

            let mut bucket=Bucket::new(bucket_name,region.clone(), credentials.clone()).unwrap().with_path_style();

            if bucket.exists().await.unwrap() {
                for b in bucket.list("".to_owned(),None).await.unwrap(){
                    bucket.delete_object(b.name).await.unwrap();
                    // bucket.delete().await.unwrap();
                }
            }else{
                bucket = Bucket::create_with_path_style(
                    bucket_name,
                    region,
                    credentials,
                    BucketConfiguration::default(),
                )
                .await.unwrap()
                .bucket;
            }

            tx.send(bucket);
        });
        rx.recv().unwrap()
    };
}

fn is_bench_mode(cli: &Cli) -> bool {
    cli.bench_mode > 0
}

fn is_first_call_mode(cli: &Cli) -> bool {
    cli.first_call_mode > 0
}

fn is_once_mode(cli: &Cli) -> bool {
    !is_bench_mode(cli) && !is_first_call_mode(cli)
}

#[enum_dispatch(SpecTarget)]
enum SpecTargetBind {
    ImgResize(demo_img_resize::ImgResize),
    WordCount(demo_word_count::WordCount),
    Parallel(demo_parallel::Parallel),
    Sequential(demo_sequential::Sequential)
}

/// unit: ms
struct Metric {
    start_call_time: u64,
    req_arrive_time: u64,
    bf_exec_time: u64,
    recover_begin_time: u64,
    fn_start_time: u64,
    fn_end_time: u64,
    receive_resp_time: u64,
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

#[enum_dispatch]
trait SpecTarget: Send + 'static {
    fn set_platform(&mut self, platform: PlatformOpsBind);
    fn get_platform(&mut self) -> &mut PlatformOpsBind;

    async fn prepare_once(&mut self, _seed: String, _cli: Cli) {
        unimplemented!()
    }
    async fn call_once(&mut self, _cli: Cli) -> Metric {
        unimplemented!()
    }
    async fn prepare_bench(&mut self, _seed: String, _cli: Cli) {}
    async fn call_bench(&mut self, _cli: Cli) {
        unimplemented!()
    }
    async fn prepare_first_call(&mut self, _seed: String, _cli: Cli) {
        unimplemented!()
    }
    async fn call_first_call(&mut self, _cli: Cli) {
        unimplemented!()
    }
}

#[enum_dispatch(PlatformOps)]
enum PlatformOpsBind {
    PlatfromOw(platform_ow::PlatfromOw),
    PlatfromWl(platform_wl::PlatfromWl),
}

#[enum_dispatch]
pub trait PlatformOps: Send + 'static {
    async fn remove_all_fn(&self);
    async fn upload_fn(&mut self, demo: &str, rename_sub: &str);
    async fn call_fn(&self, app: &str, func: &str, arg_json_value: &serde_json::Value) -> String;
}
// pub struct CallRes {
//     out: String,
//     err: String,
// }

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    // don't go thouph proxy when performance
    std::env::remove_var("http_proxy");
    std::env::remove_var("https_proxy");

    let cli = Cli::parse();
    let seed = "helloworld";

    assert!(
        !(cli.with_ow > 0 && cli.with_wl > 0),
        "Cannot run with both OpenWhisk and Waverless"
    );

    assert!(
        cli.bench_mode + cli.first_call_mode <= 1,
        "Cannot test multiple modes at one time {}",
        cli.bench_mode + cli.first_call_mode
    );

    let mut target = if cli.img_resize > 0 {
        SpecTargetBind::from(demo_img_resize::ImgResize::default())
    } else if cli.word_count > 0 {
        SpecTargetBind::from(demo_word_count::WordCount::default())
    } else if cli.parallel > 0 {
        SpecTargetBind::from(demo_parallel::Parallel::default())
    } else if cli.sequential > 0 {
        SpecTargetBind::from(demo_sequential::Sequential::default())
    } else {
        unreachable!()
    };
    target.set_platform(if cli.with_ow > 0 {
        PlatformOpsBind::from(platform_ow::PlatfromOw::default())
    } else if cli.with_wl > 0 {
        PlatformOpsBind::from(platform_wl::PlatfromWl::new())
    } else{
        panic!();
    });

    if is_bench_mode(&cli) {
        target.prepare_bench(seed.to_owned(), cli.clone()).await;
        target.call_bench(cli).await;
    } else if is_first_call_mode(&cli) {
        target
            .prepare_first_call(seed.to_owned(), cli.clone())
            .await;
        target.call_first_call(cli).await;
    } else if is_once_mode(&cli) {
        target.prepare_once(seed.to_owned(), cli.clone()).await;
        // wait for the system to be stable
        tokio::time::sleep(Duration::from_secs(5)).await;
        target.call_once(cli).await;
    }

    Ok(())
}
