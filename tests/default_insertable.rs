#![recursion_limit="128"]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_more;

#[cfg(feature = "serialization")]
#[macro_use]
extern crate serde_derive;

extern crate dotenv;

use std::env;

use diesel::prelude::*;
use diesel::insert_into;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use diesel::sql_types::Text;


table!{
    test_table {
        id -> Integer,
        num -> Text,
        my_enum -> Text,
    }
}

#[derive(Clone, Debug, PartialEq, DBEnum, FromSqlRow, AsExpression)]
#[sql_type = "Text"]
pub enum MyEnum {
    One,
    Two,
    Three,
}

#[derive(Queryable, DefaultInsertable)]
#[table_name = "test_table"]
pub struct TestTable {
    #[auto_increment]
    pub id: i32,
    pub num: String,
    pub my_enum: MyEnum,
}

#[test]
fn does_not_have_auto_increment_fields() {
    let default_insertable = NewTestTable {
        num: String::from("example"),
        my_enum: MyEnum::One,
    };

    assert_eq!(default_insertable.num, "example")
}


#[test]
fn can_be_inserted() {
    dotenv().ok();
    let pg_url = env::var("DATABASE_URL").unwrap();
    let connection = PgConnection::establish(&pg_url).unwrap();
    let default_insertable = NewTestTable {
        num: String::from("example"),
        my_enum: MyEnum::One,
    };

    let res: TestTable = insert_into(test_table::table)
        .values(&default_insertable)
        .get_result(&connection)
        .unwrap();
    assert_eq!(res.num, default_insertable.num);
    assert_eq!(res.my_enum, default_insertable.my_enum)
}

#[cfg(feature = "serialization")]
mod serialization_tests {

    extern crate serde;
    extern crate serde_json;

    use super::NewTestTable;

    #[test]
    fn can_be_deserialized() {
        let json = r#"{"num": "example"}"#;
        let from_json: NewTestTable = serde_json::from_str(json).unwrap();
        assert_eq!(from_json.num, "example")
    }
}
