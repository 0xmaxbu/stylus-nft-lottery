#![cfg_attr(not(any(feature = "export-abi", feature = "test")), no_main)]
extern crate alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use stylus_sdk::{msg, prelude::*};

mod erc721;
use crate::erc721::{Erc721, Erc721Params};

struct StylusNFTParams;
impl Erc721Params for StylusNFTParams {
    const NAME: &'static str = "MyNFT";
    const SYMBOL: &'static str = "SNFT";
}

#[external]
#[inherit(Erc721<StylusNFTParams>)]
impl StylusNFT {
    
}