use std::{collections::HashSet, process::exit};

use searcher::{get_file_contents, search};
use tokio::time::Instant;

#[tokio::main]
async fn main() {
    let roots: HashSet<String> = HashSet::from([r"C:\Users".into()]);
    let search_terms: HashSet<String> = HashSet::from(["password".into()]);
    let exclusions: HashSet<String> =
        HashSet::from(["Spotify".into(), ".cargo".into(), ".lnk".into()]);
    let now = Instant::now();
    let results = search(&roots, &search_terms, &exclusions).await;

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
