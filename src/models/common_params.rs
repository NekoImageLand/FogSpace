use czkawka_core::common_tool::DeleteMethod;
use czkawka_core::tools::similar_images::SimilarityPreset;
use salvo::macros::Extractible;
use serde::Deserialize;
use serde_aux::field_attributes::default_u32;

#[derive(Debug, Deserialize, Extractible)]
#[salvo(extract(default_source(from = "body")))]
pub struct CommonParams {
    #[serde(default = "default_u32::<0>")]
    pub thread_number: u32,
    pub directories: Vec<String>,
    pub excluded_directories: Option<Vec<String>>,
    pub excluded_items: Option<Vec<String>>,
    pub allowed_extensions: Option<Vec<String>>,
    pub is_recursive: bool,
    pub use_cache: bool,
}

#[derive(Eq, PartialEq, Clone, Debug, Copy, Deserialize)]
#[serde(remote = "SimilarityPreset")]
pub enum SimilarityPresetDef {
    Original,
    VeryHigh,
    High,
    Medium,
    Small,
    VerySmall,
    Minimal,
    None,
}

#[derive(Eq, PartialEq, Clone, Debug, Copy, Deserialize)]
#[serde(remote = "DeleteMethod")]
pub enum DeleteMethodRef {
    None,
    Delete, // Just delete items
    AllExceptNewest,
    AllExceptOldest,
    OneOldest,
    OneNewest,
    HardLink,
    AllExceptBiggest,
    AllExceptSmallest,
    OneBiggest,
    OneSmallest,
}
