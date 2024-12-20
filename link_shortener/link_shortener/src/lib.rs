use chrono::{DateTime, Utc};
use common::generators;
use common::generators::generate_random_string;
use common::id::Id;
use common::pagination::{PageRequest, PageResponse};
use std::future::Future;
use thiserror::Error;
use url::Url;

pub async fn create_link(original_url: Url, shortened_path: String, is_active: bool, expires_at: DateTime<Utc>, repository: &impl LinkRepository) -> Result<Link, Error> {
    let link = Link::new(original_url, shortened_path, is_active, expires_at);
    repository.save(link).await
}

pub async fn create_link_with_generated_path(
    original_url: Url, char_set: CharSet, shortened_path_length: usize, is_active: bool, expires_at: DateTime<Utc>, repository: &impl LinkRepository,
) -> Result<Link, Error> {
    let shortened_path = generate_random_string(&char_set.elements(), shortened_path_length)?;
    create_link(original_url, shortened_path, is_active, expires_at, repository).await
}

pub async fn find_link_by_shortened_path(shortened_path: &str, repository: &impl LinkRepository) -> Result<Option<Link>, Error> {
    repository.find_by_shortened_path(shortened_path).await
}

pub async fn find_links(page_request: &PageRequest, repository: &impl LinkRepository) -> Result<PageResponse<Link>, Error> {
    repository.find_all(page_request).await
}

pub async fn find_links_by_tags(tags: &Vec<String>, page_request: &PageRequest, repository: &impl LinkRepository) -> Result<PageResponse<Link>, Error> {
    repository.find_by_tags(tags, page_request).await
}

pub async fn update_link(id: &Id, patch: &LinkPatch, repository: &impl LinkRepository) -> Result<Link, Error> {
    let mut link = repository.find_by_id(id).await?.ok_or(Error::LinkNotFound)?;
    if let Some(original_url) = &patch.original_url {
        link.original_url = original_url.clone();
    }
    if let Some(is_active) = patch.is_active {
        link.is_active = is_active;
    }
    if let Some(expires_at) = &patch.expires_at {
        link.expires_at = expires_at.clone();
    }
    link.tags = patch.tags.iter().map(|tag| tag.to_lowercase()).collect();
    repository.update(link).await
}

pub async fn delete_link(id: &Id, repository: &impl LinkRepository) -> Result<(), Error> {
    repository.delete_by_id(id).await
}

pub async fn delete_expired_links(repository: &impl LinkRepository) -> Result<(), Error> {
    repository.delete_expired().await
}

pub async fn delete_orphaned_tags(repository: &impl LinkRepository) -> Result<(), Error> {
    repository.delete_orphaned_tags().await
}

pub enum CharSet {
    Letters,
    Digits,
    LettersAndDigits,
}

impl CharSet {
    pub fn elements(&self) -> Vec<char> {
        match self {
            CharSet::Letters => ('A'..='Z').collect(),
            CharSet::Digits => ('0'..='9').collect(),
            CharSet::LettersAndDigits => ('A'..='Z').chain('0'..='9').collect(),
        }
    }
}

impl TryFrom<String> for CharSet {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Letters" => Ok(CharSet::Letters),
            "Digits" => Ok(CharSet::Digits),
            "LettersAndDigits" => Ok(CharSet::LettersAndDigits),
            _ => Err(Error::InvalidCharSet),
        }
    }
}

pub struct Link {
    pub id: Id,
    pub original_url: Url,
    pub shortened_path: String,
    pub is_active: bool,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl Link {
    pub fn new(original_url: Url, shortened_path: String, is_active: bool, expires_at: DateTime<Utc>) -> Self {
        Self {
            id: Id::new(),
            original_url,
            shortened_path,
            is_active,
            tags: Vec::new(),
            created_at: Utc::now(),
            expires_at,
        }
    }
}

pub struct LinkPatch {
    pub original_url: Option<Url>,
    pub is_active: Option<bool>,
    pub tags: Vec<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

pub trait LinkRepository: Clone + Send + Sync {
    fn save(&self, link: Link) -> impl Future<Output = Result<Link, Error>> + Send;
    fn find_all(&self, page_request: &PageRequest) -> impl Future<Output = Result<PageResponse<Link>, Error>> + Send;
    fn find_by_id(&self, id: &Id) -> impl Future<Output = Result<Option<Link>, Error>> + Send;
    fn find_by_shortened_path(&self, shortened_path: &str) -> impl Future<Output = Result<Option<Link>, Error>> + Send;
    fn find_by_tags(&self, tags: &Vec<String>, page_request: &PageRequest) -> impl Future<Output = Result<PageResponse<Link>, Error>> + Send;
    fn update(&self, link: Link) -> impl Future<Output = Result<Link, Error>> + Send;
    fn delete_by_id(&self, id: &Id) -> impl Future<Output = Result<(), Error>> + Send;
    fn delete_expired(&self) -> impl Future<Output = Result<(), Error>> + Send;
    fn delete_orphaned_tags(&self) -> impl Future<Output = Result<(), Error>> + Send;
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("shortened path length must be greater than zero")]
    InvalidShortenedPathLength,
    #[error("link with provided shortened path already exists")]
    NonUniqueShortenedPath,
    #[error("invalid character set")]
    InvalidCharSet,
    #[error("generating qr code failed")]
    GeneratingQrCodeFailed,
    #[error("link not found")]
    LinkNotFound,
    #[error("data access error")]
    DataAccessError,
}

impl From<generators::Error> for Error {
    fn from(error: generators::Error) -> Self {
        match error {
            generators::Error::InvalidLength => Error::InvalidShortenedPathLength,
            generators::Error::InvalidCharSet => Error::InvalidCharSet,
            generators::Error::InvalidData | generators::Error::RenderingFailed => Error::GeneratingQrCodeFailed,
        }
    }
}
