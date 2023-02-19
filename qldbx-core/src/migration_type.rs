pub struct MigrationType;

impl MigrationType {
    pub fn extension() -> &'static str {
        ".partiql"
    }

    pub fn content() -> &'static str {
        "-- PartiQL content\n"
    }
}
