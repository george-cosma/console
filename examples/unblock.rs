use std::error::Error;

use console::Term;

fn main() -> Result<(), Box<dyn Error>> {
    let term = Term::stdout();
    term.unblock()?;
    let key = term.read_key()?;
    dbg!(key);

    Ok(())
}
