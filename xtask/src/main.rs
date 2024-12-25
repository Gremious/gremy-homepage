use std::{fmt::Display, io, process::ExitStatus};

use clap::{Parser, Subcommand};
use execute::command;
use shared::shared_prelude::*;

static SSH: Lazy<String> = Lazy::new(|| shared::CONFIG.ssh.clone());

#[derive(Parser)]
#[command(bin_name = "cargo xtask")]
struct Cli {
	#[command(subcommand)]
	command: Command,
}

#[derive(Subcommand)]
enum Command {
	Build {
		#[arg(short, long)] release: bool,
	},
	BuildClient {
		#[arg(short, long)] release: bool,
	},
	BuildServer {
		#[arg(short, long)] release: bool,
	},
	WatchClient,
	WatchServer,
	Deploy,
	ReRun,
}

fn main() {
	let cli = Cli::parse();
	handle_command(cli.command).unwrap();
}

fn command_output(cmd: impl AsRef<str>) -> anyhow::Result<String> {
	Ok(command(cmd).output()?.stdout.pipe(String::from_utf8)?)
}

fn handle_command(cmd: Command) -> anyhow::Result<()> {
	match cmd {
		// wasm-pack, now in ~30 lines of code
		//
		// DWARF-based debugging with var value in scope, stepping etc works with chrome plugin:
		// https://chromewebstore.google.com/detail/cc++-devtools-support-dwa/pdcpmagijalfljmkmjngeonclgbbannb
		Command::BuildClient { release } => {
			if command("wasm-bindgen -V").status().is_err() {
				command("cargo install wasm-bindgen-cli -f").status()?;
			}
			if command("wasm-opt --version").status().is_err() {
				command("cargo install wasm-opt -f").status()?;
			}
			if !command_output("rustup target list --installed")?.lines().any(|x| x == "wasm32-unknown-unknown") {
				command("rustup target add wasm32-unknown-unknown").status()?;
			}

			let build_client = format!("cargo build -p client --target wasm32-unknown-unknown --target-dir target/wasm-target {}",
				if release { "--profile wasm-release" } else { "" }
			);
			command(build_client).status()?;

			let wasm_bindgen_params = if release {
				"--remove-name-section --remove-producers-section target/wasm-target/wasm32-unknown-unknown/wasm-release/client.wasm"
			// } else if shared::CONFIG.debug_wasm {
				// "--debug --keep-debug target/wasm-target/wasm32-unknown-unknown/debug/client.wasm"
			} else {
				"--debug target/wasm-target/wasm32-unknown-unknown/debug/client.wasm"
			};

			let wasm_bindgen = format!("wasm-bindgen --out-dir public/wasm --out-name code --target web --no-typescript --omit-imports --omit-default-module-path --reference-types {wasm_bindgen_params}");
			command(wasm_bindgen).status()?;

			if release {
				command("wasm-opt public/wasm/code_bg.wasm --enable-reference-types -o public/wasm/code_bg.wasm -Os").status()?;
			}
		},
		Command::BuildServer { release } => {
			command(format!("cargo build -p server{}", if release { " --release" } else { "" })).status()?;
		},
		Command::Build { release } => {
			handle_command(Command::BuildServer { release })?;

			if release {
				let current_rustflags = std::env::var("RUSTFLAGS");
				// Copt-level=s to optimize for size
				// Note that this overrides the ones in .cargo/config.toml
				std::env::set_var("RUSTFLAGS", "-Copt-level=s --cfg=web_sys_unstable_apis --cfg=getrandom_backend=\"wasm_js\" ");
				let ret = handle_command(Command::BuildClient { release });

				current_rustflags.map_or_else(|_| std::env::remove_var("RUSTFLAGS"), |x| std::env::set_var("RUSTFLAGS", x));

				ret?;
			} else {
				handle_command(Command::BuildClient { release })?;
			}
		},
		Command::WatchClient => {
			if command("watchexec --version").status().is_err() {
				command("cargo install watchexec-cli").status()?;
			}

			let to_watch = [ "client", "shared", "server/response.html" ];
			// TODO: Watchexec borken and just gives me permissoin denied :(
		},
		Command::WatchServer => todo!(),

		Command::Deploy => {
			handle_command(Command::Build { release: true })?;

			ssh_cmd("systemctl stop gremy-homepage")?;
			ssh_cmd("mkdir /home/gremious/gremy-homepage")?;
			scp_to_server("./public", "/home/gremious/gremy-homepage/")?;
			scp_to_server("./target/release/server", "/home/gremious/gremy-homepage/server")?;
			ssh_cmd("systemctl start gremy-homepage")?;
		},
		Command::ReRun => {
			handle_command(Command::BuildClient { release: false })?;
			let maybe_errors = command_output("cargo build -p client --target wasm32-unknown-unknown --target-dir target/wasm-target")?;
			if maybe_errors.lines().any(|l| l.contains("error: coult not compile")) { return Ok(()); }

			command("cargo run -p server").status()?;
		}
	};

	Ok(())
}

fn scp_to_server(from: impl Display + Sized, to: impl Display + Sized) -> io::Result<ExitStatus> {
	command(format!("scp -Cr {from} {}:{to}", *SSH)).status()
}

fn ssh_cmd(cmd: impl std::fmt::Display) -> io::Result<ExitStatus> {
	command(format!("ssh {} \"{cmd}\"", *SSH)).status()
}

