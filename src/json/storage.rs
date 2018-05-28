use chrono::{DateTime, Local};

#[derive(Serialize, Deserialize, Getters, Debug)]
pub struct Storage {
    path: String,
    capacity: u64,
    used: u64,
    remaining: u64,
    used_prop: f64,
    remaining_prop: f64,
    datetime: DateTime<Local>,
}

#[derive(Default, Debug)]
pub struct StorageBuilder {
    pub path: String,
    pub capacity: u64,
    pub used: u64,
}

impl StorageBuilder {
    pub fn path<P: Into<String>>(mut self, path: P) -> StorageBuilder {
        self.path = path.into();
        self
    }

    pub fn capacity(mut self, capacity: u64) -> StorageBuilder {
        self.capacity = capacity;
        self
    }

    pub fn used(mut self, used: u64) -> StorageBuilder {
        self.used = used;
        self
    }

    pub fn build(&self) -> Storage {
        Storage {
            path: self.path.clone(),
            capacity: self.capacity,
            used: self.used,
            remaining: self.capacity - self.used,
            used_prop: self.used as f64 / self.capacity as f64,
            remaining_prop: (self.capacity - self.used) as f64
                / self.capacity as f64,
            datetime: Local::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_json;

    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_storage_api() {
        const CAPACITY: u64 = 12345;
        const USED: u64 = 1234;

        let builder = StorageBuilder::default();

        let v = builder.path("/").capacity(CAPACITY).used(USED).build();

        // assigned fields
        assert_eq!("/", v.path());
        assert_eq!(CAPACITY, *v.capacity());
        assert_eq!(USED, *v.used());

        // inferred fields
        assert_eq!(CAPACITY - USED, *v.remaining());
        assert_eq!(USED as f64 / CAPACITY as f64, *v.used_prop());
        assert_eq!(
            (CAPACITY - USED) as f64 / CAPACITY as f64,
            *v.remaining_prop()
        );

        // datetime is automatic
    }

    #[test]
    fn test_storage_from_str() {
        let s = r#"{
            "path":"/abc",
            "capacity":1000,
            "used":250,
            "remaining":750,
            "used_prop":0.25,
            "remaining_prop":0.75,
            "datetime":"2017-01-20T13:08:35+00:00"}"#;

        let v: Result<Storage, _> = serde_json::from_str(s);
        assert!(v.is_ok());

        let v = v.unwrap();

        // assigned fields
        assert_eq!("/abc", v.path());
        assert_eq!(1000, *v.capacity());
        assert_eq!(250, *v.used());
        assert_eq!(750, *v.remaining());
        assert_eq!(0.25, *v.used_prop());
        assert_eq!(0.75, *v.remaining_prop());
        assert_eq!(Utc.ymd(2017, 1, 20).and_hms(13, 8, 35), *v.datetime());
    }

    #[test]
    fn test_storage_to_str() {
        const CAPACITY: u64 = 1000;
        const USED: u64 = 750;

        let builder = StorageBuilder::default();

        let v = builder.path("/").capacity(CAPACITY).used(USED).build();

        // check for Serialize trait
        let s = serde_json::to_string(&v);
        assert!(s.is_ok());
    }

    #[test]
    fn test_storage_debug() {
        const CAPACITY: u64 = 1000;
        const USED: u64 = 500;

        let builder = StorageBuilder::default();

        let v = builder.path("/").capacity(CAPACITY).used(USED).build();

        // check for Debug trait
        format!("{:?}", v);
    }
}
