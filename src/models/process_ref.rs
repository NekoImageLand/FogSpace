use czkawka_core::common_dir_traversal::{CheckingMethod, ToolType};
use czkawka_core::progress_data::{CurrentStage, ProgressData};
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(remote = "ProgressData")]
pub struct ProgressDataRef {
    #[serde(with = "CurrentStageRef")]
    pub sstage: CurrentStage,
    pub checking_method: CheckingMethod,
    pub current_stage_idx: u8,
    pub max_stage_idx: u8,
    pub entries_checked: usize,
    pub entries_to_check: usize,
    pub bytes_checked: u64,
    pub bytes_to_check: u64,
    #[serde(with = "ToolTypeRef")]
    pub tool_type: ToolType,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize)]
#[serde(remote = "CurrentStage")]
enum CurrentStageRef {
    CollectingFiles,
    DuplicateCacheSaving,
    DuplicateCacheLoading,
    DuplicatePreHashCacheSaving,
    DuplicatePreHashCacheLoading,
    DuplicateScanningName,
    DuplicateScanningSizeName,
    DuplicateScanningSize,
    DuplicatePreHashing,
    DuplicateFullHashing,
    SameMusicCacheSavingTags,
    SameMusicCacheLoadingTags,
    SameMusicCacheSavingFingerprints,
    SameMusicCacheLoadingFingerprints,
    SameMusicReadingTags,
    SameMusicCalculatingFingerprints,
    SameMusicComparingTags,
    SameMusicComparingFingerprints,
    SimilarImagesCalculatingHashes,
    SimilarImagesComparingHashes,
    SimilarVideosCalculatingHashes,
    BrokenFilesChecking,
    BadExtensionsChecking,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Serialize)]
#[serde(remote = "ToolType")]
enum ToolTypeRef {
    Duplicate,
    EmptyFolders,
    EmptyFiles,
    InvalidSymlinks,
    BrokenFiles,
    BadExtensions,
    BigFile,
    SameMusic,
    SimilarImages,
    SimilarVideos,
    TemporaryFiles,
    #[default]
    None,
}
