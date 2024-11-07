pub struct HttpRequest<T> {
    pub body: T,
}

pub struct HttpResponse<T> {
    pub status_code: u32,
    pub data: Option<Result<T, String>>,
}

impl<T> HttpResponse<T> {
    pub fn new() -> Self {
        HttpResponse {
            status_code: 200,
            data: None,
        }
    }

    pub fn status(&mut self, code: u32) -> &mut Self {
        self.status_code = code;
        self
    }

    pub fn json(&mut self, data: Option<Result<T, String>>) -> &mut Self {
        self.data = data;
        self
    }
}
