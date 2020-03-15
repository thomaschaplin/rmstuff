use async_std::fs;
use async_trait::async_trait;
use std::{borrow::Cow, process::Command};

use crate::error::*;

pub fn get_size<'a, S: Into<Cow<'a, str>>>(path: S) -> RmStuffResult<u64> {
    let path_cow = path.into();
    let output = Command::new("du")
        .arg("-hs")
        .arg(path_cow.to_string())
        .output()
        .expect("failed to get size");

    let stdout: String = String::from_utf8(output.stdout)?;
    let size_str: String = stdout.split_whitespace().take(1).collect();
    let size_number: u64 = size_str
        .matches(char::is_numeric)
        .collect::<String>()
        .parse()?;
    let size_unit: String = size_str.matches(char::is_alphabetic).collect();
    let pow = match &size_unit[..] {
        "K" => 1,
        "M" => 2,
        unit => panic!(format!("Unknown unit {}", unit)),
    };

    Ok(size_number * 1024u64.checked_pow(pow).expect("Size cannot fit in u64"))
}

pub struct Deletable {
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
}

impl Deletable {
    pub async fn new<'a, S: Into<Cow<'a, str>>>(path: S) -> RmStuffResult<Deletable> {
        let path_cow = path.into();
        let metadata = fs::metadata(path_cow.to_string()).await?;

        Ok(Deletable {
            path: path_cow.to_string(),
            is_dir: metadata.is_dir(),
            size: get_size(path_cow)?,
        })
    }
}

#[derive(Clone)]
pub struct Entry {
    pub path: String,
    pub name: String,
    pub is_dir: bool,
}

#[async_trait]
trait Detector {
    async fn deletables(&self, e: Entry) -> Option<Vec<Deletable>>;
}
