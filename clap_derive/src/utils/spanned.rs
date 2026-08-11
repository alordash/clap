use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use std::ops::{Deref, DerefMut};
use syn::LitStr;
#[cfg_attr(test, rsubstitute::mock)]
/// An entity with a span attached.
#[derive(Debug, Clone)]
pub(crate) struct Sp<T> {
    val: T,
    span: Span,
}
#[cfg_attr(test, rsubstitute::mock)]
impl<T> Sp<T> {
    pub(crate) fn new(val: T, span: Span) -> Self {
        Sp { val, span }
    }
}
#[cfg_attr(test, rsubstitute::mock(base))]
impl<T> Sp<T> {
    pub(crate) fn get(&self) -> &T {
        &self.val
    }
    pub(crate) fn span(&self) -> Span {
        self.span
    }
}
#[cfg_attr(test, rsubstitute::mock(base))]
impl<T> Deref for Sp<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.val
    }
}
#[cfg_attr(test, rsubstitute::mock(base))]
impl<T> DerefMut for Sp<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.val
    }
}
impl From<Ident> for Sp<String> {
    fn from(ident: Ident) -> Self {
        Sp::new(ident.to_string(), ident.span())
    }
}
impl From<LitStr> for Sp<String> {
    fn from(lit: LitStr) -> Self {
        Sp::new(lit.value(), lit.span())
    }
}
#[cfg_attr(test, rsubstitute::mock(base))]
impl<'a> From<Sp<&'a str>> for Sp<String> {
    fn from(sp: Sp<&'a str>) -> Self {
        Sp::new(sp.val.into(), sp.span)
    }
}
#[cfg_attr(test, rsubstitute::mock(base))]
impl<U, T: PartialEq<U>> PartialEq<U> for Sp<T> {
    fn eq(&self, other: &U) -> bool {
        self.val == *other
    }
}
#[cfg_attr(test, rsubstitute::mock(base))]
impl<T: AsRef<str>> AsRef<str> for Sp<T> {
    fn as_ref(&self) -> &str {
        self.val.as_ref()
    }
}
#[cfg_attr(test, rsubstitute::mock(base))]
impl<T: ToTokens> ToTokens for Sp<T> {
    fn to_tokens(&self, stream: &mut TokenStream) {
        let tt = self.val.to_token_stream().into_iter().map(|mut tt| {
            tt.set_span(self.span);
            tt
        });
        stream.extend(tt);
    }
}
