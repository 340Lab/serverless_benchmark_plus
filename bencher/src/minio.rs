use std::{path::PathBuf, process::Stdio, time::Duration};

use s3::{creds::Credentials, Bucket, BucketConfiguration, Region};
use tokio::{process::Command, sync::OnceCell};

use crate::{config::MinioConfig, util::CommandDebugStdio};

pub async fn init_bucket(config: &MinioConfig) -> Bucket {
    let bucket_name = "serverless-bench";
    let region = Region::Custom {
        region: "eu-central-1".to_owned(),
        endpoint: config.endpoint.to_owned(),
    };
    let credentials = Credentials {
        access_key: Some(config.access_key.to_owned()),
        secret_key: Some(config.secret_key.to_owned()),
        security_token: None,
        session_token: None,
        expiration: None,
    };

    let mut bucket = Bucket::new(bucket_name, region.clone(), credentials.clone())
        .unwrap()
        .with_path_style();

    let bucket_exist = match bucket.exists().await {
        Err(e) => {
            tracing::warn!("test s3 is not started, automatically start it");
            // docker-compose up -d at ../middlewares/minio/
            // check if docker-compose is installed
            if !Command::new("docker-compose")
                .arg("--version")
                .output()
                .await
                .is_ok()
            {
                panic!("docker-compose is not installed");
            }
            let (_, _, mut res) = Command::new("docker-compose")
                .arg("up")
                .arg("-d")
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .current_dir(PathBuf::from(config.compose_path.clone()))
                .spawn_debug()
                .await;
            let res = res.wait().await.unwrap();

            assert!(
                res.success(),
                "failed to start minio at {}",
                config.compose_path
            );
            tokio::time::sleep(Duration::from_secs(15)).await;
            bucket.exists().await.unwrap()
        }
        Ok(ok) => ok,
    };

    if bucket_exist {
        for b in bucket.list("".to_owned(), None).await.unwrap() {
            bucket.delete_object(b.name).await.unwrap();
            // bucket.delete().await.unwrap();
        }
    } else {
        bucket = Bucket::create_with_path_style(
            bucket_name,
            region,
            credentials,
            BucketConfiguration::default(),
        )
        .await
        .unwrap()
        .bucket;
    }

    BUCKET.set(bucket).unwrap();

    BUCKET.get().unwrap().clone()
}

lazy_static::lazy_static! {
    pub static ref BUCKET: OnceCell<Bucket> = OnceCell::new();
}
