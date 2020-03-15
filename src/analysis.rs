use {
    async_std::{
        fs,
        prelude::*,
        sync::{channel, Receiver, Sender},
        task,
    },
    futures::future::join_all,
    std::{char, collections::VecDeque, process::Command},
};

use crate::config;
use crate::detectors::*;
use crate::error::*;

pub async fn scheduler(conf: config::Config) -> RmStuffResult<()> {
    let deleter_conf = conf.clone();

    let (s_del, r_del) = channel::<Deletable>(1024);

    task::spawn(finder(s_del.clone(), conf.dir.clone()));
    task::spawn(deleter(r_del, deleter_conf)).await?;

    Ok(())
}

async fn finder(s_del: Sender<Deletable>, path: String) -> RmStuffResult<()> {
    let markers: Vec<String> = vec!["package.json", "node_modules"]
        .iter()
        .map(|m| m.to_string())
        .collect();

    let candidates: Vec<String> = vec!["node_modules", "dist", "public", ".cache"]
        .iter()
        .map(|m| m.to_string())
        .collect();
    let mut queue: VecDeque<String> = VecDeque::new();

    queue.push_back(path.clone());

    while let Some(dir) = queue.pop_front() {
        let entries: Vec<Entry> = {
            let mut dir: fs::ReadDir = {
                match fs::read_dir(dir).await {
                    Ok(d) => d,
                    _ => return RmStuffResult::Ok(()),
                }
            };

            let mut res = vec![];
            while let Some(Ok(e)) = dir.next().await {
                let path: String = e
                    .path()
                    .to_str()
                    .ok_or_else(|| RmStuffError::new("Could not get file/dir path"))?
                    .to_string();
                let name: String = e
                    .file_name()
                    .to_str()
                    .ok_or_else(|| RmStuffError::new("Could not get file/dir name"))?
                    .to_string();
                let is_dir: bool = e.metadata().await?.is_dir();

                res.push(Entry { path, name, is_dir });
            }

            res
        };

        let is_positive: bool = {
            entries
                .iter()
                .map(|e| &e.path)
                .any(|p| markers.iter().any(|m| p.contains(m)))
        };

        if is_positive {
            let deletables: Vec<Deletable> = {
                let paths: Vec<String> = entries
                    .clone()
                    .into_iter()
                    .filter(|e| candidates.iter().any(|c| e.path.ends_with(c)))
                    .map(|e| e.path)
                    .collect();

                let mut paths_iter = paths.iter();
                let mut ds = vec![];
                while let Some(p) = paths_iter.next() {
                    ds.push(Deletable::new(p.to_string()).await?);
                }
                // .map(|e| Deletable::new(e.path))

                ds
            };

            let mut iter = deletables.into_iter();
            while let Some(d) = iter.next() {
                s_del.send(d).await;
            }
        } else {
            let mut subdirs = entries
                .iter()
                // TODO figure out why it doesn't go into src in tray-academy
                .filter(|e| !candidates.contains(&e.name))
                .filter(|e| e.is_dir);

            while let Some(subd) = subdirs.next() {
                queue.push_back(subd.path.clone());
            }
        }
    }

    RmStuffResult::Ok(())
}

async fn deleter(r_del: Receiver<Deletable>, conf: config::Config) -> RmStuffResult<()> {
    let mut deletions = vec![];
    let mut deleted_bytes: u64 = 0;

    while let Some(d) = r_del.recv().await {
        let output = Command::new("du")
            .arg("-hs")
            .arg(d.path.to_string())
            .output()
            .expect("failed to get size");

        let stdout: String = String::from_utf8(output.stdout)?;
        let size_str: String = stdout.split_whitespace().take(1).collect();

        if conf.verbose {
            println!("{}\t {}", d.path.clone(), size_str);
        }

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

        let handle = task::spawn(async move {
            if d.is_dir {
                fs::remove_dir_all(d.path.to_string()).await?;
            } else {
                fs::remove_file(d.path.to_string()).await?;
            };

            RmStuffResult::Ok(())
        });

        deletions.push(handle);

        deleted_bytes += size as u64;
    }

    join_all(deletions).await;

    let mut unitless_size = deleted_bytes;
    let mut divided_times = 0;
    while unitless_size > 1024 {
        unitless_size /= 1024;
        divided_times += 1;
    }

    let units = vec!["b", "K", "M", "G"];

    println!("deleted {}{}", unitless_size, units[divided_times]);

    RmStuffResult::Ok(())
}
