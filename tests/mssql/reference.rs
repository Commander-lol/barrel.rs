#![allow(unused_imports)]

use barrel::backend::{MsSql, SqlGenerator};
use barrel::{types, Migration, Table};

#[test]
fn in_schema() {
    let sql = MsSql::add_column(
        false,
        Some("schema"),
        "author",
        &types::foreign("users", "id"),
    );

    assert_eq!(
        sql,
        "[author] INT REFERENCES [schema].[users]([id]) NOT NULL"
    );
}

#[test]
fn ext_schema() {
    let sql = MsSql::add_column(
        false,
        Some("schema"),
        "author",
        &types::foreign_schema("other_schema", "users", "id"),
    );

    assert_eq!(
        sql,
        "[author] INT REFERENCES [other_schema].[users]([id]) NOT NULL"
    );
}
