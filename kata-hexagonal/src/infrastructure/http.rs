pub struct HttpRequest<T> {
    pub body: T,
}

pub trait HttpResponse<T> {
    fn status(&mut self, code: u16) -> &mut Self;
    fn json(&mut self, data: T) -> &mut Self;
}
