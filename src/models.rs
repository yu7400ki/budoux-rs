use std::collections::HashMap;
use std::sync::LazyLock;

pub type Model = HashMap<String, HashMap<String, i64>>;

#[cfg(feature = "ja")]
include!(concat!(env!("OUT_DIR"), "/models/ja.rs"));

#[cfg(feature = "zh-hans")]
include!(concat!(env!("OUT_DIR"), "/models/zh_hans.rs"));

#[cfg(feature = "zh-hant")]
include!(concat!(env!("OUT_DIR"), "/models/zh_hant.rs"));

#[cfg(feature = "th")]
include!(concat!(env!("OUT_DIR"), "/models/th.rs"));
