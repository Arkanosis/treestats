use indicatif::{
    ProgressBar,
    ProgressStyle,
};
use std::collections::HashMap;
use walkdir::WalkDir;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

struct ExtensionStatistics {
    count: u64,
    size: u64,
}

pub fn analyze(directory: &str) -> () {
    let progress = ProgressBar::new_spinner();
    let mut total_count = 0u64;
    let mut total_size = 0u64;
    progress.set_style(ProgressStyle::with_template("{spinner} {msg} files, {bytes} analyzed [{elapsed_precise}]").unwrap());
    let mut statistics: HashMap<String, ExtensionStatistics> = HashMap::new();
    for entry in WalkDir::new(directory)
        .follow_links(false)
        .into_iter()
        .filter_map(|entry| entry.ok()) {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    let extension = if let Some(extension) = entry.path().extension() {
                        extension.to_str().unwrap_or("other")
                    } else {
                        "other"
                    }.to_lowercase();
                    statistics.entry(extension)
                        .and_modify(|statistics| {
                            statistics.count += 1;
                            statistics.size += metadata.len();
                        }).or_insert(ExtensionStatistics {
                            count: 1,
                            size: metadata.len(),
                        });
                    total_count += 1;
                    total_size += metadata.len();
                    progress.set_message(total_count.to_string());
                    progress.set_position(total_size);
                }
            }
        }
    progress.finish();
    for (extension, statistics) in &statistics {
        println!("{}\t{}\t{}", &extension, statistics.count, statistics.size);
    }
}
