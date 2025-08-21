use diesel::prelude::*;
use diesel::result::Error;
use crate::models::{NewUser, User};
use crate::schema::users;

pub fn create_user(conn: &mut PgConnection, new_user: &NewUser) -> Result<User, Error> {
    diesel::insert_into(users::table)
        .values(new_user)
        .get_result(conn)
}

pub fn find_user_by_username(conn: &mut PgConnection, username_to_find: &str) -> Result<User, Error> {
    users::table
        .filter(users::username.eq(username_to_find))
        .first(conn)
}