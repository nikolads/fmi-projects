use super::AdjLists;

use std::convert::From;
use std::io::{self, Read};
use std::str::{self, FromStr};

impl AdjLists {
    pub fn from_file<F: Read>(mut file: F) -> Result<Self, Error> {
        let mut buff = Vec::new();
        file.read_to_end(&mut buff)?;

        let slice = str::from_utf8(&buff)?;
        let mut words = slice.split_whitespace();

        fn read_usize<'a>(words: &mut str::SplitWhitespace<'a>) -> Result<usize, Error> {
            words.next().ok_or(Error::InvalidFormat)
                .and_then(|word| word.parse::<usize>().map_err(|err| Error::Parse(err)))
        };

        let n_vert = read_usize(&mut words)?;
        let n_edges = read_usize(&mut words)?;

        let mut graph = Self::new(n_vert);

        for _ in 0..n_edges {
            let from = read_usize(&mut words)?;
            let to = read_usize(&mut words)?;

            graph.lists[from].push(to);
        }

        Ok(graph)
    }
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    NotUtf8(str::Utf8Error),
    InvalidFormat,
    Parse(<usize as FromStr>::Err),
}

// TODO: impl error::Error for Error

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Self {
        Error::NotUtf8(err)
    }
}
