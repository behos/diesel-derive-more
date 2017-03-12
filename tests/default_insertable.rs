#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate models_derive;
#[macro_use] extern crate serde_derive;
extern crate dotenv;
extern crate serde;
extern crate serde_json;

use diesel::prelude::*;
use diesel::insert;
use diesel::pg::PgConnection;
use dotenv::dotenv;


infer_schema!("dotenv:DATABASE_URL");

#[derive(Queryable, DefaultInsertable)]
#[table_name="test_table"]
pub struct TestTable {
    #[auto_increment] pub id: i32,
    pub num: String,
}

#[test]
fn does_not_have_auto_increment_fields() {
    let default_insertable = NewTestTable {
        num: String::from("example")
    };

    assert_eq!(default_insertable.num, "example")
}


#[test]
fn can_be_deserialized() {
    let json = r#"{"num": "example"}"#;
    let from_json: NewTestTable = serde_json::from_str(json).unwrap();
    assert_eq!(from_json.num, "example")
}


#[test]
fn can_be_inserted() {
    dotenv().ok();
    let pg_url = "postgres://models_derive_test:password@127.0.0.1/models_derive_test";
    let connection = PgConnection::establish(pg_url).unwrap();
    let default_insertable = NewTestTable {
        num: String::from("example")
    };

    let res: TestTable = insert(&default_insertable)
        .into(test_table::table).get_result(&connection).unwrap();
    assert_eq!(res.num, default_insertable.num)
}
