use crate::parse::Cli;

pub enum App {
    ImgResize,
    WordCount,
    Parallel,
    Sequential,
    JavaKvTest,
}

impl From<&Cli> for App {
    fn from(cli: &Cli) -> Self {
        if cli.img_resize > 0 {
            App::ImgResize
        } else if cli.word_count > 0 {
            App::WordCount
        } else if cli.parallel > 0 {
            App::Parallel
        } else if cli.sequential > 0 {
            App::Sequential
        } else {
            unimplemented!()
        }
    }
}

impl ToString for App {
    fn to_string(&self) -> String {
        match self {
            App::ImgResize => "img_resize".to_owned(),
            App::WordCount => "word_count".to_owned(),
            App::Parallel => "parallel".to_owned(),
            App::Sequential => "sequential".to_owned(),
            App::JavaKvTest => "java_kv_test".to_owned(),
        }
    }
}
