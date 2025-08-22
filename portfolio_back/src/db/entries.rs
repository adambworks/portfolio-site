use crate::models::{Entry, NewEntry, UpdateEntry};
use diesel::prelude::*;

pub fn create_entry(
    conn: &mut PgConnection,
    new_entry: &NewEntry,
) -> Result<Entry, diesel::result::Error> {
    use crate::schema::entries::dsl::entries;
    diesel::insert_into(entries)
        .values(new_entry)
        .get_result(conn)
}

pub fn select_entries(
    conn: &mut PgConnection,
    ch_id: i32,
) -> Result<Vec<Entry>, diesel::result::Error> {
    use crate::schema::entries::dsl::{chapter_id, entries, index};
    entries
        .filter(chapter_id.eq(ch_id))
        .order(index.asc())
        .load(conn)
}

pub fn update_entry(
    conn: &mut PgConnection,
    entry_id: i32,
    entry_update: &UpdateEntry,
) -> Result<Entry, diesel::result::Error> {
    use crate::schema::entries::dsl::entries;
    diesel::update(entries.find(entry_id))
        .set(entry_update)
        .get_result(conn)
}

pub fn delete_entry(
    conn: &mut PgConnection,
    entry_id: i32,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::entries::dsl::entries;
    diesel::delete(entries.find(entry_id)).execute(conn)
}