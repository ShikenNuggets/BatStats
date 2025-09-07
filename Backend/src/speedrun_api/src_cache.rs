#![allow(dead_code)]

use dashmap::DashMap;
use once_cell::sync::Lazy;

use crate::speedrun_api::{cache::Cache, types::{category::Category, game::Game, leaderboard::Leaderboard, run::Run, user::User, variable::Variable}};

pub static CATEGORY_CACHE: Lazy<Cache<Category>> = Lazy::new(Cache::new);
pub static GAME_CACHE: Lazy<Cache<Game>> = Lazy::new(Cache::new);
pub static LEADERBOARD_CACHE: Lazy<Cache<Leaderboard>> = Lazy::new(Cache::new);
pub static RUN_CACHE: Lazy<Cache<Run>> = Lazy::new(Cache::new);
pub static USER_CACHE: Lazy<Cache<User>> = Lazy::new(Cache::new);
pub static VARIABLE_CACHE: Lazy<Cache<Variable>> = Lazy::new(Cache::new);

pub static ALL_VARS_CACHE: Lazy<DashMap<String, Vec<Variable>>> = Lazy::new(DashMap::new);
pub static ALL_CATS_CACHE: Lazy<DashMap<String, Vec<Category>>> = Lazy::new(DashMap::new);
