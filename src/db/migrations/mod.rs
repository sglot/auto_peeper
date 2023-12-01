use rusqlite::Connection;

use self::{
    m1_create_table_migrations::CreateTableMigrations,
    m2_create_table_drom_bulls::CreateTableDromBulls, 
    migration_writer::write_migration,
};

use super::connection::sqlite_connection_factory;

pub mod migration_writer;

pub mod m1_create_table_migrations;
pub mod m2_create_table_drom_bulls;

pub fn migrate() {
    let conn = sqlite_connection_factory::get();

    let migrations: Vec<Box<dyn Migration>> = vec![
        Box::new(CreateTableMigrations::new()),
        Box::new(CreateTableDromBulls::new()),
    ];

    migrations
        .iter()
        .for_each(|migration: &Box<dyn Migration>| {
            migration.up(&conn);
            write_migration(migration.get_name(), &conn)
        });
}

trait Migration {
    fn up(&self, conn: &Connection);
    fn down(&self, conn: &Connection);
    fn get_name(&self) -> &str;
}
