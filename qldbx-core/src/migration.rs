use sha2::{Digest, Sha384};
use std::borrow::Cow;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Migration {
    pub name: Cow<'static, str>,
    pub statement: Cow<'static, str>,
    pub utc: i64,
}

impl fmt::Display for Migration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            r#"{{ name: "{}", statement: "{}", utc: {} }}"#,
            self.name, self.statement, self.utc
        )
    }
}

pub struct MigrationArgs {
    pub name: Cow<'static, str>,
    pub statement: Cow<'static, str>,
    pub utc: i64,
}

impl Migration {
    pub fn new(args: MigrationArgs) -> Self {
        let MigrationArgs {
            name,
            statement,
            utc,
        } = args;

        Self {
            name,
            statement,
            utc,
        }
    }

    pub fn checksum(&self) -> Vec<u8> {
        let m = format!("{}{}{}", self.name, self.statement, self.utc);
        Sha384::digest(m.as_bytes()).as_slice().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_migration() -> Migration {
        Migration::new(MigrationArgs {
            name: Cow::Borrowed("foo"),
            statement: Cow::Borrowed("create table foo"),
            utc: 1234567890,
        })
    }

    #[test]
    fn migration_test() {
        let migration = new_migration();

        let checksum = migration.checksum();
        // Verify that the checksum has the expected length.
        assert_eq!(checksum.len(), 48);
        // Verify that the checksum is correct.
        let expected_checksum = vec![
            25, 9, 195, 79, 10, 111, 73, 159, 194, 202, 158, 18, 254, 6, 2, 4, 71, 93, 68, 161,
            146, 130, 235, 59, 214, 141, 151, 85, 37, 92, 185, 159, 210, 73, 83, 86, 64, 111, 161,
            6, 147, 29, 25, 78, 109, 125, 93, 150,
        ];
        assert_eq!(checksum, expected_checksum);
    }

    #[test]
    fn test_display() {
        let migration = new_migration();

        // Verify that the output of the Display trait matches the expected string.
        assert_eq!(
            migration.to_string(),
            r#"{ name: "foo", statement: "create table foo", utc: 1234567890 }"#
        );
    }
}
