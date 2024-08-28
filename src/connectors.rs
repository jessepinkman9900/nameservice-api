pub mod base;
use strum_macros::{Display, EnumIter};

#[derive(Debug,EnumIter,Display)]
pub enum NameServices {
    BaseNameService,
    EthNameService,
    CantoNameService,
}
