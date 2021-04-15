mod utils;
use utils::{ui, URLOpts, config};

use mysql::Conn;

fn main() -> Result<(), std::io::Error>{
    let prompt = ui::bool(&"Do you want to use the MySQL Downloader?")?;
    let config = config::get();
    if prompt {
        let conn;
        match config {
            Some(config) => {
                let use_config = ui::bool(&"Do you want to use the stored connnection data")?;
                if use_config {
                    conn = config;
                } else {
                    conn = get_opts_from_input()?;
                }
            }
            None => {
                conn = get_opts_from_input()?;
            }
        };
        let conn = Conn::new(utils::build_url(conn)?);
        conn.unwrap().ping();
        println!("Succesfully connected!");
    } else {
        println!("no")
    };
    Ok(())
}

fn get_opts_from_input() -> Result<URLOpts, std::io::Error>{
    let opts = URLOpts {
        user: ui::input("Enter your username")?, 
        password: ui::pass("Enter your password")?, 
        db: ui::input("Enter your database")?, 
        host: Option::from(ui::input_with_default("Enter your connection host ( eg: localhost )", "localhost")?)
    };
    let store = ui::bool("Do you want to store the connection info? (Password is stored in plain-text without any encryption)")?;
    if store {
        config::store(&opts).unwrap();
    }
    Ok(opts)
}
