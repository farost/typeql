/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub mod date_time;
pub mod error;
pub(crate) mod identifier;
pub mod string;
pub mod token;

pub use error::Error;

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct LineColumn {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Span {
    pub begin: LineColumn,
    pub end: LineColumn,
}

pub trait Spanned {
    fn span(&self) -> Option<Span>;
}
