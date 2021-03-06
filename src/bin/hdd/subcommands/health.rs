use hdd::ata::misc::Misc;

use clap::{
	App,
	ArgMatches,
	SubCommand,
};

use serde_json;
use serde_json::value::ToJson;

use ::{DeviceArgument, when_smart_enabled};
use super::{Subcommand, arg_json};

use std::path::Path;

pub struct Health {}
impl Subcommand for Health {
	fn subcommand(&self) -> App<'static, 'static> {
		SubCommand::with_name("health")
			.about("Prints the health status of the device")
			.arg(arg_json())
	}

	fn run(
		&self,
		_: &Option<&Path>,
		dev: &Option<&DeviceArgument>,
		args: &ArgMatches,
	) {
		let dev = dev.unwrap_or_else(|| {
			// TODO show usage and whatnot
			eprint!("<device> is required\n");
			::std::process::exit(1);
		});

		let id = match dev {
			#[cfg(not(target_os = "linux"))]
			DeviceArgument::ATA(_, id) => id,
			DeviceArgument::SAT(_, id) => id,
			DeviceArgument::SCSI(_) => unimplemented!(),
		};

		let use_json = args.is_present("json");

		when_smart_enabled(&id.smart, "health status", || {
			let status = match dev {
				#[cfg(not(target_os = "linux"))]
				DeviceArgument::ATA(dev, _) => dev.get_smart_health().unwrap(),
				DeviceArgument::SAT(dev, _) => dev.get_smart_health().unwrap(),
				DeviceArgument::SCSI(_) => unimplemented!(),
			};

			if use_json {
				print!("{}\n", serde_json::to_string(&status.to_json().unwrap()).unwrap());
			} else {
				print!("S.M.A.R.T. health status: {}\n", match status {
					Some(true) => "good",
					Some(false) => "BAD",
					None => "(unknown)",
				});
			}
		});
	}
}
