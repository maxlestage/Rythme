//! The programs of `examples/`, built into the site with what they print (the `.out`
//! files that `cargo test` checks at the root of the repository).

use crate::i18n::{COUNT, Lang};

pub struct Example {
    /// The file name without `.ry`, also the anchor on the examples page.
    pub name: &'static str,
    pub code: &'static str,
    pub output: &'static str,
    /// What the example shows, in every language (`Lang::pick`).
    about: [&'static str; COUNT],
}

impl Example {
    pub fn about(&self, lang: Lang) -> &'static str {
        lang.pick(self.about)
    }

    /// Run with `rythme test` in the playground.
    pub fn is_test(&self) -> bool {
        self.name == "testing"
    }
}

/// `example!(name, [fr, en, es, de, pt, zh])`.
macro_rules! example {
    ($name:literal, $about:expr) => {
        Example {
            name: $name,
            code: include_str!(concat!("../../examples/", $name, ".ry")),
            output: include_str!(concat!("../../examples/", $name, ".out")),
            about: $about,
        }
    };
}

/// In the order of the README.
pub const EXAMPLES: &[Example] = &[
    example!(
        "hello",
        [
            "le premier programme classique",
            "the classic first program",
            "el clásico primer programa",
            "das klassische erste Programm",
            "o clássico primeiro programa",
            "经典的第一个程序",
        ]
    ),
    example!(
        "fizzbuzz",
        [
            "match sur une liste, intervalles, each",
            "match on a list, ranges, each",
            "match sobre una lista, rangos, each",
            "match auf einer Liste, Bereiche, each",
            "match em uma lista, intervalos, each",
            "对列表做 match、区间、each",
        ]
    ),
    example!(
        "fibonacci",
        [
            "définitions par cas, appels terminaux, documentation ##",
            "definitions by case, tail calls, ## docs",
            "definiciones por casos, llamadas de cola, documentación ##",
            "Definitionen pro Fall, Endaufrufe, ##-Dokumentation",
            "definições por caso, chamadas de cauda, documentação ##",
            "按情况定义、尾调用、## 文档",
        ]
    ),
    example!(
        "shapes",
        [
            "vos propres types, gardes, sort_by",
            "your own types, guards, sort_by",
            "tus propios tipos, guardas, sort_by",
            "eigene Typen, Wächter, sort_by",
            "seus próprios tipos, guardas, sort_by",
            "自定义类型、守卫、sort_by",
        ]
    ),
    example!(
        "quicksort",
        [
            "motifs de listes, filter, + sur les listes",
            "list patterns, filter, + on lists",
            "patrones de listas, filter, + sobre listas",
            "Listenmuster, filter, + auf Listen",
            "padrões de listas, filter, + em listas",
            "列表模式、filter、列表上的 +",
        ]
    ),
    example!(
        "words",
        [
            "traitement de texte, group_by, enregistrements",
            "text processing, group_by, records",
            "procesamiento de texto, group_by, registros",
            "Textverarbeitung, group_by, Records",
            "processamento de texto, group_by, registros",
            "文本处理、group_by、记录",
        ]
    ),
    example!(
        "cipher",
        [
            "le chiffre de César : char_code, char, compter les lettres",
            "the Caesar cipher: char_code, char, counting letters",
            "el cifrado César: char_code, char, contar letras",
            "die Caesar-Verschlüsselung: char_code, char, Buchstaben zählen",
            "a cifra de César: char_code, char, contar letras",
            "凯撒密码：char_code、char、统计字母",
        ]
    ),
    example!(
        "results",
        [
            "Option, Result et ?",
            "Option, Result and ?",
            "Option, Result y ?",
            "Option, Result und ?",
            "Option, Result e ?",
            "Option、Result 和 ?",
        ]
    ),
    example!(
        "students",
        [
            "enregistrements, mise à jour, regroupements, moyennes",
            "records, record update, grouping, averages",
            "registros, actualización, agrupaciones, promedios",
            "Records, Aktualisierung, Gruppierung, Durchschnitte",
            "registros, atualização, agrupamentos, médias",
            "记录、记录更新、分组、平均值",
        ]
    ),
    example!(
        "library",
        [
            "le style Ruby : blocs do |x| ... end, enchaînements de méthodes, noms en ?, unless, symboles",
            "the Ruby style: do |x| ... end blocks, method chains, names ending with ?, unless, symbols",
            "el estilo Ruby: bloques do |x| ... end, cadenas de métodos, nombres terminados en ?, unless, símbolos",
            "der Ruby-Stil: do |x| ... end-Blöcke, Methodenketten, Namen mit ? am Ende, unless, Symbole",
            "o estilo Ruby: blocos do |x| ... end, encadeamento de métodos, nomes terminados em ?, unless, símbolos",
            "Ruby 风格：do |x| ... end 块、方法链、以 ? 结尾的名字、unless、符号",
        ]
    ),
    example!(
        "sales",
        [
            "un rapport : scan, colonnes alignées avec pad_left, windows, chunks, JSON",
            "a report: scan, columns lined up with pad_left, windows, chunks, JSON",
            "un informe: scan, columnas alineadas con pad_left, windows, chunks, JSON",
            "ein Bericht: scan, mit pad_left ausgerichtete Spalten, windows, chunks, JSON",
            "um relatório: scan, colunas alinhadas com pad_left, windows, chunks, JSON",
            "一份报表：scan、用 pad_left 对齐的列、windows、chunks、JSON",
        ]
    ),
    example!(
        "calendar",
        [
            "des dates : la page d'un mois, des textes ISO 8601, days_between, add_days",
            "dates: the page of a month, ISO 8601 texts, days_between, add_days",
            "fechas: la página de un mes, textos ISO 8601, days_between, add_days",
            "Daten: das Blatt eines Monats, ISO-8601-Texte, days_between, add_days",
            "datas: a página de um mês, textos ISO 8601, days_between, add_days",
            "日期：一个月的日历页、ISO 8601 文本、days_between、add_days",
        ]
    ),
    example!(
        "logs",
        [
            "lire un journal avec des regex : named_captures, matches, find_all, replace_all, format_number",
            "reading a log with regexes: named_captures, matches, find_all, replace_all, format_number",
            "leer un registro con regex: named_captures, matches, find_all, replace_all, format_number",
            "ein Protokoll mit Regex lesen: named_captures, matches, find_all, replace_all, format_number",
            "ler um log com regex: named_captures, matches, find_all, replace_all, format_number",
            "用正则表达式读日志：named_captures、matches、find_all、replace_all、format_number",
        ]
    ),
    example!(
        "grades",
        [
            "les notes d'un tableur : csv_records avec « ; », csv_format, un tirage répétable avec random_seed",
            "grades from a spreadsheet: csv_records with `;`, csv_format, a repeatable draw with random_seed",
            "notas de una hoja de cálculo: csv_records con «;», csv_format, un sorteo repetible con random_seed",
            "Noten aus einer Tabellenkalkulation: csv_records mit „;“, csv_format, eine wiederholbare Ziehung mit random_seed",
            "notas de uma planilha: csv_records com “;”, csv_format, um sorteio repetível com random_seed",
            "电子表格里的成绩：用“;”的 csv_records、csv_format、用 random_seed 可重复的抽签",
        ]
    ),
    example!(
        "tree",
        [
            "un type récursif : arbre binaire de recherche",
            "a recursive type: binary search tree",
            "un tipo recursivo: árbol binario de búsqueda",
            "ein rekursiver Typ: binärer Suchbaum",
            "um tipo recursivo: árvore binária de busca",
            "递归类型：二叉搜索树",
        ]
    ),
    example!(
        "primes",
        [
            "un crible récursif, all, and",
            "a recursive sieve, all, and",
            "una criba recursiva, all, and",
            "ein rekursives Sieb, all, and",
            "um crivo recursivo, all, and",
            "递归筛法、all、and",
        ]
    ),
    example!("testing", ["les blocs test", "test blocks", "los bloques test", "test-Blöcke", "os blocos test", "test 块"]),
    example!(
        "bonjour",
        [
            "des noms avec des accents",
            "names with accents",
            "nombres con acentos",
            "Namen mit Akzenten",
            "nomes com acentos",
            "带重音符号的名字",
        ]
    ),
];

pub fn find(name: &str) -> Option<&'static Example> {
    EXAMPLES.iter().find(|e| e.name == name)
}

/// `{"hello": "…code…", …}`: the menu of the playground.
pub fn json() -> String {
    let mut out = String::from("{");
    for (i, e) in EXAMPLES.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&json_string(e.name));
        out.push(':');
        out.push_str(&json_string(e.code));
    }
    out.push('}');
    out
}

/// A JSON string literal.
pub fn json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            // `<` too, so that the JSON can never close a `<script>` element.
            c if (c as u32) < 0x20 || c == '<' => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
