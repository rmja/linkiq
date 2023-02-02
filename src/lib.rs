#![cfg_attr(not(test), no_std)]
#![feature(generators)]
#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]
#![feature(const_trait_impl)]
#![feature(let_chains)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
extern crate num_derive;

mod bitreader;
pub mod fec;
mod phycodedheader;
mod phyinterleaver;
pub mod stack;
pub mod wmbus;

#[cfg(feature = "ctrl")]
pub mod ctrl;
