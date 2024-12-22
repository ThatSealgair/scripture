use clap::Parser;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use textwrap::fill;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Verify if a commit message follows standards
    #[arg(short = 'm', long = "message")]
    message_string: Option<String>,

    /// Verify if a commit message file follows standards
    #[arg(short = 'f', long = "file")]
    message_file: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    standard_verbs: HashMap<String, String>,
    indicators: HashMap<String, Vec<String>>,
    verb_mapping: HashMap<String, String>,
    message_template: MessageTemplate,
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageTemplate {
    references_section: String,
    testing_section: String,
    dependencies_section: String,
    changes_section: String,
    breaking_section: String,
}

#[derive(Debug)]
struct GitChanges {
    file_changes: HashMap<String, Vec<String>>,
    breaking_changes: Vec<String>,
}

impl GitChanges {
    fn has_changes(&self) -> bool {
        !self.file_changes.is_empty()
    }
}

const COMMIT_INSTRUCTIONS: &str = r#"
To utilise this commit message:

1. Review the generated commit.md file
2. Complete any sections marked with [Required]
3. Update any sections marked with [Optional]
4. Use it directly with git commit:
   git commit -F commit.md

Or copy specific sections into your commit:
   cat commit.md | git commit -F -
"#;

impl Default for Config {
    fn default() -> Self {
        let mut standard_verbs = HashMap::new();
        standard_verbs.insert(
            "Add".to_string(),
            "Create a capability, e.g. feature, test, dependency".to_string(),
        );
        standard_verbs.insert(
            "Cut".to_string(),
            "Remove a capability, e.g. feature, test, dependency".to_string(),
        );
        standard_verbs.insert(
            "Fix".to_string(),
            "Fix an issue, e.g. bug, typo, error, misstatement".to_string(),
        );
        // ... Add other verbs

        let mut indicators = HashMap::new();
        indicators.insert(
            "fix".to_string(),
            vec!["fix".to_string(), "bug".to_string(), "issue".to_string()],
        );
        // ... Add other indicators

        let mut verb_mapping = HashMap::new();
        verb_mapping.insert("fix".to_string(), "Fix".to_string());
        // ... Add other mappings

        Config {
            standard_verbs,
            indicators,
            verb_mapping,
            message_template: MessageTemplate {
                references_section: "# References [Required]\n# Link to related tickets, docs, or discussions\nCloses #\nRelates to #\nSee also: ".to_string(),
                testing_section: "# Testing Instructions [Optional]\n# Describe how to test these changes\n1. Steps to test\n2. Expected outcomes\n3. Edge cases to verify".to_string(),
                dependencies_section: "# Dependencies [Optional]\n# List any prerequisite changes or dependencies\n- [ ] Database migrations\n- [ ] Configuration updates\n- [ ] External service changes".to_string(),
                changes_section: "# Changes Overview [Required]\n# Briefly describe the purpose of these changes".to_string(),
                breaking_section: "# Breaking Changes [Required if any]\n# List any backward-incompatible changes and migration steps".to_string(),
            },
        }
    }
}

struct CommitMessageVerifier {
    config: Config,
}

impl CommitMessageVerifier {
    fn new(config: Config) -> Self {
        Self { config }
    }

    fn verify_message(&self, message: &str) -> (bool, Vec<String>) {
        let mut errors = Vec::new();
        let lines: Vec<&str> = message.lines().collect();

        if lines.is_empty() {
            return (false, vec!["Empty commit message".to_string()]);
        }

        let subject = lines[0];
        if subject.len() > 50 {
            errors.push("Subject line exceeds 50 characters".to_string());
        }

        let first_word = subject.split_whitespace().next().unwrap_or("");
        if !self.config.standard_verbs.contains_key(first_word) {
            errors.push(format!(
                "Subject must start with standard verb: {}",
                self.config
                    .standard_verbs
                    .keys()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        if subject.ends_with('.') {
            errors.push("Subject line ends with a full stop".to_string());
        }

        if !subject.chars().next().map_or(false, |c| c.is_uppercase()) {
            errors.push("Subject line not capitalised".to_string());
        }

        if lines.get(1).map_or(false, |line| !line.is_empty()) {
            errors.push("No blank line between subject and body".to_string());
        }

        for (i, line) in lines.iter().skip(2).enumerate() {
            if !line.is_empty() && line.len() > 72 {
                errors.push(format!("Line {} exceeds 72 characters", i + 3));
            }
        }

        (!errors.is_empty(), errors)
    }

    fn verify_file(&self, file_path: &PathBuf) -> (bool, Vec<String>) {
        match fs::read_to_string(file_path) {
            Ok(message) => self.verify_message(&message),
            Err(e) => (false, vec![format!("Failed to read file: {}", e)]),
        }
    }
}

struct GitDiffAnalyzer {
    config: Config,
}

impl GitDiffAnalyzer {
    fn new(config: Config) -> Self {
        Self { config }
    }

    fn get_git_diff(&self) -> Option<String> {
        let output = Command::new("git")
            .args(["diff", "--cached"])
            .output()
            .ok()?;

        String::from_utf8(output.stdout).ok()
    }

    fn analyse_diff(&self, diff_output: &str) -> GitChanges {
        let mut file_changes = HashMap::new();
        let mut breaking_changes = Vec::new();
        let mut current_file = None;

        for line in diff_output.lines() {
            if line.starts_with("diff --git") {
                current_file = line
                    .split_whitespace()
                    .last()
                    .map(|s| s.trim_start_matches("b/").to_string());
            } else if line.starts_with('+') && !line.starts_with("+++") {
                if let Some(file) = &current_file {
                    let change = line[1..].trim().to_string();
                    if !change.is_empty() {
                        file_changes
                            .entry(file.clone())
                            .or_insert_with(Vec::new)
                            .push(change.clone());

                        if self.is_breaking_change(&change) {
                            breaking_changes
                                .push(format!("* Breaking change in {}:\n  {}", file, change));
                        }
                    }
                }
            }
        }

        GitChanges {
            file_changes,
            breaking_changes,
        }
    }

    fn is_breaking_change(&self, change: &str) -> bool {
        let breaking_indicators = [
            "remove",
            "delete",
            "deprecate",
            "break",
            "change",
            "rename",
            "refactor",
            "drop",
            "migrate",
        ];

        let change_lower = change.to_lowercase();
        breaking_indicators
            .iter()
            .any(|&word| change_lower.contains(word))
    }

    fn determine_commit_verb(&self, file_changes: &HashMap<String, Vec<String>>) -> String {
        let all_changes: String = file_changes
            .values()
            .flatten()
            .map(|s| s.to_lowercase())
            .collect::<Vec<_>>()
            .join(" ");

        for (verb, words) in &self.config.indicators {
            if words.iter().any(|word| all_changes.contains(word)) {
                return self
                    .config
                    .verb_mapping
                    .get(verb)
                    .unwrap_or(&"Add".to_string())
                    .clone();
            }
        }

        "Add".to_string()
    }
}

struct CommitMessageGenerator<'a> {
    analyzer: &'a GitDiffAnalyzer,
}

impl<'a> CommitMessageGenerator<'a> {
    fn new(analyzer: &'a GitDiffAnalyzer) -> Self {
        Self { analyzer }
    }

    fn generate_subject_line(&self, changes: &GitChanges) -> String {
        let verb = self.analyzer.determine_commit_verb(&changes.file_changes);

        let significant_changes: Vec<_> = changes
            .file_changes
            .values()
            .filter_map(|changes| changes.first())
            .collect();

        let description = significant_changes
            .first()
            .map(|s| s.trim().to_lowercase())
            .unwrap_or_else(|| "codebase".to_string());

        let subject = format!("{} {}", verb, description);
        if subject.len() > 50 {
            format!("{}...", &subject[..47])
        } else {
            subject
        }
    }

    fn wrap_body_text(&self, text: &str) -> String {
        fill(text, 72)
    }

    fn generate_message(&self, changes: &GitChanges) -> String {
        let subject = self.generate_subject_line(changes);
        let templates = &self.analyzer.config.message_template;

        let mut sections = vec![templates.references_section.clone()];

        let mut changes_section = format!("{}\n\n", templates.changes_section);
        for (file, changes_list) in &changes.file_changes {
            changes_section.push_str(&format!("* In {}:\n", file));
            for change in changes_list.iter().take(3) {
                changes_section.push_str(&format!("  - {}\n", self.wrap_body_text(change)));
            }
        }
        sections.push(changes_section);

        if !changes.breaking_changes.is_empty() {
            let mut breaking = format!("{}\n\n", templates.breaking_section);
            for change in &changes.breaking_changes {
                breaking.push_str(&format!("{}\n", self.wrap_body_text(change)));
            }
            sections.push(breaking);
        }

        sections.push(templates.testing_section.clone());
        sections.push(templates.dependencies_section.clone());

        format!("{}\n\n{}", subject, sections.join("\n\n"))
    }
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    let config = Config::default();

    if let Some(message) = cli.message_string {
        let verifier = CommitMessageVerifier::new(config);
        let (valid, errors) = verifier.verify_message(&message);
        if !valid {
            error!("Commit message validation failed:");
            for error in errors {
                error!("- {}", error);
            }
            std::process::exit(1);
        }
        info!("Commit message is valid");
        return;
    }

    if let Some(file_path) = cli.message_file {
        let verifier = CommitMessageVerifier::new(config);
        let (valid, errors) = verifier.verify_file(&file_path);
        if !valid {
            error!("Commit message validation failed:");
            for error in errors {
                error!("- {}", error);
            }
            std::process::exit(1);
        }
        info!("Commit message is valid");
        return;
    }

    let analyzer = GitDiffAnalyzer::new(config);
    let generator = CommitMessageGenerator::new(&analyzer);

    let diff_output = match analyzer.get_git_diff() {
        Some(diff) => diff,
        None => {
            error!("No staged changes found. Please stage changes with 'git add' first.");
            std::process::exit(1);
        }
    };

    let changes = analyzer.analyse_diff(&diff_output);
    if !changes.has_changes() {
        error!("No changes detected in diff.");
        std::process::exit(1);
    }

    let commit_message = generator.generate_message(&changes);

    match fs::write("commit.md", &commit_message) {
        Ok(_) => {
            info!("\n=== Generated Commit Message ===\n");
            println!("{}", commit_message);
            info!("\n===========================");
            info!("{}", COMMIT_INSTRUCTIONS);
        }
        Err(e) => {
            error!("Failed to write commit message: {}", e);
            std::process::exit(1);
        }
    }
}
