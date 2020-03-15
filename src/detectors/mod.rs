use async_std::{fs, path::Path};
use async_trait::async_trait;
use std::{borrow::Cow, convert::AsRef, ffi::OsStr, process::Command};

use crate::error::*;

pub struct Deletable<'a> {
    pub path: Cow<'a, str>,
    pub is_dir: bool,
    pub size: u32,
}

impl<'a> Deletable<'a> {
    pub async fn new<'b, S: Into<Cow<'b, str>> + AsRef<Path> + AsRef<OsStr>>(
        path: S,
    ) -> RmStuffResult<'b, Deletable<'b>> {
        let metadata = fs::metadata(path).await?;
        let is_dir = metadata.is_dir();

        let output = Command::new("du")
            .arg("-hs")
            .arg(path)
            .output()
            .expect("failed to get size");

        let stdout: String = String::from_utf8(output.stdout)?;
        let size_str: String = stdout.split_whitespace().take(1).collect();
        let size_number: u32 = size_str
            .matches(char::is_numeric)
            .collect::<String>()
            .parse()?;
        let size_unit: String = size_str.matches(char::is_alphabetic).collect();
        let size_multiplier = match &size_unit[..] {
            "K" => 1024,
            "M" => 1024 * 1024,
            unit => panic!(format!("Unknown unit {}", unit)),
        };
        let size = size_number * size_multiplier;

        Ok(Deletable {
            path: path.into(),
            is_dir,
            size,
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
