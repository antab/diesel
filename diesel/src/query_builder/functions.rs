use dsl::Select;
use expression::Expression;
use query_dsl::SelectDsl;
use super::delete_statement::DeleteStatement;
use super::insert_statement::{DefaultValues, Insert};
use super::{IncompleteInsertStatement, IncompleteUpdateStatement, IntoUpdateTarget,
            SelectStatement};

/// Creates an update statement. Helpers for updating a single row can be
/// generated by deriving [`AsChangeset`](query_builder/trait.AsChangeset.html)
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate diesel;
/// # include!("../doctest_setup.rs");
/// #
/// # table! {
/// #     users {
/// #         id -> Integer,
/// #         name -> VarChar,
/// #     }
/// # }
/// #
/// # #[cfg(feature = "postgres")]
/// # fn main() {
/// #     use self::users::dsl::*;
/// #     let connection = establish_connection();
/// let updated_row = diesel::update(users.filter(id.eq(1)))
///     .set(name.eq("James"))
///     .get_result(&connection);
/// // On backends that support it, you can call `get_result` instead of `execute`
/// // to have `RETURNING *` automatically appended to the query. Alternatively, you
/// // can explicitly return an expression by using the `returning` method before
/// // getting the result.
/// assert_eq!(Ok((1, "James".to_string())), updated_row);
/// # }
/// # #[cfg(not(feature = "postgres"))]
/// # fn main() {}
/// ```
///
/// To update multiple columns, give `set` a tuple argument:
///
/// ```rust
/// # #[macro_use] extern crate diesel;
/// # include!("../doctest_setup.rs");
/// #
/// # table! {
/// #     users {
/// #         id -> Integer,
/// #         name -> VarChar,
/// #         surname -> VarChar,
/// #     }
/// # }
/// #
/// # #[cfg(feature = "postgres")]
/// # fn main() {
/// # use users::dsl::*;
/// # let connection = establish_connection();
/// # connection.execute("DROP TABLE users").unwrap();
/// # connection.execute("CREATE TABLE users (
/// #     id SERIAL PRIMARY KEY,
/// #     name VARCHAR,
/// #     surname VARCHAR)").unwrap();
/// # connection.execute("INSERT INTO users(name, surname) VALUES('Sean', 'Griffin')").unwrap();
///
/// let updated_row = diesel::update(users.filter(id.eq(1)))
///     .set((name.eq("James"), surname.eq("Bond")))
///     .get_result(&connection);
///
/// assert_eq!(Ok((1, "James".to_string(), "Bond".to_string())), updated_row);
/// # }
/// # #[cfg(not(feature = "postgres"))]
/// # fn main() {}
/// ```
pub fn update<T: IntoUpdateTarget>(
    source: T,
) -> IncompleteUpdateStatement<T::Table, T::WhereClause> {
    IncompleteUpdateStatement::new(source.into_update_target())
}

/// Creates a delete statement. Will delete the records in the given set.
/// Because this function has a very generic name, it is not exported by
/// default.
///
/// # Examples
///
/// ### Deleting a single record:
///
/// ```rust
/// # #[macro_use] extern crate diesel;
/// # include!("../doctest_setup.rs");
/// #
/// # table! {
/// #     users {
/// #         id -> Integer,
/// #         name -> VarChar,
/// #     }
/// # }
/// #
/// # fn main() {
/// #     delete();
/// # }
/// #
/// # fn delete() -> QueryResult<()> {
/// #     use self::users::dsl::*;
/// #     let connection = establish_connection();
/// #     let get_count = || users.count().first::<i64>(&connection);
/// let old_count = get_count();
/// try!(diesel::delete(users.filter(id.eq(1))).execute(&connection));
/// assert_eq!(old_count.map(|count| count - 1), get_count());
/// # Ok(())
/// # }
/// ```
///
/// ### Deleting a whole table:
///
/// ```rust
/// # #[macro_use] extern crate diesel;
/// # include!("../doctest_setup.rs");
/// #
/// # table! {
/// #     users {
/// #         id -> Integer,
/// #         name -> VarChar,
/// #     }
/// # }
/// #
/// # fn main() {
/// #     delete();
/// # }
/// #
/// # fn delete() -> QueryResult<()> {
/// #     use self::users::dsl::*;
/// #     let connection = establish_connection();
/// #     let get_count = || users.count().first::<i64>(&connection);
/// try!(diesel::delete(users).execute(&connection));
/// assert_eq!(Ok(0), get_count());
/// # Ok(())
/// # }
/// ```
pub fn delete<T: IntoUpdateTarget>(source: T) -> DeleteStatement<T::Table, T::WhereClause> {
    let target = source.into_update_target();
    DeleteStatement::new(target.table, target.where_clause)
}

/// Creates an insert statement. Will add the given data to a table. This
/// function is not exported by default. As with other commands, the resulting
/// query can return the inserted rows if you choose.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate diesel;
/// # include!("../doctest_setup.rs");
/// #
/// # table! {
/// #     users {
/// #         id -> Integer,
/// #         name -> Text,
/// #     }
/// # }
/// #
/// # fn main() {
/// #     use self::users::dsl::*;
/// #     let connection = establish_connection();
/// let rows_inserted = diesel::insert(&name.eq("Sean"))
///     .into(users)
///     .execute(&connection);
///
/// assert_eq!(Ok(1), rows_inserted);
///
/// let new_users = vec![
///     name.eq("Tess"),
///     name.eq("Jim"),
/// ];
///
/// let rows_inserted = diesel::insert(&new_users)
///     .into(users)
///     .execute(&connection);
///
/// assert_eq!(Ok(2), rows_inserted);
/// # }
/// ```
///
/// ### Using struct for values
///
/// ```rust
/// # #[macro_use] extern crate diesel;
/// # include!("../doctest_setup.rs");
/// #
/// # table! {
/// #     users {
/// #         id -> Integer,
/// #         name -> Text,
/// #     }
/// # }
/// #
/// # fn main() {
/// #     use self::users::dsl::*;
/// #     let connection = establish_connection();
/// // Insert one record at a time
///
/// let new_user = NewUser { name: "Ruby Rhod".to_string() };
///
/// diesel::insert(&new_user)
///     .into(users)
///     .execute(&connection)
///     .unwrap();
///
/// // Insert many records
///
/// let new_users = vec![
///     NewUser { name: "Leeloo Multipass".to_string(), },
///     NewUser { name: "Korben Dallas".to_string(), },
/// ];
///
/// let inserted_names = diesel::insert(&new_users)
///     .into(users)
///     .execute(&connection)
///     .unwrap();
/// # }
/// ```
///
/// ### With return value
///
/// ```rust
/// # #[macro_use] extern crate diesel;
/// # include!("../doctest_setup.rs");
/// #
/// # table! {
/// #     users {
/// #         id -> Integer,
/// #         name -> Text,
/// #     }
/// # }
/// #
/// # #[cfg(feature = "postgres")]
/// # fn main() {
/// #     use self::users::dsl::*;
/// #     let connection = establish_connection();
/// // postgres only
/// let new_users = vec![
///     NewUser { name: "Diva Plavalaguna".to_string(), },
///     NewUser { name: "Father Vito Cornelius".to_string(), },
/// ];
///
/// let inserted_names = diesel::insert(&new_users)
///     .into(users)
///     .returning(name)
///     .get_results(&connection);
/// assert_eq!(Ok(vec!["Diva Plavalaguna".to_string(), "Father Vito Cornelius".to_string()]), inserted_names);
/// # }
/// # #[cfg(not(feature = "postgres"))]
/// # fn main() {}
/// ```
pub fn insert<T: ?Sized>(records: &T) -> IncompleteInsertStatement<&T, Insert> {
    IncompleteInsertStatement::new(records, Insert)
}

/// Creates a bare select statement, with no from clause. Primarily used for
/// testing diesel itself, but likely useful for third party crates as well. The
/// given expressions must be selectable from anywhere.
pub fn select<T>(expression: T) -> Select<SelectStatement<()>, T>
where
    T: Expression,
    SelectStatement<()>: SelectDsl<T>,
{
    SelectStatement::simple(()).select(expression)
}

/// Creates an insert statement with default values.
///
/// This function is not exported by default. As with other commands, the resulting
/// query can return the inserted rows if you choose.
#[cfg(feature = "with-deprecated")]
#[deprecated(since = "0.99.0", note = "use `insert(&default_values())` instead")]
pub fn insert_default_values() -> IncompleteInsertStatement<DefaultValues, Insert> {
    IncompleteInsertStatement::new(DefaultValues, Insert)
}

pub fn default_values() -> DefaultValues {
    DefaultValues
}
