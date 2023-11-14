use regex::Regex;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    let mut input = String::new();

    println!("Welcome To Clean Git Dev");
    println!("Firstly, select delete mode (Enter the mode number):");
    println!("1. Interactive");
    println!("2. Keyword");
    println!("3. Regex");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read mode");
    let mode = input.trim().parse::<u32>().expect("Invalid mode input");

    match mode {
        1 => interactive_delete(),
        2 => keyword_or_regex_delete(false),
        3 => keyword_or_regex_delete(true),
        _ => println!("Invalid mode input"),
    }
}

fn interactive_delete() {
    println!("Fetching local branches...");
    let branches = get_local_branches();

    if branches.is_empty() {
        println!("No local branches found.");
        return;
    }

    println!("Local branches:");
    for (index, branch) in branches.iter().enumerate() {
        println!("{}. {}", index + 1, branch);
    }

    let branches_to_delete = get_user_selection(&branches);

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

fn get_user_selection(branches: &[String]) -> Vec<String> {
    let mut input = String::new();

    print!("Enter branch numbers to delete (comma-separated): ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let selected_indices: Vec<usize> = input
        .trim()
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    let selected_branches: Vec<String> = selected_indices
        .iter()
        .filter_map(|&index| branches.get(index - 1).cloned())
        .collect();

    selected_branches
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
    print!("{} (y/n): ", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().eq_ignore_ascii_case("y")
}

fn exit_program() {
    println!("Exiting Clean Git Dev. Goodbye!");
    std::process::exit(0);
}
