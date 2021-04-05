use crate::fn_wrapper::{FnWrap, Handler};
use crate::service::{FromRequest, Request, Service};

struct App {
    services: Vec<Box<dyn Service>>,
}

impl App {
    pub fn new() -> Self {
        Self { services: vec![] }
    }
    pub fn handler<F, T>(mut self, f: F) -> Self
    where
        F: Handler<T>,
        T: FromRequest + 'static,
    {
        let f = FnWrap::new(f);
        self.services.push(Box::new(f));
        self
    }
    pub fn dispatch(&self, req: Request) {
        for s in self.services.iter() {
            s.handle_request(&req);
        }
    }
}

#[test]
fn test_my_try() {
    use mockall::*;

    #[automock]
    pub trait Handle {
        fn f0();
        fn f1(s: String);
        fn f2(n: u32, s: String);
        fn f3(t: (), s: String, n: u32);
    }
    let f0_ctx = MockHandle::f0_context();
    f0_ctx.expect().times(1).returning(|| {});

    let f1_ctx = MockHandle::f1_context();
    f1_ctx.expect().times(1).returning(|s: String| {
        assert_eq!(s, "123");
    });

    let f2_ctx = MockHandle::f2_context();
    f2_ctx.expect().times(1).returning(|n, s| {
        assert_eq!(n, 123);
        assert_eq!(s, "123");
    });

    let f3_ctx = MockHandle::f3_context();
    f3_ctx.expect().times(1).returning(|t, s, n| {
        assert_eq!(n, 123);
        assert_eq!(s, "123");
    });

    let app = App::new()
        .handler(MockHandle::f0)
        .handler(|s: String| MockHandle::f1(s))
        .handler(MockHandle::f2)
        .handler(MockHandle::f3);
    app.dispatch(Request::new("123"));
}

#[rustfmt::skip]
mod _impl_handler {
    use crate::fn_wrapper::Handler;
    // delegate
    impl<F> Handler<()> for F where F: Fn() -> () + 'static {
        fn call(&self, params: ()) {
            (self)()
        }
    }
    macro_rules! f {
        (($($Ts:ident),*), ($($Ns:tt),*)) => {
            impl<F, $($Ts,)*> Handler<( $($Ts, )* )> for F
            where
                F: Fn( $($Ts,)* ) -> () + 'static
            {
                fn call(&self, params: ( $($Ts,)* )) {
                    (self)(
                        $(params.$Ns, )*
                    )
                }
            }
        };
    }
    f!((T1), (0));
    f!((T1, T2), (0, 1));
    f!((T1, T2, T3), (0, 1, 2));
    f!((T1, T2, T3, T4), (0, 1, 2, 3));
    f!((T1, T2, T3, T4, T5), (0, 1, 2, 3, 4));
    f!((T1, T2, T3, T4, T5, T6), (0, 1, 2, 3, 4, 5));
    f!((T1, T2, T3, T4, T5, T6, T7), (0, 1, 2, 3, 4, 5, 6));
    f!((T1, T2, T3, T4, T5, T6, T7, T8), (0, 1, 2, 3, 4, 5, 6, 7));
    f!((T1, T2, T3, T4, T5, T6, T7, T8, T9), (0, 1, 2, 3, 4, 5, 6, 7, 8));
    f!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (0, 1, 2, 3, 4, 5, 6, 7, 8, 9));
}

#[rustfmt::skip]
mod _impl_from_request {
    use crate::service::{FromRequest, Request};

    impl FromRequest for String {
        fn from_request(req: &Request) -> Self {
            req.s.clone()
        }
    }
    impl FromRequest for u32 {
        fn from_request(req: &Request) -> Self {
            req.s.parse().unwrap()
        }
    }
    impl FromRequest for () {
        fn from_request(req: &Request) -> Self {
            ()
        }
    }
    // propagate
    macro_rules! f {
        ($($Ts:tt),*) => {
            impl< $($Ts,)* > FromRequest for ( $($Ts,)* )
            where
                $(
                    $Ts: FromRequest,
                )*
            {
                fn from_request(req: &Request) -> Self {
                    (
                        $(
                            $Ts::from_request(req),
                        )*
                    )
                }
            }
        };
    }
    f!(T1);
    f!(T1, T2);
    f!(T1, T2, T3);
    f!(T1, T2, T3, T4);
    f!(T1, T2, T3, T4, T5);
    f!(T1, T2, T3, T4, T5, T6);
    f!(T1, T2, T3, T4, T5, T6, T7);
    f!(T1, T2, T3, T4, T5, T6, T7, T8);
    f!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
    f!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
}
