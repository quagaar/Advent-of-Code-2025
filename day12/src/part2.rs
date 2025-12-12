use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {}

pub fn solve(_input: &str) -> Result<&'static str, Error> {
    Ok("Merry Christmas!")
}
