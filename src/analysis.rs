use {
    async_std::{
        fs,
        prelude::*,
        sync::{channel, Receiver, Sender},
        task,
    },
    futures::future::join_all,
    std::boxed::Box,
    std::collections::VecDeque,
};

use crate::config;
use crate::detectors;
use crate::error::*;
use crate::size::FileSize;

use crate::detectors::javascript::Javascript;

pub async fn scheduler(conf: config::Config) -> RmStuffResult<()> {
    let deleter_conf = conf.clone();

    let (s_del, r_del) = channel::<detectors::Deletable>(128);

    task::spawn(finder(s_del, conf.dir));
    task::spawn(deleter(r_del, deleter_conf)).await?;

    Ok(())
}

async fn finder(s_del: Sender<detectors::Deletable>, path: String) -> RmStuffResult<()> {
    let detects: Vec<Box<dyn detectors::Detector>> = vec![Box::new(Javascript::new())];
    let mut queue: VecDeque<String> = VecDeque::new();

    queue.push_back(path.clone());

    while let Some(dir) = queue.pop_front() {
        let entries: Vec<detectors::Entry> = {
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

                res.push(detectors::Entry::new(path).await?);
            }

            res
        };

        let find_futures = detects.iter().map(|d| d.find_deletables(&entries));
        let find_results: Vec<RmStuffResult<Option<Vec<detectors::Deletable>>>> =
            join_all(find_futures).await;

        let deletables: Vec<detectors::Deletable> = find_results
            .into_iter()
            .flat_map(|r| match r {
                Ok(Some(deletables)) => deletables,
                _ => vec![],
            })
            .collect();

        if deletables.len() > 0 {
            let mut iter = deletables.into_iter();
            while let Some(d) = iter.next() {
                s_del.send(d).await;
            }
        } else {
            let mut subdirs = entries
                .iter()
                // TODO figure out why it doesn't go into src in tray-academy,
                // also make sure we don't search in dirs that will be deleted
                // This could be responsible for that perf regresion
                // .filter(|e| !candidates.contains(&e.name))
                .filter(|e| e.is_dir);

            while let Some(subd) = subdirs.next() {
                queue.push_back(subd.path.clone());
            }
        }
    }

    RmStuffResult::Ok(())
}

async fn deleter(r_del: Receiver<detectors::Deletable>, conf: config::Config) -> RmStuffResult<()> {
    let mut deletions = vec![];
    let mut deleted_bytes: u64 = 0;
    let is_dry_run = conf.dry_run;

    while let Some(d) = r_del.recv().await {
        let size = detectors::get_size(d.path.clone())?;

        if conf.verbose {
            println!("{}\t {}", d.path.clone(), size.bytes());
        }

        let handle = task::spawn(async move {
            if is_dry_run {
                return RmStuffResult::Ok(());
            } else if d.is_dir {
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

    if is_dry_run {
        println!("would delete {}", total_size);
    } else {
        println!("deleted {}", total_size);
    };

    RmStuffResult::Ok(())
}
