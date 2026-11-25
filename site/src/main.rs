//! `cargo run`: the website on `http://localhost:3000` (`PORT` on Heroku). Run it from
//! `site/`, after `./build.sh` has put the interpreter in `public/` (see `README.md`).
//!
//! `cargo run -- export dist`: the whole site as static files in `dist/`, for GitHub
//! Pages. `BASE_PATH` (e.g. `/Rythme`) prefixes every link, `SITE_URL` (e.g.
//! `https://maxlestage.github.io/Rythme`) makes the absolute URLs.

fn main() -> std::io::Result<()> {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("export") => {
            let dir = args.next().unwrap_or_else(|| "dist".into());
            let base = std::env::var("BASE_PATH").unwrap_or_default();
            let site_url = std::env::var("SITE_URL").ok();
            let runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;
            let files = runtime.block_on(rythme_site::export::export(std::path::Path::new(&dir), &base, site_url))?;
            println!("{files} files written to {dir}");
            Ok(())
        }
        Some(other) => {
            eprintln!("unknown command {other:?}: run without arguments, or `export <dir>`");
            std::process::exit(2);
        }
        None => rythme_site::server::run(),
    }
}
