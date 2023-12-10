use rusqlite::Connection;

use self::{
    m1_create_table_migrations::CreateTableMigrations,
    m2_create_table_drom_bulls::CreateTableDromBulls,
    m3_rename_table_drom_bulls_to_cars::RenameTableDromBullsToCars,
    m4_add_system_field_to_cars::AddSystemFieldToCars,
    m5_add_exclusive_field_to_cars::AddExclusiveFieldToCars,
    m6_fill_system_to_cars::FillSystemToCars, m7_create_table_progress::CreateTableProgress,
    migration_writer::write_migration,
};

use super::connection::sqlite_connection_factory;

pub mod migration_checker;
pub mod migration_writer;

pub mod m1_create_table_migrations;
pub mod m2_create_table_drom_bulls;
pub mod m3_rename_table_drom_bulls_to_cars;
pub mod m4_add_system_field_to_cars;
pub mod m5_add_exclusive_field_to_cars;
pub mod m6_fill_system_to_cars;
pub mod m7_create_table_progress;

pub fn migrate() {
    let conn = sqlite_connection_factory::get();

    let migrations: Vec<Box<dyn Migration>> = vec![
        Box::new(CreateTableMigrations::new()),
        Box::new(CreateTableDromBulls::new()),
        Box::new(RenameTableDromBullsToCars::new()),
        Box::new(AddSystemFieldToCars::new()),
        Box::new(AddExclusiveFieldToCars::new()),
        Box::new(FillSystemToCars::new()),
        Box::new(CreateTableProgress::new()),
    ];

    migrations
        .iter()
        .for_each(|migration: &Box<dyn Migration>| {
            if !migration_checker::check_migration(migration.get_name(), &conn) {
                return;
            }

            migration.up(&conn);
            write_migration(migration.get_name(), &conn)
        });
}

trait Migration {
    fn up(&self, conn: &Connection);
    fn down(&self, conn: &Connection);
    fn get_name(&self) -> &str;
}
