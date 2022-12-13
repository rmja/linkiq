#![cfg_attr(not(test), no_std)]
#![feature(maybe_uninit_write_slice)]

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