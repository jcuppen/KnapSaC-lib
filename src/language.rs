use serde::Serialize;
use serde::Deserialize;

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Language {
    pub(crate) compiler_command_name: String,
    pub(crate) output_option: String,
}
