extern crate diesel;
extern crate diesel_migrations;
extern crate log;

use std::error::Error;

use diesel::{prelude::*, sqlite::Sqlite, Insertable, Queryable, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::debug;

use crate::schema::{
    instantiate_class, instantiate_function, objects, parse_class, parse_template, source,
};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn run_migrations(
    connection: &mut impl MigrationHarness<Sqlite>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    debug!("Run migrations");
    connection.run_pending_migrations(MIGRATIONS)?;
    debug!("Migrations done");

    Ok(())
}

pub fn get_connection(
    target: &str,
) -> Result<SqliteConnection, Box<dyn Error + Send + Sync + 'static>> {
    let mut connection = SqliteConnection::establish(target)?;
    run_migrations(&mut connection)?;

    // At this stage, we delete all tables
    diesel::delete(objects::table).execute(&mut connection)?;
    diesel::delete(source::table).execute(&mut connection)?;
    diesel::delete(parse_class::table).execute(&mut connection)?;
    diesel::delete(parse_template::table).execute(&mut connection)?;
    diesel::delete(instantiate_class::table).execute(&mut connection)?;
    diesel::delete(instantiate_function::table).execute(&mut connection)?;

    Ok(connection)
}

#[derive(Queryable)]
#[diesel(table_name = objects)]
pub struct Object {
    pub path: String,
    pub total_time: i32,
    pub frontend: i32,
    pub backend: i32,
}

#[derive(Insertable)]
#[diesel(table_name = objects)]
pub struct NewObject<'a> {
    pub path: &'a str,
    pub total_time: i32,
    pub frontend: i32,
    pub backend: i32,
}

impl<'a> NewObject<'a> {
    pub fn new(path: &'a str, total_time: i32, frontend: i32, backend: i32) -> Self {
        NewObject {
            path,
            total_time,
            frontend,
            backend,
        }
    }

    pub fn insert(&self, conn: &mut SqliteConnection) -> Result<(), Box<dyn Error + 'static>> {
        // This should not cause conflict
        let _ = diesel::insert_into(objects::table)
            .values(self)
            .execute(conn)?;
        Ok(())
    }
}

pub trait InsertTrait {
    fn insert(&self, conn: &mut SqliteConnection) -> Result<(), Box<dyn Error + 'static>>;
}

#[derive(Queryable)]
#[diesel(table_name = source)]
pub struct Sources {
    pub path: String,
    pub duration: i32,
    pub count: i32,
}

#[derive(Insertable)]
#[diesel(table_name = source)]
pub struct NewSource<'a> {
    pub path: &'a str,
    pub duration: i32,
    pub count: i32,
}

impl<'a> NewSource<'a> {
    pub fn new(path: &'a str, duration: i32) -> Self {
        NewSource {
            path,
            duration,
            count: 1,
        }
    }
}

impl<'a> InsertTrait for NewSource<'a> {
    fn insert(&self, conn: &mut SqliteConnection) -> Result<(), Box<dyn Error + 'static>> {
        use source::{count, duration, path};
        let _ = diesel::insert_into(source::table)
            .values(self)
            .on_conflict(path)
            .do_update()
            .set((
                count.eq(count + self.count),
                duration.eq(duration + self.duration),
            ))
            .execute(conn)?;
        Ok(())
    }
}

#[derive(Queryable)]
#[diesel(table_name = parse_class)]
pub struct ParseClass {
    pub name: String,
    pub duration: i32,
    pub count: i32,
}

#[derive(Insertable)]
#[diesel(table_name = parse_class)]
pub struct NewParseClass<'a> {
    pub name: &'a str,
    pub duration: i32,
    pub count: i32,
}

impl<'a> NewParseClass<'a> {
    pub fn new(name: &'a str, duration: i32) -> Self {
        NewParseClass {
            name,
            duration,
            count: 1,
        }
    }
}

impl<'a> InsertTrait for NewParseClass<'a> {
    fn insert(&self, conn: &mut SqliteConnection) -> Result<(), Box<dyn Error + 'static>> {
        use parse_class::{count, duration, name};
        let _ = diesel::insert_into(parse_class::table)
            .values(self)
            .on_conflict(name)
            .do_update()
            .set((
                count.eq(count + self.count),
                duration.eq(duration + self.duration),
            ))
            .execute(conn)?;
        Ok(())
    }
}

#[derive(Queryable)]
#[diesel(table_name = parse_template)]
pub struct ParseTemplate {
    pub name: String,
    pub duration: i32,
    pub count: i32,
}

#[derive(Insertable)]
#[diesel(table_name = parse_template)]
pub struct NewParseTemplate<'a> {
    pub name: &'a str,
    pub duration: i32,
    pub count: i32,
}

impl<'a> NewParseTemplate<'a> {
    pub fn new(name: &'a str, duration: i32) -> Self {
        NewParseTemplate {
            name,
            duration,
            count: 1,
        }
    }
}

impl<'a> InsertTrait for NewParseTemplate<'a> {
    fn insert(&self, conn: &mut SqliteConnection) -> Result<(), Box<dyn Error + 'static>> {
        use parse_template::{count, duration, name};
        let _ = diesel::insert_into(parse_template::table)
            .values(self)
            .on_conflict(name)
            .do_update()
            .set((
                count.eq(count + self.count),
                duration.eq(duration + self.duration),
            ))
            .execute(conn)?;
        Ok(())
    }
}

#[derive(Queryable)]
#[diesel(table_name = instantiate_class)]
pub struct InstantiateClass {
    pub name: String,
    pub duration: i32,
    pub count: i32,
}

#[derive(Insertable)]
#[diesel(table_name = instantiate_class)]
pub struct NewInstantiateClass<'a> {
    pub name: &'a str,
    pub duration: i32,
    pub count: i32,
}

impl<'a> NewInstantiateClass<'a> {
    pub fn new(name: &'a str, duration: i32) -> Self {
        NewInstantiateClass {
            name,
            duration,
            count: 1,
        }
    }
}

impl<'a> InsertTrait for NewInstantiateClass<'a> {
    fn insert(&self, conn: &mut SqliteConnection) -> Result<(), Box<dyn Error + 'static>> {
        use instantiate_class::{count, duration, name};
        let _ = diesel::insert_into(instantiate_class::table)
            .values(self)
            .on_conflict(name)
            .do_update()
            .set((
                count.eq(count + self.count),
                duration.eq(duration + self.duration),
            ))
            .execute(conn)?;
        Ok(())
    }
}

#[derive(Queryable)]
#[diesel(table_name = instantiate_function)]
pub struct InstantiateFunction {
    pub name: String,
    pub duration: i32,
    pub count: i32,
}

#[derive(Insertable)]
#[diesel(table_name = instantiate_function)]
pub struct NewInstantiateFunction<'a> {
    pub name: &'a str,
    pub duration: i32,
    pub count: i32,
}

impl<'a> NewInstantiateFunction<'a> {
    pub fn new(name: &'a str, duration: i32) -> Self {
        NewInstantiateFunction {
            name,
            duration,
            count: 1,
        }
    }
}
    
impl<'a> InsertTrait for NewInstantiateFunction<'a> {
    fn insert(&self, conn: &mut SqliteConnection) -> Result<(), Box<dyn Error + 'static>> {
        use instantiate_function::{count, duration, name};
        let _ = diesel::insert_into(instantiate_function::table)
            .values(self)
            .on_conflict(name)
            .do_update()
            .set((count.eq(count + self.count), duration.eq(duration + 1)))
            .execute(conn)?;
        Ok(())
    }
}

