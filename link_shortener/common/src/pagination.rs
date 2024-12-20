use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageRequest {
    pub index: usize,
    pub size: usize,
}

impl PageRequest {
    pub fn offset(&self) -> usize {
        self.index * self.size
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResponse<T> {
    pub content: Vec<T>,
    pub total_pages: usize,
}

impl<T> PageResponse<T> {
    pub fn empty() -> Self {
        PageResponse {
            content: Vec::new(),
            total_pages: 0,
        }
    }
    pub fn map<U>(self, mapper: impl FnMut(T) -> U) -> PageResponse<U> {
        PageResponse {
            content: self.content.into_iter().map(mapper).collect(),
            total_pages: self.total_pages,
        }
    }
}
