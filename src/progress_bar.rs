// https://mattgathu.github.io/2017/08/29/writing-cli-app-rust.html
use indicatif::{ProgressBar, ProgressStyle};
use std::io::Read;

const TEMPLATE: &'static str =
    "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})";

pub struct Bar {
    pub bar: ProgressBar,
    pub chunk_size: usize,
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

        Self { bar, chunk_size }
    }

    pub fn read_start<T>(&self, mut reader: T) -> anyhow::Result<Vec<u8>>
    where
        T: Read + Send,
    {
        let mut buf: Vec<u8> = Vec::new();

        loop {
            let mut buffer = vec![0; self.chunk_size];
            let bcount = reader.read(&mut buffer[..])?;

            buffer.truncate(bcount);

            if !buffer.is_empty() {
                buf.extend(buffer.into_boxed_slice().into_vec().iter().cloned());

                self.bar.inc(bcount as u64);
            } else {
                break;
            }
        }

        self.bar.finish();

        Ok(buf)
    }
}
