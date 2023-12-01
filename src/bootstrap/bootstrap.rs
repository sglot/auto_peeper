pub mod bootstrap {
    use dotenv::dotenv;
    use log::{error, info};
    use std::fs;

    use crate::db::migrations;

    pub fn init() -> () {
        env();
        log();
        db();
    }

    fn env() {
        dotenv().ok();
    }

    fn log() {
        log4rs::init_file("src/config/logging.yaml", Default::default()).unwrap();
    }

    fn db() {
        let path = std::env::var("DATABASE_PATH").expect("DATABASE_PATH must be set");
        let db_name = std::env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
        let mut file = path.clone();
        file.push_str(&db_name);

        if std::env::var("FORCE_MIGRATE")
            .unwrap_or("false".to_string())
            .eq("true")
        {
            create_path(&path);
            migrate();

            return;
        }

        match fs::metadata(&file) {
            Ok(_) => {
                info!("db isset");
                ()
            }
            Err(_) => {
                create_path(&path);
                migrate();
            }
        }
    }

    fn create_path(file_path: &String) {
        match fs::create_dir_all(&file_path) {
            Ok(it) => it,
            Err(err) => error!("Bootstrap db create dir error: {}", err),
        };
    }

    fn migrate() {
        migrations::migrate();
    }
}
