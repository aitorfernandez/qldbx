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
        write!(f, r#"{{ checksum: "{}" }}"#, self.checksum())
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

    pub fn checksum(&self) -> String {
        let m = format!("{}{}{}", self.name, self.statement, self.utc);
        hex::encode(Sha384::digest(m.as_bytes()).as_slice())
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
    fn checksum_test() {
        let migration = new_migration();
        assert_eq!(migration.checksum(), "1909c34f0a6f499fc2ca9e12fe060204475d44a19282eb3bd68d9755255cb99fd2495356406fa106931d194e6d7d5d96");
    }

    #[test]
    fn test_display() {
        let migration = new_migration();
        assert_eq!(
            migration.to_string(),
            r#"{ checksum: "1909c34f0a6f499fc2ca9e12fe060204475d44a19282eb3bd68d9755255cb99fd2495356406fa106931d194e6d7d5d96" }"#
        );
    }
}
