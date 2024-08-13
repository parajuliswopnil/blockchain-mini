use std::fs;

use persy::{Config, Persy};

#[derive(Clone)]
pub struct Database {
    path: String,
    db: Persy,
}

impl Database {
    pub fn new(path: String) -> Database {
        match fs::metadata(path.clone()) {
            Ok(_) => {}
            Err(_) => {
                let create_db = persy::Persy::create(path.clone());
                match create_db {
                    Ok(()) => {}
                    Err(e) => panic!("Error in creating db {:?}", e),
                }
            }
        }
        let persy_db = Persy::open(path.clone(), Config::new());
        match persy_db {
            Ok(db_instance) => Database {
                path: path,
                db: db_instance,
            },
            Err(e) => {
                panic!("Error in instanciating database, {}", e);
            }
        }
    }

    pub fn insert_data(self, index: String, key: String, value: String) {
        let mut tx = self.db.begin().unwrap();
        if !tx.exists_index(&index).unwrap() {
            tx.create_index::<String, String>(&index, persy::ValueMode::Cluster)
                .unwrap();
        }
        tx.put(&index, key, value).unwrap();
        tx.prepare().unwrap().commit().unwrap();
    }

    pub fn get_data(self, index: String, key: String) -> String {
        let value = self.db.get::<String, String>(&index, &key).unwrap();
        return value.into_iter().next().unwrap();
    }

    pub fn self_destruct(self) {
        fs::remove_file(self.path.clone()).unwrap();
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        println!("db is dropped")
    }
}

#[test]
fn test_insert_data() {
    let database = Database::new("db.persy".to_string());
    database.clone().insert_data(
        "test_index".to_string(),
        "key".to_string(),
        "value".to_string(),
    );
    let value = database
        .clone()
        .get_data("test_index".to_string(), "key".to_string());
    assert_eq!(value, "value".to_string());
    database.self_destruct();
}
