#![feature(try_blocks, async_closure, fn_traits, unboxed_closures, bound_cloned)]

mod prelude;

use serde::{Serialize, Deserialize};
use actix_web::{
	HttpServer,
	App,
	web,
};
use actix_web_actors::ws;
use actix::prelude::*;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::RwLock;
use prelude::*;

type Error = anyhow::Error;

fn reply() -> web::HttpResponse {
	static REPLY: Lazy<String> = Lazy::new(|| {
		let js = minifier::js::minify(&std::fs::read_to_string("public/wasm/client.js").expect("couldn't find the js payload"));

		minify::html::minify(&format!(
			include_str!("../response.html"),
			js = js,
			//todo: https
			path = format!("http://{}/public/wasm/client_bg.wasm", shared::CONFIG.hostname),
		))
	});

	web::HttpResponse::Ok().body(&REPLY as &str)
}

#[actix::main]
async fn main() -> anyhow::Result<()> {
	use rustls::internal::pemfile::{certs, pkcs8_private_keys};
	use actix_web::middleware::Logger;
	use anyhow::Context;

	env_logger::builder()
		.filter(None, log::LevelFilter::Info)
		.filter(Some("server"), #[cfg(debug_assertions)] log::LevelFilter::Trace, #[cfg(not(debug_assertions))] log::LevelFilter::Debug)
		.format(|buf, record| {
			use std::io::Write;

			let mut dimmed = buf.style();
			dimmed.set_color(env_logger::fmt::Color::Rgb(126, 126, 126));

			let mut level_style = buf.style();
			match record.level() {
				log::Level::Trace => &mut dimmed,
				log::Level::Warn => level_style.set_color(env_logger::fmt::Color::Yellow),
				log::Level::Error => level_style.set_color(env_logger::fmt::Color::Red),
				_ => &mut level_style,
			};

			writeln!(buf, "[{time} {level}  {module} {file}:{line}] {args}",
				time = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"),
				level = level_style.value(record.level()),
				module = record.module_path().unwrap_or("module?"),
				file = dimmed.value(record.file().unwrap_or("file?")),
				line = dimmed.value(record.line().unwrap_or(0)),
				args = record.args(),
			)
		})
		.init();

	let mut config = rustls::ServerConfig::new(rustls::NoClientAuth::new());
	let cert_file = &mut std::io::BufReader::new(std::fs::File::open(&CONFIG.ssl.cert).unwrap());
	let key_file = &mut std::io::BufReader::new(std::fs::File::open(&CONFIG.ssl.key).unwrap());
	let cert_chain = certs(cert_file).unwrap();
	let mut keys = pkcs8_private_keys(key_file).unwrap();
	config.set_single_cert(cert_chain, keys.remove(0)).unwrap();
	//
	// std::env::set_var("RUST_LOG", "actix_web=info");
	// env_logger::init();

	// tokio::runtime::Handle::current().block_on(new_api_token())?;

	HttpServer::new(|| {
		use actix_cors::Cors;
		use actix_web::{get, http, web, App, HttpRequest, HttpResponse, HttpServer};

		use actix_web::{http::ContentEncoding, middleware::*};
		use actix_files::{Files, NamedFile};

		let cors = Cors::permissive();
		// let cors = Cors::default()
		// 	.allowed_origin("http://localhost:8080")
		// 	.allowed_origin("https://localhost")
		// 	.allowed_headers(vec![http::header::ACCESS_CONTROL_ALLOW_ORIGIN]);
		// 	.expose_any_header();

		App::new()
			.wrap(Logger::new("%s in %Ts, %b bytes \"%r\""))
			.wrap(NormalizePath::default())
			.wrap(cors)
        	// .wrap(DefaultHeaders::new().header("Host", "gremy.co.uk"))
			// .wrap(Compress::new(ContentEncoding::Auto))
			// .service(web::resource("/ws").route(web::get().to(|r, stream: web::Payload| ws::start(MyWebSocket::new(), &r, stream))))
			// .service(web::resource("/ws").to(socket::socket))
			.service(web::resource("/sitemap.xml").to(async || NamedFile::open("public/sitemap.xml")))
			.service(web::resource("/robots.txt").to(async || NamedFile::open("public/robots.txt")))
			.service(actix_files::Files::new("/public", "public"))
			// rest
			.default_service(web::route().to(reply))
	})
		.bind(format!("0.0.0.0:{}", 80))?
		// .bind_rustls(format!("127.0.0.1:{}", CONFIG.port), config)?
		.run().await?
		.into_ok()
}
