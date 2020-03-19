use {
    async_std::{
        fs,
        prelude::*,
        sync::{channel, Receiver, Sender},
        task,
    },
    futures::future::join_all,
    std::collections::VecDeque,
};

use crate::config;
use crate::detectors::*;
use crate::error::*;
use crate::size::FileSize;

pub async fn scheduler(conf: config::Config) -> RmStuffResult<()> {
    let deleter_conf = conf.clone();

    let (s_del, r_del) = channel::<Deletable>(1024);

    task::spawn(finder(s_del, conf.dir.clone()));
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

                res.push(Entry::new(path).await?);
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
                    .filter(|e| candidates.iter().any(|c| &e.name == c))
                    .map(|e| e.path)
                    .collect();

                let mut paths_iter = paths.iter();
                let mut ds = vec![];
                while let Some(p) = paths_iter.next() {
                    ds.push(Deletable::new(p.to_string()).await?);
                }

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
        let size = get_size(d.path.clone())?;

        if conf.verbose {
            println!("{}\t {}", d.path.clone(), size.bytes());
        }

        let handle = task::spawn(async move {
            if d.is_dir {
                fs::remove_dir_all(d.path.to_string()).await?;
            } else {
                fs::remove_file(d.path.to_string()).await?;
            };

            RmStuffResult::Ok(())
        });

        deletions.push(handle);

        deleted_bytes += size.bytes();
    }

    join_all(deletions).await;

    let total_size = FileSize::new(deleted_bytes);

    println!("deleted {}", total_size);

    RmStuffResult::Ok(())
}
