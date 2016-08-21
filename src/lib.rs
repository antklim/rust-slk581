//! The crate provides function to encode data into [SLK581](http://registry.aristotlemetadata.com/share/349510/79)
//! format.
//!
//! The format allows encode given family name, given name, date of birth and sex into sequence
//! `XXXZZDDMMYYYYN`.
//! Where `XXX` encodes family name, `ZZ` encodes given name, `DDMMYYYY` encodes date of birth and
//! `N` encodes sex.

extern crate chrono;

use chrono::NaiveDate;
use chrono::format::ParseResult;
use std::error::Error;
use std::fmt;

use self::SLK581Error::{InvalidDateOfBirth, UnknownDateOfBirth, UnsupportedSex};

/// Placeholder for unknown family name `999`
pub const UNKNOWN_FAMILY_NAME: &'static str = "999";
/// Placeholder for unknown given name `99`
pub const UNKNOWN_GIVEN_NAME: &'static str = "99";
/// Placeholder for missing character in given or family name `2`
pub const UNKNOWN_CHARACTER_IN_NAME: char = '2';
/// Male code `1`
pub const MALE: &'static str = "1";
/// Female code `2`
pub const FEMALE: &'static str = "2";
/// Transgender code `3`
pub const TRANSGENDER: &'static str = "3";
/// Placeholder for unknown sex `3`
pub const UNKNOWN_SEX: &'static str = "3";
/// Supported input format of date of birth `YYYY-MM-DD`
pub const INPUT_DATE_FORMAT: &'static str = "%Y-%m-%d";
/// Output format of date of birth `DDMMYYYY`
pub const OUTPUT_DATE_FORMAT: &'static str = "%d%m%Y";

#[derive(PartialEq, Debug)]
pub enum SLK581Error<'a> {
    InvalidDateOfBirth,
    UnknownDateOfBirth,
    UnsupportedSex(&'a str),
}

impl<'a> fmt::Display for SLK581Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UnsupportedSex(ref sex) => write!(f, "{}: '{}'", self.description(), sex),
            _ => write!(f, "{}", self.description()),
        }
    }
}

impl<'a> Error for SLK581Error<'a> {
    fn description(&self) -> &str {
        match *self {
            InvalidDateOfBirth => "Unsupported date of birth format.",
            UnknownDateOfBirth => "Unknown date of birth.",
            UnsupportedSex(..) => "Unsupported sex",
        }
    }
}

fn sanitize_name(name: &str) -> String {
    let mut buf: String = String::with_capacity(name.len());

    for c in name.chars() {
        match c {
            'A' ... 'Z' => buf.push(c),
            _ => ()
        }
    }

    return buf;
}

fn encode_name(name: Option<&str>, take: usize, vec_pos: Vec<usize>) -> String {
    let clean_name = sanitize_name(name.unwrap().to_uppercase().as_str());
    let mut chars_iter = clean_name.chars().into_iter().take(take);
    let buf_capacity: usize = vec_pos.len();

    return vec_pos.into_iter()
        .map(|position| {
            if let Some(c) = chars_iter.nth(position) {
                return c;
            } else {
                return UNKNOWN_CHARACTER_IN_NAME;
            }
        })
        .fold(String::with_capacity(buf_capacity), |mut buf, c| {
            buf.push(c);
            buf
        });
}

fn encode_family_name(family_name: Option<&str>) -> String {
    if family_name.is_none() {
        return String::from(UNKNOWN_FAMILY_NAME);
    }

    encode_name(family_name, 5, vec![1, 0, 1])
}

fn encode_given_name(given_name: Option<&str>) -> String {
    if given_name.is_none() {
        return String::from(UNKNOWN_GIVEN_NAME);
    }

    encode_name(given_name, 3, vec![1, 0])
}

fn encode_date_of_birth<'a>(date_of_birth: Option<&str>) -> Result<String, SLK581Error<'a>> {
    if date_of_birth.is_none() {
        return Err(UnknownDateOfBirth);
    }

    let _date_of_birth: ParseResult<NaiveDate> =
        NaiveDate::parse_from_str(date_of_birth.unwrap(), INPUT_DATE_FORMAT);

    if _date_of_birth.is_err() {
        return Err(InvalidDateOfBirth);
    }

    Ok(_date_of_birth.unwrap().format(OUTPUT_DATE_FORMAT).to_string())
}

fn encode_sex<'a>(sex: Option<&'a str>) -> Result<String, SLK581Error<'a>> {
    if sex.is_none() {
        return Ok(String::from(UNKNOWN_SEX));
    }

    let _sex = sex.unwrap();
    let lc_sex = _sex.to_lowercase();
    match lc_sex.as_str() {
        "m" | "male" => Ok(String::from(MALE)),
        "f" | "female" => Ok(String::from(FEMALE)),
        "t" | "trans" => Ok(String::from(TRANSGENDER)),
        _ => Err(UnsupportedSex(_sex))
    }
}

// XXXXXDDMMYYYYN
// 1. XXX - 2, 3, 5 characters of family_name (indexing from 1)
//    if no character found at position then replace with 2
//    if no family_name return 999
// 2. XX - 2, 3 characters of given_name (indexing from 1)
//    if no character found at position then replace with 2
//    if no given_name return 999
// 3. DDMMYYYY - date_of_birth format
//    supported formats: [YYYY-MM-DD]
// 4. N - sex, 1 - male, 2 - female, 3 - unknown or transgender
//    supported values: m, male, f, female, t, trans - caseinsensetive
/// This function encodes given family name, given name, date of birth and sex in `XXXZZDDMMYYYYN`
/// sequence.
///
/// # Errors
///
/// Returns `UnknownDateOfBirth` when date of birth not provided:
///
/// ```
/// use slk581::encode;
/// use slk581::SLK581Error;
/// use slk581::SLK581Error::UnknownDateOfBirth;
///
/// let encoded_result: Result<String, SLK581Error> = encode(None, None, None, None);
/// assert_eq!(encoded_result.is_err(), true);
/// assert_eq!(encoded_result.unwrap_err(), UnknownDateOfBirth);
/// ```
///
/// Returns `InvalidDateOfBirth` when date of birth provided in invalid format:
///
/// ```
/// use slk581::encode;
/// use slk581::SLK581Error;
/// use slk581::SLK581Error::InvalidDateOfBirth;
///
/// let date_of_birth: Option<&str> = Some("20001219");
/// let encoded_result: Result<String, SLK581Error> = encode(None, None, date_of_birth, None);
/// assert_eq!(encoded_result.is_err(), true);
/// assert_eq!(encoded_result.unwrap_err(), InvalidDateOfBirth);
/// ```
///
/// Returns `UnsupportedSex` when unsupported sex value provided:
///
/// ```
/// use slk581::encode;
/// use slk581::SLK581Error;
/// use slk581::SLK581Error::UnsupportedSex;
///
/// let date_of_birth: Option<&str> = Some("2000-12-19");
/// let sex: Option<&str> = Some("test");
/// let encoded_result: Result<String, SLK581Error> = encode(None, None, date_of_birth, sex);
/// assert_eq!(encoded_result.is_err(), true);
/// assert_eq!(encoded_result.unwrap_err(), UnsupportedSex("test"));
/// ```
///
/// # Examples
/// ```
/// use slk581::encode;
/// use slk581::SLK581Error;
///
/// let date_of_birth: Option<&str> = Some("2000-12-19");
///
/// let encoded_result: Result<String, SLK581Error> =
///     encode(Some("Doe"), Some("John"), date_of_birth, Some("m"));
/// assert_eq!(encoded_result.is_ok(), true);
/// assert_eq!(encoded_result.unwrap(), "OE2OH191220001");
///
/// let encoded_result: Result<String, SLK581Error> =
///     encode(Some("Smith"), Some("Jane"), date_of_birth, Some("f"));
/// assert_eq!(encoded_result.is_ok(), true);
/// assert_eq!(encoded_result.unwrap(), "MIHAN191220002");
///
/// let encoded_result: Result<String, SLK581Error> =
///     encode(Some("O Bare"), Some("Foo"), date_of_birth, Some("t"));
/// assert_eq!(encoded_result.is_ok(), true);
/// assert_eq!(encoded_result.unwrap(), "BAEOO191220003");
/// ```
pub fn encode<'a>(family_name: Option<&str>,
                  given_name: Option<&str>,
                  date_of_birth: Option<&str>,
                  sex: Option<&'a str>) -> Result<String, SLK581Error<'a>> {

    let encoded_family_name: String = encode_family_name(family_name);
    let encoded_given_name: String = encode_given_name(given_name);
    let encoded_date_of_birth: String = try!(encode_date_of_birth(date_of_birth));
    let encoded_sex: String = try!(encode_sex(sex));

    let mut buf = String::with_capacity(14);
    buf.push_str(encoded_family_name.as_str());
    buf.push_str(encoded_given_name.as_str());
    buf.push_str(encoded_date_of_birth.as_str());
    buf.push_str(encoded_sex.as_str());

    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::encode;
    use super::SLK581Error;
    use super::SLK581Error::*;

    #[test]
    fn it_should_return_error_for_unknown_dob() {
        let encoded_result: Result<String, SLK581Error> = encode(None, None, None, None);
        assert_eq!(encoded_result.is_err(), true);
        assert_eq!(encoded_result.unwrap_err(), UnknownDateOfBirth);
    }

    #[test]
    fn it_should_return_error_for_invalid_dob() {
        let date_of_birth: Option<&str> = Some("20001219");
        let encoded_result: Result<String, SLK581Error> = encode(None, None, date_of_birth, None);
        assert_eq!(encoded_result.is_err(), true);
        assert_eq!(encoded_result.unwrap_err(), InvalidDateOfBirth);
    }

    #[test]
    fn it_should_encode_dob() {
        let date_of_birth: Option<&str> = Some("2000-12-19");
        let encoded_result: Result<String, SLK581Error> = encode(None, None, date_of_birth, None);
        assert_eq!(encoded_result.is_ok(), true);
        assert_eq!(encoded_result.unwrap(), "99999191220003");
    }

    #[test]
    fn it_should_return_error_for_unsupported_sex() {
        let date_of_birth: Option<&str> = Some("2000-12-19");
        let sex: Option<&str> = Some("test");
        let encoded_result: Result<String, SLK581Error> = encode(None, None, date_of_birth, sex);
        assert_eq!(encoded_result.is_err(), true);
        assert_eq!(encoded_result.unwrap_err(), UnsupportedSex("test"));
    }

    #[test]
    fn it_should_encode_male() {
        let date_of_birth: Option<&str> = Some("2000-12-19");
        let vec_sex: Vec<Option<&str>> = vec![Some("m"), Some("M"), Some("MaLe")];

        for sex in vec_sex {
            let encoded_result: Result<String, SLK581Error> =
                encode(None, None, date_of_birth, sex);
            assert_eq!(encoded_result.is_ok(), true);
            assert_eq!(encoded_result.unwrap(), "99999191220001");
        }
    }

    #[test]
    fn it_should_encode_female() {
        let date_of_birth: Option<&str> = Some("2000-12-19");
        let vec_sex: Vec<Option<&str>> = vec![Some("f"), Some("F"), Some("feMaLe")];

        for sex in vec_sex {
            let encoded_result: Result<String, SLK581Error> =
                encode(None, None, date_of_birth, sex);
            assert_eq!(encoded_result.is_ok(), true);
            assert_eq!(encoded_result.unwrap(), "99999191220002");
        }
    }

    #[test]
    fn it_should_encode_transgender() {
        let date_of_birth: Option<&str> = Some("2000-12-19");
        let vec_sex: Vec<Option<&str>> = vec![Some("t"), Some("T"), Some("trAnS")];

        for sex in vec_sex {
            let encoded_result: Result<String, SLK581Error> =
                encode(None, None, date_of_birth, sex);
            assert_eq!(encoded_result.is_ok(), true);
            assert_eq!(encoded_result.unwrap(), "99999191220003");
        }
    }

    #[test]
    fn it_should_encode_short_family_name() {
        let date_of_birth: Option<&str> = Some("2000-12-19");
        let sex: Option<&str> = Some("male");

        let encoded_result: Result<String, SLK581Error> =
            encode(Some("Y"), None, date_of_birth, sex);
        assert_eq!(encoded_result.is_ok(), true);
        assert_eq!(encoded_result.unwrap(), "22299191220001");

        let encoded_result: Result<String, SLK581Error> =
            encode(Some("Yo"), None, date_of_birth, sex);
        assert_eq!(encoded_result.is_ok(), true);
        assert_eq!(encoded_result.unwrap(), "O2299191220001");

        let encoded_result: Result<String, SLK581Error> =
            encode(Some("O-B"), None, date_of_birth, sex);
        assert_eq!(encoded_result.is_ok(), true);
        assert_eq!(encoded_result.unwrap(), "B2299191220001");

        let encoded_result: Result<String, SLK581Error> =
            encode(Some("O'Ber"), None, date_of_birth, sex);
        assert_eq!(encoded_result.is_ok(), true);
        assert_eq!(encoded_result.unwrap(), "BE299191220001");
    }

    #[test]
    fn it_should_encode_short_given_name() {
        let date_of_birth: Option<&str> = Some("2000-12-19");
        let sex: Option<&str> = Some("male");

        let encoded_result: Result<String, SLK581Error> =
            encode(None, Some("Y"), date_of_birth, sex);
        assert_eq!(encoded_result.is_ok(), true);
        assert_eq!(encoded_result.unwrap(), "99922191220001");

        let encoded_result: Result<String, SLK581Error> =
            encode(None, Some("Yo"), date_of_birth, sex);
        assert_eq!(encoded_result.is_ok(), true);
        assert_eq!(encoded_result.unwrap(), "999O2191220001");

        let encoded_result: Result<String, SLK581Error> =
            encode(None, Some("O-B"), date_of_birth, sex);
        assert_eq!(encoded_result.is_ok(), true);
        assert_eq!(encoded_result.unwrap(), "999B2191220001");
    }

    #[test]
    fn it_should_encode_happy_path() {
        let date_of_birth: Option<&str> = Some("2000-12-19");

        let encoded_result: Result<String, SLK581Error> =
            encode(Some("Doe"), Some("John"), date_of_birth, Some("m"));
        assert_eq!(encoded_result.is_ok(), true);
        assert_eq!(encoded_result.unwrap(), "OE2OH191220001");

        let encoded_result: Result<String, SLK581Error> =
            encode(Some("Smith"), Some("Jane"), date_of_birth, Some("f"));
        assert_eq!(encoded_result.is_ok(), true);
        assert_eq!(encoded_result.unwrap(), "MIHAN191220002");

        let encoded_result: Result<String, SLK581Error> =
            encode(Some("O Bare"), Some("Foo"), date_of_birth, Some("t"));
        assert_eq!(encoded_result.is_ok(), true);
        assert_eq!(encoded_result.unwrap(), "BAEOO191220003");
    }
}
