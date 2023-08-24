use std::{io::BufReader, file::File};

pub enum AudioSource {
    File(BufReader<File>),
}