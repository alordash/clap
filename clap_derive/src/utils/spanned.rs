use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use syn::LitStr;
use std::ops::{Deref, DerefMut};
#[rsubstitute::mock]
/// An entity with a span attached.
#[derive(Debug, Copy, Clone)]
pub(crate) struct Sp<T> {
    val: T,
    span: Span,
}
#[rsubstitute::mock(base)]
impl<T> Sp<T> {
    pub(crate) fn new(val: T, span: Span) -> Self {
        Sp { val, span }
    }
    pub(crate) fn get(&self) -> &T {
        &self.val
    }
    pub(crate) fn span(&self) -> Span {
        self.span
    }
}
#[rsubstitute::mock(base)]
impl<T> Deref for Sp<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.val
    }
}
#[rsubstitute::mock(base)]
impl<T> DerefMut for Sp<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.val
    }
}
#[rsubstitute::mock(base)]
impl From<Ident> for Sp<String> {
    fn from(ident: Ident) -> Self {
        Sp {
            val: ident.to_string(),
            span: ident.span(),
        }
    }
}
#[rsubstitute::mock(base)]
impl From<LitStr> for Sp<String> {
    fn from(lit: LitStr) -> Self {
        Sp {
            val: lit.value(),
            span: lit.span(),
        }
    }
}
#[rsubstitute::mock(base)]
impl<'a> From<Sp<&'a str>> for Sp<String> {
    fn from(sp: Sp<&'a str>) -> Self {
        Sp::new(sp.val.into(), sp.span)
    }
}
#[rsubstitute::mock(base)]
impl<U, T: PartialEq<U>> PartialEq<U> for Sp<T> {
    fn eq(&self, other: &U) -> bool {
        self.val == *other
    }
}
#[rsubstitute::mock(base)]
impl<T: AsRef<str>> AsRef<str> for Sp<T> {
    fn as_ref(&self) -> &str {
        self.val.as_ref()
    }
}
#[rsubstitute::mock(base)]
impl<T: ToTokens> ToTokens for Sp<T> {
    fn to_tokens(&self, stream: &mut TokenStream) {
        let tt = self
            .val
            .to_token_stream()
            .into_iter()
            .map(|mut tt| {
                tt.set_span(self.span);
                tt
            });
        stream.extend(tt);
    }
}
