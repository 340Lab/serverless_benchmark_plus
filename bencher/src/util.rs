use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::{Child, Command},
};

pub trait CommandDebugStdio {
    async fn spawn_debug(
        &mut self,
    ) -> (
        tokio::task::JoinHandle<String>,
        tokio::task::JoinHandle<String>,
        Child,
    );
}

impl CommandDebugStdio for Command {
    async fn spawn_debug(
        &mut self,
    ) -> (
        tokio::task::JoinHandle<String>,
        tokio::task::JoinHandle<String>,
        Child,
    ) {
        let mut child = self.spawn().unwrap();
        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        // 分别处理 stdout 和 stderr
        let mut stdout_reader = BufReader::new(stdout).lines();
        let mut stderr_reader = BufReader::new(stderr).lines();

        let stdout_task = tokio::spawn(async move {
            let mut all = String::new();
            while let Ok(Some(line)) = stdout_reader.next_line().await {
                println!("[STDOUT] {}", line);
                all += &format!("[STDOUT] {}\n", line);
            }
            all
        });

        let stderr_task = tokio::spawn(async move {
            let mut all = String::new();
            while let Ok(Some(line)) = stderr_reader.next_line().await {
                eprintln!("[STDERR] {}", line);
                all += &format!("[STDERR] {}\n", line);
            }
            all
        });

        (stdout_task, stderr_task, child)
    }
}

#[macro_export]
macro_rules! new_map {
    // 匹配空映射
    ($map_type:ident { }) => {
        $map_type::new()
    };
    // 匹配一个或多个键值对
    ($map_type:ident { $($key:expr => $value:expr),+ $(,)? }) => {{
        let map = $map_type::from([
            $( ($key, $value), )+
        ]);
        map
    }};
}
