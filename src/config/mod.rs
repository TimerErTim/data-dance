pub struct DataDanceConfiguration {
    pub port: u16,
    pub host: String,

    pub encryption: crate::objects::encryption::EncryptionLevel,
}
