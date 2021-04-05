use crate::fn_wrapper::{FnWrap, Handler};
pub trait Service {
    fn handle_request(&self, req: &Request);
}

// impl<F> Service for FnWrap<F, >
// where
//     F: Fn() -> () + 'static,
// {
//     fn handle_request(&self, req: &Request) {
//         self.call(())
//     }
// }
impl<F, T> Service for FnWrap<F, T>
where
    F: Handler<T>,
    T: FromRequest,
{
    fn handle_request(&self, req: &Request) {
        let t = T::from_request(req);
        self.f.call(t)
    }
}

pub struct Request {
    pub s: String,
}

impl Request {
    pub fn new(s: impl Into<String>) -> Self {
        Self { s: s.into() }
    }
}

pub trait FromRequest {
    fn from_request(req: &Request) -> Self;
}
