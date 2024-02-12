use diesel::{Connection, SqliteConnection};
use std::{path::PathBuf, sync::atomic::AtomicU32};

static TEST_DB_COUNTER: AtomicU32 = AtomicU32::new(0);

type TestConnection = diesel_logger::LoggingConnection<SqliteConnection>;

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

        let mut path = PathBuf::from(&name);
        path.set_extension(".sqlite");
        println!("DB PATH: {:?}", path);
        std::fs::File::create(&path).unwrap();
        let migration = std::process::Command::new("diesel")
            .args([
                "migration",
                "run",
                "--database-url",
                path.to_str().expect("Malformed test db path"),
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

    pub fn connection(&self) -> TestConnection {
        let sqconn = SqliteConnection::establish(self.path.to_str().unwrap());
        assert!(sqconn.is_ok());
        diesel_logger::LoggingConnection::new(sqconn.unwrap())
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
