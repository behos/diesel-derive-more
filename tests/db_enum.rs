#[macro_use]
extern crate models_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;


use std::error::Error;
use std::env;

use diesel::insert_into;
use diesel::prelude::*;
use diesel::types::{FromSql, Text};
use diesel::pg::{Pg, PgConnection};

use dotenv::dotenv;

infer_schema!("dotenv:DATABASE_URL");

#[derive(Debug, PartialEq, DBEnum)]
pub enum TestEnum {
    One,
    Two,
    Three,
}

#[derive(Queryable)]
pub struct TestTable {
    pub id: i32,
    pub num: TestEnum,
}


#[derive(Insertable)]
#[table_name = "test_table"]
pub struct NewTestTable {
    pub num: TestEnum,
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


#[test]
fn can_be_inserted_and_fetched_as_part_of_table() {
    dotenv().ok();
    let pg_url = env::var("DATABASE_URL").unwrap();
    let connection = PgConnection::establish(&pg_url).unwrap();
    let new = NewTestTable { num: TestEnum::One };

    let stored: TestTable = insert_into(test_table::table)
        .values(&new)
        .get_result(&connection)
        .unwrap();
    assert_eq!(new.num, stored.num)
}
