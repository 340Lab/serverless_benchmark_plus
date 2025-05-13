use std::{collections::HashMap, path::PathBuf, process::Stdio};

use tokio::{fs, process::Command};

use crate::config::Config;

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
