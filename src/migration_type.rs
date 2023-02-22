pub struct MigrationType;

impl MigrationType {
    pub fn path() -> &'static str {
        "./migrations"
    }

    pub fn extension() -> &'static str {
        ".partiql"
    }

    pub fn split_char() -> char {
        '_'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_test() {
        assert_eq!(MigrationType::path(), "./migrations");
    }

    #[test]
    fn extension_test() {
        assert_eq!(MigrationType::extension(), ".partiql");
    }

    #[test]
    fn split_char_test() {
        assert_eq!(MigrationType::split_char(), '_');
    }
}
