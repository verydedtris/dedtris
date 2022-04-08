use std::{
	path::{Path, PathBuf},
	str::FromStr,
};

use ini::Ini;

use crate::{error::Error, runtime};

pub struct Profile
{
	name: String,
	desc: String,
	lua:  PathBuf,
}

impl std::fmt::Display for Profile
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(f, "{}: {}", self.name, self.desc)
	}
}

pub fn run_game()
{
	let profiles = match load_profiles() {
		Ok(ps) => ps,
		_ => return,
	};

	let i = match super::request_item(&profiles) {
		Some(i) => i,
		_ => return,
	};

	runtime::start_tetris_game(&profiles[i].lua);
}

pub fn load_profiles() -> Result<Vec<Profile>, Error>
{
	let p = Path::new("Profiles");

	let mut v = Vec::new();

	for entry in std::fs::read_dir(p)? {
		let entry = entry?;
		let path = entry.path();
		if path.is_dir() {
			if let Some(profile) = load_profile(&path) {
				v.push(profile);
			}
		}
	}

	Ok(v)
}

pub fn load_profile(p: &Path) -> Option<Profile>
{
	let ini = Ini::load_from_file(p.join("config.ini")).ok()?;
	let sec = ini.general_section();

	let name = load_property(&sec, "name")?;
	let desc = load_property(&sec, "description")?;

	let lua = p.join("script.lua");
	if !lua.is_file() {
		return None;
	}

	Some(Profile { name, desc, lua })
}

pub fn load_property<T: FromStr>(sec: &ini::Properties, key: &str) -> Option<T>
{
	if let Some(v) = sec.get(key) {
		if let Ok(v) = v.parse::<T>() {
			Some(v)
		} else {
			None
		}
	} else {
		None
	}
}
