//! Numbers for machine learning: vectors and matrices as lists, statistics,
//! chance that follows `random_seed`. Every mistake of shape names both
//! shapes, and no input (empty, ragged, `nan`, deeply nested) makes them crash.

mod common;

use common::{fails, ok, rythme};

#[test]
fn math_of_neurons() {
    ok("print(tanh(0), sigmoid(0), relu(-1), relu(2), relu(0.5))", "0.0 0.5 0.0 2.0 0.5");
    // No overflow far from 0.
    ok("print(sigmoid(1000), sigmoid(-1000), tanh(1000))", "1.0 0.0 1.0");
    ok("nan = float(\"nan\")\nprint(sigmoid(nan), relu(nan))", "nan nan");
}

#[test]
fn vectors() {
    ok("print(dot([1, 2, 3], [4, 5, 6]), norm([3, 4]), norm([]), dot([], []))", "32.0 5.0 0.0 0.0");
    ok(
        "print(vector_add([1, 2], [3, 4]), vector_sub([1, 2], [3, 4]), vector_mul([1, 2], [3, 4]), vector_scale([1, 2], 3))",
        "[4.0, 6.0] [-2.0, -2.0] [3.0, 8.0] [3.0, 6.0]",
    );
    ok("print(vector_add([[1, 2]], [[3, 4]]), vector_scale([[1], [2]], -1), vector_add([], []))", "[[4.0, 6.0]] [[-1.0], [-2.0]] []");
    ok("print(zip_with([1, 2], [\"a\", \"b\"], fn(n, s) = repeat(s, n)))", "[\"a\", \"bb\"]");
    // Whole numbers of any size are numbers too.
    ok("print(dot([10 ** 30], [1]))", "1e30");
}

#[test]
fn mistakes_of_shape_name_both_shapes() {
    let r = fails("dot([1, 2], [3])", "E023");
    assert!(r.stderr.contains("a vector of 2 number(s) and a vector of 1"), "{}", r.stderr);
    let r = fails("vector_add([1, 2], [[1], [2]])", "E023");
    assert!(r.stderr.contains("but got a vector of 2 number(s) and a 2×1 matrix"), "{}", r.stderr);
    let r = fails("vector_sub([[1, 2], [3, 4]], [[1, 2]])", "E023");
    assert!(r.stderr.contains("a 2×2 matrix and a 1×2 matrix"), "{}", r.stderr);
    let r = fails("matmul([[1, 2], [3, 4]], [[1, 2, 3]])", "E023");
    assert!(r.stderr.contains("cannot multiply a 2×2 matrix by a 1×3 matrix"), "{}", r.stderr);
    let r = fails("matmul([[1, 2, 3]], [1, 2])", "E023");
    assert!(r.stderr.contains("a 1×3 matrix by a vector of 2 number(s)"), "{}", r.stderr);
    let r = fails("transpose([[1, 2], [3]])", "E023");
    assert!(r.stderr.contains("its row 0 has 2 number(s), but its row 1 has 1"), "{}", r.stderr);
    let r = fails("shape([[1, 2], [3]])", "E023");
    assert!(r.stderr.contains("element 0 is a vector of 2 number(s), element 1 is a vector of 1 number(s)"), "{}", r.stderr);
    let r = fails("zip_with([1], [], fn(a, b) = a)", "E023");
    assert!(r.stderr.contains("got 1 and 0 element(s)"), "{}", r.stderr);
    // What is not a number is named, with its place.
    let r = fails("dot([1, \"2\"], [1, 2])", "E005");
    assert!(r.stderr.contains("its element 1 is Text \"2\"") && r.stderr.contains("parse_float"), "{}", r.stderr);
    let r = fails("transpose([[1], 2])", "E005");
    assert!(r.stderr.contains("its row 1 is Int 2"), "{}", r.stderr);
    let r = fails("matmul([[1, true]], [[1], [2]])", "E005");
    assert!(r.stderr.contains("its row 0 holds Bool true at position 1"), "{}", r.stderr);
    fails("vector_scale([[1], [\"x\"]], 2)", "E005");
    fails("vector_add(\"ab\", \"cd\")", "E005");
}

#[test]
fn matrices() {
    ok("print(matmul([[1, 2], [3, 4]], [[5, 6], [7, 8]]))", "[[19.0, 22.0], [43.0, 50.0]]");
    ok("print(matmul([[1, 2], [3, 4]], [1, 1]), matmul(identity_matrix(2), [[2, 3], [4, 5]]))", "[3.0, 7.0] [[2.0, 3.0], [4.0, 5.0]]");
    ok("print(transpose([[1, 2, 3]]), transpose([]), matmul([], []))", "[[1.0], [2.0], [3.0]] [] []");
    ok("print(zeros(2), ones(1, 2), zeros(0, 3), zeros(2, 0), identity_matrix(0))", "[0.0, 0.0] [[1.0, 1.0]] [] [[], []] []");
    ok(
        "print(shape([1, 2]), shape([[1, 2, 3], [4, 5, 6]]), shape([]), shape([[]]), shape([[[1], [2]]]))",
        "[2] [2, 3] [0] [1, 0] [1, 2, 1]",
    );
    fails("zeros(-1)", "E023");
    fails("zeros(100000, 100000)", "E023");
    fails("identity_matrix(10000)", "E023");
}

#[test]
fn statistics() {
    ok("print(stats([1, 2, 3, 4]).mean, variance([2, 4, 4, 4, 5, 5, 7, 9]), std_dev([2, 4, 4, 4, 5, 5, 7, 9]))", "2.5 4.0 2.0");
    ok("print(percentile([5, 1, 3], 50), percentile([1, 2, 3, 4], 25), percentile([7], 90), percentile([1, 2], 0))", "3.0 1.75 7.0 1.0");
    ok("print(argmax([1, 5, 5]), argmin([4, 2, 2]), argmax([-1]))", "1 1 0");
    ok(
        "print(softmax([]), softmax([1000, 1000]), normalize([]), normalize([3, 3]), normalize([0, 5, 10]))",
        "[] [0.5, 0.5] [] [0.0, 0.0] [0.0, 0.5, 1.0]",
    );
    ok("print(stats([5, 1, 3]))", "{count: 3, mean: 3.0, median: 3.0, std_dev: 1.632993161855452, min: 1.0, max: 5.0}");
    let r = fails("stats([])", "E023");
    assert!(r.stderr.contains("`stats` of an empty list"), "{}", r.stderr);
    fails("argmax([])", "E023");
    fails("percentile([1], 101)", "E023");
    fails("percentile([], 50)", "E023");
    // `nan` never wins, and never crashes a sort.
    ok("nan = float(\"nan\")\nprint(argmax([nan, 1, 3]), argmin([nan, nan]), percentile([nan, 1, 2], 0))", "2 0 1.0");
    ok("inf = float(\"inf\")\nprint(softmax([inf, 1]), normalize([inf, 1]))", "[1.0, 0.0] [0.0, 0.0]");
}

#[test]
fn chance_follows_the_seed() {
    let code = "random_seed(3)\nprint(random_normal(10, 2), train_test_split(1..10, 0.7), random_choice_weighted([:a, :b, :c], [1, 0, 3]))";
    let a = rythme(&["-e", code]);
    let b = rythme(&["-e", code]);
    assert_eq!(a.code, 0, "{}", a.stderr);
    assert_eq!(a.stdout, b.stdout);
    ok("random_seed(1)\nsplit = train_test_split(1..10, 0.8)\nprint(len(split.train), len(split.test), sort(split.train + split.test) == 1..10)", "8 2 true");
    ok("print(train_test_split([], 0.5), random_normal(5, 0))", "{train: [], test: []} 5.0");
    ok("print(random_choice_weighted([1, 2, 3], [0, 0, 1]))", "3");
    // Many draws: the mean and the spread of the bell curve.
    ok(
        "random_seed(42)\nxs = map(1..20000, fn(_) = random_normal(3, 2))\nprint(round(stats(xs).mean, 1), round(std_dev(xs), 1))",
        "3.0 2.0",
    );
    ok("random_seed(7)\nxs = map(1..10000, fn(_) = random_choice_weighted([:a, :b], [1, 3]))\nprint(round(count(xs, fn(x) = x == :b) / 10000, 1))", "0.8");
    fails("random_normal(0, -1)", "E023");
    fails("random_choice_weighted([1, 2], [1])", "E023");
    fails("random_choice_weighted([1, 2], [0, 0])", "E023");
    fails("random_choice_weighted([1, 2], [-1, 2])", "E023");
    fails("train_test_split([1], 2)", "E023");
}

#[test]
fn deep_and_large_values_are_refused_without_crashing() {
    let deep = format!("{}1{}", "[".repeat(200), "]".repeat(200));
    fails(&format!("vector_add({deep}, {deep})"), "E023");
    fails(&format!("shape({deep})"), "E023");
    fails(&format!("vector_scale({deep}, 2)"), "E023");
    // 100 000 × 1 times 1 × 100 000: ten billion numbers.
    fails("m = transpose([1..100000])\nmatmul(m, transpose(m))", "E023");
}

#[test]
fn the_checker_knows_them() {
    let r = rythme(&["-e", "x = dot(1, [2])"]);
    assert!(r.stderr.contains("error[E005]") && r.stderr.contains("`dot` expects a list as argument 1, but got Int 1"), "{}", r.stderr);
    let r = rythme(&["-e", "x = variance([1, 2]) + \"a\""]);
    assert!(r.stderr.contains("error[E005]"), "{}", r.stderr);
}

// ----- slopes -------------------------------------------------------------------

#[test]
fn derivatives_keep_their_sure_digits() {
    ok(
        "print(derivative(fn(x) = x * x, 3), derivative(sin, 0), derivative(exp, 1), derivative(fn(x) = x * x * x, 2), derivative(sqrt, 4))",
        "6.0 1.0 2.718281829 12.0 0.25",
    );
    // At a minimum, at a corner, for a constant, and for tiny or huge slopes.
    ok(
        "print(derivative(fn(x) = x * x - 4 * x, 2), derivative(abs, 0), derivative(fn(x) = 5, 3), derivative(fn(x) = 1e-12 * x, 0), derivative(fn(x) = 1e12 * x * x, 1))",
        "0.0 0.0 0.0 1e-12 2000000000000.0",
    );
    ok("print(derivative(sigmoid, 0), derivative(cos, 1), derivative(fn(x) = x * x, 0.1))", "0.25 -0.8414709848 0.2");
}

#[test]
fn gradients() {
    ok(
        "print(gradient(fn([x, y]) = x * x + 3 * y, [2, 5]), gradient(fn(v) = dot(v, v), [1, 2, 3]), gradient(fn(v) = 1, []))",
        "[4.0, 3.0] [2.0, 4.0, 6.0] []",
    );
    // Gradient descent on (x - 3)² + (y + 1)²: down to the minimum (3, -1).
    ok(
        "fn loss([x, y]) = (x - 3) ** 2 + (y + 1) ** 2\nfn descend(p, 0) = p\nfn descend(p, n) = descend(vector_sub(p, vector_scale(gradient(loss, p), 0.1)), n - 1)\nprint(map(descend([0, 0], 100), fn(v) = round(v, 6)))",
        "[3.0, -1.0]",
    );
}

#[test]
fn mistakes_with_slopes() {
    let r = fails("derivative(fn(x) = \"a\", 1)", "E005");
    assert!(r.stderr.contains("the function given to `derivative` must return a number, but it returned Text \"a\""), "{}", r.stderr);
    fails("derivative(fn(x, y) = x, 0)", "E004");
    fails("gradient(fn(v) = v, [1])", "E005");
    fails("gradient(fn(v) = 0, [\"a\"])", "E005");
    // The function's own errors stay its own: sqrt below 0.
    fails("derivative(sqrt, 0)", "E023");
}
