// https://mattgathu.github.io/2017/08/29/writing-cli-app-rust.html
use indicatif::{ProgressBar, ProgressStyle};
use std::io::Read;

const TEMPLATE: &str =
    "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})";

pub struct Bar {
    len: Option<u64>,
    bar: ProgressBar,
    chunk_size: usize,
}

impl Bar {
    pub fn new(len: Option<u64>) -> Self {
        let (bar, chunk_size) = match len {
            Some(x) => {
                let bar = ProgressBar::new(x);

                bar.set_style(
                    ProgressStyle::default_bar()
                        .template(TEMPLATE)
                        .progress_chars("#>-"),
                );

                (bar, x as usize / 99)
            }
            None => {
                let bar = ProgressBar::new_spinner();

                bar.set_style(ProgressStyle::default_spinner());

                (bar, 1024usize)
            }
        };

        Self {
            len,
            bar,
            chunk_size,
        }
    }

    pub fn read_start(&self, mut reader: impl Read) -> anyhow::Result<Vec<u8>> {
        let mut buf: Vec<u8> = match self.len {
            Some(x) => Vec::with_capacity(x as usize),
            None => Vec::new(),
        };

        loop {
            let mut buffer = vec![0; self.chunk_size];
            let bcount = reader.read(&mut buffer[..])?;

            buffer.truncate(bcount);

            if !buffer.is_empty() {
                buf.extend(buffer.iter());

                self.bar.inc(bcount as u64);
            } else {
                break;
            }
        }

        self.bar.finish();

        Ok(buf)
    }
}
