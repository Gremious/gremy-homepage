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
#![feature(try_blocks, async_closure, fn_traits, unboxed_closures)]

use shared::shared_prelude::*;

use actix_files::NamedFile;
use actix_web::middleware::{Logger, NormalizePath, TrailingSlash};
use actix_web::{web, App, HttpResponse, HttpServer};

pub use reqwest::header;

use std::fs::File;
use std::io::BufReader;
use rustls_pemfile::{certs, pkcs8_private_keys};

#[allow(clippy::unused_async)]
async fn reply() -> HttpResponse {
	static REPLY: Lazy<String> = Lazy::new(|| {
		let js = &std::fs::read_to_string("public/wasm/code.js").expect("couldn't find the js payload");
		let js = minifier::js::minify(js);
		let path = {
			let hostname = if shared::CONFIG.dev { &format!("localhost:{}", shared::CONFIG.port) } else { &shared::CONFIG.hostname };
			format!("https://{hostname}/public/wasm/code_bg.wasm")
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
	// use actix_cors::Cors;

	init_logger();


	Ok(HttpServer::new(|| {
		// let cors = Cors::permissive();

		App::new()
			.wrap(Logger::new("%s in %Ts, %b bytes \"%r\""))
			.wrap(NormalizePath::new(TrailingSlash::Trim))
			// .wrap(cors)
			.service(web::resource("/favicon.ico").to(async || NamedFile::open("public/img/favicon/sparkling_heart.ico")))
			.service(web::resource("/sitemap.xml").to(async || NamedFile::open("public/sitemap.xml")))
			.service(web::resource("/robots.txt").to(async || NamedFile::open("public/robots.txt")))
			.service(actix_files::Files::new("/public", "public").show_files_listing())
			.default_service(web::route().to(reply))
	})
	// .bind(format!("0.0.0.0:{}", CONFIG.http_port))?
	// .bind(format!("[::]:{}", CONFIG.http_port))?
	.bind_rustls(format!("[::]:{}", shared::CONFIG.port), {
		let cert_file = &mut BufReader::new(File::open(&shared::CONFIG.ssl.cert)?);
		let key_file = &mut BufReader::new(File::open(&shared::CONFIG.ssl.key)?);

		let cert_chain = certs(cert_file).ok().context("no certs")?.into_iter()
			.map(rustls::Certificate)
			.collect::<Vec<_>>();

		let mut keys = pkcs8_private_keys(key_file).context("no private keys")?.into_iter()
			.map(rustls::PrivateKey)
			.collect::<Vec<_>>();

		rustls::ServerConfig::builder()
			.with_cipher_suites(rustls::DEFAULT_CIPHER_SUITES)
			.with_safe_default_kx_groups()
			.with_protocol_versions(rustls::DEFAULT_VERSIONS)?
			.with_no_client_auth()
			.with_single_cert(cert_chain, keys.remove(0))?
	})?
	.run().await?)
}
