use std::fs;
use toml;
use super::URLOpts;

pub fn get() -> Option<URLOpts> {
    let foo = fs::read_to_string("sql-sync-conf.toml");
    match foo {
        Ok(foo) => {
            let foo: URLOpts = toml::from_str(&foo).unwrap();
            Some(foo)
        }
        _ => {
        Option::None
        }
    }
}

pub fn store(opts: &URLOpts) -> Result<(), toml::ser::Error> {
    let string = toml::to_string(opts)?;
    fs::write("sql-sync-conf.toml", string).unwrap();
    Ok(())
}