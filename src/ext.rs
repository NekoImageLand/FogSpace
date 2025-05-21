use std::path::PathBuf;

pub trait ToPathBufs {
    fn to_pathbufs(&self) -> Vec<PathBuf>;
}

impl ToPathBufs for Vec<String> {
    fn to_pathbufs(&self) -> Vec<PathBuf> {
        self.iter().map(PathBuf::from).collect()
    }
}
