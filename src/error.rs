
#[derive(Debug)]
#[repr(u8)]
pub enum Error {
    /// Already initialised!
    AlreadyInitialised,
    /// Error transferring ERC721
    ERC721Failed(Vec<u8>),
    /// Error using Chainlink's !
    ChainlinkError(Vec<u8>),
    /// Someone tried to borrow below the utilisation rate.
    BadBorrowAttempt,
    /// Liquidation wasn't possible, the utilisation is underutilised.
    NotAbleToLiquidate,
    /// Not the owner of this position!
    NotOwner,
    /// The ctor from the factory failed!
    CtorFailed(Vec<u8>),
    /// Deployment failed from the factory!
    DeployFailure(Vec<u8>),
    /// Error unpacking the second word from Chainlink!
    ChainlinkUnpacking,
}

impl From<Error> for Vec<u8> {
    fn from(v: Error) -> Self {
        // Pack a return value of Error(uint256), with Error being 00000004.
        let mut r: [u8; 32 + 4] = [0u8; 32 + 4];
        r[3] = AUSD_ERR;
        match v {
            Error::ERC20Failed(v)
            | Error::ChainlinkError(v)
            | Error::CtorFailed(v)
            | Error::DeployFailure(v) => r[36 - v.len()..].copy_from_slice(&v),
            v => r[35] = unsafe { *<*const _>::from(&v).cast::<u8>() },
        };
        r.to_vec()
    }
}