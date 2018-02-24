#[macro_use]
extern crate diesel_derive_more;
extern crate diesel;

use std::error::Error;

use diesel::types::FromSql;
use diesel::sql_types::Text;
use diesel::pg::Pg;


#[derive(Debug, PartialEq, DBEnum)]
enum TestEnum {
    One,
    Two,
    Three,
}


#[test]
fn macro_implements_from_sql() {
    assert_eq!(from_sql(b"One").unwrap(), TestEnum::One);
    assert_eq!(from_sql(b"Two").unwrap(), TestEnum::Two);
    assert_eq!(from_sql(b"Three").unwrap(), TestEnum::Three);
}

fn from_sql(value: &[u8]) -> Result<TestEnum, Box<Error + Send + Sync>> {
    <TestEnum as FromSql<Text, Pg>>::from_sql(Some(value))
}

#[test]
fn returns_error_if_not_in_enum() {
    let result = from_sql(b"Some value");
    match result {
        Err(string) => assert_eq!(string.description(), "Unexpected value"),
        Ok(_) => assert!(false),
    }
}


#[test]
fn returns_error_if_cannot_parse_string() {
    let invalid: [u8; 5] = [128; 5];
    let result = from_sql(&invalid);
    match result {
        Err(string) => assert_eq!(string.description(), "Could not load string"),
        Ok(_) => assert!(false),
    }
}


#[test]
fn returns_error_if_no_value_provided() {
    let result = <TestEnum as FromSql<Text, Pg>>::from_sql(None);
    match result {
        Err(string) => assert_eq!(string.description(), "Value not provided"),
        Ok(_) => assert!(false),
    }
}
