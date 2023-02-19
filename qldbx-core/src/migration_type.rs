pub struct MigrationType;

impl MigrationType {
    pub fn path() -> &'static str {
        "./migrations"
    }

    pub fn extension() -> &'static str {
        ".partiql"
    }

    pub fn content() -> &'static str {
        "-- PartiQL content\n"
    }

    pub fn split_char() -> char {
        '_'
    }
}
