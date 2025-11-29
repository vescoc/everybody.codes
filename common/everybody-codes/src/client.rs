use std::{fs, io};

use aes::cipher::{BlockDecryptMut, KeyIvInit, block_padding};

use reqwest::{
    blocking::{Client as HttpClient, Response},
    header::{self, HeaderMap, HeaderValue},
    redirect::Policy,
};

type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("everybody-codes.cfg not found")]
    MissingCfg,
    #[error("everybody-codes.cfg invalid")]
    InvalidCfg(#[from] toml::de::Error),
    #[error("IO Error")]
    IO(#[from] io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum InputNotesError {
    #[error("Invalid header")]
    InvalidHeader(#[from] header::InvalidHeaderValue),
    #[error("Internal Error")]
    Internal(#[from] reqwest::Error),
    #[error("Invalid data")]
    InvalidData(#[from] serde_json::Error),
    #[error("Decode error")]
    FromHex(#[from] hex::FromHexError),
    #[error("UTF8 error")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("Unpad error")]
    Unpad(#[from] block_padding::UnpadError),
}

#[derive(Debug, serde::Deserialize)]
pub struct Client {
    session: String,
    seed: u32,
}

#[derive(Debug)]
pub struct InputNotes {
    pub part_1: Option<String>,
    pub answer_1: Option<String>,
    pub part_2: Option<String>,
    pub answer_2: Option<String>,
    pub part_3: Option<String>,
    pub answer_3: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
#[allow(clippy::struct_field_names)]
pub struct InputNotesResponse {
    #[serde(rename = "1")]
    part_1: Option<String>,
    #[serde(rename = "2")]
    part_2: Option<String>,
    #[serde(rename = "3")]
    part_3: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct KeysResponse {
    key1: Option<String>,
    key2: Option<String>,
    key3: Option<String>,
    answer1: Option<String>,
    answer2: Option<String>,
    answer3: Option<String>,
}

impl TryFrom<&str> for Client {
    type Error = Error;

    fn try_from(config: &str) -> Result<Self, Self::Error> {
        Ok(toml::from_str(config)?)
    }
}

impl TryFrom<String> for Client {
    type Error = Error;

    fn try_from(config: String) -> Result<Self, Self::Error> {
        Ok(toml::from_str(&config)?)
    }
}

impl Client {
    /// # Errors
    pub fn new_from_config() -> Result<Self, Error> {
        fs::read_to_string(
            dirs::config_dir()
                .ok_or(Error::MissingCfg)?
                .join("everybody-codes.cfg"),
        )?
        .try_into()
    }

    #[must_use]
    pub fn new(session: String, seed: u32) -> Self {
        Self { session, seed }
    }

    /// # Errors
    pub fn input_notes(&self, event: u16, quest: u8) -> Result<InputNotes, InputNotesError> {
        let cookie_header = HeaderValue::from_str(&format!("everybody-codes={}", self.session))?;
        let content_type_header = HeaderValue::from_str("text_plain")?;
        let user_agent_header = HeaderValue::from_str(&format!(
            "{} {}",
            env!("CARGO_PKG_REPOSITORY"),
            env!("CARGO_PKG_VERSION"),
        ))?;

        let url = format!(
            "https://everybody.codes/assets/{event}/{quest}/input/{}.json",
            self.seed
        );

        let mut headers = HeaderMap::new();
        headers.insert(header::COOKIE, cookie_header);
        headers.insert(header::CONTENT_TYPE, content_type_header);
        headers.insert(header::USER_AGENT, user_agent_header);

        let client = HttpClient::builder()
            .default_headers(headers)
            .redirect(Policy::none())
            .build()?;

        let input_notes: InputNotesResponse = client
            .get(url)
            .send()
            .and_then(Response::error_for_status)
            .and_then(Response::json)?;

        let url = format!("https://everybody.codes/api/event/{event}/quest/{quest}");

        let keys: KeysResponse = client
            .get(url)
            .send()
            .and_then(Response::error_for_status)
            .and_then(Response::json)?;

        Ok(InputNotes {
            part_1: input_notes
                .part_1
                .and_then(|encrypted_text| keys.key1.map(|key| decrypt(&encrypted_text, &key)))
                .transpose()?,
            answer_1: keys.answer1,
            part_2: input_notes
                .part_2
                .and_then(|encrypted_text| keys.key2.map(|key| decrypt(&encrypted_text, &key)))
                .transpose()?,
            answer_2: keys.answer2,
            part_3: input_notes
                .part_3
                .and_then(|encrypted_text| keys.key3.map(|key| decrypt(&encrypted_text, &key)))
                .transpose()?,
            answer_3: keys.answer3,
        })
    }
}

fn decrypt(encrypted_text: &str, key: &str) -> Result<String, InputNotesError> {
    let encrypted_bytes = hex::decode(encrypted_text)?;

    let data = Aes256CbcDec::new(key.as_bytes().into(), key.as_bytes()[..16].into())
        .decrypt_padded_vec_mut::<block_padding::Pkcs7>(&encrypted_bytes)?;

    Ok(std::str::from_utf8(&data)?.to_string())
}
