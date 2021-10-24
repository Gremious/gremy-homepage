#![feature(try_blocks, async_closure, fn_traits, unboxed_closures)]

use std::fs::File;
use std::io::BufReader;

use actix_files::NamedFile;
use actix_web::http::ContentEncoding;
use actix_web::middleware::{Logger, *};
use actix_web::{web, App, HttpResponse, HttpServer};

pub use reqwest::header;
use rustls_pemfile::{certs, pkcs8_private_keys};

use prelude::*;

mod prelude;
mod middleware;

#[allow(clippy::format_in_format_args)]
fn reply() -> HttpResponse {
    static REPLY: Lazy<String> = Lazy::new(|| {
        let js = minifier::js::minify(&std::fs::read_to_string("public/wasm/client.js").expect("couldn't find the js payload"));

        minify::html::minify(&format!(
            include_str!("../response.html"),
            js = js,
            path = format!("{}://{}/public/wasm/client_bg.wasm", if CONFIG.https { "https" } else { "http" }, shared::CONFIG.hostname)
        ))
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
        .format(|buf, record| {
            use std::io::Write;

            let mut dimmed = buf.style();
            dimmed.set_color(env_logger::fmt::Color::Rgb(126, 126, 126));

            let mut level_style = buf.style();
            match record.level() {
                log::Level::Info => level_style.set_color(env_logger::fmt::Color::Blue),
                log::Level::Debug => level_style.set_color(env_logger::fmt::Color::Cyan),
                log::Level::Warn => level_style.set_color(env_logger::fmt::Color::Yellow),
                log::Level::Error => level_style.set_color(env_logger::fmt::Color::Red),
                log::Level::Trace => &mut dimmed,
            };

            writeln!(
                buf,
                "[{time} {level} {module} {file}:{line}] {args}",
                time = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                level = level_style.value(record.level()),
                module = record.module_path().unwrap_or("module?"),
                file = dimmed.value(record.file().unwrap_or("file?")),
                line = dimmed.value(record.line().unwrap_or(0)),
                args = record.args(),
            )
        })
        .init();
}

#[actix::main]
async fn main() -> anyhow::Result<()> {
    init_logger();

    HttpServer::new(|| {
		App::new()
			.wrap(Logger::new("%s in %Ts, %b bytes \"%r\""))
			.wrap(NormalizePath::new(TrailingSlash::Trim))
			.wrap(Compress::new(ContentEncoding::Auto))
        	.wrap(DefaultHeaders::new().header("Access-Control-Allow-Origin", "*"))
        	.wrap(middleware::redirect::RedirectToHttps)
			.service(web::resource("/sitemap.xml").to(async || NamedFile::open("public/sitemap.xml")))
			.service(web::resource("/robots.txt").to(async || NamedFile::open("public/robots.txt")))
			.service(actix_files::Files::new("/public", "public"))
			.default_service(web::route().to(reply))
	})
		.bind(format!("0.0.0.0:{}", CONFIG.http_port))?
		.bind_rustls(format!("0.0.0.0:{}", CONFIG.https_port), {
			let cert_file = &mut BufReader::new(File::open(&CONFIG.ssl.cert)?);
			let key_file = &mut BufReader::new(File::open(&CONFIG.ssl.key)?);

			let cert_chain = certs(cert_file).ok().context("no certs")?.into_iter()
				.map(rustls::Certificate)
				.collect::<Vec<_>>();

			let mut keys = pkcs8_private_keys(key_file).ok().context("no private keys")?.into_iter()
				.map(rustls::PrivateKey)
				.collect::<Vec<_>>();

			rustls::ServerConfig::builder()
				.with_cipher_suites(rustls::DEFAULT_CIPHER_SUITES)
				.with_safe_default_kx_groups()
				.with_protocol_versions(rustls::DEFAULT_VERSIONS)?
				.with_no_client_auth()
				.with_single_cert(cert_chain, keys.remove(0))?
		})?
		.run().await?
		.into_ok()
}
