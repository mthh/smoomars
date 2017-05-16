// This file is released into Public Domain.

extern crate argparse_rs;

use gnuplot::*;
use self::argparse_rs::*;
use std::env;


pub struct Common
{
	pub no_show: bool,
	pub term: Option<String>,
}

impl Common
{
	pub fn new() -> Option<Common>
	{
		let arg_vec: Vec<_> = env::args().collect();

		let mut args = ArgParser::new(arg_vec[0].clone());

		args.add_opt("no-show", Some("false"), 'n', false, "do not run the gnuplot process.", ArgType::Flag);
		args.add_opt("terminal", None, 't', false, "specify what terminal to use for gnuplot.", ArgType::Option);

		let res = args.parse(arg_vec.iter()).unwrap();

		if res.get("help").unwrap_or(false)
		{
			args.help();
			return None;
		}

		Some(Common {
			no_show: res.get("no-show").unwrap(),
			term: res.get::<String>("terminal").map(|s| s.to_string()),
		})
	}

	pub fn show(&self, fg: &mut Figure, filename: &str)
	{
		if !self.no_show
		{
			fg.show();
		}
		fg.echo_to_file(filename);
	}

	pub fn set_term(&self, fg: &mut Figure)
	{
		self.term.as_ref().map(|t| { fg.set_terminal(&t[..], ""); });
	}
}
