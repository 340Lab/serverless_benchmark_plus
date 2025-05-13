use std::sync::Arc;

use crate::{
    config::{Config, FnDetails},
    parse::Cli,
    platform_ow, platform_wl,
};
use enum_dispatch::enum_dispatch;

#[enum_dispatch(PlatformOps)]
pub enum PlatformOpsBind {
    PlatfromOw(platform_ow::PlatfromOw),
    PlatfromWl(platform_wl::PlatfromWl),
}

impl PlatformOpsBind {
    // return if need call fn, or just triggered by data written
    pub async fn bf_call_fn(
        &self,
        app: &str,
        func: &str,
        big_data: &Option<Vec<String>>,
        fn_details: &FnDetails,
    ) -> bool {
        // write big data
        let use_minio = fn_details.write_big_data(app, func, self).await;

        // self.call_fn(
        //     &fn_details.app,
        //     &fn_details.func,
        //     &serde_json::to_value(fn_details.args).unwrap(),
        // )

        // use minio need explicit call fn
        use_minio
    }
}

#[enum_dispatch]
pub trait PlatformOps: Send + 'static {
    fn cli(&self) -> &Cli;
    async fn remove_all_fn(&self);
    async fn upload_fn(&mut self, demo: &str, rename_sub: &str);
    async fn call_fn(
        &self,
        app: &str,
        func: &str,
        arg_json_value: &serde_json::Value,
        // fn_details: &FnDetails,
        // big_data: &Option<Vec<String>>,
    ) -> String;
    async fn prepare_apps_bin(&self, apps: Vec<String>, config: &Config);
    /// may panic if not support
    async fn write_data(&self, key: &str, data: &[u8]);
}
