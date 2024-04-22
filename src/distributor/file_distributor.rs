use std::fs::write;
use std::path::PathBuf;

use crate::distributor::interface::Distributor;

pub struct FileDistributor;

impl FileDistributor {
    pub fn new() -> FileDistributor {
        FileDistributor
    }
}

impl Distributor for FileDistributor {
    fn write(&self, dist_path: impl Into<PathBuf>, body: impl Into<String>) -> anyhow::Result<()> {
        Ok(write(dist_path.into(), body.into())?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{read_to_string, remove_file};

    #[test]
    fn write_should_be_exist_file_when_success() {
        let path = "./fixtures/dummy_test";
        let distributor = FileDistributor::new();
        let result = distributor.write(&path, "Foo");

        let read_result = read_to_string(&path).unwrap();

        assert_eq!(result.is_ok(), true);
        assert_eq!(read_result, "Foo");

        remove_file(path).expect("failed clean");
    }
}
