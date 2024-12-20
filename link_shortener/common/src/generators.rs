use image::{ImageFormat, Luma};
use qrcode::QrCode;
use rand::distributions::Slice;
use rand::{thread_rng, Rng};
use sha3::{Digest, Sha3_512};
use std::io::Cursor;
use thiserror::Error;
use uuid::{NoContext, Timestamp, Uuid};

pub fn generate_id() -> String {
    let timestamp = Timestamp::now(NoContext);
    Uuid::new_v7(timestamp).simple().to_string()
}

pub fn generate_random_string(char_set: &[char], length: usize) -> Result<String, Error> {
    if length == 0 {
        return Err(Error::InvalidLength);
    }
    let distribution = Slice::new(char_set).map_err(|_| Error::InvalidCharSet)?;
    Ok(thread_rng().sample_iter(&distribution).take(length).collect())
}

pub fn generate_sha3_512(text: &str) -> String {
    format!("{:x}", Sha3_512::digest(text.as_bytes()))
}

pub fn generate_qr_code(data: &str, width: u32, height: u32) -> Result<Vec<u8>, Error> {
    let qr_code = QrCode::new(data).map_err(|_| Error::InvalidData)?;
    let mut png_data = Cursor::new(Vec::new());
    let image = qr_code.render::<Luma<u8>>().max_dimensions(width, height).build();
    image.write_to(&mut png_data, ImageFormat::Png).map_err(|_| Error::RenderingFailed)?;
    Ok(png_data.into_inner())
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("length must be greater than zero")]
    InvalidLength,
    #[error("character set cannot be empty")]
    InvalidCharSet,
    #[error("invalid QR code data")]
    InvalidData,
    #[error("rendering QR code failed")]
    RenderingFailed,
}
