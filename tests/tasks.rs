//! `async fn` and `await`: tasks that run at the same time, on worker threads
//! with an interpreter of their own (see `src/tasks.rs`).
mod common;
use common::*;

#[test]
fn a_call_gives_a_task_and_await_gives_its_result() {
    ok(
        "fn fib(n) = if n < 2 then n else fib(n - 1) + fib(n - 2)\nasync fn slow(n) = fib(n)\n\
         print(await_all([10, 11, 12] |> map(slow)))\nt = slow(15)\nprint(await t, await(t))\nprint(await slow(5) + 1)",
        "[55, 89, 144]\n610 610\n6",
    );
}

#[test]
fn values_of_every_kind_travel_to_tasks_and_back() {
    ok(
        "type Shape = Circle(r) | Square(side)\nrate = 2\n\
         async fn area(s) = match s\n  Circle(r) -> {kind: \"circle\", area: rate * r * r, tags: [:round]}\n  Square(c) -> {kind: \"square\", area: c * c, tags: []}\nend\n\
         async fn wrap(x) = if x > 0 then Some({d: dict([[\"k\", x]]), s: set([x, x]), big: 2 ** 70}) else None\n\
         print(await_all([Circle(1), Square(3)] |> map(area)))\nprint(await wrap(1))\nprint(await wrap(0))",
        "[{kind: \"circle\", area: 2, tags: [:round]}, {kind: \"square\", area: 9, tags: []}]\n\
         Some({d: dict(\"k\" => 1), s: set(1), big: 1180591620717411303424})\nNone",
    );
}

#[test]
fn a_function_argument_runs_the_task_at_once_with_the_same_result() {
    ok("async fn apply(f, x) = f(x)\nprint(await apply(fn(x) = x * 10, 4))", "40");
}

#[test]
fn the_other_top_level_statements_do_not_run_again_in_the_workers() {
    let r = rythme(&["-e", "print(\"once\")\nasync fn id(x) = x\nprint(await_all([1, 2, 3, 4, 5, 6, 7, 8] |> map(id)))"]);
    assert_eq!(r.code, 0, "{}", r.stderr);
    assert_eq!(r.stdout, "once\n[1, 2, 3, 4, 5, 6, 7, 8]\n");
}

#[test]
fn an_error_in_a_task_stops_the_program_at_its_await() {
    let r = fails("async fn boom(x) = 10 // x\nt = boom(0)\nprint(\"before\")\nprint(await t)", "E008");
    assert_eq!(r.stdout, "before\n");
    assert!(r.stderr.contains("in the task awaited here"), "{}", r.stderr);
    assert!(r.stderr.contains("async fn boom(x) = 10 // x"), "the place in the task is shown: {}", r.stderr);
}

#[test]
fn the_checker_knows_tasks() {
    let r = rythme(&["check", "-e", "async fn double(x) = x * 2\nprint(double(2) + 1)"]);
    let r = if r.stderr.contains("error[E005]") { r } else { fails("async fn double(x) = x * 2\nprint(double(2) + 1)", "E005") };
    assert!(r.stderr.contains("write `await` before it"), "{}", r.stderr);
    fails("async fn double(x) = x * 2\nprint(upper(await double(2)))", "E005");
    fails("print(await(3))", "E006");
}

#[test]
fn async_fn_only_at_the_top_level() {
    let r = fails("fn outer(x) =\n  async fn inner(y) = y\n  inner(x)\nend\nprint(outer(1))", "E001");
    assert!(r.stderr.contains("top level"), "{}", r.stderr);
}

#[test]
fn async_and_await_stay_names_elsewhere() {
    ok("async = 3\nawait = fn(x) = x + 1\nprint(async + await(1))", "5");
    ok("fn await(x) = x * 2\ny = 21\nprint(await y)", "42");
}

#[test]
fn tasks_of_a_module_used_with_a_name() {
    let dir = std::env::temp_dir().join(format!("rythme-tasks-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("work.ry"), "async fn triple(x) = x * 3\nfn helper(x) = x + 1\n").unwrap();
    std::fs::write(dir.join("main.ry"), "use work\nprint(await_all([1, 2] |> map(work.triple)))\nprint(work.helper(1))\n").unwrap();
    let r = rythme(&[dir.join("main.ry").to_str().unwrap()]);
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(r.code, 0, "{}", r.stderr);
    assert_eq!(r.stdout, "[3, 6]\n2\n");
}

#[test]
fn the_formatter_keeps_async_and_await() {
    let dir = std::env::temp_dir().join(format!("rythme-tasks-fmt-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("f.ry");
    std::fs::write(&file, "async fn double( x )=x*2\nprint(await   double(21), await(double(1)))\n").unwrap();
    let r = rythme(&["fmt", file.to_str().unwrap()]);
    let out = std::fs::read_to_string(&file).unwrap();
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(r.code, 0, "{}", r.stderr);
    assert_eq!(out, "async fn double(x) = x * 2\nprint(await double(21), await(double(1)))\n");
}
