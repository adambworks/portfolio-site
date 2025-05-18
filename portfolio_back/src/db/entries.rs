use crate::models::Entry;
use diesel::prelude::*;
use crate::schema::entries::dsl::*;

pub fn select_entries(conn: &mut PgConnection, ch_id: i32)-> Result<Vec<Entry>, diesel::result::Error>{
   return entries.filter(chapter_id.eq(ch_id))
    .order(index.asc())
    .load(conn);

}