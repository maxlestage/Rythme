//! The installers (install.sh, install.ps1) and the packaging templates,
//! tried against a fake release served by a local HTTP server.

use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(path: &str) -> String {
    std::fs::read_to_string(root().join(path)).unwrap_or_else(|e| panic!("{path}: {e}"))
}

fn temp_dir(name: &str) -> PathBuf {
    static N: AtomicUsize = AtomicUsize::new(0);
    let n = N.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!("rythme-install-{name}-{}-{n}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// The archives the release workflow builds (its `archive:` lines).
fn release_assets() -> Vec<String> {
    read(".github/workflows/release.yml").lines().filter_map(|l| l.trim().strip_prefix("archive: ")).map(|a| a.trim().to_string()).collect()
}

#[test]
fn every_release_asset_is_known_to_the_installers_and_packages() {
    let assets = release_assets();
    let mut tarballs: Vec<&str> = assets.iter().map(String::as_str).filter(|a| a.ends_with(".tar.gz")).collect();
    tarballs.sort();
    // install.sh builds the name rythme-{linux,macos}-{x86_64,arm64}.tar.gz.
    assert_eq!(
        tarballs,
        ["rythme-linux-arm64.tar.gz", "rythme-linux-x86_64.tar.gz", "rythme-macos-arm64.tar.gz", "rythme-macos-x86_64.tar.gz"]
    );
    let formula = read("packaging/homebrew/rythme.rb");
    for t in &tarballs {
        assert!(formula.contains(&format!("/{t}\"")) && formula.contains(&format!("@SHA256:{t}@")), "the Homebrew formula misses {t}");
    }
    assert!(assets.iter().any(|a| a == "rythme-windows-x86_64.zip"));
    assert!(read("install.ps1").contains("'rythme-windows-x86_64.zip'"));
    let winget = read("packaging/winget/Rythme.Rythme.installer.yaml");
    assert!(winget.contains("/rythme-windows-x86_64.zip") && winget.contains("@SHA256:rythme-windows-x86_64.zip@"));
    let workflow = read(".github/workflows/release.yml");
    assert!(workflow.contains("SHA256SUMS") && workflow.contains("packaging/render.sh"));
    for script in ["install.sh", "install.ps1"] {
        assert!(read(script).contains("SHA256SUMS"), "{script} does not check the checksums");
    }
}

// ---------------------------------------------------------------------------
// SHA-256 (FIPS 180-4), to write the fake release's SHA256SUMS.

fn sha256_hex(data: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be,
        0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa,
        0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85,
        0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3,
        0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f,
        0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];
    let mut h: [u32; 8] = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];
    let mut msg = data.to_vec();
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&((data.len() as u64) * 8).to_be_bytes());
    for block in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([block[4 * i], block[4 * i + 1], block[4 * i + 2], block[4 * i + 3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh] = h;
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ (!e & g);
            let t1 = hh.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        for (x, y) in h.iter_mut().zip([a, b, c, d, e, f, g, hh]) {
            *x = x.wrapping_add(y);
        }
    }
    h.iter().map(|x| format!("{x:08x}")).collect()
}

#[test]
fn sha256_matches_known_values() {
    assert_eq!(sha256_hex(b""), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    assert_eq!(sha256_hex(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    let long = vec![b'a'; 1000];
    assert_eq!(sha256_hex(&long), "41edece42d63e8d9bf515a9ba6932e1c20cbc9f5a5d134645adb5db1b9737ea3");
}

// ---------------------------------------------------------------------------
// A fake GitHub release: <root>/latest/download/<file> and
// <root>/download/<tag>/<file>, served over HTTP on 127.0.0.1.

/// Serves the files under `dir` and returns the base URL.
fn serve(dir: PathBuf) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    std::thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            let mut reader = BufReader::new(&stream);
            let mut request = String::new();
            if reader.read_line(&mut request).is_err() {
                continue;
            }
            loop {
                let mut line = String::new();
                if reader.read_line(&mut line).unwrap_or(0) == 0 || line == "\r\n" {
                    break;
                }
            }
            let path = request.split_whitespace().nth(1).unwrap_or("/").split('?').next().unwrap_or("/").to_string();
            let file = dir.join(path.trim_start_matches('/'));
            let mut out = &stream;
            match std::fs::read(&file) {
                Ok(body) if !path.contains("..") => {
                    // Scripts as raw.githubusercontent.com serves them.
                    let kind = if path.ends_with(".ps1") || path.ends_with(".sh") {
                        "text/plain; charset=utf-8"
                    } else {
                        "application/octet-stream"
                    };
                    let _ = write!(
                        out,
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {kind}\r\nConnection: close\r\n\r\n",
                        body.len()
                    );
                    let _ = out.write_all(&body);
                }
                _ => {
                    let _ = write!(out, "HTTP/1.1 404 Not Found\r\nContent-Length: 9\r\nConnection: close\r\n\r\nnot found");
                }
            }
            let _ = out.flush();
        }
    });
    format!("http://{addr}")
}

/// Writes `files` (name, content) with their SHA256SUMS in both the latest
/// release and the release `tag`; `corrupt` makes one checksum wrong.
fn publish(release: &Path, tag: &str, files: &[(String, Vec<u8>)], corrupt: Option<&str>) {
    let mut sums = String::new();
    for (name, body) in files {
        let mut sum = sha256_hex(body);
        if corrupt == Some(name.as_str()) {
            sum = sha256_hex(b"something else");
        }
        // The format of sha256sum: two spaces, or a star for binary mode.
        sums.push_str(&format!("{sum}  {name}\n"));
    }
    for dir in [release.join("latest/download"), release.join("download").join(tag)] {
        std::fs::create_dir_all(&dir).unwrap();
        for (name, body) in files {
            std::fs::write(dir.join(name), body).unwrap();
        }
        std::fs::write(dir.join("SHA256SUMS"), &sums).unwrap();
    }
}

fn run(cmd: &mut Command) -> (i32, String, String) {
    let out: Output = cmd.output().expect("run the installer");
    (out.status.code().unwrap_or(-1), String::from_utf8_lossy(&out.stdout).into_owned(), String::from_utf8_lossy(&out.stderr).into_owned())
}

fn version_line() -> String {
    format!("rythme {}", env!("CARGO_PKG_VERSION"))
}

// ---------------------------------------------------------------------------
// install.sh (Linux, macOS)

#[cfg(unix)]
mod unix {
    use super::*;

    /// The archive install.sh must choose on this computer.
    fn host_asset() -> Option<&'static str> {
        match (std::env::consts::OS, std::env::consts::ARCH) {
            ("linux", "x86_64") => Some("rythme-linux-x86_64.tar.gz"),
            ("linux", "aarch64") => Some("rythme-linux-arm64.tar.gz"),
            ("macos", "x86_64") => Some("rythme-macos-x86_64.tar.gz"),
            ("macos", "aarch64") => Some("rythme-macos-arm64.tar.gz"),
            _ => None,
        }
    }

    /// A tar.gz holding the real `rythme` program, as the release workflow makes it.
    fn tarball(work: &Path) -> Vec<u8> {
        let stage = work.join("stage");
        std::fs::create_dir_all(&stage).unwrap();
        std::fs::copy(env!("CARGO_BIN_EXE_rythme"), stage.join("rythme")).unwrap();
        let archive = work.join("rythme.tar.gz");
        let status = Command::new("tar").arg("-czf").arg(&archive).arg("-C").arg(&stage).arg("rythme").status().unwrap();
        assert!(status.success());
        std::fs::read(archive).unwrap()
    }

    /// A release with every archive; only this computer's one is real.
    fn release_files(work: &Path, host: &str) -> Vec<(String, Vec<u8>)> {
        let real = tarball(work);
        super::release_assets()
            .into_iter()
            .map(|a| if a == host { (a, real.clone()) } else { (a.clone(), format!("not {a}").into_bytes()) })
            .collect()
    }

    fn install_sh(base: &str, home: &Path, lang: &str, args: &[&str]) -> (i32, String, String) {
        run(Command::new("sh")
            .arg(root().join("install.sh"))
            .args(args)
            .env("RYTHME_DOWNLOAD_URL", base)
            .env("HOME", home)
            .env("LANG", lang)
            .env_remove("LC_ALL")
            .env_remove("LC_MESSAGES")
            .env_remove("RYTHME_LANG")
            .env_remove("RYTHME_INSTALL_DIR")
            .env_remove("RYTHME_VERSION")
            .env("NO_PROXY", "127.0.0.1,localhost")
            .env("no_proxy", "127.0.0.1,localhost"))
    }

    #[test]
    fn install_sh_installs_the_latest_or_a_given_release_after_checking_it() {
        let Some(host) = host_asset() else {
            eprintln!("no release archive for this system: nothing to install");
            return;
        };
        let work = temp_dir("sh");
        let release = work.join("release");
        publish(&release, "v9.9.9", &release_files(&work, host), None);
        std::fs::copy(root().join("install.sh"), release.join("install.sh")).unwrap();
        let base = serve(release);
        let home = work.join("home");
        std::fs::create_dir_all(&home).unwrap();

        // The latest release, in ~/.local/bin by default, with English messages.
        let (code, _, err) = install_sh(&base, &home, "en_US.UTF-8", &[]);
        assert_eq!(code, 0, "{err}");
        let program = home.join(".local/bin/rythme");
        let out = Command::new(&program).arg("--version").output().unwrap();
        assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), version_line());
        assert!(err.contains(&format!("Downloading {host} (latest)")), "{err}");
        assert!(err.contains("Checksum verified (SHA-256)."), "{err}");
        assert!(err.contains("is not in your PATH") && err.contains("export PATH="), "{err}");
        let out = Command::new(&program).args(["-e", "print(1 + 2)"]).output().unwrap();
        assert_eq!(String::from_utf8_lossy(&out.stdout), "3\n");

        // A given release, in a given folder (installed again over itself), in French.
        let dir = work.join("bin with space");
        let dir_text = dir.to_str().unwrap();
        for _ in 0..2 {
            let (code, _, err) = install_sh(&base, &home, "fr_FR.UTF-8", &["--dir", dir_text, "--version", "9.9.9"]);
            assert_eq!(code, 0, "{err}");
            assert!(err.contains(&format!("Téléchargement de {host} (v9.9.9)")), "{err}");
            assert!(err.contains("Somme de contrôle vérifiée (SHA-256).") && err.contains("installé dans"), "{err}");
            assert!(err.contains("n'est pas dans votre PATH"), "{err}");
        }
        assert!(dir.join("rythme").is_file());
        assert!(!dir.join(".rythme.new").exists());

        // A folder already in the PATH: no advice about the PATH.
        let (code, _, err) = run(Command::new("sh")
            .arg(root().join("install.sh"))
            .arg(format!("--dir={dir_text}"))
            .env("RYTHME_DOWNLOAD_URL", &base)
            .env("RYTHME_LANG", "en")
            .env("PATH", format!("{dir_text}:{}", std::env::var("PATH").unwrap_or_default()))
            .env("NO_PROXY", "127.0.0.1,localhost")
            .env("no_proxy", "127.0.0.1,localhost"));
        assert_eq!(code, 0, "{err}");
        assert!(err.contains("Try it: rythme learn") && !err.contains("not in your PATH"), "{err}");

        // The way the README says: `curl ... | sh -s -- --dir ...`.
        let piped = work.join("piped");
        let (code, _, err) = run(Command::new("sh")
            .arg("-c")
            .arg(format!("curl -fsSL {base}/install.sh | sh -s -- --dir '{}'", piped.display()))
            .env("RYTHME_DOWNLOAD_URL", &base)
            .env("RYTHME_LANG", "en")
            .env("NO_PROXY", "127.0.0.1,localhost")
            .env("no_proxy", "127.0.0.1,localhost"));
        assert_eq!(code, 0, "{err}");
        assert!(err.contains("Checksum verified (SHA-256).") && piped.join("rythme").is_file(), "{err}");

        // A release that does not exist.
        let (code, _, err) = install_sh(&base, &home, "C", &["--version", "v0.0.0", "--dir", work.join("none").to_str().unwrap()]);
        assert_ne!(code, 0);
        assert!(err.contains("error: could not download"), "{err}");
        assert!(!work.join("none/rythme").exists());

        // Help and wrong options.
        let (code, _, err) = install_sh(&base, &home, "fr_FR", &["--help"]);
        assert_eq!(code, 0);
        assert!(err.contains("Utilisation : install.sh"), "{err}");
        let (code, _, err) = install_sh(&base, &home, "en", &["--colour"]);
        assert_ne!(code, 0);
        assert!(err.contains("error: unknown option: --colour"), "{err}");
        let _ = std::fs::remove_dir_all(&work);
    }

    #[test]
    fn install_sh_refuses_an_archive_whose_checksum_is_wrong() {
        let Some(host) = host_asset() else {
            return;
        };
        let work = temp_dir("sh-bad");
        let release = work.join("release");
        publish(&release, "v9.9.9", &release_files(&work, host), Some(host));
        let base = serve(release);
        let dir = work.join("bin");
        for (lang, message) in [("en_GB.UTF-8", "error: the SHA-256 checksum of"), ("fr_CA.UTF-8", "erreur : la somme SHA-256 de")] {
            let (code, _, err) = install_sh(&base, &work, lang, &["--dir", dir.to_str().unwrap()]);
            assert_ne!(code, 0, "{err}");
            assert!(err.contains(message), "{err}");
            assert!(!dir.join("rythme").exists(), "installed despite a wrong checksum");
        }

        // An archive missing from SHA256SUMS is refused too.
        let release = work.join("release2");
        publish(&release, "v9.9.9", &release_files(&work, host), None);
        let sums = release.join("latest/download/SHA256SUMS");
        let kept: String =
            std::fs::read_to_string(&sums).unwrap().lines().filter(|l| !l.ends_with(host)).map(|l| format!("{l}\n")).collect();
        std::fs::write(&sums, kept).unwrap();
        let (code, _, err) = install_sh(&serve(release), &work, "en", &["--dir", dir.to_str().unwrap()]);
        assert_ne!(code, 0);
        assert!(err.contains(&format!("{host} is not listed in SHA256SUMS")), "{err}");
        assert!(!dir.join("rythme").exists());
        let _ = std::fs::remove_dir_all(&work);
    }

    #[test]
    fn render_fills_in_the_homebrew_formula_and_the_winget_manifests() {
        let work = temp_dir("render");
        let files: Vec<(String, Vec<u8>)> = super::release_assets().into_iter().map(|a| (a.clone(), a.into_bytes())).collect();
        let mut sums = String::new();
        for (name, body) in &files {
            sums.push_str(&format!("{}  {name}\n", sha256_hex(body)));
        }
        std::fs::write(work.join("SHA256SUMS"), &sums).unwrap();
        let out = work.join("out");
        let (code, _, err) = run(Command::new("sh")
            .arg(root().join("packaging/render.sh"))
            .arg("v1.2.3")
            .arg(work.join("SHA256SUMS"))
            .arg(&out)
            .env("RELEASE_DATE", "2026-01-02"));
        assert_eq!(code, 0, "{err}");
        let formula = std::fs::read_to_string(out.join("rythme.rb")).unwrap();
        assert!(formula.contains("version \"1.2.3\""));
        assert!(formula.contains("releases/download/v1.2.3/rythme-macos-arm64.tar.gz"));
        assert!(formula.contains(&format!("sha256 \"{}\"", sha256_hex(b"rythme-linux-arm64.tar.gz"))));
        assert!(!formula.contains('@'), "{formula}");
        let installer = std::fs::read_to_string(out.join("winget/Rythme.Rythme.installer.yaml")).unwrap();
        assert!(installer.contains(&format!("InstallerSha256: \"{}\"", sha256_hex(b"rythme-windows-x86_64.zip"))));
        assert!(installer.contains("ReleaseDate: \"2026-01-02\"") && installer.contains("PackageVersion: \"1.2.3\""));
        for name in ["Rythme.Rythme.yaml", "Rythme.Rythme.locale.en-US.yaml", "Rythme.Rythme.locale.fr-FR.yaml"] {
            let text = std::fs::read_to_string(out.join("winget").join(name)).unwrap();
            assert!(text.contains("PackageVersion: \"1.2.3\"") && !text.contains("@VERSION@"), "{name}");
        }

        // A checksum missing from SHA256SUMS: the render fails.
        let partial: String = sums.lines().filter(|l| !l.contains("windows")).map(|l| format!("{l}\n")).collect();
        std::fs::write(work.join("partial"), partial).unwrap();
        let (code, _, err) =
            run(Command::new("sh").arg(root().join("packaging/render.sh")).arg("1.2.3").arg(work.join("partial")).arg(work.join("out2")));
        assert_ne!(code, 0);
        assert!(err.contains("@SHA256:rythme-windows-x86_64.zip@"), "{err}");
        let _ = std::fs::remove_dir_all(&work);
    }
}

// ---------------------------------------------------------------------------
// install.ps1 (Windows; also tried with PowerShell 7 on Linux and macOS when
// `pwsh` is there, or the program named by RYTHME_PWSH).

fn powershell() -> Option<String> {
    let mut candidates: Vec<String> = std::env::var("RYTHME_PWSH").into_iter().collect();
    candidates.push("pwsh".into());
    if cfg!(windows) {
        candidates.push("powershell".into());
    }
    candidates.into_iter().find(|p| Command::new(p).args(["-NoProfile", "-Command", "exit 0"]).output().is_ok_and(|o| o.status.success()))
}

#[test]
fn install_ps1_installs_after_checking_the_checksum() {
    let Some(ps) = powershell() else {
        eprintln!("PowerShell not found: install.ps1 is tried by the CI (set RYTHME_PWSH to try it here)");
        return;
    };
    let work = temp_dir("ps1");
    // The zip of the release workflow: rythme.exe at the root.
    let stage = work.join("stage");
    std::fs::create_dir_all(&stage).unwrap();
    std::fs::copy(env!("CARGO_BIN_EXE_rythme"), stage.join("rythme.exe")).unwrap();
    let zip = work.join("rythme.zip");
    let (code, _, err) = run(Command::new(&ps).args(["-NoProfile", "-NonInteractive", "-Command"]).arg(format!(
        "Compress-Archive -Path '{}' -DestinationPath '{}'",
        stage.join("rythme.exe").display(),
        zip.display()
    )));
    assert_eq!(code, 0, "{err}");
    let real = std::fs::read(&zip).unwrap();
    let files: Vec<(String, Vec<u8>)> = release_assets()
        .into_iter()
        .map(|a| if a == "rythme-windows-x86_64.zip" { (a, real.clone()) } else { (a.clone(), a.into_bytes()) })
        .collect();

    let good = work.join("good");
    publish(&good, "v9.9.9", &files, None);
    let bad = work.join("bad");
    publish(&bad, "v9.9.9", &files, Some("rythme-windows-x86_64.zip"));
    let (good, bad) = (serve(good), serve(bad));

    let install = |base: &str, lang: &str, dir: &Path, version: Option<&str>| {
        let mut cmd = Command::new(&ps);
        cmd.args(["-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-File"])
            .arg(root().join("install.ps1"))
            .arg("-Dir")
            .arg(dir)
            .arg("-NoModifyPath")
            .env("RYTHME_DOWNLOAD_URL", base)
            .env("RYTHME_LANG", lang)
            .env_remove("RYTHME_VERSION")
            .env_remove("RYTHME_INSTALL_DIR")
            .env("NO_PROXY", "127.0.0.1,localhost")
            .env("no_proxy", "127.0.0.1,localhost");
        if let Some(v) = version {
            cmd.arg("-Version").arg(v);
        }
        run(&mut cmd)
    };

    let dir = work.join("bin");
    let (code, out, err) = install(&good, "en", &dir, None);
    assert_eq!(code, 0, "{out}{err}");
    assert!(out.contains("Downloading rythme-windows-x86_64.zip (latest)") && out.contains("Checksum verified (SHA-256)."), "{out}");
    assert!(out.contains(&format!("Installed {}", version_line())), "{out}");
    let exe = dir.join("rythme.exe");
    let o = Command::new(&exe).arg("--version").output().unwrap();
    assert_eq!(String::from_utf8_lossy(&o.stdout).trim(), version_line());

    // Again over itself, a given version, in French.
    let (code, out, err) = install(&good, "fr", &dir, Some("9.9.9"));
    assert_eq!(code, 0, "{out}{err}");
    assert!(out.contains("Téléchargement de rythme-windows-x86_64.zip (v9.9.9)") && out.contains("installé dans"), "{out}");
    assert!(!dir.join("rythme.exe.old").exists());

    // A wrong checksum: refused, nothing installed.
    let other = work.join("other");
    for (lang, message) in [
        ("en", "the SHA-256 checksum of rythme-windows-x86_64.zip is wrong"),
        ("fr", "la somme SHA-256 de rythme-windows-x86_64.zip est fausse"),
    ] {
        let (code, out, err) = install(&bad, lang, &other, None);
        assert_ne!(code, 0, "{out}{err}");
        assert!(err.contains(message), "{out}{err}");
        assert!(!other.join("rythme.exe").exists());
    }

    // A release that does not exist.
    let (code, out, err) = install(&good, "en", &other, Some("v0.0.0"));
    assert_ne!(code, 0);
    assert!(err.contains("error: could not download"), "{out}{err}");

    // The way the README says: `irm .../install.ps1 | iex`, set up by variables.
    let good_root = work.join("good");
    std::fs::copy(root().join("install.ps1"), good_root.join("install.ps1")).unwrap();
    let iex = work.join("iex");
    let (code, out, err) = run(Command::new(&ps)
        .args(["-NoProfile", "-NonInteractive", "-Command"])
        .arg(format!("irm {good}/install.ps1 | iex"))
        .env("RYTHME_DOWNLOAD_URL", &good)
        .env("RYTHME_INSTALL_DIR", &iex)
        .env("RYTHME_NO_MODIFY_PATH", "1")
        .env("RYTHME_LANG", "fr")
        .env_remove("RYTHME_VERSION")
        .env("NO_PROXY", "127.0.0.1,localhost")
        .env("no_proxy", "127.0.0.1,localhost"));
    assert_eq!(code, 0, "{out}{err}");
    assert!(out.contains("Somme de contrôle vérifiée (SHA-256).") && out.contains("installé dans"), "{out}{err}");
    assert!(iex.join("rythme.exe").is_file());
    let _ = std::fs::remove_dir_all(&work);
}
