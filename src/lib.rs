#![cfg_attr(not(test), no_std)]
#![allow(async_fn_in_trait)]
#![allow(incomplete_features)]
#![feature(const_trait_impl)]
#![feature(coroutines)]
#![feature(let_chains)]

extern crate alloc;

#[macro_use]
extern crate num_derive;

mod bitreader;
pub mod fec;
pub mod interleaver;
mod phycodedheader;
pub mod stack;

#[cfg(feature = "ctrl")]
pub mod ctrl;
