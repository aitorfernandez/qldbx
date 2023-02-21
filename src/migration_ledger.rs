use ion_c_sys::{
    reader::{IonCReader, IonCReaderHandle},
    result::IonCError,
};

#[derive(Debug)]
pub struct MigrationLedger {
    pub checksum: String,
}

impl TryFrom<String> for MigrationLedger {
    type Error = IonCError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let mut r = IonCReaderHandle::try_from(s.as_str())?;

        // step to the struct
        r.next()?;
        // step into the struct
        r.step_in()?;

        // step to checksum
        r.next()?;

        let checksum = r.read_string()?.to_string();

        Ok(Self { checksum })
    }
}
