use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref TEL_REGEX: Regex = Regex::new("^+1[0-9]{10}$").unwrap();
}

#[derive(Debug)]
pub struct Tel(pub(crate) String);

impl TryFrom<String> for Tel {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Some(_) = TEL_REGEX.find(value.as_str()) {
            return Ok(Tel(value));
        }
        Err(())
    }
}
