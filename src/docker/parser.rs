/// Yaml
/// 
/// # Description
/// 
/// Yaml parser module is use to extract the content of a yaml file
pub mod yaml {
    use std::path::PathBuf;
    use std::error::Error;
    use yaml_rust::{YamlLoader, yaml};
    use crate::cli::core::fs::toolbox;
    use crate::errors::cli_error::{CliErr, ErrCode, ErrHelper};

    /// Error constant
    const UNABLE_READ_ERR: &str = "Unable to open the docker-compose.yaml file";
    const UNABLE_PARSE_ERR: &str = "Unable to parse the docker-compose.yaml for reason: ";
    const ABS_PATH_ERROR: &str = "Unable to generate absolute path";

    /// Parse
    /// 
    /// # Description
    /// Retrieve the yaml content
    /// 
    /// # Arguments
    /// * `path` slice of string
    /// * `file_name` slice of string
    /// 
    /// # Return
    /// Result<Vec<yaml::Yaml>, CliErr>
    pub fn parse(path: &str, file_name: &str) -> Result<Vec<yaml::Yaml>, CliErr> {
        let mut paths = PathBuf::new();
        paths.push(path);
        paths.push(file_name);

        let compose_file_path = match toolbox::get_absolute_path(&paths) {
            Ok(p) => p,
            Err(err) => {
                return Err(
                    CliErr::new(
                        ABS_PATH_ERROR,
                        format!("{}", err.description()),
                        ErrCode::IOError
                    )
                );
            }
        };

        let content = toolbox::open_and_read_string_file(&compose_file_path);
        if let Ok(value) = content {
            return match parse_yaml_tree(value) {
                Ok(yaml_content) => Ok(yaml_content),
                Err(err) => {
                    return Err(
                        CliErr::new(
                            UNABLE_PARSE_ERR,
                            format!("{}", err.description()),
                            ErrCode::IOError
                        )
                    );
                }
            }
        }

        Err(
            CliErr::new(UNABLE_READ_ERR, String::from(""), ErrCode::IOError)
        )
    }

    /// Parse Yaml Tree
    ///  
    /// # Description
    /// Retrieve the `yaml-rust` library yaml representation
    /// 
    /// # Argument
    /// * `content` String
    /// 
    /// # Return
    /// Result<Vec<yaml::Yaml>, yaml_rust::ScanError>
    fn parse_yaml_tree(content: String) -> Result<Vec<yaml::Yaml> , yaml_rust::ScanError> {
        let yaml_file = YamlLoader::load_from_str(content.as_str());

        return match yaml_file {
            Ok(yaml_content) => Ok(yaml_content),
            Err(e) => Err(e)
        };
    }
}