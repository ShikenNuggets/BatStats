use dashmap::DashMap;

use crate::speedrun_api::types::traits::Cacheable;

pub struct Cache<T: Cacheable + Clone>{
	map: DashMap<String, T>,
}

impl<T: Cacheable + Clone> Cache<T>{
	pub fn new() -> Self{
		Self{
			map: DashMap::new(),
		}
	}

	pub fn get(&self, key: &str) -> Option<T>{
		return self.map.get(key).map(|v| v.clone());
	}

	pub fn insert(&self, value: &T){
		self.map.insert(value.key(), value.clone());
	}
}
