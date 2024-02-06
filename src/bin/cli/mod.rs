mod utils;

pub(crate) mod commands;
pub(crate) mod logger;

mod cli_error;
mod cli_handle_kugou;
mod cli_handle_qmc1;
mod cli_handle_qmc2;
mod cli_handle_ximalaya_android;

pub use cli_handle_kugou::cli_handle_kugou;
pub use cli_handle_qmc1::cli_handle_qmc1;
pub use cli_handle_qmc2::cli_handle_qmc2;
pub use cli_handle_ximalaya_android::cli_handle_ximalaya_android;
