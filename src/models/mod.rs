#[cfg(feature = "ja")]
mod ja;

#[cfg(feature = "zh-hans")]
mod zh_hans;

#[cfg(feature = "zh-hant")]
mod zh_hant;

#[cfg(feature = "th")]
mod th;

#[cfg(feature = "ja")]
pub use ja::MODEL as JA_MODEL;

#[cfg(feature = "zh-hans")]
pub use zh_hans::MODEL as ZH_HANS_MODEL;

#[cfg(feature = "zh-hant")]
pub use zh_hant::MODEL as ZH_HANT_MODEL;

#[cfg(feature = "th")]
pub use th::MODEL as TH_MODEL;
