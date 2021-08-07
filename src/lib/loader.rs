// https://mattgathu.github.io/2017/08/29/writing-cli-app-rust.html
use indicatif::{ProgressBar, ProgressBarIter, ProgressStyle};
use std::io::Read;

const TMPL: &str = "{spinner} [{elapsed_precise}] [{wide_bar}] {bytes}/{total_bytes} ({eta})";

pub struct Spinner {
    s: ProgressBar,
}

impl Spinner {
    pub fn new(msg: &'static str) -> Self {
        let s = ProgressBar::new_spinner();

        s.enable_steady_tick(100);
        s.set_message(msg);

        Self { s }
    }

    pub fn finish(&self) {
        self.s.finish_and_clear();
    }
}

pub struct Bar {
    bar: ProgressBar,
}

impl Bar {
    pub fn new(len: u64) -> Self {
        let bar = ProgressBar::new(len);

        bar.set_style(
            ProgressStyle::default_bar()
                .template(TMPL)
                .progress_chars("#>-"),
        );

        Self { bar }
    }

    pub fn take_reader<R: Read>(&self, r: R) -> ProgressBarIter<R> {
        self.bar.wrap_read(r)
    }

    pub fn finish(&self) {
        self.bar.finish_and_clear()
    }
}
