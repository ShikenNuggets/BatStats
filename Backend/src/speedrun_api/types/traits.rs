#![allow(dead_code)]

pub trait Cacheable{
	fn key(&self) -> String;
}
