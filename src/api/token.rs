use anyhow::{bail, Result};
use keyring::Entry;
use rpassword::prompt_password;

pub fn set_token() -> Result<()> {
    let token = prompt_password("Your API Token: ")?;
    if token.trim().is_empty() {
        bail!("No token entered, configuration aborted.");
    }

    let entry = Entry::new("linear-cli", "api_token")?;
    entry.set_password(&token)?;
    println!("Token stored successfully.");
    Ok(())
}

pub fn get_token() -> Result<String> {
    let entry = Entry::new("linear-cli", "api_token")?;
    entry.get_password().map_err(Into::into)
}
