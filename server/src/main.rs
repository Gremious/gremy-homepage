#![warn(clippy::pedantic, clippy::todo)]
#![cfg_attr(not(debug_assertions), warn(
	clippy::dbg_macro,
	clippy::use_debug,
	clippy::print_stdout,
	clippy::unimplemented,
))]
#![allow(
	clippy::too_many_lines,
	clippy::missing_panics_doc,
	clippy::wildcard_imports,
)]
#![feature(unboxed_closures)]

use actix_files::NamedFile;
use actix_web::middleware::{Logger, NormalizePath, TrailingSlash};
use actix_web::{web, App, HttpResponse, HttpServer};

pub use reqwest::header;

use std::sync::LazyLock;

#[allow(clippy::unused_async)]
async fn reply() -> HttpResponse {
	static REPLY: LazyLock<String> = LazyLock::new(|| {
		let js = &std::fs::read_to_string("public/wasm/code.js").expect("couldn't find the js payload");
		let js = minifier::js::minify(js);
		let path = {
			if cfg!(debug_assertions) {
				let hostname = &format!("localhost:{}", shared::CONFIG.port);
				format!("http://{hostname}/public/wasm/code_bg.wasm")
			} else {
				let hostname = &shared::CONFIG.hostname;
				format!("https://{hostname}/public/wasm/code_bg.wasm")
			}
		};

		minify::html::minify(&format!(include_str!("../response.html"), js = js, path = path))
	});

	HttpResponse::Ok().body(&REPLY as &str)
}

pub fn init_logger() {
	env_logger::builder()
		.filter(None, log::LevelFilter::Info)
		.filter(
			Some("server"),
			#[cfg(debug_assertions)] log::LevelFilter::Trace,
			#[cfg(not(debug_assertions))] log::LevelFilter::Debug,
		)
/*
 *         .format(|buf, record| {
 *             use std::io::Write;
 *
 *             let mut dimmed = buf.default_level_style(log::Level::Trace);
 *             dimmed.fg_color(Some(env_logger::fmt::style::Color::Rgb(env_logger::fmt::style::RgbColor(126, 126, 126))));
 *
 *             let mut level_style = buf.default_level_style(log::Level::Info);
 *             match record.level() {
 *                 log::Level::Info => level_style.fg_color(Some(style::Color::Ansi(style::AnsiColor::Blue))),
 *                 log::Level::Debug => level_style.fg_color(Some(style::Color::Ansi(style::AnsiColor::Cyan))),
 *                 log::Level::Warn => level_style.fg_color(Some(style::Color::Ansi(style::AnsiColor::Yellow))),
 *                 log::Level::Error => level_style.fg_color(Some(style::Color::Ansi(style::AnsiColor::Red))),
 *                 log::Level::Trace => dimmed,
 *             };
 *
 *             writeln!(
 *                 buf,
 *                 "[{time} {level} {module} {file}:{line}] {args}",
 *                 time = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"),
 *                 level = level_style.value(record.level()),
 *                 module = record.module_path().unwrap_or("module?"),
 *                 file = dimmed.value(record.file().unwrap_or("file?")),
 *                 line = dimmed.value(record.line().unwrap_or(0)),
 *                 args = record.args(),
 *             )
 *         })
 */
	.init();
}

#[actix::main]
async fn main() -> anyhow::Result<()> {
	init_logger();

	Ok(HttpServer::new(|| {
		App::new()
			.wrap(Logger::new("%s in %Ts, %b bytes \"%r\""))
			.wrap(NormalizePath::new(TrailingSlash::Trim))
			// .wrap(cors)
			// .service(actix_files::Files::new("/public", "public").show_files_listing())
			.default_service(web::route().to(reply))
	})
	// We just go through nginx now so there's no need to do rustls on this side
	.bind(format!("0.0.0.0:{}", shared::CONFIG.port))?
	// .bind(format!("[::]:{}", CONFIG.http_port))?
	// .bind_rustls_0_23(format!("[::]:{}", shared::CONFIG.port), {
	// .bind_rustls_0_23(format!("0.0.0.0:{}", shared::CONFIG.port), {
	//     let cert_file = &mut BufReader::new(File::open(&shared::CONFIG.ssl.cert)?);
	//     let key_file = &mut BufReader::new(File::open(&shared::CONFIG.ssl.key)?);

	//     let cert_chain = certs(cert_file)
	//         .map(|k| k.context("Bad cert").unwrap())
	//         .collect::<Vec<_>>();

	//     let key = rustls_pemfile::pkcs8_private_keys(key_file)
	//         .map(|k| k.context("Bad private key").unwrap())
	//         .map(PrivateKeyDer::Pkcs8)
	//         .collect::<Vec<_>>().first().unwrap().clone_key();

	//     CryptoProvider::install_default(rustls::crypto::ring::default_provider())
	//         .expect("Failed to init ring crypto provider");

	//     rustls::ServerConfig::builder()
	//         .with_no_client_auth()
	//         .with_single_cert(cert_chain, key)?
	// })?
	.run().await?)
}
