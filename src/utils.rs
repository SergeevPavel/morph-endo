use serde::Serialize;
use std::path::{Path};


pub fn store<T, P>(data: &T, path: P)
where
    T: Serialize,
    P: AsRef<Path>,
{
    let config = ron::ser::PrettyConfig::new()
        .with_depth_limit(4);
    let file = std::fs::File::create(Path::new("data").join(path)).unwrap();
    ron::ser::to_writer_pretty(std::io::BufWriter::new(file),
                               data,
                               config).unwrap();
}


pub fn load<T, P>(path: P) -> T
where
    P: AsRef<Path>,
    T: serde::de::DeserializeOwned,
{
    let file = std::fs::File::open(Path::new("data").join(path)).unwrap();
    ron::de::from_reader(std::io::BufReader::new(file)).unwrap()
}