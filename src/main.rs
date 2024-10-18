use std::{collections::HashSet, process::exit};

use searcher::search;
use tokio::time::Instant;

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
