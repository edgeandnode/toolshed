use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<Error>>,
}

#[derive(Debug, Deserialize)]
pub struct Error {
    pub message: String,
}

impl<T> Response<T> {
    pub fn unpack(self) -> Result<T, String> {
        self.data.ok_or_else(|| {
            self.errors
                .unwrap_or_default()
                .into_iter()
                .map(|err| err.message)
                .collect::<Vec<String>>()
                .join(", ")
        })
    }
}
