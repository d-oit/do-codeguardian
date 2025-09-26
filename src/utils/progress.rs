use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub struct ProgressReporter {
    enabled: bool,
    progress_bar: Option<ProgressBar>,
}

impl ProgressReporter {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            progress_bar: None,
        }
    }

    pub fn start_scan(&mut self, total_files: usize) {
        if !self.enabled {
            return;
        }

        let progress_bar = ProgressBar::new(total_files as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} files ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        );
        progress_bar.enable_steady_tick(Duration::from_millis(100));

        self.progress_bar = Some(progress_bar);
    }

    pub fn update(&self, message: &str) {
        if let Some(progress_bar) = &self.progress_bar {
            progress_bar.set_message(message.to_string());
            progress_bar.inc(1);
        }
    }

    pub fn finish(&self, message: &str) {
        if let Some(progress_bar) = &self.progress_bar {
            progress_bar.finish_with_message(message.to_string());
        }
    }
}
