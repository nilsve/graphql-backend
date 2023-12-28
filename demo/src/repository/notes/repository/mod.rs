use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use orm::prelude::*;

use crate::repository::notes::entities::{NewNoteEntity, NoteEntity};
use crate::schema::notes::dsl::notes;

pub struct NotesRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl NotesRepository {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

impl Repository<NoteEntity, NewNoteEntity> for NotesRepository {
    type Connection = PooledConnection<ConnectionManager<PgConnection>>;

    fn get_connection(&self) -> Self::Connection {
        self.pool.get().unwrap()
    }

    fn create(&self, item: NewNoteEntity) -> QueryResult<usize> {
        diesel::insert_into(notes)
            .values(&item)
            .execute(&mut self.get_connection())
    }

    fn update(&self, item: NoteEntity) -> QueryResult<usize> {
        todo!()
    }

    fn delete(&self, item: NoteEntity) -> QueryResult<usize> {
        todo!()
    }

    fn find(&self, id: i32) -> QueryResult<Option<NoteEntity>> {
        todo!()
    }

    fn find_all(&self) -> QueryResult<Vec<NoteEntity>> {
        notes
            .select(NoteEntity::as_select())
            .load(&mut self.get_connection())
    }
}
