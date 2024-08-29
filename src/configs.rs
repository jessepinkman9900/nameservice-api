use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Serialize, Deserialize, Clone, EnumString, EnumIter)]
pub enum Chain {
    #[strum(serialize = "ethereum")]
    Ethereum,
    #[strum(serialize = "base")]
    Base,
    #[strum(serialize = "canto")]
    Canto,
}

#[derive(Serialize, Deserialize, Debug, EnumIter, Display)]
pub enum ChainNameService {
    BaseNameService,
    EthNameService,
    CantoNameService,
}

#[derive(Debug, EnumIter, Display)]
pub enum Operation {
    NameAvailable,
}
