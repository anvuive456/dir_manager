use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Mutex,
    },
};

use colored::Colorize;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use rayon::prelude::*;
use walkdir::WalkDir;

fn exact_search(pattern: &str, folder: &str, extension: Option<&String>, context: usize) {
    let total_matches = AtomicUsize::new(0);
    let results = Mutex::new(Vec::new());

    WalkDir::new(folder)
        .into_iter()
        .filter_map(Result::ok)
        .par_bridge()
        .filter_map(|entry| {
            let path = entry.path();

            // Ignore files start with "."
            if let Some(file_name) = path.file_name() {
                if file_name.to_string_lossy().starts_with('.') {
                    return None;
                }
            }

            // Filter with extensions
            if let Some(ext) = extension {
                if path.extension().map(|e| e.to_string_lossy()) != Some(ext.into()) {
                    return None;
                }
            }

            if path.is_file() {
                Some(path.to_path_buf())
            } else {
                None
            }
        })
        .for_each(|path: PathBuf| {
            if let Ok(file) = File::open(&path) {
                let reader = BufReader::new(file);
                let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

                let mut output = String::new();
                let mut last_match = None;

                for (index, line) in lines.iter().enumerate() {
                    if line.contains(pattern) {
                        total_matches.fetch_add(1, Ordering::Relaxed);

                        // Print file if first match in file
                        if last_match.is_none() {
                            output.push_str(&format!(
                                "\n{}\nFile: {}\n",
                                "-".repeat(40).dimmed(),
                                path.display().to_string().blue()
                            ));
                        } else {
                            output.push('\n'); // Dòng cách
                        }

                        // Print before context
                        let start_context = index.saturating_sub(context);
                        let end_context = index.saturating_add(context).min(lines.len() - 1);

                        for ctx_index in start_context..index {
                            output.push_str(&format!(
                                "    {} | {}\n",
                                (ctx_index + 1).to_string().dimmed(),
                                lines[ctx_index].dimmed()
                            ));
                        }

                        // Print match
                        output.push_str(&format!(
                            "  {}:{}: {}\n",
                            (index + 1).to_string().yellow(),
                            "MATCH".red(),
                            line.replace(pattern, &pattern.green().bold().to_string())
                        ));

                        // Print after context
                        for ctx_index in (index + 1)..=end_context {
                            output.push_str(&format!(
                                "    {} | {}\n",
                                (ctx_index + 1).to_string().dimmed(),
                                lines[ctx_index].dimmed()
                            ));
                        }

                        // Update index
                        last_match = Some(index);
                    }
                }

                if !output.is_empty() {
                    let mut results_guard = results.lock().unwrap();
                    results_guard.push(output);
                }
            }
        });

    // Print result
    let results_guard = results.lock().unwrap();
    for result in results_guard.iter() {
        println!("{}", result);
    }

    // Print total match
    println!(
        "\n{} {}",
        total_matches
            .load(Ordering::Relaxed)
            .to_string()
            .green()
            .bold(),
        if total_matches.load(Ordering::Relaxed) == 1 {
            "match"
        } else {
            "matches"
        }
    );
}

fn fuzzy_search(
    pattern: &str,
    folder: &str,
    extension: Option<&String>,
    context: usize,
    fuzzy_threshold: i64,
) {
    let matcher = SkimMatcherV2::default();

    let total_matches = AtomicUsize::new(0);
    let results = Mutex::new(Vec::new());

    WalkDir::new(folder)
        .into_iter()
        .filter_map(Result::ok)
        .par_bridge()
        .filter_map(|entry| {
            let path = entry.path();

            // Ignore files start with "."
            if let Some(file_name) = path.file_name() {
                if file_name.to_string_lossy().starts_with('.') {
                    return None;
                }
            }

            // Filter by extensions
            if let Some(ext) = extension {
                if path.extension().map(|e| e.to_string_lossy()) != Some(ext.into()) {
                    return None;
                }
            }

            if path.is_file() {
                Some(path.to_path_buf())
            } else {
                None
            }
        })
        .for_each(|path: PathBuf| {
            if let Ok(file) = File::open(&path) {
                let reader = BufReader::new(file);
                let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

                let mut output = String::new();
                let mut last_match = None;

                for (index, line) in lines.iter().enumerate() {
                    if let Some(score) = matcher.fuzzy_match(line, pattern) {
                        if score < fuzzy_threshold {
                            continue;
                        }

                        total_matches.fetch_add(1, Ordering::Relaxed);

                        // Print file if first match in file
                        if last_match.is_none() {
                            output.push_str(&format!(
                                "\n{}\nFile: {}\n",
                                "-".repeat(40).dimmed(),
                                path.display().to_string().blue()
                            ));
                        } else {
                            output.push('\n'); // Dòng cách
                        }

                        // Print before context
                        let start_context = index.saturating_sub(context);
                        let end_context = index.saturating_add(context).min(lines.len() - 1);

                        for ctx_index in start_context..index {
                            output.push_str(&format!(
                                "    {} | {}\n",
                                (ctx_index + 1).to_string().dimmed(),
                                lines[ctx_index].dimmed()
                            ));
                        }
                        // Highlight the matched part manually
                        let mut highlighted_line = line.clone();
                        if let Some((_, indices)) = matcher.fuzzy_indices(line, pattern) {
                            for &idx in indices.iter().rev() {
                                highlighted_line.replace_range(
                                    idx..=idx,
                                    &line[idx..=idx].green().bold().to_string(),
                                );
                            }
                        }
                        // Print match
                        output.push_str(&format!(
                            "  {}:{}: {}\n",
                            (index + 1).to_string().yellow(),
                            format!("SCORE: {}", score).red(),
                            highlighted_line
                        ));

                        // Print after context
                        for ctx_index in (index + 1)..=end_context {
                            output.push_str(&format!(
                                "    {} | {}\n",
                                (ctx_index + 1).to_string().dimmed(),
                                lines[ctx_index].dimmed()
                            ));
                        }

                        // Update index
                        last_match = Some(index);
                    }
                }

                if !output.is_empty() {
                    let mut results_guard = results.lock().unwrap();
                    results_guard.push(output);
                }
            }
        });

    // Print result
    let results_guard = results.lock().unwrap();
    for result in results_guard.iter() {
        println!("{}", result);
    }

    // Print total match
    println!(
        "\n{} {}",
        total_matches
            .load(Ordering::Relaxed)
            .to_string()
            .green()
            .bold(),
        if total_matches.load(Ordering::Relaxed) == 1 {
            "match"
        } else {
            "matches"
        }
    );
}
pub fn search_pattern_recursive(
    pattern: &str,
    folder: &str,
    extension: Option<&String>,
    context: usize,
    use_fuzzy: bool,
    fuzzy_threshold: Option<i64>,
) -> io::Result<()> {
    println!("Search with Fuzzy: {}", use_fuzzy);

    if use_fuzzy {
        fuzzy_search(
            pattern,
            folder,
            extension,
            context,
            fuzzy_threshold.unwrap_or(30),
        );
    } else {
        exact_search(pattern, folder, extension, context);
    }

    Ok(())
}
