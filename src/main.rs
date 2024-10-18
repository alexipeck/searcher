use futures::StreamExt;
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    process::exit,
    time::Instant,
};
use tokio::{fs::File, io::AsyncReadExt};
use walkdir::WalkDir;

pub async fn get_file_contents(path: &str) -> Option<String> {
    let mut file = match File::open(path).await {
        Ok(file) => file,
        Err(_err) => return None,
    };
    let mut contents = Default::default();
    if file.read_to_string(&mut contents).await.is_err() {
        return None;
    }
    Some(contents)
}

pub async fn search(
    roots: &HashSet<String>,
    search_terms: &HashSet<String>,
    exclusions: &HashSet<String>,
) -> HashMap<String, Option<String>> {
    let results: Vec<String> = futures::stream::iter(roots.iter())
        .then(|root| async move {
            WalkDir::new(root)
                .into_iter()
                .filter_map(|entry| entry.ok())
                .par_bridge()
                .filter(|entry| {
                    if let Some(file_name) = entry.path().file_name() {
                        let file_name = file_name.to_string_lossy();
                        search_terms
                            .par_iter()
                            .any(|search_term| file_name.contains(search_term))
                            && !exclusions.par_iter().any(|exclusion| {
                                entry
                                    .path()
                                    .as_os_str()
                                    .to_string_lossy()
                                    .contains(exclusion)
                            })
                    } else {
                        false
                    }
                })
                .map(|entry| entry.path().display().to_string())
                .collect::<HashSet<String>>()
        })
        .collect::<Vec<HashSet<String>>>()
        .await
        .into_iter()
        .flatten()
        .collect();
    futures::stream::iter(results.into_iter())
        .then(|result_path| async move {
            //why is the commented out solution slower than my get_file_contents()
            /* let contents = match tokio::fs::read_to_string(&result_path).await {
                Ok(contents) => Some(contents),
                Err(_err) => None,
            }; */
            let contents = get_file_contents(&result_path).await;
            (result_path, contents)
        })
        .collect::<HashMap<String, Option<String>>>()
        .await
}

#[tokio::main]
async fn main() {
    let roots: HashSet<String> = HashSet::from(["/Users".into()]);
    let search_terms: HashSet<String> = HashSet::from(["password".into()]);
    let exclusions: HashSet<String> =
        HashSet::from(["Spotify".into(), ".cargo".into(), ".lnk".into()]);
    let now = Instant::now();
    let mut results = Default::default();
    for i in 0..10 {
        let results_ = search(&roots, &search_terms, &exclusions).await;

        if i == 0 {
            println!("{:?}", results_);
            results = results_;
        }
        break; //Comment out to run 10 times
    }

    println!("{}ms", now.elapsed().as_millis());
    let serialized = serde_json::to_string(&results);
    match serialized {
        Ok(serialized) => println!("{serialized}"),
        Err(err) => {
            eprintln!("{err}");
            exit(1)
        }
    }
}
