#![deny(
    anonymous_parameters,
    clippy::all,
    const_err,
    illegal_floating_point_literal_pattern,
    late_bound_lifetime_arguments,
    path_statements,
    patterns_in_fns_without_body,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unused_extern_crates
)]
#![warn(
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::get_unwrap,
    clippy::nursery,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::todo,
    clippy::unimplemented,
    clippy::unwrap_used,
    clippy::use_debug,
    single_use_lifetimes,
    unused_qualifications,
    variant_size_differences
)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::missing_const_for_fn,
    clippy::redundant_pub_crate
)]

mod date;
mod datetime;
mod error;
mod helpers;
mod offset;
mod peeking_take_while;
mod time;

use date::Date;
use datetime::DateTime;
use error::Error;
use offset::Offset;
use proc_macro::TokenStream;
use time::Time;

trait ToTokens {
    fn to_internal_tokens(&self, tokens: &mut TokenStream);
    fn to_external_tokens(&self, tokens: &mut TokenStream) {
        self.to_internal_tokens(tokens);
    }
    fn to_internal_token_stream(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        self.to_internal_tokens(&mut tokens);
        tokens
    }
    fn to_external_token_stream(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        self.to_external_tokens(&mut tokens);
        tokens
    }
}

macro_rules! impl_macros {
    ($($name:ident : $type:ty)*) => {$(
        #[allow(clippy::unimplemented)] // macro-generated
        #[proc_macro]
        pub fn $name(input: TokenStream) -> TokenStream {
            let string = match helpers::get_string_literal(input) {
                Ok(string) => string,
                Err(err) => return err.to_compile_error(),
            };
            let chars = &mut string.chars().peekable();

            let value = match <$type>::parse(chars) {
                Ok(value) => value,
                Err(err) => return err.to_compile_error(),
            };

            match chars.peek() {
                Some(&char) => Error::UnexpectedCharacter(char).to_compile_error(),
                None => value.to_external_token_stream(),
            }
        }
    )*};
}

impl_macros! {
    date: Date
    datetime: DateTime
    offset: Offset
    time: Time
}
