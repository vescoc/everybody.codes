use std::{fs, io, num, path, collections::HashSet};

use crate::client::{Client, Error as ClientError, InputNotesError};

macro_rules! set {
    () => { std::collections::HashSet::new() };
    ($($e:expr),+) => {
        {
            let mut set = std::collections::HashSet::with_capacity(3);
            $(
                set.insert($e);
            )+
            set
        }
    };
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Client error")]
    Client(#[from] ClientError),
    #[error("Input notes error")]
    InputNotes(#[from] InputNotesError),
    #[error("Package metadata everybody_codes missing")]
    PackageMetadataMissing,
    #[error("IO error")]
    IO(#[from] io::Error),
    #[error("Invalid toml")]
    Toml(#[from] toml::de::Error),
    #[error("Missing event")]
    MissingEvent,
    #[error("Missing quest")]
    MissingQuest,
    #[error("Invalid number conversion")]
    Number(#[from] num::TryFromIntError),
    #[error("Invalid number")]
    InvalidNumber,
}

pub struct PartsData<'a> {
    event: u16,
    quest: u8,
    parts: HashSet<u8>,
    data_dir: &'a str,
}

impl<'a> PartsData<'a> {
    #[must_use]
    pub fn new(event: u16, quest: u8, parts: HashSet<u8>, data_dir: &'a str) -> Self {
        Self { event, quest, parts, data_dir }            
    }

    /// # Errors
    pub fn new_from_cargo(data_dir: &'a str) -> Result<Self, Error> {
        let config = fs::read_to_string("Cargo.toml")?
            .parse::<toml::Table>()?;

        let data = config
            .get("package")
            .and_then(|value| value.get("metadata"))
            .and_then(|value| value.get("everybody_codes"))
            .ok_or(Error::PackageMetadataMissing)?;

        let event = get_int(data.get("event").ok_or(Error::MissingEvent)?)?;
        let quest = get_int(data.get("quest").ok_or(Error::MissingQuest)?)?;
        let parts = data.get("parts").map(get_ints).transpose()?.unwrap_or_else(|| set![1, 2, 3]);

        Ok(Self { event, quest, parts, data_dir })
    }

    /// # Errors
    pub fn store_if_necessary(&mut self) -> Result<bool, Error> {
        let data_dir = path::Path::new(self.data_dir);

        let (part_1_path, need_part_1, part_1_data) = self.need_part(data_dir, 1);
        let (part_2_path, need_part_2, part_2_data) = self.need_part(data_dir, 2);
        let (part_3_path, need_part_3, part_3_data) = self.need_part(data_dir, 3);

        if !need_part_1 && !need_part_2 && !need_part_3 {
            return Ok(false);
        }
        
        let input_notes = Client::new_from_config()?.input_notes(self.event, self.quest)?;

        let mut modified = false;

        if let Some(remote_data) = input_notes.part_1 && need_part_1 {
            if let Some(local_data) = part_1_data {
                if remote_data != local_data {
                    fs::create_dir_all(data_dir)?;
                    fs::write(part_1_path, remote_data)?;
                    modified = true;
                }
            } else {
                fs::create_dir_all(data_dir)?;
                fs::write(part_1_path, remote_data)?;
                modified = true;
            }
        }
        
        if let Some(remote_data) = input_notes.part_2 && need_part_2 {
            if let Some(local_data) = part_2_data {
                if remote_data != local_data {
                    fs::create_dir_all(data_dir)?;
                    fs::write(part_2_path, remote_data)?;
                    modified = true;
                }
            } else {
                fs::create_dir_all(data_dir)?;
                fs::write(part_2_path, remote_data)?;
                modified = true;
            }
        }
        
        if let Some(remote_data) = input_notes.part_3 && need_part_3 {
            if let Some(local_data) = part_3_data {
                if remote_data != local_data {
                    fs::create_dir_all(data_dir)?;
                    fs::write(part_3_path, remote_data)?;
                    modified = true;
                }
            } else {
                fs::create_dir_all(data_dir)?;
                fs::write(part_3_path, remote_data)?;
                modified = true;
            }
        }

        Ok(modified)
    }

    fn need_part(&self, data_dir: &path::Path, part: u8) -> (path::PathBuf, bool, Option<String>) {
        let part_path = data_dir.join(format!("part_{part}"));
        if self.parts.contains(&part) {
            let (need_part, part_data) = match fs::read_to_string(&part_path) {
                Ok(data) => (data.trim().is_empty(), Some(data)),
                Err(_) => (true, None),
            };
            (part_path, need_part, part_data)
        } else {
            (part_path, false, None)
        }
    }
}

fn get_int<T>(value: &toml::Value) -> Result<T, Error>
where
    T: TryFrom<i64>,
    Error: From<<T as TryFrom<i64>>::Error>,
{
    match value {
        toml::Value::Integer(v) => Ok(T::try_from(*v)?),
        _ => Err(Error::InvalidNumber),
    }
}

fn get_ints<T>(value: &toml::Value) -> Result<HashSet<T>, Error>
where
    T: TryFrom<i64>,
    Error: From<<T as TryFrom<i64>>::Error>,
    T: Eq + std::hash::Hash,
{
    match value {
        toml::Value::Array(array) => array.iter().map(get_int).collect::<Result<HashSet<_>, _>>(),
        _ => Err(Error::InvalidNumber),
    }
}    
