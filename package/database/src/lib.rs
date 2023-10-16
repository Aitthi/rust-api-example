pub mod models;

use serde::{Deserialize, Serialize};
pub use surrealdb::{self};
use surrealdb::{
    engine::local::{Db, SpeeDb},
    sql::Thing,
    Surreal,
};
use tokio::sync::OnceCell;

pub struct Database {
    pub db: Surreal<Db>,
    pub database: String,
    pub namespace: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}

pub async fn init() {
    let _ = get_database().await;
}

// This is a global variable that is initialized once
static GLOBAL_DB: once_cell::sync::Lazy<OnceCell<Database>> =
    once_cell::sync::Lazy::new(OnceCell::new);

pub async fn get_database() -> &'static Database {
    GLOBAL_DB
        .get_or_init(|| async {
            // println!("Init database");
            let conf = config::get_config();
            let mut namespace = conf.database.namespace.as_str();
            let mut database = conf.database.database.as_str();

            // cfg test overwrite the namespace and database
            if cfg!(test) {
                namespace = "app_test";
                database = "app_db_test";
            }

            let disk = if conf.database.engine == config::models::DatabaseEngine::Speedb {
                Surreal::new::<SpeeDb>(conf.database.file.as_str())
                    .await
                    .unwrap()
            } else {
                panic!("unknown database engine");
            };

            if let Err(e) = disk.use_ns(namespace).use_db(database).await {
                panic!("disk error: {}", e);
            }

            Database {
                db: disk,
                namespace: namespace.to_string(),
                database: database.to_string(),
            }
        })
        .await
}
