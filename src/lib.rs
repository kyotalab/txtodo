pub mod cli;
pub mod handler;
pub mod model;
pub mod util;

pub use cli::*;
pub use handler::*;
pub use model::*;
pub use util::*;

const TODO_PATH: &str = "todo.json";

const PROJECT_PREFIX: &str = "+";
const CONTEXT_PREFIX: &str = "@";
