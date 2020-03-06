#![cfg_attr(not(feature = "std"), no_std)]

pub mod class;

#[cfg(feature = "std")]
pub mod host;
