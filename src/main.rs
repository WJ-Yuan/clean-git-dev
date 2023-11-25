use inquire::{MultiSelect, Select, Confirm};
use regex::Regex;
use std::{
    io::{self, Write},
    process::{self, Command},
};

fn get_delete_mode() -> String {
    let mode: &str = loop {
        let delete_mode_options: Vec<&str> = vec!["Select", "Keyword", "Regex"];
        let delete_mode =
            Select::new("Firstly, please select delete mode: ", delete_mode_options).prompt();
        match delete_mode {
            Ok(result) => break result,
            Err(_) => continue,
        };
    };

    mode.to_string()
}

fn main() {
    let delete_mode: String = get_delete_mode();
    println!("Your choice: {} mode", &delete_mode);
    let mode = &delete_mode[..];

    match mode {
        "Select" => select_delete(),
        "Keyword" => keyword_or_regex_delete(false),
        _ => keyword_or_regex_delete(true),
    }
}

fn select_delete() {
    println!("Fetching local branches...");
    let branches: Vec<String> = get_local_branches();

    if branches.is_empty() {
        println!("No local branches found.");
        return;
    }

    let branches_to_delete = match MultiSelect::new("Local branches:", branches).prompt() {
        Ok(list) => list,
        Err(e) => {
            eprintln!("select error, {e}");
            process::exit(1);
        }
    };

    if branches_to_delete.is_empty() {
        println!("No branches selected for deletion.");
        return;
    }

    let delete_confirmed = ask_user_for_confirmation("Delete selected branches?");
    if delete_confirmed {
        delete_branches(&branches_to_delete);
        exit_program();
    }
}

fn keyword_or_regex_delete(use_regex: bool) {
    let prompt = if use_regex {
        "Enter regex pattern: "
    } else {
        "Enter keywords (comma-separated): "
    };
    let mut input = String::new();

    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let pattern = input.trim();

    if pattern.is_empty() {
        return;
    }

    // Get all local branches
    let branches = get_local_branches();

    // Filter branches based on pattern
    let matched_branches: Vec<String> = branches
        .iter()
        .filter(|&branch| {
            if use_regex {
                let regex = Regex::new(pattern).expect("Invalid regex pattern");
                regex.is_match(branch)
            } else {
                pattern
                    .split(',')
                    .any(|keyword| branch.contains(keyword.trim()))
            }
        })
        .cloned()
        .collect();

    if matched_branches.is_empty() {
        println!("No branches matched the given pattern.");
        return;
    }

    // Display matched branches to the user
    println!("Matched branches:");
    for (index, branch) in matched_branches.iter().enumerate() {
        println!("{}. {}", index + 1, branch);
    }

    let delete_matched = ask_user_for_confirmation("Delete matched branches?");
    if delete_matched {
        delete_branches(&matched_branches);
        exit_program();
    }
}

fn get_local_branches() -> Vec<String> {
    let output = Command::new("git")
        .args(&["branch", "--format=%(refname:short)"])
        .output()
        .expect("Failed to execute git command");

    let branches = String::from_utf8_lossy(&output.stdout);
    branches
        .lines()
        .map(|branch| branch.trim().to_string())
        .collect()
}

fn delete_branches(branches: &[String]) {
    for branch in branches {
        let delete_command = if cfg!(windows) {
            format!("git branch -D {}", branch)
        } else {
            format!("git branch -d {}", branch)
        };

        let status = Command::new(if cfg!(windows) { "cmd" } else { "sh" })
            .arg(if cfg!(windows) { "/C" } else { "-c" })
            .arg(&delete_command)
            .status()
            .expect("Failed to execute delete command");

        if status.success() {
            println!("Branch '{}' deleted successfully.", branch);
        } else {
            // If the command fails, retry with force delete on Windows
            if cfg!(windows) {
                let force_delete_command = format!("git branch -D {}", branch);
                let force_status = Command::new(if cfg!(windows) { "cmd" } else { "sh" })
                    .arg(if cfg!(windows) { "/C" } else { "-c" })
                    .arg(&force_delete_command)
                    .status()
                    .expect("Failed to execute force delete command");

                if force_status.success() {
                    println!("Branch '{}' force deleted successfully.", branch);
                } else {
                    println!("Failed to delete branch '{}'.", branch);
                }
            } else {
                println!("Failed to delete branch '{}'.", branch);
            }
        }
    }
}

fn ask_user_for_confirmation(prompt: &str) -> bool {
    match Confirm::new(prompt).with_default(false).prompt() {
        Ok(result) => result,
        _ => false
    }
}

fn exit_program() {
    println!("Exiting Clean Git Dev. Goodbye!");
    process::exit(0);
}
