use diesel::{Connection, SqliteConnection};
use std::{path::PathBuf, sync::atomic::AtomicU32};

static TEST_DB_COUNTER: AtomicU32 = AtomicU32::new(0);

pub struct TestDb {
    name: String,
    path: PathBuf,
    delete_on_drop: bool,
}

impl TestDb {
    pub fn new() -> Self {
        let name = format!(
            "test_db_{}_{}",
            std::process::id(),
            TEST_DB_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        );

        let path = PathBuf::from(&format!("{name}.sqlite"));

        println!("DB PATH: {:?}", path);
        std::fs::File::create(&path).unwrap();
        let migration = std::process::Command::new("sqlite3")
            .args([
                path.to_str().expect("Malformed test db path"),
                include_str!("../../../model/migrations/2024-05-19-142608_init/up.sql"),
                include_str!("../../../model/migrations/2024-05-19-142611_address/up.sql"),
            ])
            .spawn();
        if let Ok(mut child) = migration {
            match child.wait() {
                Ok(_) => (),
                Err(e) => eprintln!("{}", e),
            }
        } else {
            panic!("Could not run migration: {}", migration.err().unwrap())
        }

        Self {
            name,
            path,
            delete_on_drop: true,
        }
    }

    pub fn connection(&self) -> SqliteConnection {
        let sqconn = SqliteConnection::establish(self.path.to_str().unwrap());
        assert!(sqconn.is_ok());
        sqconn.expect("Cannot create database connection")
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

impl Drop for TestDb {
    fn drop(&mut self) {
        if !self.delete_on_drop {
            eprintln!("TestDb leaking database {}", self.name);
            return;
        }

        std::fs::remove_file(&self.path).unwrap();
    }
}
