use std::io::Write;

mod profile;

pub struct MenuItem
{
	desc:   &'static str,
	action: fn(),
}

impl std::fmt::Display for MenuItem
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(f, "{}", self.desc)
	}
}

const MENU_ITEMS: [MenuItem; 1] = [MenuItem {
	desc:   "Start a tetris game.",
	action: profile::run_game,
}];

pub fn start_menu() -> bool
{
	if let Some(i) = request_item(&MENU_ITEMS) {
		(MENU_ITEMS[i].action)();
		false
	} else {
		true
	}
}

pub fn request_item<T: std::fmt::Display>(items: &[T]) -> Option<usize>
{
	let mut input = String::new();

	loop {
		for (i, item) in items.iter().enumerate() {
			println!("{}) {}", i + 1, item);
		}

		println!("{}) exit.", items.len() + 1);

		print!("> ");

		std::io::stdout().flush().unwrap();
		std::io::stdin().read_line(&mut input).unwrap();

		if let Ok(val) = input.trim().parse::<usize>() {
			match val {
				v if v <= items.len() && v > 0 => break Some(v - 1),
				v if v == items.len() + 1 => break None,
				_ => println!("Unrecognized option."),
			}
		} else {
			println!("Input isn't a number or is invalid.");
		}

		input.clear();
	}
}

#[rustfmt::skip]
pub fn print_banner()
{
	println!(
"
███╗   ███╗ █████╗      ██╗████████╗██████╗ ██╗███████╗
████╗ ████║██╔══██╗     ██║╚══██╔══╝██╔══██╗██║██╔════╝
██╔████╔██║███████║     ██║   ██║   ██████╔╝██║███████╗
██║╚██╔╝██║██╔══██║██   ██║   ██║   ██╔══██╗██║╚════██║
██║ ╚═╝ ██║██║  ██║╚█████╔╝   ██║   ██║  ██║██║███████║
╚═╝     ╚═╝╚═╝  ╚═╝ ╚════╝    ╚═╝   ╚═╝  ╚═╝╚═╝╚══════╝
"
	);
}
