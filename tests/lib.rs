extern crate slk581;

use slk581::encode;
use slk581::SLK581Error;
use slk581::SLK581Error::*;

#[test]
fn slk581_should_return_error_for_unknown_dob() {
    let encoded_result: Result<String, SLK581Error> = encode(None, None, None, None);
    assert_eq!(encoded_result.is_err(), true);
    assert_eq!(encoded_result.unwrap_err(), UnknownDateOfBirth);
}

#[test]
fn slk581_should_return_error_for_invalid_dob() {
    let date_of_birth: Option<&str> = Some("20001219");
    let encoded_result: Result<String, SLK581Error> = encode(None, None, date_of_birth, None);
    assert_eq!(encoded_result.is_err(), true);
    assert_eq!(encoded_result.unwrap_err(), InvalidDateOfBirth);
}

#[test]
fn slk581_should_encode_dob() {
    let date_of_birth: Option<&str> = Some("2000-12-19");
    let encoded_result: Result<String, SLK581Error> = encode(None, None, date_of_birth, None);
    assert_eq!(encoded_result.is_ok(), true);
    assert_eq!(encoded_result.unwrap(), "99999191220003");
}

#[test]
fn slk581_should_return_error_for_unsupported_sex() {
    let date_of_birth: Option<&str> = Some("2000-12-19");
    let sex: Option<&str> = Some("test");
    let encoded_result: Result<String, SLK581Error> = encode(None, None, date_of_birth, sex);
    assert_eq!(encoded_result.is_err(), true);
    assert_eq!(encoded_result.unwrap_err(), UnsupportedSex("test"));
}

#[test]
fn slk581_should_encode_male() {
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
fn slk581_should_encode_female() {
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
fn slk581_should_encode_transgender() {
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
fn slk581_should_encode_short_family_name() {
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
fn slk581_should_encode_short_given_name() {
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
fn slk581_should_encode_happy_path() {
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
