use once_cell::sync::Lazy;

use crate::speedrun_api::{cache::Cache, types::{category::Category, game::Game, run::Run, user::User, variable::Variable}};

pub static CATEGORY_CACHE: Lazy<Cache<Category>> = Lazy::new(Cache::new);
pub static GAME_CACHE: Lazy<Cache<Game>> = Lazy::new(Cache::new);
pub static RUN_CACHE: Lazy<Cache<Run>> = Lazy::new(Cache::new);
pub static USER_CACHE: Lazy<Cache<User>> = Lazy::new(Cache::new);
pub static VARIABLE_CACHE: Lazy<Cache<Variable>> = Lazy::new(Cache::new);
