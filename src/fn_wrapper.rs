use crate::service::FromRequest;
use std::marker::PhantomData;

/// `Handler<(T1, T2)>`基本上等价于 `F(T1, T2)->() + 'static`。
pub trait Handler<T>: 'static {
    fn call(&self, params: T);
}
pub struct FnWrap<F, T> {
    pub f: F,
    _t: PhantomData<T>,
}

// impl<F> FnWrap<F, ()>
// where
//     F: Fn() -> () + 'static,
// {
//     pub fn new(f: F) -> Self {
//         Self { f, _t: PhantomData }
//     }
//     pub fn call(&self, t: ()) {
//         (self.f)()
//     }
// }

impl<F, T> FnWrap<F, T>
where
    F: Handler<T>,
    T: FromRequest,
{
    pub fn new(f: F) -> Self {
        Self { f, _t: PhantomData }
    }
    pub fn call(&self, t: (T,)) {
        self.f.call(t.0)
    }
}
