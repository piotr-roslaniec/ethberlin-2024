use std::fs::File;
use std::io::prelude::*;
use std::io::Read;

pub fn write_user_data(h: &[u8]) -> std::io::Result<()> {
    let mut file = File::create("/dev/attestation/user_report_data")?;
    file.write_all(h)?;
    Ok(())
}

pub fn read_quote() -> std::io::Result<Vec<u8>> {
    let mut file = File::open("/dev/attestation/quote")?;
    let mut quote = Vec::new();
    file.read_to_end(&mut quote)?;
    Ok(quote)
}
