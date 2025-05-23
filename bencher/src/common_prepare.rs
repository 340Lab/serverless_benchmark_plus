use std::{collections::HashMap, path::PathBuf, process::Stdio};

use tokio::{fs, process::Command};

use crate::{config::Config, util::CommandDebugStdio};

pub async fn link_source_app_data(source: &str, app: &str, func: &str) {
    let source_app_data = PathBuf::from("prepare_data").join(source);
    // let app_data = PathBuf::from("prepare_data").join(app);

    // to absolute path
    // let app_data = app_data.canonicalize().unwrap();
    let app_data_dir = PathBuf::from("prepare_data");

    let source_app_data = source_app_data.canonicalize().unwrap();

    // let app_data_dir = app_data.parent().unwrap();
    // let app_data_base = app_data.file_name().unwrap();

    tracing::info!("link {:?} to {:?}", source_app_data, app_data_dir.join(app));
    // fs::create_dir_all(&app_data).await.unwrap();
    // fs::symlink(source_app_data, app_data).await.unwrap();
    let (_, _, mut child) = Command::new("ln")
        .args(&[
            "-s",
            source_app_data.to_str().unwrap(),
            // app_data.to_str().unwrap(),
            app,
        ])
        .current_dir(&app_data_dir)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn_debug()
        .await;

    child.wait().await.unwrap();

    let app_data_path = app_data_dir.join(app);
    // check if the link is successful
    if !app_data_path.exists() {
        panic!(
            "link {} to {} failed",
            source_app_data.to_str().unwrap(),
            app_data_path.to_str().unwrap()
        );
    }
}

/// return each app datas
/// app->[data1,data2]
pub async fn prepare_data(
    target_apps: Vec<String>,
    config: &Config,
) -> HashMap<String, Vec<PathBuf>> {
    let mut prepare_data = HashMap::new();
    let model_apps: Vec<String> = target_apps
        .clone()
        .into_iter()
        .filter(|app| config.models.contains_key(app))
        .collect();

    for app in model_apps {
        fs::create_dir_all(PathBuf::from("prepare_data").join(&app))
            .await
            .unwrap();
        let app_entry = config.models.get(&app).unwrap();
        for (i, script) in app_entry.prepare_scripts.iter().enumerate() {
            let script_path = PathBuf::from("prepare_data")
                .join(&app)
                .join(format!("prepare_{}.py", i));
            let script_dir = PathBuf::from("prepare_data").join(&app);
            let abs_script_dir = script_dir.canonicalize().unwrap();
            // let abs_script_path = script_path.canonicalize().unwrap();
            fs::write(&script_path, script).await.unwrap();

            tracing::debug!(
                "prepare data for {} with script {}",
                app,
                script_path.to_str().unwrap()
            );
            let res = Command::new("python3")
                .args(&[script_path.file_name().unwrap().to_str().unwrap(), &*app])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .current_dir(abs_script_dir)
                .spawn()
                .unwrap()
                .wait()
                .await
                .unwrap();
            if !res.success() {
                panic!(
                    "prepare data for {} with script {} failed",
                    app,
                    script_path.to_str().unwrap()
                );
            }
        }

        for data in app_entry.prepare_data.iter() {
            let data_path = PathBuf::from("prepare_data").join(&app).join(data);
            if !data_path.exists() {
                panic!("prepare data failed {:?}", data);
            }
            prepare_data
                .entry(app.to_owned())
                .or_insert(vec![])
                .push(data_path);
        }
        // for data in app_entry.prepare_data.iter() {
        //     prepare_data.push(data.clone());
        // }
    }

    prepare_data
}
