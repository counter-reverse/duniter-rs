extern crate regex;

use self::regex::Regex;

lazy_static! {
    #[derive(Debug)]
    pub static ref WS2P_V1_ENDPOINT_REGEX: Regex = Regex::new(
        "^WS2P (?P<version>[1-9][0-9]* )?(?P<uuid>[a-f0-9]{6,8}) (?P<host>[a-z_][a-z0-9-_.]*|[0-9.]+|[0-9a-f:]+) (?P<port>[0-9]+)(?: /?(?P<path>.+)?)? *$"
    ).unwrap();
}
pub static WS2P_OUTCOMING_INTERVAL_AT_STARTUP: &'static u64 = &75;
pub static WS2P_OUTCOMING_INTERVAL: &'static u64 = &300;
pub static WS2P_DEFAULT_OUTCOMING_QUOTA: &'static usize = &10;
pub static WS2P_NEGOTIATION_TIMEOUT: &'static u64 = &15;
//pub static WS2P_REQUEST_TIMEOUT : &'static u64 = &30;
pub static WS2P_EXPIRE_TIMEOUT: &'static u64 = &120;
pub static WS2P_SPAM_INTERVAL_IN_MILLI_SECS: &'static u64 = &80;
pub static WS2P_SPAM_LIMIT: &'static usize = &6;
pub static WS2P_SPAM_SLEEP_TIME_IN_SEC: &'static u64 = &100;
pub static DURATION_BEFORE_RECORDING_ENDPOINT: &'static u64 = &180;
pub static BLOCKS_REQUEST_INTERVAL: &'static u64 = &60;
pub static PENDING_IDENTITIES_REQUEST_INTERVAL: &'static u64 = &40;
