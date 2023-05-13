// // Copyright 2023 Natalie Baker // AGPLv3 //

use std::path::PathBuf;

pub trait PathBufLongExt {
    fn long_ext(&self) -> Option<&str>;
}

impl PathBufLongExt for PathBuf {
    fn long_ext(&self) -> Option<&str> {
        self.file_name().and_then(|v| v.to_str()).and_then(|v| v.find('.').map(|p| v.split_at(p+1).1))
    }
}