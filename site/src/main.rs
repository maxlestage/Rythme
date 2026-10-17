//! `cargo run`: the website on `http://localhost:3000` (`PORT` on Heroku). Run it from
//! `site/`, after `./build.sh` has put the interpreter in `public/` (see `README.md`).

fn main() -> std::io::Result<()> {
    rythme_site::server::run()
}
