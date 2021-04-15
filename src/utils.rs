use mysql::{OptsBuilder};
use serde_derive::{Deserialize, Serialize};

pub mod ux;
pub use ux as ui;

pub mod config;

#[derive(Deserialize,Debug,Serialize)]
pub struct URLOpts {
    pub user: String,
    pub db: String,
    pub password: String,
    pub host: Option<String>,
}


pub fn build_url(opts: URLOpts) -> Result<OptsBuilder, std::io::Error> {
    let builder = OptsBuilder::new()
        .user(Some(opts.user))
        .db_name(Some(opts.db))
        .pass(Some(opts.password))
        .ip_or_hostname(opts.host);
    Ok(builder)
}