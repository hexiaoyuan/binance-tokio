mod account;
mod general;
mod market;
mod user_data;
mod withdraw;

/// https://api.binance.com https://api1.binance.com https://api2.binance.com https://api3.binance.com
pub const BINANCE_US_URL: &'static str = "https://api.binance.us";

pub use self::account::AccountClient;
pub use self::market::MarketDataClient;
pub use self::general::GeneralClient;
pub use self::user_data::UserDataClient;
pub use self::withdraw::WithdrawalClient;