use async_std::fs;
use std::{borrow::Cow, process::Command};
use trait_async::trait_async;

use crate::error::*;
use crate::size::FileSize;

pub mod javascript;

pub fn get_size<'a, S: Into<Cow<'a, str>>>(path: S) -> RmStuffResult<FileSize> {
    let path_cow = path.into();
    let output = Command::new("du")
        .arg("-ks")
        .arg(path_cow.to_string())
        .output()
        .expect("failed to get size");

    let stdout: String = String::from_utf8(output.stdout)?;
    let size_str: String = stdout.split_whitespace().take(1).collect();
    let size_number: u64 = size_str
        .matches(char::is_numeric)
        .collect::<String>()
        .parse()?;

    Ok(FileSize::new(size_number * 1024))
}

#[derive(Debug)]
pub struct Deletable {
    pub path: String,
    pub is_dir: bool,
    pub size: FileSize,
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

impl Entry {
    pub async fn new<'a, S: Into<Cow<'a, str>>>(path: S) -> RmStuffResult<Entry> {
        let path_cow = path.into();
        let metadata = fs::metadata(path_cow.to_string()).await?;

        let name = {
            match path_cow.to_string().split("/").last() {
                Some(part) => part.to_string(),
                None => panic!("Cannot determine the file name"),
            }
        };

        Ok(Entry {
            path: path_cow.to_string(),
            name,
            is_dir: metadata.is_dir(),
        })
    }
}

#[trait_async]
pub trait Detector: Send + Sync {
    fn markers(&self) -> Vec<String>;
    fn deletables(&self) -> Vec<String>;

    async fn find_deletables(&self, entries: &Vec<Entry>) -> RmStuffResult<Option<Vec<Deletable>>> {
        let is_positive: bool = entries
            .iter()
            .map(|e| &e.path)
            .any(|p| self.markers().iter().any(|m| p.contains(m)));

        match is_positive {
            false => Ok(None),
            true => {
                let paths: Vec<String> = entries
                    .clone()
                    .into_iter()
                    .filter(|e| self.deletables().iter().any(|c| &e.name == c))
                    .map(|e| e.path)
                    .collect();

                let mut paths_iter = paths.iter();
                let mut ds = vec![];
                while let Some(p) = paths_iter.next() {
                    ds.push(Deletable::new(p.to_string()).await?);
                }

                Ok(Some(ds))
            }
        }
    }
}
