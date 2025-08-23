use std::error::Error;

use colorize::AnsiColor;

use crate::dot::get_dots_db;

pub fn on_list() -> Result<(), Box<dyn Error>> {
    let db = get_dots_db()?;

    for dot in db {
        let output = format!("{} {}", dot.name.bold(), dot.original_path.display());
        println!("{output}");
    }

    Ok(())
}
