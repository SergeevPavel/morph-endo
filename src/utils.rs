use serde::Serialize;
use std::path::PathBuf;


pub fn store<T, S>(data: &T, file_name: S)
where
    T: Serialize,
    S: AsRef<str>,
{
    let config = ron::ser::PrettyConfig::new()
        .with_depth_limit(4);
    let path: PathBuf = ["data", file_name.as_ref()].iter().collect();
    let file = std::fs::File::create(path).unwrap();
    ron::ser::to_writer_pretty(std::io::BufWriter::new(file),
                               data,
                               config).unwrap();
}


pub fn load<T, S>(file_name: S) -> T
where
    S: AsRef<str>,
    T: serde::de::DeserializeOwned,
{
    let path: PathBuf = ["data", file_name.as_ref()].iter().collect();
    let file = std::fs::File::open(path).unwrap();
    ron::de::from_reader(std::io::BufReader::new(file)).unwrap()
}