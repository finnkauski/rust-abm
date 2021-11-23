use crate::data::Employee;
use std::path::Path;

pub fn load_people(path: &Path) -> Result<Vec<Employee>, csv::Error> {
    let mut reader = csv::Reader::from_path(path)?;
    reader.deserialize().collect()
}
