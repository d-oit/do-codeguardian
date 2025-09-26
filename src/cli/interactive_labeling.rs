//! Interactive Labeling Tool for Training Data
//!
//! Provides a terminal-based interface for manually labeling findings
//! to create high-quality training datasets.

use crate::ml::feature_extractor::FeatureExtractor;
use crate::ml::training_data::{FeedbackSource, TrainingDataset, TrainingExample};
use crate::types::{Finding, Severity};
use anyhow::Result;
use crossterm::{
    cursor, execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use std::io::{self, Write};

/// Interactive labeling session
pub struct InteractiveLabelingSession {
    feature_extractor: FeatureExtractor,
    dataset: TrainingDataset,
    current_index: usize,
    findings: Vec<Finding>,
    stats: LabelingStats,
}

#[derive(Debug, Default)]
pub struct LabelingStats {
    pub total_findings: usize,
    pub labeled: usize,
    pub skipped: usize,
    pub true_positives: usize,
    pub false_positives: usize,
    pub session_time: std::time::Duration,
}

impl InteractiveLabelingSession {
    /// Create new interactive labeling session
    pub fn new(findings: Vec<Finding>) -> Self {
        let total_findings = findings.len();
        Self {
            feature_extractor: FeatureExtractor::new(),
            dataset: TrainingDataset::new(),
            current_index: 0,
            findings,
            stats: LabelingStats {
                total_findings,
                ..Default::default()
            },
        }
    }

    /// Start the interactive labeling session
    pub async fn run(&mut self) -> Result<TrainingDataset> {
        let start_time = std::time::Instant::now();

        // Setup terminal
        terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, terminal::Clear(ClearType::All))?;

        println!("🏷️  Interactive Training Data Labeling");
        println!("=========================================");
        println!();
        println!("Instructions:");
        println!("  [y] - Mark as True Positive (real issue)");
        println!("  [n] - Mark as False Positive (false alarm)");
        println!("  [s] - Skip this finding");
        println!("  [?] - Show help");
        println!("  [q] - Quit and save progress");
        println!();
        println!("Press any key to start...");

        // Wait for user to start
        self.wait_for_key()?;

        // Main labeling loop
        while self.current_index < self.findings.len() {
            let finding = &self.findings[self.current_index];

            // Clear screen and show current finding
            execute!(stdout, terminal::Clear(ClearType::All))?;
            self.display_finding(finding)?;
            self.display_progress()?;

            // Get user input
            match self.get_user_input()? {
                UserAction::TruePositive => {
                    self.label_finding(finding, true).await?;
                    self.stats.true_positives += 1;
                    self.stats.labeled += 1;
                    self.current_index += 1;
                }
                UserAction::FalsePositive => {
                    self.label_finding(finding, false).await?;
                    self.stats.false_positives += 1;
                    self.stats.labeled += 1;
                    self.current_index += 1;
                }
                UserAction::Skip => {
                    self.stats.skipped += 1;
                    self.current_index += 1;
                }
                UserAction::Help => {
                    self.show_help()?;
                }
                UserAction::Quit => {
                    break;
                }
                UserAction::Previous => {
                    if self.current_index > 0 {
                        self.current_index -= 1;
                    }
                }
            }
        }

        // Cleanup terminal
        terminal::disable_raw_mode()?;
        execute!(stdout, terminal::Clear(ClearType::All))?;

        // Update session stats
        self.stats.session_time = start_time.elapsed();

        // Show final summary
        self.show_summary()?;

        Ok(self.dataset.clone())
    }

    /// Display current finding information
    fn display_finding(&self, finding: &Finding) -> Result<()> {
        let mut stdout = io::stdout();

        // Header
        queue!(stdout, SetForegroundColor(Color::Cyan))?;
        queue!(
            stdout,
            Print(format!(
                "Finding {}/{}\n",
                self.current_index + 1,
                self.findings.len()
            ))
        )?;
        queue!(stdout, ResetColor)?;
        queue!(stdout, Print("─".repeat(50)))?;
        queue!(stdout, Print("\n\n"))?;

        // Severity with color coding
        let severity_color = match finding.severity {
            Severity::Critical => Color::Red,
            Severity::High => Color::DarkRed,
            Severity::Medium => Color::Yellow,
            Severity::Low => Color::Blue,
            Severity::Info => Color::Grey,
        };

        queue!(stdout, SetForegroundColor(severity_color))?;
        queue!(stdout, Print(format!("Severity: {:?}\n", finding.severity)))?;
        queue!(stdout, ResetColor)?;

        // Basic information
        queue!(stdout, Print(format!("Analyzer: {}\n", finding.analyzer)))?;
        queue!(stdout, Print(format!("Rule: {}\n", finding.rule)))?;
        queue!(stdout, Print(format!("File: {}\n", finding.file.display())))?;
        queue!(stdout, Print(format!("Line: {}\n", finding.line)))?;
        queue!(stdout, Print("\n"))?;

        // Message
        queue!(stdout, SetForegroundColor(Color::White))?;
        queue!(stdout, Print("Message:\n"))?;
        queue!(stdout, ResetColor)?;
        queue!(stdout, Print(format!("  {}\n", finding.message)))?;
        queue!(stdout, Print("\n"))?;

        // Description if available
        if let Some(description) = &finding.description {
            queue!(stdout, SetForegroundColor(Color::DarkCyan))?;
            queue!(stdout, Print("Description:\n"))?;
            queue!(stdout, ResetColor)?;
            queue!(stdout, Print(format!("  {}\n", description)))?;
            queue!(stdout, Print("\n"))?;
        }

        // Suggestion if available
        if let Some(suggestion) = &finding.suggestion {
            queue!(stdout, SetForegroundColor(Color::Green))?;
            queue!(stdout, Print("Suggestion:\n"))?;
            queue!(stdout, ResetColor)?;
            queue!(stdout, Print(format!("  {}\n", suggestion)))?;
            queue!(stdout, Print("\n"))?;
        }

        stdout.flush()?;
        Ok(())
    }

    /// Display progress information
    fn display_progress(&self) -> Result<()> {
        let mut stdout = io::stdout();

        queue!(stdout, Print("─".repeat(50)))?;
        queue!(stdout, Print("\n"))?;

        let progress_percent = (self.current_index as f64 / self.findings.len() as f64) * 100.0;
        queue!(
            stdout,
            Print(format!("Progress: {:.1}% ", progress_percent))
        )?;

        // Progress bar
        let bar_width = 20;
        let filled =
            ((self.current_index as f64 / self.findings.len() as f64) * bar_width as f64) as usize;
        queue!(stdout, Print("["))?;
        queue!(stdout, SetForegroundColor(Color::Green))?;
        queue!(stdout, Print("█".repeat(filled)))?;
        queue!(stdout, ResetColor)?;
        queue!(stdout, Print("░".repeat(bar_width - filled)))?;
        queue!(stdout, Print("]\n"))?;

        queue!(
            stdout,
            Print(format!(
                "Labeled: {} | Skipped: {} | TP: {} | FP: {}\n",
                self.stats.labeled,
                self.stats.skipped,
                self.stats.true_positives,
                self.stats.false_positives
            ))
        )?;
        queue!(stdout, Print("\n"))?;

        // Action prompt
        queue!(stdout, SetForegroundColor(Color::Yellow))?;
        queue!(
            stdout,
            Print("Is this a real issue? [y]es / [n]o / [s]kip / [?]help / [q]uit: ")
        )?;
        queue!(stdout, ResetColor)?;

        stdout.flush()?;
        Ok(())
    }

    /// Get user input and return action
    fn get_user_input(&self) -> Result<UserAction> {
        loop {
            if let Ok(event) = crossterm::event::read() {
                if let crossterm::event::Event::Key(key_event) = event {
                    match key_event.code {
                        crossterm::event::KeyCode::Char('y')
                        | crossterm::event::KeyCode::Char('Y') => {
                            return Ok(UserAction::TruePositive);
                        }
                        crossterm::event::KeyCode::Char('n')
                        | crossterm::event::KeyCode::Char('N') => {
                            return Ok(UserAction::FalsePositive);
                        }
                        crossterm::event::KeyCode::Char('s')
                        | crossterm::event::KeyCode::Char('S') => {
                            return Ok(UserAction::Skip);
                        }
                        crossterm::event::KeyCode::Char('?') => {
                            return Ok(UserAction::Help);
                        }
                        crossterm::event::KeyCode::Char('q')
                        | crossterm::event::KeyCode::Char('Q') => {
                            return Ok(UserAction::Quit);
                        }
                        crossterm::event::KeyCode::Char('p')
                        | crossterm::event::KeyCode::Char('P') => {
                            return Ok(UserAction::Previous);
                        }
                        crossterm::event::KeyCode::Esc => {
                            return Ok(UserAction::Quit);
                        }
                        _ => {
                            // Invalid key, continue loop
                        }
                    }
                }
            }
        }
    }

    /// Wait for any key press
    fn wait_for_key(&self) -> Result<()> {
        crossterm::event::read()?;
        Ok(())
    }

    /// Show help screen
    fn show_help(&self) -> Result<()> {
        let mut stdout = io::stdout();
        execute!(stdout, terminal::Clear(ClearType::All))?;

        queue!(stdout, SetForegroundColor(Color::Cyan))?;
        queue!(stdout, Print("Help - Training Data Labeling\n"))?;
        queue!(stdout, ResetColor)?;
        queue!(stdout, Print("═".repeat(40)))?;
        queue!(stdout, Print("\n\n"))?;

        queue!(stdout, Print("Labeling Guidelines:\n\n"))?;

        queue!(stdout, SetForegroundColor(Color::Green))?;
        queue!(stdout, Print("True Positive (y)"))?;
        queue!(stdout, ResetColor)?;
        queue!(stdout, Print(" - Mark if:\n"))?;
        queue!(
            stdout,
            Print("  • Finding represents a real security issue\n")
        )?;
        queue!(
            stdout,
            Print("  • Code quality problem that should be fixed\n")
        )?;
        queue!(stdout, Print("  • Performance issue with impact\n"))?;
        queue!(stdout, Print("  • Compliance violation\n\n"))?;

        queue!(stdout, SetForegroundColor(Color::Red))?;
        queue!(stdout, Print("False Positive (n)"))?;
        queue!(stdout, ResetColor)?;
        queue!(stdout, Print(" - Mark if:\n"))?;
        queue!(stdout, Print("  • Finding is incorrect or misleading\n"))?;
        queue!(stdout, Print("  • Code is acceptable in this context\n"))?;
        queue!(stdout, Print("  • Test code with intentional patterns\n"))?;
        queue!(stdout, Print("  • Documentation or comments\n\n"))?;

        queue!(stdout, SetForegroundColor(Color::Yellow))?;
        queue!(stdout, Print("Skip (s)"))?;
        queue!(stdout, ResetColor)?;
        queue!(stdout, Print(" - Use when:\n"))?;
        queue!(stdout, Print("  • Uncertain about the classification\n"))?;
        queue!(stdout, Print("  • Need more context to decide\n"))?;
        queue!(stdout, Print("  • Complex edge case\n\n"))?;

        queue!(stdout, Print("Navigation:\n"))?;
        queue!(stdout, Print("  • p - Go to previous finding\n"))?;
        queue!(stdout, Print("  • q - Quit and save progress\n"))?;
        queue!(stdout, Print("  • ? - Show this help\n\n"))?;

        queue!(stdout, Print("Press any key to continue..."))?;
        stdout.flush()?;

        self.wait_for_key()?;
        Ok(())
    }

    /// Label a finding and add to dataset
    async fn label_finding(&mut self, finding: &Finding, is_true_positive: bool) -> Result<()> {
        let features = self.feature_extractor.extract_features(finding)?;

        let example = TrainingExample {
            finding_id: finding.id.clone(),
            features,
            is_true_positive,
            feedback_source: FeedbackSource::UserFeedback,
            timestamp: chrono::Utc::now(),
        };

        self.dataset.add_example(example);
        Ok(())
    }

    /// Show final session summary
    fn show_summary(&self) -> Result<()> {
        println!("\n🎉 Labeling Session Complete!");
        println!("════════════════════════════════");
        println!();
        println!("📊 Session Statistics:");
        println!("  • Total findings: {}", self.stats.total_findings);
        println!("  • Labeled: {}", self.stats.labeled);
        println!("  • Skipped: {}", self.stats.skipped);
        println!("  • True positives: {}", self.stats.true_positives);
        println!("  • False positives: {}", self.stats.false_positives);

        if self.stats.labeled > 0 {
            let accuracy_estimate = self.stats.true_positives as f64 / self.stats.labeled as f64;
            println!("  • True positive rate: {:.1}%", accuracy_estimate * 100.0);
        }

        println!(
            "  • Session time: {:.1} minutes",
            self.stats.session_time.as_secs_f64() / 60.0
        );

        if self.stats.labeled > 0 {
            let labels_per_minute =
                self.stats.labeled as f64 / (self.stats.session_time.as_secs_f64() / 60.0);
            println!("  • Labeling rate: {:.1} labels/minute", labels_per_minute);
        }

        println!();

        // Data quality assessment
        if self.stats.labeled >= 10 {
            let balance_ratio =
                self.stats.true_positives as f64 / self.stats.false_positives.max(1) as f64;
            println!("📈 Data Quality:");
            println!("  • Balance ratio: {:.2}", balance_ratio);

            if balance_ratio > 0.3 && balance_ratio < 3.0 {
                println!("  • ✅ Well-balanced dataset");
            } else if balance_ratio > 10.0 || balance_ratio < 0.1 {
                println!("  • ⚠️  Severely imbalanced - consider more diverse examples");
            } else {
                println!("  • ⚠️  Moderately imbalanced - could benefit from more examples");
            }
        }

        println!();
        Ok(())
    }
}

#[derive(Debug)]
enum UserAction {
    TruePositive,
    FalsePositive,
    Skip,
    Help,
    Quit,
    Previous,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Finding;
    use std::path::PathBuf;

    #[test]
    fn test_labeling_session_creation() -> Result<(), Box<dyn std::error::Error>> {
        let findings = vec![Finding::new(
            "test_analyzer",
            "test_rule",
            Severity::Medium,
            PathBuf::from("test.rs"),
            42,
            "Test finding".to_string(),
        )];

        let session = InteractiveLabelingSession::new(findings);
        assert_eq!(session.stats.total_findings, 1);
        assert_eq!(session.current_index, 0);
    }
}
