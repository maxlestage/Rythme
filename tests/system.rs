//! Files, folders and the system: folders are listed, created and never
//! deleted, file information, the lines of the standard input, paths of
//! Linux, macOS and Windows, the options of the command line, exit codes.
//! Every function that touches the system gives a Result with a clear error.

mod common;

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use common::{fails, ok, rythme, Run};

/// A new empty folder for one test.
fn folder(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rythme-system-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Runs `code` in the folder `dir`, with `words` after it and `input` on the standard input.
fn run_in(dir: &Path, lang: &str, code: &str, words: &[&str], input: &str) -> Run {
    std::fs::write(dir.join("prog.ry"), code).unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_rythme"))
        .arg("prog.ry")
        .args(words)
        .current_dir(dir)
        .env("RYTHME_LANG", lang)
        .env("NO_COLOR", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("run rythme");
    child.stdin.take().unwrap().write_all(input.as_bytes()).unwrap();
    let out = child.wait_with_output().unwrap();
    Run {
        stdout: String::from_utf8_lossy(&out.stdout).replace("\r\n", "\n"),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        code: out.status.code().unwrap_or(-1),
    }
}

#[track_caller]
fn prints(dir: &Path, code: &str, expected: &str) {
    let r = run_in(dir, "en", code, &[], "");
    assert_eq!(r.code, 0, "code:\n{code}\nstderr:\n{}", r.stderr);
    assert_eq!(r.stdout.trim_end(), expected, "code:\n{code}");
}

#[test]
fn folders_are_created_listed_and_never_deleted() {
    let dir = folder("folders");
    prints(
        &dir,
        r#"print(create_folder("a/b/c"), create_folder("a/b"), list_folder("a"), list_folder("a/b/c"))"#,
        r#"Ok(()) Ok(()) Ok(["b"]) Ok([])"#,
    );
    assert!(dir.join("a/b/c").is_dir());
    std::fs::write(dir.join("a/z.txt"), "z").unwrap();
    std::fs::write(dir.join("a/B.txt"), "b").unwrap();
    std::fs::write(dir.join("a/é.txt"), "e").unwrap();
    // In order, with the files and the folders, but not what is inside the folders.
    prints(&dir, r#"print(list_folder("a"))"#, r#"Ok(["B.txt", "b", "z.txt", "é.txt"])"#);
    prints(&dir, r#"print(list_folder("."))"#, r#"Ok(["a", "prog.ry"])"#);
    prints(
        &dir,
        r#"print(list_folder("missing"), list_folder("a/z.txt"), list_folder(""))"#,
        r#"Err("folder not found: missing") Err("a/z.txt is a file, not a folder: `read_file` reads a file") Err("the path is empty: write \".\" for the current folder")"#,
    );
    prints(&dir, r#"print(create_folder("a/z.txt"))"#, r#"Err("a/z.txt already exists, and it is a file, not a folder")"#);
    // `delete_file` deletes one file, never a folder, even an empty one.
    prints(
        &dir,
        r#"print(delete_file("a/z.txt"), delete_file("a/z.txt"), delete_file("a/b/c"), delete_file("a"), delete_file(""))"#,
        r#"Ok(()) Err("file not found: a/z.txt") Err("a/b/c is a folder: `delete_file` deletes only files, never folders") Err("a is a folder: `delete_file` deletes only files, never folders") Err("the path is empty: write \".\" for the current folder")"#,
    );
    assert!(!dir.join("a/z.txt").exists() && dir.join("a/b/c").is_dir() && dir.join("a/B.txt").is_file());
    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(unix)]
#[test]
fn deleting_a_link_keeps_what_it_points_to() {
    let dir = folder("links");
    std::fs::create_dir(dir.join("kept")).unwrap();
    std::fs::write(dir.join("kept/data.txt"), "data").unwrap();
    std::os::unix::fs::symlink(dir.join("kept/data.txt"), dir.join("file-link")).unwrap();
    std::os::unix::fs::symlink(dir.join("kept"), dir.join("folder-link")).unwrap();
    prints(&dir, r#"print(delete_file("file-link"), delete_file("folder-link"))"#, "Ok(()) Ok(())");
    assert!(dir.join("kept/data.txt").is_file());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn file_info_tells_what_a_path_is() {
    let dir = folder("info");
    std::fs::write(dir.join("notes.txt"), "hello world").unwrap();
    std::fs::write(dir.join("empty.txt"), "").unwrap();
    std::fs::create_dir(dir.join("photos")).unwrap();
    prints(
        &dir,
        r#"print(file_info("notes.txt"), file_info("empty.txt"), file_info("photos"), file_info("photos/"))"#,
        "Ok({is_folder: false, size: 11}) Ok({is_folder: false, size: 0}) Ok({is_folder: true, size: 0}) Ok({is_folder: true, size: 0})",
    );
    prints(
        &dir,
        r#"print(file_info("nope.txt"), file_info(""))"#,
        r#"Err("file not found: nope.txt") Err("the path is empty: write \".\" for the current folder")"#,
    );
    prints(&dir, r#"print(file_exists("photos"), file_exists("notes.txt"), file_exists("nope"))"#, "true true false");
    // The checker knows the fields: a mistake is found before running.
    let r = run_in(&dir, "en", "info = unwrap(file_info(\"notes.txt\"))\nprint(info.size + \"bytes\")\n", &[], "");
    assert_eq!(r.code, 1, "{}", r.stderr);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn errors_are_written_in_the_language_of_the_messages() {
    let dir = folder("french");
    std::fs::create_dir(dir.join("d")).unwrap();
    let code = r#"print(list_folder("x"), file_info("x"), delete_file("d"), create_folder("prog.ry"), options(["--a", "--a"]))"#;
    let r = run_in(&dir, "fr", code, &[], "");
    assert_eq!(
        r.stdout.trim_end(),
        r#"Err("dossier introuvable : x") Err("fichier introuvable : x") Err("d est un dossier : `delete_file` supprime seulement des fichiers, jamais des dossiers") Err("prog.ry existe déjà, et c'est un fichier, pas un dossier") Err("l'option `a` est donnée deux fois")"#
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn input_lines_reads_until_the_end() {
    let dir = folder("input");
    let code = "print(input_lines())\n";
    for (input, expected) in [
        ("Ada\nAlan\n", r#"Ok(["Ada", "Alan"])"#),
        ("Ada\nAlan", r#"Ok(["Ada", "Alan"])"#),
        ("one\r\ntwo\r\n", r#"Ok(["one", "two"])"#),
        ("\n\nx\n", r#"Ok(["", "", "x"])"#),
        ("", "Ok([])"),
        ("é ü\n", r#"Ok(["é ü"])"#),
    ] {
        let r = run_in(&dir, "en", code, &[], input);
        assert_eq!(r.code, 0, "{}", r.stderr);
        assert_eq!(r.stdout.trim_end(), expected, "input {input:?}");
    }
    // After `input`, the lines that are left.
    let r = run_in(&dir, "en", "name = input()\nprint(name, input_lines())\n", &[], "Ada\nb\nc\n");
    assert_eq!(r.stdout.trim_end(), r#"Ada Ok(["b", "c"])"#);
    // Then nothing more: `input` stops, `input_lines` gives no lines.
    let r = run_in(&dir, "en", "print(input_lines(), input_lines())\ninput()\n", &[], "a\n");
    assert_eq!(r.stdout.trim_end(), r#"Ok(["a"]) Ok([])"#);
    assert!(r.stderr.contains("E017"), "{}", r.stderr);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn paths_of_linux_and_macos() {
    ok(
        r#"print(join_path(["data", "2024", "notes.txt"]), join_path(["/", "home", "ada"]), join_path(["/home/", "ada/", "x"]))"#,
        "data/2024/notes.txt /home/ada /home/ada/x",
    );
    ok(
        r#"print(join_path(["", "a", "", "b"]), join_path([]), join_path(["a"]), join_path(["/"]), join_path(["..", ".", "x"]))"#,
        "a/b  a / .././x",
    );
    ok(
        r#"print(split_path("/home/ada/notes.txt"), split_path("a//b/"), split_path(""), split_path("/"), split_path("../x"))"#,
        r#"["/", "home", "ada", "notes.txt"] ["a", "b"] [] ["/"] ["..", "x"]"#,
    );
    ok(
        r#"print(split_path("photos/cat.jpg") |> last, split_path("/Users/ada/Documents") |> join_path)"#,
        r#"Some("cat.jpg") /Users/ada/Documents"#,
    );
    ok(
        r#"print(join_path(["dossier", "été", "photo 1.jpg"]), split_path("dossier/été/photo 1.jpg"))"#,
        r#"dossier/été/photo 1.jpg ["dossier", "été", "photo 1.jpg"]"#,
    );
}

#[test]
fn paths_of_windows() {
    ok(
        r#"print(join_path(["C:\\Users", "ada", "notes.txt"]), join_path(["C:\\", "Users"]), join_path(["C:/", "Users"]))"#,
        r"C:\Users\ada\notes.txt C:\Users C:/Users",
    );
    // With both separators, `/`: Windows understands it too.
    ok(r#"print(join_path(["C:\\Users", "ada/notes.txt"]), join_path(["photos", "2024"]))"#, r"C:\Users/ada/notes.txt photos/2024");
    ok(
        r#"print(split_path("C:\\Users\\ada\\notes.txt"), split_path("C:/Users/ada"), split_path("D:notes.txt"))"#,
        r#"["C:\\", "Users", "ada", "notes.txt"] ["C:/", "Users", "ada"] ["D:", "notes.txt"]"#,
    );
    ok(
        r#"print(split_path("\\\\server\\share\\x.txt"), split_path("\\\\server\\share\\x.txt") |> join_path)"#,
        r#"["\\\\", "server", "share", "x.txt"] \\server\share\x.txt"#,
    );
    ok(r#"print(split_path("C:\\Users\\ada\\") |> join_path, split_path("a\\b/c"))"#, r#"C:\Users\ada ["a", "b", "c"]"#);
}

#[test]
fn a_path_never_leaves_its_folder_by_surprise() {
    for code in [
        r#"join_path(["data", "/etc/passwd"])"#,
        r#"join_path(["data", "\\Windows"])"#,
        r#"join_path(["data", "C:\\Windows"])"#,
        r#"join_path(["data", "c:"])"#,
    ] {
        let r = fails(code, "E023");
        assert!(r.stderr.contains("only the first part of a path can start at a root"), "{}", r.stderr);
    }
    let r = fails(r#"join_path(["photos", 2024])"#, "E023");
    assert!(
        r.stderr.contains("`join_path` needs a list of texts, but it contains Int 2024") && r.stderr.contains("text(value)"),
        "{}",
        r.stderr
    );
    fails(r#"join_path("a/b")"#, "E005");
    fails("split_path(1)", "E005");
}

#[test]
fn options_are_read_into_a_record() {
    ok(
        r#"print(options(["--dry-run", "--name=Ada", "notes.txt", "-v", "more.txt"]))"#,
        r#"Ok({args: ["notes.txt", "more.txt"], dry_run: true, name: "Ada", v: true})"#,
    );
    ok(
        r#"print(options([]), options(["-"]), options(["-5", "-0.5"]), options(["--n="]), options(["--sep=a=b"]))"#,
        r#"Ok({args: []}) Ok({args: ["-"]}) Ok({args: ["-5", "-0.5"]}) Ok({args: [], n: ""}) Ok({args: [], sep: "a=b"})"#,
    );
    ok(
        r#"print(options(["--", "--x", "-v"]), options(["--été", "-é"]))"#,
        r#"Ok({args: ["--x", "-v"]}) Ok({args: [], été: true, é: true})"#,
    );
    for (words, error) in [
        (r#"["--a", "--a=1"]"#, "the option `a` is given twice"),
        (r#"["--dry-run", "--dry_run"]"#, "the option `dry_run` is given twice"),
        (
            r#"["-abc"]"#,
            "`-abc` is not an option that Rythme understands: write `--abc` for a long name, or `-a -b` for several one-letter options",
        ),
        (r#"["--1x"]"#, "`--1x` is not a valid option"),
        (r#"["--=x"]"#, "`--=x` is not a valid option"),
        (r#"["-?"]"#, "`-?` is not a valid option"),
        (r#"["--a b"]"#, "`--a b` is not a valid option"),
        (r#"["--args"]"#, "`--args` cannot be an option: the field `args` holds the words that are not options"),
    ] {
        let r = rythme(&["-e", &format!("print(options({words}))")]);
        assert_eq!(r.code, 0, "{}", r.stderr);
        assert!(r.stdout.starts_with("Err(\"") && r.stdout.contains(error), "{words}: {}", r.stdout);
    }
    let r = fails(r#"options(["a", 1])"#, "E023");
    assert!(r.stderr.contains("`options` needs a list of texts"), "{}", r.stderr);
    fails(r#"options("--verbose")"#, "E005");
}

#[test]
fn options_come_from_the_command_line() {
    let dir = folder("options");
    let code = r#"opts = unwrap(options())
name = unwrap_or(get(opts, "name"), "World")
print("Hello, {name}!", opts.args, has(opts, "loud"), args())
"#;
    let r = run_in(&dir, "en", code, &["--name=Ada", "notes.txt", "--loud"], "");
    assert_eq!(r.stdout.trim_end(), r#"Hello, Ada! ["notes.txt"] true ["--name=Ada", "notes.txt", "--loud"]"#, "{}", r.stderr);
    let r = run_in(&dir, "en", code, &[], "");
    assert_eq!(r.stdout.trim_end(), "Hello, World! [] false []", "{}", r.stderr);
    // A mistake on the command line is an `Err`: the program explains it and chooses its exit code.
    let code =
        "match options()\n  Ok(o) -> print(o)\n  Err(e) -> do\n    print(\"usage: prog.ry [--name=NAME]: {e}\")\n    exit(2)\n  end\nend\n";
    let r = run_in(&dir, "en", code, &["-xyz"], "");
    assert_eq!(r.code, 2, "{}", r.stderr);
    assert!(r.stdout.starts_with("usage: prog.ry [--name=NAME]: `-xyz` is not an option"), "{}", r.stdout);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn exit_codes_are_the_documented_ones() {
    let dir = folder("exit");
    assert_eq!(run_in(&dir, "en", "print(1)\n", &[], "").code, 0);
    assert_eq!(run_in(&dir, "en", "print(1 / 0)\n", &[], "").code, 1);
    assert_eq!(run_in(&dir, "en", "print(\n", &[], "").code, 1);
    assert_eq!(run_in(&dir, "en", "exit(3)\n", &[], "").code, 3);
    assert_eq!(run_in(&dir, "en", "exit()\n", &[], "").code, 0);
    assert_eq!(run_in(&dir, "en", "exit(256)\n", &[], "").code, 1);
    assert_eq!(rythme(&["frobnicate"]).code, 2);
    // `rythme help system` explains them.
    let r = rythme(&["help", "system"]);
    assert!(r.stdout.contains("Exit codes") && r.stdout.contains("101") && r.stdout.contains("list_folder"), "{}", r.stdout);
    let r = common::rythme_lang("fr", &["help", "dossiers"]);
    assert!(r.stdout.contains("Codes de sortie"), "{}", r.stdout);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn names_of_other_languages_point_to_these_functions() {
    for (name, rythme_name) in [
        ("listdir", "list_folder"),
        ("mkdir", "create_folder"),
        ("makedirs", "create_folder"),
        ("unlink", "delete_file"),
        ("stat", "file_info"),
        ("path_join", "join_path"),
        ("argparse", "options"),
        ("stdin_lines", "input_lines"),
    ] {
        let r = rythme(&["-e", &format!("{name}(\"x\")")]);
        assert!(r.stderr.contains(&format!("in Rythme, this is called `{rythme_name}`")), "{name}: {}", r.stderr);
    }
}

#[test]
fn the_checker_knows_what_these_functions_give() {
    let dir = folder("check");
    for (code, error) in [
        ("x = list_folder(\".\") + 1\n", "E005"),
        ("n = len(split_path(5))\n", "E005"),
        ("x = join_path(\"a\")\n", "E005"),
        ("x = len(file_info(\"a\"))\n", "E005"),
    ] {
        std::fs::write(dir.join("prog.ry"), code).unwrap();
        let r = Command::new(env!("CARGO_BIN_EXE_rythme"))
            .args(["check", "prog.ry"])
            .current_dir(&dir)
            .env("RYTHME_LANG", "en")
            .env("NO_COLOR", "1")
            .output()
            .unwrap();
        let stderr = String::from_utf8_lossy(&r.stderr);
        assert!(stderr.contains(error), "{code}: {stderr}");
    }
    // A correct program is never rejected.
    std::fs::write(
        dir.join("prog.ry"),
        "o = unwrap(options())\nprint(o.args, get(o, \"x\"), has(o, \"verbose\"))\ninfo = file_info(\"prog.ry\")\nprint(map(info, fn(i) = i.size + 1), join_path(split_path(\"a/b\")))\nprint(map(input_lines(), len), map(list_folder(\".\"), fn(names) = len(names)))\n",
    )
    .unwrap();
    let r = Command::new(env!("CARGO_BIN_EXE_rythme"))
        .args(["check", "prog.ry"])
        .current_dir(&dir)
        .env("RYTHME_LANG", "en")
        .env("NO_COLOR", "1")
        .output()
        .unwrap();
    assert!(r.status.success() && String::from_utf8_lossy(&r.stderr).is_empty(), "{}", String::from_utf8_lossy(&r.stderr));
    let _ = std::fs::remove_dir_all(&dir);
}
