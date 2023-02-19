use sha2::{Digest, Sha384};
use std::borrow::Cow;
use std::fmt;

#[derive(Clone, Debug)]
pub(crate) struct Migration {
    pub checksum: Cow<'static, [u8]>,
    pub description: Cow<'static, str>,
    pub statement: Cow<'static, str>,
    pub version: i64,
}

impl fmt::Display for Migration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "description: {}, statement: {}, version: {}",
            self.description, self.statement, self.version
        )
    }
}

pub(crate) struct MigrationArgs {
    pub description: Cow<'static, str>,
    pub statement: Cow<'static, str>,
    pub version: i64,
}

impl Migration {
    pub fn new(args: MigrationArgs) -> Self {
        let MigrationArgs {
            description,
            statement,
            version,
        } = args;
        let checksum = Cow::Owned(Vec::from(Sha384::digest(statement.as_bytes()).as_slice()));

        Self {
            checksum,
            description,
            statement,
            version,
        }
    }
}
