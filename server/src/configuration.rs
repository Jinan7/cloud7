#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub s3: S3Settings,
}

#[derive(serde::Deserialize, Debug)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
}

#[derive(serde::Deserialize, Debug)]
pub struct S3Settings {
    pub bucket: String,
    pub expires_in: u64,
}
enum ENVIRONMENT {
    LOCAL,
    PRODUCTION
}

impl TryFrom<String>  for ENVIRONMENT {

    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        
        match value.as_str() {
            "local" => Ok(ENVIRONMENT::LOCAL),
            "production" => Ok(ENVIRONMENT::PRODUCTION),
            _ => Err(format!("{} is not a valid environment", value))
        }
    }
}

impl AsRef<str> for ENVIRONMENT {
    
    fn as_ref(&self) -> &str {       
        match self {
            ENVIRONMENT::LOCAL => "local",
            ENVIRONMENT::PRODUCTION => "production",
        }
    }
}


pub fn get_configuration() -> Settings {

    let environment: ENVIRONMENT = std::env::var("ENVIRONMENT")
        .unwrap_or_else(
            |_| "local".into()
        )
        .try_into()
        .expect("failed to read environment");

    let base_dir = {
        let root_dir = std::env::current_dir().expect("could not read root directory");
        root_dir.join("configuration")
    };

    let base_config_path = base_dir.join("base.yaml");
    let environment_config_path = base_dir.join(format!("{}.yaml", environment.as_ref()));
    
    let config = config::Config::builder()
        .add_source(config::File::from(base_config_path))
        .add_source(config::File::from(environment_config_path))
        .build()
        .expect("failed to build config");

    let settings = config.try_deserialize::<Settings>().expect("failed to deserialize config");

    settings
}


#[cfg(test)]
mod test {
    use crate::configuration::get_configuration;

    #[test]
    fn get_configuration_test() {

        let settings = get_configuration();
        dbg!(settings);
    }
}