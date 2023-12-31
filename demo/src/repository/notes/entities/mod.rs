use crate::schema::notes::dsl::notes;
use diesel::prelude::*;
use orm::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::notes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NoteEntity {
    pub id: i32,
    pub title: String,
    pub body: String,
    // created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::notes)]
pub struct NewNoteEntity {
    pub title: String,
    pub body: String,
}

impl Entity for NoteEntity {}
impl Entity for NewNoteEntity {}
