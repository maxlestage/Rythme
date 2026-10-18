// Portuguese: each English text of Rythme with its translation. How to add or
// update a language: CONTRIBUTING.md, "Messages in every language".
Language {
    code: "pt",
    name: "português",
    texts: &[
        // src/main.rs
        ("unknown language `{v}`", "idioma desconhecido `{v}`"),
        (
            r#"Rythme ran into an internal problem. This is a bug in Rythme, not in your program.
Please report it, with your program, at https://github.com/maxlestage/Rythme/issues"#,
            r#"O Rythme encontrou um problema interno. Isso é um bug do Rythme, não do seu programa.
Por favor, informe o problema, junto com o seu programa, em https://github.com/maxlestage/Rythme/issues"#,
        ),
        ("`--lang` needs a language", "`--lang` precisa de um idioma"),
        ("{what}: write `--lang` and one of {codes}, like `rythme --lang fr hello.ry`", "{what}: escreva `--lang` e um dos códigos {codes}, como `rythme --lang pt ola.ry`"),
        ("usage: rythme explain CODE   (codes: {})", "uso: rythme explain CÓDIGO   (códigos: {})"),
        ("unknown code `{code}`. Known codes: {}", "código desconhecido `{code}`. Códigos conhecidos: {}"),
        ("which file? Example: rythme run hello.ry", "qual arquivo? Exemplo: rythme run ola.ry"),
        ("which file? Example: rythme check hello.ry", "qual arquivo? Exemplo: rythme check ola.ry"),
        ("which code? Example: rythme -e 'print(1 + 2)'", "qual código? Exemplo: rythme -e 'print(1 + 2)'"),
        ("unknown command or file `{cmd}`", "comando ou arquivo desconhecido `{cmd}`"),
        (
            r#"
did you mean `rythme {best}`?"#,
            r#"
você quis dizer `rythme {best}`?"#,
        ),
        (
            r#"
see `rythme help`"#,
            r#"
veja `rythme help`"#,
        ),
        ("current folder: {}", "pasta atual: {}"),
        ("Rythme files usually end with `.ry` ({})", "os arquivos do Rythme costumam terminar em `.ry` ({})"),
        ("✓ {shown}: no problems found", "✓ {shown}: nenhum problema encontrado"),
        ("✓ {shown}: no errors, {n} warning(s)", "✓ {shown}: nenhum erro, {n} aviso(s)"),
        ("✗ {shown}: {n} error(s)", "✗ {shown}: {n} erro(s)"),
        // src/bigint.rs
        ("this whole number would be too large (more than {MAX_DIGITS} digits)", "este número inteiro ficaria grande demais (mais de {MAX_DIGITS} algarismos)"),
        ("does it grow without end, in a function that calls itself? For an approximate value, use a decimal number (Float), like `2.0 ** 1000`", "ele cresce sem parar, em uma função que chama a si mesma? Para um valor aproximado, use um número decimal (Float), como `2.0 ** 1000`"),
        // src/builtins/date.rs
        ("the year must go from 1 to 9999, not {year}", "o ano deve ir de 1 a 9999, não {year}"),
        ("the month must go from 1 to 12, not {month}", "o mês deve ir de 1 a 12, não {month}"),
        ("the day must go from 1 to {len} in {name} {year}, not {day}", "o dia deve ir de 1 a {len} em {name} de {year}, não {day}"),
        (" ({year} is not a leap year)", " ({year} não é um ano bissexto)"),
        ("the hour must go from 0 to 23, not {hour}", "a hora deve ir de 0 a 23, não {hour}"),
        ("the minute must go from 0 to 59, not {minute}", "o minuto deve ir de 0 a 59, não {minute}"),
        ("the second must go from 0 to 59, not {second}", "o segundo deve ir de 0 a 59, não {second}"),
        ("the offset must go from -1080 to 1080 minutes (-18:00 to +18:00), not {offset}", "o deslocamento deve ir de -1080 a 1080 minutos (-18:00 a +18:00), não {offset}"),
        ("a date has no field `{k}`", "uma data não tem o campo `{k}`"),
        (" (did you mean `{best}`?)", " (você quis dizer `{best}`?)"),
        ("the field `{k}` is too large for a date", "o campo `{k}` é grande demais para uma data"),
        ("the field `{k}` must be a whole number, not {}", "o campo `{k}` deve ser um número inteiro, não {}"),
        ("the field `{k}` is missing", "falta o campo `{k}`"),
        ("\"{text}\" is not a date written as ISO 8601: write it like \"2024-02-29\" or \"2024-02-29T13:05:00\"", "\"{text}\" não é uma data escrita em ISO 8601: escreva como \"2024-02-29\" ou \"2024-02-29T13:05:00\""),
        ("\"{text}\" is not a valid date: {why}", "\"{text}\" não é uma data válida: {why}"),
        ("`{f}` needs a valid date: {why}", "`{f}` precisa de uma data válida: {why}"),
        ("not a valid date", "não é uma data válida"),
        ("a date is a record like `{{year: 2024, month: 2, day: 29}}`, with `hour`, `minute`, `second` and `offset` (minutes ahead of UTC) if needed", "uma data é um registro como `{{year: 2024, month: 2, day: 29}}`, com `hour`, `minute`, `second` e `offset` (minutos à frente do UTC) se necessário"),
        ("`{f}` gives a date outside of the years 1 to 9999", "`{f}` dá uma data fora dos anos 1 a 9999"),
        ("the month must go from 1 to 12, not {m}", "o mês deve ir de 1 a 12, não {m}"),
        ("the clock of the system gives a date outside of the years 1 to 9999", "o relógio do sistema dá uma data fora dos anos 1 a 9999"),
        // src/builtins/dicts.rs
        ("`{f}`: {} cannot be a key of a dict, nor an element of a set", "`{f}`: {} não pode ser chave de um dicionário, nem elemento de um conjunto"),
        ("it holds {}", "ele contém {}"),
        ("keys and elements must be values that can be compared: anything but functions and `nan`", "chaves e elementos devem ser valores que podem ser comparados: qualquer coisa, menos funções e `nan`"),
        ("`{f}` needs a list of [key, value] pairs, but it contains {}", "`{f}` precisa de uma lista de pares [chave, valor], mas ela contém {}"),
        ("each pair is a list of two elements, like `dict([[\"Ada\", 36], [\"Alan\", 41]])`", "cada par é uma lista de dois elementos, como `dict([[\"Ada\", 36], [\"Alan\", 41]])`"),
        // src/builtins/property.rs
        ("this is not a generator: {}", "isto não é um gerador: {}"),
        ("not a generator", "não é um gerador"),
        ("make one with `ints()`, `floats()`, `texts()`, `bools()`, `lists_of(g)`, `one_of(list)` or `records_of({{x: g}})`", "crie um com `ints()`, `floats()`, `texts()`, `bools()`, `lists_of(g)`, `one_of(list)` ou `records_of({{x: g}})`"),
        ("generators can be put inside each other {MAX_DEPTH} times at most", "os geradores podem ser colocados uns dentro dos outros no máximo {MAX_DEPTH} vezes"),
        ("its field `{what}` is missing or wrong", "o campo `{what}` dele está faltando ou está errado"),
        ("`{f}` takes no argument, or two: the smallest and the largest value, like `{f}(0, 10)`", "`{f}` não recebe nenhum argumento, ou recebe dois: o menor e o maior valor, como `{f}(0, 10)`"),
        ("{f}({}, {}): the minimum is larger than the maximum", "{f}({}, {}): o mínimo é maior que o máximo"),
        ("`floats` needs finite numbers, not {} and {}", "`floats` precisa de números finitos, não {} e {}"),
        ("`{f}` needs a length from 0 to {MAX_LENGTH}, not {n}", "`{f}` precisa de um comprimento de 0 a {MAX_LENGTH}, não {n}"),
        ("`one_of` needs at least one value to choose from", "`one_of` precisa de pelo menos um valor para escolher"),
        ("the field `{k}` given to `records_of` must be a generator, like `ints()`, not {}", "o campo `{k}` dado a `records_of` deve ser um gerador, como `ints()`, não {}"),
        ("`check_all` tries from 1 to {MAX_CASES} values, not {cases}", "`check_all` testa de 1 a {MAX_CASES} valores, não {cases}"),
        ("the property is false for {v}", "a propriedade é falsa para {v}"),
        ("false for {v}", "falsa para {v}"),
        ("the property gives {r} for {v}", "a propriedade dá {r} para {v}"),
        ("{r} for {v}", "{r} para {v}"),
        ("the property fails for {v}: {}", "a propriedade falha para {v}: {}"),
        ("checked here", "verificada aqui"),
        ("found on try {case} of {cases}", "encontrado na tentativa {case} de {cases}"),
        ("found on try {case} of {cases}, then simplified from {} in {steps} step(s)", "encontrado na tentativa {case} de {cases}, depois simplificado a partir de {} em {steps} passo(s)"),
        ("seed {seed}: every run tries the same values; `rythme test --seed N` tries others", "semente {seed}: cada execução testa os mesmos valores; `rythme test --seed N` testa outros"),
        ("the function must hold for every value of the generator: fix the function, or give the generator a smaller range, like `ints(1, 100)`", "a função deve valer para todo valor do gerador: corrija a função, ou dê ao gerador um intervalo menor, como `ints(1, 100)`"),
        // src/builtins/regex.rs
        ("for the sign `{c}` itself, write `[{c}]` (in a Rythme text: \"[\\{c}]\")", "para o próprio sinal `{c}`, escreva `[{c}]` (em um texto do Rythme: \"[\\{c}]\")"),
        ("for the sign `{c}` itself, write `[{c}]`", "para o próprio sinal `{c}`, escreva `[{c}]`"),
        ("`{q}` has nothing to repeat", "`{q}` não tem nada para repetir"),
        ("nothing before it to repeat", "nada antes dele para repetir"),
        ("put it after a character or a group, like `a{q}` or `(ab){q}`; {}", "coloque-o depois de um caractere ou de um grupo, como `a{q}` ou `(ab){q}`; {}"),
        ("`{{` must start a count, like {{3}} or {{2,5}}", "`{{` deve começar uma contagem, como {{3}} ou {{2,5}}"),
        ("not a count", "não é uma contagem"),
        ("{}; in a Rythme text, braces are written `\\{{` and `\\}}`, like \"[0-9]\\{{4\\}}\"", "{}; em um texto do Rythme, as chaves se escrevem `\\{{` e `\\}}`, como \"[0-9]\\{{4\\}}\""),
        ("a count can be at most {MAX_COUNT}", "uma contagem pode ser no máximo {MAX_COUNT}"),
        ("too large", "grande demais"),
        ("for more, use `*` or `+`, or {{n,}} without a maximum", "para mais, use `*` ou `+`, ou {{n,}} sem máximo"),
        ("the count goes backwards: {min} is more than {max}", "a contagem está ao contrário: {min} é maior que {max}"),
        ("smallest first", "o menor primeiro"),
        ("write the smallest number first: {{{max},{min}}}", "escreva o menor número primeiro: {{{max},{min}}}"),
        ("a repetition cannot be repeated again", "uma repetição não pode ser repetida de novo"),
        ("repeats a repetition", "repete uma repetição"),
        ("put the repeated part in parentheses, like `(a+)*`, or keep one sign, like `a*`; {}", "coloque a parte repetida entre parênteses, como `(a+)*`, ou mantenha um só sinal, como `a*`; {}"),
        ("the groups are nested too deeply (more than {MAX_DEPTH})", "os grupos estão aninhados fundo demais (mais de {MAX_DEPTH})"),
        ("too deep", "fundo demais"),
        ("`(?<` must be followed by a name and `>`", "`(?<` deve ser seguido de um nome e de `>`"),
        ("not a group name", "não é um nome de grupo"),
        ("a named group looks like `(?<year>[0-9]+)`: the name has letters, digits and `_`", "um grupo com nome se parece com `(?<year>[0-9]+)`: o nome tem letras, algarismos e `_`"),
        ("two groups are named `{name}`", "dois grupos se chamam `{name}`"),
        ("already used", "já usado"),
        ("give each group its own name", "dê a cada grupo um nome próprio"),
        ("`(?i)` must be at the very start of the regex", "`(?i)` deve ficar bem no início da regex"),
        ("not at the start", "não está no início"),
        ("`(?i)` ignores the case in the whole regex: write it first", "`(?i)` ignora maiúsculas e minúsculas na regex inteira: escreva-o primeiro"),
        ("unknown kind of group", "tipo de grupo desconhecido"),
        ("unknown", "desconhecido"),
        ("the groups are `(...)`, `(?:...)` (a group that captures nothing) and `(?<name>...)` (a named group); `(?i)` at the start ignores the case", "os grupos são `(...)`, `(?:...)` (um grupo que não captura nada) e `(?<name>...)` (um grupo com nome); `(?i)` no início ignora maiúsculas e minúsculas"),
        ("this `(` is never closed", "este `(` nunca é fechado"),
        ("opened here", "aberto aqui"),
        ("add a `)` where the group ends; {}", "adicione um `)` onde o grupo termina; {}"),
        ("the regex ends with a lone `\\`", "a regex termina com um `\\` sozinho"),
        ("followed by nothing", "seguido de nada"),
        ("a `\\` comes before a letter like d, w, s or b, or before a sign for the sign itself, like `\\.`", "um `\\` vem antes de uma letra como d, w, s ou b, ou antes de um sinal para o próprio sinal, como `\\.`"),
        ("`\\{c}` means nothing inside `[...]`", "`\\{c}` não significa nada dentro de `[...]`"),
        ("not a character", "não é um caractere"),
        ("`\\b` is the edge of a word: write it outside of the brackets", "`\\b` é a borda de uma palavra: escreva-o fora dos colchetes"),
        ("`\\{c}` means nothing in a regex", "`\\{c}` não significa nada em uma regex"),
        (
            r#"the known ones: \d (a digit), \w (a letter, a digit or _), \s (a space), \b (the edge of a word),
their opposites \D, \W, \S, \B, \n, \t, and `\` before a sign for the sign itself, like \. or \("#,
            r#"os conhecidos: \d (um algarismo), \w (uma letra, um algarismo ou _), \s (um espaço), \b (a borda de uma palavra),
seus opostos \D, \W, \S, \B, \n, \t, e `\` antes de um sinal para o próprio sinal, como \. ou \("#,
        ),
        ("this `[` is never closed", "este `[` nunca é fechado"),
        ("add a `]` after the characters; for the sign `]` itself, put it first: `[]]`", "adicione um `]` depois dos caracteres; para o próprio sinal `]`, coloque-o primeiro: `[]]`"),
        ("a range must end with one character", "um intervalo deve terminar com um único caractere"),
        ("not a range", "não é um intervalo"),
        ("for the sign `-` itself, put it last: `[a-]`", "para o próprio sinal `-`, coloque-o por último: `[a-]`"),
        ("the range `{lo}-{hi}` goes backwards", "o intervalo `{lo}-{hi}` está ao contrário"),
        ("backwards", "ao contrário"),
        ("write the first character first: `{hi}-{lo}`", "escreva o primeiro caractere primeiro: `{hi}-{lo}`"),
        ("this `)` closes nothing", "este `)` não fecha nada"),
        ("no `(` before it", "nenhum `(` antes dele"),
        ("the regex is too large once its repetitions are written out", "a regex fica grande demais quando suas repetições são escritas por extenso"),
        ("use smaller counts in {{n,m}}, or `*` and `+`, which cost nothing", "use contagens menores em {{n,m}}, ou `*` e `+`, que não custam nada"),
        ("`$1` is the text of group 1, `$<name>` of a named group, `$0` the whole match; for the sign `$` itself, write `$$`", "`$1` é o texto do grupo 1, `$<name>` o de um grupo com nome, `$0` a correspondência inteira; para o próprio sinal `$`, escreva `$$`"),
        ("the regex has no group", "a regex não tem nenhum grupo"),
        ("the regex has 1 group", "a regex tem 1 grupo"),
        ("the regex has {g} groups", "a regex tem {g} grupos"),
        ("there is no group {n}: {has}", "não existe o grupo {n}: {has}"),
        ("no such group", "grupo inexistente"),
        ("`$<` must be followed by a group name and `>`", "`$<` deve ser seguido de um nome de grupo e de `>`"),
        ("never closed", "nunca fechado"),
        ("the regex has no named group", "a regex não tem nenhum grupo com nome"),
        ("its named groups are {}", "os grupos com nome dela são {}"),
        ("the regex has no group named `{name}`: {has}", "a regex não tem nenhum grupo chamado `{name}`: {has}"),
        ("unknown group", "grupo desconhecido"),
        ("`$` must be followed by a group number, `<name>` or `$`", "`$` deve ser seguido de um número de grupo, de `<name>` ou de `$`"),
        ("a lone `$`", "um `$` sozinho"),
        ("invalid replacement: {}", "substituição inválida: {}"),
        ("invalid regex: {}", "regex inválida: {}"),
        ("with this replacement", "com esta substituição"),
        ("with this regex", "com esta regex"),
        ("here in the replacement:", "aqui na substituição:"),
        ("here in the regex:", "aqui na regex:"),
        ("`named_captures` needs a regex with named groups", "`named_captures` precisa de uma regex com grupos com nome"),
        ("name a group like this: `(?<year>[0-9]+)`; for groups without names, use `captures`", "dê nome a um grupo assim: `(?<year>[0-9]+)`; para grupos sem nome, use `captures`"),
        // src/builtins/system.rs
        ("`{f}` is not available in the browser: a program here has no input to read. Run it on your computer: rythme file.ry < lines.txt", "`{f}` não está disponível no navegador: aqui um programa não tem nenhuma entrada para ler. Execute-o no seu computador: rythme arquivo.ry < linhas.txt"),
        ("`{f}` is not available in the browser: a program here has no files or folders. Run it on your computer: rythme file.ry", "`{f}` não está disponível no navegador: aqui um programa não tem arquivos nem pastas. Execute-o no seu computador: rythme arquivo.ry"),
        ("the path is empty: write \".\" for the current folder", "o caminho está vazio: escreva \".\" para a pasta atual"),
        ("folder not found: {path}", "pasta não encontrada: {path}"),
        ("file not found: {path}", "arquivo não encontrado: {path}"),
        ("permission denied: {path}", "permissão negada: {path}"),
        ("cannot access {path}: {e}", "não é possível acessar {path}: {e}"),
        ("{path} is a file, not a folder: `read_file` reads a file", "{path} é um arquivo, não uma pasta: `read_file` lê um arquivo"),
        ("{path} already exists, and it is a file, not a folder", "{path} já existe, e é um arquivo, não uma pasta"),
        ("{path} is a folder: `delete_file` deletes only files, never folders", "{path} é uma pasta: `delete_file` apaga apenas arquivos, nunca pastas"),
        ("cannot read the standard input: {e}", "não é possível ler a entrada padrão: {e}"),
        ("`join_path` needs a list of texts, but it contains {}", "`join_path` precisa de uma lista de textos, mas ela contém {}"),
        ("`text(value)` turns a value into a text", "`text(value)` transforma um valor em texto"),
        ("`join_path` cannot add \"{part}\" after \"{out}\": only the first part of a path can start at a root (`/`, `\\` or a drive like `C:`)", "`join_path` não pode adicionar \"{part}\" depois de \"{out}\": só a primeira parte de um caminho pode começar em uma raiz (`/`, `\\` ou uma unidade como `C:`)"),
        ("remove the separator at the start of \"{part}\", or put it first in the list", "remova o separador no início de \"{part}\", ou coloque-o primeiro na lista"),
        ("`{w}` is not a valid option: its name starts with a letter, followed by letters, digits, `-` or `_` (write `--` before words that start with `-`)", "`{w}` não é uma opção válida: o nome dela começa com uma letra, seguida de letras, algarismos, `-` ou `_` (escreva `--` antes de palavras que começam com `-`)"),
        ("`{w}` is not an option that Rythme understands: write `--{name}` for a long name, or `-a -b` for several one-letter options", "`{w}` não é uma opção que o Rythme entende: escreva `--{name}` para um nome longo, ou `-a -b` para várias opções de uma letra"),
        ("`{w}` cannot be an option: the field `args` holds the words that are not options", "`{w}` não pode ser uma opção: o campo `args` guarda as palavras que não são opções"),
        ("the option `{name}` is given twice", "a opção `{name}` foi dada duas vezes"),
        ("`options` needs a list of texts, like the one of `args()`, but it contains {}", "`options` precisa de uma lista de textos, como a de `args()`, mas ela contém {}"),
        // src/builtins/text_tools.rs
        ("`{f}` writes at most 100 decimals, not {d}", "`{f}` escreve no máximo 100 casas decimais, não {d}"),
        ("`{f}` does not know the language \"{l}\"", "`{f}` não conhece o idioma \"{l}\""),
        ("the languages: \"en\" (1,234.5), \"fr\" (1 234,5), \"de\", \"es\", \"it\", \"nl\" and \"pt\" (1.234,5)", "os idiomas: \"en\" (1,234.5), \"fr\" (1 234,5), \"de\", \"es\", \"it\", \"nl\" e \"pt\" (1.234,5)"),
        // src/builtins.rs
        ("Basics", "Básico"),
        ("Numbers", "Números"),
        ("Text", "Texto"),
        ("Lists", "Listas"),
        ("Records", "Registros"),
        ("Dictionaries and sets", "Dicionários e conjuntos"),
        ("Option and Result", "Option e Result"),
        ("Files", "Arquivos"),
        ("Dates and times", "Datas e horas"),
        ("Property tests", "Testes de propriedades"),
        ("usage: `{}`", "uso: `{}`"),
        ("a list", "uma lista"),
        ("a text", "um texto"),
        ("a whole number (Int)", "um número inteiro (Int)"),
        ("a number", "um número"),
        ("a function", "uma função"),
        ("a record", "um registro"),
        ("true or false", "true ou false"),
        ("a list or a text", "uma lista ou um texto"),
        ("a text or a list", "um texto ou uma lista"),
        ("a text, a list, a record, a dict or a set", "um texto, uma lista, um registro, um dicionário ou um conjunto"),
        ("a list, a text, a record or a dict", "uma lista, um texto, um registro ou um dicionário"),
        ("a record or a dict", "um registro ou um dicionário"),
        ("a record, a dict or a set", "um registro, um dicionário ou um conjunto"),
        ("a list, a dict or a set", "uma lista, um dicionário ou um conjunto"),
        ("a list or a set", "uma lista ou um conjunto"),
        ("a dict or a set", "um dicionário ou um conjunto"),
        ("a dict", "um dicionário"),
        ("a set", "um conjunto"),
        ("a list of [key, value] pairs (or a record)", "uma lista de pares [chave, valor] (ou um registro)"),
        ("a list (or several values)", "uma lista (ou vários valores)"),
        ("a number or a text", "um número ou um texto"),
        ("a list, a set, an Option or a Result", "uma lista, um conjunto, uma Option ou um Result"),
        ("an Option or a Result", "uma Option ou um Result"),
        ("an Option", "uma Option"),
        ("a Result", "um Result"),
        ("a date (a record {{year, month, day}})", "uma data (um registro {{year, month, day}})"),
        ("a generator (like `ints()`)", "um gerador (como `ints()`)"),
        ("a text or a function", "um texto ou uma função"),
        ("`{f}` expects {expected} as argument {pos}, but got {got}", "`{f}` espera {expected} como argumento {pos}, mas recebeu {got}"),
        ("wrong kind of value", "tipo de valor errado"),
        ("a text is not a list: `chars(text)` gives the list of its characters", "um texto não é uma lista: `chars(text)` dá a lista dos seus caracteres"),
        ("a record is not a list: `entries(record)`, `keys(record)` or `values(record)` give one", "um registro não é uma lista: `entries(record)`, `keys(record)` ou `values(record)` dão uma"),
        ("a dict is not a list: `entries(dict)`, `keys(dict)` or `values(dict)` give one", "um dicionário não é uma lista: `entries(dict)`, `keys(dict)` ou `values(dict)` dão uma"),
        ("a set is not a list: `values(set)` gives the list of its elements, in order", "um conjunto não é uma lista: `values(set)` dá a lista dos seus elementos, em ordem"),
        ("to change the values of a dict, use `map_values(dict, f)`", "para mudar os valores de um dicionário, use `map_values(dict, f)`"),
        ("for a list, use `{cousin}`", "para uma lista, use `{cousin}`"),
        ("to read a date from a text like \"2024-02-29\", use `parse_date(text)`", "para ler uma data de um texto como \"2024-02-29\", use `parse_date(text)`"),
        ("for a list, use `find(list, f)`", "para uma lista, use `find(list, f)`"),
        ("for an Option or a Result, use `and_then(x, f)`", "para uma Option ou um Result, use `and_then(x, f)`"),
        ("the function given to `{f}` must return a list, but it returned {got}", "a função dada a `{f}` deve devolver uma lista, mas devolveu {got}"),
        ("the function given to `{f}` must return a text, but it returned {got}", "a função dada a `{f}` deve devolver um texto, mas devolveu {got}"),
        ("to make a text of any value, use `text(x)`", "para transformar qualquer valor em texto, use `text(x)`"),
        ("an Option (Some or None)", "uma Option (Some ou None)"),
        ("a Result (Ok or Err)", "um Result (Ok ou Err)"),
        ("the function given to `{f}` must return {expected}, but it returned {got}", "a função dada a `{f}` deve devolver {expected}, mas devolveu {got}"),
        ("to change the value inside without another check, use `map(x, f)`", "para mudar o valor de dentro sem outra verificação, use `map(x, f)`"),
        ("the function given to `{f}` must return true or false, but it returned {got}", "a função dada a `{f}` deve devolver true ou false, mas devolveu {got}"),
        ("the number {n} is too large for `{f}`", "o número {n} é grande demais para `{f}`"),
        ("{digits} digits", "{digits} algarismos"),
        ("`{f}` needs a number of 0 or more, not {n}", "`{f}` precisa de um número igual ou maior que 0, não {n}"),
        ("`{f}` needs a number of 1 or more, not 0", "`{f}` precisa de um número igual ou maior que 1, não 0"),
        ("the list is empty", "a lista está vazia"),
        ("the list has {len} element(s)", "a lista tem {len} elemento(s)"),
        ("position {n} is out of range: {what}", "a posição {n} está fora do intervalo: {what}"),
        ("no element here", "nenhum elemento aqui"),
        ("valid positions go from 0 to {last} (or from -1 backwards, counting from the end)", "as posições válidas vão de 0 a {last} (ou de -1 para trás, contando a partir do fim)"),
        ("at least {min}", "pelo menos {min}"),
        ("{min} to {max}", "{min} a {max}"),
        ("`{f}` gives {given} argument(s) to this function, but it takes {expected}", "`{f}` dá {given} argumento(s) a esta função, mas ela recebe {expected}"),
        ("takes {expected} argument(s)", "recebe {expected} argumento(s)"),
        ("`nan` (not a number) cannot be converted to a whole number", "`nan` (não é um número) não pode ser convertido em número inteiro"),
        ("`nan` is the result of a calculation that has none, like `0.0 * inf`: check where this value comes from", "`nan` é o resultado de um cálculo que não tem resultado, como `0.0 * inf`: verifique de onde vem esse valor"),
        ("`{}` (infinity) cannot be converted to a whole number", "`{}` (infinito) não pode ser convertido em número inteiro"),
        ("an infinity comes from a calculation too large for decimal numbers, like `1e308 * 10.0`, or from a division by `0.0`", "um infinito vem de um cálculo grande demais para números decimais, como `1e308 * 10.0`, ou de uma divisão por `0.0`"),
        ("`text` writes at most 100 decimals, not {digits}", "`text` escreve no máximo 100 casas decimais, não {digits}"),
        ("`input` found the end of the input", "`input` chegou ao fim da entrada"),
        ("no more lines to read", "não há mais linhas para ler"),
        ("the keyboard input was closed (Ctrl-D), or the text sent to the program has ended", "a entrada do teclado foi fechada (Ctrl-D), ou o texto enviado ao programa terminou"),
        ("assertion failed: {}", "asserção falhou: {}"),
        ("assertion failed", "asserção falhou"),
        ("this condition is false", "esta condição é falsa"),
        (
            r#"got:      {got}
expected: {expected}"#,
            r#"obtido:   {got}
esperado: {expected}"#,
        ),
        ("first difference: character {at}", "primeira diferença: caractere {at}"),
        ("assertion failed: the values are different", "asserção falhou: os valores são diferentes"),
        (" at {path}", " em {path}"),
        ("the list{at} has {} element(s), {} expected", "a lista{at} tem {} elemento(s), {} esperado(s)"),
        ("in excess", "a mais"),
        ("missing", "faltando"),
        ("{word} from [{from}]: {}{dots}", "{word} a partir de [{from}]: {}{dots}"),
        ("missing field(s){at}: {}", "campo(s) faltando{at}: {}"),
        ("field(s) not expected{at}: {}", "campo(s) não esperado(s){at}: {}"),
        (", which are of different types: {} and {}", ", que são de tipos diferentes: {} e {}"),
        ("first difference at {path}: got {}, expected {}{kinds}", "primeira diferença em {path}: obtido {}, esperado {}{kinds}"),
        ("the values are of different types: {} and {}", "os valores são de tipos diferentes: {} e {}"),
        ("(no line)", "(nenhuma linha)"),
        (
            r#"first different line: line {}
  got:      {}
  expected: {}"#,
            r#"primeira linha diferente: linha {}
  obtido:   {}
  esperado: {}"#,
        ),
        ("the program stopped with `fail`", "o programa parou com `fail`"),
        ("the program stopped here", "o programa parou aqui"),
        ("an exit code goes from 0 to 255, not {code}", "um código de saída vai de 0 a 255, não {code}"),
        ("the program stopped with exit({code})", "o programa parou com exit({code})"),
        ("cannot pick a random element in an empty list", "não é possível escolher um elemento aleatório em uma lista vazia"),
        ("random({lo}, {hi}): the minimum is larger than the maximum", "random({lo}, {hi}): o mínimo é maior que o máximo"),
        ("`{name}` of an empty list", "`{name}` de uma lista vazia"),
        ("`{name}` cannot compare {} and {}", "`{name}` não consegue comparar {} e {}"),
        ("cannot take the square root of a negative number ({x})", "não é possível calcular a raiz quadrada de um número negativo ({x})"),
        ("use `sqrt(abs(x))` if you want the root of its absolute value", "use `sqrt(abs(x))` se você quiser a raiz do valor absoluto"),
        ("cannot convert \"{s}\" to a whole number", "não é possível converter \"{s}\" em número inteiro"),
        ("if the text may not be a number, use `parse_int(t)`: it gives `Ok(n)` or `Err(...)` that you can check", "se o texto pode não ser um número, use `parse_int(t)`: ele dá `Ok(n)` ou `Err(...)`, que você pode verificar"),
        ("cannot convert \"{s}\" to a decimal number", "não é possível converter \"{s}\" em número decimal"),
        ("if the text may not be a number, use `parse_float(t)`, which gives `Ok(x)` or `Err(...)`", "se o texto pode não ser um número, use `parse_float(t)`, que dá `Ok(x)` ou `Err(...)`"),
        ("`{name}` needs a number from -1 to 1, not {x}", "`{name}` precisa de um número de -1 a 1, não {x}"),
        ("the logarithm only exists for positive numbers, not {x}", "o logaritmo só existe para números positivos, não para {x}"),
        ("invalid logarithm base: {base}", "base de logaritmo inválida: {base}"),
        ("`{f}` needs a single character, but \"{s}\" has {}", "`{f}` precisa de um único caractere, mas \"{s}\" tem {}"),
        ("`replace` cannot replace an empty text", "`replace` não pode substituir um texto vazio"),
        ("{n} is not the number of a character", "{n} não é o número de um caractere"),
        ("character numbers go from 0 to 1114111, except 55296 to 57343", "os números de caractere vão de 0 a 1114111, exceto de 55296 a 57343"),
        ("the repeated text would be too large", "o texto repetido ficaria grande demais"),
        ("the repeated list would be too large", "a lista repetida ficaria grande demais"),
        ("this number has more than {} digits", "este número tem mais de {} algarismos"),
        ("\"{s}\" is not a whole number", "\"{s}\" não é um número inteiro"),
        ("\"{s}\" is not a number", "\"{s}\" não é um número"),
        ("JSON, line {line}, column {column}: {msg}", "JSON, linha {line}, coluna {column}: {msg}"),
        ("expected {what}, but found `{c}`", "esperava {what}, mas encontrou `{c}`"),
        ("expected {what}, but the text ends", "esperava {what}, mas o texto termina"),
        ("a value", "um valor"),
        ("the end of the text", "o fim do texto"),
        ("`,` or `]`", "`,` ou `]`"),
        ("`,` or `}}`", "`,` ou `}}`"),
        ("a name in quotes", "um nome entre aspas"),
        ("invalid number: JSON writes numbers like 0, -12, 3.5 or 1e-3", "número inválido: o JSON escreve números como 0, -12, 3.5 ou 1e-3"),
        ("invalid escape: after `\\` comes one of \" \\ / b f n r t u", "escape inválido: depois de `\\` vem um de \" \\ / b f n r t u"),
        ("a line break or a tab inside a text is written with `\\`: `\\n`, `\\t`", "uma quebra de linha ou uma tabulação dentro de um texto se escreve com `\\`: `\\n`, `\\t`"),
        ("a text is never closed: `\"` is missing", "um texto nunca é fechado: falta `\"`"),
        ("`\\u` must be followed by 4 hexadecimal digits", "`\\u` deve ser seguido de 4 algarismos hexadecimais"),
        ("invalid `\\u` escape: half of a character (a surrogate) without its other half", "escape `\\u` inválido: metade de um caractere (um surrogate) sem a outra metade"),
        ("`to_json` indents by 16 spaces at most, not {n}", "`to_json` indenta no máximo 16 espaços, não {n}"),
        ("the JSON text would be too large", "o texto JSON ficaria grande demais"),
        ("the values are nested too deeply to be indented: write them on one line, with `to_json(value)`", "os valores estão aninhados fundo demais para serem indentados: escreva-os em uma linha, com `to_json(value)`"),
        ("`to_json` cannot write {} in JSON", "`to_json` não pode escrever {} em JSON"),
        ("JSON has no infinite numbers and no NaN", "o JSON não tem números infinitos nem NaN"),
        (
            r#"a dict is written as a JSON object only when its keys are all texts, or all symbols:
turn the keys into texts first, or write `entries(dict)`"#,
            r#"um dicionário só é escrito como objeto JSON quando suas chaves são todas textos, ou todas símbolos:
transforme as chaves em textos primeiro, ou escreva `entries(dict)`"#,
        ),
        (
            r#"JSON only holds numbers, texts, true and false, lists, records and null (`None`):
turn your value into a record first, like {{kind: "Circle", radius: r}}"#,
            r#"o JSON só guarda números, textos, true e false, listas, registros e null (`None`):
transforme seu valor em um registro primeiro, como {{kind: "Circle", radius: r}}"#,
        ),
        ("`sum` needs a list of numbers, but it contains {}", "`sum` precisa de uma lista de números, mas ela contém {}"),
        ("`product` needs a list of numbers, but it contains {}", "`product` precisa de uma lista de números, mas ela contém {}"),
        ("all the elements must be of the same kind: all numbers, or all texts...", "todos os elementos devem ser do mesmo tipo: todos números, ou todos textos..."),
        ("the result of `windows` would be too large", "o resultado de `windows` ficaria grande demais"),
        ("the function `key` must give values of the same kind for every element: all numbers, or all texts...", "a função `key` deve dar valores do mesmo tipo para todos os elementos: todos números, ou todos textos..."),
        ("`flatten` needs a list of lists, but it contains {}", "`flatten` precisa de uma lista de listas, mas ela contém {}"),
        ("`from_entries` needs a list of [name, value] pairs, but it contains {}", "`from_entries` precisa de uma lista de pares [nome, valor], mas ela contém {}"),
        ("each pair is a list of two elements whose name is a text, like [\"age\", 36]", "cada par é uma lista de dois elementos cujo nome é um texto, como [\"age\", 36]"),
        ("`unwrap` found `None`: there is no value", "`unwrap` encontrou `None`: não há nenhum valor"),
        ("handle the missing value with `match`, or give a default: `unwrap_or(x, default)`", "trate o valor ausente com `match`, ou dê um valor padrão: `unwrap_or(x, default)`"),
        ("`unwrap` found an error: {}", "`unwrap` encontrou um erro: {}"),
        ("handle the error with `match`, pass it on with `?`, or give a default: `unwrap_or(x, default)`", "trate o erro com `match`, repasse-o com `?`, ou dê um valor padrão: `unwrap_or(x, default)`"),
        // src/check/needs.rs
        ("`{name}` stops with this error, every time", "`{name}` para com este erro, todas as vezes"),
        ("here", "aqui"),
        ("in `{name}`: {label}", "em `{name}`: {label}"),
        ("`{name}` always fails with {}", "`{name}` sempre falha com {}"),
        ("fails every time", "falha todas as vezes"),
        ("defined here", "definido aqui"),
        ("from how `{name}` uses `{p}`, it needs: `{sig}`", "pelo jeito como `{name}` usa `{p}`, ele precisa de: `{sig}`"),
        // src/check.rs
        ("the type `{name}` is part of Rythme", "o tipo `{name}` faz parte do Rythme"),
        ("give your type another name", "dê outro nome ao seu tipo"),
        ("the type `{}` is already defined", "o tipo `{}` já está definido"),
        ("first defined here", "definido primeiro aqui"),
        ("give it another name", "dê outro nome a ele"),
        ("the field `{f}` appears twice in `{}`", "o campo `{f}` aparece duas vezes em `{}`"),
        ("`{s}` is already the name of the file `{file}`", "`{s}` já é o nome do arquivo `{file}`"),
        ("choose another name here, or give the file another one: `use ... as other_name`", "escolha outro nome aqui, ou dê outro nome ao arquivo: `use ... as other_name`"),
        ("`{s}` is already defined", "`{s}` já está definido"),
        ("defined again here", "definido de novo aqui"),
        ("a constructor belongs to a single type: rename one of them", "um construtor pertence a um único tipo: renomeie um deles"),
        ("to give several definitions to a function (one per case), write them one right after the other", "para dar várias definições a uma função (uma por caso), escreva-as uma logo depois da outra"),
        ("`{s}` hides the built-in function `{s}`", "`{s}` esconde a função embutida `{s}`"),
        ("`{s}` hides the built-in value `{s}`", "`{s}` esconde o valor embutido `{s}`"),
        ("in this file, `{s}` will now mean your definition; choose another name to keep the built-in one", "neste arquivo, `{s}` agora vai significar a sua definição; escolha outro nome para manter a embutida"),
        ("`{s}` is part of Rythme", "`{s}` faz parte do Rythme"),
        ("choose another name", "escolha outro nome"),
        ("the result of `{name}` is lost", "o resultado de `{name}` se perde"),
        ("this value is lost", "este valor se perde"),
        (
            r#"values never change: `{sig}` gives a new value, and changes nothing.
To keep it, give it a name: `new = {sig}`"#,
            r#"os valores nunca mudam: `{sig}` dá um valor novo, e não muda nada.
Para guardá-lo, dê um nome a ele: `new = {sig}`"#,
        ),
        ("this test can never fail", "este teste nunca pode falhar"),
        ("its result, {}, is never compared with what is expected", "o resultado dele, {}, nunca é comparado com o que se espera"),
        (
            r#"a test fails only when it stops with an error, or ends with `false`, `None` or `Err(...)`;
nothing in this one can"#,
            r#"um teste só falha quando para com um erro, ou termina com `false`, `None` ou `Err(...)`;
nada neste aqui pode fazer isso"#,
        ),
        ("compare the result with what you expect: `assert_eq(actual, expected)`", "compare o resultado com o que você espera: `assert_eq(actual, expected)`"),
        ("remove `{name}` from the pattern, or write `{name}: _{name}` if this is on purpose", "remova `{name}` do padrão, ou escreva `{name}: _{name}` se for de propósito"),
        ("remove it, or name it `_{name}` if this is on purpose", "remova-o, ou chame-o de `_{name}` se for de propósito"),
        ("`{name}` is never used", "`{name}` nunca é usado"),
        ("`{s}` appears twice in this pattern", "`{s}` aparece duas vezes neste padrão"),
        ("first here", "primeiro aqui"),
        ("use two different names, then compare them with `==` in a condition: `[a, b] if a == b -> ...`", "use dois nomes diferentes e depois compare-os com `==` em uma condição: `[a, b] if a == b -> ...`"),
        ("`{s}` is used before being defined", "`{s}` é usado antes de ser definido"),
        ("used here", "usado aqui"),
        ("defined later, here", "definido depois, aqui"),
        ("move the definition above this line", "mova a definição para cima desta linha"),
        ("`{s}` is the name of the file `{file}`, not a value", "`{s}` é o nome do arquivo `{file}`, não um valor"),
        ("write `{s}.` and the name of one of its definitions, like `{s}.{example}`", "escreva `{s}.` e o nome de uma das definições dele, como `{s}.{example}`"),
        ("`{x}` is private to the file `{file}`", "`{x}` é privado do arquivo `{file}`"),
        ("private", "privado"),
        ("a name starting with `_` can only be used in its own file", "um nome que começa com `_` só pode ser usado no próprio arquivo"),
        ("to use it from other files, rename it without `_` in `{file}`", "para usá-lo em outros arquivos, renomeie-o sem `_` em `{file}`"),
        ("the file `{file}` has no `{x}`", "o arquivo `{file}` não tem `{x}`"),
        ("not in `{file}`", "não está em `{file}`"),
        ("did you mean `{m}.{best}`?", "você quis dizer `{m}.{best}`?"),
        ("`{file}` defines nothing that other files can use", "`{file}` não define nada que outros arquivos possam usar"),
        ("`{file}` gives: {}{more}", "`{file}` oferece: {}{more}"),
        ("`{name}` is private to the file `{file}`", "`{name}` é privado do arquivo `{file}`"),
        ("unknown name `{name}`", "nome desconhecido `{name}`"),
        ("not defined here", "não definido aqui"),
        ("`{name}` is defined in `{file}`: write `{m}.{name}`", "`{name}` está definido em `{file}`: escreva `{m}.{name}`"),
        ("`{name}` is defined in `{file}`: add `use {stem}` at the top of the file, then write `{stem}.{name}`", "`{name}` está definido em `{file}`: adicione `use {stem}` no início do arquivo e depois escreva `{stem}.{name}`"),
        ("not defined", "não definido"),
        ("defined here, further down", "definido aqui, mais abaixo"),
        ("in a block, a function can be used only after it, unless they are written one after the other with nothing between them: move `{name}` up, next to the function that uses it", "em um bloco, uma função só pode ser usada depois dela, a não ser que estejam escritas uma logo após a outra, sem nada entre elas: suba `{name}`, para perto da função que a usa"),
        ("in a block, a name can be used only after the line that defines it: move `{name} = ...` up", "em um bloco, um nome só pode ser usado depois da linha que o define: suba `{name} = ...`"),
        ("did you mean `{best}`?", "você quis dizer `{best}`?"),
        ("to create your own kind of value, define a type: `type Name = {name}(field) | Other`", "para criar o seu próprio tipo de valor, defina um tipo: `type Name = {name}(field) | Other`"),
        ("define it before using it: `{name} = ...` or `fn {name}(...) = ...`", "defina-o antes de usá-lo: `{name} = ...` ou `fn {name}(...) = ...`"),
        ("this definition of `{name}` can never be used", "esta definição de `{name}` nunca pode ser usada"),
        ("this definition already accepts every argument", "esta definição já aceita qualquer argumento"),
        ("definitions are tried in order: put the most general one last", "as definições são testadas em ordem: coloque a mais geral por último"),
        ("this definition already accepts these arguments", "esta definição já aceita esses argumentos"),
        ("definitions are tried in order: remove this one, or change its patterns", "as definições são testadas em ordem: remova esta, ou mude os padrões dela"),
        ("this function receives {}, which never matches this pattern", "esta função recebe {}, que nunca corresponde a este padrão"),
        ("never matches", "nunca corresponde"),
        ("`{name}` does not handle every possible argument", "`{name}` não trata todos os argumentos possíveis"),
        ("not handled: {}", "não tratado: {}"),
        ("add a definition for it, like `fn {name}({}) = ...`", "adicione uma definição para isso, como `fn {name}({}) = ...`"),
        ("the value {} does not match this pattern", "o valor {} não corresponde a este padrão"),
        ("a value of type {} never matches this pattern", "um valor do tipo {} nunca corresponde a este padrão"),
        ("`_` can only be used in patterns", "`_` só pode ser usado em padrões"),
        ("`_` means \"anything, ignored\"; give the value a real name to use it", "`_` significa \"qualquer coisa, ignorada\"; dê um nome de verdade ao valor para usá-lo"),
        ("`?` can only be used inside a function", "`?` só pode ser usado dentro de uma função"),
        ("outside of a function, handle the value with `match` or `unwrap_or`", "fora de uma função, trate o valor com `match` ou `unwrap_or`"),
        ("`?` needs an Option or a Result, but got {}", "`?` precisa de uma Option ou de um Result, mas recebeu {}"),
        ("`?` takes the value out of `Some(x)` / `Ok(x)`, and returns `None` / `Err(e)` from the current function", "`?` tira o valor de dentro de `Some(x)` / `Ok(x)`, e devolve `None` / `Err(e)` da função atual"),
        ("the field `{k}` appears twice", "o campo `{k}` aparece duas vezes"),
        ("`..` expects {expected} here, but got {}", "`..` espera {expected} aqui, mas recebeu {}"),
        ("this value is not a function", "este valor não é uma função"),
        ("cannot be called", "não pode ser chamado"),
        ("to multiply, write `*` explicitly: `2 * (x + 1)`", "para multiplicar, escreva `*` explicitamente: `2 * (x + 1)`"),
        ("to take one element, use brackets: `x[0]`; between two values, a `,` may be missing", "para pegar um elemento, use colchetes: `x[0]`; entre dois valores, talvez falte uma `,`"),
        ("to read a field, use a dot: `person.name`", "para ler um campo, use um ponto: `person.name`"),
        ("`{s}` is not a function", "`{s}` não é uma função"),
        ("`{s}` is {}", "`{s}` é {}"),
        ("returns {returned}", "devolve {returned}"),
        ("this assertion always fails: {a} and {b} are never equal", "esta asserção sempre falha: {a} e {b} nunca são iguais"),
        (
            r#"one value is inside an Option or a Result: compare it with `Some(...)` or `Ok(...)`,
like `assert_eq(parse_int("3"), Ok(3))`"#,
            r#"um dos valores está dentro de uma Option ou de um Result: compare-o com `Some(...)` ou `Ok(...)`,
como `assert_eq(parse_int("3"), Ok(3))`"#,
        ),
        ("values of different kinds are never equal: Rythme never converts values silently", "valores de tipos diferentes nunca são iguais: o Rythme nunca converte valores sem avisar"),
        ("a constructor, shown as `<constructor ...>`", "um construtor, mostrado como `<constructor ...>`"),
        ("a function, shown as `<fn ...>`", "uma função, mostrada como `<fn ...>`"),
        ("this shows a function, not a value", "isto mostra uma função, não um valor"),
        ("to show what it gives, call it: `{call}`", "para mostrar o que ela dá, chame-a: `{call}`"),
        ("no definition of `{name}` accepts these arguments: {name}({})", "nenhuma definição de `{name}` aceita estes argumentos: {name}({})"),
        ("called here", "chamado aqui"),
        ("does not match", "não corresponde"),
        ("add a definition that handles this case, for example one that ends with a plain name or `_`", "adicione uma definição que trate este caso, por exemplo uma que termine com um nome simples ou `_`"),
        ("the field `{name}` is not a function", "o campo `{name}` não é uma função"),
        ("`{name}` is {}", "`{name}` é {}"),
        ("to read the field, write it without parentheses: `.{name}`", "para ler o campo, escreva-o sem parênteses: `.{name}`"),
        ("there is no function named `{shown}`", "não existe nenhuma função chamada `{shown}`"),
        ("`value.{shown}(...)` calls `{shown}(value, ...)`, or the field `{shown}` of a record", "`value.{shown}(...)` chama `{shown}(value, ...)`, ou o campo `{shown}` de um registro"),
        ("`{name}` is a value, not a function", "`{name}` é um valor, não uma função"),
        ("write `{name}` without parentheses", "escreva `{name}` sem parênteses"),
        ("this record has no field `{name}`", "este registro não tem o campo `{name}`"),
        ("unknown field", "campo desconhecido"),
        ("the record is empty: {{}}", "o registro está vazio: {{}}"),
        ("its fields are: {}", "os campos dele são: {}"),
        ("to get an Option instead of an error, use `get(record, \"{name}\")`", "para receber uma Option em vez de um erro, use `get(record, \"{name}\")`"),
        ("{} has no fields", "{} não tem campos"),
        ("no field `{name}` here", "nenhum campo `{name}` aqui"),
        ("did you mean `.{best}`? (`value.{best}` calls `{best}(value)`)", "você quis dizer `.{best}`? (`value.{best}` chama `{best}(value)`)"),
        ("`{c}` has no field `{name}`", "`{c}` não tem o campo `{name}`"),
        ("no variant of `{tn}` has a field `{name}`", "nenhuma variante de `{tn}` tem o campo `{name}`"),
        ("`{c}` has no fields", "`{c}` não tem campos"),
        ("the fields of `{c}` are: {}", "os campos de `{c}` são: {}"),
        ("the variants of `{tn}` have no fields", "as variantes de `{tn}` não têm campos"),
        ("their fields are: {}", "os campos delas são: {}"),
        ("a value of type `{tn}` may be another variant: use `match` to handle each case", "um valor do tipo `{tn}` pode ser outra variante: use `match` para tratar cada caso"),
        ("cannot use `[...]` on {}", "não é possível usar `[...]` em {}"),
        ("`[...]` works on lists, texts, records and dicts", "`[...]` funciona com listas, textos, registros e dicionários"),
        ("an index must be a whole number (Int), but got {}", "um índice deve ser um número inteiro (Int), mas recebeu {}"),
        ("the name of a field must be a text, but got {}", "o nome de um campo deve ser um texto, mas recebeu {}"),
        ("read a field with `record[\"name\"]` or `record.name`", "leia um campo com `record[\"name\"]` ou `record.name`"),
        ("this record has no field `{k}`", "este registro não tem o campo `{k}`"),
        ("use `get(record, key)` to receive an Option instead", "use `get(record, key)` para receber uma Option em vez disso"),
        ("{} and {}", "{} e {}"),
        ("a range needs whole numbers, but got {got}", "um intervalo precisa de números inteiros, mas recebeu {got}"),
        ("example: `1..10` is the list of the numbers from 1 to 10", "exemplo: `1..10` é a lista dos números de 1 a 10"),
        ("cannot use `-` on {}", "não é possível usar `-` com {}"),
        ("a condition must be `true` or `false`, but this is {}", "uma condição deve ser `true` ou `false`, mas isto é {}"),
        ("not a Bool", "não é um Bool"),
        ("compare it explicitly, for example `x != 0` or `x > 0`", "compare explicitamente, por exemplo `x != 0` ou `x > 0`"),
        ("to test if it is empty, use `is_empty(x)` or `not is_empty(x)`", "para testar se está vazio, use `is_empty(x)` ou `not is_empty(x)`"),
        ("use `is_some(x)`, `is_ok(x)`, or `match` to look inside", "use `is_some(x)`, `is_ok(x)` ou `match` para olhar dentro"),
        ("cannot use `{sym}` with {ln} and {rn}", "não é possível usar `{sym}` com {ln} e {rn}"),
        ("cannot use `{sym}` with {ln}", "não é possível usar `{sym}` com {ln}"),
        ("cannot use `{sym}` with {rn}", "não é possível usar `{sym}` com {rn}"),
        ("`{name} -x` subtracts here; to call `{name}`, write `{name}(-x)` (a call without parentheses starts a statement or a result, such as a branch of `if` or the body after the `=` of a function, and its name is not given a value with `=` or as a parameter)", "`{name} -x` subtrai aqui; para chamar `{name}`, escreva `{name}(-x)` (uma chamada sem parênteses começa uma instrução ou um resultado, como um ramo de `if` ou o corpo depois do `=` de uma função, e o nome dela não recebe um valor com `=` nem como parâmetro)"),
        ("to join two sets or two dicts, use `union(a, b)`; to take one away from the other, `difference(a, b)`", "para juntar dois conjuntos ou dois dicionários, use `union(a, b)`; para tirar um do outro, `difference(a, b)`"),
        (
            r#"to build a text with a value inside, use braces: "total: {{x}}"
or convert the value first: text(x)"#,
            r#"para montar um texto com um valor dentro, use chaves: "total: {{x}}"
ou converta o valor primeiro: text(x)"#,
        ),
        ("to add one element to a list, use `push(list, x)` or `[..list, x]`", "para adicionar um elemento a uma lista, use `push(list, x)` ou `[..list, x]`"),
        ("to repeat a text or a list, use `repeat(x, n)`", "para repetir um texto ou uma lista, use `repeat(x, n)`"),
        ("convert the text to a number first, with `int(t)` or `float(t)`", "converta o texto em número primeiro, com `int(t)` ou `float(t)`"),
        ("only values of the same kind can be compared", "só valores do mesmo tipo podem ser comparados"),
        ("this comparison is always false: {ln} and {rn} are never equal", "esta comparação é sempre falsa: {ln} e {rn} nunca são iguais"),
        ("this comparison is always true: {ln} and {rn} are never equal", "esta comparação é sempre verdadeira: {ln} e {rn} nunca são iguais"),
        ("to look inside an Option, use `match`, `unwrap_or(x, default)` or `is_some(x)`", "para olhar dentro de uma Option, use `match`, `unwrap_or(x, default)` ou `is_some(x)`"),
        ("Rythme never converts values silently: convert one side first, with `text(x)` or `int(t)`", "o Rythme nunca converte valores sem avisar: converta um dos lados primeiro, com `text(x)` ou `int(t)`"),
        ("this case can never be reached", "este caso nunca pode ser alcançado"),
        ("this case already handles everything", "este caso já trata tudo"),
        ("cases are tried in order: put `_` last", "os casos são testados em ordem: coloque `_` por último"),
        ("this case already handles these values", "este caso já trata esses valores"),
        ("a value goes to the first case that fits: remove this case, or change it", "um valor vai para o primeiro caso que serve: remova este caso, ou mude-o"),
        ("no case of this `match` handles {}", "nenhum caso deste `match` trata {}"),
        ("this value", "este valor"),
        ("add a case for this value, or a last case `_ -> ...` that handles everything else", "adicione um caso para este valor, ou um último caso `_ -> ...` que trate todo o resto"),
        ("this `match` does not handle every case", "este `match` não trata todos os casos"),
        ("add a last case `_ -> ...` for all the other values", "adicione um último caso `_ -> ...` para todos os outros valores"),
        ("add the missing cases, like `{} -> ...`, or a last case `_ -> ...`", "adicione os casos que faltam, como `{} -> ...`, ou um último caso `_ -> ...`"),
        ("unknown constructor `{n}`", "construtor desconhecido `{n}`"),
        ("constructors are defined with `type`: `type Name = {n}(field) | Other`", "os construtores são definidos com `type`: `type Name = {n}(field) | Other`"),
        ("`{name}` has {} field(s), but this pattern gives {}", "`{name}` tem {} campo(s), mas este padrão dá {}"),
        ("write it like `{example}`", "escreva-o como `{example}`"),
        ("this range never matches", "este intervalo nunca corresponde"),
        ("no number goes from {lo} up to {hi}", "nenhum número vai de {lo} até {hi}"),
        ("a range goes from the smaller number to the larger one: `{hi}..{lo}`", "um intervalo vai do número menor ao maior: `{hi}..{lo}`"),
        ("patterns joined with `|` cannot give names (here `{s}`)", "padrões unidos com `|` não podem dar nomes (aqui `{s}`)"),
        ("write separate cases instead", "escreva casos separados em vez disso"),
        ("handles some of these numbers", "trata alguns destes números"),
        ("together, the cases before it already handle all its numbers: remove it, or change it", "juntos, os casos anteriores já tratam todos os números dele: remova-o, ou mude-o"),
        ("these ranges overlap", "estes intervalos se sobrepõem"),
        ("{both} is also here", "{both} também está aqui"),
        ("{both} goes here, the first that fits", "{both} vai para cá, o primeiro que serve"),
        ("both ends of a range are included: make the ranges follow each other without overlapping, like `0..9` then `10..19`", "as duas pontas de um intervalo estão incluídas: faça os intervalos se seguirem sem se sobrepor, como `0..9` e depois `10..19`"),
        ("use `match` to handle the different possible shapes of a value", "use `match` para tratar as diferentes formas possíveis de um valor"),
        ("a value of type {}", "um valor do tipo {}"),
        ("this line computes a value but does nothing with it", "esta linha calcula um valor mas não faz nada com ele"),
        (
            r#"a line that starts with `-` is a new statement. To continue the line above,
end that line with `-` instead:
  total = price -
    discount"#,
            r#"uma linha que começa com `-` é uma nova instrução. Para continuar a linha de cima,
termine aquela linha com `-` em vez disso:
  total = price -
    discount"#,
        ),
        (
            r#"to keep the value, give it a name: `result = ...`; to show it: `print(...)`.
Values never change: `x + 1` does not change `x`."#,
            r#"para guardar o valor, dê um nome a ele: `result = ...`; para mostrá-lo: `print(...)`.
Os valores nunca mudam: `x + 1` não muda `x`."#,
        ),
        // src/diagnostic.rs
        ("error", "erro"),
        ("warning", "aviso"),
        ("help", "ajuda"),
        ("call trace (most recent first):", "chamadas em andamento (a mais recente primeiro):"),
        ("an anonymous function", "uma função anônima"),
        ("in", "em"),
        ("called at", "chamada em"),
        ("and {} more calls", "e mais {} chamadas"),
        ("more about this error: `rythme explain {}`", "mais sobre este erro: `rythme explain {}`"),
        // src/doc.rs
        ("`{a}` needs a file name, like `rythme doc geometry.ry --out geometry.html`", "`{a}` precisa de um nome de arquivo, como `rythme doc geometria.ry --out geometria.html`"),
        ("unknown option `{o}` for `rythme doc`", "opção desconhecida `{o}` para `rythme doc`"),
        (
            r#"
did you mean `{best}`?"#,
            r#"
você quis dizer `{best}`?"#,
        ),
        (
            r#"
see `rythme help doc`"#,
            r#"
veja `rythme help doc`"#,
        ),
        (
            r#"which files? Example: rythme doc geometry.ry
  a folder documents all its .ry files: rythme doc .
  --html writes one HTML page instead of Markdown
  --out FILE writes to a file (a name ending with .html gives HTML)
see `rythme help doc`"#,
            r#"quais arquivos? Exemplo: rythme doc geometria.ry
  uma pasta documenta todos os seus arquivos .ry: rythme doc .
  --html escreve uma página HTML em vez de Markdown
  --out ARQUIVO escreve em um arquivo (um nome terminado em .html dá HTML)
veja `rythme help doc`"#,
        ),
        ("no .ry file in `{target}`", "nenhum arquivo .ry em `{target}`"),
        ("Documentation of {one}", "Documentação de {one}"),
        ("Documentation", "Documentação"),
        ("written: {file}", "escrito: {file}"),
        ("cannot write the file `{file}`: {e}", "não é possível escrever o arquivo `{file}`: {e}"),
        ("Types", "Tipos"),
        ("Functions", "Funções"),
        ("Values", "Valores"),
        ("Tests", "Testes"),
        ("Uses", "Usa"),
        ("Contents", "Conteúdo"),
        ("Built-in functions mentioned", "Funções embutidas mencionadas"),
        ("Example", "Exemplo"),
        ("Nothing public to document.", "Nada público para documentar."),
        ("Documentation written by `rythme doc`", "Documentação escrita por `rythme doc`"),
        (" Same as: {} (its Ruby names).", " O mesmo que: {} (os nomes em Ruby)."),
        ("Example:", "Exemplo:"),
        (" as {open}{n}{close}", " com o nome {open}{n}{close}"),
        (": ", ": "),
        // src/driver.rs
        ("`{shown}` is a folder, not a file", "`{shown}` é uma pasta, não um arquivo"),
        ("give a file inside it, like {}", "indique um arquivo dentro dela, como {}"),
        ("file `{shown}` not found", "arquivo `{shown}` não encontrado"),
        ("no permission to read the file `{shown}`", "sem permissão para ler o arquivo `{shown}`"),
        ("cannot read the file `{shown}`: {e}", "não é possível ler o arquivo `{shown}`: {e}"),
        ("the file `{shown}` is not text in the UTF-8 encoding (unexpected byte on line {line})", "o arquivo `{shown}` não é texto na codificação UTF-8 (byte inesperado na linha {line})"),
        ("it looks like it was saved in UTF-16", "parece que ele foi salvo em UTF-16"),
        ("save it again with the UTF-8 encoding (most editors have it in \"Save as\" or in the status bar)", "salve-o de novo com a codificação UTF-8 (a maioria dos editores tem isso em \"Salvar como\" ou na barra de status)"),
        ("(`{name}` now has {n} definitions, tried in order; :reset to start again)", "(`{name}` agora tem {n} definições, testadas em ordem; :reset para recomeçar)"),
        ("file `{p}` not found", "arquivo `{p}` não encontrado"),
        ("looked for: {}", "procurado: {}"),
        ("the Rythme files there are: {}{more}", "os arquivos do Rythme lá são: {}{more}"),
        ("the path is relative to the folder of the file that contains `use`", "o caminho é relativo à pasta do arquivo que contém `use`"),
        ("`{p}` uses itself", "`{p}` usa a si mesmo"),
        ("`{last}` and `{first}` use each other", "`{last}` e `{first}` usam um ao outro"),
        ("the circle: {}", "o círculo: {}"),
        ("files cannot use each other: move what both need into a third file, used by both", "os arquivos não podem usar um ao outro: mova o que os dois precisam para um terceiro arquivo, usado pelos dois"),
        ("the name `{n}` is given to two files: `{first}` and `{second}`", "o nome `{n}` é dado a dois arquivos: `{first}` e `{second}`"),
        ("give this one another name: `use ... as other_name`", "dê outro nome a este: `use ... as other_name`"),
        ("`{s}` is defined by two used files: `{first}` and `{second}`", "`{s}` é definido por dois arquivos usados: `{first}` e `{second}`"),
        ("use the files with their names, without quotes, and write which one you mean: `use geometry`, then `geometry.{s}`", "use os arquivos pelos nomes, sem aspas, e escreva qual você quer: `use geometry`, depois `geometry.{s}`"),
        // src/exercise.rs
        ("which exercise? Example: rythme exercise start 1   (`rythme exercise list` shows them all)", "qual exercício? Exemplo: rythme exercise start 1   (`rythme exercise list` mostra todos)"),
        ("which file? Example: rythme exercise check 01-values.ry", "qual arquivo? Exemplo: rythme exercise check 01-values.ry"),
        ("unknown action `{other}` for `rythme exercise`", "ação desconhecida `{other}` para `rythme exercise`"),
        (
            r#"
did you mean `rythme exercise {best}`?"#,
            r#"
você quis dizer `rythme exercise {best}`?"#,
        ),
        (
            r#"
the actions are: list, start NAME, check FILE (see `rythme help exercises`)"#,
            r#"
as ações são: list, start NOME, check ARQUIVO (veja `rythme help exercises`)"#,
        ),
        ("{} exercises for a first course, from the first values to tests, from ★☆☆ (easy) to ★★★.", "{} exercícios para um primeiro curso, dos primeiros valores até os testes, de ★☆☆ (fácil) a ★★★."),
        (
            r#"Start one:       rythme exercise start 1
Check your work: rythme exercise check {}.ry"#,
            r#"Comece um:              rythme exercise start 1
Verifique seu trabalho: rythme exercise check {}.ry"#,
        ),
        ("there is no exercise `{name}`", "não existe o exercício `{name}`"),
        (
            r#"
the exercises are numbered from 1 to {}: `rythme exercise list` shows them"#,
            r#"
os exercícios são numerados de 1 a {}: `rythme exercise list` mostra todos"#,
        ),
        ("`{file}` already exists here: open it to continue, or rename it to start again", "`{file}` já existe aqui: abra-o para continuar, ou renomeie-o para recomeçar"),
        ("cannot write `{file}`: {error}", "não é possível escrever `{file}`: {error}"),
        (
            r#"Created {file}: exercise {} · {title}  {level}

Open it, read the statement and the hints at the top, and replace each
`fail("to do")` with your code. Then check your work:

  rythme exercise check {file}"#,
            r#"Criado {file}: exercício {} · {title}  {level}

Abra-o, leia o enunciado e as dicas no topo, e substitua cada
`fail("to do")` pelo seu código. Depois verifique seu trabalho:

  rythme exercise check {file}"#,
        ),
        ("file `{name}` not found", "arquivo `{name}` não encontrado"),
        (
            r#"
to start this exercise: rythme exercise start {}"#,
            r#"
para começar este exercício: rythme exercise start {}"#,
        ),
        (
            r#"✗ The file has errors, so its tests could not run.
  Fix the first error above, then check again."#,
            r#"✗ O arquivo tem erros, então os testes dele não puderam rodar.
  Corrija o primeiro erro acima e verifique de novo."#,
        ),
        ("✗ This file has no tests. An exercise comes with its tests: is it the right file?", "✗ Este arquivo não tem testes. Um exercício vem com os seus testes: é o arquivo certo?"),
        ("✓ Well done: all {total} tests pass!", "✓ Muito bem: todos os {total} testes passam!"),
        (
            r#"  Next exercise: {title}  {level}
    rythme exercise start {}"#,
            r#"  Próximo exercício: {title}  {level}
    rythme exercise start {}"#,
        ),
        ("  That was the last exercise: you have finished them all. Congratulations!", "  Esse foi o último exercício: você terminou todos. Parabéns!"),
        ("✗ All {total} tests pass, but...", "✗ Todos os {total} testes passam, mas..."),
        ("✗ {passed} of {total} tests pass. Keep going!", "✗ {passed} de {total} testes passam. Continue assim!"),
        (
            r#"  - The tests of the exercise were changed: the exercise is solved when its own tests
    pass. `rythme exercise start {number}` gives a fresh copy (rename your file first)."#,
            r#"  - Os testes do exercício foram alterados: o exercício está resolvido quando os próprios testes
    dele passam. `rythme exercise start {number}` dá uma cópia nova (renomeie seu arquivo antes)."#,
        ),
        ("  - Some functions are still to do: replace each `fail(\"to do\")` with your code.", "  - Algumas funções ainda estão por fazer: substitua cada `fail(\"to do\")` pelo seu código."),
        (
            r#"  - Start with the first failure above: compare what you got with what was expected.
  - Stuck? Read the hints at the top of the file, or `rythme help NAME` for a function."#,
            r#"  - Comece pela primeira falha acima: compare o que você obteve com o que era esperado.
  - Travou? Leia as dicas no topo do arquivo, ou `rythme help NOME` para uma função."#,
        ),
        // src/format.rs
        ("`{name}` could not be formatted safely, so it was left as it is", "`{name}` não pôde ser formatado com segurança, então foi deixado como está"),
        ("this is a bug in Rythme, not in your program: please report it, with the file, at https://github.com/maxlestage/Rythme/issues", "isso é um bug do Rythme, não do seu programa: por favor, informe o problema, com o arquivo, em https://github.com/maxlestage/Rythme/issues"),
        ("unknown option `{o}` for `rythme fmt`", "opção desconhecida `{o}` para `rythme fmt`"),
        (
            r#"
see `rythme help fmt`"#,
            r#"
veja `rythme help fmt`"#,
        ),
        (
            r#"which files? Example: rythme fmt hello.ry
  a folder formats all its .ry files: rythme fmt .
  - reads the code from the standard input
  --check only checks, for continuous integration
see `rythme help fmt`"#,
            r#"quais arquivos? Exemplo: rythme fmt ola.ry
  uma pasta formata todos os seus arquivos .ry: rythme fmt .
  - lê o código da entrada padrão
  --check só verifica, para a integração contínua
veja `rythme help fmt`"#,
        ),
        ("needs formatting: {shown}", "precisa de formatação: {shown}"),
        ("cannot write the file `{shown}`: {e}", "não é possível escrever o arquivo `{shown}`: {e}"),
        ("formatted: {shown}", "formatado: {shown}"),
        ("needs formatting: <stdin>", "precisa de formatação: <stdin>"),
        ("{changed} file(s) formatted, {unchanged} already formatted", "{changed} arquivo(s) formatado(s), {unchanged} já formatado(s)"),
        ("✓ {unchanged} file(s) already formatted", "✓ {unchanged} arquivo(s) já formatado(s)"),
        ("✗ {changed} file(s) to format, {unchanged} already formatted; `rythme fmt` (without --check) formats them", "✗ {changed} arquivo(s) para formatar, {unchanged} já formatado(s); `rythme fmt` (sem --check) formata todos"),
        ("✗ {errors} file(s) left as they are because of errors", "✗ {errors} arquivo(s) deixado(s) como estão por causa de erros"),
        ("file or folder `{shown}` not found", "arquivo ou pasta `{shown}` não encontrado"),
        ("did you mean `{}`?", "você quis dizer `{}`?"),
        // src/help.rs
        (
            r#"Rythme {v} — help

Commands
  rythme                    interactive mode (try code line by line)
  rythme file.ry            run a program
  rythme check file.ry      find problems without running the program
  rythme test file.ry       run the tests of a file (or of a folder)
  rythme fmt file.ry        give the code its official layout (--check: only check)
  rythme doc file.ry        write its documentation, in Markdown (--html: a web page)
  rythme steps file.ry      show the calculation step by step (fib(3) → fib(2) + fib(1) → ...)
  rythme exercise           exercises checked automatically (list, start, check)
  rythme new NAME           create a project (rythme.toml, src/, tests/)
  rythme install            download the dependencies of the project
  rythme learn              guided tour of the language, step by step
  rythme help TOPIC         help on a topic or a function
  rythme explain E003       explain an error code
  rythme -e "code"          run a piece of code
  rythme lsp                language server, for editors (VS Code, Neovim...)
  option --lang CODE        language of the messages: en, fr, es, de, pt, zh (or RYTHME_LANG)

Topics (rythme help TOPIC)
  start       the language in one minute
  syntax      the whole syntax on one page
  functions   functions, |>, recursion
  lists       lists and their functions
  text        texts
  records     records {{name: value}}
  dicts       dictionaries and sets: dict(...), set(...)
  types       your own types
  match       choosing according to the shape of a value
  errors      Option, Result and ?
  functional  no loops: what to do instead
  tests       writing and running tests
  exercises   exercises to learn, with their tests
  modules     using several files
  system      files, folders, paths, options, input, exit codes
  projects    projects and packages: rythme new, rythme.toml, rythme install
  dates       dates and times: now(), today(), calendar, ISO 8601
  regex       searching, cutting and replacing with regular expressions
  repl        the interactive mode
  fmt         the official layout of the code (rythme fmt)
  doc         documentation from the ## comments (rythme doc)
  steps       the calculation shown step by step (rythme steps)
  editors     Rythme in your editor (VS Code, Neovim, Helix...)
  ruby        coming from Ruby: blocks, symbols, method chains
  builtins    every built-in function

A specific function: rythme help map
"#,
            r#"Rythme {v} — ajuda

Comandos
  rythme                    modo interativo (testar código linha por linha)
  rythme arquivo.ry         executar um programa
  rythme check arquivo.ry   encontrar problemas sem executar o programa
  rythme test arquivo.ry    executar os testes de um arquivo (ou de uma pasta)
  rythme fmt arquivo.ry     dar ao código o seu layout oficial (--check: só verificar)
  rythme doc arquivo.ry     escrever a documentação dele, em Markdown (--html: uma página web)
  rythme steps arquivo.ry   mostrar o cálculo passo a passo (fib(3) → fib(2) + fib(1) → ...)
  rythme exercise           exercícios verificados automaticamente (list, start, check)
  rythme new NOME           criar um projeto (rythme.toml, src/, tests/)
  rythme install            baixar as dependências do projeto
  rythme learn              visita guiada pela linguagem, passo a passo
  rythme help TEMA          ajuda sobre um tema ou uma função
  rythme explain E003       explicar um código de erro
  rythme -e "código"        executar um trecho de código
  rythme lsp                servidor de linguagem, para editores (VS Code, Neovim...)
  opção --lang CÓDIGO       idioma das mensagens: en, fr, es, de, pt, zh (ou RYTHME_LANG)

Temas (rythme help TEMA)
  start       a linguagem em um minuto
  syntax      toda a sintaxe em uma página
  functions   funções, |>, recursão
  lists       listas e suas funções
  text        textos
  records     registros {{name: value}}
  dicts       dicionários e conjuntos: dict(...), set(...)
  types       os seus próprios tipos
  match       escolher de acordo com a forma de um valor
  errors      Option, Result e ?
  functional  sem laços: o que fazer em vez disso
  tests       escrever e executar testes
  exercises   exercícios para aprender, com os seus testes
  modules     usar vários arquivos
  system      arquivos, pastas, caminhos, opções, entrada, códigos de saída
  projects    projetos e pacotes: rythme new, rythme.toml, rythme install
  dates       datas e horas: now(), today(), calendário, ISO 8601
  regex       buscar, cortar e substituir com expressões regulares
  repl        o modo interativo
  fmt         o layout oficial do código (rythme fmt)
  doc         documentação a partir dos comentários ## (rythme doc)
  steps       o cálculo mostrado passo a passo (rythme steps)
  editors     o Rythme no seu editor (VS Code, Neovim, Helix...)
  ruby        vindo do Ruby: blocos, símbolos, cadeias de métodos
  builtins    todas as funções embutidas

Uma função específica: rythme help map
"#,
        ),
        ("{alias}  (the Ruby name of `{of}`: the same function)", "{alias}  (o nome Ruby de `{of}`: a mesma função)"),
        (
            r#"pi
  The number π = 3.141592653589793."#,
            r#"pi
  O número π = 3.141592653589793."#,
        ),
        ("The function:", "A função:"),
        ("No help found for `{name}`.", "Nenhuma ajuda encontrada para `{name}`."),
        (
            r#"
Did you mean `{best}`?"#,
            r#"
Você quis dizer `{best}`?"#,
        ),
        (
            r#"
See the list of topics with `rythme help`."#,
            r#"
Veja a lista de temas com `rythme help`."#,
        ),
        ("keyword", "palavra-chave"),
        (
            r#"  More: rythme help {topic}
"#,
            r#"  Mais: rythme help {topic}
"#,
        ),
        (
            r#"  Same as: {} (its Ruby names)
"#,
            r#"  O mesmo que: {} (os nomes em Ruby)
"#,
        ),
        (
            r#"BUILT-IN FUNCTIONS
"#,
            r#"FUNÇÕES EMBUTIDAS
"#,
        ),
        ("the number π", "o número π"),
        ("{} (same as {})", "{} (o mesmo que {})"),
        ("  Ruby names:", "  Nomes Ruby:"),
        ("build an Option or a Result", "criar uma Option ou um Result"),
        (
            r#"
Details and example for one function: rythme help NAME (or help(NAME) in code)
"#,
            r#"
Detalhes e exemplo de uma função: rythme help NOME (ou help(NOME) no código)
"#,
        ),
        (
            r#"help(x) explains x: a function (help(map)), a topic (help("lists")), or a value (help([1, 2])).
Topics: start, syntax, functions, lists, text, records, types, match, errors, functional, tests, modules, builtins."#,
            r#"help(x) explica x: uma função (help(map)), um tema (help("lists")) ou um valor (help([1, 2])).
Temas: start, syntax, functions, lists, text, records, types, match, errors, functional, tests, modules, builtins."#,
        ),
        (
            r#"  (no documentation: write `## description` on the line above `fn`)
"#,
            r#"  (sem documentação: escreva `## descrição` na linha acima de `fn`)
"#,
        ),
        ("defined at", "definido em"),
        (
            r#"{}({})
  A constructor of the type `{}`.
"#,
            r#"{}({})
  Um construtor do tipo `{}`.
"#,
        ),
        (
            r#"This is a value of type {ty}: {}
"#,
            r#"Este é um valor do tipo {ty}: {}
"#,
        ),
        (
            r#"Useful functions: {}
"#,
            r#"Funções úteis: {}
"#,
        ),
        (
            r#"Also: {also}
"#,
            r#"Também: {also}
"#,
        ),
        (
            r#"Use `match` to handle each variant of `{}`.
"#,
            r#"Use `match` para tratar cada variante de `{}`.
"#,
        ),
        ("[Enter] next   [q] quit > ", "[Enter] seguinte   [q] sair > "),
        ("To practice, run `rythme` and type code, or do the exercises: `rythme exercise list`. `rythme help builtins` lists every function.", "Para praticar, execute `rythme` e digite código, ou faça os exercícios: `rythme exercise list`. `rythme help builtins` lista todas as funções."),
        // src/interp.rs
        ("this list would be too large (more than {MAX_LIST} elements)", "esta lista ficaria grande demais (mais de {MAX_LIST} elementos)"),
        ("this text would be too large (more than {mb} MB)", "este texto ficaria grande demais (mais de {mb} MB)"),
        ("does it grow in a function that calls itself? Check that the function always reaches a case where it stops", "ele cresce em uma função que chama a si mesma? Verifique se a função sempre chega a um caso em que para"),
        ("no function named `{}`", "nenhuma função chamada `{}`"),
        ("`value.{0}(...)` calls `{0}(value, ...)`", "`value.{0}(...)` chama `{0}(value, ...)`"),
        ("`{name}` is not defined yet", "`{name}` ainda não está definido"),
        ("the file runs from top to bottom: move this definition above the line that uses it (directly or through a function)", "o arquivo roda de cima para baixo: mova esta definição para cima da linha que a usa (diretamente ou por meio de uma função)"),
        ("`{ctor}` has no field `{name}`", "`{ctor}` não tem o campo `{name}`"),
        ("`{ctor}` has no fields", "`{ctor}` não tem campos"),
        ("the fields of `{ctor}` are: {}", "os campos de `{ctor}` são: {}"),
        ("a value of type `{}` may be another variant: use `match` to handle each case", "um valor do tipo `{}` pode ser outra variante: use `match` para tratar cada caso"),
        ("too many nested calls ({depth} calls in progress)", "chamadas aninhadas demais ({depth} chamadas em andamento)"),
        ("this call goes too deep", "esta chamada vai fundo demais"),
        (
            r#"a recursive function that never stops? Check that it always reaches a base case.
Otherwise, make the recursive call the very last thing the function does
(tail call), for example with an accumulator:
  fn sum([], acc) = acc
  fn sum([x, ..rest], acc) = sum(rest, acc + x)
or use `fold`, `map`, `filter`, which have no limit."#,
            r#"uma função recursiva que nunca para? Verifique se ela sempre chega a um caso base.
Senão, faça da chamada recursiva a última coisa que a função faz
(chamada de cauda), por exemplo com um acumulador:
  fn sum([], acc) = acc
  fn sum([x, ..rest], acc) = sum(rest, acc + x)
ou use `fold`, `map`, `filter`, que não têm limite."#,
        ),
        ("the program was stopped: it made more calls than allowed for this run", "o programa foi parado: ele fez mais chamadas do que o permitido para esta execução"),
        ("does it call itself forever? Check that it always reaches a base case", "ele chama a si mesmo para sempre? Verifique se ele sempre chega a um caso base"),
        ("{} is not a function", "{} não é uma função"),
        ("`{0}` is a value, not a function: write `{0}` without parentheses", "`{0}` é um valor, não uma função: escreva `{0}` sem parênteses"),
        ("`..` expects {what} here, but got {}", "`..` espera {what} aqui, mas recebeu {}"),
        ("division by zero", "divisão por zero"),
        (
            r#"check the divisor first: `if b == 0 then ... else a / b`,
or return a Result: `if b == 0 then Err("division by zero") else Ok(a / b)`"#,
            r#"verifique o divisor primeiro: `if b == 0 then ... else a / b`,
ou devolva um Result: `if b == 0 then Err("divisão por zero") else Ok(a / b)`"#,
        ),
        ("`0 ** -n` is `1 / 0 ** n`", "`0 ** -n` é `1 / 0 ** n`"),
        ("{min} argument{s}", "{min} argumento{s}"),
        ("at least {min} argument{s}", "pelo menos {min} argumento{s}"),
        ("{min} to {max} arguments", "de {min} a {max} argumentos"),
        ("{got} {verb}", "{got}"),
        ("`{name}` takes {expected}, but {given}", "`{name}` espera {expected}, mas recebeu {given}"),
        ("{got} argument(s) here", "{got} argumento(s) aqui"),
        ("usage: `{sig}`", "uso: `{sig}`"),
        ("cannot use `{sym}` with {lt} and {rt}", "não é possível usar `{sym}` com {lt} e {rt}"),
        ("`nan` (not a number) cannot be compared", "`nan` (não é um número) não pode ser comparado"),
        ("a negative number ({}) to a fractional power ({}) has no value among real numbers", "um número negativo ({}) elevado a uma potência fracionária ({}) não tem valor entre os números reais"),
        ("use `abs(x) ** p` for the power of its absolute value", "use `abs(x) ** p` para a potência do valor absoluto"),
        ("this range is too large ({} elements)", "este intervalo é grande demais ({} elementos)"),
        ("ranges are lists: keep them under {MAX_RANGE} elements", "os intervalos são listas: mantenha-os abaixo de {MAX_RANGE} elementos"),
        ("a range needs whole numbers, but got {} and {}", "um intervalo precisa de números inteiros, mas recebeu {} e {}"),
        ("index {n} is out of range: {what}", "o índice {n} está fora do intervalo: {what}"),
        ("valid indices go from 0 to {} (or -1 for the last element)", "os índices válidos vão de 0 a {} (ou -1 para o último elemento)"),
        ("to get an Option instead of an error, use `get(x, {n})`", "para receber uma Option em vez de um erro, use `get(x, {n})`"),
        ("the text is empty", "o texto está vazio"),
        ("the text has {len} character(s)", "o texto tem {len} caractere(s)"),
        ("the list has {} element(s)", "a lista tem {} elemento(s)"),
        ("this dict has no key {}", "este dicionário não tem a chave {}"),
        ("use `get(dict, key)` to receive an Option instead, or `has(dict, key)` to check first", "use `get(dict, key)` para receber uma Option em vez disso, ou `has(dict, key)` para verificar antes"),
        // src/lexer.rs
        ("`...` is not an operator in Rythme", "`...` não é um operador do Rythme"),
        ("write `1..10` for a range (10 included), or `[first, ..rest]` in a list", "escreva `1..10` para um intervalo (10 incluído), ou `[first, ..rest]` em uma lista"),
        ("`++` is not an operator in Rythme", "`++` não é um operador do Rythme"),
        (
            r#"to add one, write `x + 1` and give the result a new name (values never change);
to join two lists or two texts, use `+`: `[1] + [2]`"#,
            r#"para somar um, escreva `x + 1` e dê um novo nome ao resultado (os valores nunca mudam);
para juntar duas listas ou dois textos, use `+`: `[1] + [2]`"#,
        ),
        ("unexpected character `&`", "caractere inesperado `&`"),
        ("use `and` to combine two conditions", "use `and` para combinar duas condições"),
        ("this `{{` in a text is never closed", "esta `{{` em um texto nunca é fechada"),
        ("starts a value inserted in the text", "começa um valor inserido no texto"),
        ("close it with `}}`, or write `\\{{` to get a literal brace: \"\\{{\"", "feche-a com `}}`, ou escreva `\\{{` para ter uma chave literal: \"\\{{\""),
        ("unexpected invisible character {code}", "caractere invisível inesperado {code}"),
        ("this is a special space (often copied from a document or a web page): replace it with a normal space", "este é um espaço especial (muitas vezes copiado de um documento ou de uma página web): troque-o por um espaço normal"),
        ("this character cannot be seen (it is often copied from a document or a web page): delete it", "este caractere não pode ser visto (muitas vezes é copiado de um documento ou de uma página web): apague-o"),
        ("unexpected character `{c}`", "caractere inesperado `{c}`"),
        ("write `{s}` instead", "escreva `{s}` em vez disso"),
        ("texts use double quotes: \"hello\"", "os textos usam aspas duplas: \"olá\""),
        ("use `**` for powers: `2 ** 10`", "use `**` para potências: `2 ** 10`"),
        ("use straight double quotes `\"` for texts", "use aspas duplas retas `\"` para textos"),
        ("to insert a value in a text, use braces: \"total: {{x}}\"", "para inserir um valor em um texto, use chaves: \"total: {{x}}\""),
        ("invalid number `{text}`", "número inválido `{text}`"),
        ("hexadecimal numbers use the digits 0 to 9 and a to f, like `0xff`; binary numbers use 0 and 1, like `0b101`", "os números hexadecimais usam os algarismos de 0 a 9 e de a a f, como `0xff`; os binários usam 0 e 1, como `0b101`"),
        ("a name cannot start with a digit: `{}`", "um nome não pode começar com um algarismo: `{}`"),
        ("to multiply, write it explicitly: `{num} * {word}`", "para multiplicar, escreva explicitamente: `{num} * {word}`"),
        ("texts are nested too deeply inside `{{...}}`", "os textos estão aninhados fundo demais dentro de `{{...}}`"),
        ("this text is never closed", "este texto nunca é fechado"),
        ("the text starts here", "o texto começa aqui"),
        ("add a `\"` at the end of the text", "adicione um `\"` no fim do texto"),
        ("invalid unicode escape", "escape unicode inválido"),
        ("write it like `\\u{{e9}}` for é", "escreva como `\\u{{e9}}` para é"),
        ("unknown escape sequence in text", "sequência de escape desconhecida no texto"),
        ("in a regex, write the backslash twice: \"\\\\{c}\" is the text \\{c}", "em uma regex, escreva a barra invertida duas vezes: \"\\\\{c}\" é o texto \\{c}"),
        (
            r#"valid escapes: \n (new line), \t (tab), \r (carriage return), \0 (null character), \\ (backslash),
\" (quote), \{{ and \}} (braces), \# (a `#` before a brace), \u{{e9}} (a character by its Unicode number: é)"#,
            r#"escapes válidos: \n (nova linha), \t (tabulação), \r (retorno de carro), \0 (caractere nulo), \\ (barra invertida),
\" (aspas), \{{ e \}} (chaves), \# (um `#` antes de uma chave), \u{{e9}} (um caractere pelo seu número Unicode: é)"#,
        ),
        ("inside it, this text is never closed", "dentro dele, este texto nunca é fechado"),
        ("empty `{{}}` in a text", "`{{}}` vazio em um texto"),
        ("put a value inside, like \"{{name}}\", or write `\\{{` for a literal brace", "coloque um valor dentro, como \"{{name}}\", ou escreva `\\{{` para uma chave literal"),
        ("inside a text, `{{` starts an inserted value: to write braces, use `\\{{` and `\\}}`, like \"\\{{\\\"a\\\": 1\\}}\"", "dentro de um texto, `{{` começa um valor inserido: para escrever chaves, use `\\{{` e `\\}}`, como \"\\{{\\\"a\\\": 1\\}}\""),
        ("the number `{n}`", "o número `{n}`"),
        ("the number `{f}`", "o número `{f}`"),
        ("the name `{s}`", "o nome `{s}`"),
        ("the symbol `:{s}`", "o símbolo `:{s}`"),
        ("the end of the line", "o fim da linha"),
        ("the end of the code", "o fim do código"),
        ("the keyword `{k}`", "a palavra-chave `{k}`"),
        // src/lsp/analysis.rs
        ("cannot read the file `{shown}`", "não é possível ler o arquivo `{shown}`"),
        ("the file `{p}` has {n} error(s)", "o arquivo `{p}` tem {n} erro(s)"),
        ("first: {}{place}", "primeiro: {}{place}"),
        ("open `{p}` to see and fix them", "abra `{p}` para vê-los e corrigi-los"),
        // src/lsp/extras.rs
        ("▶ Run", "▶ Executar"),
        ("▶ Run the {n} tests", "▶ Executar os {n} testes"),
        ("▶ Run test", "▶ Executar teste"),
        // src/lsp/features.rs
        (
            r#"
more: `rythme explain {}`"#,
            r#"
mais: `rythme explain {}`"#,
        ),
        (
            r#"

Same as: {} (its Ruby names)"#,
            r#"

O mesmo que: {} (os nomes em Ruby)"#,
        ),
        ("**{alias}** — the Ruby name of `{of}`: the same function", "**{alias}** — o nome Ruby de `{of}`: a mesma função"),
        ("A value that is there, in an `Option`. The other case is `None`.", "Um valor que está presente, em uma `Option`. O outro caso é `None`."),
        ("No value, in an `Option`: what `find`, `first` or `get` give when there is nothing. Handle it with `match` or `unwrap_or`.", "Nenhum valor, em uma `Option`: o que `find`, `first` ou `get` dão quando não há nada. Trate-o com `match` ou `unwrap_or`."),
        ("The result of an operation that succeeded, in a `Result`. The other case is `Err(error)`.", "O resultado de uma operação que deu certo, em um `Result`. O outro caso é `Err(error)`."),
        ("The result of an operation that failed, in a `Result`. Inside a function, `?` stops and passes it on.", "O resultado de uma operação que falhou, em um `Result`. Dentro de uma função, `?` para e o repassa."),
        ("**{word}** is not used in Rythme.", "**{word}** não é usado no Rythme."),
        ("{} more definitions", "mais {} definições"),
        ("Inferred from its code:", "Deduzido do código:"),
        ("local function", "função local"),
        ("parameter of `{f}`", "parâmetro de `{f}`"),
        ("parameter of an anonymous function", "parâmetro de uma função anônima"),
        ("local value", "valor local"),
        ("given by the case `{pattern}` of a `match`", "dado pelo caso `{pattern}` de um `match`"),
        ("constructor of the type `{t}`", "construtor do tipo `{t}`"),
        ("*defined in `{name}`*", "*definido em `{name}`*"),
        ("The number π.", "O número π."),
        (
            r#"

`value.{n}(...)` calls `{n}(value, ...)`, or the field `{n}` of a record; `value.{n}` reads the field `{n}`, or calls `{n}(value)`"#,
            r#"

`value.{n}(...)` chama `{n}(value, ...)`, ou o campo `{n}` de um registro; `value.{n}` lê o campo `{n}`, ou chama `{n}(value)`"#,
        ),
        ("the file `{p}` is not found", "o arquivo `{p}` não foi encontrado"),
        (
            r#"

Its definitions are used through the name `{n}`: `{n}.name`"#,
            r#"

As definições dele são usadas pelo nome `{n}`: `{n}.name`"#,
        ),
        ("Defines:", "Define:"),
        ("name", "nome"),
        ("value", "valor"),
        ("other", "outro"),
        ("pattern", "padrao"),
        ("result", "resultado"),
        ("what it checks", "o que ele verifica"),
        ("actual", "obtido"),
        ("expected", "esperado"),
        ("condition", "condicao"),
        ("file", "arquivo"),
        ("a function on several lines", "uma função em várias linhas"),
        ("a function on one line", "uma função em uma linha"),
        ("choose according to the shape of a value", "escolher de acordo com a forma de um valor"),
        ("a test, run by `rythme test`", "um teste, executado por `rythme test`"),
        ("a test on several lines", "um teste em várias linhas"),
        ("choose between two values", "escolher entre dois valores"),
        ("type ${{1:Name}} = ${{2:First}}(${{3:field}}) | ${{0:Second}}", "type ${{1:Nome}} = ${{2:Primeiro}}(${{3:campo}}) | ${{0:Segundo}}"),
        ("your own type", "o seu próprio tipo"),
        ("several steps inside an expression", "várias etapas dentro de uma expressão"),
        ("a block: a function given as the last argument of a call", "um bloco: uma função dada como último argumento de uma chamada"),
        ("the opposite of `if`", "o contrário de `if`"),
        ("use another file: `use geometry`, then `geometry.area(2)`", "usar outro arquivo: `use geometry`, depois `geometry.area(2)`"),
        ("same as {}", "o mesmo que {}"),
        ("field", "campo"),
        ("package", "pacote"),
        ("folder", "pasta"),
        ("Add `use {file}` and write `{stem}.{written}`", "Adicionar `use {file}` e escrever `{stem}.{written}`"),
        ("Replace with `{new}`", "Substituir por `{new}`"),
        // src/lsp/mod.rs
        ("rythme lsp: unknown option `{other}` (ignored)", "rythme lsp: opção desconhecida `{other}` (ignorada)"),
        ("rythme lsp is the language server: your editor starts it and talks to it. See `rythme help editors`. (Ctrl-C to quit)", "rythme lsp é o servidor de linguagem: o seu editor o inicia e conversa com ele. Veja `rythme help editors`. (Ctrl-C para sair)"),
        ("the server is not initialized yet", "o servidor ainda não foi inicializado"),
        ("the server is shutting down", "o servidor está sendo encerrado"),
        ("internal error in Rythme (please report it): {}", "erro interno no Rythme (por favor, informe o problema): {}"),
        ("not formatted: the code has an error ({first})", "não formatado: o código tem um erro ({first})"),
        // src/lsp/refs.rs
        ("there is no name here", "não há nenhum nome aqui"),
        ("the name of a test is a text: edit it directly", "o nome de um teste é um texto: edite-o diretamente"),
        ("`{name}` cannot be found", "`{name}` não foi encontrado"),
        ("`{name}` is a built-in function: it cannot be renamed", "`{name}` é uma função embutida: ela não pode ser renomeada"),
        ("`{name}` is part of Rythme: it cannot be renamed", "`{name}` faz parte do Rythme: não pode ser renomeado"),
        ("`{name}` is not defined: there is nothing to rename", "`{name}` não está definido: não há nada para renomear"),
        ("`{new}` is not a valid name", "`{new}` não é um nome válido"),
        ("`{new}` is a keyword", "`{new}` é uma palavra-chave"),
        ("the names of types and constructors start with an uppercase letter", "os nomes de tipos e de construtores começam com letra maiúscula"),
        ("names start with a lowercase letter (uppercase is for types and constructors)", "os nomes começam com letra minúscula (a maiúscula é para tipos e construtores)"),
        ("`{new}` is a built-in function: choose another name", "`{new}` é uma função embutida: escolha outro nome"),
        ("`{shown}` has a syntax error: fix it before renaming", "`{shown}` tem um erro de sintaxe: corrija-o antes de renomear"),
        ("`{old_name}` is also the name of a record field, and `value.{old_name}(...)` may call that field: rename it by hand", "`{old_name}` também é o nome de um campo de registro, e `value.{old_name}(...)` pode chamar esse campo: renomeie-o à mão"),
        ("cannot rename `{old_name}` to `{new_name}`: {why}", "não é possível renomear `{old_name}` para `{new_name}`: {why}"),
        ("some names would then refer to something else", "alguns nomes passariam a se referir a outra coisa"),
        ("the code would not be valid", "o código não seria válido"),
        // src/manifest.rs
        ("expected a name here", "esperava um nome aqui"),
        ("each line is `name = \"value\"`, like `version = \"0.1.0\"`", "cada linha é `name = \"value\"`, como `version = \"0.1.0\"`"),
        ("this text is not closed", "este texto não está fechado"),
        ("add `{quote}` at the end of the text", "adicione `{quote}` no fim do texto"),
        ("unknown escape in a text", "escape desconhecido em um texto"),
        ("write `\\\\` for a backslash, or use `/` in paths: they work on every system", "escreva `\\\\` para uma barra invertida, ou use `/` nos caminhos: funciona em todos os sistemas"),
        ("expected `=` after `{key}`", "esperava `=` depois de `{key}`"),
        ("`{key}` is written twice", "`{key}` está escrito duas vezes"),
        ("expected `,` or `}}` here", "esperava `,` ou `}}` aqui"),
        ("a table is written on one line: `{{ git = \"...\", tag = \"...\" }}`", "uma tabela é escrita em uma linha: `{{ git = \"...\", tag = \"...\" }}`"),
        ("a value is a text, written in quotes", "um valor é um texto, escrito entre aspas"),
        ("write `\"{example}\"`", "escreva `\"{example}\"`"),
        ("expected `{close}` here", "esperava `{close}` aqui"),
        ("write `{key} = \"...\"`", "escreva `{key} = \"...\"`"),
        ("first written here", "escrito primeiro aqui"),
        ("unexpected text after the value", "texto inesperado depois do valor"),
        ("write one value per line; a comment starts with `#`", "escreva um valor por linha; um comentário começa com `#`"),
        ("`{name}` cannot be the name of a package: it is used in `use {name}/file`, so it must be a Rythme name in lower case (letters, digits, `_`)", "`{name}` não pode ser o nome de um pacote: ele é usado em `use {name}/file`, então deve ser um nome do Rythme em minúsculas (letras, algarismos, `_`)"),
        ("`{key}` is {what}, written in quotes", "`{key}` é {what}, escrito entre aspas"),
        ("unknown key `{key}`", "chave desconhecida `{key}`"),
        ("the keys here are {}", "as chaves aqui são {}"),
        ("write `name = \"{b}\"`", "escreva `name = \"{b}\"`"),
        ("`{v}` is not a version", "`{v}` não é uma versão"),
        ("a version is three numbers: `version = \"1.0.0\"` (major.minor.patch)", "uma versão são três números: `version = \"1.0.0\"` (maior.menor.correção)"),
        ("`entry` is the path of a `.ry` file in the project", "`entry` é o caminho de um arquivo `.ry` do projeto"),
        ("like `entry = \"src/main.ry\"`", "como `entry = \"src/main.ry\"`"),
        ("unknown section `{n}`", "seção desconhecida `{n}`"),
        ("write `name`, `version` and `entry` at the top of the file, without a section", "escreva `name`, `version` e `entry` no início do arquivo, sem seção"),
        ("the only section is `[dependencies]`", "a única seção é `[dependencies]`"),
        ("`{shown}` has no `name`", "`{shown}` não tem `name`"),
        ("add a line like `name = \"hello\"` at the top", "adicione uma linha como `name = \"ola\"` no início"),
        ("`{shown}` has no `version`", "`{shown}` não tem `version`"),
        ("add a line like `version = \"0.1.0\"` at the top", "adicione uma linha como `version = \"0.1.0\"` no início"),
        ("write `{name} = {{ git = \"https://...\", tag = \"v1.0.0\" }}` (or `commit = \"...\"`), or `{name} = {{ path = \"../folder\" }}`", "escreva `{name} = {{ git = \"https://...\", tag = \"v1.0.0\" }}` (ou `commit = \"...\"`), ou `{name} = {{ path = \"../pasta\" }}`"),
        ("call it `{b}`", "chame-o de `{b}`"),
        ("the project `{own}` cannot depend on itself", "o projeto `{own}` não pode depender de si mesmo"),
        ("`use {own}/file` already reaches its own files", "`use {own}/file` já alcança os próprios arquivos dele"),
        ("the dependency `{name}` is written as a table", "a dependência `{name}` é escrita como uma tabela"),
        ("the dependency `{name}` has both `git` and `path`", "a dependência `{name}` tem `git` e `path` ao mesmo tempo"),
        ("the dependency `{name}` needs `git` or `path`", "a dependência `{name}` precisa de `git` ou de `path`"),
        ("`tag` and `commit` are for a dependency from git, not from a folder", "`tag` e `commit` são para uma dependência vinda do git, não de uma pasta"),
        ("`{c}` is not a commit", "`{c}` não é um commit"),
        ("give a `tag` or a `commit`, not both", "dê uma `tag` ou um `commit`, não os dois"),
        ("the dependency `{name}` needs a fixed version: a `tag` or a `commit`", "a dependência `{name}` precisa de uma versão fixa: uma `tag` ou um `commit`"),
        ("the `git` address of `{name}` is empty", "o endereço `git` de `{name}` está vazio"),
        ("this file is written by `rythme install`: delete it and run `rythme install` again", "este arquivo é escrito por `rythme install`: apague-o e execute `rythme install` de novo"),
        ("unexpected line in `rythme.lock`", "linha inesperada em `rythme.lock`"),
        ("this entry of `rythme.lock` is not complete", "esta entrada de `rythme.lock` não está completa"),
        (
            r#"# The exact commit of each dependency from git, written by `rythme install`.
# Keep this file with the project, so that everyone gets the same code.
"#,
            r#"# O commit exato de cada dependência vinda do git, escrito por `rythme install`.
# Mantenha este arquivo com o projeto, para que todos recebam o mesmo código.
"#,
        ),
        // src/parser.rs
        ("all the definitions of `{name}` must have the same number of parameters", "todas as definições de `{name}` devem ter o mesmo número de parâmetros"),
        ("{} parameter(s) here", "{} parâmetro(s) aqui"),
        ("expected {expected}, found {found}", "esperava {expected}, encontrou {found}"),
        ("expected {expected} here", "esperava {expected} aqui"),
        ("to compare two values, use `==`", "para comparar dois valores, use `==`"),
        ("use `->` in `match` cases; a dict is built with `dict([[key, value], ...])`", "use `->` nos casos de `match`; um dicionário é criado com `dict([[key, value], ...])`"),
        ("use `and` to combine conditions", "use `and` para combinar condições"),
        ("use `or` to combine conditions", "use `or` para combinar condições"),
        ("use `not` to negate a condition: `not done`", "use `not` para negar uma condição: `not done`"),
        ("Rythme does not use braces for blocks: write `do` ... `end`, or put a function body on the following lines and finish with `end`", "o Rythme não usa chaves para blocos: escreva `do` ... `end`, ou coloque o corpo de uma função nas linhas seguintes e termine com `end`"),
        ("Rythme does not use `:` to start a block; see `rythme help syntax`", "o Rythme não usa `:` para começar um bloco; veja `rythme help syntax`"),
        ("`{word}` is not used in Rythme", "`{word}` não é usado no Rythme"),
        ("comes from another language", "vem de outra linguagem"),
        ("unexpected here: {found}", "inesperado aqui: {found}"),
        ("the statement should end before this", "a instrução deveria terminar antes disto"),
        ("to call a function, use parentheses: `{name}(...)`", "para chamar uma função, use parênteses: `{name}(...)`"),
        ("a block `do |x| ... end` is given to a call, on its line: `names.each do |name| print(name) end`", "um bloco `do |x| ... end` é dado a uma chamada, na linha dela: `names.each do |name| print(name) end`"),
        ("`else` must follow `if condition then value` (an `if` branch holds one expression; use `do ... end` for several lines)", "`else` deve vir depois de `if condição then valor` (um ramo de `if` contém uma expressão; use `do ... end` para várias linhas)"),
        ("put each statement on its own line", "coloque cada instrução na sua própria linha"),
        ("`{word}` can only be used at the top level of a file", "`{word}` só pode ser usado no nível mais alto de um arquivo"),
        ("move it outside of the function or block", "mova-o para fora da função ou do bloco"),
        ("before `=` comes a name or a pattern, like `[first, ..rest]`, `{{name, ..}}` or `Some(x)`", "antes de `=` vem um nome ou um padrão, como `[first, ..rest]`, `{{name, ..}}` ou `Some(x)`"),
        ("`=`", "`=`"),
        ("values cannot be changed in Rythme", "os valores não podem ser alterados no Rythme"),
        ("a record pattern cannot give a name to the other fields", "um padrão de registro não pode dar nome aos outros campos"),
        ("write `{{name, ..}}` to ignore the other fields; for a record without some fields, use `remove(record, \"name\")`", "escreva `{{name, ..}}` para ignorar os outros campos; para um registro sem alguns campos, use `remove(record, \"name\")`"),
        ("only a function can have a name that ends with `?`", "só uma função pode ter um nome que termina com `?`"),
        ("define it with `fn`: `fn {s}(x) = ...`; for a value, use a name without `?`", "defina-a com `fn`: `fn {s}(x) = ...`; para um valor, use um nome sem `?`"),
        ("invalid `..` in this pattern", "`..` inválido neste padrão"),
        ("`..name` must be last: `[first, ..rest]`", "`..name` deve vir por último: `[first, ..rest]`"),
        ("cannot give a value to this", "não é possível dar um valor a isto"),
        ("this is not a name", "isto não é um nome"),
        ("`=` gives a name to a value", "`=` dá um nome a um valor"),
        ("to define a function, write `fn name(x) = ...`", "para definir uma função, escreva `fn name(x) = ...`"),
        ("records cannot be changed; build a new one: `new = {{..old, {name}: value}}`", "os registros não podem ser alterados; crie um novo: `new = {{..old, {name}: value}}`"),
        ("values cannot be changed; build a new one: `set_at(list, i, x)` for a list, `put(record, name, x)` for a record", "os valores não podem ser alterados; crie um novo: `set_at(list, i, x)` para uma lista, `put(record, name, x)` para um registro"),
        ("a function name", "um nome de função"),
        ("`(` and the parameters", "`(` e os parâmetros"),
        ("even without parameters, write the parentheses: `fn greet() = print(\"hi\")`", "mesmo sem parâmetros, escreva os parênteses: `fn greet() = print(\"hi\")`"),
        ("functions use `=`, not `{arrow}`", "as funções usam `=`, não `{arrow}`"),
        ("write `fn(x) = x * 2`", "escreva `fn(x) = x * 2`"),
        ("`=` or a new line", "`=` ou uma nova linha"),
        (
            r#"a function is written `fn name(x) = expression`,
or with its body on the following lines, finished by `end`"#,
            r#"uma função se escreve `fn name(x) = expressão`,
ou com o corpo nas linhas seguintes, terminado por `end`"#,
        ),
        ("type names start with an uppercase letter", "os nomes de tipo começam com letra maiúscula"),
        ("write `type {cap} = ...`", "escreva `type {cap} = ...`"),
        ("a type name", "um nome de tipo"),
        ("example: `type Shape = Circle(radius) | Square(side)`", "exemplo: `type Shape = Circle(radius) | Square(side)`"),
        ("variant names start with an uppercase letter", "os nomes de variante começam com letra maiúscula"),
        ("write `{cap}`", "escreva `{cap}`"),
        ("a variant name, like `Circle`", "um nome de variante, como `Circle`"),
        ("a field name", "um nome de campo"),
        ("fields are named in lowercase: `Circle(radius)`", "os campos têm nomes em minúsculas: `Circle(radius)`"),
        ("the name of the test, as a text", "o nome do teste, como texto"),
        ("example: `test \"addition works\" = assert_eq(1 + 1, 2)`", "exemplo: `test \"a soma funciona\" = assert_eq(1 + 1, 2)`"),
        ("write the name of the file without `.ry`", "escreva o nome do arquivo sem `.ry`"),
        ("example: `use {path}`", "exemplo: `use {path}`"),
        ("a name for the file, in lowercase", "um nome para o arquivo, em minúsculas"),
        ("example: `use geometry as geo`, then `geo.area(2)`", "exemplo: `use geometry as geo`, depois `geo.area(2)`"),
        ("the name of a file", "o nome de um arquivo"),
        ("example: `use geometry` for the file `geometry.ry`, then `geometry.area(2)`", "exemplo: `use geometry` para o arquivo `geometry.ry`, depois `geometry.area(2)`"),
        ("for a file whose name starts with an uppercase letter, write its path as a text: `use \"Shapes.ry\" as shapes`", "para um arquivo cujo nome começa com letra maiúscula, escreva o caminho como texto: `use \"Shapes.ry\" as shapes`"),
        ("the end of the expression", "o fim da expressão"),
        ("comparisons cannot be chained", "as comparações não podem ser encadeadas"),
        ("combine them with `and`: `a < b and b < c`", "combine-as com `and`: `a < b and b < c`"),
        ("`{sym}` cannot follow a `|>` step directly", "`{sym}` não pode vir logo depois de uma etapa `|>`"),
        (
            r#"`|>` comes after the calculations: put the pipeline in parentheses,
like `(x |> f) {sym} 1`, or continue it with a function: `x |> f |> fn(v) = v {sym} 1`"#,
            r#"`|>` vem depois dos cálculos: coloque o pipeline entre parênteses,
como `(x |> f) {sym} 1`, ou continue-o com uma função: `x |> f |> fn(v) = v {sym} 1`"#,
        ),
        ("a block must follow a call", "um bloco deve vir depois de uma chamada"),
        ("not given to a function", "não foi dado a uma função"),
        (
            r#"write the block on the line of the call it is given to: `xs.map do |x| x * 2 end`;
for a function as a value, write `fn(x) = x * 2`"#,
            r#"escreva o bloco na linha da chamada a que ele é dado: `xs.map do |x| x * 2 end`;
para uma função como valor, escreva `fn(x) = x * 2`"#,
        ),
        ("`,` or `|`", "`,` ou `|`"),
        ("the parameters of a block go between bars: `do |name, age| ... end`", "os parâmetros de um bloco ficam entre barras: `do |name, age| ... end`"),
        ("`}}`", "`}}`"),
        ("the block starts here", "o bloco começa aqui"),
        ("a field or function name after `.`", "um nome de campo ou de função depois de `.`"),
        ("an expression", "uma expressão"),
        ("`]`", "`]`"),
        ("`,` or `{closing}`", "`,` ou `{closing}`"),
        ("separate the elements with commas", "separe os elementos com vírgulas"),
        ("this code is nested too deeply (more than {MAX_NESTING} levels)", "este código está aninhado fundo demais (mais de {MAX_NESTING} níveis)"),
        ("split it into several named values or functions", "divida-o em vários valores ou funções com nome"),
        ("this expression is too long or nested too deeply (more than {MAX_DEPTH} levels)", "esta expressão é longa demais ou está aninhada fundo demais (mais de {MAX_DEPTH} níveis)"),
        (
            r#"split it into several named values or functions;
to combine many values, put them in a list: `sum([a, b, c])`, `join([x, y, z])`"#,
            r#"divida-a em vários valores ou funções com nome;
para combinar muitos valores, coloque-os em uma lista: `sum([a, b, c])`, `join([x, y, z])`"#,
        ),
        ("Rythme has no tuples", "o Rythme não tem tuplas"),
        ("group values in a list `[a, b]` or a record `{{x: a, y: b}}`", "agrupe os valores em uma lista `[a, b]` ou em um registro `{{x: a, y: b}}`"),
        ("`)`", "`)`"),
        ("a named function cannot be used inside an expression", "uma função com nome não pode ser usada dentro de uma expressão"),
        ("define it on its own line, or use an anonymous function: `fn(x) = x * 2`", "defina-a em uma linha própria, ou use uma função anônima: `fn(x) = x * 2`"),
        ("comments start with `#`", "os comentários começam com `#`"),
        ("`else` must follow `if condition then value`; a branch holds a single expression (use `do ... end` for several lines)", "`else` deve vir depois de `if condição then valor`; um ramo contém uma única expressão (use `do ... end` para várias linhas)"),
        ("a record is written `{{name: \"Ada\", age: 36}}`; for a block of code, use `do ... end`", "um registro se escreve `{{name: \"Ada\", age: 36}}`; para um bloco de código, use `do ... end`"),
        ("`:` and a value", "`:` e um valor"),
        ("`then`", "`then`"),
        ("an `{word}` is written: `{word} condition then value else other_value`", "um `{word}` se escreve: `{word} condição then valor else outro_valor`"),
        ("this `{word}` has no `else`", "este `{word}` não tem `else`"),
        ("every `{word}` must have an `else`", "todo `{word}` deve ter um `else`"),
        ("expected `else` here", "esperava `else` aqui"),
        ("write `else if`", "escreva `else if`"),
        (
            r#"in Rythme, `unless` always gives a value, so it needs both cases:
  unless age >= 18 then "minor" else "adult""#,
            r#"no Rythme, `unless` sempre dá um valor, então precisa dos dois casos:
  unless age >= 18 then "menor" else "adulto""#,
        ),
        (
            r#"in Rythme, `if` always gives a value, so it needs both cases:
  if age >= 18 then "adult" else "minor""#,
            r#"no Rythme, `if` sempre dá um valor, então precisa dos dois casos:
  if age >= 18 then "adulto" else "menor""#,
        ),
        ("a branch holds a single expression; for several lines, use `do ... end`", "um ramo contém uma única expressão; para várias linhas, use `do ... end`"),
        ("a new line", "uma nova linha"),
        (
            r#"write each case on its own line, then `end`:
  match x
    0 -> "zero"
    _ -> "other"
  end"#,
            r#"escreva cada caso na sua própria linha, depois `end`:
  match x
    0 -> "zero"
    _ -> "outro"
  end"#,
        ),
        ("no `|` is needed before a case", "não é preciso `|` antes de um caso"),
        ("write each case as `pattern -> result`", "escreva cada caso como `padrão -> resultado`"),
        ("`->`", "`->`"),
        ("each case is written `pattern -> result`", "cada caso se escreve `padrão -> resultado`"),
        ("put each case on its own line", "coloque cada caso na sua própria linha"),
        ("a `match` needs at least one case", "um `match` precisa de pelo menos um caso"),
        ("a number after `..`", "um número depois de `..`"),
        ("only a text can come before `+` in a pattern", "só um texto pode vir antes de `+` em um padrão"),
        ("to look at the end of a text, use a guard: `s if ends_with(s, \"!\") -> ...`", "para olhar o fim de um texto, use uma condição de guarda: `s if ends_with(s, \"!\") -> ...`"),
        ("after `+`, a pattern takes a name or `_`", "depois de `+`, um padrão recebe um nome ou `_`"),
        ("a pattern cannot contain `{{...}}`", "um padrão não pode conter `{{...}}`"),
        ("use a guard instead: `s if s == \"...\" -> ...`", "use uma condição de guarda em vez disso: `s if s == \"...\" -> ...`"),
        ("`..rest` must be the last element: `[first, ..rest]`", "`..rest` deve ser o último elemento: `[first, ..rest]`"),
        ("`:` and a pattern", "`:` e um padrão"),
        ("a pattern", "um padrão"),
        ("patterns look like: `x`, `_`, `0`, `\"text\"`, `:ok`, `Some(x)`, `[first, ..rest]`, `{{name}}`", "os padrões são assim: `x`, `_`, `0`, `\"text\"`, `:ok`, `Some(x)`, `[first, ..rest]`, `{{name}}`"),
        ("the ends of a range in a pattern are whole numbers of 64 bits at most", "as pontas de um intervalo em um padrão são números inteiros de no máximo 64 bits"),
        ("too large for a range", "grande demais para um intervalo"),
        ("compare with a guard instead: `n if n >= low and n <= high -> ...`; a range still matches large numbers between its ends, and a decimal end like `1.0e30` works too", "compare com uma condição de guarda em vez disso: `n if n >= low and n <= high -> ...`; um intervalo ainda corresponde a números grandes entre as suas pontas, e uma ponta decimal como `1.0e30` também funciona"),
        ("a range in a pattern goes from a number to a number", "um intervalo em um padrão vai de um número a um número"),
        ("not a number", "não é um número"),
        ("the arguments of a call without parentheses are separated by commas: `print a, b`", "os argumentos de uma chamada sem parênteses são separados por vírgulas: `print a, b`"),
        ("write the numbers themselves, like `1..9` or `0.0..0.5`; to compare with names, use a guard: `n if n >= low and n <= high -> ...`", "escreva os próprios números, como `1..9` ou `0.0..0.5`; para comparar com nomes, use uma condição de guarda: `n if n >= low and n <= high -> ...`"),
        ("a text that starts with a text is written `\"Mr \" + name`: `name` is the rest of the text", "um texto que começa com um texto se escreve `\"Sr. \" + name`: `name` é o resto do texto"),
        ("missing `end` to close this `{what}`", "falta `end` para fechar este `{what}`"),
        ("this `{what}` is never closed", "este `{what}` nunca é fechado"),
        ("the code ends here", "o código termina aqui"),
        ("add `end` on its own line after the last line of the block", "adicione `end` em uma linha própria depois da última linha do bloco"),
        ("values never change in Rythme: give the new value a new name, like `total2 = total + 1`", "os valores nunca mudam no Rythme: dê um novo nome ao novo valor, como `total2 = total + 1`"),
        // src/project.rs
        ("no folder to keep the packages in", "nenhuma pasta para guardar os pacotes"),
        ("set the variable RYTHME_HOME to a folder, like `RYTHME_HOME=~/.rythme`", "defina a variável RYTHME_HOME com uma pasta, como `RYTHME_HOME=~/.rythme`"),
        ("the folder `{p}`", "a pasta `{p}`"),
        ("run `rythme install` here, in the folder of the project", "execute `rythme install` aqui, na pasta do projeto"),
        ("run `rythme install` in `{root}`, the folder of the project", "execute `rythme install` em `{root}`, a pasta do projeto"),
        ("the folder `{p}` of the package `{name}` does not exist", "a pasta `{p}` do pacote `{name}` não existe"),
        ("fix its `path` in `rythme.toml`", "corrija o `path` dele em `rythme.toml`"),
        ("the package `{name}` is not installed", "o pacote `{name}` não está instalado"),
        ("`rythme.toml` asks for it from {}", "`rythme.toml` pede esse pacote de {}"),
        ("`{shown_dir}` is not a Rythme package: it has no `rythme.toml`", "`{shown_dir}` não é um pacote do Rythme: não tem `rythme.toml`"),
        ("use one of its files: `use {package}/file`", "use um dos arquivos dele: `use {package}/file`"),
        ("the package `{package}` has no file `{entry}`", "o pacote `{package}` não tem o arquivo `{entry}`"),
        ("the package `{package}` has no file `{wanted}`", "o pacote `{package}` não tem o arquivo `{wanted}`"),
        ("did you mean `use {prefix}{best}`?", "você quis dizer `use {prefix}{best}`?"),
        ("its files there are: {}", "os arquivos dele lá são: {}"),
        ("did you mean `use {best}/{rest}`?", "você quis dizer `use {best}/{rest}`?"),
        ("in this project, `use` also reaches the files of the packages {}", "neste projeto, `use` também alcança os arquivos dos pacotes {}"),
        ("the program `git` was not found", "o programa `git` não foi encontrado"),
        ("Rythme uses git to download the dependencies that come from git", "o Rythme usa o git para baixar as dependências que vêm do git"),
        ("install Git (https://git-scm.com), then run `rythme install` again; dependencies with `path` do not need it", "instale o Git (https://git-scm.com) e depois execute `rythme install` de novo; as dependências com `path` não precisam dele"),
        ("git says: {said}", "o git diz: {said}"),
        ("the tag `{t}` of `{name}` does not exist in `{url}`", "a tag `{t}` de `{name}` não existe em `{url}`"),
        ("the commit `{c}` of `{name}` does not exist in `{url}`", "o commit `{c}` de `{name}` não existe em `{url}`"),
        ("did you mean `tag = \"{best}\"`?", "você quis dizer `tag = \"{best}\"`?"),
        ("its tags are: {}", "as tags dele são: {}"),
        ("it has no tags: give a `commit` instead", "ele não tem tags: dê um `commit` em vez disso"),
        ("cannot write in `{}`: {e}", "não é possível escrever em `{}`: {e}"),
        ("cannot download `{name}` from `{url}`", "não é possível baixar `{name}` de `{url}`"),
        ("cannot take the commit `{commit}` of `{name}`", "não é possível pegar o commit `{commit}` de `{name}`"),
        ("no `rythme.toml` in this folder or above it", "nenhum `rythme.toml` nesta pasta nem acima dela"),
        ("go to the folder of a project, or create one with `rythme new NAME`", "vá para a pasta de um projeto, ou crie um com `rythme new NOME`"),
        (
            r#"`rythme install` takes no argument (`{a}`): it installs the dependencies of `rythme.toml`
see `rythme help projects`"#,
            r#"`rythme install` não recebe nenhum argumento (`{a}`): ele instala as dependências do `rythme.toml`
veja `rythme help projects`"#,
        ),
        ("`{name}` has no dependencies: add them under `[dependencies]` in `rythme.toml`", "`{name}` não tem dependências: adicione-as em `[dependencies]` no `rythme.toml`"),
        ("Installing the dependencies of `{name}`", "Instalando as dependências de `{name}`"),
        ("the folder `{p}` of the package `{dep_name}` does not exist", "a pasta `{p}` do pacote `{dep_name}` não existe"),
        ("folder {p}", "pasta {p}"),
        ("downloaded", "baixado"),
        ("already there", "já estava lá"),
        ("`{dep_name}` is not a Rythme package: `{shown_dir}` has no `rythme.toml`", "`{dep_name}` não é um pacote do Rythme: `{shown_dir}` não tem `rythme.toml`"),
        ("a package is a project made by `rythme new`: its `rythme.toml`, and its files in `src/`", "um pacote é um projeto criado por `rythme new`: o seu `rythme.toml`, e os seus arquivos em `src/`"),
        ("  ✓ {dep_name} {version}: {line}", "  ✓ {dep_name} {version}: {line}"),
        ("cannot write `{}`: {e}", "não é possível escrever `{}`: {e}"),
        ("{count} package(s) ready", "{count} pacote(s) pronto(s)"),
        ("{count} package(s) ready ({more} needed by the other ones)", "{count} pacote(s) pronto(s) ({more} necessário(s) para os outros)"),
        ("which name? Example: rythme new hello", "qual nome? Exemplo: rythme new ola"),
        ("`rythme new` takes one name, like `rythme new hello`", "`rythme new` recebe um nome, como `rythme new ola`"),
        ("try `rythme new {b}`", "tente `rythme new {b}`"),
        ("`{shown_dir}` already exists", "`{shown_dir}` já existe"),
        ("choose another name, or another folder", "escolha outro nome, ou outra pasta"),
        ("Created the project `{name}`:", "Projeto `{name}` criado:"),
        ("Next: `cd {shown_dir}`, then `rythme run` to run it and `rythme test` to test it (see `rythme help projects`)", "Próximo passo: `cd {shown_dir}`, depois `rythme run` para executá-lo e `rythme test` para testá-lo (veja `rythme help projects`)"),
        ("greeting", "saudacao"),
        ("hello", "ola"),
        (
            r#"# The project: `rythme run`, `rythme test` and `rythme check` in this folder.
name = "{name}"
version = "0.1.0"
entry = "src/main.ry"

[dependencies]
# From git, at a tag or a commit, then `rythme install`:
# mathlib = {{ git = "https://github.com/someone/mathlib", tag = "v1.0.0" }}
# From a folder:
# tools = {{ path = "../tools" }}
"#,
            r#"# O projeto: `rythme run`, `rythme test` e `rythme check` nesta pasta.
name = "{name}"
version = "0.1.0"
entry = "src/main.ry"

[dependencies]
# Do git, em uma tag ou em um commit, depois `rythme install`:
# mathlib = {{ git = "https://github.com/alguem/mathlib", tag = "v1.0.0" }}
# De uma pasta:
# ferramentas = {{ path = "../ferramentas" }}
"#,
        ),
        (
            r#"# The program: `rythme run` in the folder of the project.
use {module}

print({module}.{func}("World"))
"#,
            r#"# O programa: `rythme run` na pasta do projeto.
use {module}

print({module}.{func}("mundo"))
"#,
        ),
        (
            r#"## A greeting for `name`.
fn {func}(name) = "Hello, {{name}}!"
"#,
            r#"## Uma saudação para `nome`.
fn {func}(nome) = "Olá, {{nome}}!"
"#,
        ),
        (
            r#"# The tests: `rythme test` in the folder of the project.
use {name}/{module}

test "{func}" = assert_eq({module}.{func}("Ada"), "Hello, Ada!")
"#,
            r#"# Os testes: `rythme test` na pasta do projeto.
use {name}/{module}

test "{func}" = assert_eq({module}.{func}("Ada"), "Olá, Ada!")
"#,
        ),
        (
            r#"# {name}

A [Rythme](https://github.com/maxlestage/Rythme) project.

```sh
rythme run        # run the program (src/main.ry)
rythme test       # run the tests (tests/)
rythme check      # find problems without running
rythme fmt        # give the code its official layout
rythme install    # download the dependencies of rythme.toml
```
"#,
            r#"# {name}

Um projeto [Rythme](https://github.com/maxlestage/Rythme).

```sh
rythme run        # executar o programa (src/main.ry)
rythme test       # executar os testes (tests/)
rythme check      # encontrar problemas sem executar
rythme fmt        # dar ao código o seu layout oficial
rythme install    # baixar as dependências do rythme.toml
```
"#,
        ),
        (
            r#"# Files that git does not keep (keep rythme.lock).
.DS_Store
Thumbs.db
*.swp
*~
"#,
            r#"# Os arquivos que o git não guarda (mantenha o rythme.lock).
.DS_Store
Thumbs.db
*.swp
*~
"#,
        ),
        ("src/greeting.ry", "src/saudacao.ry"),
        ("tests/greeting_test.ry", "tests/saudacao_test.ry"),
        // src/repl.rs
        (":help for help, :quit to leave", ":help para ajuda, :quit para sair"),
        ("(type :quit to leave)", "(digite :quit para sair)"),
        ("(cancelled)", "(cancelado)"),
        ("cannot read what is typed: {e}", "não é possível ler o que foi digitado: {e}"),
        ("usage: :doc NAME (a function, a topic...)", "uso: :doc NOME (uma função, um tema...)"),
        ("usage: :load file.ry", "uso: :load arquivo.ry"),
        ("{} loaded", "{} carregado"),
        ("Everything was forgotten.", "Tudo foi esquecido."),
        ("Messages are now in English.", "As mensagens agora estão em português."),
        ("usage: :lang fr  (or another of {codes})", "uso: :lang pt  (ou outro de {codes})"),
        ("unknown code; codes look like E003 or W001", "código desconhecido; os códigos são como E003 ou W001"),
        ("usage: :steps EXPRESSION, like :steps fib(4)", "uso: :steps EXPRESSÃO, como :steps fib(4)"),
        ("Nothing defined yet.", "Nada definido ainda."),
        ("unknown command `{other}`", "comando desconhecido `{other}`"),
        (" — did you mean `{best}`?", " — você quis dizer `{best}`?"),
        (":help lists the commands", ":help lista os comandos"),
        // src/steps.rs
        ("  ... (more steps, not shown: {n} are shown at most, change it with --max)", "  ... (mais etapas, não mostradas: no máximo {n} são mostradas, mude isso com --max)"),
        ("this calculation became too deep to be shown step by step (more than {MAX_DEPTH} operations inside each other)", "este cálculo ficou fundo demais para ser mostrado passo a passo (mais de {MAX_DEPTH} operações umas dentro das outras)"),
        ("try it with smaller values, or run the program normally: `rythme file.ry`", "tente com valores menores, ou execute o programa normalmente: `rythme arquivo.ry`"),
        ("the evaluation was stopped after {limit} steps: does it ever end?", "a avaliação foi parada depois de {limit} etapas: ela chega a terminar?"),
        ("a function that calls itself must always reach a case where it stops", "uma função que chama a si mesma deve sempre chegar a um caso em que para"),
        ("Nothing to show: this file only has definitions. Give an expression to compute, like: rythme steps file.ry \"fib(5)\"", "Nada para mostrar: este arquivo só tem definições. Dê uma expressão para calcular, como: rythme steps arquivo.ry \"fib(5)\""),
        ("`{flag}` needs a number, like `--max 50` or `--width 80` (20 at least)", "`{flag}` precisa de um número, como `--max 50` ou `--width 80` (no mínimo 20)"),
        (
            r#"usage: rythme steps file.ry [expression] [--max N] [--width N]
example: rythme steps fibonacci.ry "fib(4)""#,
            r#"uso: rythme steps arquivo.ry [expressão] [--max N] [--width N]
exemplo: rythme steps fibonacci.ry "fib(4)""#,
        ),
        // src/suggest.rs
        ("to join two lists or two texts, use `+`: `[1, 2] + [3]`; for a list of lists, use `flatten`", "para juntar duas listas ou dois textos, use `+`: `[1, 2] + [3]`; para uma lista de listas, use `flatten`"),
        ("in Rythme, this is called `{rythme}`", "no Rythme, isto se chama `{rythme}`"),
        ("booleans are written in lowercase: `true`", "os booleanos se escrevem em minúsculas: `true`"),
        ("booleans are written in lowercase: `false`", "os booleanos se escrevem em minúsculas: `false`"),
        ("Rythme has no null: use `None` for \"no value\" and `Some(x)` for a value", "o Rythme não tem null: use `None` para \"nenhum valor\" e `Some(x)` para um valor"),
        ("in Rythme, this is called `Some`", "no Rythme, isto se chama `Some`"),
        ("in Rythme, an error value is written `Err(...)`", "no Rythme, um valor de erro se escreve `Err(...)`"),
        ("in Rythme, a success value is written `Ok(...)`", "no Rythme, um valor de sucesso se escreve `Ok(...)`"),
        ("Rythme has no objects: functions receive their data as arguments, like `area(shape)` or `shape.area()`", "o Rythme não tem objetos: as funções recebem os seus dados como argumentos, como `area(shape)` ou `shape.area()`"),
        ("write a range with `..`: `1..10` is the list from 1 to 10 (included), and `0..10 |> step_by(2)` counts by 2", "escreva um intervalo com `..`: `1..10` é a lista de 1 a 10 (incluído), e `0..10 |> step_by(2)` conta de 2 em 2"),
        ("use the operator `//` for a whole division: `7 // 2` is 3", "use o operador `//` para uma divisão inteira: `7 // 2` é 3"),
        ("use the operator `%` for the remainder: `7 % 3` is 1", "use o operador `%` para o resto: `7 % 3` é 1"),
        ("use the operators `//` and `%`: `7 // 2` is 3 and `7 % 2` is 1", "use os operadores `//` e `%`: `7 // 2` é 3 e `7 % 2` é 1"),
        ("to put values in a text, use braces: \"total: {{x}}\"; to write a number with 2 decimals: `text(x, 2)`", "para colocar valores em um texto, use chaves: \"total: {{x}}\"; para escrever um número com 2 casas decimais: `text(x, 2)`"),
        ("read the whole file with `read_file(path)`, then cut it with `lines(text)`; the lines of the standard input are `input_lines()`", "leia o arquivo inteiro com `read_file(path)`, depois corte-o com `lines(text)`; as linhas da entrada padrão são `input_lines()`"),
        ("to merge two records, write `{{..a, ..b}}`: the fields of `b` replace those of `a`; for two dicts or two sets, `union(a, b)`", "para juntar dois registros, escreva `{{..a, ..b}}`: os campos de `b` substituem os de `a`; para dois dicionários ou dois conjuntos, `union(a, b)`"),
        ("write `unwrap_or(get(x, key), default)`: the value, or the default if it is missing", "escreva `unwrap_or(get(x, key), default)`: o valor, ou o valor padrão se ele faltar"),
        ("pair the lists, then combine each pair: `zip(a, b) |> map(fn([x, y]) = x + y)`", "junte as listas em pares, depois combine cada par: `zip(a, b) |> map(fn([x, y]) = x + y)`"),
        ("sort from the largest to the smallest with `sort(list) |> reverse`", "ordene do maior para o menor com `sort(list) |> reverse`"),
        ("compute the average with `sum(list) / len(list)`", "calcule a média com `sum(list) / len(list)`"),
        ("keep x between low and high with `max(low, min(x, high))`", "mantenha x entre low e high com `max(low, min(x, high))`"),
        ("count how many times each value appears with `group_by(list, fn(x) = x) |> map_values(len)`", "conte quantas vezes cada valor aparece com `group_by(list, fn(x) = x) |> map_values(len)`"),
        ("a decimal number x is NaN (not a number) when `x != x`", "um número decimal x é NaN (não é um número) quando `x != x`"),
        ("`n.times do |i| ... end` is written `(0..n - 1).each do |i| ... end`", "`n.times do |i| ... end` se escreve `(0..n - 1).each do |i| ... end`"),
        ("to use another file, write `use other_file`, then `other_file.name`", "para usar outro arquivo, escreva `use other_file`, depois `other_file.name`"),
        ("there is no `new`: build values directly, like `{{name: \"Ada\"}}` or `Circle(2.0)`", "não existe `new`: crie os valores diretamente, como `{{name: \"Ada\"}}` ou `Circle(2.0)`"),
        ("values never change in Rythme: build a new value instead", "os valores nunca mudam no Rythme: crie um novo valor em vez disso"),
        (
            r#"Rythme has no loops. Work on the whole list instead:
  1..5 |> each(fn(i) = print(i))
  [1, 2, 3] |> map(fn(x) = x * 2)
or write a recursive function. See `rythme help functional`."#,
            r#"o Rythme não tem laços. Trabalhe com a lista inteira em vez disso:
  1..5 |> each(fn(i) = print(i))
  [1, 2, 3] |> map(fn(x) = x * 2)
ou escreva uma função recursiva. Veja `rythme help functional`."#,
        ),
        ("Rythme has no loops to interrupt: use `find`, `take`, `filter`, or a recursive function", "o Rythme não tem laços para interromper: use `find`, `take`, `filter` ou uma função recursiva"),
        (
            r#"no `return` needed: the last expression of a function is its result.
To stop early with an error, return `Err(...)` and use `?`."#,
            r#"não é preciso `return`: a última expressão de uma função é o seu resultado.
Para parar antes com um erro, devolva `Err(...)` e use `?`."#,
        ),
        ("just write `name = value` (values never change, so no keyword is needed)", "escreva simplesmente `name = value` (os valores nunca mudam, então nenhuma palavra-chave é necessária)"),
        ("functions are written `fn name(x) = x * 2`, and anonymous ones `fn(x) = x * 2`", "as funções se escrevem `fn name(x) = x * 2`, e as anônimas `fn(x) = x * 2`"),
        (
            r#"Rythme has no classes. Describe your data with `type`:
  type Shape = Circle(radius) | Square(side)
or use a record: {{name: "Ada", age: 36}}"#,
            r#"o Rythme não tem classes. Descreva os seus dados com `type`:
  type Shape = Circle(radius) | Square(side)
ou use um registro: {{name: "Ada", age: 36}}"#,
        ),
        (
            r#"use `match`:
  match value
    0 -> "zero"
    _ -> "other"
  end"#,
            r#"use `match`:
  match value
    0 -> "zero"
    _ -> "outro"
  end"#,
        ),
        (
            r#"errors are values in Rythme: return `Ok(value)` or `Err(message)`,
then handle them with `match` or pass them on with `?`. `fail(message)` stops the program."#,
            r#"os erros são valores no Rythme: devolva `Ok(value)` ou `Err(message)`,
depois trate-os com `match` ou repasse-os com `?`. `fail(message)` para o programa."#,
        ),
        // src/testing.rs
        (".", ","),
        ("the test returned `false`", "o teste devolveu `false`"),
        ("use `assert_eq(actual, expected)` to see the values", "use `assert_eq(actual, expected)` para ver os valores"),
        ("the test ended with {}", "o teste terminou com {}"),
        ("a test passes when it ends without error, and its value is not `false`, `Err(...)` or `None`", "um teste passa quando termina sem erro, e o valor dele não é `false`, `Err(...)` nem `None`"),
        (
            r#"No tests found. Write one like this:
  test "addition" = assert_eq(1 + 1, 2)"#,
            r#"Nenhum teste encontrado. Escreva um assim:
  test "soma" = assert_eq(1 + 1, 2)"#,
        ),
        ("Running {n} test(s)", "Executando {n} teste(s)"),
        ("Running {n} of {total} test(s)", "Executando {n} de {total} teste(s)"),
        ("Running {n} test(s) of {file}", "Executando {n} teste(s) de {file}"),
        ("Running {n} of {total} test(s) of {file}", "Executando {n} de {total} teste(s) de {file}"),
        ("{f} file(s), ", "{f} arquivo(s), "),
        ("{total} test(s): {passed} passed, {failed} failed", "{total} teste(s): {passed} passaram, {failed} falharam"),
        (", {left_out} left out by the filter", ", {left_out} deixado(s) de fora pelo filtro"),
        (", {broken} file(s) with errors", ", {broken} arquivo(s) com erros"),
        ("No test name contains {}.", "Nenhum nome de teste contém {}."),
        (" or ", " ou "),
        ("{red}✗{reset} {}: the file has errors, so its tests could not run", "{red}✗{reset} {}: o arquivo tem erros, então os testes dele não puderam rodar"),
        (
            r#"No file with tests in {}. Write a test in a `.ry` file like this:
  test "addition" = assert_eq(1 + 1, 2)"#,
            r#"Nenhum arquivo com testes em {}. Escreva um teste em um arquivo `.ry` assim:
  test "soma" = assert_eq(1 + 1, 2)"#,
        ),
        ("Failed tests:", "Testes que falharam:"),
        ("`{name}` needs {what}", "`{name}` precisa de {what}"),
        ("a part of the name of a test", "uma parte do nome de um teste"),
        ("a number, or `random`", "um número, ou `random`"),
        ("the seed must be a whole number of 0 or more, or `random`, not `{v}`", "a semente deve ser um número inteiro igual ou maior que 0, ou `random`, não `{v}`"),
        ("unknown option `{name}` for `rythme test`", "opção desconhecida `{name}` para `rythme test`"),
        (
            r#"
the options are: --filter NAME, --seed N, --watch (see `rythme help tests`)"#,
            r#"
as opções são: --filter NOME, --seed N, --watch (veja `rythme help tests`)"#,
        ),
        ("which file or folder? Example: rythme test my_tests.ry, or rythme test . for the files of this folder", "qual arquivo ou pasta? Exemplo: rythme test meus_testes.ry, ou rythme test . para os arquivos desta pasta"),
        ("Seed: {}", "Semente: {}"),
        (
            r#"
Watching {} file(s): the tests run again when one changes (Ctrl+C to stop)."#,
            r#"
Observando {} arquivo(s): os testes rodam de novo quando um deles muda (Ctrl+C para parar)."#,
        ),
        (
            r#"
{} changed: running the tests again
"#,
            r#"
{} mudou: executando os testes de novo
"#,
        ),
        // src/value.rs
        ("constructor", "construtor"),
        // playground/src/lib.rs
        ("✓ no problems found", "✓ nenhum problema encontrado"),
        ("✓ no errors, {n} warning(s)", "✓ nenhum erro, {n} aviso(s)"),
        // help pages, explanations, keywords, functions, months
        (
            r#"RYTHME IN ONE MINUTE

Rythme is a simple language where values never change and everything is a
function. Programs are files ending with .ry.

  # A comment
  name = "World"                 # a name for a value (it never changes)
  print("Hello, {name}!")        # {...} inserts a value in a text

  fn square(x) = x * x           # a function
  print(square(4))               # 16

  [1, 2, 3, 4]
    |> filter(is_even)           # keep the even numbers
    |> map(square)               # square each one
    |> sum                       # add them up
    |> print                     # 20

Run it:     rythme hello.ry
Check it:   rythme check hello.ry
Try code:   rythme          (interactive mode)

Rythme helps you all the time: every error says where it is, why, and how
to fix it. `rythme explain E003` explains an error code in detail."#,
            r#"RYTHME EM UM MINUTO

O Rythme é uma linguagem simples em que os valores nunca mudam e tudo é uma
função. Os programas são arquivos que terminam em .ry.

  # Um comentário
  nome = "mundo"                 # um nome para um valor (ele nunca muda)
  print("Olá, {nome}!")          # {...} insere um valor em um texto

  fn quadrado(x) = x * x         # uma função
  print(quadrado(4))             # 16

  [1, 2, 3, 4]
    |> filter(is_even)           # ficar com os números pares
    |> map(quadrado)             # elevar cada um ao quadrado
    |> sum                       # somá-los
    |> print                     # 20

Execute:          rythme ola.ry
Verifique:        rythme check ola.ry
Teste código:     rythme          (modo interativo)

O Rythme ajuda você o tempo todo: cada erro diz onde está, por quê, e como
corrigi-lo. `rythme explain E003` explica um código de erro em detalhe."#,
        ),
        (
            r#"SYNTAX

Values
  42   -7   1_000_000            whole numbers (Int), of any size
  3.14   2.0                     decimal numbers (Float)
  "hello"   "total: {x}"         texts (Text); {...} (or #{...}) inserts a value
  true   false                   booleans (Bool)
  :ok   :error                   symbols (Symbol): a name used as a value
  [1, 2, 3]                      lists
  {name: "Ada", age: 36}         records
  Some(3)   None                 a value that may be missing (Option)
  Ok(3)   Err("oops")            a result that may fail (Result)
  1..5                           the list [1, 2, 3, 4, 5]

Names
  x = 5                          a name for a value; it never changes
  [a, b] = [1, 2]                take a value apart
  {name, age} = person

Operators
  + - * /   // (whole division)   % (remainder)   ** (power)
  == != < <= > >=   and   or   not
  x |> f(y)                      same as f(x, y): steps from left to right
  x.f(y)                         also the same as f(x, y)
  x.f                            the field f of x; if x has none, f(x): xs.sum
  xs                             a chain may go on, each line starting with .
    .select(even?)
    .sum

Functions
  fn double(x) = x * 2           on one line, with =
  fn greet(name)                 on several lines, finished by `end`
    message = "Hello {name}"
    print(message)               the last line gives the result
  end
  fn(x) = x + 1                  an anonymous function
  fn adult?(p) = p.age >= 18     a name may end with ? (see rythme help ruby)

Blocks: a function given as the last argument of a call
  xs.map do |x| x * 2 end        same as map(xs, fn(x) = x * 2)
  xs.each do |x|                 on several lines, finished by `end`
    print(x)
  end
  xs.select { |x| x > 2 }        with braces

Conditions (every if has an else)
  if x > 0 then "positive" else if x == 0 then "zero" else "negative"
  unless x > 0 then "small" else "big"     the opposite of if

Choosing by shape
  match list
    [] -> "empty"
    [x] -> "one element: {x}"
    [x, ..rest] -> "{x}, then {len(rest)} more"
  end

Several steps inside an expression
  do
    a = 1
    a + 1
  end

Your own types
  type Shape = Circle(radius) | Rect(width, height)

Tests and other files
  test "double works" = assert_eq(double(2), 4)
  use geometry                   then geometry.area(2)

Comments
  # an ordinary comment
  ## a documentation comment, above a function or a type, shown by help(...)"#,
            r#"SINTAXE

Valores
  42   -7   1_000_000            números inteiros (Int), de qualquer tamanho
  3.14   2.0                     números decimais (Float)
  "olá"   "total: {x}"           textos (Text); {...} (ou #{...}) insere um valor
  true   false                   booleanos (Bool)
  :ok   :error                   símbolos (Symbol): um nome usado como valor
  [1, 2, 3]                      listas
  {name: "Ada", age: 36}         registros
  Some(3)   None                 um valor que pode faltar (Option)
  Ok(3)   Err("ops")             um resultado que pode falhar (Result)
  1..5                           a lista [1, 2, 3, 4, 5]

Nomes
  x = 5                          um nome para um valor; ele nunca muda
  [a, b] = [1, 2]                desmontar um valor
  {name, age} = person

Operadores
  + - * /   // (divisão inteira)   % (resto)   ** (potência)
  == != < <= > >=   and   or   not
  x |> f(y)                      o mesmo que f(x, y): etapas da esquerda para a direita
  x.f(y)                         também o mesmo que f(x, y)
  x.f                            o campo f de x; se x não tiver, f(x): xs.sum
  xs                             uma cadeia pode continuar, cada linha começando com .
    .select(even?)
    .sum

Funções
  fn dobro(x) = x * 2            em uma linha, com =
  fn saudar(nome)                em várias linhas, terminada por `end`
    mensagem = "Olá {nome}"
    print(mensagem)              a última linha dá o resultado
  end
  fn(x) = x + 1                  uma função anônima
  fn adult?(p) = p.age >= 18     um nome pode terminar com ? (veja rythme help ruby)

Blocos: uma função dada como último argumento de uma chamada
  xs.map do |x| x * 2 end        o mesmo que map(xs, fn(x) = x * 2)
  xs.each do |x|                 em várias linhas, terminado por `end`
    print(x)
  end
  xs.select { |x| x > 2 }        com chaves

Condições (todo if tem um else)
  if x > 0 then "positivo" else if x == 0 then "zero" else "negativo"
  unless x > 0 then "pequeno" else "grande"     o contrário de if

Escolher pela forma
  match list
    [] -> "vazia"
    [x] -> "um elemento: {x}"
    [x, ..rest] -> "{x}, e mais {len(rest)}"
  end

Várias etapas dentro de uma expressão
  do
    a = 1
    a + 1
  end

Os seus próprios tipos
  type Shape = Circle(radius) | Rect(width, height)

Testes e outros arquivos
  test "dobro funciona" = assert_eq(dobro(2), 4)
  use geometry                   depois geometry.area(2)

Comentários
  # um comentário comum
  ## um comentário de documentação, acima de uma função ou de um tipo, mostrado por help(...)"#,
        ),
        (
            r#"FUNCTIONS

Define and call
  fn add(a, b) = a + b
  add(2, 3)                      # 5

On several lines: the last expression is the result
  fn describe(n)
    kind = if n % 2 == 0 then "even" else "odd"
    "{n} is {kind}"
  end

One definition per case (tried in order)
  fn fib(0) = 0
  fn fib(1) = 1
  fn fib(n) = fib(n - 1) + fib(n - 2)

With a condition (a guard)
  fn sign(n) if n < 0 = "negative"
  fn sign(0) = "zero"
  fn sign(_) = "positive"

Functions are values
  double = fn(x) = x * 2
  [1, 2, 3] |> map(double)       # [2, 4, 6]
  [1, 2, 3] |> map(fn(x) = x + 1)

Pipes read from left to right
  "  Hello  " |> trim |> upper   # "HELLO"
  x |> f(a, b)    means    f(x, a, b)

Method style
  "hello".upper()                # same as upper("hello")

Documentation
  ## Adds two numbers.
  fn add(a, b) = a + b
  help(add)                      # shows "Adds two numbers."

Recursion without limit
  When the recursive call is the very last thing a function does, it can
  repeat millions of times:
  fn count_down(0) = print("Liftoff!")
  fn count_down(n)
    print(n)
    count_down(n - 1)
  end"#,
            r#"FUNÇÕES

Definir e chamar
  fn somar(a, b) = a + b
  somar(2, 3)                    # 5

Em várias linhas: a última expressão é o resultado
  fn descrever(n)
    tipo = if n % 2 == 0 then "par" else "ímpar"
    "{n} é {tipo}"
  end

Uma definição por caso (testadas em ordem)
  fn fib(0) = 0
  fn fib(1) = 1
  fn fib(n) = fib(n - 1) + fib(n - 2)

Com uma condição (uma guarda)
  fn sinal(n) if n < 0 = "negativo"
  fn sinal(0) = "zero"
  fn sinal(_) = "positivo"

As funções são valores
  dobro = fn(x) = x * 2
  [1, 2, 3] |> map(dobro)        # [2, 4, 6]
  [1, 2, 3] |> map(fn(x) = x + 1)

Os pipes se leem da esquerda para a direita
  "  Olá  " |> trim |> upper     # "OLÁ"
  x |> f(a, b)    significa    f(x, a, b)

Estilo de método
  "olá".upper()                  # o mesmo que upper("olá")

Documentação
  ## Soma dois números.
  fn somar(a, b) = a + b
  help(somar)                    # mostra "Soma dois números."

Recursão sem limite
  Quando a chamada recursiva é a última coisa que uma função faz, ela pode
  se repetir milhões de vezes:
  fn contagem(0) = print("Decolar!")
  fn contagem(n)
    print(n)
    contagem(n - 1)
  end"#,
        ),
        (
            r#"LISTS

  numbers = [3, 1, 2]
  numbers[0]                     # 3 (the first; -1 is the last)
  len(numbers)                   # 3
  1..5                           # [1, 2, 3, 4, 5]
  [0, ..numbers]                 # [0, 3, 1, 2]   (.. inserts a whole list)
  numbers + [9]                  # [3, 1, 2, 9]

Work on the whole list (no loops needed)
  map(numbers, fn(x) = x * 10)            # [30, 10, 20]
  filter(numbers, fn(x) = x > 1)          # [3, 2]
  fold(numbers, 0, fn(acc, x) = acc + x)  # 6
  sort(numbers)                           # [1, 2, 3]
  numbers |> each(fn(x) = print(x))       # prints each element

Safe access (never an error)
  first([])                      # None
  get(numbers, 10)               # None

"Change" a list: you get a new list, the old one stays the same
  set_at(numbers, 0, 9)          # [9, 1, 2]
  insert_at(numbers, 1, 7)       # [3, 7, 1, 2]
  remove_at(numbers, -1)         # [3, 1]

Pieces of a list
  take_while(numbers, fn(x) = x > 2)      # [3]
  chunks([1, 2, 3, 4, 5], 2)              # [[1, 2], [3, 4], [5]]
  windows([1, 2, 3], 2)                   # [[1, 2], [2, 3]]
  0..10 |> step_by(5)                     # [0, 5, 10]

Take a list apart with patterns
  match numbers
    [] -> "empty"
    [first, ..rest] -> "{first}, then {len(rest)} more"
  end

All the functions: rythme help builtins"#,
            r#"LISTAS

  numbers = [3, 1, 2]
  numbers[0]                     # 3 (o primeiro; -1 é o último)
  len(numbers)                   # 3
  1..5                           # [1, 2, 3, 4, 5]
  [0, ..numbers]                 # [0, 3, 1, 2]   (.. insere uma lista inteira)
  numbers + [9]                  # [3, 1, 2, 9]

Trabalhe com a lista inteira (sem precisar de laços)
  map(numbers, fn(x) = x * 10)            # [30, 10, 20]
  filter(numbers, fn(x) = x > 1)          # [3, 2]
  fold(numbers, 0, fn(acc, x) = acc + x)  # 6
  sort(numbers)                           # [1, 2, 3]
  numbers |> each(fn(x) = print(x))       # mostra cada elemento

Acesso seguro (nunca um erro)
  first([])                      # None
  get(numbers, 10)               # None

"Mudar" uma lista: você recebe uma lista nova, a antiga continua igual
  set_at(numbers, 0, 9)          # [9, 1, 2]
  insert_at(numbers, 1, 7)       # [3, 7, 1, 2]
  remove_at(numbers, -1)         # [3, 1]

Pedaços de uma lista
  take_while(numbers, fn(x) = x > 2)      # [3]
  chunks([1, 2, 3, 4, 5], 2)              # [[1, 2], [3, 4], [5]]
  windows([1, 2, 3], 2)                   # [[1, 2], [2, 3]]
  0..10 |> step_by(5)                     # [0, 5, 10]

Desmontar uma lista com padrões
  match numbers
    [] -> "empty"
    [first, ..rest] -> "{first}, then {len(rest)} more"
  end

Todas as funções: rythme help builtins"#,
        ),
        (
            r#"TEXT

  name = "Ada"
  "Hello, {name}!"               # "Hello, Ada!"  ({...} inserts any expression)
  "2 + 2 = {2 + 2}"              # "2 + 2 = 4"
  "a" + "b"                      # "ab"
  len("été")                     # 3 (characters, accents included)
  upper("abc")   lower("ABC")   trim("  x  ")
  split("a,b,c", ",")            # ["a", "b", "c"]
  join(["a", "b"], "-")          # "a-b"
  replace("2024", "4", "5")      # "2025"
  contains("hello", "ell")       # true
  text(42)                       # "42"
  text(3.14159, 2)               # "3.14" (a number with 2 decimals)
  pad_left("7", 3, "0")          # "007"  (and pad_right: to line up columns)
  char_code("A")   char(66)      # 65 and "B"
  int("42")                      # 42
  parse_int("x42")               # Err(...): a conversion that cannot crash
  trim_start("  x")   trim_end("x  ")
  words("Hi, you!")              # ["Hi", "you"]
  equals_ignore_case("Ada", "ADA")          # true
  format_number(1234.5, 2)       # "1,234.50"   (with "fr": "1 234,50")
  find_all("3 cats, 12 dogs", "[0-9]+")     # ["3", "12"]  (regexes: rythme help regex)

Special characters: \n (new line), \t (tab), \r (carriage return),
\0 (null character), \" (quote), \\ (backslash), \{ and \} (braces, because
{ starts an insertion), \u{e9} (any character by its Unicode number: é)."#,
            r#"TEXTO

  name = "Ada"
  "Hello, {name}!"               # "Hello, Ada!"  ({...} insere qualquer expressão)
  "2 + 2 = {2 + 2}"              # "2 + 2 = 4"
  "a" + "b"                      # "ab"
  len("été")                     # 3 (caracteres, acentos incluídos)
  upper("abc")   lower("ABC")   trim("  x  ")
  split("a,b,c", ",")            # ["a", "b", "c"]
  join(["a", "b"], "-")          # "a-b"
  replace("2024", "4", "5")      # "2025"
  contains("hello", "ell")       # true
  text(42)                       # "42"
  text(3.14159, 2)               # "3.14" (um número com 2 casas decimais)
  pad_left("7", 3, "0")          # "007"  (e pad_right: para alinhar colunas)
  char_code("A")   char(66)      # 65 e "B"
  int("42")                      # 42
  parse_int("x42")               # Err(...): uma conversão que não pode quebrar
  trim_start("  x")   trim_end("x  ")
  words("Hi, you!")              # ["Hi", "you"]
  equals_ignore_case("Ada", "ADA")          # true
  format_number(1234.5, 2)       # "1,234.50"   (com "pt": "1.234,50")
  find_all("3 cats, 12 dogs", "[0-9]+")     # ["3", "12"]  (regex: rythme help regex)

Caracteres especiais: \n (nova linha), \t (tabulação), \r (retorno de carro),
\0 (caractere nulo), \" (aspas), \\ (barra invertida), \{ e \} (chaves, porque
{ começa uma inserção), \u{e9} (qualquer caractere pelo seu número Unicode: é)."#,
        ),
        (
            r#"RECORDS

A record groups named values:
  ada = {name: "Ada", born: 1815}
  ada.name                       # "Ada"
  ada["name"]                    # "Ada"

Records never change: build a new one from an old one
  older = {..ada, born: 1816}    # same fields, `born` replaced

Take apart
  {name, born} = ada
  fn greet({name}) = "Hello {name}"

Useful functions
  keys(ada)   values(ada)   has(ada, "name")   get(ada, "x")   put(ada, "x", 1)
  entries(ada)                   # [["name", "Ada"], ["born", 1815]]
  from_entries([["x", 1]])       # {x: 1}

Records as dictionaries
  counts = {apple: 2}
  put(counts, "pear", 5)         # {apple: 2, pear: 5}
  map_values(counts, fn(n) = n * 10)      # {apple: 20}
  group_by(["a", "b", "a"], fn(x) = x) |> map_values(len)   # {a: 2, b: 1}
  For keys that are not names (numbers, lists...), use a dict: rythme help dicts

JSON, the format of data on the web
  parse_json(text)               # Ok(...): objects become records, arrays lists
  to_json(value)                 # the text, ready to save with write_file"#,
            r#"REGISTROS

Um registro agrupa valores com nome:
  ada = {name: "Ada", born: 1815}
  ada.name                       # "Ada"
  ada["name"]                    # "Ada"

Os registros nunca mudam: crie um novo a partir de um antigo
  older = {..ada, born: 1816}    # os mesmos campos, `born` substituído

Desmontar
  {name, born} = ada
  fn greet({name}) = "Hello {name}"

Funções úteis
  keys(ada)   values(ada)   has(ada, "name")   get(ada, "x")   put(ada, "x", 1)
  entries(ada)                   # [["name", "Ada"], ["born", 1815]]
  from_entries([["x", 1]])       # {x: 1}

Registros como dicionários
  counts = {apple: 2}
  put(counts, "pear", 5)         # {apple: 2, pear: 5}
  map_values(counts, fn(n) = n * 10)      # {apple: 20}
  group_by(["a", "b", "a"], fn(x) = x) |> map_values(len)   # {a: 2, b: 1}
  Para chaves que não são nomes (números, listas...), use um dicionário: rythme help dicts

JSON, o formato de dados da web
  parse_json(text)               # Ok(...): os objetos viram registros, os arrays listas
  to_json(value)                 # o texto, pronto para salvar com write_file"#,
        ),
        (
            r#"DICTIONARIES AND SETS

A dict (dictionary) gives a value for each key. Keys can be any value but
functions: numbers, texts, lists, records... They are kept in order.
  ages = dict([["Ada", 36], ["Alan", 41]])
  ages                           # dict("Ada" => 36, "Alan" => 41)
  ages["Ada"]                    # 36 (an error if the key is missing)
  get(ages, "Bob")               # None: an Option, never an error
  grid = dict([[[0, 0], "start"]])   # a list as a key
  dict({x: 1})                   # from a record: dict("x" => 1)

Dicts never change: functions give a new one, quickly (in O(log n))
  put(ages, "Bob", 20)           # added (or replaced)
  remove(ages, "Ada")            # without this key
  has(ages, "Ada")   len(ages)   keys(ages)   values(ages)   entries(ages)
  map_values(ages, fn(n) = n + 1)
  filter(ages, fn(name, age) = age > 40)   # f receives the key and the value
  ages.each do |name, age| print(name, age) end   # reject and select too
  union(a, b)                    # both; for a key in both, b's value wins

A set holds values without repetition, in order.
  s = set([3, 1, 3])             # set(1, 3)
  has(s, 3)   push(s, 5)   remove(s, 1)   len(s)   values(s)   # values: a list
  union(a, b)   intersection(a, b)   difference(a, b)   is_subset(a, b)
  filter(s, is_odd)   map(s, fn(x) = x * 10)

Dicts and sets compare with == and <, and print as dict(k => v, ...) and
set(a, b, ...). `to_json` writes a dict whose keys are all texts (or all symbols) as an
object, a set as an array; `dict(parse_json(text)?)` reads an object back."#,
            r#"DICIONÁRIOS E CONJUNTOS

Um dict (dicionário) dá um valor para cada chave. As chaves podem ser qualquer
valor, menos funções: números, textos, listas, registros... Elas ficam em ordem.
  ages = dict([["Ada", 36], ["Alan", 41]])
  ages                           # dict("Ada" => 36, "Alan" => 41)
  ages["Ada"]                    # 36 (um erro se a chave faltar)
  get(ages, "Bob")               # None: uma Option, nunca um erro
  grid = dict([[[0, 0], "start"]])   # uma lista como chave
  dict({x: 1})                   # a partir de um registro: dict("x" => 1)

Os dicionários nunca mudam: as funções dão um novo, rapidamente (em O(log n))
  put(ages, "Bob", 20)           # adicionado (ou substituído)
  remove(ages, "Ada")            # sem esta chave
  has(ages, "Ada")   len(ages)   keys(ages)   values(ages)   entries(ages)
  map_values(ages, fn(n) = n + 1)
  filter(ages, fn(name, age) = age > 40)   # f recebe a chave e o valor
  ages.each do |name, age| print(name, age) end   # reject e select também
  union(a, b)                    # os dois; para uma chave nos dois, ganha o valor de b

Um conjunto guarda valores sem repetição, em ordem.
  s = set([3, 1, 3])             # set(1, 3)
  has(s, 3)   push(s, 5)   remove(s, 1)   len(s)   values(s)   # values: uma lista
  union(a, b)   intersection(a, b)   difference(a, b)   is_subset(a, b)
  filter(s, is_odd)   map(s, fn(x) = x * 10)

Dicionários e conjuntos se comparam com == e <, e aparecem como dict(k => v, ...)
e set(a, b, ...). `to_json` escreve um dicionário cujas chaves são todas textos
(ou todas símbolos) como um objeto, e um conjunto como um array;
`dict(parse_json(text)?)` lê um objeto de volta."#,
        ),
        (
            r#"YOUR OWN TYPES

A type lists the possible shapes of a value:
  type Shape = Circle(radius) | Rect(width, height) | Empty

  c = Circle(2.0)
  c.radius                       # 2.0

Handle each shape with match
  fn area(shape) = match shape
    Circle(r) -> pi * r * r
    Rect(w, h) -> w * h
    Empty -> 0
  end

Or with one definition per shape
  fn perimeter(Circle(r)) = 2 * pi * r
  fn perimeter(Rect(w, h)) = 2 * (w + h)
  fn perimeter(Empty) = 0

If you forget a shape, `rythme check` tells you which one.

A type can contain itself (trees...):
  type Tree = Leaf | Node(left, value, right)"#,
            r#"OS SEUS PRÓPRIOS TIPOS

Um tipo lista as formas possíveis de um valor:
  type Shape = Circle(radius) | Rect(width, height) | Empty

  c = Circle(2.0)
  c.radius                       # 2.0

Trate cada forma com match
  fn area(shape) = match shape
    Circle(r) -> pi * r * r
    Rect(w, h) -> w * h
    Empty -> 0
  end

Ou com uma definição por forma
  fn perimeter(Circle(r)) = 2 * pi * r
  fn perimeter(Rect(w, h)) = 2 * (w + h)
  fn perimeter(Empty) = 0

Se você esquecer uma forma, `rythme check` diz qual.

Um tipo pode conter a si mesmo (árvores...):
  type Tree = Leaf | Node(left, value, right)"#,
        ),
        (
            r#"MATCH AND PATTERNS

`match` compares a value with patterns, in order, and gives the result of the
first one that fits:

  fn describe(x) = match x
    0 -> "zero"
    1 | 2 | 3 -> "small"
    n if n < 0 -> "negative"
    _ -> "big"
  end

Patterns
  42   "text"   true             exactly this value
  1..9   -5..-1   0.0..0.5       a number from the first to the second, both included
  "Mr " + name                   a text that starts with "Mr "; name is the rest
  x                              anything, named x
  _                              anything, ignored
  Some(x)   Circle(r)            a constructor and what it contains
  []   [a, b]   [first, ..rest]  lists, by shape
  {name, age}                    a record with these fields
  p if condition                 a pattern and a condition (guard)
  a | b                          one pattern or the other

Patterns work everywhere a name is given:
  [x, y] = [1, 2]
  fn head_or_zero([]) = 0
  fn head_or_zero([x, ..]) = x"#,
            r#"MATCH E PADRÕES

`match` compara um valor com padrões, em ordem, e dá o resultado do primeiro
que serve:

  fn describe(x) = match x
    0 -> "zero"
    1 | 2 | 3 -> "small"
    n if n < 0 -> "negative"
    _ -> "big"
  end

Padrões
  42   "text"   true             exatamente este valor
  1..9   -5..-1   0.0..0.5       um número do primeiro ao segundo, os dois incluídos
  "Mr " + name                   um texto que começa com "Mr "; name é o resto
  x                              qualquer coisa, chamada x
  _                              qualquer coisa, ignorada
  Some(x)   Circle(r)            um construtor e o que ele contém
  []   [a, b]   [first, ..rest]  listas, pela forma
  {name, age}                    um registro com estes campos
  p if condition                 um padrão e uma condição (guarda)
  a | b                          um padrão ou o outro

Os padrões funcionam em todo lugar em que se dá um nome:
  [x, y] = [1, 2]
  fn head_or_zero([]) = 0
  fn head_or_zero([x, ..]) = x"#,
        ),
        (
            r#"ERRORS ARE VALUES (Option, Result, ?)

Rythme has no null and no exceptions. A value that may be missing is an
Option: Some(value) or None. An operation that may fail gives a Result:
Ok(value) or Err(message). You cannot forget to handle them.

  fn divide(a, b) = if b == 0 then Err("division by zero") else Ok(a / b)

  match divide(10, 2)
    Ok(x) -> print("result: {x}")
    Err(e) -> print("error: {e}")
  end

The ? operator: take the value, or stop and pass the error on
  fn average(a, b)
    x = parse_float(a)?          # if this is Err(...), average returns it
    y = parse_float(b)?
    Ok((x + y) / 2)
  end

Quick tools
  unwrap_or(get(list, 5), 0)     # the value, or a default
  unwrap(Some(3))                # 3 (stops the program on None or Err)
  is_some(x)   is_none(x)   is_ok(x)   is_err(x)
  map(Some(2), fn(x) = x * 10)   # Some(20)
  ok_or(get(person, "age"), "no age")   # Err("no age") if missing: ready for ?
  and_then(x, f)                 # one more step that may fail
  map_err(r, fn(e) = "config: {e}")     # explain an error better

To stop the whole program: fail("message")."#,
            r#"OS ERROS SÃO VALORES (Option, Result, ?)

O Rythme não tem null nem exceções. Um valor que pode faltar é uma Option:
Some(value) ou None. Uma operação que pode falhar dá um Result: Ok(value) ou
Err(message). Você não tem como esquecer de tratá-los.

  fn divide(a, b) = if b == 0 then Err("division by zero") else Ok(a / b)

  match divide(10, 2)
    Ok(x) -> print("result: {x}")
    Err(e) -> print("error: {e}")
  end

O operador ?: pegar o valor, ou parar e repassar o erro
  fn average(a, b)
    x = parse_float(a)?          # se isto for Err(...), average o devolve
    y = parse_float(b)?
    Ok((x + y) / 2)
  end

Ferramentas rápidas
  unwrap_or(get(list, 5), 0)     # o valor, ou um padrão
  unwrap(Some(3))                # 3 (para o programa com None ou Err)
  is_some(x)   is_none(x)   is_ok(x)   is_err(x)
  map(Some(2), fn(x) = x * 10)   # Some(20)
  ok_or(get(person, "age"), "no age")   # Err("no age") se faltar: pronto para ?
  and_then(x, f)                 # mais uma etapa que pode falhar
  map_err(r, fn(e) = "config: {e}")     # explicar melhor um erro

Para parar o programa inteiro: fail("message")."#,
        ),
        (
            r#"THINKING FUNCTIONALLY

In Rythme, values never change and there are no loops. It may seem strange,
but programs become easier to read, test and trust: a function always gives
the same result for the same arguments.

Instead of a loop that prints:       1..5 |> each(fn(i) = print(i))
Instead of a loop that transforms:   map(list, f)
Instead of a loop that selects:      filter(list, f)
Instead of a loop that accumulates:  fold(list, start, f)   or sum, product
Instead of running totals:           scan(list, start, f)
Instead of a loop that searches:     find, any, all, index_of, min_by, max_by
Instead of "break":                  take_while(list, f)
Instead of "while":                  a recursive function

  # Python:  total = 0
  #          for x in numbers: total += x * x
  total = numbers |> map(fn(x) = x * x) |> sum

  # A "while" loop as a recursive function:
  fn collatz(1, steps) = steps
  fn collatz(n, steps) =
    if n % 2 == 0 then collatz(n // 2, steps + 1) else collatz(3 * n + 1, steps + 1)
  collatz(27, 0)                 # 111

Instead of changing a value, make a new one:
  x2 = x + 1
  person2 = {..person, age: 37}
  list2 = push(list, 4)"#,
            r#"PENSAR DE FORMA FUNCIONAL

No Rythme, os valores nunca mudam e não há laços. Pode parecer estranho, mas os
programas ficam mais fáceis de ler, testar e confiar: uma função sempre dá o
mesmo resultado para os mesmos argumentos.

Em vez de um laço que mostra:         1..5 |> each(fn(i) = print(i))
Em vez de um laço que transforma:     map(list, f)
Em vez de um laço que seleciona:      filter(list, f)
Em vez de um laço que acumula:        fold(list, start, f)   ou sum, product
Em vez de totais parciais:            scan(list, start, f)
Em vez de um laço que procura:        find, any, all, index_of, min_by, max_by
Em vez de "break":                    take_while(list, f)
Em vez de "while":                    uma função recursiva

  # Python:  total = 0
  #          for x in numbers: total += x * x
  total = numbers |> map(fn(x) = x * x) |> sum

  # Um laço "while" como função recursiva:
  fn collatz(1, steps) = steps
  fn collatz(n, steps) =
    if n % 2 == 0 then collatz(n // 2, steps + 1) else collatz(3 * n + 1, steps + 1)
  collatz(27, 0)                 # 111

Em vez de mudar um valor, crie um novo:
  x2 = x + 1
  person2 = {..person, age: 37}
  list2 = push(list, 4)"#,
        ),
        (
            r#"TESTS

Write tests next to your code:
  fn double(x) = x * 2

  test "double of 2 is 4" = assert_eq(double(2), 4)

  test "several checks"
    assert_eq(double(0), 0)
    assert(double(-1) < 0, "should stay negative")
  end

Run them:
  rythme test my_file.ry            the tests of one file
  rythme test .                     of every .ry file of a folder (and below)
  rythme test . --filter double     only the tests whose name contains "double"
  rythme test . --watch             again each time a file changes (Ctrl+C stops)
  rythme test my_file.ry -- a b     with `args()` giving ["a", "b"]
  rythme test                       in a project: its files with tests

While testing, top-level expressions such as print(...) are not run: the
definitions are loaded, then each test runs. A test passes if it ends without
error, and its value is not false, Err(...) or None. The summary gives the
counts and the time; the exit code is 0 when every test passes.

Property tests: check that something holds for many values
  test "reversing twice gives the list back" =
    check_all(fn(l) = reverse(reverse(l)) == l, lists_of(ints()))

Generators: ints(min, max), floats(min, max), texts(max_length), bools(),
lists_of(generator, max_length), one_of(list), records_of({x: ints(), y: texts()}).
`sample(generator)` shows what one gives. When a value makes the property
false, it is simplified (shrinking) and shown with the seed that made it: the
same values come back on every run, and `--seed N` tries others
(`--seed random` picks one)."#,
            r#"TESTES

Escreva os testes ao lado do seu código:
  fn double(x) = x * 2

  test "double of 2 is 4" = assert_eq(double(2), 4)

  test "several checks"
    assert_eq(double(0), 0)
    assert(double(-1) < 0, "should stay negative")
  end

Para executá-los:
  rythme test my_file.ry            os testes de um arquivo
  rythme test .                     de todos os arquivos .ry de uma pasta (e abaixo)
  rythme test . --filter double     só os testes cujo nome contém "double"
  rythme test . --watch             de novo a cada vez que um arquivo muda (Ctrl+C para)
  rythme test my_file.ry -- a b     com `args()` dando ["a", "b"]
  rythme test                       em um projeto: os arquivos dele com testes

Durante os testes, as expressões do nível mais alto como print(...) não são
executadas: as definições são carregadas, depois cada teste roda. Um teste passa
se termina sem erro e o valor dele não é false, Err(...) nem None. O resumo dá as
contagens e o tempo; o código de saída é 0 quando todos os testes passam.

Testes de propriedades: verifique que algo vale para muitos valores
  test "reversing twice gives the list back" =
    check_all(fn(l) = reverse(reverse(l)) == l, lists_of(ints()))

Geradores: ints(min, max), floats(min, max), texts(max_length), bools(),
lists_of(generator, max_length), one_of(list), records_of({x: ints(), y: texts()}).
`sample(generator)` mostra o que um deles dá. Quando um valor torna a
propriedade falsa, ele é simplificado (shrinking) e mostrado com a semente que
o gerou: os mesmos valores voltam a cada execução, e `--seed N` testa outros
(`--seed random` escolhe uma)."#,
        ),
        (
            r#"EXERCISES

Rythme comes with 30 graded exercises for a first course, from the first
values to recursion, lists, records, types, Option, Result and tests. Each one
is a file with a statement, hints, the functions to write and their tests.

  rythme exercise list              the exercises, from ★☆☆ (easy) to ★★★
  rythme exercise start 7           copies exercise 7 into the current folder
  rythme exercise check 07-recursion.ry
                                    runs its tests, and says what to do next

In the file, replace each fail("to do") with your code; do not change the
tests. `rythme test file.ry` runs the tests too. The exercises are also in the
folder docs/exercises of the repository of Rythme, in English and French."#,
            r#"EXERCÍCIOS

O Rythme vem com 30 exercícios graduados para um primeiro curso, dos primeiros
valores até recursão, listas, registros, tipos, Option, Result e testes. Cada
um é um arquivo com um enunciado, dicas, as funções a escrever e os testes delas.

  rythme exercise list              os exercícios, de ★☆☆ (fácil) a ★★★
  rythme exercise start 7           copia o exercício 7 para a pasta atual
  rythme exercise check 07-recursion.ry
                                    roda os testes dele, e diz o que fazer em seguida

No arquivo, substitua cada fail("to do") pelo seu código; não mude os testes.
`rythme test arquivo.ry` também roda os testes. Os exercícios também estão na
pasta docs/exercises do repositório do Rythme, em inglês e em francês."#,
        ),
        (
            r#"SEVERAL FILES

  # geometry.ry
  ## Area of a rectangle.
  fn area(w, h) = w * h
  fn _check(x) = x >= 0          # private: starts with `_`

  # main.ry
  use geometry                    # the file geometry.ry, next to main.ry
  print(geometry.area(2, 3))      # 6

`use geometry` gives the definitions of geometry.ry through its name:
`geometry.area`, `geometry.Circle(1)`, and in a `match`, `geometry.Circle(r)`.
So you always see where a name comes from. A file in a folder:
`use shapes/circle`, then `circle.perimeter(1)`. Another name: `use
geometry as geo`, then `geo.area(2, 3)`. Names starting with `_` stay
private to their file.

The path is relative to the folder of the file that contains `use`. Each file
runs only once. Rythme says which file defines a name you forgot to use.

Older form: `use "geometry.ry"` (with quotes) gives its definitions directly,
as `area(2, 3)`; prefer `use geometry`."#,
            r#"VÁRIOS ARQUIVOS

  # geometry.ry
  ## Area of a rectangle.
  fn area(w, h) = w * h
  fn _check(x) = x >= 0          # privado: começa com `_`

  # main.ry
  use geometry                    # o arquivo geometry.ry, ao lado de main.ry
  print(geometry.area(2, 3))      # 6

`use geometry` dá as definições de geometry.ry pelo nome dele:
`geometry.area`, `geometry.Circle(1)`, e em um `match`, `geometry.Circle(r)`.
Assim você sempre vê de onde vem um nome. Um arquivo em uma pasta:
`use shapes/circle`, depois `circle.perimeter(1)`. Outro nome: `use
geometry as geo`, depois `geo.area(2, 3)`. Os nomes que começam com `_` ficam
privados no próprio arquivo.

O caminho é relativo à pasta do arquivo que contém `use`. Cada arquivo roda uma
só vez. O Rythme diz qual arquivo define um nome que você esqueceu de usar.

Forma antiga: `use "geometry.ry"` (com aspas) dá as definições dele diretamente,
como `area(2, 3)`; prefira `use geometry`."#,
        ),
        (
            r#"FILES AND THE SYSTEM

Every function that touches the system returns a Result: Ok(...), or
Err(message) with a message that says what went wrong.

Files and folders (relative paths start in the current folder)
  read_file("notes.txt")         # Ok("...") or Err("file not found: notes.txt")
  write_file("out.txt", "hi")    # Ok(()), replaces the content
  append_file("log.txt", "done\n")
  file_exists("notes.txt")       # true or false
  file_info("notes.txt")         # Ok({is_folder: false, size: 11}), in bytes
  list_folder(".")               # Ok(["notes.txt", "photos"]), in order
  create_folder("backup/2024")   # Ok(()), with the missing folders around it
  delete_file("old.txt")         # Ok(()); never a folder

Paths that work on Linux, macOS and Windows
  join_path(["data", "notes.txt"])       # "data/notes.txt"
  split_path("/home/ada/notes.txt")      # ["/", "home", "ada", "notes.txt"]
  split_path("C:\Users\ada")             # ["C:\", "Users", "ada"]

The command line: rythme tool.ry --verbose --name=Ada notes.txt
  args()                         # ["--verbose", "--name=Ada", "notes.txt"]
  options()                      # Ok({args: ["notes.txt"], verbose: true, name: "Ada"})
  opts = unwrap(options())
  name = unwrap_or(get(opts, "name"), "World")   # an option that may be missing
  env("HOME")                    # Some("/home/ada") or None

The standard input: rythme count.ry < names.txt
  input("Name? ")                # one line
  input_lines()                  # Ok(["Ada", "Alan"]): every line until the end

Exit codes, for the scripts that run your program
  0    everything went well (also at the end of the program)
  1    an error in the program: syntax, check, while running, failed test
  2    a wrong use of the command (unknown command or option)
  101  an internal error of Rythme (a bug, to report)
  exit(code)                     # stops now with this code, from 0 to 255

In the playground, in the browser, there are no files and no input: these
functions give Err("... is not available in the browser ...")."#,
            r#"ARQUIVOS E O SISTEMA

Toda função que mexe com o sistema devolve um Result: Ok(...), ou
Err(message) com uma mensagem que diz o que deu errado.

Arquivos e pastas (os caminhos relativos começam na pasta atual)
  read_file("notes.txt")         # Ok("...") ou Err("file not found: notes.txt")
  write_file("out.txt", "hi")    # Ok(()), substitui o conteúdo
  append_file("log.txt", "done\n")
  file_exists("notes.txt")       # true ou false
  file_info("notes.txt")         # Ok({is_folder: false, size: 11}), em bytes
  list_folder(".")               # Ok(["notes.txt", "photos"]), em ordem
  create_folder("backup/2024")   # Ok(()), com as pastas que faltam em volta
  delete_file("old.txt")         # Ok(()); nunca uma pasta

Caminhos que funcionam no Linux, no macOS e no Windows
  join_path(["data", "notes.txt"])       # "data/notes.txt"
  split_path("/home/ada/notes.txt")      # ["/", "home", "ada", "notes.txt"]
  split_path("C:\Users\ada")             # ["C:\", "Users", "ada"]

A linha de comando: rythme tool.ry --verbose --name=Ada notes.txt
  args()                         # ["--verbose", "--name=Ada", "notes.txt"]
  options()                      # Ok({args: ["notes.txt"], verbose: true, name: "Ada"})
  opts = unwrap(options())
  name = unwrap_or(get(opts, "name"), "World")   # uma opção que pode faltar
  env("HOME")                    # Some("/home/ada") ou None

A entrada padrão: rythme count.ry < names.txt
  input("Name? ")                # uma linha
  input_lines()                  # Ok(["Ada", "Alan"]): todas as linhas até o fim

Códigos de saída, para os scripts que executam o seu programa
  0    tudo correu bem (também no fim do programa)
  1    um erro no programa: sintaxe, verificação, durante a execução, teste que falhou
  2    um uso errado do comando (comando ou opção desconhecida)
  101  um erro interno do Rythme (um bug, para informar)
  exit(code)                     # para agora com este código, de 0 a 255

No playground, no navegador, não há arquivos nem entrada: estas funções
dão Err("... is not available in the browser ...")."#,
        ),
        (
            r#"PROJECTS AND PACKAGES

  rythme new hello        # a folder hello/ with a new project:
                          #   rythme.toml, src/main.ry, src/greeting.ry,
                          #   tests/greeting_test.ry, README.md, .gitignore
  cd hello
  rythme run              # runs the program: `entry` of rythme.toml
  rythme test             # runs the files with tests of src/ and tests/
  rythme check            # checks every file; also `rythme fmt`, `rythme doc`

rythme.toml describes the project:

  name = "hello"
  version = "0.1.0"
  entry = "src/main.ry"

  [dependencies]
  mathlib = { git = "https://github.com/someone/mathlib", tag = "v1.0.0" }
  tools = { path = "../tools" }

A dependency comes from git, at a `tag` or a `commit`, or from a folder.
`rythme install` downloads them (with git) and writes rythme.lock, the exact
commit of each one: keep it with the project, so that everyone gets the
same code. Then:

  use mathlib/vectors       # the file src/vectors.ry of mathlib
  use mathlib               # its program, the `entry` of its rythme.toml
  use hello/greeting        # a file of the project itself, from tests/

A file next to the one that contains `use` comes first. Packages from git
are kept in ~/.rythme/packages (or RYTHME_HOME/packages)."#,
            r#"PROJETOS E PACOTES

  rythme new hello        # uma pasta hello/ com um projeto novo:
                          #   rythme.toml, src/main.ry, src/saudacao.ry,
                          #   tests/saudacao_test.ry, README.md, .gitignore
  cd hello
  rythme run              # executa o programa: o `entry` do rythme.toml
  rythme test             # executa os arquivos com testes de src/ e tests/
  rythme check            # verifica todos os arquivos; também `rythme fmt`, `rythme doc`

O rythme.toml descreve o projeto:

  name = "hello"
  version = "0.1.0"
  entry = "src/main.ry"

  [dependencies]
  mathlib = { git = "https://github.com/someone/mathlib", tag = "v1.0.0" }
  tools = { path = "../tools" }

Uma dependência vem do git, em uma `tag` ou em um `commit`, ou de uma pasta.
`rythme install` baixa as dependências (com o git) e escreve o rythme.lock, o
commit exato de cada uma: mantenha-o com o projeto, para que todos recebam o
mesmo código. Depois:

  use mathlib/vectors       # o arquivo src/vectors.ry de mathlib
  use mathlib               # o programa dele, o `entry` do rythme.toml dele
  use hello/saudacao        # um arquivo do próprio projeto, a partir de tests/

Um arquivo ao lado daquele que contém `use` vem primeiro. Os pacotes do git
ficam guardados em ~/.rythme/packages (ou RYTHME_HOME/packages)."#,
        ),
        (
            r#"DATES AND TIMES

A date is a record: {year, month, day}, with hour, minute and second for a
moment of the day, and offset (minutes ahead of UTC) when it is known.
  today()                        # {year: 2026, month: 10, day: 18}
  now()                          # {year: 2026, ..., hour: 14, minute: 5, second: 9, offset: 120}
  birthday = {year: 1815, month: 12, day: 10}
  birthday.year                  # 1815

Text and back, in the ISO 8601 form
  parse_date("2024-02-29")       # Ok({year: 2024, month: 2, day: 29})
  parse_date("2024-02-30")       # Err("... February 2024 ...")
  parse_date("2024-02-29T13:05:00+01:00")    # with a time and an offset
  format_date({year: 2024, month: 2, day: 29, hour: 9})   # "2024-02-29T09:00:00"

Calendar arithmetic (leap years and month lengths included)
  add_days({year: 2024, month: 2, day: 28}, 1)     # {year: 2024, month: 2, day: 29}
  add_seconds(now(), 3600)       # in one hour
  days_between(today(), {year: 2027, month: 1, day: 1})   # days left
  seconds_between(a, b)          # negative when b is before a
  day_of_week(today())           # 1 = Monday ... 7 = Sunday
  days_in_month(2023, 2)         # 28
  is_leap_year(2000)             # true
  is_date({year: 2023, month: 2, day: 29})   # false

A date that does not exist stops the functions with E023, which says why.
The local offset comes from the system (UTC when it does not tell); time()
gives seconds since 1970 as a Float, to measure durations."#,
            r#"DATAS E HORAS

Uma data é um registro: {year, month, day}, com hour, minute e second para um
momento do dia, e offset (minutos à frente do UTC) quando ele é conhecido.
  today()                        # {year: 2026, month: 10, day: 18}
  now()                          # {year: 2026, ..., hour: 14, minute: 5, second: 9, offset: 120}
  birthday = {year: 1815, month: 12, day: 10}
  birthday.year                  # 1815

Para texto e de volta, no formato ISO 8601
  parse_date("2024-02-29")       # Ok({year: 2024, month: 2, day: 29})
  parse_date("2024-02-30")       # Err("... fevereiro de 2024 ...")
  parse_date("2024-02-29T13:05:00+01:00")    # com hora e deslocamento
  format_date({year: 2024, month: 2, day: 29, hour: 9})   # "2024-02-29T09:00:00"

Cálculos de calendário (anos bissextos e tamanhos dos meses incluídos)
  add_days({year: 2024, month: 2, day: 28}, 1)     # {year: 2024, month: 2, day: 29}
  add_seconds(now(), 3600)       # daqui a uma hora
  days_between(today(), {year: 2027, month: 1, day: 1})   # dias que faltam
  seconds_between(a, b)          # negativo quando b vem antes de a
  day_of_week(today())           # 1 = segunda-feira ... 7 = domingo
  days_in_month(2023, 2)         # 28
  is_leap_year(2000)             # true
  is_date({year: 2023, month: 2, day: 29})   # false

Uma data que não existe para as funções com E023, que diz por quê.
O deslocamento local vem do sistema (UTC quando ele não informa); time()
dá os segundos desde 1970 como Float, para medir durações."#,
        ),
        (
            r#"REGEX: SEARCHING WITH A PATTERN

A regex (regular expression) describes pieces of text: "[0-9]+" means "one
digit or more". Rythme reads the usual regexes, and a search never takes
more than (length of the text) × (size of the regex).

  matches("2024", "[0-9]+")                  # true: the WHOLE text matches
  find_first("tel: 06 12", "[0-9]+")         # Some("06"): the first piece
  find_all("3 cats, 12 dogs", "[0-9]+")      # ["3", "12"]
  split_by("a, b;c", "[,;] *")               # ["a", "b", "c"]
  replace_all("a cat", "\\b\\w", upper)      # "A Cat" (a function for each piece)
  replace_all("29/02/2024", "(\\d+)/(\\d+)/(\\d+)", "$3-$2-$1")   # "2024-02-29"
  captures("2024-02", "([0-9]+)-([0-9]+)")   # Some(["2024-02", "2024", "02"])
  named_captures("2024-02", "(?<year>[0-9]+)-(?<month>[0-9]+)")
                                             # Some({year: "2024", month: "02"})

What a regex can say
  abc          these characters, in this order
  .            any character but a line break
  [abc]        one of these characters; [a-z] one from a to z; [^0-9] any other
  \d \w \s     a digit (0-9); a letter (accents included), a digit or _; a space
  \D \W \S     anything else
  \b           the edge of a word
  ^  $         the start and the end of the text
  x|y          x or y
  (...)        a group: to repeat it, or to capture its text ($1, `captures`)
  (?:...)      a group that captures nothing; (?<name>...) a group with a name
  x*  x+  x?   x zero times or more, once or more, zero or one time (the longest)
  x{3}  x{2,5}  x{2,}    3 times, 2 to 5 times, 2 times or more
  x*?  x+?     the shortest instead of the longest
  (?i)         at the start: uppercase and lowercase letters count as equal

Writing a regex in a Rythme text
  A backslash is written twice: "\\d+" is the regex \d+. Braces are written
  \{ and \}: "[0-9]\{4\}" is the regex [0-9]{4}. For a sign itself, put it in
  brackets: "[.]" is a dot, "[(]" a parenthesis, "[$]" a dollar.

A mistake in a regex stops the program with E026, which shows where it is;
when the regex is written in the code, `rythme check` finds it before."#,
            r#"REGEX: BUSCAR COM UM PADRÃO

Uma regex (expressão regular) descreve pedaços de texto: "[0-9]+" quer dizer
"um algarismo ou mais". O Rythme lê as regex comuns, e uma busca nunca leva
mais que (tamanho do texto) × (tamanho da regex).

  matches("2024", "[0-9]+")                  # true: o texto INTEIRO corresponde
  find_first("tel: 06 12", "[0-9]+")         # Some("06"): o primeiro pedaço
  find_all("3 cats, 12 dogs", "[0-9]+")      # ["3", "12"]
  split_by("a, b;c", "[,;] *")               # ["a", "b", "c"]
  replace_all("a cat", "\\b\\w", upper)      # "A Cat" (uma função para cada pedaço)
  replace_all("29/02/2024", "(\\d+)/(\\d+)/(\\d+)", "$3-$2-$1")   # "2024-02-29"
  captures("2024-02", "([0-9]+)-([0-9]+)")   # Some(["2024-02", "2024", "02"])
  named_captures("2024-02", "(?<year>[0-9]+)-(?<month>[0-9]+)")
                                             # Some({year: "2024", month: "02"})

O que uma regex pode dizer
  abc          estes caracteres, nesta ordem
  .            qualquer caractere, menos uma quebra de linha
  [abc]        um destes caracteres; [a-z] um de a a z; [^0-9] qualquer outro
  \d \w \s     um algarismo (0-9); uma letra (acentos incluídos), um algarismo ou _; um espaço
  \D \W \S     qualquer outra coisa
  \b           a borda de uma palavra
  ^  $         o início e o fim do texto
  x|y          x ou y
  (...)        um grupo: para repeti-lo, ou para capturar o texto dele ($1, `captures`)
  (?:...)      um grupo que não captura nada; (?<name>...) um grupo com nome
  x*  x+  x?   x zero vezes ou mais, uma vez ou mais, zero ou uma vez (o mais longo)
  x{3}  x{2,5}  x{2,}    3 vezes, de 2 a 5 vezes, 2 vezes ou mais
  x*?  x+?     o mais curto em vez do mais longo
  (?i)         no início: letras maiúsculas e minúsculas contam como iguais

Escrever uma regex em um texto do Rythme
  A barra invertida se escreve duas vezes: "\\d+" é a regex \d+. As chaves se
  escrevem \{ e \}: "[0-9]\{4\}" é a regex [0-9]{4}. Para o próprio sinal,
  coloque-o entre colchetes: "[.]" é um ponto, "[(]" um parêntese, "[$]" um cifrão.

Um erro em uma regex para o programa com E026, que mostra onde ele está;
quando a regex está escrita no código, `rythme check` o encontra antes."#,
        ),
        (
            r#"THE INTERACTIVE MODE

Run `rythme` without a file. Type an expression and see its value:
  ry> 1 + 2
  3
Definitions are kept from one line to the next. If a line is not finished
(open bracket, block without `end`...), it continues on the next line.

Commands
  :help           this help              :quit          leave (or Ctrl-D)
  :doc map        help on a name         :load f.ry     load a file
  :reset          forget everything      :lang fr       change the language (en, fr, es, de, pt, zh)
  :learn          guided tour            :explain E003  explain an error
  :steps fib(4)   show the calculation step by step
The Tab key completes names."#,
            r#"O MODO INTERATIVO

Execute `rythme` sem arquivo. Digite uma expressão e veja o valor dela:
  ry> 1 + 2
  3
As definições são mantidas de uma linha para a outra. Se uma linha não estiver
terminada (parêntese aberto, bloco sem `end`...), ela continua na linha seguinte.

Comandos
  :help           esta ajuda             :quit          sair (ou Ctrl-D)
  :doc map        ajuda sobre um nome    :load f.ry     carregar um arquivo
  :reset          esquecer tudo          :lang pt       mudar o idioma (en, fr, es, de, pt, zh)
  :learn          visita guiada          :explain E003  explicar um erro
  :steps fib(4)   mostrar o cálculo passo a passo
A tecla Tab completa os nomes."#,
        ),
        (
            r#"FORMATTING: rythme fmt

`rythme fmt` gives code the official layout of Rythme: every program looks
the same, and nobody has to argue about spaces.

  rythme fmt hello.ry            format a file (it is rewritten)
  rythme fmt .                   every .ry file of a folder and its subfolders
  rythme fmt --check .           only check: lists the files to format and
                                 fails (exit code 1) if there are any, for CI
  rythme fmt - < hello.ry        reads the code, writes the result

The style
  - 2 spaces per level of indentation
  - spaces around operators: x = a + b * 2   xs |> map(f)   n -> n * 2
  - none inside brackets or around ..: f(x)   [1, 2]   {name: "Ada"}   1..10
  - a pipeline written on several lines: one step per line
      total = prices
        |> filter(fn(p) = p > 10)
        |> sum
  - a type written on several lines: one variant per line
      type Shape =
        | Circle(radius)
        | Rect(width, height)
  - a list written on several lines ends with a comma
  - a block written on one line stays on one line:
      xs.map do |x| x * 2 end      xs.map { |x| x * 2 }
    on several lines, its statements go one level deeper than `do`
  - one statement per line (no `;`), and `end` on its own line
  - a blank line around a definition written on several lines; never two
    blank lines in a row
  - every comment is kept; the comments at the end of consecutive lines
    are aligned

Line breaks are kept where the language allows a choice, and long lines are
kept as written. The formatter never changes what a program does: a file with
a syntax error is left as it is, and the error is shown."#,
            r#"FORMATAÇÃO: rythme fmt

`rythme fmt` dá ao código o layout oficial do Rythme: todo programa fica com a
mesma cara, e ninguém precisa discutir sobre espaços.

  rythme fmt hello.ry            formatar um arquivo (ele é reescrito)
  rythme fmt .                   todos os arquivos .ry de uma pasta e das subpastas
  rythme fmt --check .           só verificar: lista os arquivos a formatar e
                                 falha (código de saída 1) se houver algum, para CI
  rythme fmt - < hello.ry        lê o código, escreve o resultado

O estilo
  - 2 espaços por nível de indentação
  - espaços em volta dos operadores: x = a + b * 2   xs |> map(f)   n -> n * 2
  - nenhum dentro de parênteses ou em volta de ..: f(x)   [1, 2]   {name: "Ada"}   1..10
  - um pipeline escrito em várias linhas: uma etapa por linha
      total = prices
        |> filter(fn(p) = p > 10)
        |> sum
  - um tipo escrito em várias linhas: uma variante por linha
      type Shape =
        | Circle(radius)
        | Rect(width, height)
  - uma lista escrita em várias linhas termina com uma vírgula
  - um bloco escrito em uma linha fica em uma linha:
      xs.map do |x| x * 2 end      xs.map { |x| x * 2 }
    em várias linhas, as instruções dele vão um nível mais fundo que `do`
  - uma instrução por linha (sem `;`), e `end` em uma linha própria
  - uma linha em branco em volta de uma definição escrita em várias linhas;
    nunca duas linhas em branco seguidas
  - todo comentário é mantido; os comentários no fim de linhas consecutivas
    são alinhados

As quebras de linha são mantidas onde a linguagem permite escolher, e as linhas
longas ficam como foram escritas. O formatador nunca muda o que um programa faz:
um arquivo com um erro de sintaxe é deixado como está, e o erro é mostrado."#,
        ),
        (
            r#"DOCUMENTATION: rythme doc

`rythme doc` writes the documentation of your code from its `##` comments,
its types and the definitions of its functions.

  rythme doc geometry.ry         Markdown on the screen
  rythme doc .                   every .ry file of a folder and its subfolders
  rythme doc --html geometry.ry  one web page, which needs nothing else
  rythme doc . --out doc.html    written to a file (.html gives the web page)

Write a `##` comment just above what it documents:

  ## Shapes and their areas.      <- at the top, followed by a blank line:
                                     it describes the whole file
  ## A shape to draw.
  type Shape = Circle(radius) | Rect(width, height)

  ## The area of a shape. See `len` and `area`.
  fn area(Circle(r)) = pi * r * r
  fn area(Rect(w, h)) = w * h

What is written
  - for each file: its description, the files it uses (documented too),
    its types, its functions (the first line of every definition), its
    values and the names of its tests
  - names starting with `_` are left out: they stay inside their file
  - a name between backticks in a comment, like `area` or `len(list)`,
    becomes a link to its documentation; after `use geometry`,
    `geometry.area` leads to `area` in geometry.ry; the built-in functions
    named this way are explained at the end
  - the titles are in the language of the messages (`--lang fr`)

In a comment, a blank `##` line starts a new paragraph, and lines starting
with `- ` make a list."#,
            r#"DOCUMENTAÇÃO: rythme doc

`rythme doc` escreve a documentação do seu código a partir dos comentários `##`,
dos tipos e das definições das funções.

  rythme doc geometry.ry         Markdown na tela
  rythme doc .                   todos os arquivos .ry de uma pasta e das subpastas
  rythme doc --html geometry.ry  uma página web, que não precisa de mais nada
  rythme doc . --out doc.html    escrito em um arquivo (.html dá a página web)

Escreva um comentário `##` logo acima do que ele documenta:

  ## Shapes and their areas.      <- no topo, seguido de uma linha em branco:
                                     descreve o arquivo inteiro
  ## A shape to draw.
  type Shape = Circle(radius) | Rect(width, height)

  ## The area of a shape. See `len` and `area`.
  fn area(Circle(r)) = pi * r * r
  fn area(Rect(w, h)) = w * h

O que é escrito
  - para cada arquivo: a descrição, os arquivos que ele usa (documentados
    também), os tipos, as funções (a primeira linha de cada definição), os
    valores e os nomes dos testes
  - os nomes que começam com `_` ficam de fora: eles ficam dentro do arquivo
  - um nome entre crases em um comentário, como `area` ou `len(list)`, vira um
    link para a documentação dele; depois de `use geometry`, `geometry.area`
    leva a `area` em geometry.ry; as funções embutidas citadas assim são
    explicadas no fim
  - os títulos ficam no idioma das mensagens (`--lang pt`)

Em um comentário, uma linha `##` vazia começa um novo parágrafo, e as linhas que
começam com `- ` formam uma lista."#,
        ),
        (
            r#"STEP BY STEP: rythme steps

`rythme steps` shows how a program computes its values, one step at a time:
each call is replaced by the body of its function, with the values of the
arguments in place, until only the result remains.

  rythme steps fib.ry              each definition and expression of the file
  rythme steps fib.ry "fib(4)"     an expression, after the definitions of the file
  rythme steps fib.ry --max 50     show at most 50 steps of each expression
  rythme steps fib.ry --width 80   cut the lines longer than 80 characters

With  fn fib(n) = if n < 2 then n else fib(n - 1) + fib(n - 2)

  fib(3)
    → fib(2) + fib(1)
    → fib(1) + fib(0) + fib(1)
    → 1 + fib(0) + fib(1)
    → 1 + 0 + fib(1)
    → 1 + fib(1)
    → 1 + 1
    → 2

A step is one call or one operation. The body of a called function is shown
already simplified as far as it can be without calling another function.
Built-in functions (and the functions of other files) give their result in
one step. What the program prints appears between the steps. After the
steps shown (200 by default), the rest is computed without being shown.

In the interactive mode: :steps fib(4). In the playground: the Steps button.
In French: rythme etapes."#,
            r#"PASSO A PASSO: rythme steps

`rythme steps` mostra como um programa calcula os seus valores, uma etapa de
cada vez: cada chamada é substituída pelo corpo da sua função, com os valores
dos argumentos no lugar, até sobrar só o resultado.

  rythme steps fib.ry              cada definição e expressão do arquivo
  rythme steps fib.ry "fib(4)"     uma expressão, depois das definições do arquivo
  rythme steps fib.ry --max 50     mostrar no máximo 50 etapas de cada expressão
  rythme steps fib.ry --width 80   cortar as linhas com mais de 80 caracteres

Com  fn fib(n) = if n < 2 then n else fib(n - 1) + fib(n - 2)

  fib(3)
    → fib(2) + fib(1)
    → fib(1) + fib(0) + fib(1)
    → 1 + fib(0) + fib(1)
    → 1 + 0 + fib(1)
    → 1 + fib(1)
    → 1 + 1
    → 2

Uma etapa é uma chamada ou uma operação. O corpo de uma função chamada é
mostrado já simplificado o máximo possível sem chamar outra função. As funções
embutidas (e as funções de outros arquivos) dão o resultado em uma etapa. O que
o programa mostra aparece entre as etapas. Depois das etapas mostradas (200 por
padrão), o resto é calculado sem ser mostrado.

No modo interativo: :steps fib(4). No playground: o botão Steps.
Em francês: rythme etapes."#,
        ),
        (
            r#"RYTHME IN YOUR EDITOR

Rythme helps inside the editor too: problems as you type, the documentation
of a function under the mouse (with the signature inferred from its code,
like `fn area(side: Int|Float) -> Int|Float`), completion, the parameters of
a call, go to definition, references, rename, quick fixes for typos,
formatting like `rythme fmt`, the types of the names when they are certain
(and the signatures of the functions), colors that follow what each name is,
"Run" and "Run test" above the code, the names of the whole workspace,
folding. This is the language server, that the editor
starts by itself:
  rythme lsp

VS Code
  the extension in editors/vscode: colors, snippets and the language server.
  Settings: rythme.path (the program), rythme.language (en, fr, es, de, pt, zh or auto).

Neovim, Helix, Zed, Emacs, Sublime Text, Kate...
  every editor that speaks the Language Server Protocol: tell it to start
  `rythme lsp` for .ry files. Ready-made configurations: editors/README.md
  ("Run" and "Run test" name the commands rythme.runFile, rythme.testFile and
  rythme.runTest, that the editor carries out: see editors/README.md)

Zed
  the extension in editors/zed: colors and the language server.

Colors in Neovim, Helix and Zed
  the Tree-sitter grammar in editors/tree-sitter-rythme (see editors/README.md).

The language of the messages follows the editor, unless `--lang` (en, fr, es, de, pt, zh) or
RYTHME_LANG chooses it: rythme --lang fr lsp"#,
            r#"O RYTHME NO SEU EDITOR

O Rythme ajuda também dentro do editor: problemas enquanto você digita, a
documentação de uma função sob o mouse (com a assinatura deduzida do código,
como `fn area(side: Int|Float) -> Int|Float`), autocompletar, os parâmetros de
uma chamada, ir para a definição, referências, renomear, correções rápidas de
erros de digitação, formatação como `rythme fmt`, os tipos dos nomes quando são
certos (e as assinaturas das funções), cores que seguem o que cada nome é,
"Run" e "Run test" acima do código, os nomes de todo o espaço de trabalho,
dobra de código. Isso é o servidor de linguagem, que o editor inicia sozinho:
  rythme lsp

VS Code
  a extensão em editors/vscode: cores, snippets e o servidor de linguagem.
  Configurações: rythme.path (o programa), rythme.language (en, fr, es, de, pt, zh ou auto).

Neovim, Helix, Zed, Emacs, Sublime Text, Kate...
  todo editor que fala o Language Server Protocol: diga a ele para iniciar
  `rythme lsp` para os arquivos .ry. Configurações prontas: editors/README.md
  ("Run" e "Run test" nomeiam os comandos rythme.runFile, rythme.testFile e
  rythme.runTest, que o editor executa: veja editors/README.md)

Zed
  a extensão em editors/zed: cores e o servidor de linguagem.

Cores no Neovim, no Helix e no Zed
  a gramática Tree-sitter em editors/tree-sitter-rythme (veja editors/README.md).

O idioma das mensagens segue o do editor, a não ser que `--lang` (en, fr, es, de, pt, zh)
ou RYTHME_LANG o escolha: rythme --lang pt lsp"#,
        ),
        (
            r#"COMING FROM RUBY

Rythme reads like Ruby, but its values never change: no mutation, no nil,
no exceptions. What you know from Ruby mostly works:

  names = ["Ada", "Grace", "Alan"]
  names.each do |name|
    puts "Hello, #{name}!"
  end

  [1, 2, 3, 4, 5, 6]
    .select(even?)
    .map { |x| x * x }
    .sum                         # 56

  fn adult?(person) = person.age >= 18
  kind = unless adult?(ada) then :minor else :adult

The same as in Ruby
  calls           puts "hi"   print x, y   assert_eq total, 42
                  (at the start of a statement or a result; see below)
  blocks          xs.map do |x| x * 2 end      xs.map { |x| x * 2 }
                  a block is a function: the same as fn(x) = x * 2
  method calls    xs.sum   name.upcase   xs.first(2)   xs.include?(3)
  names with ?    even?   empty?   include?(x)   fn adult?(p) = ...
  symbols         :ok   :error (compared with ==, used in match)
  texts           "Hello #{name}" (or "Hello {name}"); "\#{" writes #{
  unless          unless cond then a else b
  Ruby names      puts to_s to_i to_f length size upcase downcase strip
                  select reject reduce inject uniq each_slice each_cons
                  each_with_index include? empty? even? odd? any? all?
                  start_with? end_with? key? lstrip rstrip casecmp?
                  (see rythme help builtins)

What is different
  parentheses     a call inside an expression needs them: x = len(xs),
                  not x = len xs
  no mutation     no x += 1: give the new value a new name, total2 = total + 1
  no nil          a missing value is None, a present one Some(x)
  no exceptions   errors are values, Ok(x) or Err(message), passed on with ?
  if and unless   always have an else: they give a value; no `return`, the
                  last line of a function is its result
  no classes      records {name: "Ada"} and types: type Shape = Circle(r) | Square(s)
  no loops        each, map, select, reduce, or a function that calls itself
  case/when       match value ... end, one case per line: pattern -> result
  hashes          dict([[:ada, 36]]) (keys of any kind, symbols too), or
                  records {name: "Ada", age: 36}

Calls without parentheses
  A statement that starts with a name, followed on its line by the start of
  a value (a number, a text, a symbol, a name, a constructor, not, if,
  match, fn), is a call: print x, y is print(x, y). A do block after the
  arguments is given to the call: each xs do |x| ... end. Like Ruby, a [ or
  a - after a space, without a space after the -, starts the arguments too
  (puts [1, 2], puts -1), except when the code gives the name a value (with
  =, as a parameter or in a pattern): then x -1 is x - 1 and xs [0] is
  xs[0], as before. `rythme fmt` writes those as x - 1 and xs[0].

  A result may start the same way: a branch of if or unless, a case of
  match, the body after the = of a function or a test. Its arguments end
  at else, end, }, ) or the end of the line:

  fn greet(name) = puts "Hello, #{name}!"
  fn check(ok) = if ok then puts "yes" else puts "no"
  match name
    "Dr " + rest -> puts "doctor", rest
    _ -> puts name
  end
  xs.each { |x| puts x }

How x.name works
  x.name reads the field `name` of a record; when x has no such field, it
  calls the function `name` with x: xs.sum is sum(xs), xs.first(2) is
  first(xs, 2). A chain may go on over several lines, each starting with `.`.

When ? is part of a name
  even?(4), xs.include?(3), fn adult?(p): right after a name, before `(`.
  even? without `(`: when such a function exists (built-in, or defined with
  fn in the file). Otherwise value? is the ? that stops on None or Err, and
  (value)? always is."#,
            r#"VINDO DO RUBY

O Rythme se lê como Ruby, mas os seus valores nunca mudam: sem mutação, sem nil,
sem exceções. O que você sabe de Ruby funciona quase sempre:

  names = ["Ada", "Grace", "Alan"]
  names.each do |name|
    puts "Hello, #{name}!"
  end

  [1, 2, 3, 4, 5, 6]
    .select(even?)
    .map { |x| x * x }
    .sum                         # 56

  fn adult?(person) = person.age >= 18
  kind = unless adult?(ada) then :minor else :adult

Igual ao Ruby
  chamadas        puts "hi"   print x, y   assert_eq total, 42
                  (no início de uma instrução ou de um resultado; veja abaixo)
  blocos          xs.map do |x| x * 2 end      xs.map { |x| x * 2 }
                  um bloco é uma função: o mesmo que fn(x) = x * 2
  métodos         xs.sum   name.upcase   xs.first(2)   xs.include?(3)
  nomes com ?     even?   empty?   include?(x)   fn adult?(p) = ...
  símbolos        :ok   :error (comparados com ==, usados em match)
  textos          "Hello #{name}" (ou "Hello {name}"); "\#{" escreve #{
  unless          unless cond then a else b
  nomes Ruby      puts to_s to_i to_f length size upcase downcase strip
                  select reject reduce inject uniq each_slice each_cons
                  each_with_index include? empty? even? odd? any? all?
                  start_with? end_with? key? lstrip rstrip casecmp?
                  (veja rythme help builtins)

O que é diferente
  parênteses      uma chamada dentro de uma expressão precisa deles: x = len(xs),
                  não x = len xs
  sem mutação     sem x += 1: dê um novo nome ao novo valor, total2 = total + 1
  sem nil         um valor ausente é None, um presente é Some(x)
  sem exceções    os erros são valores, Ok(x) ou Err(message), repassados com ?
  if e unless     sempre têm um else: eles dão um valor; sem `return`, a
                  última linha de uma função é o seu resultado
  sem classes     registros {name: "Ada"} e tipos: type Shape = Circle(r) | Square(s)
  sem laços       each, map, select, reduce, ou uma função que chama a si mesma
  case/when       match value ... end, um caso por linha: padrão -> resultado
  hashes          dict([[:ada, 36]]) (chaves de qualquer tipo, símbolos também), ou
                  registros {name: "Ada", age: 36}

Chamadas sem parênteses
  Uma instrução que começa com um nome, seguido na mesma linha do início de um
  valor (um número, um texto, um símbolo, um nome, um construtor, not, if,
  match, fn), é uma chamada: print x, y é print(x, y). Um bloco do depois dos
  argumentos é dado à chamada: each xs do |x| ... end. Como no Ruby, um [ ou
  um - depois de um espaço, sem espaço depois do -, também começa os argumentos
  (puts [1, 2], puts -1), a não ser quando o código dá um valor ao nome (com
  =, como parâmetro ou em um padrão): aí x -1 é x - 1 e xs [0] é xs[0], como
  antes. `rythme fmt` os escreve como x - 1 e xs[0].

  Um resultado pode começar da mesma forma: um ramo de if ou unless, um caso de
  match, o corpo depois do = de uma função ou de um teste. Os argumentos dele
  terminam em else, end, }, ) ou no fim da linha:

  fn greet(name) = puts "Hello, #{name}!"
  fn check(ok) = if ok then puts "yes" else puts "no"
  match name
    "Dr " + rest -> puts "doctor", rest
    _ -> puts name
  end
  xs.each { |x| puts x }

Como x.name funciona
  x.name lê o campo `name` de um registro; quando x não tem esse campo, ele
  chama a função `name` com x: xs.sum é sum(xs), xs.first(2) é
  first(xs, 2). Uma cadeia pode continuar por várias linhas, cada uma começando com `.`.

Quando ? faz parte de um nome
  even?(4), xs.include?(3), fn adult?(p): logo depois de um nome, antes de `(`.
  even? sem `(`: quando existe uma função assim (embutida, ou definida com
  fn no arquivo). Senão, value? é o ? que para em None ou Err, e
  (value)? sempre é."#,
        ),
        (
            r#"E001 — Syntax error

The code does not follow the grammar of Rythme at this place. The message
says what was expected. Frequent causes:
  - a missing `)`, `]` or `,`
  - `then` or `else` missing in an `if`
  - braces { } used for a block (Rythme uses a new line and `end`)
  - a keyword from another language (for, while, return, let, def...)

Wrong:   if x > 0 { print(x) }
Right:   if x > 0 then print(x) else print("not positive")"#,
            r#"E001 — Erro de sintaxe

O código não segue a gramática do Rythme neste lugar. A mensagem diz o que era
esperado. Causas frequentes:
  - falta um `)`, um `]` ou uma `,`
  - falta `then` ou `else` em um `if`
  - chaves { } usadas para um bloco (o Rythme usa uma nova linha e `end`)
  - uma palavra-chave de outra linguagem (for, while, return, let, def...)

Errado:  if x > 0 { print(x) }
Certo:   if x > 0 then print(x) else print("not positive")"#,
        ),
        (
            r#"E002 — Text never closed

A text starts with " and must end with " on the same text. A `{` inside a
text starts an insertion and must be closed by `}`.

Wrong:   print("hello)
Right:   print("hello")
To write a brace in a text: "\{not inserted\}""#,
            r#"E002 — Texto nunca fechado

Um texto começa com " e deve terminar com " no mesmo texto. Uma `{` dentro de
um texto começa uma inserção e deve ser fechada por `}`.

Errado:  print("hello)
Certo:   print("hello")
Para escrever uma chave em um texto: "\{não inserido\}""#,
        ),
        (
            r#"E003 — Unknown name

This name is not defined at this place. Check the spelling (uppercase and
lowercase letters are different), and that the name is defined before it is
used. Inside a function, the parameters and the names defined above are
visible, and local functions written one right after the other see each
other; at the top level, all functions are visible everywhere.

Wrong:   print(lenght([1, 2]))
Right:   print(len([1, 2]))

Rythme suggests the closest existing name when there is one, and the file
that defines it when it is another file of the folder:

  use geometry                   # then
  print(geometry.area(2))        # not area(2)

A name starting with `_` is private to its file: other files cannot use it."#,
            r#"E003 — Nome desconhecido

Este nome não está definido neste lugar. Verifique a grafia (letras maiúsculas
e minúsculas são diferentes), e se o nome está definido antes de ser usado.
Dentro de uma função, os parâmetros e os nomes definidos acima são visíveis, e
as funções locais escritas uma logo depois da outra se enxergam; no nível mais
alto, todas as funções são visíveis em todo lugar.

Errado:  print(lenght([1, 2]))
Certo:   print(len([1, 2]))

O Rythme sugere o nome existente mais parecido, quando há um, e o arquivo que o
define, quando é outro arquivo da pasta:

  use geometry                   # depois
  print(geometry.area(2))        # e não area(2)

Um nome que começa com `_` é privado do seu arquivo: outros arquivos não podem usá-lo."#,
        ),
        (
            r#"E004 — Wrong number of arguments

The function is called with more or fewer values than it takes. Remember that
with `|>`, the value on the left is the first argument.

Wrong:   fn add(a, b) = a + b
         add(1)
Right:   add(1, 2)
Also right:  1 |> add(2)

`rythme help NAME` shows how to call a built-in function."#,
            r#"E004 — Número errado de argumentos

A função é chamada com mais ou menos valores do que ela recebe. Lembre-se de que
com `|>` o valor da esquerda é o primeiro argumento.

Errado:  fn add(a, b) = a + b
         add(1)
Certo:   add(1, 2)
Também certo:  1 |> add(2)

`rythme help NOME` mostra como chamar uma função embutida."#,
        ),
        (
            r#"E005 — Wrong kind of value

The operation does not work with this kind of value. Rythme never converts
values silently: "1" + 1 is an error, not "11" or 2.

Wrong:   "age: " + 36
Right:   "age: {36}"              (insert the value in the text)
Right:   "age: " + text(36)       (convert it first)

Conditions must be true or false:
Wrong:   if len(list) then ... else ...
Right:   if len(list) > 0 then ... else ...

When the kinds of the values are certain, `rythme check` finds this mistake
before the program runs, even in code that does not run every time, and in
the calls that give a function what it cannot use: `area("big")` when
`area(side)` computes `side * side`."#,
            r#"E005 — Tipo de valor errado

A operação não funciona com este tipo de valor. O Rythme nunca converte valores
sem avisar: "1" + 1 é um erro, não "11" nem 2.

Errado:  "age: " + 36
Certo:   "age: {36}"              (inserir o valor no texto)
Certo:   "age: " + text(36)       (convertê-lo primeiro)

As condições devem ser true ou false:
Errado:  if len(list) then ... else ...
Certo:   if len(list) > 0 then ... else ...

Quando os tipos dos valores são certos, `rythme check` encontra este erro antes de
o programa rodar, mesmo em código que não roda toda vez, e nas chamadas que dão
a uma função o que ela não consegue usar: `area("big")` quando `area(side)`
calcula `side * side`."#,
        ),
        (
            r#"E006 — No case matches

None of the cases of a `match` (or of the definitions of a function, or the
pattern of a definition like `[a, b] = list`) accepts this value.

Wrong:   fn name(1) = "one"
         name(2)
Right:   fn name(1) = "one"
         fn name(_) = "another number"

`rythme check` warns about forgotten cases before running (W002). When the
kind of the value is certain, it also reports, before running, a case that
can never fit it: `[first, ..rest]` for a Text, `Circle(r)` for a number."#,
            r#"E006 — Nenhum caso corresponde

Nenhum dos casos de um `match` (ou das definições de uma função, ou o padrão de
uma definição como `[a, b] = list`) aceita este valor.

Errado:  fn name(1) = "one"
         name(2)
Certo:   fn name(1) = "one"
         fn name(_) = "another number"

`rythme check` avisa sobre casos esquecidos antes de executar (W002). Quando o
tipo do valor é certo, ele também aponta, antes de executar, um caso que nunca
pode servir: `[first, ..rest]` para um Text, `Circle(r)` para um número."#,
        ),
        (
            r#"E007 — Index out of range

Positions in a list or a text go from 0 to length - 1 (and -1 is the last).

Wrong:   [10, 20][2]
Right:   [10, 20][1]              # 20
Safe:    get([10, 20], 2)         # None, never an error"#,
            r#"E007 — Índice fora do intervalo

As posições em uma lista ou em um texto vão de 0 a comprimento - 1 (e -1 é a última).

Errado:  [10, 20][2]
Certo:   [10, 20][1]              # 20
Seguro:  get([10, 20], 2)         # None, nunca um erro"#,
        ),
        (
            r#"E008 — Division by zero

Dividing by zero has no meaning. Check the divisor before, or return a Result.

  fn divide(a, b) = if b == 0 then Err("division by zero") else Ok(a / b)"#,
            r#"E008 — Divisão por zero

Dividir por zero não tem sentido. Verifique o divisor antes, ou devolva um Result.

  fn divide(a, b) = if b == 0 then Err("division by zero") else Ok(a / b)"#,
        ),
        (
            r#"E009 — Not a function

Parentheses after a value call it, but only functions can be called.

Wrong:   x = 5
         x(3)
Wrong:   None()                   (None is a value)
Right:   None
To multiply, write it:  2 * (x + 1)"#,
            r#"E009 — Não é uma função

Parênteses depois de um valor o chamam, mas só funções podem ser chamadas.

Errado:  x = 5
         x(3)
Errado:  None()                   (None é um valor)
Certo:   None
Para multiplicar, escreva:  2 * (x + 1)"#,
        ),
        (
            r#"E010 — Unknown field

The record (or the constructor) has no field with this name; or the dict
has no such key (`dict[key]`).

Wrong:   {name: "Ada"}.nmae
Right:   {name: "Ada"}.name
Safe:    get(record, "name")     # an Option, never an error
         get(dict, key)          # the same for a dict

When the fields of the record are certain, `rythme check` finds this mistake
before the program runs, and suggests the closest field."#,
            r#"E010 — Campo desconhecido

O registro (ou o construtor) não tem nenhum campo com este nome; ou o dicionário
não tem essa chave (`dict[key]`).

Errado:  {name: "Ada"}.nmae
Certo:   {name: "Ada"}.name
Seguro:  get(record, "name")     # uma Option, nunca um erro
         get(dict, key)          # o mesmo para um dicionário

Quando os campos do registro são certos, `rythme check` encontra este erro antes
de o programa rodar, e sugere o campo mais parecido."#,
        ),
        (
            r#"E011 — Already defined

In Rythme a name always means the same value: it cannot be changed or defined
twice in the same place. This is what makes programs easy to follow.

Wrong:   x = 1
         x = x + 1
Right:   x = 1
         y = x + 1

For a function with several cases, write the definitions one right after the
other:
  fn fib(0) = 0
  fn fib(1) = 1
  fn fib(n) = fib(n - 1) + fib(n - 2)

The name given to a used file (`use geometry`, `use ... as geo`) cannot name
anything else in the file; and two files used with quotes (`use "a.ry"`)
cannot define the same name: use them by their names instead
(`use a`, then `a.area`)."#,
            r#"E011 — Já definido

No Rythme, um nome sempre significa o mesmo valor: ele não pode ser mudado nem
definido duas vezes no mesmo lugar. É isso que torna os programas fáceis de seguir.

Errado:  x = 1
         x = x + 1
Certo:   x = 1
         y = x + 1

Para uma função com vários casos, escreva as definições uma logo depois da
outra:
  fn fib(0) = 0
  fn fib(1) = 1
  fn fib(n) = fib(n - 1) + fib(n - 2)

O nome dado a um arquivo usado (`use geometry`, `use ... as geo`) não pode nomear
mais nada no arquivo; e dois arquivos usados com aspas (`use "a.ry"`) não podem
definir o mesmo nome: use-os pelos nomes em vez disso (`use a`, depois `a.area`)."#,
        ),
        (
            r#"E012 — Number too large (no longer given)

Whole numbers (Int) have no limit any more: they grow as needed, and stay
exact. Earlier versions of Rythme stopped with this error beyond 64 bits.

  2 ** 100        # 1267650600228229401496703205376
  2.0 ** 100      # 1.2676506002282294e30 (a Float is approximate)

Only a whole number of more than 100000 digits stops the program, with
E023: it is usually a calculation that grows without end."#,
            r#"E012 — Número grande demais (não acontece mais)

Os números inteiros (Int) não têm mais limite: eles crescem conforme necessário,
e continuam exatos. Versões antigas do Rythme paravam com este erro além de 64 bits.

  2 ** 100        # 1267650600228229401496703205376
  2.0 ** 100      # 1.2676506002282294e30 (um Float é aproximado)

Só um número inteiro com mais de 100000 algarismos para o programa, com
E023: em geral é um cálculo que cresce sem parar."#,
        ),
        (
            r#"E013 — unwrap found no value

`unwrap(x)` takes the value out of Some(...) or Ok(...), and stops the
program on None or Err(...). Handle the missing case instead:

  match get(list, 5)
    Some(x) -> x
    None -> 0
  end
or simply:  unwrap_or(get(list, 5), 0)"#,
            r#"E013 — unwrap não encontrou nenhum valor

`unwrap(x)` tira o valor de dentro de Some(...) ou Ok(...), e para o
programa com None ou Err(...). Trate o caso ausente em vez disso:

  match get(list, 5)
    Some(x) -> x
    None -> 0
  end
ou simplesmente:  unwrap_or(get(list, 5), 0)"#,
        ),
        (
            r#"E014 — Too many nested calls

A function calls itself (or others) too many times without finishing.
Either it never reaches its base case, or it needs to be written so that the
recursive call is the very last thing it does (a tail call): those calls
have no limit.

Limited:     fn sum([]) = 0
             fn sum([x, ..rest]) = x + sum(rest)       # + happens after
No limit:    fn sum([], acc) = acc
             fn sum([x, ..rest], acc) = sum(rest, acc + x)
Simplest:    sum(list)  or  fold(list, 0, fn(a, x) = a + x)"#,
            r#"E014 — Chamadas aninhadas demais

Uma função chama a si mesma (ou outras) vezes demais sem terminar. Ou ela nunca
chega ao seu caso base, ou precisa ser escrita de modo que a chamada recursiva
seja a última coisa que ela faz (uma chamada de cauda): essas chamadas não têm
limite.

Limitado:    fn sum([]) = 0
             fn sum([x, ..rest]) = x + sum(rest)       # o + acontece depois
Sem limite:  fn sum([], acc) = acc
             fn sum([x, ..rest], acc) = sum(rest, acc + x)
Simples:     sum(list)  ou  fold(list, 0, fn(a, x) = a + x)"#,
        ),
        (
            r#"E015 — Assertion failed

`assert(condition)` or `assert_eq(actual, expected)` found that something is
not as expected. In a test, this means the test fails: the message shows the
values to compare.

`rythme check` finds before running an `assert_eq` that can never pass,
because its two values are of kinds that are never equal:
Wrong:   assert_eq(parse_int("3"), 3)      # a Result is never an Int
Right:   assert_eq(parse_int("3"), Ok(3))

`check_all(property, generator)` reports E015 too when a value makes the
property false: it shows the simplest value found, and the seed that gave it
(the same values come back on every run; `rythme test --seed N` tries others)."#,
            r#"E015 — Asserção falhou

`assert(condition)` ou `assert_eq(actual, expected)` descobriu que algo não está
como esperado. Em um teste, isso quer dizer que o teste falha: a mensagem mostra
os valores a comparar.

`rythme check` encontra antes de executar um `assert_eq` que nunca pode passar,
porque os seus dois valores são de tipos que nunca são iguais:
Errado:  assert_eq(parse_int("3"), 3)      # um Result nunca é um Int
Certo:   assert_eq(parse_int("3"), Ok(3))

`check_all(property, generator)` também aponta E015 quando um valor torna a
propriedade falsa: ele mostra o valor mais simples encontrado, e a semente que o
gerou (os mesmos valores voltam a cada execução; `rythme test --seed N` testa outros)."#,
        ),
        (
            r#"E016 — Wrong use of ?

`x?` takes the value out of Some(...) or Ok(...); on None or Err(...), it makes
the current function return it immediately. So it only works inside a
function, on an Option or a Result.

  fn total(a, b)
    x = parse_int(a)?
    y = parse_int(b)?
    Ok(x + y)
  end"#,
            r#"E016 — Uso errado de ?

`x?` tira o valor de dentro de Some(...) ou Ok(...); com None ou Err(...), ele faz
a função atual devolvê-lo imediatamente. Então só funciona dentro de uma
função, com uma Option ou um Result.

  fn total(a, b)
    x = parse_int(a)?
    y = parse_int(b)?
    Ok(x + y)
  end"#,
        ),
        (
            r#"E017 — File problem

A file could not be read, or a file given to `use` was not found, or files
use each other in a circle. Paths in `use` are relative to the folder of the
file that contains the `use`. Programs are text files saved in UTF-8 (the
usual choice of editors).

`input()` also stops with this error when there is nothing more to read:
the keyboard input was closed (Ctrl-D), or the text sent to the program
(`rythme prog.ry < answers.txt`) has no more lines."#,
            r#"E017 — Problema com um arquivo

Um arquivo não pôde ser lido, ou um arquivo dado a `use` não foi encontrado, ou
arquivos usam uns aos outros em círculo. Os caminhos em `use` são relativos à
pasta do arquivo que contém o `use`. Os programas são arquivos de texto salvos
em UTF-8 (a escolha comum dos editores).

`input()` também para com este erro quando não há mais nada para ler: a entrada
do teclado foi fechada (Ctrl-D), ou o texto enviado ao programa
(`rythme prog.ry < respostas.txt`) não tem mais linhas."#,
        ),
        (
            r#"E018 — Unexpected character

This character has no meaning in Rythme here. Texts use straight double
quotes "...", powers use **, and comments start with #."#,
            r#"E018 — Caractere inesperado

Este caractere não tem sentido aqui no Rythme. Os textos usam aspas duplas retas
"...", as potências usam **, e os comentários começam com #."#,
        ),
        (
            r#"E019 — if without else

In Rythme, `if` is an expression: it always gives a value, so both cases are
needed. Each branch holds a single expression; use `do ... end` for several.

Wrong:   if x > 0 then print("positive")
Right:   if x > 0 then print("positive") else ()
Right:   if x > 0 then "positive" else if x == 0 then "zero" else "negative""#,
            r#"E019 — if sem else

No Rythme, `if` é uma expressão: ele sempre dá um valor, então os dois casos são
necessários. Cada ramo contém uma única expressão; use `do ... end` para várias.

Errado:  if x > 0 then print("positive")
Certo:   if x > 0 then print("positive") else ()
Certo:   if x > 0 then "positive" else if x == 0 then "zero" else "negative""#,
        ),
        (
            r#"E020 — Missing end

A function written on several lines, a `match` or a `do` block must finish
with `end` on its own line.

  fn greet(name)
    print("Hello {name}")
  end"#,
            r#"E020 — Falta end

Uma função escrita em várias linhas, um `match` ou um bloco `do` deve terminar
com `end` em uma linha própria.

  fn greet(name)
    print("Hello {name}")
  end"#,
        ),
        (
            r#"E021 — Program stopped by fail

The program called `fail(message)`, which stops everything. For errors that
the caller could handle, return Err(message) instead."#,
            r#"E021 — Programa parado por fail

O programa chamou `fail(message)`, que para tudo. Para erros que quem chamou
poderia tratar, devolva Err(message) em vez disso."#,
        ),
        (
            r#"E022 — Impossible conversion

The text does not contain a number. When the text comes from a user or a
file, use the safe versions, which give a Result:

  match parse_int(input("Age? "))
    Ok(age) -> print("Next year: {age + 1}")
    Err(e) -> print(e)
  end"#,
            r#"E022 — Conversão impossível

O texto não contém um número. Quando o texto vem de um usuário ou de um arquivo,
use as versões seguras, que dão um Result:

  match parse_int(input("Age? "))
    Ok(age) -> print("Next year: {age + 1}")
    Err(e) -> print(e)
  end"#,
        ),
        (
            r#"E023 — Invalid value

The function received a value of the right kind, but outside of what it
accepts: a negative count, the minimum of an empty list, the square root of
a negative number, a date that does not exist (30 February), a range, a
list or a text too large, a whole number of more than 100000 digits... The
message explains the limit."#,
            r#"E023 — Valor inválido

A função recebeu um valor do tipo certo, mas fora do que ela aceita: uma
contagem negativa, o mínimo de uma lista vazia, a raiz quadrada de um número
negativo, uma data que não existe (30 de fevereiro), um intervalo, uma lista ou
um texto grande demais, um número inteiro com mais de 100000 algarismos... A
mensagem explica o limite."#,
        ),
        (
            r#"E024 — A value is lost

A line computes a value, but nothing uses it: only the last line of a block
gives its value, and values never change, so the result is simply lost.

Wrong:   total = price
         - discount              # a new line: this is a separate statement
Right:   total = price -
           discount              # end the line with the operator to continue
Wrong:   x + 1                   # does not change x
Right:   y = x + 1
To show a value, print it:  print(x + 1)"#,
            r#"E024 — Um valor se perde

Uma linha calcula um valor, mas nada o usa: só a última linha de um bloco dá o
seu valor, e os valores nunca mudam, então o resultado simplesmente se perde.

Errado:  total = price
         - discount              # uma nova linha: isto é uma instrução separada
Certo:   total = price -
           discount              # termine a linha com o operador para continuar
Errado:  x + 1                   # não muda x
Certo:   y = x + 1
Para mostrar um valor, mostre-o com print:  print(x + 1)"#,
        ),
        (
            r#"E025 — Project or package problem

The `rythme.toml` of the project (or the `rythme.lock` written by
`rythme install`) cannot be read, a dependency is not installed, or
`rythme install` cannot download one. A `rythme.toml` looks like this:

  name = "hello"                 # a Rythme name in lower case
  version = "0.1.0"              # three numbers
  entry = "src/main.ry"          # the program (this one by default)

  [dependencies]
  mathlib = { git = "https://github.com/someone/mathlib", tag = "v1.0.0" }
  tools = { path = "../tools" }

Every value is a text, in quotes. A dependency from git needs a fixed
version, a `tag` or a `commit`, and the program `git`
(https://git-scm.com). After a change of `[dependencies]`, run
`rythme install` in the folder of the project. More: `rythme help projects`."#,
            r#"E025 — Problema de projeto ou de pacote

O `rythme.toml` do projeto (ou o `rythme.lock` escrito por `rythme install`) não
pode ser lido, uma dependência não está instalada, ou `rythme install` não
consegue baixar uma. Um `rythme.toml` é assim:

  name = "hello"                 # um nome do Rythme em minúsculas
  version = "0.1.0"              # três números
  entry = "src/main.ry"          # o programa (este, por padrão)

  [dependencies]
  mathlib = { git = "https://github.com/someone/mathlib", tag = "v1.0.0" }
  tools = { path = "../tools" }

Cada valor é um texto, entre aspas. Uma dependência do git precisa de uma versão
fixa, uma `tag` ou um `commit`, e do programa `git` (https://git-scm.com).
Depois de mudar `[dependencies]`, execute `rythme install` na pasta do projeto.
Mais: `rythme help projects`."#,
        ),
        (
            r#"E026 — Invalid regex

A regex (a regular expression, given to `matches`, `find_all`, `split_by`,
`replace_all`...) breaks one of its rules, or the replacement of
`replace_all` uses a group that does not exist. The message shows where.

Wrong:   find_all(text, "([0-9]+")       # the `(` is never closed
Right:   find_all(text, "([0-9]+)")
Wrong:   split_by(text, "*")             # `*` repeats nothing
Right:   split_by(text, "[*]")           # the sign * itself, in brackets
Wrong:   replace_all(date, "(\\d+)-(\\d+)", "$3")    # only 2 groups
Right:   replace_all(date, "(\\d+)-(\\d+)", "$2/$1")

In a Rythme text, a backslash is written twice ("\\d") and braces are
written \{ and \} ("[0-9]\{4\}"). `rythme help regex` shows everything a
regex can say. When the regex is written in the code, `rythme check` finds
the mistake before running."#,
            r#"E026 — Regex inválida

Uma regex (uma expressão regular, dada a `matches`, `find_all`, `split_by`,
`replace_all`...) quebra uma das suas regras, ou a substituição de
`replace_all` usa um grupo que não existe. A mensagem mostra onde.

Errado:  find_all(text, "([0-9]+")       # o `(` nunca é fechado
Certo:   find_all(text, "([0-9]+)")
Errado:  split_by(text, "*")             # `*` não repete nada
Certo:   split_by(text, "[*]")           # o próprio sinal *, entre colchetes
Errado:  replace_all(date, "(\\d+)-(\\d+)", "$3")    # só 2 grupos
Certo:   replace_all(date, "(\\d+)-(\\d+)", "$2/$1")

Em um texto do Rythme, a barra invertida se escreve duas vezes ("\\d") e as
chaves se escrevem \{ e \} ("[0-9]\{4\}"). `rythme help regex` mostra tudo o
que uma regex pode dizer. Quando a regex está escrita no código, `rythme check`
encontra o erro antes de executar."#,
        ),
        (
            r#"W001 — Unused name

A name is defined but never used. It is often a typo or a forgotten step.
If it is on purpose, start the name with `_`: `_unused = ...`."#,
            r#"W001 — Nome não usado

Um nome é definido mas nunca usado. Muitas vezes é um erro de digitação ou uma
etapa esquecida. Se for de propósito, comece o nome com `_`: `_unused = ...`."#,
        ),
        (
            r#"W002 — Forgotten case

A `match` (or a function with several definitions) does not handle every
possible value. The message lists what is missing. If that value comes, the
program will stop with E006.

  type Light = Red | Orange | Green
  fn next(Red) = Green
  fn next(Green) = Orange
  # W002: `next` does not handle `Orange`. Add the missing definition:
  fn next(Orange) = Red"#,
            r#"W002 — Caso esquecido

Um `match` (ou uma função com várias definições) não trata todos os valores
possíveis. A mensagem lista o que falta. Se esse valor aparecer, o programa
vai parar com E006.

  type Light = Red | Orange | Green
  fn next(Red) = Green
  fn next(Green) = Orange
  # W002: `next` não trata `Orange`. Adicione a definição que falta:
  fn next(Orange) = Red"#,
        ),
        (
            r#"W003 — Built-in function hidden

You defined a name that already belongs to a built-in function. In your file,
the name now means your definition. Choose another name to keep both."#,
            r#"W003 — Função embutida escondida

Você definiu um nome que já pertence a uma função embutida. No seu arquivo, o
nome agora significa a sua definição. Escolha outro nome para manter os dois."#,
        ),
        (
            r#"W004 — Case never reached

Cases are tried in order, and a value goes to the first one that fits. A
case (or a definition of a function) can never be used when:
  - a case before it accepts everything (`_`, or a plain name): put that
    one last;
  - a case before it already handles the same values: `1 -> ...` twice;
  - the value can never have this shape: a Text never matches a list
    pattern like `[first, ..rest]`, a number never matches `Some(x)`;
  - a range is empty: `9..1` holds no number, write `1..9`.

  match name                     # `name` is a Text
    [first, ..] -> first         # W004: a Text is not a list
    _ -> "?"
  end
To look at the characters of a text, use `chars(name)`."#,
            r#"W004 — Caso nunca alcançado

Os casos são testados em ordem, e um valor vai para o primeiro que serve. Um
caso (ou uma definição de uma função) nunca pode ser usado quando:
  - um caso anterior aceita tudo (`_`, ou um nome simples): coloque esse
    por último;
  - um caso anterior já trata os mesmos valores: `1 -> ...` duas vezes;
  - o valor nunca pode ter essa forma: um Text nunca corresponde a um padrão
    de lista como `[first, ..rest]`, um número nunca corresponde a `Some(x)`;
  - um intervalo está vazio: `9..1` não contém nenhum número, escreva `1..9`.

  match name                     # `name` é um Text
    [first, ..] -> first         # W004: um Text não é uma lista
    _ -> "?"
  end
Para olhar os caracteres de um texto, use `chars(name)`."#,
        ),
        (
            r#"W005 — Unknown function after a dot

`value.name(args)` calls the function `name(value, args)`, or the field `name`
of a record. No function has this name, and no record of the file has such a
field: it is probably a typo.

Wrong:   [1, 2].lenght()
Right:   [1, 2].len()"#,
            r#"W005 — Função desconhecida depois de um ponto

`value.name(args)` chama a função `name(value, args)`, ou o campo `name` de um
registro. Nenhuma função tem este nome, e nenhum registro do arquivo tem esse
campo: provavelmente é um erro de digitação.

Errado:  [1, 2].lenght()
Certo:   [1, 2].len()"#,
        ),
        (
            r#"W006 — Comparison always false

Values of different kinds are never equal: Rythme never converts them
silently. When `x` is a number, `x == "1"` is always false and `x != "1"`
always true. It is often a forgotten conversion, or a value inside an Option
compared as if it were outside.

Wrong:   if count == "3" then ...            (a number and a text)
Right:   if count == 3 then ...
Wrong:   get(scores, "ada") == 19            (an Option and a number)
Right:   get(scores, "ada") == Some(19)
Right:   unwrap_or(get(scores, "ada"), 0) == 19"#,
            r#"W006 — Comparação sempre falsa

Valores de tipos diferentes nunca são iguais: o Rythme nunca os converte sem
avisar. Quando `x` é um número, `x == "1"` é sempre falso e `x != "1"` sempre
verdadeiro. Muitas vezes é uma conversão esquecida, ou um valor dentro de uma
Option comparado como se estivesse fora.

Errado:  if count == "3" then ...            (um número e um texto)
Certo:   if count == 3 then ...
Errado:  get(scores, "ada") == 19            (uma Option e um número)
Certo:   get(scores, "ada") == Some(19)
Certo:   unwrap_or(get(scores, "ada"), 0) == 19"#,
        ),
        (
            r#"W007 — Test that can never fail

A test fails only when it stops with an error, or when it ends with `false`,
`None` or `Err(...)`. A test that only computes or prints a value with
functions that never stop, and never compares it with what is expected,
can never fail: it checks nothing.

Wrong:   test "length" = len([1, 2])
Wrong:   test "length" = print(len([1, 2]))
Right:   test "length" = assert_eq(len([1, 2]), 2)
Right:   test "length" = len([1, 2]) == 2"#,
            r#"W007 — Teste que nunca pode falhar

Um teste só falha quando para com um erro, ou quando termina com `false`,
`None` ou `Err(...)`. Um teste que só calcula ou mostra um valor com funções
que nunca param, e nunca o compara com o que é esperado, nunca pode falhar:
ele não verifica nada.

Errado:  test "length" = len([1, 2])
Errado:  test "length" = print(len([1, 2]))
Certo:   test "length" = assert_eq(len([1, 2]), 2)
Certo:   test "length" = len([1, 2]) == 2"#,
        ),
        (
            r#"W008 — Result lost

Values never change in Rythme: `push(list, 4)` does not add 4 to `list`, it
gives a new list. A line that only calls such a function, and does nothing
with what it gives, does nothing at all.

Wrong:   push(numbers, 4)               # `numbers` is unchanged
Right:   more = push(numbers, 4)
Wrong:   name.upper()
Right:   print(name.upper())"#,
            r#"W008 — Resultado perdido

Os valores nunca mudam no Rythme: `push(list, 4)` não adiciona 4 a `list`, ele
dá uma lista nova. Uma linha que só chama uma função assim, e não faz nada com
o que ela dá, não faz nada.

Errado:  push(numbers, 4)               # `numbers` continua igual
Certo:   more = push(numbers, 4)
Errado:  name.upper()
Certo:   print(name.upper())"#,
        ),
        (
            r#"W009 — A function shown instead of its value

A function shown in a text or printed appears as `<fn name>`. To show what
it gives, call it, with parentheses, even when it takes no argument.

  fn greeting() = "hello"
Wrong:   print("{greeting}")          # shows <fn greeting>
Right:   print("{greeting()}")        # shows hello"#,
            r#"W009 — Uma função mostrada em vez do seu valor

Uma função mostrada em um texto ou impressa aparece como `<fn name>`. Para
mostrar o que ela dá, chame-a, com parênteses, mesmo quando ela não recebe
nenhum argumento.

  fn greeting() = "hello"
Errado:  print("{greeting}")          # mostra <fn greeting>
Certo:   print("{greeting()}")        # mostra hello"#,
        ),
        (
            r#"W010 — Ranges that overlap

A range pattern `a..b` matches the numbers from `a` to `b`, both included.
When two ranges of a `match` (or of the definitions of a function) share
numbers, these numbers always go to the first one: the second never sees
them. It is often an end written twice.

Wrong:   match age
           0..18 -> "young"
           18..65 -> "adult"         # W010: 18 goes to "young"
           _ -> "senior"
         end
Right:   0..17 -> "young"
         18..64 -> "adult"
For decimal numbers, which fall between 17 and 18, compare instead:
  a if a < 18 -> "young""#,
            r#"W010 — Intervalos que se sobrepõem

Um padrão de intervalo `a..b` corresponde aos números de `a` a `b`, os dois
incluídos. Quando dois intervalos de um `match` (ou das definições de uma
função) têm números em comum, esses números sempre vão para o primeiro: o
segundo nunca os vê. Muitas vezes é uma ponta escrita duas vezes.

Errado:  match age
           0..18 -> "young"
           18..65 -> "adult"         # W010: 18 vai para "young"
           _ -> "senior"
         end
Certo:   0..17 -> "young"
         18..64 -> "adult"
Para números decimais, que ficam entre 17 e 18, compare em vez disso:
  a if a < 18 -> "young""#,
        ),
        ("Defines a function: `fn name(x) = expression`, or with its body on the following lines, finished by `end`. Without a name, `fn(x) = x * 2` is an anonymous function.", "Define uma função: `fn name(x) = expression`, ou com o corpo nas linhas seguintes, terminado por `end`. Sem nome, `fn(x) = x * 2` é uma função anônima."),
        ("Chooses between two values: `if condition then value else other`. Every `if` has an `else`, because an `if` always gives a value.", "Escolhe entre dois valores: `if condition then value else other`. Todo `if` tem um `else`, porque um `if` sempre dá um valor."),
        ("Separates the condition of an `if` from the value it gives when the condition is true.", "Separa a condição de um `if` do valor que ele dá quando a condição é verdadeira."),
        ("Gives the value of an `if` when the condition is false. It is always required; `else if` chains several conditions.", "Dá o valor de um `if` quando a condição é falsa. É sempre obrigatório; `else if` encadeia várias condições."),
        ("The opposite of `if`: `unless condition then a else b` gives `b` when the condition is true, `a` otherwise. Like `if`, it always has an `else`.", "O contrário de `if`: `unless condition then a else b` dá `b` quando a condição é verdadeira, e `a` caso contrário. Como `if`, sempre tem um `else`."),
        ("Chooses according to the shape of a value: one case per line, `pattern -> result`, finished by `end`. Cases are tried in order, and `rythme check` warns about forgotten cases.", "Escolhe de acordo com a forma de um valor: um caso por linha, `pattern -> result`, terminado por `end`. Os casos são testados em ordem, e `rythme check` avisa sobre casos esquecidos."),
        ("Closes a block: a function written on several lines, a `match`, a `do` or a `test`.", "Fecha um bloco: uma função escrita em várias linhas, um `match`, um `do` ou um `test`."),
        ("Groups several steps inside an expression, up to `end`. The last line gives the value. After a call, `do |x| ... end` is a block: a function given as the last argument.", "Agrupa várias etapas dentro de uma expressão, até `end`. A última linha dá o valor. Depois de uma chamada, `do |x| ... end` é um bloco: uma função dada como último argumento."),
        ("Defines your own type and its variants. Each variant is a constructor, with or without fields.", "Define o seu próprio tipo e as suas variantes. Cada variante é um construtor, com ou sem campos."),
        ("Defines a test, next to the code. `rythme test file.ry` runs the tests of the file.", "Define um teste, ao lado do código. `rythme test arquivo.ry` executa os testes do arquivo."),
        ("Uses another file through its name: `use geometry`, then `geometry.area(2)`. The path is relative to the folder of the file that contains `use`.", "Usa outro arquivo pelo nome dele: `use geometry`, depois `geometry.area(2)`. O caminho é relativo à pasta do arquivo que contém `use`."),
        ("True when both conditions are true. The second one is computed only if the first one is true.", "Verdadeiro quando as duas condições são verdadeiras. A segunda só é calculada se a primeira for verdadeira."),
        ("True when at least one of the conditions is true. The second one is computed only if the first one is false.", "Verdadeiro quando pelo menos uma das condições é verdadeira. A segunda só é calculada se a primeira for falsa."),
        ("Inverts a condition: `not true` is `false`.", "Inverte uma condição: `not true` é `false`."),
        ("One of the two values of the type Bool.", "Um dos dois valores do tipo Bool."),
        ("Shows values on the screen, separated by spaces, then goes to the next line.", "Mostra valores na tela, separados por espaços, e depois passa para a linha seguinte."),
        ("Shows a value with its position in the code (on the error output) and returns it unchanged. Handy in the middle of a pipeline.", "Mostra um valor com a sua posição no código (na saída de erros) e o devolve sem mudança. Útil no meio de um pipeline."),
        ("Converts any value to text. With a number of decimals, writes a number with exactly that many digits after the point.", "Converte qualquer valor em texto. Com um número de casas decimais, escreve um número com exatamente essa quantidade de algarismos depois do ponto."),
        ("Shows the prompt, then reads a line typed on the keyboard and returns it as text.", "Mostra o prompt, depois lê uma linha digitada no teclado e a devolve como texto."),
        ("Reads all the lines of the standard input, until its end: `Ok(list of texts)`, or `Err(message)` if it cannot be read. For `rythme count.ry < names.txt`, or text typed until Ctrl-D (Ctrl-Z then Enter on Windows).", "Lê todas as linhas da entrada padrão, até o fim: `Ok(list of texts)`, ou `Err(message)` se ela não puder ser lida. Para `rythme count.ry < names.txt`, ou texto digitado até Ctrl-D (Ctrl-Z e depois Enter no Windows)."),
        ("Returns the name of the type of a value: \"Int\", \"Float\", \"Text\", \"Bool\", \"List\", \"Record\", \"Function\", \"Unit\" (for `()`), \"Symbol\" (for `:ok`), \"Option\", \"Result\", or the name of your own type.", "Devolve o nome do tipo de um valor: \"Int\", \"Float\", \"Text\", \"Bool\", \"List\", \"Record\", \"Function\", \"Unit\" (para `()`), \"Symbol\" (para `:ok`), \"Option\", \"Result\", ou o nome do seu próprio tipo."),
        ("Shows help: about a function (`help(map)`), a topic (`help(\"lists\")`), or what you can do with a value (`help([1, 2])`).", "Mostra ajuda: sobre uma função (`help(map)`), um tema (`help(\"lists\")`), ou o que você pode fazer com um valor (`help([1, 2])`)."),
        ("Stops with an error if the condition is false. Mostly used in tests.", "Para com um erro se a condição for falsa. Usado principalmente em testes."),
        ("Stops with an error showing both values if they are different. Mostly used in tests.", "Para com um erro que mostra os dois valores se eles forem diferentes. Usado principalmente em testes."),
        ("Stops the program with an error message. Prefer returning `Err(...)` for errors that can be handled.", "Para o programa com uma mensagem de erro. Prefira devolver `Err(...)` para erros que podem ser tratados."),
        ("Returns the list of the words given after the file name on the command line.", "Devolve a lista das palavras dadas depois do nome do arquivo na linha de comando."),
        ("Reads the options of the command line (the words of `args()`, or the list given) into a record, in a Result: `--name=value` gives the text `value`, `--name` and `-n` give `true`, a `-` in a name becomes `_`, and the other words are in the field `args` (all of them after `--`). An option given twice, or a word like `-abc`, gives `Err(message)`. Use `get(opts, \"name\")` or `has(opts, \"name\")` for an option that may be missing.", "Lê as opções da linha de comando (as palavras de `args()`, ou a lista dada) para um registro, em um Result: `--name=value` dá o texto `value`, `--name` e `-n` dão `true`, um `-` em um nome vira `_`, e as outras palavras ficam no campo `args` (todas as que vêm depois de `--`). Uma opção dada duas vezes, ou uma palavra como `-abc`, dá `Err(message)`. Use `get(opts, \"name\")` ou `has(opts, \"name\")` para uma opção que pode faltar."),
        ("The value of an environment variable of the system, as an Option: `Some(text)`, or `None` if it is not set.", "O valor de uma variável de ambiente do sistema, como Option: `Some(text)`, ou `None` se ela não estiver definida."),
        ("Stops the program immediately. The code (0 by default) tells the system if everything went well (0) or not; Rythme itself exits with 1 after an error in the program (see `rythme help system`).", "Para o programa imediatamente. O código (0 por padrão) diz ao sistema se tudo correu bem (0) ou não; o próprio Rythme sai com 1 depois de um erro no programa (veja `rythme help system`)."),
        ("Returns the current time in seconds (a Float), counted from 1 January 1970. Subtract two times to measure a duration.", "Devolve a hora atual em segundos (um Float), contados a partir de 1º de janeiro de 1970. Subtraia dois tempos para medir uma duração."),
        ("random() gives a decimal number between 0 and 1; random(a, b) a whole number from a to b (included); random(list) a random element.", "random() dá um número decimal entre 0 e 1; random(a, b) um número inteiro de a a b (incluídos); random(list) um elemento aleatório."),
        ("Absolute value: the number without its sign.", "Valor absoluto: o número sem o sinal."),
        ("The smallest of the values, or of the elements of a list.", "O menor dos valores, ou dos elementos de uma lista."),
        ("The largest of the values, or of the elements of a list.", "O maior dos valores, ou dos elementos de uma lista."),
        ("Square root.", "Raiz quadrada."),
        ("x to the power n (same as `x ** n`).", "x elevado a n (o mesmo que `x ** n`)."),
        ("Rounds down to a whole number.", "Arredonda para baixo, para um número inteiro."),
        ("Rounds up to a whole number.", "Arredonda para cima, para um número inteiro."),
        ("Rounds to the nearest whole number, or to the given number of decimal digits.", "Arredonda para o número inteiro mais próximo, ou para o número de casas decimais dado."),
        ("Converts to a whole number (Int): decimals are cut, texts are read.", "Converte em número inteiro (Int): as casas decimais são cortadas, os textos são lidos."),
        ("Converts to a decimal number (Float).", "Converte em número decimal (Float)."),
        ("Sine (x in radians).", "Seno (x em radianos)."),
        ("Cosine (x in radians).", "Cosseno (x em radianos)."),
        ("Tangent (x in radians).", "Tangente (x em radianos)."),
        ("Arc sine: the angle (in radians) whose sine is x, for x from -1 to 1.", "Arco seno: o ângulo (em radianos) cujo seno é x, para x de -1 a 1."),
        ("Arc cosine: the angle (in radians) whose cosine is x, for x from -1 to 1.", "Arco cosseno: o ângulo (em radianos) cujo cosseno é x, para x de -1 a 1."),
        ("Arc tangent: the angle (in radians) whose tangent is y. With two numbers, the angle of the point (x, y), from -pi to pi.", "Arco tangente: o ângulo (em radianos) cuja tangente é y. Com dois números, o ângulo do ponto (x, y), de -pi a pi."),
        ("Logarithm: natural (base e) by default, or in the given base.", "Logaritmo: natural (base e) por padrão, ou na base dada."),
        ("e to the power x.", "e elevado a x."),
        ("Is the whole number even?", "O número inteiro é par?"),
        ("Is the whole number odd?", "O número inteiro é ímpar?"),
        ("Greatest common divisor of two whole numbers: the largest number that divides both.", "Máximo divisor comum de dois números inteiros: o maior número que divide os dois."),
        ("Least common multiple of two whole numbers: the smallest number divisible by both.", "Mínimo múltiplo comum de dois números inteiros: o menor número divisível pelos dois."),
        ("Length of a text (in characters), a list, a record, a dict or a set.", "Comprimento de um texto (em caracteres), de uma lista, de um registro, de um dicionário ou de um conjunto."),
        ("The text in uppercase.", "O texto em maiúsculas."),
        ("The text in lowercase.", "O texto em minúsculas."),
        ("The text with its first character in uppercase (the rest is unchanged).", "O texto com o primeiro caractere em maiúscula (o resto fica igual)."),
        ("The text without spaces and line breaks at the start and the end.", "O texto sem espaços e quebras de linha no início e no fim."),
        ("Adds `fill` (a space by default) before the text until it is `width` characters long: numbers line up in columns. A value that is not a text is first shown as `print` shows it.", "Adiciona `fill` (um espaço por padrão) antes do texto até ele ter `width` caracteres: os números se alinham em colunas. Um valor que não é texto é primeiro mostrado como `print` o mostra."),
        ("Adds `fill` (a space by default) after the text until it is `width` characters long.", "Adiciona `fill` (um espaço por padrão) depois do texto até ele ter `width` caracteres."),
        ("Cuts a text into a list of texts. Without separator, cuts at spaces.", "Corta um texto em uma lista de textos. Sem separador, corta nos espaços."),
        ("Glues the elements of a list into one text, with the separator between them.", "Cola os elementos de uma lista em um único texto, com o separador entre eles."),
        ("Does the text contain this piece of text? Does the list (or the set) contain this element? Does the record have this field, or the dict this key?", "O texto contém este pedaço de texto? A lista (ou o conjunto) contém este elemento? O registro tem este campo, ou o dicionário esta chave?"),
        ("Does the text start with this piece of text?", "O texto começa com este pedaço de texto?"),
        ("Does the text end with this piece of text?", "O texto termina com este pedaço de texto?"),
        ("Replaces every occurrence of `old` by `new`.", "Substitui cada ocorrência de `old` por `new`."),
        ("The list of the characters of a text.", "A lista dos caracteres de um texto."),
        ("The list of the lines of a text.", "A lista das linhas de um texto."),
        ("The number of a character (its Unicode code point).", "O número de um caractere (o seu ponto de código Unicode)."),
        ("The character with this number (Unicode code point): the opposite of `char_code`.", "O caractere com este número (ponto de código Unicode): o contrário de `char_code`."),
        ("Is the text made only of digits from 0 to 9 (and not empty)?", "O texto é feito só de algarismos de 0 a 9 (e não está vazio)?"),
        ("Is the text made only of letters, accented ones included (and not empty)?", "O texto é feito só de letras, incluindo as acentuadas (e não está vazio)?"),
        ("Is the text made only of spaces, tabs and line breaks (and not empty)?", "O texto é feito só de espaços, tabulações e quebras de linha (e não está vazio)?"),
        ("A text or a list repeated n times.", "Um texto ou uma lista repetidos n vezes."),
        ("Reads a whole number from a text: `Ok(number)`, or `Err(message)` if the text is not a number.", "Lê um número inteiro de um texto: `Ok(number)`, ou `Err(message)` se o texto não for um número."),
        ("Reads a decimal number from a text: `Ok(number)` or `Err(message)`.", "Lê um número decimal de um texto: `Ok(number)` ou `Err(message)`."),
        ("Reads a JSON text: `Ok(value)`, or `Err(message)` if the text is not valid JSON. Objects become records, arrays lists, and null `None`.", "Lê um texto JSON: `Ok(value)`, ou `Err(message)` se o texto não for JSON válido. Os objetos viram registros, os arrays listas, e null vira `None`."),
        ("Writes a value as JSON text: records become objects, lists arrays, and `None` null. With `indent`, one element per line, shifted by that many spaces.", "Escreve um valor como texto JSON: os registros viram objetos, as listas arrays, e `None` vira null. Com `indent`, um elemento por linha, deslocado de tantos espaços."),
        ("The text without spaces and line breaks at the start.", "O texto sem espaços e quebras de linha no início."),
        ("The text without spaces and line breaks at the end.", "O texto sem espaços e quebras de linha no fim."),
        ("The list of the words of a text: letters and digits, accents included, with `'` or `-` inside a word. Punctuation and spaces separate them.", "A lista das palavras de um texto: letras e algarismos, acentos incluídos, com `'` ou `-` dentro de uma palavra. A pontuação e os espaços as separam."),
        ("Are the two texts the same when uppercase and lowercase letters count as equal?", "Os dois textos são iguais quando letras maiúsculas e minúsculas contam como iguais?"),
        ("Writes a number for people: thousands separated, with exactly `decimals` decimals if given, in the style of a language: \"en\" (1,234.5, by default), \"fr\" (1 234,5), \"de\", \"es\", \"it\", \"nl\" or \"pt\" (1.234,5). To line it up in a column, use `pad_left`.", "Escreve um número para pessoas: milhares separados, com exatamente `decimals` casas decimais se dado, no estilo de um idioma: \"en\" (1,234.5, por padrão), \"fr\" (1 234,5), \"de\", \"es\", \"it\", \"nl\" ou \"pt\" (1.234,5). Para alinhá-lo em uma coluna, use `pad_left`."),
        ("Does the whole text match the regex (a regular expression: see `rythme help regex`)? To look for the regex anywhere in the text, use `find_first`.", "O texto inteiro corresponde à regex (uma expressão regular: veja `rythme help regex`)? Para procurar a regex em qualquer lugar do texto, use `find_first`."),
        ("The first piece of the text that matches the regex, as an Option: `Some(piece)`, or `None` if there is none.", "O primeiro pedaço do texto que corresponde à regex, como Option: `Some(piece)`, ou `None` se não houver nenhum."),
        ("Every piece of the text that matches the regex, from left to right.", "Cada pedaço do texto que corresponde à regex, da esquerda para a direita."),
        ("The first match of the regex, as an Option of a list: the whole piece, then the text of each group `(...)`, in order (\"\" for a group that matched nothing).", "A primeira correspondência da regex, como Option de uma lista: o pedaço inteiro, depois o texto de cada grupo `(...)`, em ordem (\"\" para um grupo que não correspondeu a nada)."),
        ("Like `captures`, for every match: a list of lists.", "Como `captures`, para cada correspondência: uma lista de listas."),
        ("The first match of a regex with named groups `(?<name>...)`, as an Option of a record: one field for each name.", "A primeira correspondência de uma regex com grupos com nome `(?<name>...)`, como Option de um registro: um campo para cada nome."),
        ("Cuts a text at every piece that matches the regex.", "Corta um texto em cada pedaço que corresponde à regex."),
        ("Replaces every piece that matches the regex. `new` is a text, where `$1` stands for the text of group 1 (`$<name>` for a named group, `$0` for the whole piece, `$$` for a `$`), or a function that receives each piece and gives its replacement.", "Substitui cada pedaço que corresponde à regex. `new` é um texto, em que `$1` representa o texto do grupo 1 (`$<name>` para um grupo com nome, `$0` para o pedaço inteiro, `$$` para um `$`), ou uma função que recebe cada pedaço e dá a substituição dele."),
        ("Applies the function f to each element and returns the list of the results. Also works on Option and Result, and on a set (giving the set of the results).", "Aplica a função f a cada elemento e devolve a lista dos resultados. Também funciona com Option e Result, e com um conjunto (dando o conjunto dos resultados)."),
        ("Keeps only the elements for which f returns true. Also works on a set, and on a dict, where f receives each key and its value.", "Mantém só os elementos para os quais f devolve true. Também funciona com um conjunto, e com um dicionário, em que f recebe cada chave e o seu valor."),
        ("Combines all the elements into one value: starts with `start`, then computes f(result, element) for each element.", "Combina todos os elementos em um só valor: começa com `start`, depois calcula f(result, element) para cada elemento."),
        ("Like fold, but gives the list of every intermediate result: running totals, for example.", "Como fold, mas dá a lista de cada resultado intermediário: totais parciais, por exemplo."),
        ("Calls f on each element, for its effect (like printing). Returns (). Also works on a set, and on a dict, where f receives each key and its value.", "Chama f para cada elemento, pelo seu efeito (como mostrar algo). Devolve (). Também funciona com um conjunto, e com um dicionário, em que f recebe cada chave e o seu valor."),
        ("The sum of the numbers of a list; with f, the sum of f(x) for each element.", "A soma dos números de uma lista; com f, a soma de f(x) para cada elemento."),
        ("The product of the numbers of a list.", "O produto dos números de uma lista."),
        ("The first element of a list (or character of a text), as an Option: `Some(x)`, or `None` if there is none. With n, the first n elements (like `take`).", "O primeiro elemento de uma lista (ou caractere de um texto), como Option: `Some(x)`, ou `None` se não houver nenhum. Com n, os n primeiros elementos (como `take`)."),
        ("The last element of a list (or character of a text), as an Option. With n, the last n elements.", "O último elemento de uma lista (ou caractere de um texto), como Option. Com n, os n últimos elementos."),
        ("The list (or text) without its first element.", "A lista (ou o texto) sem o primeiro elemento."),
        ("The element at a position of a list or text, the field of a record, or the value of a key of a dict, as an Option (never an error). Negative positions count from the end.", "O elemento em uma posição de uma lista ou de um texto, o campo de um registro, ou o valor de uma chave de um dicionário, como Option (nunca um erro). As posições negativas contam a partir do fim."),
        ("A new list with x added at the end; or a new set with x added.", "Uma lista nova com x adicionado no fim; ou um conjunto novo com x adicionado."),
        ("A new list where the element at position i is replaced by x. Negative positions count from the end.", "Uma lista nova em que o elemento na posição i é substituído por x. As posições negativas contam a partir do fim."),
        ("A new list with x inserted so that it is at position i (-1 adds it at the end).", "Uma lista nova com x inserido de modo a ficar na posição i (-1 o adiciona no fim)."),
        ("A new list without the element at position i. Negative positions count from the end.", "Uma lista nova sem o elemento na posição i. As posições negativas contam a partir do fim."),
        ("The list or text in reverse order.", "A lista ou o texto em ordem inversa."),
        ("The list sorted from smallest to largest (numbers, texts in alphabetical order...). For the largest first: `sort(list) |> reverse`.", "A lista ordenada do menor para o maior (números, textos em ordem alfabética...). Para o maior primeiro: `sort(list) |> reverse`."),
        ("Sorts the list by the value that the function `key` gives for each element. Elements with the same value keep their order.", "Ordena a lista pelo valor que a função `key` dá para cada elemento. Os elementos com o mesmo valor mantêm a sua ordem."),
        ("The list without duplicates (the first occurrence is kept).", "A lista sem repetições (a primeira ocorrência é mantida)."),
        ("The elements of the list in a random order.", "Os elementos da lista em uma ordem aleatória."),
        ("The part of a list or text from position `start` (included) to `end` (excluded). Negative positions count from the end.", "A parte de uma lista ou de um texto da posição `start` (incluída) até `end` (excluída). As posições negativas contam a partir do fim."),
        ("The first n elements of a list or text.", "Os n primeiros elementos de uma lista ou de um texto."),
        ("The list or text without its first n elements.", "A lista ou o texto sem os seus n primeiros elementos."),
        ("The first elements of a list (or characters of a text), as long as f is true for them.", "Os primeiros elementos de uma lista (ou caracteres de um texto), enquanto f for verdadeira para eles."),
        ("The list (or text) without its first elements for which f is true.", "A lista (ou o texto) sem os seus primeiros elementos para os quais f é verdadeira."),
        ("One element out of n of a list (or text), starting with the first. On a range: the numbers with a step.", "Um elemento a cada n de uma lista (ou de um texto), começando pelo primeiro. Em um intervalo: os números com um passo."),
        ("Cuts a list (or text) into pieces of n elements; the last piece may be shorter.", "Corta uma lista (ou um texto) em pedaços de n elementos; o último pedaço pode ser mais curto."),
        ("Every group of n neighbouring elements of a list (or text), in order: handy to compare each element with the next one.", "Cada grupo de n elementos vizinhos de uma lista (ou de um texto), em ordem: útil para comparar cada elemento com o seguinte."),
        ("Pairs the elements of two lists: [[a1, b1], [a2, b2], ...]. It stops at the end of the shorter list.", "Junta em pares os elementos de duas listas: [[a1, b1], [a2, b2], ...]. Para no fim da lista mais curta."),
        ("Pairs each element with its position: [[0, x0], [1, x1], ...].", "Junta cada elemento com a sua posição: [[0, x0], [1, x1], ...]."),
        ("Is f true for at least one element?", "f é verdadeira para pelo menos um elemento?"),
        ("Is f true for every element?", "f é verdadeira para todos os elementos?"),
        ("The first element for which f is true, as an Option.", "O primeiro elemento para o qual f é verdadeira, como Option."),
        ("How many elements make f true; without f, how many elements there are.", "Quantos elementos tornam f verdadeira; sem f, quantos elementos existem."),
        ("The elements for which f is false: the opposite of `filter`. Also works on a set and on a dict, like `filter`.", "Os elementos para os quais f é falsa: o contrário de `filter`. Também funciona com um conjunto e com um dicionário, como `filter`."),
        ("Calls f(element, position) for each element, in order (positions start at 0). For effects, like `print`.", "Chama f(element, position) para cada elemento, em ordem (as posições começam em 0). Para efeitos, como `print`."),
        ("The position of an element in a list (or of a piece of text in a text), as an Option.", "A posição de um elemento em uma lista (ou de um pedaço de texto em um texto), como Option."),
        ("The element for which the function `key` gives the smallest value (the first one if there are several).", "O elemento para o qual a função `key` dá o menor valor (o primeiro, se houver vários)."),
        ("The element for which the function `key` gives the largest value (the first one if there are several).", "O elemento para o qual a função `key` dá o maior valor (o primeiro, se houver vários)."),
        ("Joins a list of lists into one list.", "Junta uma lista de listas em uma só lista."),
        ("Like map, when f returns a list: the lists are joined.", "Como map, quando f devolve uma lista: as listas são juntadas."),
        ("Splits a list in two: [elements where f is true, elements where f is false].", "Divide uma lista em duas: [elementos em que f é verdadeira, elementos em que f é falsa]."),
        ("Groups the elements in a record: the key is the text of what `key` returns, the value the list of matching elements.", "Agrupa os elementos em um registro: a chave é o texto do que `key` devolve, o valor é a lista dos elementos correspondentes."),
        ("Is the list, text, record, dict or set empty?", "A lista, o texto, o registro, o dicionário ou o conjunto está vazio?"),
        ("The list of the field names of a record, or of the keys of a dict (in order).", "A lista dos nomes dos campos de um registro, ou das chaves de um dicionário (em ordem)."),
        ("The list of the values of a record or of a dict (in the order of its keys), or of the elements of a set (in order).", "A lista dos valores de um registro ou de um dicionário (na ordem das suas chaves), ou dos elementos de um conjunto (em ordem)."),
        ("The list of the [name, value] pairs of a record, or of the [key, value] pairs of a dict.", "A lista dos pares [nome, valor] de um registro, ou dos pares [chave, valor] de um dicionário."),
        ("Builds a record from a list of [name, value] pairs: the opposite of `entries`.", "Cria um registro a partir de uma lista de pares [nome, valor]: o contrário de `entries`."),
        ("Does the record have this field? Does the dict have this key, or the set this element?", "O registro tem este campo? O dicionário tem esta chave, ou o conjunto este elemento?"),
        ("A new record with the field set to the value (added or replaced); or a new dict with the key set to the value.", "Um registro novo com o campo definido com o valor (adicionado ou substituído); ou um dicionário novo com a chave definida com o valor."),
        ("A new record without this field; or a new dict without this key, or a new set without this element.", "Um registro novo sem este campo; ou um dicionário novo sem esta chave, ou um conjunto novo sem este elemento."),
        ("A new record (or dict) where f is applied to every value; the names (or keys) stay the same.", "Um registro (ou dicionário) novo em que f é aplicada a cada valor; os nomes (ou chaves) continuam os mesmos."),
        ("A dictionary: keys of any kind (but functions), each with a value, kept in the order of the keys. Made from a list of [key, value] pairs (the last value of a key wins), from a record, or empty with `dict()`. Use it with get, put, has, remove, keys, values, entries, len, map_values, filter, union...", "Um dicionário: chaves de qualquer tipo (menos funções), cada uma com um valor, mantidas na ordem das chaves. Criado a partir de uma lista de pares [chave, valor] (o último valor de uma chave ganha), de um registro, ou vazio com `dict()`. Use-o com get, put, has, remove, keys, values, entries, len, map_values, filter, union..."),
        ("A set: values (but functions) without repetition, kept in order. Made from a list, or empty with `set()`. Use it with has, push, remove, len, values, filter, map, union, intersection, difference, is_subset.", "Um conjunto: valores (menos funções) sem repetição, mantidos em ordem. Criado a partir de uma lista, ou vazio com `set()`. Use-o com has, push, remove, len, values, filter, map, union, intersection, difference, is_subset."),
        ("The elements of two sets together; or the pairs of two dicts, where the value of b wins for a key in both.", "Os elementos de dois conjuntos juntos; ou os pares de dois dicionários, em que o valor de b ganha para uma chave presente nos dois."),
        ("The elements of the set a that are also in b; or the pairs of the dict a whose key is in b.", "Os elementos do conjunto a que também estão em b; ou os pares do dicionário a cuja chave está em b."),
        ("The elements of the set a that are not in b; or the pairs of the dict a whose key is not in b.", "Os elementos do conjunto a que não estão em b; ou os pares do dicionário a cuja chave não está em b."),
        ("Is every element of the set a also in the set b?", "Todo elemento do conjunto a também está no conjunto b?"),
        ("The value inside `Some(x)` or `Ok(x)`. Stops the program with an error for `None` or `Err(e)`.", "O valor dentro de `Some(x)` ou `Ok(x)`. Para o programa com um erro para `None` ou `Err(e)`."),
        ("The value inside `Some(x)` or `Ok(x)`, or the default value for `None` or `Err(e)`.", "O valor dentro de `Some(x)` ou `Ok(x)`, ou o valor padrão para `None` ou `Err(e)`."),
        ("Is it `Some(...)`?", "É `Some(...)`?"),
        ("Is it `None`?", "É `None`?"),
        ("Is it `Ok(...)`?", "É `Ok(...)`?"),
        ("Is it `Err(...)`?", "É `Err(...)`?"),
        ("Chains steps that may fail: for `Some(x)` or `Ok(x)`, gives f(x), which must be an Option or a Result; `None` and `Err(e)` stay as they are.", "Encadeia etapas que podem falhar: para `Some(x)` ou `Ok(x)`, dá f(x), que deve ser uma Option ou um Result; `None` e `Err(e)` ficam como estão."),
        ("Turns an Option into a Result: `Some(x)` gives `Ok(x)`, and `None` gives `Err(error)`. Handy before `?`.", "Transforma uma Option em um Result: `Some(x)` dá `Ok(x)`, e `None` dá `Err(error)`. Útil antes de `?`."),
        ("For `Err(e)`, gives `Err(f(e))`; `Ok(x)` stays as it is. Handy to explain an error better.", "Para `Err(e)`, dá `Err(f(e))`; `Ok(x)` fica como está. Útil para explicar melhor um erro."),
        ("Reads a whole text file: `Ok(text)`, or `Err(message)` if it cannot be read.", "Lê um arquivo de texto inteiro: `Ok(text)`, ou `Err(message)` se ele não puder ser lido."),
        ("Writes a text in a file (replacing its content): `Ok(())` or `Err(message)`.", "Escreve um texto em um arquivo (substituindo o conteúdo): `Ok(())` ou `Err(message)`."),
        ("Adds a text at the end of a file, creating the file if needed: `Ok(())` or `Err(message)`.", "Adiciona um texto no fim de um arquivo, criando o arquivo se necessário: `Ok(())` ou `Err(message)`."),
        ("Does this file (or folder) exist? Always `false` in the browser.", "Este arquivo (ou pasta) existe? Sempre `false` no navegador."),
        ("What is known about a file or a folder: `Ok({is_folder, size})`, with its size in bytes (0 for a folder), or `Err(message)` if it does not exist or cannot be read.", "O que se sabe sobre um arquivo ou uma pasta: `Ok({is_folder, size})`, com o tamanho em bytes (0 para uma pasta), ou `Err(message)` se ele não existir ou não puder ser lido."),
        ("The names of the files and folders inside a folder, in alphabetical order: `Ok(list of texts)`, or `Err(message)`. `list_folder(\".\")` lists the current folder.", "Os nomes dos arquivos e pastas dentro de uma pasta, em ordem alfabética: `Ok(list of texts)`, ou `Err(message)`. `list_folder(\".\")` lista a pasta atual."),
        ("Creates a folder, and the folders around it that are missing: `Ok(())`, also when it already exists, or `Err(message)`.", "Cria uma pasta, e as pastas em volta dela que estiverem faltando: `Ok(())`, também quando ela já existe, ou `Err(message)`."),
        ("Deletes one file (never a folder: that is an error): `Ok(())` or `Err(message)`. A deleted file cannot be brought back.", "Apaga um arquivo (nunca uma pasta: isso é um erro): `Ok(())` ou `Err(message)`. Um arquivo apagado não pode ser recuperado."),
        ("Joins the parts of a path with `/`, which works on Linux, macOS and Windows (with `\\` if the parts already use only `\\`). Only the first part can start at a root (`/`, `C:`): a part like \"/etc\" after another one is an error.", "Junta as partes de um caminho com `/`, que funciona no Linux, no macOS e no Windows (com `\\` se as partes já usam só `\\`). Só a primeira parte pode começar em uma raiz (`/`, `C:`): uma parte como \"/etc\" depois de outra é um erro."),
        ("The parts of a path, split at each `/` or `\\`; the first part of an absolute path is its root (`/`, `C:\\`...). `join_path` puts them back together.", "As partes de um caminho, separadas em cada `/` ou `\\`; a primeira parte de um caminho absoluto é a raiz dele (`/`, `C:\\`...). `join_path` as junta de novo."),
        ("The date and time now, where the program runs: a record {year, month, day, hour, minute, second, offset}, where `offset` is the number of minutes the local time is ahead of UTC (0 when the system does not tell).", "A data e a hora de agora, onde o programa roda: um registro {year, month, day, hour, minute, second, offset}, em que `offset` é o número de minutos que a hora local está à frente do UTC (0 quando o sistema não informa)."),
        ("The date of today, where the program runs: a record {year, month, day}.", "A data de hoje, onde o programa roda: um registro {year, month, day}."),
        ("Reads a date written as ISO 8601 (\"2024-02-29\", \"2024-02-29T13:05:00\", with `Z` or an offset like `+01:00` if needed): `Ok(date)`, or `Err(message)` if it is not a valid date.", "Lê uma data escrita em ISO 8601 (\"2024-02-29\", \"2024-02-29T13:05:00\", com `Z` ou um deslocamento como `+01:00` se necessário): `Ok(date)`, ou `Err(message)` se não for uma data válida."),
        ("Writes a date as ISO 8601: \"2024-02-29\", with the time if the date has one, and `Z` or the offset if it has one.", "Escreve uma data em ISO 8601: \"2024-02-29\", com a hora se a data tiver uma, e `Z` ou o deslocamento se tiver um."),
        ("Is the value a valid date: a record {year, month, day} (with `hour`, `minute`, `second`, `offset` if needed) for a day that exists?", "O valor é uma data válida: um registro {year, month, day} (com `hour`, `minute`, `second`, `offset` se necessário) de um dia que existe?"),
        ("The date n days later (or earlier, when n is negative), at the same time of day.", "A data n dias depois (ou antes, quando n é negativo), na mesma hora do dia."),
        ("The date and time n seconds later (or earlier, when n is negative). Add `60 * minutes` or `3600 * hours` the same way.", "A data e a hora n segundos depois (ou antes, quando n é negativo). Some `60 * minutes` ou `3600 * hours` do mesmo jeito."),
        ("The number of whole days from date a to date b: negative when b is before a.", "O número de dias inteiros da data a até a data b: negativo quando b vem antes de a."),
        ("The number of seconds from date a to date b, taking their offsets into account: negative when b is before a.", "O número de segundos da data a até a data b, levando em conta os deslocamentos: negativo quando b vem antes de a."),
        ("The day of the week of a date, from 1 (Monday) to 7 (Sunday), as in ISO 8601.", "O dia da semana de uma data, de 1 (segunda-feira) a 7 (domingo), como na ISO 8601."),
        ("The number of days of a month (from 1 to 12) of a year: 28 to 31.", "O número de dias de um mês (de 1 a 12) de um ano: de 28 a 31."),
        ("Does the year have a 29 February? Every 4 years, except every 100 years, except every 400 years.", "O ano tem um 29 de fevereiro? A cada 4 anos, exceto a cada 100 anos, exceto a cada 400 anos."),
        ("Checks that a property holds for many values made by a generator (100, or `cases`): the function is called with each value and must give `true` (or `()`, `Some(...)`, `Ok(...)`). When a value makes it false or stops it with an error, that value is simplified as much as possible (shrinking) and shown, with the seed that made it: the same values come back on every run. The property may also come last, as a block: `check_all(ints()) { |x| x + 0 == x }`.", "Verifica que uma propriedade vale para muitos valores criados por um gerador (100, ou `cases`): a função é chamada com cada valor e deve dar `true` (ou `()`, `Some(...)`, `Ok(...)`). Quando um valor a torna falsa ou a para com um erro, esse valor é simplificado o máximo possível (shrinking) e mostrado, com a semente que o gerou: os mesmos valores voltam a cada execução. A propriedade também pode vir por último, como um bloco: `check_all(ints()) { |x| x + 0 == x }`."),
        ("A list of `n` values (10 if not given) made by a generator, to see what `check_all` will try.", "Uma lista de `n` valores (10 se não for dado) criados por um gerador, para ver o que `check_all` vai testar."),
        ("A generator of whole numbers from `min` to `max` (from -1000 to 1000 if not given), often near 0 and at the edges. A failing number is simplified towards 0.", "Um gerador de números inteiros de `min` a `max` (de -1000 a 1000 se não forem dados), muitas vezes perto de 0 e nas bordas. Um número que falha é simplificado em direção a 0."),
        ("A generator of decimal numbers from `min` to `max` (from -1000.0 to 1000.0 if not given).", "Um gerador de números decimais de `min` a `max` (de -1000.0 a 1000.0 se não forem dados)."),
        ("A generator of texts of at most `max_length` characters (20 if not given): letters mostly, with digits, spaces, punctuation and accents. A failing text is made shorter and simpler.", "Um gerador de textos de no máximo `max_length` caracteres (20 se não for dado): principalmente letras, com algarismos, espaços, pontuação e acentos. Um texto que falha é deixado mais curto e mais simples."),
        ("A generator of `true` and `false`.", "Um gerador de `true` e `false`."),
        ("A generator of lists of at most `max_length` elements (10 if not given), each made by `generator`. A failing list is made shorter, then its elements simpler.", "Um gerador de listas de no máximo `max_length` elementos (10 se não for dado), cada um criado por `generator`. Uma lista que falha é deixada mais curta, e depois os elementos dela mais simples."),
        ("A generator that picks one of the values of a list. A failing value is replaced by one written before it.", "Um gerador que escolhe um dos valores de uma lista. Um valor que falha é substituído por um escrito antes dele."),
        ("A generator of records, with a generator for each field: `records_of({name: texts(), age: ints(0, 120)})`. Handy to give several values to `check_all`.", "Um gerador de registros, com um gerador para cada campo: `records_of({name: texts(), age: ints(0, 120)})`. Útil para dar vários valores a `check_all`."),
        ("January", "janeiro"),
        ("February", "fevereiro"),
        ("March", "março"),
        ("April", "abril"),
        ("May", "maio"),
        ("June", "junho"),
        ("July", "julho"),
        ("August", "agosto"),
        ("September", "setembro"),
        ("October", "outubro"),
        ("November", "novembro"),
        ("December", "dezembro"),
    ],
}
