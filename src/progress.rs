use indicatif::{ProgressBar, ProgressStyle};

pub struct Progress {
    pub progress_bar: ProgressBar,
}

impl Progress {
    pub fn new(total: u64) -> Self {
        let progress_bar = ProgressBar::new(total);
        progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.yellow} [{elapsed_precise}] [{bar:40.green/red}] {pos:>7}/{len:7} ({eta})")
            .map(|style| style.progress_chars("#>-")) // Apply progress_chars if Ok
            .unwrap_or_else(|e| {
                eprintln!("Failed to set progress bar style: {}", e);
                ProgressStyle::default_bar() // Use default style on error
            }));
        Self { progress_bar }
    }

    pub fn inc(&self, step: u64) {
        self.progress_bar.inc(step);
    }

    pub fn finish(&self) {
        self.progress_bar.finish_with_message("Busting complete!");
    }
}
