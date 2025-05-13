// fn write_big_data() {
//     // let mut file = File::create("big_data.txt").unwrap();
//     // file.write_all(data.as_bytes()).unwrap();
// }

use crate::{
    config::FnDetails,
    fs::app_dir,
    minio::BUCKET,
    platform::{PlatformOps, PlatformOpsBind},
};
use dashmap::DashMap;
use std::{
    collections::HashMap,
    path::{self, Path, PathBuf},
    sync::Arc,
};

// cache the read
lazy_static::lazy_static! {
    pub static ref CACHE: std::sync::OnceLock<DashMap<String, Arc<Vec<u8>>>> = std::sync::OnceLock::new();
}

async fn read_file(path: &impl AsRef<Path>) -> Arc<Vec<u8>> {
    //access cache first
    {
        let cache = CACHE.get_or_init(|| DashMap::new());
        if let Some(content) = cache.get(path.as_ref().to_str().unwrap()) {
            return content.clone();
        }
    }

    let content = tokio::fs::read(path).await.unwrap();
    {
        let mut cache = CACHE.get_or_init(|| DashMap::new());
        let ret = Arc::new(content);
        cache.insert(path.as_ref().to_str().unwrap().to_string(), ret.clone());
        ret
    }
}

impl FnDetails {
    pub async fn write_big_data(
        &self,
        app_name: &str,
        fn_name: &str,
        platform: &PlatformOpsBind,
    ) -> bool {
        // read file content and write
        // we don't need the detailed path, just bool
        let use_minio: bool = if let Some(args) = &self.args {
            if let Some(use_minio) = args.get("use_minio") {
                // Some(use_minio)
                if let Some(use_minio) = use_minio.as_str() {
                    !use_minio.is_empty()
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };

        let Some(big_data) = self.big_data.clone() else {
            tracing::error!("no big data to write");
            return use_minio;
        };

        tracing::info!("writing big data: {:?}", big_data);

        // for each big data item
        for big_data in big_data.iter() {
            let big_data_item = big_data.split(":").collect::<Vec<&str>>();
            let big_data_read_path = big_data_item[0];
            let big_data_write_path = big_data_item[1];

            let to_read_path = app_dir(app_name).join(big_data_read_path);
            // panic if file not exists

            if !to_read_path.exists() {
                panic!("{} not exists", to_read_path.display());
            }

            // read all content
            let content = read_file(&to_read_path).await;

            if use_minio {
                // write to minio
                let bucket = BUCKET.get().unwrap();
                bucket
                    .put_object(
                        big_data_write_path,
                        // format!("{}/{}/{}", app_name, fn_name, big_data_write_path),
                        &content,
                    )
                    .await
                    .unwrap();
            } else {
                // try platform write_data
                platform.write_data(big_data_write_path, &content).await;
            }
        }
        use_minio
    }
}
