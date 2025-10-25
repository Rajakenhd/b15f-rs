#![deny(missing_docs,
    missing_debug_implementations,
    trivial_casts,
    unstable_features,
    unused_import_braces, unused_qualifications)]

//! Driver code for the board 15
//! 
//! It aims to be syntactically similar to the [`original`] implementation.
//! The original repository is no longer maintained, and this library doesn't maintain it either,
//! it simply provides the same functionality in another language.
//! 
//! [`original`]: https://github.com/devfix/b15f

pub mod b15f;
mod usart;
mod request;
mod error;
mod assert;

pub use crate::b15f::B15F;