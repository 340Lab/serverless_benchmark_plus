use std::collections::HashMap;

use serde_yaml::Value;

pub type Resp = HashMap<String, Value>;

pub trait RespMetricExt {}
