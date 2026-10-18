// Spanish: each English text of Rythme with its translation. How to add or
// update a language: CONTRIBUTING.md, "Messages in every language".
Language {
    code: "es",
    name: "español",
    texts: &[
        // src/main.rs
        ("unknown language `{v}`", "idioma desconocido `{v}`"),
        (
            r#"Rythme ran into an internal problem. This is a bug in Rythme, not in your program.
Please report it, with your program, at https://github.com/maxlestage/Rythme/issues"#,
            r#"Rythme se encontró con un problema interno. Es un fallo de Rythme, no de tu programa.
Por favor, avísanos, con tu programa, en https://github.com/maxlestage/Rythme/issues"#,
        ),
        ("`--lang` needs a language", "`--lang` necesita un idioma"),
        ("{what}: write `--lang` and one of {codes}, like `rythme --lang fr hello.ry`", "{what}: escribe `--lang` y uno de {codes}, como `rythme --lang es hola.ry`"),
        ("usage: rythme explain CODE   (codes: {})", "uso: rythme explain CÓDIGO   (códigos: {})"),
        ("unknown code `{code}`. Known codes: {}", "código desconocido `{code}`. Códigos conocidos: {}"),
        ("which file? Example: rythme run hello.ry", "¿qué archivo? Ejemplo: rythme run hola.ry"),
        ("which file? Example: rythme check hello.ry", "¿qué archivo? Ejemplo: rythme check hola.ry"),
        ("which code? Example: rythme -e 'print(1 + 2)'", "¿qué código? Ejemplo: rythme -e 'print(1 + 2)'"),
        ("unknown command or file `{cmd}`", "comando o archivo desconocido `{cmd}`"),
        (
            r#"
did you mean `rythme {best}`?"#,
            r#"
¿quisiste decir `rythme {best}`?"#,
        ),
        (
            r#"
see `rythme help`"#,
            r#"
consulta `rythme help`"#,
        ),
        ("current folder: {}", "carpeta actual: {}"),
        ("Rythme files usually end with `.ry` ({})", "los archivos de Rythme suelen terminar en `.ry` ({})"),
        ("✓ {shown}: no problems found", "✓ {shown}: no se encontró ningún problema"),
        ("✓ {shown}: no errors, {n} warning(s)", "✓ {shown}: sin errores, {n} advertencia(s)"),
        ("✗ {shown}: {n} error(s)", "✗ {shown}: {n} error(es)"),
        // src/bigint.rs
        ("this whole number would be too large (more than {MAX_DIGITS} digits)", "este número entero sería demasiado grande (más de {MAX_DIGITS} cifras)"),
        ("does it grow without end, in a function that calls itself? For an approximate value, use a decimal number (Float), like `2.0 ** 1000`", "¿crece sin fin, en una función que se llama a sí misma? Para un valor aproximado, usa un número decimal (Float), como `2.0 ** 1000`"),
        // src/builtins/date.rs
        ("the year must go from 1 to 9999, not {year}", "el año debe ir de 1 a 9999, no {year}"),
        ("the month must go from 1 to 12, not {month}", "el mes debe ir de 1 a 12, no {month}"),
        ("the day must go from 1 to {len} in {name} {year}, not {day}", "el día debe ir de 1 a {len} en {name} de {year}, no {day}"),
        (" ({year} is not a leap year)", " ({year} no es un año bisiesto)"),
        ("the hour must go from 0 to 23, not {hour}", "la hora debe ir de 0 a 23, no {hour}"),
        ("the minute must go from 0 to 59, not {minute}", "el minuto debe ir de 0 a 59, no {minute}"),
        ("the second must go from 0 to 59, not {second}", "el segundo debe ir de 0 a 59, no {second}"),
        ("the offset must go from -1080 to 1080 minutes (-18:00 to +18:00), not {offset}", "el desfase debe ir de -1080 a 1080 minutos (-18:00 a +18:00), no {offset}"),
        ("a date has no field `{k}`", "una fecha no tiene el campo `{k}`"),
        (" (did you mean `{best}`?)", " (¿quisiste decir `{best}`?)"),
        ("the field `{k}` is too large for a date", "el campo `{k}` es demasiado grande para una fecha"),
        ("the field `{k}` must be a whole number, not {}", "el campo `{k}` debe ser un número entero, no {}"),
        ("the field `{k}` is missing", "falta el campo `{k}`"),
        ("\"{text}\" is not a date written as ISO 8601: write it like \"2024-02-29\" or \"2024-02-29T13:05:00\"", "\"{text}\" no es una fecha escrita en ISO 8601: escríbela como \"2024-02-29\" o \"2024-02-29T13:05:00\""),
        ("\"{text}\" is not a valid date: {why}", "\"{text}\" no es una fecha válida: {why}"),
        ("`{f}` needs a valid date: {why}", "`{f}` necesita una fecha válida: {why}"),
        ("not a valid date", "no es una fecha válida"),
        ("a date is a record like `{{year: 2024, month: 2, day: 29}}`, with `hour`, `minute`, `second` and `offset` (minutes ahead of UTC) if needed", "una fecha es un registro como `{{year: 2024, month: 2, day: 29}}`, con `hour`, `minute`, `second` y `offset` (minutos de adelanto sobre UTC) si hace falta"),
        ("`{f}` gives a date outside of the years 1 to 9999", "`{f}` da una fecha fuera de los años 1 a 9999"),
        ("the month must go from 1 to 12, not {m}", "el mes debe ir de 1 a 12, no {m}"),
        ("the clock of the system gives a date outside of the years 1 to 9999", "el reloj del sistema da una fecha fuera de los años 1 a 9999"),
        // src/builtins/dicts.rs
        ("`{f}`: {} cannot be a key of a dict, nor an element of a set", "`{f}`: {} no puede ser una clave de diccionario, ni un elemento de conjunto"),
        ("it holds {}", "contiene {}"),
        ("keys and elements must be values that can be compared: anything but functions and `nan`", "las claves y los elementos deben ser valores comparables: todo menos las funciones y `nan`"),
        ("`{f}` needs a list of [key, value] pairs, but it contains {}", "`{f}` necesita una lista de pares [clave, valor], pero contiene {}"),
        ("each pair is a list of two elements, like `dict([[\"Ada\", 36], [\"Alan\", 41]])`", "cada par es una lista de dos elementos, como `dict([[\"Ada\", 36], [\"Alan\", 41]])`"),
        // src/builtins/property.rs
        ("this is not a generator: {}", "esto no es un generador: {}"),
        ("not a generator", "no es un generador"),
        ("make one with `ints()`, `floats()`, `texts()`, `bools()`, `lists_of(g)`, `one_of(list)` or `records_of({{x: g}})`", "crea uno con `ints()`, `floats()`, `texts()`, `bools()`, `lists_of(g)`, `one_of(lista)` o `records_of({{x: g}})`"),
        ("generators can be put inside each other {MAX_DEPTH} times at most", "los generadores se pueden anidar {MAX_DEPTH} veces como máximo"),
        ("its field `{what}` is missing or wrong", "su campo `{what}` falta o es incorrecto"),
        ("`{f}` takes no argument, or two: the smallest and the largest value, like `{f}(0, 10)`", "`{f}` no recibe ningún argumento, o dos: el valor más pequeño y el más grande, como `{f}(0, 10)`"),
        ("{f}({}, {}): the minimum is larger than the maximum", "{f}({}, {}): el mínimo es mayor que el máximo"),
        ("`floats` needs finite numbers, not {} and {}", "`floats` necesita números finitos, no {} y {}"),
        ("`{f}` needs a length from 0 to {MAX_LENGTH}, not {n}", "`{f}` necesita una longitud de 0 a {MAX_LENGTH}, no {n}"),
        ("`one_of` needs at least one value to choose from", "`one_of` necesita al menos un valor entre el que elegir"),
        ("the field `{k}` given to `records_of` must be a generator, like `ints()`, not {}", "el campo `{k}` dado a `records_of` debe ser un generador, como `ints()`, no {}"),
        ("`check_all` tries from 1 to {MAX_CASES} values, not {cases}", "`check_all` prueba de 1 a {MAX_CASES} valores, no {cases}"),
        ("the property is false for {v}", "la propiedad es falsa para {v}"),
        ("false for {v}", "falsa para {v}"),
        ("the property gives {r} for {v}", "la propiedad da {r} para {v}"),
        ("{r} for {v}", "{r} para {v}"),
        ("the property fails for {v}: {}", "la propiedad falla para {v}: {}"),
        ("checked here", "comprobada aquí"),
        ("found on try {case} of {cases}", "encontrado en el intento {case} de {cases}"),
        ("found on try {case} of {cases}, then simplified from {} in {steps} step(s)", "encontrado en el intento {case} de {cases}, y luego simplificado desde {} en {steps} paso(s)"),
        ("seed {seed}: every run tries the same values; `rythme test --seed N` tries others", "semilla {seed}: cada ejecución prueba los mismos valores; `rythme test --seed N` prueba otros"),
        ("the function must hold for every value of the generator: fix the function, or give the generator a smaller range, like `ints(1, 100)`", "la función debe cumplirse para cada valor del generador: corrige la función, o da al generador un intervalo más pequeño, como `ints(1, 100)`"),
        // src/builtins/regex.rs
        ("for the sign `{c}` itself, write `[{c}]` (in a Rythme text: \"[\\{c}]\")", "para el signo `{c}` en sí, escribe `[{c}]` (en un texto de Rythme: \"[\\{c}]\")"),
        ("for the sign `{c}` itself, write `[{c}]`", "para el signo `{c}` en sí, escribe `[{c}]`"),
        ("`{q}` has nothing to repeat", "`{q}` no tiene nada que repetir"),
        ("nothing before it to repeat", "nada que repetir antes"),
        ("put it after a character or a group, like `a{q}` or `(ab){q}`; {}", "ponlo después de un carácter o de un grupo, como `a{q}` o `(ab){q}`; {}"),
        ("`{{` must start a count, like {{3}} or {{2,5}}", "`{{` debe empezar un número de veces, como {{3}} o {{2,5}}"),
        ("not a count", "no es un número de veces"),
        ("{}; in a Rythme text, braces are written `\\{{` and `\\}}`, like \"[0-9]\\{{4\\}}\"", "{}; en un texto de Rythme, las llaves se escriben `\\{{` y `\\}}`, como \"[0-9]\\{{4\\}}\""),
        ("a count can be at most {MAX_COUNT}", "un número de veces puede valer como máximo {MAX_COUNT}"),
        ("too large", "demasiado grande"),
        ("for more, use `*` or `+`, or {{n,}} without a maximum", "para más, usa `*` o `+`, o {{n,}} sin máximo"),
        ("the count goes backwards: {min} is more than {max}", "el número de veces va al revés: {min} es mayor que {max}"),
        ("smallest first", "el más pequeño primero"),
        ("write the smallest number first: {{{max},{min}}}", "escribe primero el número más pequeño: {{{max},{min}}}"),
        ("a repetition cannot be repeated again", "una repetición no se puede volver a repetir"),
        ("repeats a repetition", "repite una repetición"),
        ("put the repeated part in parentheses, like `(a+)*`, or keep one sign, like `a*`; {}", "pon la parte repetida entre paréntesis, como `(a+)*`, o deja un solo signo, como `a*`; {}"),
        ("the groups are nested too deeply (more than {MAX_DEPTH})", "los grupos están anidados demasiado (más de {MAX_DEPTH})"),
        ("too deep", "demasiado profundo"),
        ("`(?<` must be followed by a name and `>`", "`(?<` debe ir seguido de un nombre y de `>`"),
        ("not a group name", "no es un nombre de grupo"),
        ("a named group looks like `(?<year>[0-9]+)`: the name has letters, digits and `_`", "un grupo con nombre se escribe así: `(?<anio>[0-9]+)`: el nombre tiene letras, cifras y `_`"),
        ("two groups are named `{name}`", "dos grupos se llaman `{name}`"),
        ("already used", "ya usado"),
        ("give each group its own name", "da a cada grupo su propio nombre"),
        ("`(?i)` must be at the very start of the regex", "`(?i)` debe estar justo al principio de la regex"),
        ("not at the start", "no está al principio"),
        ("`(?i)` ignores the case in the whole regex: write it first", "`(?i)` ignora mayúsculas y minúsculas en toda la regex: escríbelo primero"),
        ("unknown kind of group", "tipo de grupo desconocido"),
        ("unknown", "desconocido"),
        ("the groups are `(...)`, `(?:...)` (a group that captures nothing) and `(?<name>...)` (a named group); `(?i)` at the start ignores the case", "los grupos son `(...)`, `(?:...)` (un grupo que no captura nada) y `(?<nombre>...)` (un grupo con nombre); `(?i)` al principio ignora mayúsculas y minúsculas"),
        ("this `(` is never closed", "este `(` nunca se cierra"),
        ("opened here", "abierto aquí"),
        ("add a `)` where the group ends; {}", "añade un `)` donde termina el grupo; {}"),
        ("the regex ends with a lone `\\`", "la regex termina con un `\\` solo"),
        ("followed by nothing", "seguido de nada"),
        ("a `\\` comes before a letter like d, w, s or b, or before a sign for the sign itself, like `\\.`", "un `\\` va delante de una letra como d, w, s o b, o delante de un signo para el signo en sí, como `\\.`"),
        ("`\\{c}` means nothing inside `[...]`", "`\\{c}` no significa nada dentro de `[...]`"),
        ("not a character", "no es un carácter"),
        ("`\\b` is the edge of a word: write it outside of the brackets", "`\\b` es el borde de una palabra: escríbelo fuera de los corchetes"),
        ("`\\{c}` means nothing in a regex", "`\\{c}` no significa nada en una regex"),
        (
            r#"the known ones: \d (a digit), \w (a letter, a digit or _), \s (a space), \b (the edge of a word),
their opposites \D, \W, \S, \B, \n, \t, and `\` before a sign for the sign itself, like \. or \("#,
            r#"los que existen: \d (una cifra), \w (una letra, una cifra o _), \s (un espacio), \b (el borde de una palabra),
sus contrarios \D, \W, \S, \B, \n, \t, y `\` delante de un signo para el signo en sí, como \. o \("#,
        ),
        ("this `[` is never closed", "este `[` nunca se cierra"),
        ("add a `]` after the characters; for the sign `]` itself, put it first: `[]]`", "añade un `]` después de los caracteres; para el signo `]` en sí, ponlo primero: `[]]`"),
        ("a range must end with one character", "un intervalo debe terminar con un carácter"),
        ("not a range", "no es un intervalo"),
        ("for the sign `-` itself, put it last: `[a-]`", "para el signo `-` en sí, ponlo al final: `[a-]`"),
        ("the range `{lo}-{hi}` goes backwards", "el intervalo `{lo}-{hi}` va al revés"),
        ("backwards", "al revés"),
        ("write the first character first: `{hi}-{lo}`", "escribe primero el primer carácter: `{hi}-{lo}`"),
        ("this `)` closes nothing", "este `)` no cierra nada"),
        ("no `(` before it", "no hay `(` antes"),
        ("the regex is too large once its repetitions are written out", "la regex es demasiado grande una vez escritas sus repeticiones"),
        ("use smaller counts in {{n,m}}, or `*` and `+`, which cost nothing", "usa números más pequeños en {{n,m}}, o `*` y `+`, que no cuestan nada"),
        ("`$1` is the text of group 1, `$<name>` of a named group, `$0` the whole match; for the sign `$` itself, write `$$`", "`$1` es el texto del grupo 1, `$<nombre>` el de un grupo con nombre, `$0` todo el fragmento encontrado; para el signo `$` en sí, escribe `$$`"),
        ("the regex has no group", "la regex no tiene ningún grupo"),
        ("the regex has 1 group", "la regex tiene 1 grupo"),
        ("the regex has {g} groups", "la regex tiene {g} grupos"),
        ("there is no group {n}: {has}", "no hay grupo {n}: {has}"),
        ("no such group", "no existe ese grupo"),
        ("`$<` must be followed by a group name and `>`", "`$<` debe ir seguido de un nombre de grupo y de `>`"),
        ("never closed", "nunca se cierra"),
        ("the regex has no named group", "la regex no tiene ningún grupo con nombre"),
        ("its named groups are {}", "sus grupos con nombre son {}"),
        ("the regex has no group named `{name}`: {has}", "la regex no tiene ningún grupo llamado `{name}`: {has}"),
        ("unknown group", "grupo desconocido"),
        ("`$` must be followed by a group number, `<name>` or `$`", "`$` debe ir seguido de un número de grupo, de `<nombre>` o de `$`"),
        ("a lone `$`", "un `$` solo"),
        ("invalid replacement: {}", "reemplazo no válido: {}"),
        ("invalid regex: {}", "regex no válida: {}"),
        ("with this replacement", "con este reemplazo"),
        ("with this regex", "con esta regex"),
        ("here in the replacement:", "aquí en el reemplazo:"),
        ("here in the regex:", "aquí en la regex:"),
        ("`named_captures` needs a regex with named groups", "`named_captures` necesita una regex con grupos con nombre"),
        ("name a group like this: `(?<year>[0-9]+)`; for groups without names, use `captures`", "da nombre a un grupo así: `(?<anio>[0-9]+)`; para grupos sin nombre, usa `captures`"),
        // src/builtins/system.rs
        ("`{f}` is not available in the browser: a program here has no input to read. Run it on your computer: rythme file.ry < lines.txt", "`{f}` no está disponible en el navegador: aquí un programa no tiene entrada que leer. Ejecútalo en tu ordenador: rythme archivo.ry < lineas.txt"),
        ("`{f}` is not available in the browser: a program here has no files or folders. Run it on your computer: rythme file.ry", "`{f}` no está disponible en el navegador: aquí un programa no tiene archivos ni carpetas. Ejecútalo en tu ordenador: rythme archivo.ry"),
        ("the path is empty: write \".\" for the current folder", "la ruta está vacía: escribe \".\" para la carpeta actual"),
        ("folder not found: {path}", "carpeta no encontrada: {path}"),
        ("file not found: {path}", "archivo no encontrado: {path}"),
        ("permission denied: {path}", "permiso denegado: {path}"),
        ("cannot access {path}: {e}", "no se puede acceder a {path}: {e}"),
        ("{path} is a file, not a folder: `read_file` reads a file", "{path} es un archivo, no una carpeta: `read_file` lee un archivo"),
        ("{path} already exists, and it is a file, not a folder", "{path} ya existe, y es un archivo, no una carpeta"),
        ("{path} is a folder: `delete_file` deletes only files, never folders", "{path} es una carpeta: `delete_file` solo borra archivos, nunca carpetas"),
        ("cannot read the standard input: {e}", "no se puede leer la entrada estándar: {e}"),
        ("`join_path` needs a list of texts, but it contains {}", "`join_path` necesita una lista de textos, pero contiene {}"),
        ("`text(value)` turns a value into a text", "`text(valor)` convierte un valor en texto"),
        ("`join_path` cannot add \"{part}\" after \"{out}\": only the first part of a path can start at a root (`/`, `\\` or a drive like `C:`)", "`join_path` no puede añadir \"{part}\" después de \"{out}\": solo la primera parte de una ruta puede empezar en una raíz (`/`, `\\` o una unidad como `C:`)"),
        ("remove the separator at the start of \"{part}\", or put it first in the list", "quita el separador al principio de \"{part}\", o ponlo primero en la lista"),
        ("`{w}` is not a valid option: its name starts with a letter, followed by letters, digits, `-` or `_` (write `--` before words that start with `-`)", "`{w}` no es una opción válida: su nombre empieza por una letra, seguida de letras, cifras, `-` o `_` (escribe `--` antes de las palabras que empiezan por `-`)"),
        ("`{w}` is not an option that Rythme understands: write `--{name}` for a long name, or `-a -b` for several one-letter options", "`{w}` no es una opción que Rythme entienda: escribe `--{name}` para un nombre largo, o `-a -b` para varias opciones de una letra"),
        ("`{w}` cannot be an option: the field `args` holds the words that are not options", "`{w}` no puede ser una opción: el campo `args` contiene las palabras que no son opciones"),
        ("the option `{name}` is given twice", "la opción `{name}` se da dos veces"),
        ("`options` needs a list of texts, like the one of `args()`, but it contains {}", "`options` necesita una lista de textos, como la de `args()`, pero contiene {}"),
        // src/builtins/text_tools.rs
        ("`{f}` writes at most 100 decimals, not {d}", "`{f}` escribe 100 decimales como máximo, no {d}"),
        ("`{f}` does not know the language \"{l}\"", "`{f}` no conoce el idioma \"{l}\""),
        ("the languages: \"en\" (1,234.5), \"fr\" (1 234,5), \"de\", \"es\", \"it\", \"nl\" and \"pt\" (1.234,5)", "los idiomas: \"en\" (1,234.5), \"fr\" (1 234,5), \"de\", \"es\", \"it\", \"nl\" y \"pt\" (1.234,5)"),
        // src/builtins.rs
        ("Basics", "Lo básico"),
        ("Numbers", "Números"),
        ("Text", "Texto"),
        ("Lists", "Listas"),
        ("Records", "Registros"),
        ("Dictionaries and sets", "Diccionarios y conjuntos"),
        ("Option and Result", "Option y Result"),
        ("Files", "Archivos"),
        ("Dates and times", "Fechas y horas"),
        ("Property tests", "Pruebas de propiedades"),
        ("usage: `{}`", "uso: `{}`"),
        ("a list", "una lista"),
        ("a text", "un texto"),
        ("a whole number (Int)", "un número entero (Int)"),
        ("a number", "un número"),
        ("a function", "una función"),
        ("a record", "un registro"),
        ("true or false", "true o false"),
        ("a list or a text", "una lista o un texto"),
        ("a text or a list", "un texto o una lista"),
        ("a text, a list, a record, a dict or a set", "un texto, una lista, un registro, un diccionario o un conjunto"),
        ("a list, a text, a record or a dict", "una lista, un texto, un registro o un diccionario"),
        ("a record or a dict", "un registro o un diccionario"),
        ("a record, a dict or a set", "un registro, un diccionario o un conjunto"),
        ("a list, a dict or a set", "una lista, un diccionario o un conjunto"),
        ("a list or a set", "una lista o un conjunto"),
        ("a dict or a set", "un diccionario o un conjunto"),
        ("a dict", "un diccionario"),
        ("a set", "un conjunto"),
        ("a list of [key, value] pairs (or a record)", "una lista de pares [clave, valor] (o un registro)"),
        ("a list (or several values)", "una lista (o varios valores)"),
        ("a number or a text", "un número o un texto"),
        ("a list, a set, an Option or a Result", "una lista, un conjunto, una Option o un Result"),
        ("an Option or a Result", "una Option o un Result"),
        ("an Option", "una Option"),
        ("a Result", "un Result"),
        ("a date (a record {{year, month, day}})", "una fecha (un registro {{year, month, day}})"),
        ("a generator (like `ints()`)", "un generador (como `ints()`)"),
        ("a text or a function", "un texto o una función"),
        ("`{f}` expects {expected} as argument {pos}, but got {got}", "`{f}` espera {expected} como argumento {pos}, pero recibió {got}"),
        ("wrong kind of value", "tipo de valor incorrecto"),
        ("a text is not a list: `chars(text)` gives the list of its characters", "un texto no es una lista: `chars(texto)` da la lista de sus caracteres"),
        ("a record is not a list: `entries(record)`, `keys(record)` or `values(record)` give one", "un registro no es una lista: `entries(registro)`, `keys(registro)` o `values(registro)` dan una"),
        ("a dict is not a list: `entries(dict)`, `keys(dict)` or `values(dict)` give one", "un diccionario no es una lista: `entries(dic)`, `keys(dic)` o `values(dic)` dan una"),
        ("a set is not a list: `values(set)` gives the list of its elements, in order", "un conjunto no es una lista: `values(conjunto)` da la lista de sus elementos, en orden"),
        ("to change the values of a dict, use `map_values(dict, f)`", "para cambiar los valores de un diccionario, usa `map_values(dic, f)`"),
        ("for a list, use `{cousin}`", "para una lista, usa `{cousin}`"),
        ("to read a date from a text like \"2024-02-29\", use `parse_date(text)`", "para leer una fecha de un texto como \"2024-02-29\", usa `parse_date(texto)`"),
        ("for a list, use `find(list, f)`", "para una lista, usa `find(lista, f)`"),
        ("for an Option or a Result, use `and_then(x, f)`", "para una Option o un Result, usa `and_then(x, f)`"),
        ("the function given to `{f}` must return a list, but it returned {got}", "la función dada a `{f}` debe devolver una lista, pero devolvió {got}"),
        ("the function given to `{f}` must return a text, but it returned {got}", "la función dada a `{f}` debe devolver un texto, pero devolvió {got}"),
        ("to make a text of any value, use `text(x)`", "para hacer un texto con cualquier valor, usa `text(x)`"),
        ("an Option (Some or None)", "una Option (Some o None)"),
        ("a Result (Ok or Err)", "un Result (Ok o Err)"),
        ("the function given to `{f}` must return {expected}, but it returned {got}", "la función dada a `{f}` debe devolver {expected}, pero devolvió {got}"),
        ("to change the value inside without another check, use `map(x, f)`", "para cambiar el valor de dentro sin otra comprobación, usa `map(x, f)`"),
        ("the function given to `{f}` must return true or false, but it returned {got}", "la función dada a `{f}` debe devolver true o false, pero devolvió {got}"),
        ("the number {n} is too large for `{f}`", "el número {n} es demasiado grande para `{f}`"),
        ("{digits} digits", "{digits} cifras"),
        ("`{f}` needs a number of 0 or more, not {n}", "`{f}` necesita un número igual o mayor que 0, no {n}"),
        ("`{f}` needs a number of 1 or more, not 0", "`{f}` necesita un número igual o mayor que 1, no 0"),
        ("the list is empty", "la lista está vacía"),
        ("the list has {len} element(s)", "la lista tiene {len} elemento(s)"),
        ("position {n} is out of range: {what}", "la posición {n} está fuera de rango: {what}"),
        ("no element here", "no hay ningún elemento aquí"),
        ("valid positions go from 0 to {last} (or from -1 backwards, counting from the end)", "las posiciones válidas van de 0 a {last} (o de -1 hacia atrás, contando desde el final)"),
        ("at least {min}", "al menos {min}"),
        ("{min} to {max}", "de {min} a {max}"),
        ("`{f}` gives {given} argument(s) to this function, but it takes {expected}", "`{f}` da {given} argumento(s) a esta función, pero espera {expected}"),
        ("takes {expected} argument(s)", "espera {expected} argumento(s)"),
        ("`nan` (not a number) cannot be converted to a whole number", "`nan` (no es un número) no se puede convertir en un número entero"),
        ("`nan` is the result of a calculation that has none, like `0.0 * inf`: check where this value comes from", "`nan` es el resultado de un cálculo que no tiene resultado, como `0.0 * inf`: comprueba de dónde viene este valor"),
        ("`{}` (infinity) cannot be converted to a whole number", "`{}` (el infinito) no se puede convertir en un número entero"),
        ("an infinity comes from a calculation too large for decimal numbers, like `1e308 * 10.0`, or from a division by `0.0`", "un infinito viene de un cálculo demasiado grande para los números decimales, como `1e308 * 10.0`, o de una división entre `0.0`"),
        ("`text` writes at most 100 decimals, not {digits}", "`text` escribe 100 decimales como máximo, no {digits}"),
        ("`input` found the end of the input", "`input` llegó al final de la entrada"),
        ("no more lines to read", "no quedan líneas que leer"),
        ("the keyboard input was closed (Ctrl-D), or the text sent to the program has ended", "la entrada del teclado se cerró (Ctrl-D), o el texto enviado al programa se terminó"),
        ("assertion failed: {}", "aserción fallida: {}"),
        ("assertion failed", "aserción fallida"),
        ("this condition is false", "esta condición es falsa"),
        (
            r#"got:      {got}
expected: {expected}"#,
            r#"obtenido: {got}
esperado: {expected}"#,
        ),
        ("first difference: character {at}", "primera diferencia: carácter {at}"),
        ("assertion failed: the values are different", "aserción fallida: los valores son distintos"),
        (" at {path}", " en {path}"),
        ("the list{at} has {} element(s), {} expected", "la lista{at} tiene {} elemento(s), se esperaban {}"),
        ("in excess", "de más"),
        ("missing", "que faltan"),
        ("{word} from [{from}]: {}{dots}", "{word} desde [{from}]: {}{dots}"),
        ("missing field(s){at}: {}", "campo(s) que faltan{at}: {}"),
        ("field(s) not expected{at}: {}", "campo(s) inesperado(s){at}: {}"),
        (", which are of different types: {} and {}", ", que son de tipos distintos: {} y {}"),
        ("first difference at {path}: got {}, expected {}{kinds}", "primera diferencia en {path}: se obtuvo {}, se esperaba {}{kinds}"),
        ("the values are of different types: {} and {}", "los valores son de tipos distintos: {} y {}"),
        ("(no line)", "(sin línea)"),
        (
            r#"first different line: line {}
  got:      {}
  expected: {}"#,
            r#"primera línea distinta: línea {}
  obtenido: {}
  esperado: {}"#,
        ),
        ("the program stopped with `fail`", "el programa se detuvo con `fail`"),
        ("the program stopped here", "el programa se detuvo aquí"),
        ("an exit code goes from 0 to 255, not {code}", "un código de salida va de 0 a 255, no {code}"),
        ("the program stopped with exit({code})", "el programa se detuvo con exit({code})"),
        ("cannot pick a random element in an empty list", "no se puede elegir un elemento al azar en una lista vacía"),
        ("random({lo}, {hi}): the minimum is larger than the maximum", "random({lo}, {hi}): el mínimo es mayor que el máximo"),
        ("`{name}` of an empty list", "`{name}` de una lista vacía"),
        ("`{name}` cannot compare {} and {}", "`{name}` no puede comparar {} y {}"),
        ("cannot take the square root of a negative number ({x})", "no se puede sacar la raíz cuadrada de un número negativo ({x})"),
        ("use `sqrt(abs(x))` if you want the root of its absolute value", "usa `sqrt(abs(x))` si quieres la raíz de su valor absoluto"),
        ("cannot convert \"{s}\" to a whole number", "no se puede convertir \"{s}\" en un número entero"),
        ("if the text may not be a number, use `parse_int(t)`: it gives `Ok(n)` or `Err(...)` that you can check", "si el texto puede no ser un número, usa `parse_int(t)`: da `Ok(n)` o `Err(...)`, que puedes comprobar"),
        ("cannot convert \"{s}\" to a decimal number", "no se puede convertir \"{s}\" en un número decimal"),
        ("if the text may not be a number, use `parse_float(t)`, which gives `Ok(x)` or `Err(...)`", "si el texto puede no ser un número, usa `parse_float(t)`, que da `Ok(x)` o `Err(...)`"),
        ("`{name}` needs a number from -1 to 1, not {x}", "`{name}` necesita un número de -1 a 1, no {x}"),
        ("the logarithm only exists for positive numbers, not {x}", "el logaritmo solo existe para los números positivos, no {x}"),
        ("invalid logarithm base: {base}", "base de logaritmo no válida: {base}"),
        ("`{f}` needs a single character, but \"{s}\" has {}", "`{f}` necesita un solo carácter, pero \"{s}\" tiene {}"),
        ("`replace` cannot replace an empty text", "`replace` no puede reemplazar un texto vacío"),
        ("{n} is not the number of a character", "{n} no es el número de un carácter"),
        ("character numbers go from 0 to 1114111, except 55296 to 57343", "los números de carácter van de 0 a 1114111, salvo de 55296 a 57343"),
        ("the repeated text would be too large", "el texto repetido sería demasiado grande"),
        ("the repeated list would be too large", "la lista repetida sería demasiado grande"),
        ("this number has more than {} digits", "este número tiene más de {} cifras"),
        ("\"{s}\" is not a whole number", "\"{s}\" no es un número entero"),
        ("\"{s}\" is not a number", "\"{s}\" no es un número"),
        ("JSON, line {line}, column {column}: {msg}", "JSON, línea {line}, columna {column}: {msg}"),
        ("expected {what}, but found `{c}`", "se esperaba {what}, pero se encontró `{c}`"),
        ("expected {what}, but the text ends", "se esperaba {what}, pero el texto termina"),
        ("a value", "un valor"),
        ("the end of the text", "el final del texto"),
        ("`,` or `]`", "`,` o `]`"),
        ("`,` or `}}`", "`,` o `}}`"),
        ("a name in quotes", "un nombre entre comillas"),
        ("invalid number: JSON writes numbers like 0, -12, 3.5 or 1e-3", "número no válido: JSON escribe los números como 0, -12, 3.5 o 1e-3"),
        ("invalid escape: after `\\` comes one of \" \\ / b f n r t u", "escape no válido: después de `\\` va uno de \" \\ / b f n r t u"),
        ("a line break or a tab inside a text is written with `\\`: `\\n`, `\\t`", "un salto de línea o un tabulador dentro de un texto se escribe con `\\`: `\\n`, `\\t`"),
        ("a text is never closed: `\"` is missing", "un texto nunca se cierra: falta `\"`"),
        ("`\\u` must be followed by 4 hexadecimal digits", "`\\u` debe ir seguido de 4 cifras hexadecimales"),
        ("invalid `\\u` escape: half of a character (a surrogate) without its other half", "escape `\\u` no válido: la mitad de un carácter (un surrogate) sin su otra mitad"),
        ("`to_json` indents by 16 spaces at most, not {n}", "`to_json` sangra 16 espacios como máximo, no {n}"),
        ("the JSON text would be too large", "el texto JSON sería demasiado grande"),
        ("the values are nested too deeply to be indented: write them on one line, with `to_json(value)`", "los valores están anidados demasiado para sangrarlos: escríbelos en una línea, con `to_json(valor)`"),
        ("`to_json` cannot write {} in JSON", "`to_json` no puede escribir {} en JSON"),
        ("JSON has no infinite numbers and no NaN", "JSON no tiene números infinitos ni NaN"),
        (
            r#"a dict is written as a JSON object only when its keys are all texts, or all symbols:
turn the keys into texts first, or write `entries(dict)`"#,
            r#"un diccionario se escribe como un objeto JSON solo cuando sus claves son todas textos, o todas símbolos:
convierte primero las claves en textos, o escribe `entries(dic)`"#,
        ),
        (
            r#"JSON only holds numbers, texts, true and false, lists, records and null (`None`):
turn your value into a record first, like {{kind: "Circle", radius: r}}"#,
            r#"JSON solo contiene números, textos, true y false, listas, registros y null (`None`):
convierte primero tu valor en un registro, como {{tipo: "Circulo", radio: r}}"#,
        ),
        ("`sum` needs a list of numbers, but it contains {}", "`sum` necesita una lista de números, pero contiene {}"),
        ("`product` needs a list of numbers, but it contains {}", "`product` necesita una lista de números, pero contiene {}"),
        ("all the elements must be of the same kind: all numbers, or all texts...", "todos los elementos deben ser del mismo tipo: todos números, o todos textos..."),
        ("the result of `windows` would be too large", "el resultado de `windows` sería demasiado grande"),
        ("the function `key` must give values of the same kind for every element: all numbers, or all texts...", "la función `key` debe dar valores del mismo tipo para cada elemento: todos números, o todos textos..."),
        ("`flatten` needs a list of lists, but it contains {}", "`flatten` necesita una lista de listas, pero contiene {}"),
        ("`from_entries` needs a list of [name, value] pairs, but it contains {}", "`from_entries` necesita una lista de pares [nombre, valor], pero contiene {}"),
        ("each pair is a list of two elements whose name is a text, like [\"age\", 36]", "cada par es una lista de dos elementos cuyo nombre es un texto, como [\"edad\", 36]"),
        ("`unwrap` found `None`: there is no value", "`unwrap` encontró `None`: no hay ningún valor"),
        ("handle the missing value with `match`, or give a default: `unwrap_or(x, default)`", "trata la falta de valor con `match`, o da un valor por defecto: `unwrap_or(x, por_defecto)`"),
        ("`unwrap` found an error: {}", "`unwrap` encontró un error: {}"),
        ("handle the error with `match`, pass it on with `?`, or give a default: `unwrap_or(x, default)`", "trata el error con `match`, pásalo con `?`, o da un valor por defecto: `unwrap_or(x, por_defecto)`"),
        // src/check/needs.rs
        ("`{name}` stops with this error, every time", "`{name}` se detiene con este error, cada vez"),
        ("here", "aquí"),
        ("in `{name}`: {label}", "en `{name}`: {label}"),
        ("`{name}` always fails with {}", "`{name}` siempre falla con {}"),
        ("fails every time", "falla cada vez"),
        ("defined here", "definida aquí"),
        ("from how `{name}` uses `{p}`, it needs: `{sig}`", "por cómo `{name}` usa `{p}`, necesita: `{sig}`"),
        // src/check.rs
        ("`{s} [...]` is read as an index, not as a call", "`{s} [...]` se lee como un índice, no como una llamada"),
        ("to call `{s}`, put its argument in parentheses: `{s}([...])`; for an index, write `{s}[...]` without a space", "para llamar a `{s}`, pon su argumento entre paréntesis: `{s}([...])`; para un índice, escribe `{s}[...]` sin espacio"),
        ("`{s} -...` is read as a subtraction, not as a call", "`{s} -...` se lee como una resta, no como una llamada"),
        ("to call `{s}`, put its argument in parentheses: `{s}(-x)`; for a subtraction, write `{s} - x`", "para llamar a `{s}`, pon su argumento entre paréntesis: `{s}(-x)`; para una resta, escribe `{s} - x`"),
        ("`{s}` is not a value here", "`{s}` no es un valor aquí"),
        ("`{s}` is a function here", "`{s}` es una función aquí"),
        ("because `{s}` is given a value here", "porque `{s}` recibe un valor aquí"),
        ("a call without parentheses, like `{s} [1, 2]` or `{s} -1`, is read so only when the file never gives `{s}` a value", "una llamada sin paréntesis, como `{s} [1, 2]` o `{s} -1`, solo se lee así cuando el archivo nunca da un valor a `{s}`"),
        ("the type `{name}` is part of Rythme", "el tipo `{name}` forma parte de Rythme"),
        ("give your type another name", "da otro nombre a tu tipo"),
        ("the type `{}` is already defined", "el tipo `{}` ya está definido"),
        ("first defined here", "definido por primera vez aquí"),
        ("give it another name", "dale otro nombre"),
        ("the field `{f}` appears twice in `{}`", "el campo `{f}` aparece dos veces en `{}`"),
        ("`{s}` is already the name of the file `{file}`", "`{s}` ya es el nombre del archivo `{file}`"),
        ("choose another name here, or give the file another one: `use ... as other_name`", "elige otro nombre aquí, o da otro al archivo: `use ... as otro_nombre`"),
        ("`{s}` is already defined", "`{s}` ya está definido"),
        ("defined again here", "definido de nuevo aquí"),
        ("a constructor belongs to a single type: rename one of them", "un constructor pertenece a un solo tipo: cambia el nombre de uno de los dos"),
        ("to give several definitions to a function (one per case), write them one right after the other", "para dar varias definiciones a una función (una por caso), escríbelas una justo después de la otra"),
        ("`{s}` hides the built-in function `{s}`", "`{s}` oculta la función integrada `{s}`"),
        ("`{s}` hides the built-in value `{s}`", "`{s}` oculta el valor integrado `{s}`"),
        ("in this file, `{s}` will now mean your definition; choose another name to keep the built-in one", "en este archivo, `{s}` significará ahora tu definición; elige otro nombre para conservar la integrada"),
        ("`{s}` is part of Rythme", "`{s}` forma parte de Rythme"),
        ("choose another name", "elige otro nombre"),
        ("the result of `{name}` is lost", "el resultado de `{name}` se pierde"),
        ("this value is lost", "este valor se pierde"),
        (
            r#"values never change: `{sig}` gives a new value, and changes nothing.
To keep it, give it a name: `new = {sig}`"#,
            r#"los valores nunca cambian: `{sig}` da un valor nuevo, y no cambia nada.
Para conservarlo, dale un nombre: `nuevo = {sig}`"#,
        ),
        ("this test can never fail", "esta prueba nunca puede fallar"),
        ("its result, {}, is never compared with what is expected", "su resultado, {}, nunca se compara con lo esperado"),
        (
            r#"a test fails only when it stops with an error, or ends with `false`, `None` or `Err(...)`;
nothing in this one can"#,
            r#"una prueba solo falla cuando se detiene con un error, o termina con `false`, `None` o `Err(...)`;
nada en esta puede hacerlo"#,
        ),
        ("compare the result with what you expect: `assert_eq(actual, expected)`", "compara el resultado con lo que esperas: `assert_eq(obtenido, esperado)`"),
        ("remove `{name}` from the pattern, or write `{name}: _{name}` if this is on purpose", "quita `{name}` del patrón, o escribe `{name}: _{name}` si es a propósito"),
        ("remove it, or name it `_{name}` if this is on purpose", "quítalo, o llámalo `_{name}` si es a propósito"),
        ("`{name}` is never used", "`{name}` nunca se usa"),
        ("`{s}` appears twice in this pattern", "`{s}` aparece dos veces en este patrón"),
        ("first here", "la primera vez aquí"),
        ("use two different names, then compare them with `==` in a condition: `[a, b] if a == b -> ...`", "usa dos nombres distintos, y luego compáralos con `==` en una condición: `[a, b] if a == b -> ...`"),
        ("`{s}` is used before being defined", "`{s}` se usa antes de definirse"),
        ("used here", "usado aquí"),
        ("defined later, here", "definido más adelante, aquí"),
        ("move the definition above this line", "mueve la definición por encima de esta línea"),
        ("`{s}` is the name of the file `{file}`, not a value", "`{s}` es el nombre del archivo `{file}`, no un valor"),
        ("write `{s}.` and the name of one of its definitions, like `{s}.{example}`", "escribe `{s}.` y el nombre de una de sus definiciones, como `{s}.{example}`"),
        ("`{x}` is private to the file `{file}`", "`{x}` es privado del archivo `{file}`"),
        ("private", "privado"),
        ("a name starting with `_` can only be used in its own file", "un nombre que empieza por `_` solo se puede usar en su propio archivo"),
        ("to use it from other files, rename it without `_` in `{file}`", "para usarlo desde otros archivos, quítale el `_` en `{file}`"),
        ("the file `{file}` has no `{x}`", "el archivo `{file}` no tiene `{x}`"),
        ("`{m}.{x}?` is a function of `{file}`, but without `(` right after it, `?` is the operator: call it, `{m}.{x}?(...)`, or give it as `fn(v) = {m}.{x}?(v)`", "`{m}.{x}?` es una función de `{file}`, pero sin `(` justo después, `?` es el operador: llámala, `{m}.{x}?(...)`, o pásala como `fn(v) = {m}.{x}?(v)`"),
        ("not in `{file}`", "no está en `{file}`"),
        ("did you mean `{m}.{best}`?", "¿quisiste decir `{m}.{best}`?"),
        ("`{file}` defines nothing that other files can use", "`{file}` no define nada que otros archivos puedan usar"),
        ("`{file}` gives: {}{more}", "`{file}` da: {}{more}"),
        ("`{name}` is private to the file `{file}`", "`{name}` es privado del archivo `{file}`"),
        ("unknown name `{name}`", "nombre desconocido `{name}`"),
        ("not defined here", "no definido aquí"),
        ("`{name}` is defined in `{file}`: write `{m}.{name}`", "`{name}` está definido en `{file}`: escribe `{m}.{name}`"),
        ("`{name}` is defined in `{file}`: add `use {stem}` at the top of the file, then write `{stem}.{name}`", "`{name}` está definido en `{file}`: añade `use {stem}` al principio del archivo, y luego escribe `{stem}.{name}`"),
        ("not defined", "no definido"),
        ("defined here, further down", "definido aquí, más abajo"),
        ("in a block, a function can be used only after it, unless they are written one after the other with nothing between them: move `{name}` up, next to the function that uses it", "en un bloque, una función solo se puede usar después de ella, salvo si están escritas una tras otra sin nada entre ellas: sube `{name}`, junto a la función que la usa"),
        ("in a block, a name can be used only after the line that defines it: move `{name} = ...` up", "en un bloque, un nombre solo se puede usar después de la línea que lo define: sube `{name} = ...`"),
        ("did you mean `{best}`?", "¿quisiste decir `{best}`?"),
        ("to create your own kind of value, define a type: `type Name = {name}(field) | Other`", "para crear tu propio tipo de valor, define un tipo: `type Nombre = {name}(campo) | Otro`"),
        ("define it before using it: `{name} = ...` or `fn {name}(...) = ...`", "defínelo antes de usarlo: `{name} = ...` o `fn {name}(...) = ...`"),
        ("this definition of `{name}` can never be used", "esta definición de `{name}` nunca se puede usar"),
        ("this definition already accepts every argument", "esta definición ya acepta cualquier argumento"),
        ("definitions are tried in order: put the most general one last", "las definiciones se prueban en orden: pon la más general al final"),
        ("this definition already accepts these arguments", "esta definición ya acepta estos argumentos"),
        ("definitions are tried in order: remove this one, or change its patterns", "las definiciones se prueban en orden: quita esta, o cambia sus patrones"),
        ("this function receives {}, which never matches this pattern", "esta función recibe {}, que nunca encaja con este patrón"),
        ("never matches", "nunca encaja"),
        ("`{name}` does not handle every possible argument", "`{name}` no trata todos los argumentos posibles"),
        ("not handled: {}", "sin tratar: {}"),
        ("add a definition for it, like `fn {name}({}) = ...`", "añade una definición para ese caso, como `fn {name}({}) = ...`"),
        ("the value {} does not match this pattern", "el valor {} no encaja con este patrón"),
        ("a value of type {} never matches this pattern", "un valor de tipo {} nunca encaja con este patrón"),
        ("`_` can only be used in patterns", "`_` solo se puede usar en los patrones"),
        ("`_` means \"anything, ignored\"; give the value a real name to use it", "`_` significa «cualquier cosa, ignorada»; da un nombre de verdad al valor para usarlo"),
        ("`?` can only be used inside a function", "`?` solo se puede usar dentro de una función"),
        ("outside of a function, handle the value with `match` or `unwrap_or`", "fuera de una función, trata el valor con `match` o `unwrap_or`"),
        ("`?` needs an Option or a Result, but got {}", "`?` necesita una Option o un Result, pero recibió {}"),
        ("`?` takes the value out of `Some(x)` / `Ok(x)`, and returns `None` / `Err(e)` from the current function", "`?` saca el valor de `Some(x)` / `Ok(x)`, y devuelve `None` / `Err(e)` desde la función actual"),
        ("the field `{k}` appears twice", "el campo `{k}` aparece dos veces"),
        ("`..` expects {expected} here, but got {}", "`..` espera {expected} aquí, pero recibió {}"),
        ("this value is not a function", "este valor no es una función"),
        ("cannot be called", "no se puede llamar"),
        ("to multiply, write `*` explicitly: `2 * (x + 1)`", "para multiplicar, escribe `*` explícitamente: `2 * (x + 1)`"),
        ("to take one element, use brackets: `x[0]`; between two values, a `,` may be missing", "para tomar un elemento, usa corchetes: `x[0]`; entre dos valores, quizá falte una `,`"),
        ("to read a field, use a dot: `person.name`", "para leer un campo, usa un punto: `persona.nombre`"),
        ("`{s}` is not a function", "`{s}` no es una función"),
        ("`{s}` is {}", "`{s}` es {}"),
        ("returns {returned}", "devuelve {returned}"),
        ("this assertion always fails: {a} and {b} are never equal", "esta aserción siempre falla: {a} y {b} nunca son iguales"),
        (
            r#"one value is inside an Option or a Result: compare it with `Some(...)` or `Ok(...)`,
like `assert_eq(parse_int("3"), Ok(3))`"#,
            r#"uno de los valores está dentro de una Option o un Result: compáralo con `Some(...)` u `Ok(...)`,
como `assert_eq(parse_int("3"), Ok(3))`"#,
        ),
        ("values of different kinds are never equal: Rythme never converts values silently", "los valores de tipos distintos nunca son iguales: Rythme nunca convierte los valores sin avisar"),
        ("these are two different types named `{name}`, defined in two files: their values are never equal", "son dos tipos diferentes llamados `{name}`, definidos en dos archivos: sus valores nunca son iguales"),
        ("a constructor, shown as `<constructor ...>`", "un constructor, mostrado como `<constructor ...>`"),
        ("a function, shown as `<fn ...>`", "una función, mostrada como `<fn ...>`"),
        ("this shows a function, not a value", "esto muestra una función, no un valor"),
        ("to show what it gives, call it: `{call}`", "para mostrar lo que da, llámala: `{call}`"),
        ("no definition of `{name}` accepts these arguments: {name}({})", "ninguna definición de `{name}` acepta estos argumentos: {name}({})"),
        ("called here", "llamada aquí"),
        ("does not match", "no encaja"),
        ("add a definition that handles this case, for example one that ends with a plain name or `_`", "añade una definición que trate este caso, por ejemplo una que termine con un simple nombre o `_`"),
        ("the field `{name}` is not a function", "el campo `{name}` no es una función"),
        ("`{name}` is {}", "`{name}` es {}"),
        ("to read the field, write it without parentheses: `.{name}`", "para leer el campo, escríbelo sin paréntesis: `.{name}`"),
        ("there is no function named `{shown}`", "no existe ninguna función llamada `{shown}`"),
        ("`value.{shown}(...)` calls `{shown}(value, ...)`, or the field `{shown}` of a record", "`valor.{shown}(...)` llama a `{shown}(valor, ...)`, o al campo `{shown}` de un registro"),
        ("`{name}` is a value, not a function", "`{name}` es un valor, no una función"),
        ("write `{name}` without parentheses", "escribe `{name}` sin paréntesis"),
        ("this record has no field `{name}`", "este registro no tiene el campo `{name}`"),
        ("unknown field", "campo desconocido"),
        ("the record is empty: {{}}", "el registro está vacío: {{}}"),
        ("its fields are: {}", "sus campos son: {}"),
        ("to get an Option instead of an error, use `get(record, \"{name}\")`", "para obtener una Option en vez de un error, usa `get(registro, \"{name}\")`"),
        ("{} has no fields", "{} no tiene campos"),
        ("no field `{name}` here", "no hay ningún campo `{name}` aquí"),
        ("did you mean `.{best}`? (`value.{best}` calls `{best}(value)`)", "¿quisiste decir `.{best}`? (`valor.{best}` llama a `{best}(valor)`)"),
        ("`{c}` has no field `{name}`", "`{c}` no tiene el campo `{name}`"),
        ("no variant of `{tn}` has a field `{name}`", "ninguna variante de `{tn}` tiene el campo `{name}`"),
        ("`{c}` has no fields", "`{c}` no tiene campos"),
        ("the fields of `{c}` are: {}", "los campos de `{c}` son: {}"),
        ("the variants of `{tn}` have no fields", "las variantes de `{tn}` no tienen campos"),
        ("their fields are: {}", "sus campos son: {}"),
        ("a value of type `{tn}` may be another variant: use `match` to handle each case", "un valor del tipo `{tn}` puede ser otra variante: usa `match` para tratar cada caso"),
        ("cannot use `[...]` on {}", "no se puede usar `[...]` con {}"),
        ("`[...]` works on lists, texts, records and dicts", "`[...]` funciona con las listas, los textos, los registros y los diccionarios"),
        ("an index must be a whole number (Int), but got {}", "un índice debe ser un número entero (Int), pero recibió {}"),
        ("the name of a field must be a text, but got {}", "el nombre de un campo debe ser un texto, pero recibió {}"),
        ("read a field with `record[\"name\"]` or `record.name`", "lee un campo con `registro[\"nombre\"]` o `registro.nombre`"),
        ("this record has no field `{k}`", "este registro no tiene el campo `{k}`"),
        ("use `get(record, key)` to receive an Option instead", "usa `get(registro, clave)` para recibir una Option en su lugar"),
        ("{} and {}", "{} y {}"),
        ("a range needs whole numbers, but got {got}", "un intervalo necesita números enteros, pero recibió {got}"),
        ("example: `1..10` is the list of the numbers from 1 to 10", "ejemplo: `1..10` es la lista de los números del 1 al 10"),
        ("cannot use `-` on {}", "no se puede usar `-` con {}"),
        ("a condition must be `true` or `false`, but this is {}", "una condición debe ser `true` o `false`, pero esto es {}"),
        ("not a Bool", "no es un Bool"),
        ("compare it explicitly, for example `x != 0` or `x > 0`", "compáralo explícitamente, por ejemplo `x != 0` o `x > 0`"),
        ("to test if it is empty, use `is_empty(x)` or `not is_empty(x)`", "para comprobar si está vacío, usa `is_empty(x)` o `not is_empty(x)`"),
        ("use `is_some(x)`, `is_ok(x)`, or `match` to look inside", "usa `is_some(x)`, `is_ok(x)`, o `match` para mirar dentro"),
        ("cannot use `{sym}` with {ln} and {rn}", "no se puede usar `{sym}` con {ln} y {rn}"),
        ("cannot use `{sym}` with {ln}", "no se puede usar `{sym}` con {ln}"),
        ("cannot use `{sym}` with {rn}", "no se puede usar `{sym}` con {rn}"),
        ("`{name} -x` subtracts here; to call `{name}`, write `{name}(-x)` (a call without parentheses starts a statement or a result, such as a branch of `if` or the body after the `=` of a function, and its name is not given a value with `=` or as a parameter)", "`{name} -x` resta aquí; para llamar a `{name}`, escribe `{name}(-x)` (una llamada sin paréntesis empieza una instrucción o un resultado, como una rama de `if` o el cuerpo tras el `=` de una función, y su nombre no recibe un valor con `=` ni como parámetro)"),
        ("to join two sets or two dicts, use `union(a, b)`; to take one away from the other, `difference(a, b)`", "para unir dos conjuntos o dos diccionarios, usa `union(a, b)`; para quitar uno del otro, `difference(a, b)`"),
        (
            r#"to build a text with a value inside, use braces: "total: {{x}}"
or convert the value first: text(x)"#,
            r#"para construir un texto con un valor dentro, usa llaves: "total: {{x}}"
o convierte antes el valor: text(x)"#,
        ),
        ("to add one element to a list, use `push(list, x)` or `[..list, x]`", "para añadir un elemento a una lista, usa `push(lista, x)` o `[..lista, x]`"),
        ("to repeat a text or a list, use `repeat(x, n)`", "para repetir un texto o una lista, usa `repeat(x, n)`"),
        ("convert the text to a number first, with `int(t)` or `float(t)`", "convierte antes el texto en número, con `int(t)` o `float(t)`"),
        ("only values of the same kind can be compared", "solo se pueden comparar valores del mismo tipo"),
        ("this comparison is always false: {ln} and {rn} are never equal", "esta comparación siempre es falsa: {ln} y {rn} nunca son iguales"),
        ("this comparison is always true: {ln} and {rn} are never equal", "esta comparación siempre es verdadera: {ln} y {rn} nunca son iguales"),
        ("to look inside an Option, use `match`, `unwrap_or(x, default)` or `is_some(x)`", "para mirar dentro de una Option, usa `match`, `unwrap_or(x, por_defecto)` o `is_some(x)`"),
        ("Rythme never converts values silently: convert one side first, with `text(x)` or `int(t)`", "Rythme nunca convierte los valores sin avisar: convierte antes uno de los lados, con `text(x)` o `int(t)`"),
        ("this case can never be reached", "este caso nunca se puede alcanzar"),
        ("this case already handles everything", "este caso ya lo trata todo"),
        ("cases are tried in order: put `_` last", "los casos se prueban en orden: pon `_` al final"),
        ("this case already handles these values", "este caso ya trata estos valores"),
        ("a value goes to the first case that fits: remove this case, or change it", "un valor va al primer caso que le encaja: quita este caso, o cámbialo"),
        ("no case of this `match` handles {}", "ningún caso de este `match` trata {}"),
        ("this value", "este valor"),
        ("add a case for this value, or a last case `_ -> ...` that handles everything else", "añade un caso para este valor, o un último caso `_ -> ...` que trate todo lo demás"),
        ("this `match` does not handle every case", "este `match` no trata todos los casos"),
        ("add a last case `_ -> ...` for all the other values", "añade un último caso `_ -> ...` para todos los demás valores"),
        ("add the missing cases, like `{} -> ...`, or a last case `_ -> ...`", "añade los casos que faltan, como `{} -> ...`, o un último caso `_ -> ...`"),
        ("unknown constructor `{n}`", "constructor desconocido `{n}`"),
        ("constructors are defined with `type`: `type Name = {n}(field) | Other`", "los constructores se definen con `type`: `type Nombre = {n}(campo) | Otro`"),
        ("`{name}` has {} field(s), but this pattern gives {}", "`{name}` tiene {} campo(s), pero este patrón da {}"),
        ("write it like `{example}`", "escríbelo como `{example}`"),
        ("this range never matches", "este intervalo nunca encaja"),
        ("no number goes from {lo} up to {hi}", "ningún número va de {lo} hasta {hi}"),
        ("a range goes from the smaller number to the larger one: `{hi}..{lo}`", "un intervalo va del número más pequeño al más grande: `{hi}..{lo}`"),
        ("patterns joined with `|` cannot give names (here `{s}`)", "los patrones unidos con `|` no pueden dar nombres (aquí `{s}`)"),
        ("write separate cases instead", "escribe casos separados en su lugar"),
        ("handles some of these numbers", "trata una parte de estos números"),
        ("together, the cases before it already handle all its numbers: remove it, or change it", "juntos, los casos anteriores ya tratan todos sus números: quítalo, o cámbialo"),
        ("these ranges overlap", "estos intervalos se solapan"),
        ("{both} is also here", "{both} también está aquí"),
        ("{both} goes here, the first that fits", "{both} va aquí, al primero que encaja"),
        ("both ends of a range are included: make the ranges follow each other without overlapping, like `0..9` then `10..19`", "los dos extremos de un intervalo están incluidos: haz que los intervalos se sigan sin solaparse, como `0..9` y luego `10..19`"),
        ("use `match` to handle the different possible shapes of a value", "usa `match` para tratar las distintas formas posibles de un valor"),
        ("a value of type {}", "un valor de tipo {}"),
        ("this line computes a value but does nothing with it", "esta línea calcula un valor pero no hace nada con él"),
        (
            r#"a line that starts with `-` is a new statement. To continue the line above,
end that line with `-` instead:
  total = price -
    discount"#,
            r#"una línea que empieza por `-` es una instrucción nueva. Para continuar la línea de arriba,
termina mejor esa línea con `-`:
  total = precio -
    descuento"#,
        ),
        (
            r#"to keep the value, give it a name: `result = ...`; to show it: `print(...)`.
Values never change: `x + 1` does not change `x`."#,
            r#"para conservar el valor, dale un nombre: `resultado = ...`; para mostrarlo: `print(...)`.
Los valores nunca cambian: `x + 1` no cambia `x`."#,
        ),
        // src/diagnostic.rs
        ("error", "error"),
        ("warning", "advertencia"),
        ("help", "ayuda"),
        ("call trace (most recent first):", "llamadas en curso (la más reciente primero):"),
        ("an anonymous function", "una función anónima"),
        ("in", "en"),
        ("called at", "llamada en"),
        ("and {} more calls", "y {} llamadas más"),
        ("more about this error: `rythme explain {}`", "más sobre este error: `rythme explain {}`"),
        // src/doc.rs
        ("`{a}` needs a file name, like `rythme doc geometry.ry --out geometry.html`", "`{a}` necesita un nombre de archivo, como `rythme doc geometria.ry --out geometria.html`"),
        ("unknown option `{o}` for `rythme doc`", "opción desconocida `{o}` para `rythme doc`"),
        (
            r#"
did you mean `{best}`?"#,
            r#"
¿quisiste decir `{best}`?"#,
        ),
        (
            r#"
see `rythme help doc`"#,
            r#"
consulta `rythme help doc`"#,
        ),
        (
            r#"which files? Example: rythme doc geometry.ry
  a folder documents all its .ry files: rythme doc .
  --html writes one HTML page instead of Markdown
  --out FILE writes to a file (a name ending with .html gives HTML)
see `rythme help doc`"#,
            r#"¿qué archivos? Ejemplo: rythme doc geometria.ry
  una carpeta documenta todos sus archivos .ry: rythme doc .
  --html escribe una página HTML en vez de Markdown
  --out ARCHIVO escribe en un archivo (un nombre que termina en .html da HTML)
consulta `rythme help doc`"#,
        ),
        ("no .ry file in `{target}`", "ningún archivo .ry en `{target}`"),
        ("Documentation of {one}", "Documentación de {one}"),
        ("Documentation", "Documentación"),
        ("written: {file}", "escrito: {file}"),
        ("cannot write the file `{file}`: {e}", "no se puede escribir el archivo `{file}`: {e}"),
        ("Types", "Tipos"),
        ("Functions", "Funciones"),
        ("Values", "Valores"),
        ("Tests", "Pruebas"),
        ("Uses", "Usa"),
        ("Contents", "Índice"),
        ("Built-in functions mentioned", "Funciones integradas citadas"),
        ("Example", "Ejemplo"),
        ("Nothing public to document.", "Nada público que documentar."),
        ("Documentation written by `rythme doc`", "Documentación escrita por `rythme doc`"),
        (" Same as: {} (its Ruby names).", " Igual que: {} (sus nombres de Ruby)."),
        ("Example:", "Ejemplo:"),
        (" as {open}{n}{close}", " con el nombre {open}{n}{close}"),
        (": ", ": "),
        // src/driver.rs
        ("`{shown}` is a folder, not a file", "`{shown}` es una carpeta, no un archivo"),
        ("give a file inside it, like {}", "da un archivo que esté dentro, como {}"),
        ("file `{shown}` not found", "archivo `{shown}` no encontrado"),
        ("no permission to read the file `{shown}`", "no hay permiso para leer el archivo `{shown}`"),
        ("cannot read the file `{shown}`: {e}", "no se puede leer el archivo `{shown}`: {e}"),
        ("the file `{shown}` is not text in the UTF-8 encoding (unexpected byte on line {line})", "el archivo `{shown}` no es texto en la codificación UTF-8 (byte inesperado en la línea {line})"),
        ("it looks like it was saved in UTF-16", "parece que se guardó en UTF-16"),
        ("save it again with the UTF-8 encoding (most editors have it in \"Save as\" or in the status bar)", "guárdalo de nuevo con la codificación UTF-8 (la mayoría de los editores lo ofrecen en «Guardar como» o en la barra de estado)"),
        ("(`{name}` now has {n} definitions, tried in order; :reset to start again)", "(`{name}` tiene ahora {n} definiciones, probadas en orden; :reset para empezar de nuevo)"),
        ("file `{p}` not found", "archivo `{p}` no encontrado"),
        ("looked for: {}", "se buscó: {}"),
        ("the Rythme files there are: {}{more}", "los archivos de Rythme de esa carpeta son: {}{more}"),
        ("the path is relative to the folder of the file that contains `use`", "la ruta es relativa a la carpeta del archivo que contiene `use`"),
        ("`{p}` uses itself", "`{p}` se usa a sí mismo"),
        ("`{last}` and `{first}` use each other", "`{last}` y `{first}` se usan el uno al otro"),
        ("the circle: {}", "el círculo: {}"),
        ("files cannot use each other: move what both need into a third file, used by both", "los archivos no pueden usarse el uno al otro: mueve lo que ambos necesitan a un tercer archivo, usado por los dos"),
        ("the name `{n}` is given to two files: `{first}` and `{second}`", "el nombre `{n}` se da a dos archivos: `{first}` y `{second}`"),
        ("give this one another name: `use ... as other_name`", "da otro nombre a este: `use ... as otro_nombre`"),
        ("`{s}` is defined by two used files: `{first}` and `{second}`", "`{s}` está definido en dos archivos usados: `{first}` y `{second}`"),
        ("use the files with their names, without quotes, and write which one you mean: `use geometry`, then `geometry.{s}`", "usa los archivos por su nombre, sin comillas, y escribe a cuál te refieres: `use geometria`, y luego `geometria.{s}`"),
        // src/exercise.rs
        ("which exercise? Example: rythme exercise start 1   (`rythme exercise list` shows them all)", "¿qué ejercicio? Ejemplo: rythme exercise start 1   (`rythme exercise list` los muestra todos)"),
        ("which file? Example: rythme exercise check 01-values.ry", "¿qué archivo? Ejemplo: rythme exercise check 01-valores.ry"),
        ("unknown action `{other}` for `rythme exercise`", "acción desconocida `{other}` para `rythme exercise`"),
        (
            r#"
did you mean `rythme exercise {best}`?"#,
            r#"
¿quisiste decir `rythme exercise {best}`?"#,
        ),
        (
            r#"
the actions are: list, start NAME, check FILE (see `rythme help exercises`)"#,
            r#"
las acciones son: list, start NOMBRE, check ARCHIVO (consulta `rythme help exercises`)"#,
        ),
        ("{} exercises for a first course, from the first values to tests, from ★☆☆ (easy) to ★★★.", "{} ejercicios para un primer curso, de los primeros valores a las pruebas, de ★☆☆ (fácil) a ★★★."),
        (
            r#"Start one:       rythme exercise start 1
Check your work: rythme exercise check {}.ry"#,
            r#"Empezar uno:      rythme exercise start 1
Comprobar:        rythme exercise check {}.ry"#,
        ),
        ("there is no exercise `{name}`", "no hay ningún ejercicio `{name}`"),
        (
            r#"
the exercises are numbered from 1 to {}: `rythme exercise list` shows them"#,
            r#"
los ejercicios están numerados del 1 al {}: `rythme exercise list` los muestra"#,
        ),
        ("`{file}` already exists here: open it to continue, or rename it to start again", "`{file}` ya existe aquí: ábrelo para continuar, o cámbiale el nombre para empezar de nuevo"),
        ("cannot write `{file}`: {error}", "no se puede escribir `{file}`: {error}"),
        (
            r#"Created {file}: exercise {} · {title}  {level}

Open it, read the statement and the hints at the top, and replace each
`fail("to do")` with your code. Then check your work:

  rythme exercise check {file}"#,
            r#"Creado {file}: ejercicio {} · {title}  {level}

Ábrelo, lee el enunciado y las pistas de arriba, y reemplaza cada
`fail("por hacer")` por tu código. Después comprueba tu trabajo:

  rythme exercise check {file}"#,
        ),
        ("file `{name}` not found", "archivo `{name}` no encontrado"),
        (
            r#"
to start this exercise: rythme exercise start {}"#,
            r#"
para empezar este ejercicio: rythme exercise start {}"#,
        ),
        (
            r#"✗ The file has errors, so its tests could not run.
  Fix the first error above, then check again."#,
            r#"✗ El archivo tiene errores, así que sus pruebas no se pudieron ejecutar.
  Corrige el primer error de arriba, y vuelve a comprobar."#,
        ),
        ("✗ This file has no tests. An exercise comes with its tests: is it the right file?", "✗ Este archivo no tiene pruebas. Un ejercicio viene con sus pruebas: ¿es el archivo correcto?"),
        ("✓ Well done: all {total} tests pass!", "✓ ¡Bien hecho: las {total} pruebas pasan!"),
        (
            r#"  Next exercise: {title}  {level}
    rythme exercise start {}"#,
            r#"  Siguiente ejercicio: {title}  {level}
    rythme exercise start {}"#,
        ),
        ("  That was the last exercise: you have finished them all. Congratulations!", "  Era el último ejercicio: los has terminado todos. ¡Enhorabuena!"),
        ("✗ All {total} tests pass, but...", "✗ Las {total} pruebas pasan, pero..."),
        ("✗ {passed} of {total} tests pass. Keep going!", "✗ Pasan {passed} de {total} pruebas. ¡Sigue así!"),
        (
            r#"  - The tests of the exercise were changed: the exercise is solved when its own tests
    pass. `rythme exercise start {number}` gives a fresh copy (rename your file first)."#,
            r#"  - Las pruebas del ejercicio se cambiaron: el ejercicio está resuelto cuando pasan sus propias
    pruebas. `rythme exercise start {number}` da una copia nueva (cambia antes el nombre de tu archivo)."#,
        ),
        ("  - Some functions are still to do: replace each `fail(\"to do\")` with your code.", "  - Aún quedan funciones por hacer: reemplaza cada `fail(\"por hacer\")` por tu código."),
        (
            r#"  - Start with the first failure above: compare what you got with what was expected.
  - Stuck? Read the hints at the top of the file, or `rythme help NAME` for a function."#,
            r#"  - Empieza por el primer fallo de arriba: compara lo que obtuviste con lo que se esperaba.
  - ¿Atascado? Lee las pistas al principio del archivo, o `rythme help NOMBRE` para una función."#,
        ),
        // src/format.rs
        ("`{name}` could not be formatted safely, so it was left as it is", "`{name}` no se pudo formatear sin riesgo, así que se dejó tal cual"),
        ("this is a bug in Rythme, not in your program: please report it, with the file, at https://github.com/maxlestage/Rythme/issues", "es un fallo de Rythme, no de tu programa: por favor, avísanos, con el archivo, en https://github.com/maxlestage/Rythme/issues"),
        ("unknown option `{o}` for `rythme fmt`", "opción desconocida `{o}` para `rythme fmt`"),
        (
            r#"
see `rythme help fmt`"#,
            r#"
consulta `rythme help fmt`"#,
        ),
        (
            r#"which files? Example: rythme fmt hello.ry
  a folder formats all its .ry files: rythme fmt .
  - reads the code from the standard input
  --check only checks, for continuous integration
see `rythme help fmt`"#,
            r#"¿qué archivos? Ejemplo: rythme fmt hola.ry
  una carpeta formatea todos sus archivos .ry: rythme fmt .
  - lee el código de la entrada estándar
  --check solo comprueba, para la integración continua
consulta `rythme help fmt`"#,
        ),
        ("needs formatting: {shown}", "necesita formato: {shown}"),
        ("cannot write the file `{shown}`: {e}", "no se puede escribir el archivo `{shown}`: {e}"),
        ("formatted: {shown}", "formateado: {shown}"),
        ("needs formatting: <stdin>", "necesita formato: <stdin>"),
        ("{changed} file(s) formatted, {unchanged} already formatted", "{changed} archivo(s) formateado(s), {unchanged} ya con formato"),
        ("✓ {unchanged} file(s) already formatted", "✓ {unchanged} archivo(s) ya con formato"),
        ("✗ {changed} file(s) to format, {unchanged} already formatted; `rythme fmt` (without --check) formats them", "✗ {changed} archivo(s) por formatear, {unchanged} ya con formato; `rythme fmt` (sin --check) los formatea"),
        ("✗ {errors} file(s) left as they are because of errors", "✗ {errors} archivo(s) dejado(s) tal cual por culpa de errores"),
        ("file or folder `{shown}` not found", "archivo o carpeta `{shown}` no encontrado"),
        ("did you mean `{}`?", "¿quisiste decir `{}`?"),
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
            r#"Rythme {v} — ayuda

Comandos
  rythme                    modo interactivo (probar código línea a línea)
  rythme archivo.ry         ejecutar un programa
  rythme check archivo.ry   encontrar problemas sin ejecutar el programa
  rythme test archivo.ry    ejecutar las pruebas de un archivo (o de una carpeta)
  rythme fmt archivo.ry     dar al código su formato oficial (--check: solo comprobar)
  rythme doc archivo.ry     escribir su documentación, en Markdown (--html: una página web)
  rythme steps archivo.ry   mostrar el cálculo paso a paso (fib(3) → fib(2) + fib(1) → ...)
  rythme exercise           ejercicios corregidos automáticamente (list, start, check)
  rythme new NOMBRE         crear un proyecto (rythme.toml, src/, tests/)
  rythme install            descargar las dependencias del proyecto
  rythme learn              visita guiada del lenguaje, paso a paso
  rythme help TEMA          ayuda sobre un tema o una función
  rythme explain E003       explicar un código de error
  rythme -e "código"        ejecutar un trozo de código
  rythme lsp                servidor de lenguaje, para los editores (VS Code, Neovim...)
  opción --lang CÓDIGO      idioma de los mensajes: en, fr, es, de, pt, zh (o variable RYTHME_LANG)

Temas (rythme help TEMA)
  start       el lenguaje en un minuto
  syntax      toda la sintaxis en una página
  functions   funciones, |>, recursión
  lists       las listas y sus funciones
  text        textos
  records     registros {{nombre: valor}}
  dicts       diccionarios y conjuntos: dict(...), set(...)
  types       tus propios tipos
  match       elegir según la forma de un valor
  errors      Option, Result y ?
  functional  sin bucles: qué hacer en su lugar
  tests       escribir y ejecutar pruebas
  exercises   ejercicios para aprender, con sus pruebas
  modules     usar varios archivos
  system      archivos, carpetas, rutas, opciones, entrada, códigos de salida
  projects    proyectos y paquetes: rythme new, rythme.toml, rythme install
  dates       fechas y horas: now(), today(), calendario, ISO 8601
  regex       buscar, cortar y reemplazar con expresiones regulares
  repl        el modo interactivo
  fmt         el formato oficial del código (rythme fmt)
  doc         la documentación sacada de los comentarios ## (rythme doc)
  steps       el cálculo mostrado paso a paso (rythme steps)
  editors     Rythme en tu editor (VS Code, Neovim, Helix...)
  ruby        si vienes de Ruby: bloques, símbolos, cadenas de métodos
  builtins    todas las funciones integradas

Una función concreta: rythme help map
"#,
        ),
        ("{alias}  (the Ruby name of `{of}`: the same function)", "{alias}  (el nombre de Ruby de `{of}`: la misma función)"),
        (
            r#"pi
  The number π = 3.141592653589793."#,
            r#"pi
  El número π = 3.141592653589793."#,
        ),
        ("The function:", "La función:"),
        ("No help found for `{name}`.", "No se encontró ayuda para `{name}`."),
        (
            r#"
Did you mean `{best}`?"#,
            r#"
¿Quisiste decir `{best}`?"#,
        ),
        (
            r#"
See the list of topics with `rythme help`."#,
            r#"
Consulta la lista de temas con `rythme help`."#,
        ),
        ("keyword", "palabra clave"),
        (
            r#"  More: rythme help {topic}
"#,
            r#"  Más: rythme help {topic}
"#,
        ),
        (
            r#"  Same as: {} (its Ruby names)
"#,
            r#"  Igual que: {} (sus nombres de Ruby)
"#,
        ),
        (
            r#"BUILT-IN FUNCTIONS
"#,
            r#"FUNCIONES INTEGRADAS
"#,
        ),
        ("the number π", "el número π"),
        ("{} (same as {})", "{} (igual que {})"),
        ("  Ruby names:", "  Nombres de Ruby:"),
        ("build an Option or a Result", "construir una Option o un Result"),
        (
            r#"
Details and example for one function: rythme help NAME (or help(NAME) in code)
"#,
            r#"
Detalles y ejemplo de una función: rythme help NOMBRE (o help(NOMBRE) en el código)
"#,
        ),
        (
            r#"help(x) explains x: a function (help(map)), a topic (help("lists")), or a value (help([1, 2])).
Topics: start, syntax, functions, lists, text, records, types, match, errors, functional, tests, modules, builtins."#,
            r#"help(x) explica x: una función (help(map)), un tema (help("lists")), o un valor (help([1, 2])).
Temas: start, syntax, functions, lists, text, records, types, match, errors, functional, tests, modules, builtins."#,
        ),
        (
            r#"  (no documentation: write `## description` on the line above `fn`)
"#,
            r#"  (sin documentación: escribe `## descripción` en la línea de encima de `fn`)
"#,
        ),
        ("defined at", "definida en"),
        (
            r#"{}({})
  A constructor of the type `{}`.
"#,
            r#"{}({})
  Un constructor del tipo `{}`.
"#,
        ),
        (
            r#"This is a value of type {ty}: {}
"#,
            r#"Esto es un valor de tipo {ty}: {}
"#,
        ),
        (
            r#"Useful functions: {}
"#,
            r#"Funciones útiles: {}
"#,
        ),
        (
            r#"Also: {also}
"#,
            r#"También: {also}
"#,
        ),
        (
            r#"Use `match` to handle each variant of `{}`.
"#,
            r#"Usa `match` para tratar cada variante de `{}`.
"#,
        ),
        ("[Enter] next   [q] quit > ", "[Intro] siguiente   [q] salir > "),
        ("To practice, run `rythme` and type code, or do the exercises: `rythme exercise list`. `rythme help builtins` lists every function.", "Para practicar, ejecuta `rythme` y escribe código, o haz los ejercicios: `rythme exercise list`. `rythme help builtins` lista todas las funciones."),
        // src/interp.rs
        ("record", "registro"),
        ("`:{field}` is a symbol, not a text: write `{r}.{field}` or `{r}[\"{field}\"]`, or `get({r}, \"{field}\")` to receive an Option", "`:{field}` es un símbolo, no un texto: escribe `{r}.{field}` o `{r}[\"{field}\"]`, o `get({r}, \"{field}\")` para recibir una Option"),
        ("the dict has the key {shown}, which is not the same key as {key}: write `[{shown}]`", "el diccionario tiene la clave {shown}, que no es la misma clave que {key}: escribe `[{shown}]`"),
        ("this list would be too large (more than {MAX_LIST} elements)", "esta lista sería demasiado grande (más de {MAX_LIST} elementos)"),
        ("this text would be too large (more than {mb} MB)", "este texto sería demasiado grande (más de {mb} MB)"),
        ("does it grow in a function that calls itself? Check that the function always reaches a case where it stops", "¿crece en una función que se llama a sí misma? Comprueba que la función siempre llega a un caso en el que se detiene"),
        ("no function named `{}`", "ninguna función se llama `{}`"),
        ("`value.{0}(...)` calls `{0}(value, ...)`", "`valor.{0}(...)` llama a `{0}(valor, ...)`"),
        ("`{name}` is not defined yet", "`{name}` aún no está definido"),
        ("the file runs from top to bottom: move this definition above the line that uses it (directly or through a function)", "el archivo se ejecuta de arriba abajo: mueve esta definición por encima de la línea que la usa (directamente o a través de una función)"),
        ("`{ctor}` has no field `{name}`", "`{ctor}` no tiene el campo `{name}`"),
        ("`{ctor}` has no fields", "`{ctor}` no tiene campos"),
        ("the fields of `{ctor}` are: {}", "los campos de `{ctor}` son: {}"),
        ("a value of type `{}` may be another variant: use `match` to handle each case", "un valor del tipo `{}` puede ser otra variante: usa `match` para tratar cada caso"),
        ("too many nested calls ({depth} calls in progress)", "demasiadas llamadas anidadas ({depth} llamadas en curso)"),
        ("this call goes too deep", "esta llamada va demasiado lejos"),
        (
            r#"a recursive function that never stops? Check that it always reaches a base case.
Otherwise, make the recursive call the very last thing the function does
(tail call), for example with an accumulator:
  fn sum([], acc) = acc
  fn sum([x, ..rest], acc) = sum(rest, acc + x)
or use `fold`, `map`, `filter`, which have no limit."#,
            r#"¿una función recursiva que nunca se detiene? Comprueba que siempre llega a un caso base.
Si no, haz que la llamada recursiva sea lo último que hace la función
(llamada final), por ejemplo con un acumulador:
  fn suma([], acc) = acc
  fn suma([x, ..resto], acc) = suma(resto, acc + x)
o usa `fold`, `map`, `filter`, que no tienen límite."#,
        ),
        ("the program was stopped: it made more calls than allowed for this run", "el programa se detuvo: hizo más llamadas de las permitidas para esta ejecución"),
        ("does it call itself forever? Check that it always reaches a base case", "¿se llama a sí misma sin fin? Comprueba que siempre llega a un caso base"),
        ("{} is not a function", "{} no es una función"),
        ("`{0}` is a value, not a function: write `{0}` without parentheses", "`{0}` es un valor, no una función: escribe `{0}` sin paréntesis"),
        ("`..` expects {what} here, but got {}", "`..` espera {what} aquí, pero recibió {}"),
        ("division by zero", "división entre cero"),
        (
            r#"check the divisor first: `if b == 0 then ... else a / b`,
or return a Result: `if b == 0 then Err("division by zero") else Ok(a / b)`"#,
            r#"comprueba antes el divisor: `if b == 0 then ... else a / b`,
o devuelve un Result: `if b == 0 then Err("división entre cero") else Ok(a / b)`"#,
        ),
        ("`0 ** -n` is `1 / 0 ** n`", "`0 ** -n` es `1 / 0 ** n`"),
        ("{min} argument{s}", "{min} argumento{s}"),
        ("at least {min} argument{s}", "al menos {min} argumento{s}"),
        ("{min} to {max} arguments", "de {min} a {max} argumentos"),
        ("{got} {verb}", "{got}"),
        ("`{name}` takes {expected}, but {given}", "`{name}` espera {expected}, pero recibió {given}"),
        ("{got} argument(s) here", "{got} argumento(s) aquí"),
        ("usage: `{sig}`", "uso: `{sig}`"),
        ("cannot use `{sym}` with {lt} and {rt}", "no se puede usar `{sym}` con {lt} y {rt}"),
        ("`nan` (not a number) cannot be compared", "`nan` (no es un número) no se puede comparar"),
        ("a negative number ({}) to a fractional power ({}) has no value among real numbers", "un número negativo ({}) elevado a una potencia no entera ({}) no tiene valor entre los números reales"),
        ("use `abs(x) ** p` for the power of its absolute value", "usa `abs(x) ** p` para la potencia de su valor absoluto"),
        ("this range is too large ({} elements)", "este intervalo es demasiado grande ({} elementos)"),
        ("ranges are lists: keep them under {MAX_RANGE} elements", "los intervalos son listas: mantenlos por debajo de {MAX_RANGE} elementos"),
        ("a range needs whole numbers, but got {} and {}", "un intervalo necesita números enteros, pero recibió {} y {}"),
        ("index {n} is out of range: {what}", "el índice {n} está fuera de rango: {what}"),
        ("valid indices go from 0 to {} (or -1 for the last element)", "los índices válidos van de 0 a {} (o -1 para el último elemento)"),
        ("to get an Option instead of an error, use `get(x, {n})`", "para obtener una Option en vez de un error, usa `get(x, {n})`"),
        ("the text is empty", "el texto está vacío"),
        ("the text has {len} character(s)", "el texto tiene {len} carácter(es)"),
        ("the list has {} element(s)", "la lista tiene {} elemento(s)"),
        ("this dict has no key {}", "este diccionario no tiene la clave {}"),
        ("use `get(dict, key)` to receive an Option instead, or `has(dict, key)` to check first", "usa `get(dic, clave)` para recibir una Option en su lugar, o `has(dic, clave)` para comprobarlo antes"),
        // src/lexer.rs
        ("`...` is not an operator in Rythme", "`...` no es un operador en Rythme"),
        ("write `1..10` for a range (10 included), or `[first, ..rest]` in a list", "escribe `1..10` para un intervalo (10 incluido), o `[primero, ..resto]` en una lista"),
        ("`++` is not an operator in Rythme", "`++` no es un operador en Rythme"),
        (
            r#"to add one, write `x + 1` and give the result a new name (values never change);
to join two lists or two texts, use `+`: `[1] + [2]`"#,
            r#"para sumar uno, escribe `x + 1` y da un nombre nuevo al resultado (los valores nunca cambian);
para juntar dos listas o dos textos, usa `+`: `[1] + [2]`"#,
        ),
        ("unexpected character `&`", "carácter inesperado `&`"),
        ("use `and` to combine two conditions", "usa `and` para combinar dos condiciones"),
        ("this `{{` in a text is never closed", "esta `{{` en un texto nunca se cierra"),
        ("starts a value inserted in the text", "empieza un valor insertado en el texto"),
        ("close it with `}}`, or write `\\{{` to get a literal brace: \"\\{{\"", "ciérrala con `}}`, o escribe `\\{{` para obtener una llave tal cual: \"\\{{\""),
        ("unexpected invisible character {code}", "carácter invisible inesperado {code}"),
        ("this is a special space (often copied from a document or a web page): replace it with a normal space", "es un espacio especial (a menudo copiado de un documento o de una página web): reemplázalo por un espacio normal"),
        ("this character cannot be seen (it is often copied from a document or a web page): delete it", "este carácter no se ve (a menudo se copia de un documento o de una página web): bórralo"),
        ("unexpected character `{c}`", "carácter inesperado `{c}`"),
        ("write `{s}` instead", "escribe `{s}` en su lugar"),
        ("texts use double quotes: \"hello\"", "los textos van entre comillas dobles: \"hola\""),
        ("use `**` for powers: `2 ** 10`", "usa `**` para las potencias: `2 ** 10`"),
        ("use straight double quotes `\"` for texts", "usa comillas dobles rectas `\"` para los textos"),
        ("to insert a value in a text, use braces: \"total: {{x}}\"", "para insertar un valor en un texto, usa llaves: \"total: {{x}}\""),
        ("invalid number `{text}`", "número no válido `{text}`"),
        ("hexadecimal numbers use the digits 0 to 9 and a to f, like `0xff`; binary numbers use 0 and 1, like `0b101`", "los números hexadecimales usan las cifras 0 a 9 y a a f, como `0xff`; los binarios usan 0 y 1, como `0b101`"),
        ("a name cannot start with a digit: `{}`", "un nombre no puede empezar por una cifra: `{}`"),
        ("to multiply, write it explicitly: `{num} * {word}`", "para multiplicar, escríbelo explícitamente: `{num} * {word}`"),
        ("texts are nested too deeply inside `{{...}}`", "los textos están anidados demasiado dentro de `{{...}}`"),
        ("this text is never closed", "este texto nunca se cierra"),
        ("the text starts here", "el texto empieza aquí"),
        ("add a `\"` at the end of the text", "añade un `\"` al final del texto"),
        ("invalid unicode escape", "escape unicode no válido"),
        ("write it like `\\u{{e9}}` for é", "escríbelo como `\\u{{e9}}` para é"),
        ("unknown escape sequence in text", "secuencia de escape desconocida en el texto"),
        ("in a regex, write the backslash twice: \"\\\\{c}\" is the text \\{c}", "en una regex, escribe la barra invertida dos veces: \"\\\\{c}\" es el texto \\{c}"),
        (
            r#"valid escapes: \n (new line), \t (tab), \r (carriage return), \0 (null character), \\ (backslash),
\" (quote), \{{ and \}} (braces), \# (a `#` before a brace), \u{{e9}} (a character by its Unicode number: é)"#,
            r#"escapes válidos: \n (nueva línea), \t (tabulador), \r (retorno de carro), \0 (carácter nulo), \\ (barra invertida),
\" (comilla), \{{ y \}} (llaves), \# (un `#` delante de una llave), \u{{e9}} (un carácter por su número Unicode: é)"#,
        ),
        ("inside it, this text is never closed", "dentro, este texto nunca se cierra"),
        ("empty `{{}}` in a text", "`{{}}` vacío en un texto"),
        ("put a value inside, like \"{{name}}\", or write `\\{{` for a literal brace", "pon un valor dentro, como \"{{nombre}}\", o escribe `\\{{` para una llave tal cual"),
        ("inside a text, `{{` starts an inserted value: to write braces, use `\\{{` and `\\}}`, like \"\\{{\\\"a\\\": 1\\}}\"", "dentro de un texto, `{{` empieza un valor insertado: para escribir llaves, usa `\\{{` y `\\}}`, como \"\\{{\\\"a\\\": 1\\}}\""),
        ("the number `{n}`", "el número `{n}`"),
        ("the number `{f}`", "el número `{f}`"),
        ("the name `{s}`", "el nombre `{s}`"),
        ("the symbol `:{s}`", "el símbolo `:{s}`"),
        ("the end of the line", "el final de la línea"),
        ("the end of the code", "el final del código"),
        ("the keyword `{k}`", "la palabra clave `{k}`"),
        // src/lsp/analysis.rs
        ("cannot read the file `{shown}`", "no se puede leer el archivo `{shown}`"),
        ("the file `{p}` has {n} error(s)", "el archivo `{p}` tiene {n} error(es)"),
        ("first: {}{place}", "el primero: {}{place}"),
        ("open `{p}` to see and fix them", "abre `{p}` para verlos y corregirlos"),
        // src/lsp/extras.rs
        ("▶ Run", "▶ Ejecutar"),
        ("▶ Run the {n} tests", "▶ Ejecutar las {n} pruebas"),
        ("▶ Run test", "▶ Ejecutar la prueba"),
        // src/lsp/features.rs
        (
            r#"
more: `rythme explain {}`"#,
            r#"
más: `rythme explain {}`"#,
        ),
        (
            r#"

Same as: {} (its Ruby names)"#,
            r#"

Igual que: {} (sus nombres de Ruby)"#,
        ),
        ("**{alias}** — the Ruby name of `{of}`: the same function", "**{alias}** — el nombre de Ruby de `{of}`: la misma función"),
        ("A value that is there, in an `Option`. The other case is `None`.", "Un valor que está ahí, en una `Option`. El otro caso es `None`."),
        ("No value, in an `Option`: what `find`, `first` or `get` give when there is nothing. Handle it with `match` or `unwrap_or`.", "Ningún valor, en una `Option`: lo que dan `find`, `first` o `get` cuando no hay nada. Trátalo con `match` o `unwrap_or`."),
        ("The result of an operation that succeeded, in a `Result`. The other case is `Err(error)`.", "El resultado de una operación que salió bien, en un `Result`. El otro caso es `Err(error)`."),
        ("The result of an operation that failed, in a `Result`. Inside a function, `?` stops and passes it on.", "El resultado de una operación que falló, en un `Result`. Dentro de una función, `?` se detiene y lo pasa hacia arriba."),
        ("**{word}** is not used in Rythme.", "**{word}** no se usa en Rythme."),
        ("{} more definitions", "{} definiciones más"),
        ("Inferred from its code:", "Deducido de su código:"),
        ("local function", "función local"),
        ("parameter of `{f}`", "parámetro de `{f}`"),
        ("parameter of an anonymous function", "parámetro de una función anónima"),
        ("local value", "valor local"),
        ("given by the case `{pattern}` of a `match`", "dado por el caso `{pattern}` de un `match`"),
        ("constructor of the type `{t}`", "constructor del tipo `{t}`"),
        ("*defined in `{name}`*", "*definido en `{name}`*"),
        ("The number π.", "El número π."),
        (
            r#"

`value.{n}(...)` calls `{n}(value, ...)`, or the field `{n}` of a record; `value.{n}` reads the field `{n}`, or calls `{n}(value)`"#,
            r#"

`valor.{n}(...)` llama a `{n}(valor, ...)`, o al campo `{n}` de un registro; `valor.{n}` lee el campo `{n}`, o llama a `{n}(valor)`"#,
        ),
        ("the file `{p}` is not found", "el archivo `{p}` no se encuentra"),
        (
            r#"

Its definitions are used through the name `{n}`: `{n}.name`"#,
            r#"

Sus definiciones se usan con el nombre `{n}`: `{n}.nombre`"#,
        ),
        ("Defines:", "Define:"),
        ("name", "nombre"),
        ("value", "valor"),
        ("other", "otro"),
        ("pattern", "patrón"),
        ("result", "resultado"),
        ("what it checks", "qué comprueba"),
        ("actual", "obtenido"),
        ("expected", "esperado"),
        ("condition", "condición"),
        ("file", "archivo"),
        ("a function on several lines", "una función en varias líneas"),
        ("a function on one line", "una función en una línea"),
        ("choose according to the shape of a value", "elegir según la forma de un valor"),
        ("a test, run by `rythme test`", "una prueba, ejecutada por `rythme test`"),
        ("a test on several lines", "una prueba en varias líneas"),
        ("choose between two values", "elegir entre dos valores"),
        ("type ${{1:Name}} = ${{2:First}}(${{3:field}}) | ${{0:Second}}", "type ${{1:Nombre}} = ${{2:Primero}}(${{3:campo}}) | ${{0:Segundo}}"),
        ("your own type", "tu propio tipo"),
        ("several steps inside an expression", "varios pasos dentro de una expresión"),
        ("a block: a function given as the last argument of a call", "un bloque: una función dada como último argumento de una llamada"),
        ("the opposite of `if`", "lo contrario de `if`"),
        ("use another file: `use geometry`, then `geometry.area(2)`", "usar otro archivo: `use geometria`, y luego `geometria.area(2)`"),
        ("same as {}", "igual que {}"),
        ("field", "campo"),
        ("package", "paquete"),
        ("folder", "carpeta"),
        ("Add `use {file}` and write `{stem}.{written}`", "Añadir `use {file}` y escribir `{stem}.{written}`"),
        ("Replace with `{new}`", "Reemplazar por `{new}`"),
        // src/lsp/mod.rs
        ("rythme lsp: unknown option `{other}` (ignored)", "rythme lsp: opción desconocida `{other}` (ignorada)"),
        ("rythme lsp is the language server: your editor starts it and talks to it. See `rythme help editors`. (Ctrl-C to quit)", "rythme lsp es el servidor de lenguaje: tu editor lo inicia y habla con él. Consulta `rythme help editors`. (Ctrl-C para salir)"),
        ("the server is not initialized yet", "el servidor aún no está inicializado"),
        ("the server is shutting down", "el servidor se está cerrando"),
        ("internal error in Rythme (please report it): {}", "error interno de Rythme (por favor, avísanos): {}"),
        ("not formatted: the code has an error ({first})", "sin formatear: el código tiene un error ({first})"),
        // src/lsp/refs.rs
        ("there is no name here", "aquí no hay ningún nombre"),
        ("the name of a test is a text: edit it directly", "el nombre de una prueba es un texto: edítalo directamente"),
        ("`{name}` cannot be found", "no se encuentra `{name}`"),
        ("`{name}` is a built-in function: it cannot be renamed", "`{name}` es una función integrada: no se puede renombrar"),
        ("`{name}` is part of Rythme: it cannot be renamed", "`{name}` forma parte de Rythme: no se puede renombrar"),
        ("`{name}` is not defined: there is nothing to rename", "`{name}` no está definido: no hay nada que renombrar"),
        ("`{new}` is not a valid name", "`{new}` no es un nombre válido"),
        ("`{new}` is a keyword", "`{new}` es una palabra clave"),
        ("the names of types and constructors start with an uppercase letter", "los nombres de tipos y de constructores empiezan por mayúscula"),
        ("names start with a lowercase letter (uppercase is for types and constructors)", "los nombres empiezan por minúscula (las mayúsculas son para los tipos y los constructores)"),
        ("`{new}` is a built-in function: choose another name", "`{new}` es una función integrada: elige otro nombre"),
        ("`{shown}` has a syntax error: fix it before renaming", "`{shown}` tiene un error de sintaxis: corrígelo antes de renombrar"),
        ("`{old_name}` is also the name of a record field, and `value.{old_name}(...)` may call that field: rename it by hand", "`{old_name}` también es el nombre de un campo de registro, y `valor.{old_name}(...)` puede llamar a ese campo: renómbralo a mano"),
        ("cannot rename `{old_name}` to `{new_name}`: {why}", "no se puede renombrar `{old_name}` a `{new_name}`: {why}"),
        ("some names would then refer to something else", "algunos nombres se referirían entonces a otra cosa"),
        ("the code would not be valid", "el código no sería válido"),
        // src/manifest.rs
        ("the section `[{name}]` is written twice", "la sección `[{name}]` está escrita dos veces"),
        ("put all the lines of `[{name}]` under one `[{name}]`", "pon todas las líneas de `[{name}]` bajo un solo `[{name}]`"),
        ("`{name}` is written both as `[{name}]` and as `[[{name}]]`", "`{name}` está escrito a la vez como `[{name}]` y como `[[{name}]]`"),
        ("expected a name here", "se esperaba un nombre aquí"),
        ("each line is `name = \"value\"`, like `version = \"0.1.0\"`", "cada línea es `nombre = \"valor\"`, como `version = \"0.1.0\"`"),
        ("this text is not closed", "este texto no está cerrado"),
        ("add `{quote}` at the end of the text", "añade `{quote}` al final del texto"),
        ("unknown escape in a text", "escape desconocido en un texto"),
        ("write `\\\\` for a backslash, or use `/` in paths: they work on every system", "escribe `\\\\` para una barra invertida, o usa `/` en las rutas: funcionan en todos los sistemas"),
        ("expected `=` after `{key}`", "se esperaba `=` después de `{key}`"),
        ("`{key}` is written twice", "`{key}` está escrito dos veces"),
        ("expected `,` or `}}` here", "se esperaba `,` o `}}` aquí"),
        ("a table is written on one line: `{{ git = \"...\", tag = \"...\" }}`", "una tabla se escribe en una línea: `{{ git = \"...\", tag = \"...\" }}`"),
        ("a value is a text, written in quotes", "un valor es un texto, escrito entre comillas"),
        ("write `\"{example}\"`", "escribe `\"{example}\"`"),
        ("expected `{close}` here", "se esperaba `{close}` aquí"),
        ("write `{key} = \"...\"`", "escribe `{key} = \"...\"`"),
        ("first written here", "escrito por primera vez aquí"),
        ("unexpected text after the value", "texto inesperado después del valor"),
        ("write one value per line; a comment starts with `#`", "escribe un valor por línea; un comentario empieza por `#`"),
        ("`{name}` cannot be the name of a package: it is used in `use {name}/file`, so it must be a Rythme name in lower case (letters, digits, `_`)", "`{name}` no puede ser el nombre de un paquete: se usa en `use {name}/archivo`, así que debe ser un nombre de Rythme en minúsculas (letras, cifras, `_`)"),
        ("`{key}` is {what}, written in quotes", "`{key}` es {what}, escrito entre comillas"),
        ("unknown key `{key}`", "clave desconocida `{key}`"),
        ("the keys here are {}", "las claves aquí son {}"),
        ("write `name = \"{b}\"`", "escribe `name = \"{b}\"`"),
        ("`{v}` is not a version", "`{v}` no es una versión"),
        ("a version is three numbers: `version = \"1.0.0\"` (major.minor.patch)", "una versión son tres números: `version = \"1.0.0\"` (mayor.menor.parche)"),
        ("`entry` is the path of a `.ry` file in the project", "`entry` es la ruta de un archivo `.ry` del proyecto"),
        ("like `entry = \"src/main.ry\"`", "como `entry = \"src/main.ry\"`"),
        ("unknown section `{n}`", "sección desconocida `{n}`"),
        ("write `name`, `version` and `entry` at the top of the file, without a section", "escribe `name`, `version` y `entry` al principio del archivo, sin sección"),
        ("the only section is `[dependencies]`", "la única sección es `[dependencies]`"),
        ("`{shown}` has no `name`", "`{shown}` no tiene `name`"),
        ("add a line like `name = \"hello\"` at the top", "añade una línea como `name = \"hola\"` al principio"),
        ("`{shown}` has no `version`", "`{shown}` no tiene `version`"),
        ("add a line like `version = \"0.1.0\"` at the top", "añade una línea como `version = \"0.1.0\"` al principio"),
        ("write `{name} = {{ git = \"https://...\", tag = \"v1.0.0\" }}` (or `commit = \"...\"`), or `{name} = {{ path = \"../folder\" }}`", "escribe `{name} = {{ git = \"https://...\", tag = \"v1.0.0\" }}` (o `commit = \"...\"`), o `{name} = {{ path = \"../carpeta\" }}`"),
        ("call it `{b}`", "llámala `{b}`"),
        ("the project `{own}` cannot depend on itself", "el proyecto `{own}` no puede depender de sí mismo"),
        ("`use {own}/file` already reaches its own files", "`use {own}/archivo` ya llega a sus propios archivos"),
        ("the dependency `{name}` is written as a table", "la dependencia `{name}` se escribe como una tabla"),
        ("the dependency `{name}` has both `git` and `path`", "la dependencia `{name}` tiene a la vez `git` y `path`"),
        ("the dependency `{name}` needs `git` or `path`", "la dependencia `{name}` necesita `git` o `path`"),
        ("`tag` and `commit` are for a dependency from git, not from a folder", "`tag` y `commit` son para una dependencia que viene de git, no de una carpeta"),
        ("`{c}` is not a commit", "`{c}` no es un commit"),
        ("give a `tag` or a `commit`, not both", "da un `tag` o un `commit`, no los dos"),
        ("the dependency `{name}` needs a fixed version: a `tag` or a `commit`", "la dependencia `{name}` necesita una versión fija: un `tag` o un `commit`"),
        ("the `git` address of `{name}` is empty", "la dirección `git` de `{name}` está vacía"),
        ("this file is written by `rythme install`: delete it and run `rythme install` again", "este archivo lo escribe `rythme install`: bórralo y vuelve a ejecutar `rythme install`"),
        ("unexpected line in `rythme.lock`", "línea inesperada en `rythme.lock`"),
        ("this entry of `rythme.lock` is not complete", "esta entrada de `rythme.lock` no está completa"),
        (
            r#"# The exact commit of each dependency from git, written by `rythme install`.
# Keep this file with the project, so that everyone gets the same code.
"#,
            r#"# El commit exacto de cada dependencia que viene de git, escrito por `rythme install`.
# Guarda este archivo con el proyecto, para que todo el mundo tenga el mismo código.
"#,
        ),
        // src/parser.rs
        ("a case of `match` gives one expression; for a result on several lines, write `do` after `->`, then the lines, then `end`:\n  Ok(x) -> do\n    y = x + 1\n    y * 2\n  end\nor move the lines into a function: `Ok(x) -> handle(x)`", "un caso de `match` da una sola expresión; para un resultado en varias líneas, escribe `do` después de `->`, luego las líneas, luego `end`:\n  Ok(x) -> do\n    y = x + 1\n    y * 2\n  end\no pon las líneas en una función: `Ok(x) -> tratar(x)`"),
        ("`{name}` is read as a value here, not called, because the code gives it a value (line {line}): to call it, put its argument in parentheses, like `{name}([1, 2])`", "`{name}` se lee aquí como un valor, no se llama, porque el código le da un valor (línea {line}): para llamarla, pon su argumento entre paréntesis, como `{name}([1, 2])`"),
        ("Rythme has no `condition ? a : b`", "Rythme no tiene `condición ? a : b`"),
        ("`?` does not choose between two values", "`?` no elige entre dos valores"),
        ("write `if condition then a else b`, like `if x > 2 then \"big\" else \"small\"`", "escribe `if condición then a else b`, como `if x > 2 then \"grande\" else \"pequeño\"`"),
        ("after a value, `?` takes it out of `Some(x)` or `Ok(x)` and stops on `None` or `Err(e)`; at the end of a name, it is part of the name (`even?`)", "después de un valor, `?` lo saca de `Some(x)` u `Ok(x)` y se detiene en `None` o `Err(e)`; al final de un nombre, forma parte del nombre (`even?`)"),
        ("all the definitions of `{name}` must have the same number of parameters", "todas las definiciones de `{name}` deben tener el mismo número de parámetros"),
        ("{} parameter(s) here", "{} parámetro(s) aquí"),
        ("expected {expected}, found {found}", "se esperaba {expected}, pero se encontró {found}"),
        ("expected {expected} here", "se esperaba {expected} aquí"),
        ("to compare two values, use `==`", "para comparar dos valores, usa `==`"),
        ("use `->` in `match` cases; a dict is built with `dict([[key, value], ...])`", "usa `->` en los casos de un `match`; un diccionario se construye con `dict([[clave, valor], ...])`"),
        ("use `and` to combine conditions", "usa `and` para combinar condiciones"),
        ("use `or` to combine conditions", "usa `or` para combinar condiciones"),
        ("use `not` to negate a condition: `not done`", "usa `not` para negar una condición: `not hecho`"),
        ("Rythme does not use braces for blocks: write `do` ... `end`, or put a function body on the following lines and finish with `end`", "Rythme no usa llaves para los bloques: escribe `do` ... `end`, o pon el cuerpo de una función en las líneas siguientes y termina con `end`"),
        ("Rythme does not use `:` to start a block; see `rythme help syntax`", "Rythme no usa `:` para empezar un bloque; consulta `rythme help syntax`"),
        ("`{word}` is not used in Rythme", "`{word}` no se usa en Rythme"),
        ("comes from another language", "viene de otro lenguaje"),
        ("unexpected here: {found}", "inesperado aquí: {found}"),
        ("the statement should end before this", "la instrucción debería terminar antes de esto"),
        ("to call a function, use parentheses: `{name}(...)`", "para llamar a una función, usa paréntesis: `{name}(...)`"),
        ("a block `do |x| ... end` is given to a call, on its line: `names.each do |name| print(name) end`", "un bloque `do |x| ... end` se da a una llamada, en su misma línea: `nombres.each do |nombre| print(nombre) end`"),
        ("`else` must follow `if condition then value` (an `if` branch holds one expression; use `do ... end` for several lines)", "`else` debe seguir a `if condición then valor` (una rama de `if` contiene una sola expresión; usa `do ... end` para varias líneas)"),
        ("put each statement on its own line", "pon cada instrucción en su propia línea"),
        ("`{word}` can only be used at the top level of a file", "`{word}` solo se puede usar en el nivel superior de un archivo"),
        ("move it outside of the function or block", "sácalo de la función o del bloque"),
        ("before `=` comes a name or a pattern, like `[first, ..rest]`, `{{name, ..}}` or `Some(x)`", "antes de `=` va un nombre o un patrón, como `[primero, ..resto]`, `{{nombre, ..}}` o `Some(x)`"),
        ("`=`", "`=`"),
        ("values cannot be changed in Rythme", "los valores no se pueden cambiar en Rythme"),
        ("a record pattern cannot give a name to the other fields", "un patrón de registro no puede dar nombre a los demás campos"),
        ("write `{{name, ..}}` to ignore the other fields; for a record without some fields, use `remove(record, \"name\")`", "escribe `{{nombre, ..}}` para ignorar los demás campos; para un registro sin algunos campos, usa `remove(registro, \"nombre\")`"),
        ("only a function can have a name that ends with `?`", "solo una función puede tener un nombre que termine en `?`"),
        ("define it with `fn`: `fn {s}(x) = ...`; for a value, use a name without `?`", "defínela con `fn`: `fn {s}(x) = ...`; para un valor, usa un nombre sin `?`"),
        ("invalid `..` in this pattern", "`..` no válido en este patrón"),
        ("`..name` must be last: `[first, ..rest]`", "`..nombre` debe ir al final: `[primero, ..resto]`"),
        ("cannot give a value to this", "no se puede dar un valor a esto"),
        ("this is not a name", "esto no es un nombre"),
        ("`=` gives a name to a value", "`=` da un nombre a un valor"),
        ("to define a function, write `fn name(x) = ...`", "para definir una función, escribe `fn nombre(x) = ...`"),
        ("records cannot be changed; build a new one: `new = {{..old, {name}: value}}`", "los registros no se pueden cambiar; construye uno nuevo: `nuevo = {{..viejo, {name}: valor}}`"),
        ("values cannot be changed; build a new one: `set_at(list, i, x)` for a list, `put(record, name, x)` for a record", "los valores no se pueden cambiar; construye uno nuevo: `set_at(lista, i, x)` para una lista, `put(registro, nombre, x)` para un registro"),
        ("a function name", "un nombre de función"),
        ("`(` and the parameters", "`(` y los parámetros"),
        ("even without parameters, write the parentheses: `fn greet() = print(\"hi\")`", "incluso sin parámetros, escribe los paréntesis: `fn saludar() = print(\"hola\")`"),
        ("functions use `=`, not `{arrow}`", "las funciones usan `=`, no `{arrow}`"),
        ("write `fn(x) = x * 2`", "escribe `fn(x) = x * 2`"),
        ("`=` or a new line", "`=` o una línea nueva"),
        (
            r#"a function is written `fn name(x) = expression`,
or with its body on the following lines, finished by `end`"#,
            r#"una función se escribe `fn nombre(x) = expresión`,
o con su cuerpo en las líneas siguientes, terminado con `end`"#,
        ),
        ("type names start with an uppercase letter", "los nombres de tipo empiezan por mayúscula"),
        ("write `type {cap} = ...`", "escribe `type {cap} = ...`"),
        ("a type name", "un nombre de tipo"),
        ("example: `type Shape = Circle(radius) | Square(side)`", "ejemplo: `type Forma = Circulo(radio) | Cuadrado(lado)`"),
        ("variant names start with an uppercase letter", "los nombres de variante empiezan por mayúscula"),
        ("write `{cap}`", "escribe `{cap}`"),
        ("a variant name, like `Circle`", "un nombre de variante, como `Circulo`"),
        ("a field name", "un nombre de campo"),
        ("fields are named in lowercase: `Circle(radius)`", "los campos se nombran en minúsculas: `Circulo(radio)`"),
        ("the name of the test, as a text", "el nombre de la prueba, como texto"),
        ("example: `test \"addition works\" = assert_eq(1 + 1, 2)`", "ejemplo: `test \"la suma funciona\" = assert_eq(1 + 1, 2)`"),
        ("write the name of the file without `.ry`", "escribe el nombre del archivo sin `.ry`"),
        ("example: `use {path}`", "ejemplo: `use {path}`"),
        ("a name for the file, in lowercase", "un nombre para el archivo, en minúsculas"),
        ("example: `use geometry as geo`, then `geo.area(2)`", "ejemplo: `use geometria as geo`, y luego `geo.area(2)`"),
        ("the name of a file", "el nombre de un archivo"),
        ("example: `use geometry` for the file `geometry.ry`, then `geometry.area(2)`", "ejemplo: `use geometria` para el archivo `geometria.ry`, y luego `geometria.area(2)`"),
        ("for a file whose name starts with an uppercase letter, write its path as a text: `use \"Shapes.ry\" as shapes`", "para un archivo cuyo nombre empieza por mayúscula, escribe su ruta como texto: `use \"Formas.ry\" as formas`"),
        ("the end of the expression", "el final de la expresión"),
        ("comparisons cannot be chained", "las comparaciones no se pueden encadenar"),
        ("combine them with `and`: `a < b and b < c`", "combínalas con `and`: `a < b and b < c`"),
        ("`{sym}` cannot follow a `|>` step directly", "`{sym}` no puede seguir directamente a un paso `|>`"),
        (
            r#"`|>` comes after the calculations: put the pipeline in parentheses,
like `(x |> f) {sym} 1`, or continue it with a function: `x |> f |> fn(v) = v {sym} 1`"#,
            r#"`|>` va después de los cálculos: pon la cadena entre paréntesis,
como `(x |> f) {sym} 1`, o continúala con una función: `x |> f |> fn(v) = v {sym} 1`"#,
        ),
        ("a block must follow a call", "un bloque debe seguir a una llamada"),
        ("not given to a function", "no se da a ninguna función"),
        (
            r#"write the block on the line of the call it is given to: `xs.map do |x| x * 2 end`;
for a function as a value, write `fn(x) = x * 2`"#,
            r#"escribe el bloque en la línea de la llamada a la que se da: `xs.map do |x| x * 2 end`;
para una función como valor, escribe `fn(x) = x * 2`"#,
        ),
        ("`,` or `|`", "`,` o `|`"),
        ("the parameters of a block go between bars: `do |name, age| ... end`", "los parámetros de un bloque van entre barras: `do |nombre, edad| ... end`"),
        ("`}}`", "`}}`"),
        ("the block starts here", "el bloque empieza aquí"),
        ("a field or function name after `.`", "un nombre de campo o de función después de `.`"),
        ("an expression", "una expresión"),
        ("`]`", "`]`"),
        ("`,` or `{closing}`", "`,` o `{closing}`"),
        ("separate the elements with commas", "separa los elementos con comas"),
        ("this code is nested too deeply (more than {MAX_NESTING} levels)", "este código está anidado demasiado (más de {MAX_NESTING} niveles)"),
        ("split it into several named values or functions", "divídelo en varios valores o funciones con nombre"),
        ("this expression is too long or nested too deeply (more than {MAX_DEPTH} levels)", "esta expresión es demasiado larga o está anidada demasiado (más de {MAX_DEPTH} niveles)"),
        (
            r#"split it into several named values or functions;
to combine many values, put them in a list: `sum([a, b, c])`, `join([x, y, z])`"#,
            r#"divídela en varios valores o funciones con nombre;
para combinar muchos valores, ponlos en una lista: `sum([a, b, c])`, `join([x, y, z])`"#,
        ),
        ("Rythme has no tuples", "Rythme no tiene tuplas"),
        ("group values in a list `[a, b]` or a record `{{x: a, y: b}}`", "agrupa los valores en una lista `[a, b]` o un registro `{{x: a, y: b}}`"),
        ("`)`", "`)`"),
        ("a named function cannot be used inside an expression", "una función con nombre no se puede usar dentro de una expresión"),
        ("define it on its own line, or use an anonymous function: `fn(x) = x * 2`", "defínela en su propia línea, o usa una función anónima: `fn(x) = x * 2`"),
        ("comments start with `#`", "los comentarios empiezan por `#`"),
        ("`else` must follow `if condition then value`; a branch holds a single expression (use `do ... end` for several lines)", "`else` debe seguir a `if condición then valor`; una rama contiene una sola expresión (usa `do ... end` para varias líneas)"),
        ("a record is written `{{name: \"Ada\", age: 36}}`; for a block of code, use `do ... end`", "un registro se escribe `{{nombre: \"Ada\", edad: 36}}`; para un bloque de código, usa `do ... end`"),
        ("`:` and a value", "`:` y un valor"),
        ("`then`", "`then`"),
        ("an `{word}` is written: `{word} condition then value else other_value`", "un `{word}` se escribe: `{word} condición then valor else otro_valor`"),
        ("this `{word}` has no `else`", "este `{word}` no tiene `else`"),
        ("every `{word}` must have an `else`", "todo `{word}` debe tener un `else`"),
        ("expected `else` here", "se esperaba `else` aquí"),
        ("write `else if`", "escribe `else if`"),
        (
            r#"in Rythme, `unless` always gives a value, so it needs both cases:
  unless age >= 18 then "minor" else "adult""#,
            r#"en Rythme, `unless` siempre da un valor, así que necesita los dos casos:
  unless edad >= 18 then "menor" else "adulto""#,
        ),
        (
            r#"in Rythme, `if` always gives a value, so it needs both cases:
  if age >= 18 then "adult" else "minor""#,
            r#"en Rythme, `if` siempre da un valor, así que necesita los dos casos:
  if edad >= 18 then "adulto" else "menor""#,
        ),
        ("a branch holds a single expression; for several lines, use `do ... end`", "una rama contiene una sola expresión; para varias líneas, usa `do ... end`"),
        ("a new line", "una línea nueva"),
        (
            r#"write each case on its own line, then `end`:
  match x
    0 -> "zero"
    _ -> "other"
  end"#,
            r#"escribe cada caso en su propia línea, y luego `end`:
  match x
    0 -> "cero"
    _ -> "otro"
  end"#,
        ),
        ("no `|` is needed before a case", "no hace falta `|` antes de un caso"),
        ("write each case as `pattern -> result`", "escribe cada caso como `patrón -> resultado`"),
        ("`->`", "`->`"),
        ("each case is written `pattern -> result`", "cada caso se escribe `patrón -> resultado`"),
        ("put each case on its own line", "pon cada caso en su propia línea"),
        ("a `match` needs at least one case", "un `match` necesita al menos un caso"),
        ("a number after `..`", "un número después de `..`"),
        ("only a text can come before `+` in a pattern", "solo un texto puede ir antes de `+` en un patrón"),
        ("to look at the end of a text, use a guard: `s if ends_with(s, \"!\") -> ...`", "para mirar el final de un texto, usa una condición: `s if ends_with(s, \"!\") -> ...`"),
        ("after `+`, a pattern takes a name or `_`", "después de `+`, un patrón lleva un nombre o `_`"),
        ("a pattern cannot contain `{{...}}`", "un patrón no puede contener `{{...}}`"),
        ("use a guard instead: `s if s == \"...\" -> ...`", "usa una condición en su lugar: `s if s == \"...\" -> ...`"),
        ("`..rest` must be the last element: `[first, ..rest]`", "`..resto` debe ser el último elemento: `[primero, ..resto]`"),
        ("`:` and a pattern", "`:` y un patrón"),
        ("a pattern", "un patrón"),
        ("patterns look like: `x`, `_`, `0`, `\"text\"`, `:ok`, `Some(x)`, `[first, ..rest]`, `{{name}}`", "los patrones son como: `x`, `_`, `0`, `\"texto\"`, `:ok`, `Some(x)`, `[primero, ..resto]`, `{{nombre}}`"),
        ("the ends of a range in a pattern are whole numbers of 64 bits at most", "los extremos de un intervalo en un patrón son números enteros de 64 bits como máximo"),
        ("too large for a range", "demasiado grande para un intervalo"),
        ("compare with a guard instead: `n if n >= low and n <= high -> ...`; a range still matches large numbers between its ends, and a decimal end like `1.0e30` works too", "compara mejor con una condición: `n if n >= bajo and n <= alto -> ...`; un intervalo sigue aceptando los números grandes entre sus extremos, y un extremo decimal como `1.0e30` también funciona"),
        ("a range in a pattern goes from a number to a number", "un intervalo en un patrón va de un número a un número"),
        ("not a number", "no es un número"),
        ("the arguments of a call without parentheses are separated by commas: `print a, b`", "los argumentos de una llamada sin paréntesis se separan con comas: `print a, b`"),
        ("write the numbers themselves, like `1..9` or `0.0..0.5`; to compare with names, use a guard: `n if n >= low and n <= high -> ...`", "escribe los números mismos, como `1..9` o `0.0..0.5`; para comparar con nombres, usa una condición: `n if n >= bajo and n <= alto -> ...`"),
        ("a text that starts with a text is written `\"Mr \" + name`: `name` is the rest of the text", "un texto que empieza por un texto se escribe `\"Sr. \" + nombre`: `nombre` es el resto del texto"),
        ("missing `end` to close this `{what}`", "falta `end` para cerrar este `{what}`"),
        ("this `{what}` is never closed", "este `{what}` nunca se cierra"),
        ("the code ends here", "el código termina aquí"),
        ("add `end` on its own line after the last line of the block", "añade `end` en su propia línea después de la última línea del bloque"),
        ("values never change in Rythme: give the new value a new name, like `total2 = total + 1`", "los valores nunca cambian en Rythme: da un nombre nuevo al valor nuevo, como `total2 = total + 1`"),
        // src/project.rs
        ("no folder to keep the packages in", "no hay ninguna carpeta donde guardar los paquetes"),
        ("set the variable RYTHME_HOME to a folder, like `RYTHME_HOME=~/.rythme`", "da una carpeta a la variable RYTHME_HOME, como `RYTHME_HOME=~/.rythme`"),
        ("the folder `{p}`", "la carpeta `{p}`"),
        ("run `rythme install` here, in the folder of the project", "ejecuta `rythme install` aquí, en la carpeta del proyecto"),
        ("run `rythme install` in `{root}`, the folder of the project", "ejecuta `rythme install` en `{root}`, la carpeta del proyecto"),
        ("the folder `{p}` of the package `{name}` does not exist", "la carpeta `{p}` del paquete `{name}` no existe"),
        ("fix its `path` in `rythme.toml`", "corrige su `path` en `rythme.toml`"),
        ("the package `{name}` is not installed", "el paquete `{name}` no está instalado"),
        ("`rythme.toml` asks for it from {}", "`rythme.toml` lo pide desde {}"),
        ("`{shown_dir}` is not a Rythme package: it has no `rythme.toml`", "`{shown_dir}` no es un paquete de Rythme: no tiene `rythme.toml`"),
        ("use one of its files: `use {package}/file`", "usa uno de sus archivos: `use {package}/archivo`"),
        ("the package `{package}` has no file `{entry}`", "el paquete `{package}` no tiene el archivo `{entry}`"),
        ("the package `{package}` has no file `{wanted}`", "el paquete `{package}` no tiene el archivo `{wanted}`"),
        ("did you mean `use {prefix}{best}`?", "¿quisiste decir `use {prefix}{best}`?"),
        ("its files there are: {}", "sus archivos ahí son: {}"),
        ("did you mean `use {best}/{rest}`?", "¿quisiste decir `use {best}/{rest}`?"),
        ("in this project, `use` also reaches the files of the packages {}", "en este proyecto, `use` también llega a los archivos de los paquetes {}"),
        ("the program `git` was not found", "no se encontró el programa `git`"),
        ("Rythme uses git to download the dependencies that come from git", "Rythme usa git para descargar las dependencias que vienen de git"),
        ("install Git (https://git-scm.com), then run `rythme install` again; dependencies with `path` do not need it", "instala Git (https://git-scm.com), y luego vuelve a ejecutar `rythme install`; las dependencias con `path` no lo necesitan"),
        ("git says: {said}", "git dice: {said}"),
        ("the tag `{t}` of `{name}` does not exist in `{url}`", "el tag `{t}` de `{name}` no existe en `{url}`"),
        ("the commit `{c}` of `{name}` does not exist in `{url}`", "el commit `{c}` de `{name}` no existe en `{url}`"),
        ("did you mean `tag = \"{best}\"`?", "¿quisiste decir `tag = \"{best}\"`?"),
        ("its tags are: {}", "sus tags son: {}"),
        ("it has no tags: give a `commit` instead", "no tiene tags: da un `commit` en su lugar"),
        ("cannot write in `{}`: {e}", "no se puede escribir en `{}`: {e}"),
        ("cannot download `{name}` from `{url}`", "no se puede descargar `{name}` desde `{url}`"),
        ("cannot take the commit `{commit}` of `{name}`", "no se puede tomar el commit `{commit}` de `{name}`"),
        ("no `rythme.toml` in this folder or above it", "no hay ningún `rythme.toml` en esta carpeta ni en las de encima"),
        ("go to the folder of a project, or create one with `rythme new NAME`", "ve a la carpeta de un proyecto, o crea uno con `rythme new NOMBRE`"),
        (
            r#"`rythme install` takes no argument (`{a}`): it installs the dependencies of `rythme.toml`
see `rythme help projects`"#,
            r#"`rythme install` no recibe argumentos (`{a}`): instala las dependencias de `rythme.toml`
consulta `rythme help projects`"#,
        ),
        ("`{name}` has no dependencies: add them under `[dependencies]` in `rythme.toml`", "`{name}` no tiene dependencias: añádelas bajo `[dependencies]` en `rythme.toml`"),
        ("Installing the dependencies of `{name}`", "Instalando las dependencias de `{name}`"),
        ("the folder `{p}` of the package `{dep_name}` does not exist", "la carpeta `{p}` del paquete `{dep_name}` no existe"),
        ("folder {p}", "carpeta {p}"),
        ("downloaded", "descargado"),
        ("already there", "ya estaba"),
        ("`{dep_name}` is not a Rythme package: `{shown_dir}` has no `rythme.toml`", "`{dep_name}` no es un paquete de Rythme: `{shown_dir}` no tiene `rythme.toml`"),
        ("a package is a project made by `rythme new`: its `rythme.toml`, and its files in `src/`", "un paquete es un proyecto creado con `rythme new`: su `rythme.toml`, y sus archivos en `src/`"),
        ("  ✓ {dep_name} {version}: {line}", "  ✓ {dep_name} {version}: {line}"),
        ("cannot write `{}`: {e}", "no se puede escribir `{}`: {e}"),
        ("{count} package(s) ready", "{count} paquete(s) listo(s)"),
        ("{count} package(s) ready ({more} needed by the other ones)", "{count} paquete(s) listo(s) ({more} pedido(s) por los demás)"),
        ("which name? Example: rythme new hello", "¿qué nombre? Ejemplo: rythme new hola"),
        ("`rythme new` takes one name, like `rythme new hello`", "`rythme new` recibe un solo nombre, como `rythme new hola`"),
        ("try `rythme new {b}`", "prueba `rythme new {b}`"),
        ("`{shown_dir}` already exists", "`{shown_dir}` ya existe"),
        ("choose another name, or another folder", "elige otro nombre, u otra carpeta"),
        ("Created the project `{name}`:", "Proyecto `{name}` creado:"),
        ("Next: `cd {shown_dir}`, then `rythme run` to run it and `rythme test` to test it (see `rythme help projects`)", "Siguiente paso: `cd {shown_dir}`, y luego `rythme run` para ejecutarlo y `rythme test` para probarlo (consulta `rythme help projects`)"),
        ("greeting", "saludo"),
        ("hello", "hola"),
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
            r#"# El proyecto: `rythme run`, `rythme test` y `rythme check` en esta carpeta.
name = "{name}"
version = "0.1.0"
entry = "src/main.ry"

[dependencies]
# Desde git, en un tag o un commit, y luego `rythme install`:
# mathlib = {{ git = "https://github.com/alguien/mathlib", tag = "v1.0.0" }}
# Desde una carpeta:
# herramientas = {{ path = "../herramientas" }}
"#,
        ),
        (
            r#"# The program: `rythme run` in the folder of the project.
use {module}

print({module}.{func}("World"))
"#,
            r#"# El programa: `rythme run` en la carpeta del proyecto.
use {module}

print({module}.{func}("mundo"))
"#,
        ),
        (
            r#"## A greeting for `name`.
fn {func}(name) = "Hello, {{name}}!"
"#,
            r#"## Un saludo para `nombre`.
fn {func}(nombre) = "¡Hola, {{nombre}}!"
"#,
        ),
        (
            r#"# The tests: `rythme test` in the folder of the project.
use {name}/{module}

test "{func}" = assert_eq({module}.{func}("Ada"), "Hello, Ada!")
"#,
            r#"# Las pruebas: `rythme test` en la carpeta del proyecto.
use {name}/{module}

test "{func}" = assert_eq({module}.{func}("Ada"), "¡Hola, Ada!")
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

Un proyecto [Rythme](https://github.com/maxlestage/Rythme).

```sh
rythme run        # ejecutar el programa (src/main.ry)
rythme test       # ejecutar las pruebas (tests/)
rythme check      # encontrar problemas sin ejecutar
rythme fmt        # dar al código su formato oficial
rythme install    # descargar las dependencias de rythme.toml
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
            r#"# Los archivos que git no guarda (guarda rythme.lock).
.DS_Store
Thumbs.db
*.swp
*~
"#,
        ),
        ("src/greeting.ry", "src/saludo.ry"),
        ("tests/greeting_test.ry", "tests/saludo_test.ry"),
        // src/repl.rs
        ("usage: :explain CODE, like :explain E003   (codes: {})", "uso: :explain CÓDIGO, como :explain E003   (códigos: {})"),
        (":help for help, :quit to leave", ":help para la ayuda, :quit para salir"),
        ("(type :quit to leave)", "(escribe :quit para salir)"),
        ("(cancelled)", "(cancelado)"),
        ("cannot read what is typed: {e}", "no se puede leer lo que se escribe: {e}"),
        ("usage: :doc NAME (a function, a topic...)", "uso: :doc NOMBRE (una función, un tema...)"),
        ("usage: :load file.ry", "uso: :load archivo.ry"),
        ("{} loaded", "{} cargado"),
        ("Everything was forgotten.", "Todo se ha olvidado."),
        ("Messages are now in English.", "Los mensajes ahora están en español."),
        ("usage: :lang fr  (or another of {codes})", "uso: :lang es  (u otro de {codes})"),
        ("usage: :steps EXPRESSION, like :steps fib(4)", "uso: :steps EXPRESIÓN, como :steps fib(4)"),
        ("Nothing defined yet.", "Aún no hay nada definido."),
        ("unknown command `{other}`", "comando desconocido `{other}`"),
        (" — did you mean `{best}`?", " — ¿quisiste decir `{best}`?"),
        (":help lists the commands", ":help lista los comandos"),
        // src/steps.rs
        ("unknown option `{flag}` for `rythme steps`", "opción desconocida `{flag}` para `rythme steps`"),
        ("\nthe options are: --max N, --width N (see `rythme help steps`)", "\nlas opciones son: --max N, --width N (ver `rythme help steps`)"),
        ("  ... (more steps, not shown: {n} are shown at most, change it with --max)", "  ... (más pasos, no mostrados: se muestran {n} como máximo, cámbialo con --max)"),
        ("this calculation became too deep to be shown step by step (more than {MAX_DEPTH} operations inside each other)", "este cálculo se volvió demasiado profundo para mostrarlo paso a paso (más de {MAX_DEPTH} operaciones unas dentro de otras)"),
        ("try it with smaller values, or run the program normally: `rythme file.ry`", "pruébalo con valores más pequeños, o ejecuta el programa normalmente: `rythme archivo.ry`"),
        ("the evaluation was stopped after {limit} steps: does it ever end?", "la evaluación se detuvo tras {limit} pasos: ¿termina alguna vez?"),
        ("a function that calls itself must always reach a case where it stops", "una función que se llama a sí misma siempre debe llegar a un caso en el que se detiene"),
        ("Nothing to show: this file only has definitions. Give an expression to compute, like: rythme steps file.ry \"fib(5)\"", "Nada que mostrar: este archivo solo tiene definiciones. Da una expresión que calcular, como: rythme steps archivo.ry \"fib(5)\""),
        ("`{flag}` needs a number, like `--max 50` or `--width 80` (20 at least)", "`{flag}` necesita un número, como `--max 50` o `--width 80` (20 como mínimo)"),
        (
            r#"usage: rythme steps file.ry [expression] [--max N] [--width N]
example: rythme steps fibonacci.ry "fib(4)""#,
            r#"uso: rythme steps archivo.ry [expresión] [--max N] [--width N]
ejemplo: rythme steps fibonacci.ry "fib(4)""#,
        ),
        // src/suggest.rs
        ("to join two lists or two texts, use `+`: `[1, 2] + [3]`; for a list of lists, use `flatten`", "para juntar dos listas o dos textos, usa `+`: `[1, 2] + [3]`; para una lista de listas, usa `flatten`"),
        ("in Rythme, this is called `{rythme}`", "en Rythme, esto se llama `{rythme}`"),
        ("booleans are written in lowercase: `true`", "los booleanos se escriben en minúsculas: `true`"),
        ("booleans are written in lowercase: `false`", "los booleanos se escriben en minúsculas: `false`"),
        ("Rythme has no null: use `None` for \"no value\" and `Some(x)` for a value", "Rythme no tiene null: usa `None` para «ningún valor» y `Some(x)` para un valor"),
        ("in Rythme, this is called `Some`", "en Rythme, esto se llama `Some`"),
        ("in Rythme, an error value is written `Err(...)`", "en Rythme, un valor de error se escribe `Err(...)`"),
        ("in Rythme, a success value is written `Ok(...)`", "en Rythme, un valor de éxito se escribe `Ok(...)`"),
        ("Rythme has no objects: functions receive their data as arguments, like `area(shape)` or `shape.area()`", "Rythme no tiene objetos: las funciones reciben sus datos como argumentos, como `area(forma)` o `forma.area()`"),
        ("write a range with `..`: `1..10` is the list from 1 to 10 (included), and `0..10 |> step_by(2)` counts by 2", "escribe un intervalo con `..`: `1..10` es la lista del 1 al 10 (incluido), y `0..10 |> step_by(2)` cuenta de 2 en 2"),
        ("use the operator `//` for a whole division: `7 // 2` is 3", "usa el operador `//` para una división entera: `7 // 2` es 3"),
        ("use the operator `%` for the remainder: `7 % 3` is 1", "usa el operador `%` para el resto: `7 % 3` es 1"),
        ("use the operators `//` and `%`: `7 // 2` is 3 and `7 % 2` is 1", "usa los operadores `//` y `%`: `7 // 2` es 3 y `7 % 2` es 1"),
        ("to put values in a text, use braces: \"total: {{x}}\"; to write a number with 2 decimals: `text(x, 2)`", "para poner valores en un texto, usa llaves: \"total: {{x}}\"; para escribir un número con 2 decimales: `text(x, 2)`"),
        ("read the whole file with `read_file(path)`, then cut it with `lines(text)`; the lines of the standard input are `input_lines()`", "lee el archivo entero con `read_file(ruta)`, y luego córtalo con `lines(texto)`; las líneas de la entrada estándar son `input_lines()`"),
        ("to merge two records, write `{{..a, ..b}}`: the fields of `b` replace those of `a`; for two dicts or two sets, `union(a, b)`", "para fusionar dos registros, escribe `{{..a, ..b}}`: los campos de `b` reemplazan a los de `a`; para dos diccionarios o dos conjuntos, `union(a, b)`"),
        ("write `unwrap_or(get(x, key), default)`: the value, or the default if it is missing", "escribe `unwrap_or(get(x, clave), por_defecto)`: el valor, o el valor por defecto si falta"),
        ("pair the lists, then combine each pair: `zip(a, b) |> map(fn([x, y]) = x + y)`", "empareja las listas, y luego combina cada par: `zip(a, b) |> map(fn([x, y]) = x + y)`"),
        ("sort from the largest to the smallest with `sort(list) |> reverse`", "ordena del más grande al más pequeño con `sort(lista) |> reverse`"),
        ("compute the average with `sum(list) / len(list)`", "calcula la media con `sum(lista) / len(lista)`"),
        ("keep x between low and high with `max(low, min(x, high))`", "mantén x entre bajo y alto con `max(bajo, min(x, alto))`"),
        ("count how many times each value appears with `group_by(list, fn(x) = x) |> map_values(len)`", "cuenta cuántas veces aparece cada valor con `group_by(lista, fn(x) = x) |> map_values(len)`"),
        ("a decimal number x is NaN (not a number) when `x != x`", "un número decimal x es NaN (no es un número) cuando `x != x`"),
        ("`n.times do |i| ... end` is written `(0..n - 1).each do |i| ... end`", "`n.times do |i| ... end` se escribe `(0..n - 1).each do |i| ... end`"),
        ("to use another file, write `use other_file`, then `other_file.name`", "para usar otro archivo, escribe `use otro_archivo`, y luego `otro_archivo.nombre`"),
        ("there is no `new`: build values directly, like `{{name: \"Ada\"}}` or `Circle(2.0)`", "no hay `new`: construye los valores directamente, como `{{nombre: \"Ada\"}}` o `Circulo(2.0)`"),
        ("values never change in Rythme: build a new value instead", "los valores nunca cambian en Rythme: construye un valor nuevo en su lugar"),
        (
            r#"Rythme has no loops. Work on the whole list instead:
  1..5 |> each(fn(i) = print(i))
  [1, 2, 3] |> map(fn(x) = x * 2)
or write a recursive function. See `rythme help functional`."#,
            r#"Rythme no tiene bucles. Trabaja con la lista entera en su lugar:
  1..5 |> each(fn(i) = print(i))
  [1, 2, 3] |> map(fn(x) = x * 2)
o escribe una función recursiva. Consulta `rythme help functional`."#,
        ),
        ("Rythme has no loops to interrupt: use `find`, `take`, `filter`, or a recursive function", "Rythme no tiene bucles que interrumpir: usa `find`, `take`, `filter`, o una función recursiva"),
        (
            r#"no `return` needed: the last expression of a function is its result.
To stop early with an error, return `Err(...)` and use `?`."#,
            r#"no hace falta `return`: la última expresión de una función es su resultado.
Para parar antes con un error, devuelve `Err(...)` y usa `?`."#,
        ),
        ("just write `name = value` (values never change, so no keyword is needed)", "escribe simplemente `nombre = valor` (los valores nunca cambian, así que no hace falta ninguna palabra clave)"),
        ("functions are written `fn name(x) = x * 2`, and anonymous ones `fn(x) = x * 2`", "las funciones se escriben `fn nombre(x) = x * 2`, y las anónimas `fn(x) = x * 2`"),
        (
            r#"Rythme has no classes. Describe your data with `type`:
  type Shape = Circle(radius) | Square(side)
or use a record: {{name: "Ada", age: 36}}"#,
            r#"Rythme no tiene clases. Describe tus datos con `type`:
  type Forma = Circulo(radio) | Cuadrado(lado)
o usa un registro: {{nombre: "Ada", edad: 36}}"#,
        ),
        (
            r#"use `match`:
  match value
    0 -> "zero"
    _ -> "other"
  end"#,
            r#"usa `match`:
  match valor
    0 -> "cero"
    _ -> "otro"
  end"#,
        ),
        (
            r#"errors are values in Rythme: return `Ok(value)` or `Err(message)`,
then handle them with `match` or pass them on with `?`. `fail(message)` stops the program."#,
            r#"los errores son valores en Rythme: devuelve `Ok(valor)` o `Err(mensaje)`,
y luego trátalos con `match` o pásalos hacia arriba con `?`. `fail(mensaje)` detiene el programa."#,
        ),
        // src/testing.rs
        (".", ","),
        ("the test returned `false`", "la prueba devolvió `false`"),
        ("use `assert_eq(actual, expected)` to see the values", "usa `assert_eq(obtenido, esperado)` para ver los valores"),
        ("the test ended with {}", "la prueba terminó con {}"),
        ("a test passes when it ends without error, and its value is not `false`, `Err(...)` or `None`", "una prueba pasa cuando termina sin error, y su valor no es `false`, `Err(...)` ni `None`"),
        (
            r#"No tests found. Write one like this:
  test "addition" = assert_eq(1 + 1, 2)"#,
            r#"No se encontró ninguna prueba. Escribe una así:
  test "suma" = assert_eq(1 + 1, 2)"#,
        ),
        ("Running {n} test(s)", "Ejecutando {n} prueba(s)"),
        ("Running {n} of {total} test(s)", "Ejecutando {n} de {total} prueba(s)"),
        ("Running {n} test(s) of {file}", "Ejecutando {n} prueba(s) de {file}"),
        ("Running {n} of {total} test(s) of {file}", "Ejecutando {n} de {total} prueba(s) de {file}"),
        ("{f} file(s), ", "{f} archivo(s), "),
        ("{total} test(s): {passed} passed, {failed} failed", "{total} prueba(s): {passed} pasada(s), {failed} fallida(s)"),
        (", {left_out} left out by the filter", ", {left_out} descartada(s) por el filtro"),
        (", {broken} file(s) with errors", ", {broken} archivo(s) con errores"),
        ("No test name contains {}.", "Ningún nombre de prueba contiene {}."),
        (" or ", " o "),
        ("{red}✗{reset} {}: the file has errors, so its tests could not run", "{red}✗{reset} {}: el archivo tiene errores, así que sus pruebas no se pudieron ejecutar"),
        (
            r#"No file with tests in {}. Write a test in a `.ry` file like this:
  test "addition" = assert_eq(1 + 1, 2)"#,
            r#"No hay ningún archivo con pruebas en {}. Escribe una prueba en un archivo `.ry` así:
  test "suma" = assert_eq(1 + 1, 2)"#,
        ),
        ("Failed tests:", "Pruebas fallidas:"),
        ("`{name}` needs {what}", "`{name}` necesita {what}"),
        ("a part of the name of a test", "una parte del nombre de una prueba"),
        ("a number, or `random`", "un número, o `random`"),
        ("the seed must be a whole number of 0 or more, or `random`, not `{v}`", "la semilla debe ser un número entero igual o mayor que 0, o `random`, no `{v}`"),
        ("unknown option `{name}` for `rythme test`", "opción desconocida `{name}` para `rythme test`"),
        (
            r#"
the options are: --filter NAME, --seed N, --watch (see `rythme help tests`)"#,
            r#"
las opciones son: --filter NOMBRE, --seed N, --watch (consulta `rythme help tests`)"#,
        ),
        ("which file or folder? Example: rythme test my_tests.ry, or rythme test . for the files of this folder", "¿qué archivo o carpeta? Ejemplo: rythme test mis_pruebas.ry, o rythme test . para los archivos de esta carpeta"),
        ("Seed: {}", "Semilla: {}"),
        (
            r#"
Watching {} file(s): the tests run again when one changes (Ctrl+C to stop)."#,
            r#"
Vigilando {} archivo(s): las pruebas se vuelven a ejecutar cuando uno cambia (Ctrl+C para parar)."#,
        ),
        (
            r#"
{} changed: running the tests again
"#,
            r#"
{} cambió: ejecutando de nuevo las pruebas
"#,
        ),
        // src/value.rs
        ("constructor", "constructor"),
        // playground/src/lib.rs
        ("✓ no problems found", "✓ no se encontró ningún problema"),
        ("✓ no errors, {n} warning(s)", "✓ sin errores, {n} advertencia(s)"),
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
            r#"RYTHME EN UN MINUTO

Rythme es un lenguaje sencillo en el que los valores nunca cambian y todo es
una función. Los programas son archivos que terminan en .ry.

  # Un comentario
  nombre = "mundo"               # un nombre para un valor (nunca cambia)
  print("¡Hola, {nombre}!")      # {...} inserta un valor en un texto

  fn cuadrado(x) = x * x         # una función
  print(cuadrado(4))             # 16

  [1, 2, 3, 4]
    |> filter(is_even)           # quedarse con los números pares
    |> map(cuadrado)             # elevar cada uno al cuadrado
    |> sum                       # sumarlos
    |> print                     # 20

Ejecútalo:      rythme hola.ry
Compruébalo:    rythme check hola.ry
Prueba código:  rythme          (modo interactivo)

Rythme te ayuda en todo momento: cada error dice dónde está, por qué, y cómo
corregirlo. `rythme explain E003` explica un código de error en detalle."#,
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
            r#"SINTAXIS

Valores
  42   -7   1_000_000            números enteros (Int), de cualquier tamaño
  3.14   2.0                     números decimales (Float)
  "hola"   "total: {x}"          textos (Text); {...} (o #{...}) inserta un valor
  true   false                   booleanos (Bool)
  :ok   :error                   símbolos (Symbol): un nombre usado como valor
  [1, 2, 3]                      listas
  {nombre: "Ada", edad: 36}      registros
  Some(3)   None                 un valor que puede faltar (Option)
  Ok(3)   Err("vaya")            un resultado que puede fallar (Result)
  1..5                           la lista [1, 2, 3, 4, 5]

Nombres
  x = 5                          un nombre para un valor; nunca cambia
  [a, b] = [1, 2]                descomponer un valor
  {nombre, edad} = persona

Operadores
  + - * /   // (división entera)   % (resto)   ** (potencia)
  == != < <= > >=   and   or   not
  x |> f(y)                      igual que f(x, y): pasos de izquierda a derecha
  x.f(y)                         también igual que f(x, y)
  x.f                            el campo f de x; si x no lo tiene, f(x): xs.sum
  xs                             una cadena puede seguir, cada línea empieza por .
    .select(even?)
    .sum

Funciones
  fn doble(x) = x * 2            en una línea, con =
  fn saludar(nombre)             en varias líneas, terminadas con `end`
    mensaje = "Hola {nombre}"
    print(mensaje)               la última línea da el resultado
  end
  fn(x) = x + 1                  una función anónima
  fn adulto?(p) = p.edad >= 18   un nombre puede terminar en ? (consulta rythme help ruby)

Bloques: una función dada como último argumento de una llamada
  xs.map do |x| x * 2 end        igual que map(xs, fn(x) = x * 2)
  xs.each do |x|                 en varias líneas, terminadas con `end`
    print(x)
  end
  xs.select { |x| x > 2 }        con llaves

Condiciones (todo if tiene un else)
  if x > 0 then "positivo" else if x == 0 then "cero" else "negativo"
  unless x > 0 then "pequeño" else "grande"     lo contrario de if

Elegir según la forma
  match lista
    [] -> "vacía"
    [x] -> "un elemento: {x}"
    [x, ..resto] -> "{x}, y {len(resto)} más"
  end

Varios pasos dentro de una expresión
  do
    a = 1
    a + 1
  end

Tus propios tipos
  type Forma = Circulo(radio) | Rect(ancho, alto)

Pruebas y otros archivos
  test "doble funciona" = assert_eq(doble(2), 4)
  use geometria                  y luego geometria.area(2)

Comentarios
  # un comentario normal
  ## un comentario de documentación, encima de una función o de un tipo, que muestra help(...)"#,
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
            r#"FUNCIONES

Definir y llamar
  fn sumar(a, b) = a + b
  sumar(2, 3)                    # 5

En varias líneas: la última expresión es el resultado
  fn describir(n)
    tipo = if n % 2 == 0 then "par" else "impar"
    "{n} es {tipo}"
  end

Una definición por caso (probadas en orden)
  fn fib(0) = 0
  fn fib(1) = 1
  fn fib(n) = fib(n - 1) + fib(n - 2)

Con una condición (una guarda)
  fn signo(n) if n < 0 = "negativo"
  fn signo(0) = "cero"
  fn signo(_) = "positivo"

Las funciones son valores
  doble = fn(x) = x * 2
  [1, 2, 3] |> map(doble)        # [2, 4, 6]
  [1, 2, 3] |> map(fn(x) = x + 1)

Las tuberías se leen de izquierda a derecha
  "  Hola  " |> trim |> upper    # "HOLA"
  x |> f(a, b)    significa    f(x, a, b)

Estilo de método
  "hola".upper()                 # igual que upper("hola")

Documentación
  ## Suma dos números.
  fn sumar(a, b) = a + b
  help(sumar)                    # muestra "Suma dos números."

Recursión sin límite
  Cuando la llamada recursiva es lo último que hace una función, puede
  repetirse millones de veces:
  fn cuenta_atras(0) = print("¡Despegue!")
  fn cuenta_atras(n)
    print(n)
    cuenta_atras(n - 1)
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

  numeros = [3, 1, 2]
  numeros[0]                     # 3 (el primero; -1 es el último)
  len(numeros)                   # 3
  1..5                           # [1, 2, 3, 4, 5]
  [0, ..numeros]                 # [0, 3, 1, 2]   (.. inserta una lista entera)
  numeros + [9]                  # [3, 1, 2, 9]

Trabajar con la lista entera (sin bucles)
  map(numeros, fn(x) = x * 10)            # [30, 10, 20]
  filter(numeros, fn(x) = x > 1)          # [3, 2]
  fold(numeros, 0, fn(acc, x) = acc + x)  # 6
  sort(numeros)                           # [1, 2, 3]
  numeros |> each(fn(x) = print(x))       # muestra cada elemento

Acceso seguro (nunca un error)
  first([])                      # None
  get(numeros, 10)               # None

"Cambiar" una lista: obtienes una lista nueva, la antigua sigue igual
  set_at(numeros, 0, 9)          # [9, 1, 2]
  insert_at(numeros, 1, 7)       # [3, 7, 1, 2]
  remove_at(numeros, -1)         # [3, 1]

Trozos de una lista
  take_while(numeros, fn(x) = x > 2)      # [3]
  chunks([1, 2, 3, 4, 5], 2)              # [[1, 2], [3, 4], [5]]
  windows([1, 2, 3], 2)                   # [[1, 2], [2, 3]]
  0..10 |> step_by(5)                     # [0, 5, 10]

Descomponer una lista con patrones
  match numeros
    [] -> "vacía"
    [primero, ..resto] -> "{primero}, y {len(resto)} más"
  end

Todas las funciones: rythme help builtins"#,
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

  nombre = "Ada"
  "Hola, {nombre}!"              # "Hola, Ada!"  ({...} inserta cualquier expresión)
  "2 + 2 = {2 + 2}"              # "2 + 2 = 4"
  "a" + "b"                      # "ab"
  len("año")                     # 3 (caracteres, tildes incluidas)
  upper("abc")   lower("ABC")   trim("  x  ")
  split("a,b,c", ",")            # ["a", "b", "c"]
  join(["a", "b"], "-")          # "a-b"
  replace("2024", "4", "5")      # "2025"
  contains("hola", "ol")         # true
  text(42)                       # "42"
  text(3.14159, 2)               # "3.14" (un número con 2 decimales)
  pad_left("7", 3, "0")          # "007"  (y pad_right: para alinear columnas)
  char_code("A")   char(66)      # 65 y "B"
  int("42")                      # 42
  parse_int("x42")               # Err(...): una conversión que no puede romperse
  trim_start("  x")   trim_end("x  ")
  words("¡Hola, tú!")            # ["Hola", "tú"]
  equals_ignore_case("Ada", "ADA")          # true
  format_number(1234.5, 2)       # "1,234.50"   (con "es": "1.234,50")
  find_all("3 gatos, 12 perros", "[0-9]+")  # ["3", "12"]  (regex: rythme help regex)

Caracteres especiales: \n (nueva línea), \t (tabulador), \r (retorno de carro),
\0 (carácter nulo), \" (comilla), \\ (barra invertida), \{ y \} (llaves, porque
{ empieza una inserción), \u{e9} (cualquier carácter por su número Unicode: é)."#,
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

Un registro agrupa valores con nombre:
  ada = {nombre: "Ada", nacida: 1815}
  ada.nombre                     # "Ada"
  ada["nombre"]                  # "Ada"

Los registros nunca cambian: construye uno nuevo a partir de uno viejo
  otra = {..ada, nacida: 1816}   # los mismos campos, `nacida` reemplazado

Descomponer
  {nombre, nacida} = ada
  fn saludar({nombre}) = "Hola {nombre}"

Funciones útiles
  keys(ada)   values(ada)   has(ada, "nombre")   get(ada, "x")   put(ada, "x", 1)
  entries(ada)                   # [["nombre", "Ada"], ["nacida", 1815]]
  from_entries([["x", 1]])       # {x: 1}

Registros como diccionarios
  cuentas = {manzana: 2}
  put(cuentas, "pera", 5)        # {manzana: 2, pera: 5}
  map_values(cuentas, fn(n) = n * 10)     # {manzana: 20}
  group_by(["a", "b", "a"], fn(x) = x) |> map_values(len)   # {a: 2, b: 1}
  Para claves que no son nombres (números, listas...), usa un dict: rythme help dicts

JSON, el formato de los datos en la web
  parse_json(texto)              # Ok(...): los objetos se vuelven registros, los arrays listas
  to_json(valor)                 # el texto, listo para guardar con write_file"#,
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
            r#"DICCIONARIOS Y CONJUNTOS

Un dict (diccionario) da un valor para cada clave. Las claves pueden ser
cualquier valor salvo funciones: números, textos, listas, registros... Se
guardan en orden.
  edades = dict([["Ada", 36], ["Alan", 41]])
  edades                         # dict("Ada" => 36, "Alan" => 41)
  edades["Ada"]                  # 36 (un error si falta la clave)
  get(edades, "Bob")             # None: una Option, nunca un error
  rejilla = dict([[[0, 0], "inicio"]])   # una lista como clave
  dict({x: 1})                   # a partir de un registro: dict("x" => 1)

Los dicts nunca cambian: las funciones dan uno nuevo, rápido (en O(log n))
  put(edades, "Bob", 20)         # añadido (o reemplazado)
  remove(edades, "Ada")          # sin esta clave
  has(edades, "Ada")   len(edades)   keys(edades)   values(edades)   entries(edades)
  map_values(edades, fn(n) = n + 1)
  filter(edades, fn(nombre, edad) = edad > 40)   # f recibe la clave y el valor
  edades.each do |nombre, edad| print(nombre, edad) end   # también reject y select
  union(a, b)                    # los dos; para una clave en ambos, gana el valor de b

Un conjunto (set) guarda valores sin repetición, en orden.
  s = set([3, 1, 3])             # set(1, 3)
  has(s, 3)   push(s, 5)   remove(s, 1)   len(s)   values(s)   # values: una lista
  union(a, b)   intersection(a, b)   difference(a, b)   is_subset(a, b)
  filter(s, is_odd)   map(s, fn(x) = x * 10)

Los dicts y los conjuntos se comparan con == y <, y se muestran como
dict(k => v, ...) y set(a, b, ...). `to_json` escribe un dict cuyas claves son
todas textos (o todas símbolos) como un objeto, y un conjunto como un array;
`dict(parse_json(texto)?)` vuelve a leer un objeto."#,
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
            r#"TUS PROPIOS TIPOS

Un tipo enumera las formas posibles de un valor:
  type Forma = Circulo(radio) | Rect(ancho, alto) | Vacia

  c = Circulo(2.0)
  c.radio                        # 2.0

Trata cada forma con match
  fn area(forma) = match forma
    Circulo(r) -> pi * r * r
    Rect(w, h) -> w * h
    Vacia -> 0
  end

O con una definición por forma
  fn perimetro(Circulo(r)) = 2 * pi * r
  fn perimetro(Rect(w, h)) = 2 * (w + h)
  fn perimetro(Vacia) = 0

Si olvidas una forma, `rythme check` te dice cuál.

Un tipo puede contenerse a sí mismo (árboles...):
  type Arbol = Hoja | Nodo(izquierda, valor, derecha)"#,
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
            r#"MATCH Y PATRONES

`match` compara un valor con patrones, en orden, y da el resultado del
primero que encaja:

  fn describir(x) = match x
    0 -> "cero"
    1 | 2 | 3 -> "pequeño"
    n if n < 0 -> "negativo"
    _ -> "grande"
  end

Patrones
  42   "texto"   true            exactamente este valor
  1..9   -5..-1   0.0..0.5       un número del primero al segundo, ambos incluidos
  "Sr. " + nombre                un texto que empieza por "Sr. "; nombre es el resto
  x                              cualquier cosa, llamada x
  _                              cualquier cosa, ignorada
  Some(x)   Circulo(r)           un constructor y lo que contiene
  []   [a, b]   [primero, ..resto]   listas, según su forma
  {nombre, edad}                 un registro con estos campos
  p if condición                 un patrón y una condición (guarda)
  a | b                          un patrón o el otro

Los patrones funcionan en todos los sitios donde se da un nombre:
  [x, y] = [1, 2]
  fn cabeza_o_cero([]) = 0
  fn cabeza_o_cero([x, ..]) = x"#,
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
            r#"LOS ERRORES SON VALORES (Option, Result, ?)

Rythme no tiene null ni excepciones. Un valor que puede faltar es una
Option: Some(valor) o None. Una operación que puede fallar da un Result:
Ok(valor) o Err(mensaje). No puedes olvidarte de tratarlos.

  fn dividir(a, b) = if b == 0 then Err("división entre cero") else Ok(a / b)

  match dividir(10, 2)
    Ok(x) -> print("resultado: {x}")
    Err(e) -> print("error: {e}")
  end

El operador ?: toma el valor, o se detiene y pasa el error hacia arriba
  fn media(a, b)
    x = parse_float(a)?          # si es Err(...), media lo devuelve
    y = parse_float(b)?
    Ok((x + y) / 2)
  end

Herramientas rápidas
  unwrap_or(get(lista, 5), 0)    # el valor, o uno por defecto
  unwrap(Some(3))                # 3 (detiene el programa con None o Err)
  is_some(x)   is_none(x)   is_ok(x)   is_err(x)
  map(Some(2), fn(x) = x * 10)   # Some(20)
  ok_or(get(persona, "edad"), "sin edad")   # Err("sin edad") si falta: listo para ?
  and_then(x, f)                 # un paso más que puede fallar
  map_err(r, fn(e) = "config: {e}")     # explicar mejor un error

Para detener todo el programa: fail("mensaje")."#,
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

En Rythme, los valores nunca cambian y no hay bucles. Puede parecer raro,
pero los programas se vuelven más fáciles de leer, de probar y de entender:
una función siempre da el mismo resultado para los mismos argumentos.

En vez de un bucle que muestra:        1..5 |> each(fn(i) = print(i))
En vez de un bucle que transforma:     map(lista, f)
En vez de un bucle que selecciona:     filter(lista, f)
En vez de un bucle que acumula:        fold(lista, inicio, f)   o sum, product
En vez de totales parciales:           scan(lista, inicio, f)
En vez de un bucle que busca:          find, any, all, index_of, min_by, max_by
En vez de "break":                     take_while(lista, f)
En vez de "while":                     una función recursiva

  # Python:  total = 0
  #          for x in numeros: total += x * x
  total = numeros |> map(fn(x) = x * x) |> sum

  # Un bucle "while" como función recursiva:
  fn collatz(1, pasos) = pasos
  fn collatz(n, pasos) =
    if n % 2 == 0 then collatz(n // 2, pasos + 1) else collatz(3 * n + 1, pasos + 1)
  collatz(27, 0)                 # 111

En vez de cambiar un valor, crea uno nuevo:
  x2 = x + 1
  persona2 = {..persona, edad: 37}
  lista2 = push(lista, 4)"#,
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
            r#"PRUEBAS

Escribe las pruebas junto a tu código:
  fn doble(x) = x * 2

  test "el doble de 2 es 4" = assert_eq(doble(2), 4)

  test "varias comprobaciones"
    assert_eq(doble(0), 0)
    assert(doble(-1) < 0, "debería seguir siendo negativo")
  end

Ejecútalas:
  rythme test mi_archivo.ry         las pruebas de un archivo
  rythme test .                     las de cada archivo .ry de una carpeta (y de las de dentro)
  rythme test . --filter doble      solo las pruebas cuyo nombre contiene "doble"
  rythme test . --watch             otra vez cada vez que cambia un archivo (Ctrl+C para parar)
  rythme test mi_archivo.ry -- a b  con `args()` dando ["a", "b"]
  rythme test                       en un proyecto: sus archivos con pruebas

Al probar, las expresiones del nivel superior como print(...) no se ejecutan:
se cargan las definiciones, y luego se ejecuta cada prueba. Una prueba pasa si
termina sin error, y su valor no es false, Err(...) ni None. El resumen da las
cuentas y el tiempo; el código de salida es 0 cuando pasan todas las pruebas.

Pruebas de propiedades: comprobar que algo se cumple para muchos valores
  test "invertir dos veces devuelve la lista" =
    check_all(fn(l) = reverse(reverse(l)) == l, lists_of(ints()))

Generadores: ints(min, max), floats(min, max), texts(max_longitud), bools(),
lists_of(generador, max_longitud), one_of(lista), records_of({x: ints(), y: texts()}).
`sample(generador)` muestra lo que da uno. Cuando un valor hace falsa la
propiedad, se simplifica (shrinking) y se muestra con la semilla que lo
produjo: los mismos valores vuelven en cada ejecución, y `--seed N` prueba
otros (`--seed random` elige una)."#,
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
tests. `rythme test file.ry` runs the tests too. The exercises follow the
language of the messages (`rythme --lang es exercise start 7`); the folder
docs/exercises of the repository of Rythme has them in English and French."#,
            r#"EJERCICIOS

Rythme trae 30 ejercicios graduados para un primer curso, de los primeros
valores a la recursión, las listas, los registros, los tipos, Option, Result y
las pruebas. Cada uno es un archivo con un enunciado, pistas, las funciones
que escribir y sus pruebas.

  rythme exercise list              los ejercicios, de ★☆☆ (fácil) a ★★★
  rythme exercise start 7           copia el ejercicio 7 en la carpeta actual
  rythme exercise check 07-recursion.ry
                                    ejecuta sus pruebas, y dice qué hacer después

En el archivo, reemplaza cada fail("por hacer") por tu código; no cambies las
pruebas. `rythme test archivo.ry` también ejecuta las pruebas. Los ejercicios
siguen el idioma de los mensajes (`rythme --lang es exercise start 7`); la
carpeta docs/exercises del repositorio de Rythme los tiene en inglés y en
francés."#,
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
            r#"VARIOS ARCHIVOS

  # geometria.ry
  ## Área de un rectángulo.
  fn area(w, h) = w * h
  fn _comprobar(x) = x >= 0       # privada: empieza por `_`

  # main.ry
  use geometria                   # el archivo geometria.ry, junto a main.ry
  print(geometria.area(2, 3))     # 6

`use geometria` da las definiciones de geometria.ry a través de su nombre:
`geometria.area`, `geometria.Circulo(1)`, y en un `match`, `geometria.Circulo(r)`.
Así siempre ves de dónde viene un nombre. Un archivo en una carpeta:
`use formas/circulo`, y luego `circulo.perimetro(1)`. Otro nombre: `use
geometria as geo`, y luego `geo.area(2, 3)`. Los nombres que empiezan por `_`
son privados de su archivo.

La ruta es relativa a la carpeta del archivo que contiene `use`. Cada archivo
se ejecuta una sola vez. Rythme dice qué archivo define un nombre que olvidaste
usar.

Forma antigua: `use "geometria.ry"` (con comillas) da sus definiciones
directamente, como `area(2, 3)`; mejor usa `use geometria`."#,
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
            r#"ARCHIVOS Y SISTEMA

Toda función que toca el sistema devuelve un Result: Ok(...), o
Err(mensaje) con un mensaje que dice qué salió mal.

Archivos y carpetas (las rutas relativas empiezan en la carpeta actual)
  read_file("notas.txt")         # Ok("...") o Err("archivo no encontrado: notas.txt")
  write_file("salida.txt", "hola")   # Ok(()), reemplaza el contenido
  append_file("registro.txt", "hecho\n")
  file_exists("notas.txt")       # true o false
  file_info("notas.txt")         # Ok({is_folder: false, size: 11}), en bytes
  list_folder(".")               # Ok(["notas.txt", "fotos"]), en orden
  create_folder("copia/2024")    # Ok(()), con las carpetas que falten alrededor
  delete_file("viejo.txt")       # Ok(()); nunca una carpeta

Rutas que funcionan en Linux, macOS y Windows
  join_path(["datos", "notas.txt"])      # "datos/notas.txt"
  split_path("/home/ada/notas.txt")      # ["/", "home", "ada", "notas.txt"]
  split_path("C:\Users\ada")             # ["C:\", "Users", "ada"]

La línea de comandos: rythme herramienta.ry --verbose --name=Ada notas.txt
  args()                         # ["--verbose", "--name=Ada", "notas.txt"]
  options()                      # Ok({args: ["notas.txt"], verbose: true, name: "Ada"})
  opts = unwrap(options())
  nombre = unwrap_or(get(opts, "name"), "mundo")   # una opción que puede faltar
  env("HOME")                    # Some("/home/ada") o None

La entrada estándar: rythme contar.ry < nombres.txt
  input("¿Nombre? ")             # una línea
  input_lines()                  # Ok(["Ada", "Alan"]): todas las líneas hasta el final

Códigos de salida, para los scripts que ejecutan tu programa
  0    todo fue bien (también al final del programa)
  1    un error en el programa: sintaxis, comprobación, durante la ejecución, prueba fallida
  2    un mal uso del comando (comando u opción desconocidos)
  101  un error interno de Rythme (un fallo, que hay que avisar)
  exit(código)                   # se detiene ya con este código, de 0 a 255

En el playground, en el navegador, no hay archivos ni entrada: estas
funciones dan Err("... no está disponible en el navegador ...")."#,
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
            r#"PROYECTOS Y PAQUETES

  rythme new hola         # una carpeta hola/ con un proyecto nuevo:
                          #   rythme.toml, src/main.ry, src/saludo.ry,
                          #   tests/saludo_test.ry, README.md, .gitignore
  cd hola
  rythme run              # ejecuta el programa: `entry` de rythme.toml
  rythme test             # ejecuta los archivos con pruebas de src/ y tests/
  rythme check            # comprueba cada archivo; también `rythme fmt`, `rythme doc`

rythme.toml describe el proyecto:

  name = "hola"
  version = "0.1.0"
  entry = "src/main.ry"

  [dependencies]
  mathlib = { git = "https://github.com/alguien/mathlib", tag = "v1.0.0" }
  herramientas = { path = "../herramientas" }

Una dependencia viene de git, en un `tag` o un `commit`, o de una carpeta.
`rythme install` las descarga (con git) y escribe rythme.lock, el commit exacto
de cada una: guárdalo con el proyecto, para que todo el mundo tenga el mismo
código. Después:

  use mathlib/vectores      # el archivo src/vectores.ry de mathlib
  use mathlib               # su programa, el `entry` de su rythme.toml
  use hola/saludo           # un archivo del propio proyecto, desde tests/

Un archivo junto al que contiene `use` va primero. Los paquetes de git se
guardan en ~/.rythme/packages (o RYTHME_HOME/packages)."#,
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
            r#"FECHAS Y HORAS

Una fecha es un registro: {year, month, day}, con hour, minute y second para un
momento del día, y offset (minutos de adelanto sobre UTC) cuando se conoce.
  today()                        # {year: 2026, month: 10, day: 18}
  now()                          # {year: 2026, ..., hour: 14, minute: 5, second: 9, offset: 120}
  cumple = {year: 1815, month: 12, day: 10}
  cumple.year                    # 1815

De texto y vuelta, en la forma ISO 8601
  parse_date("2024-02-29")       # Ok({year: 2024, month: 2, day: 29})
  parse_date("2024-02-30")       # Err("... febrero de 2024 ...")
  parse_date("2024-02-29T13:05:00+01:00")    # con una hora y un desfase
  format_date({year: 2024, month: 2, day: 29, hour: 9})   # "2024-02-29T09:00:00"

Aritmética del calendario (años bisiestos y largos de mes incluidos)
  add_days({year: 2024, month: 2, day: 28}, 1)     # {year: 2024, month: 2, day: 29}
  add_seconds(now(), 3600)       # dentro de una hora
  days_between(today(), {year: 2027, month: 1, day: 1})   # días que quedan
  seconds_between(a, b)          # negativo cuando b es anterior a a
  day_of_week(today())           # 1 = lunes ... 7 = domingo
  days_in_month(2023, 2)         # 28
  is_leap_year(2000)             # true
  is_date({year: 2023, month: 2, day: 29})   # false

Una fecha que no existe detiene las funciones con E023, que dice por qué.
El desfase local viene del sistema (UTC cuando no lo dice); time() da los
segundos desde 1970 como un Float, para medir duraciones."#,
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
            r#"REGEX: BUSCAR CON UN PATRÓN

Una regex (expresión regular) describe trozos de texto: "[0-9]+" significa
"una cifra o más". Rythme lee las regex habituales, y una búsqueda nunca tarda
más que (longitud del texto) × (tamaño de la regex).

  matches("2024", "[0-9]+")                  # true: el texto ENTERO encaja
  find_first("tel: 06 12", "[0-9]+")         # Some("06"): el primer trozo
  find_all("3 gatos, 12 perros", "[0-9]+")   # ["3", "12"]
  split_by("a, b;c", "[,;] *")               # ["a", "b", "c"]
  replace_all("un gato", "\\b\\w", upper)    # "Un Gato" (una función para cada trozo)
  replace_all("29/02/2024", "(\\d+)/(\\d+)/(\\d+)", "$3-$2-$1")   # "2024-02-29"
  captures("2024-02", "([0-9]+)-([0-9]+)")   # Some(["2024-02", "2024", "02"])
  named_captures("2024-02", "(?<anio>[0-9]+)-(?<mes>[0-9]+)")
                                             # Some({anio: "2024", mes: "02"})

Lo que puede decir una regex
  abc          estos caracteres, en este orden
  .            cualquier carácter salvo un salto de línea
  [abc]        uno de estos caracteres; [a-z] uno de a a z; [^0-9] cualquier otro
  \d \w \s     una cifra (0-9); una letra (con tildes), una cifra o _; un espacio
  \D \W \S     cualquier otra cosa
  \b           el borde de una palabra
  ^  $         el principio y el final del texto
  x|y          x o y
  (...)        un grupo: para repetirlo, o para capturar su texto ($1, `captures`)
  (?:...)      un grupo que no captura nada; (?<nombre>...) un grupo con nombre
  x*  x+  x?   x cero veces o más, una vez o más, cero o una vez (lo más largo)
  x{3}  x{2,5}  x{2,}    3 veces, de 2 a 5 veces, 2 veces o más
  x*?  x+?     lo más corto en vez de lo más largo
  (?i)         al principio: mayúsculas y minúsculas cuentan como iguales

Escribir una regex en un texto de Rythme
  La barra invertida se escribe dos veces: "\\d+" es la regex \d+. Las llaves
  se escriben \{ y \}: "[0-9]\{4\}" es la regex [0-9]{4}. Para un signo en sí,
  ponlo entre corchetes: "[.]" es un punto, "[(]" un paréntesis, "[$]" un dólar.

Un error en una regex detiene el programa con E026, que muestra dónde está;
cuando la regex está escrita en el código, `rythme check` lo encuentra antes."#,
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
            r#"EL MODO INTERACTIVO

Ejecuta `rythme` sin archivo. Escribe una expresión y mira su valor:
  ry> 1 + 2
  3
Las definiciones se conservan de una línea a otra. Si una línea no está
terminada (paréntesis abierto, bloque sin `end`...), sigue en la línea
siguiente.

Comandos
  :help           esta ayuda             :quit          salir (o Ctrl-D)
  :doc map        ayuda sobre un nombre  :load f.ry     cargar un archivo
  :reset          olvidarlo todo         :lang es       cambiar el idioma (en, fr, es, de, pt, zh)
  :learn          visita guiada          :explain E003  explicar un error
  :steps fib(4)   mostrar el cálculo paso a paso
La tecla Tab completa los nombres."#,
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
            r#"FORMATO: rythme fmt

`rythme fmt` da al código el formato oficial de Rythme: todos los programas se
ven igual, y nadie tiene que discutir sobre los espacios.

  rythme fmt hola.ry             formatear un archivo (se reescribe)
  rythme fmt .                   cada archivo .ry de una carpeta y de sus subcarpetas
  rythme fmt --check .           solo comprobar: lista los archivos por formatear y
                                 falla (código de salida 1) si hay alguno, para la CI
  rythme fmt - < hola.ry         lee el código, escribe el resultado

El estilo
  - 2 espacios por nivel de sangría
  - espacios alrededor de los operadores: x = a + b * 2   xs |> map(f)   n -> n * 2
  - ninguno dentro de los paréntesis ni alrededor de ..: f(x)   [1, 2]   {nombre: "Ada"}   1..10
  - una cadena escrita en varias líneas: un paso por línea
      total = precios
        |> filter(fn(p) = p > 10)
        |> sum
  - un tipo escrito en varias líneas: una variante por línea
      type Forma =
        | Circulo(radio)
        | Rect(ancho, alto)
  - una lista escrita en varias líneas termina con una coma
  - un bloque escrito en una línea se queda en una línea:
      xs.map do |x| x * 2 end      xs.map { |x| x * 2 }
    en varias líneas, sus instrucciones van un nivel más adentro que `do`
  - una instrucción por línea (sin `;`), y `end` en su propia línea
  - una línea en blanco alrededor de una definición escrita en varias líneas;
    nunca dos líneas en blanco seguidas
  - se conservan todos los comentarios; los comentarios al final de líneas
    seguidas se alinean

Los saltos de línea se conservan donde el lenguaje permite elegir, y las
líneas largas se dejan como están escritas. El formateador nunca cambia lo que
hace un programa: un archivo con un error de sintaxis se deja tal cual, y se
muestra el error."#,
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
            r#"DOCUMENTACIÓN: rythme doc

`rythme doc` escribe la documentación de tu código a partir de sus comentarios
`##`, sus tipos y las definiciones de sus funciones.

  rythme doc geometria.ry        Markdown en la pantalla
  rythme doc .                   cada archivo .ry de una carpeta y de sus subcarpetas
  rythme doc --html geometria.ry una página web, que no necesita nada más
  rythme doc . --out doc.html    escrita en un archivo (.html da la página web)

Escribe un comentario `##` justo encima de lo que documenta:

  ## Formas y sus áreas.          <- al principio, seguido de una línea en blanco:
                                     describe todo el archivo
  ## Una forma que dibujar.
  type Forma = Circulo(radio) | Rect(ancho, alto)

  ## El área de una forma. Consulta `len` y `area`.
  fn area(Circulo(r)) = pi * r * r
  fn area(Rect(w, h)) = w * h

Lo que se escribe
  - para cada archivo: su descripción, los archivos que usa (documentados
    también), sus tipos, sus funciones (la primera línea de cada definición),
    sus valores y los nombres de sus pruebas
  - los nombres que empiezan por `_` se omiten: se quedan dentro de su archivo
  - un nombre entre comillas invertidas en un comentario, como `area` o
    `len(lista)`, se convierte en un enlace a su documentación; tras
    `use geometria`, `geometria.area` lleva a `area` en geometria.ry; las
    funciones integradas citadas así se explican al final
  - los títulos están en el idioma de los mensajes (`--lang es`)

En un comentario, una línea `##` vacía empieza un párrafo nuevo, y las líneas
que empiezan por `- ` forman una lista."#,
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
            r#"PASO A PASO: rythme steps

`rythme steps` muestra cómo calcula un programa sus valores, paso a paso:
cada llamada se reemplaza por el cuerpo de su función, con los valores de los
argumentos en su lugar, hasta que solo queda el resultado.

  rythme steps fib.ry              cada definición y expresión del archivo
  rythme steps fib.ry "fib(4)"     una expresión, tras las definiciones del archivo
  rythme steps fib.ry --max 50     mostrar como máximo 50 pasos de cada expresión
  rythme steps fib.ry --width 80   cortar las líneas de más de 80 caracteres

Con  fn fib(n) = if n < 2 then n else fib(n - 1) + fib(n - 2)

  fib(3)
    → fib(2) + fib(1)
    → fib(1) + fib(0) + fib(1)
    → 1 + fib(0) + fib(1)
    → 1 + 0 + fib(1)
    → 1 + fib(1)
    → 1 + 1
    → 2

Un paso es una llamada o una operación. El cuerpo de una función llamada se
muestra ya simplificado todo lo posible sin llamar a otra función. Las
funciones integradas (y las funciones de otros archivos) dan su resultado en
un paso. Lo que el programa muestra aparece entre los pasos. Después de los
pasos mostrados (200 por defecto), el resto se calcula sin mostrarse.

En el modo interactivo: :steps fib(4). En el playground: el botón Steps.
En francés: rythme etapes."#,
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
            r#"RYTHME EN TU EDITOR

Rythme también ayuda dentro del editor: los problemas mientras escribes, la
documentación de una función bajo el ratón (con la firma deducida de su
código, como `fn area(lado: Int|Float) -> Int|Float`), el autocompletado, los
parámetros de una llamada, ir a la definición, las referencias, renombrar,
correcciones rápidas de erratas, el formato como `rythme fmt`, los tipos de
los nombres cuando son seguros (y las firmas de las funciones), colores que
siguen lo que es cada nombre, "Run" y "Run test" encima del código, los
nombres de todo el espacio de trabajo, el plegado. Es el servidor de lenguaje,
que el editor inicia solo:
  rythme lsp

VS Code
  la extensión de editors/vscode: colores, fragmentos y el servidor de lenguaje.
  Ajustes: rythme.path (el programa), rythme.language (en, fr, es, de, pt, zh o auto).

Neovim, Helix, Zed, Emacs, Sublime Text, Kate...
  todo editor que hable el Language Server Protocol: dile que inicie
  `rythme lsp` para los archivos .ry. Configuraciones listas: editors/README.md
  ("Run" y "Run test" nombran los comandos rythme.runFile, rythme.testFile y
  rythme.runTest, que el editor ejecuta: consulta editors/README.md)

Zed
  la extensión de editors/zed: colores y el servidor de lenguaje.

Colores en Neovim, Helix y Zed
  la gramática Tree-sitter de editors/tree-sitter-rythme (consulta editors/README.md).

El idioma de los mensajes sigue al del editor, salvo si `--lang` (en, fr, es, de, pt, zh) o
RYTHME_LANG lo eligen: rythme --lang es lsp"#,
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
            r#"SI VIENES DE RUBY

Rythme se lee como Ruby, pero sus valores nunca cambian: sin mutación, sin nil,
sin excepciones. Lo que sabes de Ruby funciona casi siempre:

  nombres = ["Ada", "Grace", "Alan"]
  nombres.each do |nombre|
    puts "Hola, #{nombre}!"
  end

  [1, 2, 3, 4, 5, 6]
    .select(even?)
    .map { |x| x * x }
    .sum                         # 56

  fn adulto?(persona) = persona.edad >= 18
  tipo = unless adulto?(ada) then :menor else :adulto

Igual que en Ruby
  llamadas        puts "hola"   print x, y   assert_eq total, 42
                  (al principio de una instrucción o de un resultado; mira abajo)
  bloques         xs.map do |x| x * 2 end      xs.map { |x| x * 2 }
                  un bloque es una función: lo mismo que fn(x) = x * 2
  métodos         xs.sum   nombre.upcase   xs.first(2)   xs.include?(3)
  nombres con ?   even?   empty?   include?(x)   fn adulto?(p) = ...
  símbolos        :ok   :error (comparados con ==, usados en match)
  textos          "Hola #{nombre}" (o "Hola {nombre}"); "\#{" escribe #{
  unless          unless cond then a else b
  nombres Ruby    puts to_s to_i to_f length size upcase downcase strip
                  select reject reduce inject uniq each_slice each_cons
                  each_with_index include? empty? even? odd? any? all?
                  start_with? end_with? key? lstrip rstrip casecmp?
                  (consulta rythme help builtins)

Lo que cambia
  paréntesis      una llamada dentro de una expresión los necesita: x = len(xs),
                  no x = len xs
  sin mutación    nada de x += 1: da un nombre nuevo al valor nuevo, total2 = total + 1
  sin nil         un valor que falta es None, uno presente Some(x)
  sin excepciones los errores son valores, Ok(x) o Err(mensaje), que se pasan con ?
  if y unless     siempre tienen un else: dan un valor; sin `return`, la última
                  línea de una función es su resultado
  sin clases      registros {nombre: "Ada"} y tipos: type Forma = Circulo(r) | Cuadrado(l)
  sin bucles      each, map, select, reduce, o una función que se llama a sí misma
  case/when       match valor ... end, un caso por línea: patrón -> resultado
  hashes          dict([[:ada, 36]]) (claves de cualquier tipo, también símbolos), o
                  registros {nombre: "Ada", edad: 36}

Llamadas sin paréntesis
  Una instrucción que empieza por un nombre, seguido en su línea por el
  principio de un valor (un número, un texto, un símbolo, un nombre, un
  constructor, not, if, match, fn), es una llamada: print x, y es print(x, y).
  Un bloque do después de los argumentos se da a la llamada: each xs do |x| ...
  end. Como en Ruby, un [ o un - tras un espacio, sin espacio después del -,
  también empieza los argumentos (puts [1, 2], puts -1), salvo cuando el código
  da un valor al nombre (con =, como parámetro o en un patrón): entonces x -1
  es x - 1 y xs [0] es xs[0], como antes. `rythme fmt` los escribe x - 1 y xs[0].

  Un resultado puede empezar igual: una rama de if o unless, un caso de match,
  el cuerpo tras el = de una función o de una prueba. Sus argumentos terminan
  en else, end, }, ) o el final de la línea:

  fn saludar(nombre) = puts "Hola, #{nombre}!"
  fn comprobar(ok) = if ok then puts "sí" else puts "no"
  match nombre
    "Dr. " + resto -> puts "doctor", resto
    _ -> puts nombre
  end
  xs.each { |x| puts x }

Cómo funciona x.nombre
  x.nombre lee el campo `nombre` de un registro; cuando x no tiene ese campo,
  llama a la función `nombre` con x: xs.sum es sum(xs), xs.first(2) es
  first(xs, 2). Una cadena puede seguir en varias líneas, cada una empezando
  por `.`.

Cuándo ? forma parte de un nombre
  even?(4), xs.include?(3), fn adulto?(p): justo después de un nombre, antes de `(`.
  even? sin `(`: cuando existe una función así (integrada, o definida con fn
  en el archivo). Si no, valor? es el ? que se detiene con None o Err, y
  (valor)? siempre lo es."#,
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
            r#"E001 — Error de sintaxis

El código no sigue la gramática de Rythme en este punto. El mensaje dice lo
que se esperaba. Causas frecuentes:
  - falta un `)`, `]` o `,`
  - falta `then` o `else` en un `if`
  - llaves { } usadas para un bloque (Rythme usa una línea nueva y `end`)
  - una palabra clave de otro lenguaje (for, while, return, let, def...)

Mal:     if x > 0 { print(x) }
Bien:    if x > 0 then print(x) else print("no es positivo")"#,
        ),
        (
            r#"E002 — Text never closed

A text starts with " and must end with " on the same text. A `{` inside a
text starts an insertion and must be closed by `}`.

Wrong:   print("hello)
Right:   print("hello")
To write a brace in a text: "\{not inserted\}""#,
            r#"E002 — Texto sin cerrar

Un texto empieza con " y debe terminar con " en el mismo texto. Una `{` dentro
de un texto empieza una inserción y debe cerrarse con `}`.

Mal:     print("hola)
Bien:    print("hola")
Para escribir una llave en un texto: "\{no insertado\}""#,
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
            r#"E003 — Nombre desconocido

Este nombre no está definido en este punto. Comprueba cómo está escrito (las
mayúsculas y las minúsculas son distintas), y que el nombre se define antes de
usarse. Dentro de una función se ven los parámetros y los nombres definidos
arriba, y las funciones locales escritas una justo después de la otra se ven
entre sí; en el nivel superior, todas las funciones se ven en todas partes.

Mal:     print(lenght([1, 2]))
Bien:    print(len([1, 2]))

Rythme propone el nombre existente más parecido cuando lo hay, y el archivo
que lo define cuando es otro archivo de la carpeta:

  use geometria                  # y luego
  print(geometria.area(2))       # no area(2)

Un nombre que empieza por `_` es privado de su archivo: los demás archivos no
pueden usarlo."#,
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
            r#"E004 — Número de argumentos incorrecto

La función se llama con más o menos valores de los que recibe. Recuerda que
con `|>`, el valor de la izquierda es el primer argumento.

Mal:     fn sumar(a, b) = a + b
         sumar(1)
Bien:    sumar(1, 2)
También bien:  1 |> sumar(2)

`rythme help NOMBRE` muestra cómo llamar a una función integrada."#,
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
            r#"E005 — Tipo de valor incorrecto

La operación no funciona con este tipo de valor. Rythme nunca convierte los
valores sin avisar: "1" + 1 es un error, no "11" ni 2.

Mal:     "edad: " + 36
Bien:    "edad: {36}"             (insertar el valor en el texto)
Bien:    "edad: " + text(36)      (convertirlo antes)

Las condiciones deben ser true o false:
Mal:     if len(lista) then ... else ...
Bien:    if len(lista) > 0 then ... else ...

Cuando los tipos de los valores son seguros, `rythme check` encuentra este
error antes de que se ejecute el programa, incluso en código que no se ejecuta
siempre, y en las llamadas que dan a una función lo que no puede usar:
`area("grande")` cuando `area(lado)` calcula `lado * lado`."#,
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
            r#"E006 — Ningún caso encaja

Ninguno de los casos de un `match` (ni de las definiciones de una función, ni
el patrón de una definición como `[a, b] = lista`) acepta este valor.

Mal:     fn nombre(1) = "uno"
         nombre(2)
Bien:    fn nombre(1) = "uno"
         fn nombre(_) = "otro número"

`rythme check` avisa de los casos olvidados antes de ejecutar (W002). Cuando
el tipo del valor es seguro, también señala, antes de ejecutar, un caso que
nunca puede encajar: `[primero, ..resto]` para un Text, `Circulo(r)` para un
número."#,
        ),
        (
            r#"E007 — Index out of range

Positions in a list or a text go from 0 to length - 1 (and -1 is the last).

Wrong:   [10, 20][2]
Right:   [10, 20][1]              # 20
Safe:    get([10, 20], 2)         # None, never an error"#,
            r#"E007 — Índice fuera de rango

Las posiciones en una lista o un texto van de 0 a longitud - 1 (y -1 es la última).

Mal:     [10, 20][2]
Bien:    [10, 20][1]              # 20
Seguro:  get([10, 20], 2)         # None, nunca un error"#,
        ),
        (
            r#"E008 — Division by zero

Dividing by zero has no meaning. Check the divisor before, or return a Result.

  fn divide(a, b) = if b == 0 then Err("division by zero") else Ok(a / b)"#,
            r#"E008 — División entre cero

Dividir entre cero no tiene sentido. Comprueba antes el divisor, o devuelve un Result.

  fn dividir(a, b) = if b == 0 then Err("división entre cero") else Ok(a / b)"#,
        ),
        (
            r#"E009 — Not a function

Parentheses after a value call it, but only functions can be called.

Wrong:   x = 5
         x(3)
Wrong:   None()                   (None is a value)
Right:   None
To multiply, write it:  2 * (x + 1)"#,
            r#"E009 — No es una función

Unos paréntesis después de un valor lo llaman, pero solo se pueden llamar las
funciones.

Mal:     x = 5
         x(3)
Mal:     None()                   (None es un valor)
Bien:    None
Para multiplicar, escríbelo:  2 * (x + 1)"#,
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
            r#"E010 — Campo desconocido

El registro (o el constructor) no tiene ningún campo con este nombre; o el
dict no tiene esa clave (`dict[clave]`).

Mal:     {nombre: "Ada"}.nombe
Bien:    {nombre: "Ada"}.nombre
Seguro:  get(registro, "nombre") # una Option, nunca un error
         get(dic, clave)         # lo mismo para un dict

Cuando los campos del registro son seguros, `rythme check` encuentra este
error antes de que se ejecute el programa, y propone el campo más parecido."#,
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
            r#"E011 — Ya definido

En Rythme un nombre siempre significa el mismo valor: no se puede cambiar ni
definir dos veces en el mismo sitio. Es lo que hace que los programas sean
fáciles de seguir.

Mal:     x = 1
         x = x + 1
Bien:    x = 1
         y = x + 1

Para una función con varios casos, escribe las definiciones una justo después
de la otra:
  fn fib(0) = 0
  fn fib(1) = 1
  fn fib(n) = fib(n - 1) + fib(n - 2)

El nombre dado a un archivo usado (`use geometria`, `use ... as geo`) no puede
nombrar nada más en el archivo; y dos archivos usados con comillas
(`use "a.ry"`) no pueden definir el mismo nombre: úsalos mejor por su nombre
(`use a`, y luego `a.area`)."#,
        ),
        (
            r#"E012 — Number too large (no longer given)

Whole numbers (Int) have no limit any more: they grow as needed, and stay
exact. Earlier versions of Rythme stopped with this error beyond 64 bits.

  2 ** 100        # 1267650600228229401496703205376
  2.0 ** 100      # 1.2676506002282294e30 (a Float is approximate)

Only a whole number of more than 100000 digits stops the program, with
E023: it is usually a calculation that grows without end."#,
            r#"E012 — Número demasiado grande (ya no se da)

Los números enteros (Int) ya no tienen límite: crecen lo que haga falta, y
siguen siendo exactos. Las versiones anteriores de Rythme se detenían con este
error más allá de 64 bits.

  2 ** 100        # 1267650600228229401496703205376
  2.0 ** 100      # 1.2676506002282294e30 (un Float es aproximado)

Solo un número entero de más de 100000 cifras detiene el programa, con
E023: suele ser un cálculo que crece sin fin."#,
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
            r#"E013 — unwrap no encontró ningún valor

`unwrap(x)` saca el valor de Some(...) u Ok(...), y detiene el programa con
None o Err(...). Trata mejor el caso en que falta:

  match get(lista, 5)
    Some(x) -> x
    None -> 0
  end
o simplemente:  unwrap_or(get(lista, 5), 0)"#,
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
            r#"E014 — Demasiadas llamadas anidadas

Una función se llama a sí misma (o a otras) demasiadas veces sin terminar.
O nunca llega a su caso base, o hay que escribirla de modo que la llamada
recursiva sea lo último que hace (una llamada final): esas llamadas no tienen
límite.

Limitada:    fn suma([]) = 0
             fn suma([x, ..resto]) = x + suma(resto)      # + ocurre después
Sin límite:  fn suma([], acc) = acc
             fn suma([x, ..resto], acc) = suma(resto, acc + x)
Lo más fácil: sum(lista)  o  fold(lista, 0, fn(a, x) = a + x)"#,
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
            r#"E015 — Aserción fallida

`assert(condición)` o `assert_eq(obtenido, esperado)` encontró que algo no es
como se esperaba. En una prueba, significa que la prueba falla: el mensaje
muestra los valores que comparar.

`rythme check` encuentra antes de ejecutar un `assert_eq` que nunca puede
pasar, porque sus dos valores son de tipos que nunca son iguales:
Mal:     assert_eq(parse_int("3"), 3)      # un Result nunca es un Int
Bien:    assert_eq(parse_int("3"), Ok(3))

`check_all(propiedad, generador)` también señala E015 cuando un valor hace
falsa la propiedad: muestra el valor más simple encontrado, y la semilla que
lo dio (los mismos valores vuelven en cada ejecución; `rythme test --seed N`
prueba otros)."#,
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
            r#"E016 — Mal uso de ?

`x?` saca el valor de Some(...) u Ok(...); con None o Err(...), hace que la
función actual lo devuelva enseguida. Así que solo funciona dentro de una
función, con una Option o un Result.

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
            r#"E017 — Problema con un archivo

No se pudo leer un archivo, o no se encontró un archivo dado a `use`, o unos
archivos se usan entre sí en círculo. Las rutas de `use` son relativas a la
carpeta del archivo que contiene el `use`. Los programas son archivos de texto
guardados en UTF-8 (la opción habitual de los editores).

`input()` también se detiene con este error cuando no queda nada que leer: la
entrada del teclado se cerró (Ctrl-D), o el texto enviado al programa
(`rythme prog.ry < respuestas.txt`) no tiene más líneas."#,
        ),
        (
            r#"E018 — Unexpected character

This character has no meaning in Rythme here. Texts use straight double
quotes "...", powers use **, and comments start with #."#,
            r#"E018 — Carácter inesperado

Este carácter no significa nada en Rythme aquí. Los textos usan comillas
dobles rectas "...", las potencias usan **, y los comentarios empiezan por #."#,
        ),
        (
            r#"E019 — if without else

In Rythme, `if` is an expression: it always gives a value, so both cases are
needed. Each branch holds a single expression; use `do ... end` for several.

Wrong:   if x > 0 then print("positive")
Right:   if x > 0 then print("positive") else ()
Right:   if x > 0 then "positive" else if x == 0 then "zero" else "negative""#,
            r#"E019 — if sin else

En Rythme, `if` es una expresión: siempre da un valor, así que hacen falta los
dos casos. Cada rama contiene una sola expresión; usa `do ... end` para varias.

Mal:     if x > 0 then print("positivo")
Bien:    if x > 0 then print("positivo") else ()
Bien:    if x > 0 then "positivo" else if x == 0 then "cero" else "negativo""#,
        ),
        (
            r#"E020 — Missing end

A function written on several lines, a `match` or a `do` block must finish
with `end` on its own line.

  fn greet(name)
    print("Hello {name}")
  end"#,
            r#"E020 — Falta end

Una función escrita en varias líneas, un `match` o un bloque `do` deben
terminar con `end` en su propia línea.

  fn saludar(nombre)
    print("Hola {nombre}")
  end"#,
        ),
        (
            r#"E021 — Program stopped by fail

The program called `fail(message)`, which stops everything. For errors that
the caller could handle, return Err(message) instead."#,
            r#"E021 — Programa detenido por fail

El programa llamó a `fail(mensaje)`, que lo detiene todo. Para los errores que
quien llama podría tratar, devuelve mejor Err(mensaje)."#,
        ),
        (
            r#"E022 — Impossible conversion

The text does not contain a number. When the text comes from a user or a
file, use the safe versions, which give a Result:

  match parse_int(input("Age? "))
    Ok(age) -> print("Next year: {age + 1}")
    Err(e) -> print(e)
  end"#,
            r#"E022 — Conversión imposible

El texto no contiene un número. Cuando el texto viene de un usuario o de un
archivo, usa las versiones seguras, que dan un Result:

  match parse_int(input("¿Edad? "))
    Ok(edad) -> print("El año que viene: {edad + 1}")
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
            r#"E023 — Valor no válido

La función recibió un valor del tipo correcto, pero fuera de lo que acepta:
una cantidad negativa, el mínimo de una lista vacía, la raíz cuadrada de un
número negativo, una fecha que no existe (30 de febrero), un intervalo, una
lista o un texto demasiado grandes, un número entero de más de 100000
cifras... El mensaje explica el límite."#,
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
            r#"E024 — Se pierde un valor

Una línea calcula un valor, pero nada lo usa: solo la última línea de un
bloque da su valor, y los valores nunca cambian, así que el resultado
simplemente se pierde.

Mal:     total = precio
         - descuento             # una línea nueva: es otra instrucción
Bien:    total = precio -
           descuento             # termina la línea con el operador para seguir
Mal:     x + 1                   # no cambia x
Bien:    y = x + 1
Para mostrar un valor, imprímelo:  print(x + 1)"#,
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
            r#"E025 — Problema de proyecto o de paquete

No se puede leer el `rythme.toml` del proyecto (o el `rythme.lock` escrito por
`rythme install`), una dependencia no está instalada, o `rythme install` no
puede descargar una. Un `rythme.toml` es así:

  name = "hola"                  # un nombre de Rythme en minúsculas
  version = "0.1.0"              # tres números
  entry = "src/main.ry"          # el programa (este por defecto)

  [dependencies]
  mathlib = { git = "https://github.com/alguien/mathlib", tag = "v1.0.0" }
  herramientas = { path = "../herramientas" }

Cada valor es un texto, entre comillas. Una dependencia de git necesita una
versión fija, un `tag` o un `commit`, y el programa `git`
(https://git-scm.com). Tras cambiar `[dependencies]`, ejecuta
`rythme install` en la carpeta del proyecto. Más: `rythme help projects`."#,
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
            r#"E026 — Regex no válida

Una regex (una expresión regular, dada a `matches`, `find_all`, `split_by`,
`replace_all`...) incumple una de sus reglas, o el reemplazo de `replace_all`
usa un grupo que no existe. El mensaje muestra dónde.

Mal:     find_all(texto, "([0-9]+")      # el `(` nunca se cierra
Bien:    find_all(texto, "([0-9]+)")
Mal:     split_by(texto, "*")            # `*` no repite nada
Bien:    split_by(texto, "[*]")          # el signo * en sí, entre corchetes
Mal:     replace_all(fecha, "(\\d+)-(\\d+)", "$3")    # solo 2 grupos
Bien:    replace_all(fecha, "(\\d+)-(\\d+)", "$2/$1")

En un texto de Rythme, la barra invertida se escribe dos veces ("\\d") y las
llaves se escriben \{ y \} ("[0-9]\{4\}"). `rythme help regex` muestra todo lo
que puede decir una regex. Cuando la regex está escrita en el código,
`rythme check` encuentra el error antes de ejecutar."#,
        ),
        (
            r#"W001 — Unused name

A name is defined but never used. It is often a typo or a forgotten step.
If it is on purpose, start the name with `_`: `_unused = ...`."#,
            r#"W001 — Nombre sin usar

Un nombre se define pero nunca se usa. Suele ser una errata o un paso
olvidado. Si es a propósito, empieza el nombre por `_`: `_sin_usar = ...`."#,
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
            r#"W002 — Caso olvidado

Un `match` (o una función con varias definiciones) no trata todos los valores
posibles. El mensaje lista lo que falta. Si llega ese valor, el programa se
detendrá con E006.

  type Semaforo = Rojo | Ambar | Verde
  fn siguiente(Rojo) = Verde
  fn siguiente(Verde) = Ambar
  # W002: `siguiente` no trata `Ambar`. Añade la definición que falta:
  fn siguiente(Ambar) = Rojo"#,
        ),
        (
            r#"W003 — Built-in function hidden

You defined a name that already belongs to a built-in function. In your file,
the name now means your definition. Choose another name to keep both."#,
            r#"W003 — Función integrada oculta

Definiste un nombre que ya pertenece a una función integrada. En tu archivo,
el nombre significa ahora tu definición. Elige otro nombre para conservar las
dos."#,
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
            r#"W004 — Caso nunca alcanzado

Los casos se prueban en orden, y un valor va al primero que le encaja. Un caso
(o una definición de una función) nunca se puede usar cuando:
  - un caso anterior lo acepta todo (`_`, o un simple nombre): pon ese
    al final;
  - un caso anterior ya trata los mismos valores: `1 -> ...` dos veces;
  - el valor nunca puede tener esta forma: un Text nunca encaja con un patrón
    de lista como `[primero, ..resto]`, un número nunca encaja con `Some(x)`;
  - un intervalo está vacío: `9..1` no contiene ningún número, escribe `1..9`.

  match nombre                   # `nombre` es un Text
    [primero, ..] -> primero     # W004: un Text no es una lista
    _ -> "?"
  end
Para mirar los caracteres de un texto, usa `chars(nombre)`."#,
        ),
        (
            r#"W005 — Unknown function after a dot

`value.name(args)` calls the function `name(value, args)`, or the field `name`
of a record. No function has this name, and no record of the file has such a
field: it is probably a typo.

Wrong:   [1, 2].lenght()
Right:   [1, 2].len()"#,
            r#"W005 — Función desconocida tras un punto

`valor.nombre(args)` llama a la función `nombre(valor, args)`, o al campo
`nombre` de un registro. Ninguna función tiene este nombre, y ningún registro
del archivo tiene ese campo: probablemente es una errata.

Mal:     [1, 2].lenght()
Bien:    [1, 2].len()"#,
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
            r#"W006 — Comparación siempre falsa

Los valores de tipos distintos nunca son iguales: Rythme nunca los convierte
sin avisar. Cuando `x` es un número, `x == "1"` siempre es falso y `x != "1"`
siempre verdadero. Suele ser una conversión olvidada, o un valor dentro de una
Option comparado como si estuviera fuera.

Mal:     if cuenta == "3" then ...           (un número y un texto)
Bien:    if cuenta == 3 then ...
Mal:     get(notas, "ada") == 19             (una Option y un número)
Bien:    get(notas, "ada") == Some(19)
Bien:    unwrap_or(get(notas, "ada"), 0) == 19"#,
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
            r#"W007 — Prueba que nunca puede fallar

Una prueba solo falla cuando se detiene con un error, o cuando termina con
`false`, `None` o `Err(...)`. Una prueba que solo calcula o muestra un valor
con funciones que nunca se detienen, y nunca lo compara con lo esperado, nunca
puede fallar: no comprueba nada.

Mal:     test "longitud" = len([1, 2])
Mal:     test "longitud" = print(len([1, 2]))
Bien:    test "longitud" = assert_eq(len([1, 2]), 2)
Bien:    test "longitud" = len([1, 2]) == 2"#,
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

Los valores nunca cambian en Rythme: `push(lista, 4)` no añade 4 a `lista`, da
una lista nueva. Una línea que solo llama a una función así, y no hace nada
con lo que da, no hace nada en absoluto.

Mal:     push(numeros, 4)               # `numeros` no cambia
Bien:    mas = push(numeros, 4)
Mal:     nombre.upper()
Bien:    print(nombre.upper())"#,
        ),
        (
            r#"W009 — A function shown instead of its value

A function shown in a text or printed appears as `<fn name>`. To show what
it gives, call it, with parentheses, even when it takes no argument.

  fn greeting() = "hello"
Wrong:   print("{greeting}")          # shows <fn greeting>
Right:   print("{greeting()}")        # shows hello"#,
            r#"W009 — Una función mostrada en vez de su valor

Una función mostrada en un texto o impresa aparece como `<fn nombre>`. Para
mostrar lo que da, llámala, con paréntesis, aunque no reciba argumentos.

  fn saludo() = "hola"
Mal:     print("{saludo}")            # muestra <fn saludo>
Bien:    print("{saludo()}")          # muestra hola"#,
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
            r#"W010 — Intervalos que se solapan

Un patrón de intervalo `a..b` encaja con los números de `a` a `b`, ambos
incluidos. Cuando dos intervalos de un `match` (o de las definiciones de una
función) comparten números, esos números siempre van al primero: el segundo
nunca los ve. Suele ser un extremo escrito dos veces.

Mal:     match edad
           0..18 -> "joven"
           18..65 -> "adulto"        # W010: 18 va a "joven"
           _ -> "mayor"
         end
Bien:    0..17 -> "joven"
         18..64 -> "adulto"
Para los números decimales, que caen entre 17 y 18, compara mejor:
  a if a < 18 -> "joven""#,
        ),
        (
            r#"W011 — Index or subtraction instead of a call

At the start of a statement or of a result, `show [1, 2]` and `show -1`
call `show` without parentheses, unless the file gives `show` a value
somewhere: with `=`, as a parameter of a function or a block, or in a
pattern. Then they are an index and a subtraction, everywhere in the file,
even where `show` is a function. The value may be given far away, to a
parameter of another function.

Wrong:   fn show(xs) = print(xs)
         fn twice(show) = show * 2
         show [1, 2]                  # W011: an index of the function
Right:   show([1, 2])
Or give the parameter another name: fn twice(n) = n * 2"#,
            r#"W011 — Índice o resta en lugar de una llamada

Al comienzo de una instrucción o de un resultado, `show [1, 2]` y `show -1`
llaman a `show` sin paréntesis, salvo si el archivo da un valor a `show` en
alguna parte: con `=`, como parámetro de una función o de un bloque, o en un
patrón. Entonces son un índice y una resta, en todo el archivo, incluso donde
`show` es una función. El valor puede darse lejos, a un parámetro de otra
función.

Mal:     fn show(xs) = print(xs)
         fn twice(show) = show * 2
         show [1, 2]                  # W011: un índice de la función
Bien:    show([1, 2])
O da otro nombre al parámetro: fn twice(n) = n * 2"#,
        ),
        ("Defines a function: `fn name(x) = expression`, or with its body on the following lines, finished by `end`. Without a name, `fn(x) = x * 2` is an anonymous function.", "Define una función: `fn nombre(x) = expresión`, o con su cuerpo en las líneas siguientes, terminado con `end`. Sin nombre, `fn(x) = x * 2` es una función anónima."),
        ("Chooses between two values: `if condition then value else other`. Every `if` has an `else`, because an `if` always gives a value.", "Elige entre dos valores: `if condición then valor else otro`. Todo `if` tiene un `else`, porque un `if` siempre da un valor."),
        ("Separates the condition of an `if` from the value it gives when the condition is true.", "Separa la condición de un `if` del valor que da cuando la condición es verdadera."),
        ("Gives the value of an `if` when the condition is false. It is always required; `else if` chains several conditions.", "Da el valor de un `if` cuando la condición es falsa. Siempre es obligatorio; `else if` encadena varias condiciones."),
        ("The opposite of `if`: `unless condition then a else b` gives `b` when the condition is true, `a` otherwise. Like `if`, it always has an `else`.", "Lo contrario de `if`: `unless condición then a else b` da `b` cuando la condición es verdadera, y `a` si no. Como `if`, siempre tiene un `else`."),
        ("Chooses according to the shape of a value: one case per line, `pattern -> result`, finished by `end`. Cases are tried in order, and `rythme check` warns about forgotten cases.", "Elige según la forma de un valor: un caso por línea, `patrón -> resultado`, terminado con `end`. Los casos se prueban en orden, y `rythme check` avisa de los casos olvidados."),
        ("Closes a block: a function written on several lines, a `match`, a `do` or a `test`.", "Cierra un bloque: una función escrita en varias líneas, un `match`, un `do` o un `test`."),
        ("Groups several steps inside an expression, up to `end`. The last line gives the value. After a call, `do |x| ... end` is a block: a function given as the last argument.", "Agrupa varios pasos dentro de una expresión, hasta `end`. La última línea da el valor. Después de una llamada, `do |x| ... end` es un bloque: una función dada como último argumento."),
        ("Defines your own type and its variants. Each variant is a constructor, with or without fields.", "Define tu propio tipo y sus variantes. Cada variante es un constructor, con o sin campos."),
        ("Defines a test, next to the code. `rythme test file.ry` runs the tests of the file.", "Define una prueba, junto al código. `rythme test archivo.ry` ejecuta las pruebas del archivo."),
        ("Uses another file through its name: `use geometry`, then `geometry.area(2)`. The path is relative to the folder of the file that contains `use`.", "Usa otro archivo a través de su nombre: `use geometria`, y luego `geometria.area(2)`. La ruta es relativa a la carpeta del archivo que contiene `use`."),
        ("True when both conditions are true. The second one is computed only if the first one is true.", "Verdadero cuando las dos condiciones son verdaderas. La segunda solo se calcula si la primera es verdadera."),
        ("True when at least one of the conditions is true. The second one is computed only if the first one is false.", "Verdadero cuando al menos una de las condiciones es verdadera. La segunda solo se calcula si la primera es falsa."),
        ("Inverts a condition: `not true` is `false`.", "Invierte una condición: `not true` es `false`."),
        ("One of the two values of the type Bool.", "Uno de los dos valores del tipo Bool."),
        ("Shows values on the screen, separated by spaces, then goes to the next line.", "Muestra valores en la pantalla, separados por espacios, y luego pasa a la línea siguiente."),
        ("Shows a value with its position in the code (on the error output) and returns it unchanged. Handy in the middle of a pipeline.", "Muestra un valor con su posición en el código (en la salida de errores) y lo devuelve sin cambios. Útil en medio de una cadena."),
        ("Converts any value to text. With a number of decimals, writes a number with exactly that many digits after the point.", "Convierte cualquier valor en texto. Con un número de decimales, escribe un número con exactamente esas cifras después del punto."),
        ("Shows the prompt, then reads a line typed on the keyboard and returns it as text.", "Muestra el mensaje, luego lee una línea escrita en el teclado y la devuelve como texto."),
        ("Reads all the lines of the standard input, until its end: `Ok(list of texts)`, or `Err(message)` if it cannot be read. For `rythme count.ry < names.txt`, or text typed until Ctrl-D (Ctrl-Z then Enter on Windows).", "Lee todas las líneas de la entrada estándar, hasta su final: `Ok(lista de textos)`, o `Err(mensaje)` si no se puede leer. Para `rythme contar.ry < nombres.txt`, o texto escrito hasta Ctrl-D (Ctrl-Z y luego Intro en Windows)."),
        ("Returns the name of the type of a value: \"Int\", \"Float\", \"Text\", \"Bool\", \"List\", \"Record\", \"Function\", \"Unit\" (for `()`), \"Symbol\" (for `:ok`), \"Option\", \"Result\", or the name of your own type.", "Devuelve el nombre del tipo de un valor: \"Int\", \"Float\", \"Text\", \"Bool\", \"List\", \"Record\", \"Function\", \"Unit\" (para `()`), \"Symbol\" (para `:ok`), \"Option\", \"Result\", o el nombre de tu propio tipo."),
        ("Shows help: about a function (`help(map)`), a topic (`help(\"lists\")`), or what you can do with a value (`help([1, 2])`).", "Muestra la ayuda: sobre una función (`help(map)`), un tema (`help(\"lists\")`), o lo que puedes hacer con un valor (`help([1, 2])`)."),
        ("Stops with an error if the condition is false. Mostly used in tests.", "Se detiene con un error si la condición es falsa. Se usa sobre todo en las pruebas."),
        ("Stops with an error showing both values if they are different. Mostly used in tests.", "Se detiene con un error que muestra los dos valores si son distintos. Se usa sobre todo en las pruebas."),
        ("Stops the program with an error message. Prefer returning `Err(...)` for errors that can be handled.", "Detiene el programa con un mensaje de error. Mejor devuelve `Err(...)` para los errores que se pueden tratar."),
        ("Returns the list of the words given after the file name on the command line.", "Devuelve la lista de las palabras dadas después del nombre del archivo en la línea de comandos."),
        ("Reads the options of the command line (the words of `args()`, or the list given) into a record, in a Result: `--name=value` gives the text `value`, `--name` and `-n` give `true`, a `-` in a name becomes `_`, and the other words are in the field `args` (all of them after `--`). An option given twice, or a word like `-abc`, gives `Err(message)`. Use `get(opts, \"name\")` or `has(opts, \"name\")` for an option that may be missing.", "Lee las opciones de la línea de comandos (las palabras de `args()`, o la lista dada) en un registro, dentro de un Result: `--nombre=valor` da el texto `valor`, `--nombre` y `-n` dan `true`, un `-` en un nombre se vuelve `_`, y las demás palabras están en el campo `args` (todas las que van después de `--`). Una opción dada dos veces, o una palabra como `-abc`, da `Err(mensaje)`. Usa `get(opts, \"nombre\")` o `has(opts, \"nombre\")` para una opción que puede faltar."),
        ("The value of an environment variable of the system, as an Option: `Some(text)`, or `None` if it is not set.", "El valor de una variable de entorno del sistema, como Option: `Some(texto)`, o `None` si no está definida."),
        ("Stops the program immediately. The code (0 by default) tells the system if everything went well (0) or not; Rythme itself exits with 1 after an error in the program (see `rythme help system`).", "Detiene el programa enseguida. El código (0 por defecto) dice al sistema si todo fue bien (0) o no; el propio Rythme sale con 1 tras un error en el programa (consulta `rythme help system`)."),
        ("Returns the current time in seconds (a Float), counted from 1 January 1970. Subtract two times to measure a duration.", "Devuelve la hora actual en segundos (un Float), contados desde el 1 de enero de 1970. Resta dos horas para medir una duración."),
        ("random() gives a decimal number between 0 and 1; random(a, b) a whole number from a to b (included); random(list) a random element.", "random() da un número decimal entre 0 y 1; random(a, b) un número entero de a a b (incluido); random(lista) un elemento al azar."),
        ("Absolute value: the number without its sign.", "Valor absoluto: el número sin su signo."),
        ("The smallest of the values, or of the elements of a list.", "El más pequeño de los valores, o de los elementos de una lista."),
        ("The largest of the values, or of the elements of a list.", "El más grande de los valores, o de los elementos de una lista."),
        ("Square root.", "Raíz cuadrada."),
        ("x to the power n (same as `x ** n`).", "x elevado a n (igual que `x ** n`)."),
        ("Rounds down to a whole number.", "Redondea hacia abajo a un número entero."),
        ("Rounds up to a whole number.", "Redondea hacia arriba a un número entero."),
        ("Rounds to the nearest whole number, or to the given number of decimal digits.", "Redondea al número entero más cercano, o al número de decimales dado."),
        ("Converts to a whole number (Int): decimals are cut, texts are read.", "Convierte en un número entero (Int): los decimales se cortan, los textos se leen."),
        ("Converts to a decimal number (Float).", "Convierte en un número decimal (Float)."),
        ("Sine (x in radians).", "Seno (x en radianes)."),
        ("Cosine (x in radians).", "Coseno (x en radianes)."),
        ("Tangent (x in radians).", "Tangente (x en radianes)."),
        ("Arc sine: the angle (in radians) whose sine is x, for x from -1 to 1.", "Arcoseno: el ángulo (en radianes) cuyo seno es x, para x de -1 a 1."),
        ("Arc cosine: the angle (in radians) whose cosine is x, for x from -1 to 1.", "Arcocoseno: el ángulo (en radianes) cuyo coseno es x, para x de -1 a 1."),
        ("Arc tangent: the angle (in radians) whose tangent is y. With two numbers, the angle of the point (x, y), from -pi to pi.", "Arcotangente: el ángulo (en radianes) cuya tangente es y. Con dos números, el ángulo del punto (x, y), de -pi a pi."),
        ("Logarithm: natural (base e) by default, or in the given base.", "Logaritmo: natural (base e) por defecto, o en la base dada."),
        ("e to the power x.", "e elevado a x."),
        ("Is the whole number even?", "¿Es par el número entero?"),
        ("Is the whole number odd?", "¿Es impar el número entero?"),
        ("Greatest common divisor of two whole numbers: the largest number that divides both.", "Máximo común divisor de dos números enteros: el mayor número que divide a los dos."),
        ("Least common multiple of two whole numbers: the smallest number divisible by both.", "Mínimo común múltiplo de dos números enteros: el menor número divisible por los dos."),
        ("Length of a text (in characters), a list, a record, a dict or a set.", "Longitud de un texto (en caracteres), una lista, un registro, un dict o un conjunto."),
        ("The text in uppercase.", "El texto en mayúsculas."),
        ("The text in lowercase.", "El texto en minúsculas."),
        ("The text with its first character in uppercase (the rest is unchanged).", "El texto con su primer carácter en mayúscula (el resto no cambia)."),
        ("The text without spaces and line breaks at the start and the end.", "El texto sin espacios ni saltos de línea al principio y al final."),
        ("Adds `fill` (a space by default) before the text until it is `width` characters long: numbers line up in columns. A value that is not a text is first shown as `print` shows it.", "Añade `fill` (un espacio por defecto) delante del texto hasta que mida `width` caracteres: los números se alinean en columnas. Un valor que no es un texto se muestra antes como lo muestra `print`."),
        ("Adds `fill` (a space by default) after the text until it is `width` characters long.", "Añade `fill` (un espacio por defecto) detrás del texto hasta que mida `width` caracteres."),
        ("Cuts a text into a list of texts. Without separator, cuts at spaces.", "Corta un texto en una lista de textos. Sin separador, corta en los espacios."),
        ("Glues the elements of a list into one text, with the separator between them.", "Pega los elementos de una lista en un solo texto, con el separador entre ellos."),
        ("Does the text contain this piece of text? Does the list (or the set) contain this element? Does the record have this field, or the dict this key?", "¿Contiene el texto este trozo de texto? ¿Contiene la lista (o el conjunto) este elemento? ¿Tiene el registro este campo, o el dict esta clave?"),
        ("Does the text start with this piece of text?", "¿Empieza el texto por este trozo de texto?"),
        ("Does the text end with this piece of text?", "¿Termina el texto con este trozo de texto?"),
        ("Replaces every occurrence of `old` by `new`.", "Reemplaza cada aparición de `old` por `new`."),
        ("The list of the characters of a text.", "La lista de los caracteres de un texto."),
        ("The list of the lines of a text.", "La lista de las líneas de un texto."),
        ("The number of a character (its Unicode code point).", "El número de un carácter (su punto de código Unicode)."),
        ("The character with this number (Unicode code point): the opposite of `char_code`.", "El carácter con este número (punto de código Unicode): lo contrario de `char_code`."),
        ("Is the text made only of digits from 0 to 9 (and not empty)?", "¿Está hecho el texto solo de cifras del 0 al 9 (y no vacío)?"),
        ("Is the text made only of letters, accented ones included (and not empty)?", "¿Está hecho el texto solo de letras, también con tilde (y no vacío)?"),
        ("Is the text made only of spaces, tabs and line breaks (and not empty)?", "¿Está hecho el texto solo de espacios, tabuladores y saltos de línea (y no vacío)?"),
        ("A text or a list repeated n times.", "Un texto o una lista repetidos n veces."),
        ("Reads a whole number from a text: `Ok(number)`, or `Err(message)` if the text is not a number.", "Lee un número entero de un texto: `Ok(número)`, o `Err(mensaje)` si el texto no es un número."),
        ("Reads a decimal number from a text: `Ok(number)` or `Err(message)`.", "Lee un número decimal de un texto: `Ok(número)` o `Err(mensaje)`."),
        ("Reads a JSON text: `Ok(value)`, or `Err(message)` if the text is not valid JSON. Objects become records, arrays lists, and null `None`.", "Lee un texto JSON: `Ok(valor)`, o `Err(mensaje)` si el texto no es JSON válido. Los objetos se vuelven registros, los arrays listas, y null `None`."),
        ("Writes a value as JSON text: records become objects, lists arrays, and `None` null. With `indent`, one element per line, shifted by that many spaces.", "Escribe un valor como texto JSON: los registros se vuelven objetos, las listas arrays, y `None` null. Con `indent`, un elemento por línea, desplazado ese número de espacios."),
        ("The text without spaces and line breaks at the start.", "El texto sin espacios ni saltos de línea al principio."),
        ("The text without spaces and line breaks at the end.", "El texto sin espacios ni saltos de línea al final."),
        ("The list of the words of a text: letters and digits, accents included, with `'` or `-` inside a word. Punctuation and spaces separate them.", "La lista de las palabras de un texto: letras y cifras, tildes incluidas, con `'` o `-` dentro de una palabra. Los signos de puntuación y los espacios las separan."),
        ("Are the two texts the same when uppercase and lowercase letters count as equal?", "¿Son iguales los dos textos cuando las mayúsculas y las minúsculas cuentan como iguales?"),
        ("Writes a number for people: thousands separated, with exactly `decimals` decimals if given, in the style of a language: \"en\" (1,234.5, by default), \"fr\" (1 234,5), \"de\", \"es\", \"it\", \"nl\" or \"pt\" (1.234,5). To line it up in a column, use `pad_left`.", "Escribe un número para las personas: miles separados, con exactamente `decimals` decimales si se dan, al estilo de un idioma: \"en\" (1,234.5, por defecto), \"fr\" (1 234,5), \"de\", \"es\", \"it\", \"nl\" o \"pt\" (1.234,5). Para alinearlo en una columna, usa `pad_left`."),
        ("Does the whole text match the regex (a regular expression: see `rythme help regex`)? To look for the regex anywhere in the text, use `find_first`.", "¿Encaja el texto entero con la regex (una expresión regular: consulta `rythme help regex`)? Para buscar la regex en cualquier parte del texto, usa `find_first`."),
        ("The first piece of the text that matches the regex, as an Option: `Some(piece)`, or `None` if there is none.", "El primer trozo del texto que encaja con la regex, como Option: `Some(trozo)`, o `None` si no hay ninguno."),
        ("Every piece of the text that matches the regex, from left to right.", "Todos los trozos del texto que encajan con la regex, de izquierda a derecha."),
        ("The first match of the regex, as an Option of a list: the whole piece, then the text of each group `(...)`, in order (\"\" for a group that matched nothing).", "La primera coincidencia de la regex, como Option de una lista: el trozo entero, y luego el texto de cada grupo `(...)`, en orden (\"\" para un grupo que no encajó con nada)."),
        ("Like `captures`, for every match: a list of lists.", "Como `captures`, para cada coincidencia: una lista de listas."),
        ("The first match of a regex with named groups `(?<name>...)`, as an Option of a record: one field for each name.", "La primera coincidencia de una regex con grupos con nombre `(?<nombre>...)`, como Option de un registro: un campo para cada nombre."),
        ("Cuts a text at every piece that matches the regex.", "Corta un texto en cada trozo que encaja con la regex."),
        ("Replaces every piece that matches the regex. `new` is a text, where `$1` stands for the text of group 1 (`$<name>` for a named group, `$0` for the whole piece, `$$` for a `$`), or a function that receives each piece and gives its replacement.", "Reemplaza cada trozo que encaja con la regex. `new` es un texto, donde `$1` representa el texto del grupo 1 (`$<nombre>` para un grupo con nombre, `$0` para el trozo entero, `$$` para un `$`), o una función que recibe cada trozo y da su reemplazo."),
        ("Applies the function f to each element and returns the list of the results. Also works on Option and Result, and on a set (giving the set of the results).", "Aplica la función f a cada elemento y devuelve la lista de los resultados. También funciona con Option y Result, y con un conjunto (dando el conjunto de los resultados)."),
        ("Keeps only the elements for which f returns true. Also works on a set, and on a dict, where f receives each key and its value.", "Se queda solo con los elementos para los que f devuelve true. También funciona con un conjunto, y con un dict, donde f recibe cada clave y su valor."),
        ("Combines all the elements into one value: starts with `start`, then computes f(result, element) for each element.", "Combina todos los elementos en un solo valor: empieza con `start`, y luego calcula f(resultado, elemento) para cada elemento."),
        ("Like fold, but gives the list of every intermediate result: running totals, for example.", "Como fold, pero da la lista de todos los resultados intermedios: totales parciales, por ejemplo."),
        ("Calls f on each element, for its effect (like printing). Returns (). Also works on a set, and on a dict, where f receives each key and its value.", "Llama a f con cada elemento, por su efecto (como mostrar). Devuelve (). También funciona con un conjunto, y con un dict, donde f recibe cada clave y su valor."),
        ("The sum of the numbers of a list; with f, the sum of f(x) for each element.", "La suma de los números de una lista; con f, la suma de f(x) para cada elemento."),
        ("The product of the numbers of a list.", "El producto de los números de una lista."),
        ("The first element of a list (or character of a text), as an Option: `Some(x)`, or `None` if there is none. With n, the first n elements (like `take`).", "El primer elemento de una lista (o carácter de un texto), como Option: `Some(x)`, o `None` si no hay ninguno. Con n, los n primeros elementos (como `take`)."),
        ("The last element of a list (or character of a text), as an Option. With n, the last n elements.", "El último elemento de una lista (o carácter de un texto), como Option. Con n, los n últimos elementos."),
        ("The list (or text) without its first element.", "La lista (o el texto) sin su primer elemento."),
        ("The element at a position of a list or text, the field of a record, or the value of a key of a dict, as an Option (never an error). Negative positions count from the end.", "El elemento en una posición de una lista o de un texto, el campo de un registro, o el valor de una clave de un dict, como Option (nunca un error). Las posiciones negativas cuentan desde el final."),
        ("A new list with x added at the end; or a new set with x added.", "Una lista nueva con x añadido al final; o un conjunto nuevo con x añadido."),
        ("A new list where the element at position i is replaced by x. Negative positions count from the end.", "Una lista nueva en la que el elemento de la posición i se reemplaza por x. Las posiciones negativas cuentan desde el final."),
        ("A new list with x inserted so that it is at position i (-1 adds it at the end).", "Una lista nueva con x insertado para que quede en la posición i (-1 lo añade al final)."),
        ("A new list without the element at position i. Negative positions count from the end.", "Una lista nueva sin el elemento de la posición i. Las posiciones negativas cuentan desde el final."),
        ("The list or text in reverse order.", "La lista o el texto en orden inverso."),
        ("The list sorted from smallest to largest (numbers, texts in alphabetical order...). For the largest first: `sort(list) |> reverse`.", "La lista ordenada de menor a mayor (números, textos en orden alfabético...). Para el mayor primero: `sort(lista) |> reverse`."),
        ("Sorts the list by the value that the function `key` gives for each element. Elements with the same value keep their order.", "Ordena la lista según el valor que da la función `key` para cada elemento. Los elementos con el mismo valor conservan su orden."),
        ("The list without duplicates (the first occurrence is kept).", "La lista sin repetidos (se conserva la primera aparición)."),
        ("The elements of the list in a random order.", "Los elementos de la lista en un orden aleatorio."),
        ("The part of a list or text from position `start` (included) to `end` (excluded). Negative positions count from the end.", "La parte de una lista o de un texto desde la posición `start` (incluida) hasta `end` (excluida). Las posiciones negativas cuentan desde el final."),
        ("The first n elements of a list or text.", "Los n primeros elementos de una lista o de un texto."),
        ("The list or text without its first n elements.", "La lista o el texto sin sus n primeros elementos."),
        ("The first elements of a list (or characters of a text), as long as f is true for them.", "Los primeros elementos de una lista (o caracteres de un texto), mientras f sea verdadera para ellos."),
        ("The list (or text) without its first elements for which f is true.", "La lista (o el texto) sin sus primeros elementos para los que f es verdadera."),
        ("One element out of n of a list (or text), starting with the first. On a range: the numbers with a step.", "Un elemento de cada n de una lista (o de un texto), empezando por el primero. En un intervalo: los números con un paso."),
        ("Cuts a list (or text) into pieces of n elements; the last piece may be shorter.", "Corta una lista (o un texto) en trozos de n elementos; el último trozo puede ser más corto."),
        ("Every group of n neighbouring elements of a list (or text), in order: handy to compare each element with the next one.", "Cada grupo de n elementos vecinos de una lista (o de un texto), en orden: útil para comparar cada elemento con el siguiente."),
        ("Pairs the elements of two lists: [[a1, b1], [a2, b2], ...]. It stops at the end of the shorter list.", "Empareja los elementos de dos listas: [[a1, b1], [a2, b2], ...]. Se detiene al final de la lista más corta."),
        ("Pairs each element with its position: [[0, x0], [1, x1], ...].", "Empareja cada elemento con su posición: [[0, x0], [1, x1], ...]."),
        ("Is f true for at least one element?", "¿Es f verdadera para al menos un elemento?"),
        ("Is f true for every element?", "¿Es f verdadera para todos los elementos?"),
        ("The first element for which f is true, as an Option.", "El primer elemento para el que f es verdadera, como Option."),
        ("How many elements make f true; without f, how many elements there are.", "Cuántos elementos hacen verdadera a f; sin f, cuántos elementos hay."),
        ("The elements for which f is false: the opposite of `filter`. Also works on a set and on a dict, like `filter`.", "Los elementos para los que f es falsa: lo contrario de `filter`. También funciona con un conjunto y con un dict, como `filter`."),
        ("Calls f(element, position) for each element, in order (positions start at 0). For effects, like `print`.", "Llama a f(elemento, posición) para cada elemento, en orden (las posiciones empiezan en 0). Para efectos, como `print`."),
        ("The position of an element in a list (or of a piece of text in a text), as an Option.", "La posición de un elemento en una lista (o de un trozo de texto en un texto), como Option."),
        ("The element for which the function `key` gives the smallest value (the first one if there are several).", "El elemento para el que la función `key` da el menor valor (el primero si hay varios)."),
        ("The element for which the function `key` gives the largest value (the first one if there are several).", "El elemento para el que la función `key` da el mayor valor (el primero si hay varios)."),
        ("Joins a list of lists into one list.", "Junta una lista de listas en una sola lista."),
        ("Like map, when f returns a list: the lists are joined.", "Como map, cuando f devuelve una lista: las listas se juntan."),
        ("Splits a list in two: [elements where f is true, elements where f is false].", "Divide una lista en dos: [elementos donde f es verdadera, elementos donde f es falsa]."),
        ("Groups the elements in a record: the key is the text of what `key` returns, the value the list of matching elements.", "Agrupa los elementos en un registro: la clave es el texto de lo que devuelve `key`, el valor la lista de los elementos correspondientes."),
        ("Is the list, text, record, dict or set empty?", "¿Está vacío la lista, el texto, el registro, el dict o el conjunto?"),
        ("The list of the field names of a record, or of the keys of a dict (in order).", "La lista de los nombres de los campos de un registro, o de las claves de un dict (en orden)."),
        ("The list of the values of a record or of a dict (in the order of its keys), or of the elements of a set (in order).", "La lista de los valores de un registro o de un dict (en el orden de sus claves), o de los elementos de un conjunto (en orden)."),
        ("The list of the [name, value] pairs of a record, or of the [key, value] pairs of a dict.", "La lista de los pares [nombre, valor] de un registro, o de los pares [clave, valor] de un dict."),
        ("Builds a record from a list of [name, value] pairs: the opposite of `entries`.", "Construye un registro a partir de una lista de pares [nombre, valor]: lo contrario de `entries`."),
        ("Does the record have this field? Does the dict have this key, or the set this element?", "¿Tiene el registro este campo? ¿Tiene el dict esta clave, o el conjunto este elemento?"),
        ("A new record with the field set to the value (added or replaced); or a new dict with the key set to the value.", "Un registro nuevo con el campo puesto al valor (añadido o reemplazado); o un dict nuevo con la clave puesta al valor."),
        ("A new record without this field; or a new dict without this key, or a new set without this element.", "Un registro nuevo sin este campo; o un dict nuevo sin esta clave, o un conjunto nuevo sin este elemento."),
        ("A new record (or dict) where f is applied to every value; the names (or keys) stay the same.", "Un registro (o dict) nuevo en el que f se aplica a cada valor; los nombres (o las claves) no cambian."),
        ("A dictionary: keys of any kind (but functions), each with a value, kept in the order of the keys. Made from a list of [key, value] pairs (the last value of a key wins), from a record, or empty with `dict()`. Use it with get, put, has, remove, keys, values, entries, len, map_values, filter, union...", "Un diccionario: claves de cualquier tipo (salvo funciones), cada una con un valor, guardadas en el orden de las claves. Se crea a partir de una lista de pares [clave, valor] (gana el último valor de una clave), de un registro, o vacío con `dict()`. Se usa con get, put, has, remove, keys, values, entries, len, map_values, filter, union..."),
        ("A set: values (but functions) without repetition, kept in order. Made from a list, or empty with `set()`. Use it with has, push, remove, len, values, filter, map, union, intersection, difference, is_subset.", "Un conjunto: valores (salvo funciones) sin repetición, guardados en orden. Se crea a partir de una lista, o vacío con `set()`. Se usa con has, push, remove, len, values, filter, map, union, intersection, difference, is_subset."),
        ("The elements of two sets together; or the pairs of two dicts, where the value of b wins for a key in both.", "Los elementos de dos conjuntos juntos; o los pares de dos dicts, donde gana el valor de b para una clave que está en los dos."),
        ("The elements of the set a that are also in b; or the pairs of the dict a whose key is in b.", "Los elementos del conjunto a que también están en b; o los pares del dict a cuya clave está en b."),
        ("The elements of the set a that are not in b; or the pairs of the dict a whose key is not in b.", "Los elementos del conjunto a que no están en b; o los pares del dict a cuya clave no está en b."),
        ("Is every element of the set a also in the set b?", "¿Está cada elemento del conjunto a también en el conjunto b?"),
        ("The value inside `Some(x)` or `Ok(x)`. Stops the program with an error for `None` or `Err(e)`.", "El valor dentro de `Some(x)` u `Ok(x)`. Detiene el programa con un error para `None` o `Err(e)`."),
        ("The value inside `Some(x)` or `Ok(x)`, or the default value for `None` or `Err(e)`.", "El valor dentro de `Some(x)` u `Ok(x)`, o el valor por defecto para `None` o `Err(e)`."),
        ("Is it `Some(...)`?", "¿Es `Some(...)`?"),
        ("Is it `None`?", "¿Es `None`?"),
        ("Is it `Ok(...)`?", "¿Es `Ok(...)`?"),
        ("Is it `Err(...)`?", "¿Es `Err(...)`?"),
        ("Chains steps that may fail: for `Some(x)` or `Ok(x)`, gives f(x), which must be an Option or a Result; `None` and `Err(e)` stay as they are.", "Encadena pasos que pueden fallar: para `Some(x)` u `Ok(x)`, da f(x), que debe ser una Option o un Result; `None` y `Err(e)` se quedan como están."),
        ("Turns an Option into a Result: `Some(x)` gives `Ok(x)`, and `None` gives `Err(error)`. Handy before `?`.", "Convierte una Option en un Result: `Some(x)` da `Ok(x)`, y `None` da `Err(error)`. Útil antes de `?`."),
        ("For `Err(e)`, gives `Err(f(e))`; `Ok(x)` stays as it is. Handy to explain an error better.", "Para `Err(e)`, da `Err(f(e))`; `Ok(x)` se queda como está. Útil para explicar mejor un error."),
        ("Reads a whole text file: `Ok(text)`, or `Err(message)` if it cannot be read.", "Lee un archivo de texto entero: `Ok(texto)`, o `Err(mensaje)` si no se puede leer."),
        ("Writes a text in a file (replacing its content): `Ok(())` or `Err(message)`.", "Escribe un texto en un archivo (reemplazando su contenido): `Ok(())` o `Err(mensaje)`."),
        ("Adds a text at the end of a file, creating the file if needed: `Ok(())` or `Err(message)`.", "Añade un texto al final de un archivo, creando el archivo si hace falta: `Ok(())` o `Err(mensaje)`."),
        ("Does this file (or folder) exist? Always `false` in the browser.", "¿Existe este archivo (o carpeta)? Siempre `false` en el navegador."),
        ("What is known about a file or a folder: `Ok({is_folder, size})`, with its size in bytes (0 for a folder), or `Err(message)` if it does not exist or cannot be read.", "Lo que se sabe de un archivo o de una carpeta: `Ok({is_folder, size})`, con su tamaño en bytes (0 para una carpeta), o `Err(mensaje)` si no existe o no se puede leer."),
        ("The names of the files and folders inside a folder, in alphabetical order: `Ok(list of texts)`, or `Err(message)`. `list_folder(\".\")` lists the current folder.", "Los nombres de los archivos y carpetas que hay dentro de una carpeta, en orden alfabético: `Ok(lista de textos)`, o `Err(mensaje)`. `list_folder(\".\")` lista la carpeta actual."),
        ("Creates a folder, and the folders around it that are missing: `Ok(())`, also when it already exists, or `Err(message)`.", "Crea una carpeta, y las carpetas que falten a su alrededor: `Ok(())`, también cuando ya existe, o `Err(mensaje)`."),
        ("Deletes one file (never a folder: that is an error): `Ok(())` or `Err(message)`. A deleted file cannot be brought back.", "Borra un archivo (nunca una carpeta: eso es un error): `Ok(())` o `Err(mensaje)`. Un archivo borrado no se puede recuperar."),
        ("Joins the parts of a path with `/`, which works on Linux, macOS and Windows (with `\\` if the parts already use only `\\`). Only the first part can start at a root (`/`, `C:`): a part like \"/etc\" after another one is an error.", "Junta las partes de una ruta con `/`, que funciona en Linux, macOS y Windows (con `\\` si las partes ya usan solo `\\`). Solo la primera parte puede empezar en una raíz (`/`, `C:`): una parte como \"/etc\" después de otra es un error."),
        ("The parts of a path, split at each `/` or `\\`; the first part of an absolute path is its root (`/`, `C:\\`...). `join_path` puts them back together.", "Las partes de una ruta, cortada en cada `/` o `\\`; la primera parte de una ruta absoluta es su raíz (`/`, `C:\\`...). `join_path` las vuelve a juntar."),
        ("The date and time now, where the program runs: a record {year, month, day, hour, minute, second, offset}, where `offset` is the number of minutes the local time is ahead of UTC (0 when the system does not tell).", "La fecha y la hora de ahora, donde se ejecuta el programa: un registro {year, month, day, hour, minute, second, offset}, donde `offset` es el número de minutos que la hora local va por delante de UTC (0 cuando el sistema no lo dice)."),
        ("The date of today, where the program runs: a record {year, month, day}.", "La fecha de hoy, donde se ejecuta el programa: un registro {year, month, day}."),
        ("Reads a date written as ISO 8601 (\"2024-02-29\", \"2024-02-29T13:05:00\", with `Z` or an offset like `+01:00` if needed): `Ok(date)`, or `Err(message)` if it is not a valid date.", "Lee una fecha escrita en ISO 8601 (\"2024-02-29\", \"2024-02-29T13:05:00\", con `Z` o un desfase como `+01:00` si hace falta): `Ok(fecha)`, o `Err(mensaje)` si no es una fecha válida."),
        ("Writes a date as ISO 8601: \"2024-02-29\", with the time if the date has one, and `Z` or the offset if it has one.", "Escribe una fecha en ISO 8601: \"2024-02-29\", con la hora si la fecha la tiene, y `Z` o el desfase si lo tiene."),
        ("Is the value a valid date: a record {year, month, day} (with `hour`, `minute`, `second`, `offset` if needed) for a day that exists?", "¿Es el valor una fecha válida: un registro {year, month, day} (con `hour`, `minute`, `second`, `offset` si hace falta) para un día que existe?"),
        ("The date n days later (or earlier, when n is negative), at the same time of day.", "La fecha n días después (o antes, cuando n es negativo), a la misma hora del día."),
        ("The date and time n seconds later (or earlier, when n is negative). Add `60 * minutes` or `3600 * hours` the same way.", "La fecha y la hora n segundos después (o antes, cuando n es negativo). Suma `60 * minutos` o `3600 * horas` de la misma forma."),
        ("The number of whole days from date a to date b: negative when b is before a.", "El número de días completos de la fecha a a la fecha b: negativo cuando b es anterior a a."),
        ("The number of seconds from date a to date b, taking their offsets into account: negative when b is before a.", "El número de segundos de la fecha a a la fecha b, teniendo en cuenta sus desfases: negativo cuando b es anterior a a."),
        ("The day of the week of a date, from 1 (Monday) to 7 (Sunday), as in ISO 8601.", "El día de la semana de una fecha, de 1 (lunes) a 7 (domingo), como en ISO 8601."),
        ("The number of days of a month (from 1 to 12) of a year: 28 to 31.", "El número de días de un mes (de 1 a 12) de un año: de 28 a 31."),
        ("Does the year have a 29 February? Every 4 years, except every 100 years, except every 400 years.", "¿Tiene el año un 29 de febrero? Cada 4 años, salvo cada 100 años, salvo cada 400 años."),
        ("Checks that a property holds for many values made by a generator (100, or `cases`): the function is called with each value and must give `true` (or `()`, `Some(...)`, `Ok(...)`). When a value makes it false or stops it with an error, that value is simplified as much as possible (shrinking) and shown, with the seed that made it: the same values come back on every run. The property may also come last, as a block: `check_all(ints()) { |x| x + 0 == x }`.", "Comprueba que una propiedad se cumple para muchos valores creados por un generador (100, o `cases`): la función se llama con cada valor y debe dar `true` (o `()`, `Some(...)`, `Ok(...)`). Cuando un valor la hace falsa o la detiene con un error, ese valor se simplifica todo lo posible (shrinking) y se muestra, con la semilla que lo produjo: los mismos valores vuelven en cada ejecución. La propiedad también puede ir al final, como un bloque: `check_all(ints()) { |x| x + 0 == x }`."),
        ("A list of `n` values (10 if not given) made by a generator, to see what `check_all` will try.", "Una lista de `n` valores (10 si no se da) creados por un generador, para ver lo que probará `check_all`."),
        ("A generator of whole numbers from `min` to `max` (from -1000 to 1000 if not given), often near 0 and at the edges. A failing number is simplified towards 0.", "Un generador de números enteros de `min` a `max` (de -1000 a 1000 si no se dan), a menudo cerca de 0 y en los extremos. Un número que falla se simplifica hacia 0."),
        ("A generator of decimal numbers from `min` to `max` (from -1000.0 to 1000.0 if not given).", "Un generador de números decimales de `min` a `max` (de -1000.0 a 1000.0 si no se dan)."),
        ("A generator of texts of at most `max_length` characters (20 if not given): letters mostly, with digits, spaces, punctuation and accents. A failing text is made shorter and simpler.", "Un generador de textos de `max_length` caracteres como máximo (20 si no se da): sobre todo letras, con cifras, espacios, signos de puntuación y tildes. Un texto que falla se vuelve más corto y más simple."),
        ("A generator of `true` and `false`.", "Un generador de `true` y `false`."),
        ("A generator of lists of at most `max_length` elements (10 if not given), each made by `generator`. A failing list is made shorter, then its elements simpler.", "Un generador de listas de `max_length` elementos como máximo (10 si no se da), cada uno creado por `generator`. Una lista que falla se vuelve más corta, y luego sus elementos más simples."),
        ("A generator that picks one of the values of a list. A failing value is replaced by one written before it.", "Un generador que elige uno de los valores de una lista. Un valor que falla se reemplaza por uno escrito antes que él."),
        ("A generator of records, with a generator for each field: `records_of({name: texts(), age: ints(0, 120)})`. Handy to give several values to `check_all`.", "Un generador de registros, con un generador para cada campo: `records_of({nombre: texts(), edad: ints(0, 120)})`. Útil para dar varios valores a `check_all`."),
        ("January", "enero"),
        ("February", "febrero"),
        ("March", "marzo"),
        ("April", "abril"),
        ("May", "mayo"),
        ("June", "junio"),
        ("July", "julio"),
        ("August", "agosto"),
        ("September", "septiembre"),
        ("October", "octubre"),
        ("November", "noviembre"),
        ("December", "diciembre"),
        // docs/exercises: the exercises of `rythme exercise` (see src/exercise.rs)
        ("# --- Tests (do not change them) ---", "# --- Pruebas (no las cambies) ---"),
        ("to do", "por hacer"),
        ("01-values", "01-valores"),
        (
            r#"# Exercise 01 · First values  ★☆☆
#
# A name receives a value with `=` and keeps it forever: in Rythme, values
# never change.
#
# Statement
#   `seconds_per_minute` is given. Replace each `0` (and the empty text)
#   below with an expression that uses the names defined before it:
#   - `seconds_per_hour`: the number of seconds in one hour;
#   - `seconds_per_day`: the number of seconds in one day;
#   - `seconds_per_week`: the number of seconds in one week;
#   - `summary`: the text "A week has 604800 seconds.", where the number
#     comes from `seconds_per_week` (do not type it yourself).
#
# Hints
#   1. An hour has 60 minutes: `60 * seconds_per_minute`.
#   2. A day has 24 hours, a week has 7 days.
#   3. Inside a text, `{name}` is replaced by the value of `name`:
#      "I have {count} apples".
#
# Check your work:  rythme exercise check 01-values.ry
#              or:  rythme test 01-values.ry

seconds_per_minute = 60
seconds_per_hour = 0
seconds_per_day = 0
seconds_per_week = 0
summary = ""

# --- Tests (do not change them) ---

test "seconds in an hour" = assert_eq(seconds_per_hour, 3600)

test "seconds in a day" = assert_eq(seconds_per_day, 86400)

test "seconds in a week" = assert_eq(seconds_per_week, 604800)

test "the summary" = assert_eq(summary, "A week has 604800 seconds.")
"#,
            r#"# Ejercicio 01 · Primeros valores  ★☆☆
#
# Un nombre recibe un valor con `=` y lo conserva para siempre: en Rythme,
# los valores nunca cambian.
#
# Enunciado
#   `segundos_por_minuto` ya está dado. Reemplaza cada `0` (y el texto
#   vacío) de abajo por una expresión que use los nombres definidos antes:
#   - `segundos_por_hora`: el número de segundos de una hora;
#   - `segundos_por_dia`: el número de segundos de un día;
#   - `segundos_por_semana`: el número de segundos de una semana;
#   - `resumen`: el texto "Una semana tiene 604800 segundos.", donde el
#     número viene de `segundos_por_semana` (no lo escribas tú).
#
# Pistas
#   1. Una hora tiene 60 minutos: `60 * segundos_por_minuto`.
#   2. Un día tiene 24 horas, una semana tiene 7 días.
#   3. Dentro de un texto, `{nombre}` se reemplaza por el valor de `nombre`:
#      "Tengo {cantidad} manzanas".
#
# Comprueba tu trabajo:  rythme exercise check 01-valores.ry
#                    o:  rythme test 01-valores.ry

segundos_por_minuto = 60
segundos_por_hora = 0
segundos_por_dia = 0
segundos_por_semana = 0
resumen = ""

# --- Pruebas (no las cambies) ---

test "segundos de una hora" = assert_eq(segundos_por_hora, 3600)

test "segundos de un día" = assert_eq(segundos_por_dia, 86400)

test "segundos de una semana" = assert_eq(segundos_por_semana, 604800)

test "el resumen" = assert_eq(resumen, "Una semana tiene 604800 segundos.")
"#,
        ),
        ("02-texts", "02-textos"),
        (
            r#"# Exercise 02 · Texts  ★☆☆
#
# A function takes values and gives back a new value:
#   fn double(x) = x * 2
#
# Statement
#   Write the four functions below so that their tests pass:
#   - `greet(name)` gives "Hello, Ada!" for "Ada";
#   - `shout(text)` gives the text in uppercase followed by "!";
#   - `initials(first, last)` gives "A.L." for "Ada" and "Lovelace";
#   - `introduce(name, age)` gives "Ada is 36 years old.".
#
# Hints
#   1. "Hello, {name}!" inserts the value of `name` into the text.
#   2. `upper("hi")` is "HI", and `+` joins two texts: "HI" + "!".
#   3. `word[0]` is the first character of `word`; it can go inside braces
#      too: "{word[0]}.".
#
# Check your work:  rythme exercise check 02-texts.ry
#              or:  rythme test 02-texts.ry

## A greeting for `name`.
fn greet(name) = fail("to do")

## The text in uppercase, followed by "!".
fn shout(text) = fail("to do")

## The initials of a person, each followed by a dot.
fn initials(first, last) = fail("to do")

## A sentence that introduces a person.
fn introduce(name, age) = fail("to do")

# --- Tests (do not change them) ---

test "greet" = assert_eq(greet("Ada"), "Hello, Ada!")

test "greet someone else" = assert_eq(greet("Alan"), "Hello, Alan!")

test "shout"
  assert_eq(shout("hello"), "HELLO!")
  assert_eq(shout("Stop"), "STOP!")
end

test "initials"
  assert_eq(initials("Ada", "Lovelace"), "A.L.")
  assert_eq(initials("grace", "hopper"), "g.h.")
end

test "introduce" = assert_eq(introduce("Ada", 36), "Ada is 36 years old.")
"#,
            r#"# Ejercicio 02 · Textos  ★☆☆
#
# Una función recibe valores y devuelve un valor nuevo:
#   fn doble(x) = x * 2
#
# Enunciado
#   Escribe las cuatro funciones de abajo para que pasen sus pruebas:
#   - `saludar(nombre)` da "¡Hola, Ada!" para "Ada";
#   - `gritar(texto)` da el texto en mayúsculas seguido de "!";
#   - `iniciales(nombre, apellido)` da "A.L." para "Ada" y "Lovelace";
#   - `presentar(nombre, edad)` da "Ada tiene 36 años.".
#
# Pistas
#   1. "¡Hola, {nombre}!" inserta el valor de `nombre` en el texto.
#   2. `upper("hola")` es "HOLA", y `+` une dos textos: "HOLA" + "!".
#   3. `palabra[0]` es el primer carácter de `palabra`; también puede ir
#      entre llaves: "{palabra[0]}.".
#
# Comprueba tu trabajo:  rythme exercise check 02-textos.ry
#                    o:  rythme test 02-textos.ry

## Un saludo para `nombre`.
fn saludar(nombre) = fail("por hacer")

## El texto en mayúsculas, seguido de "!".
fn gritar(texto) = fail("por hacer")

## Las iniciales de una persona, cada una seguida de un punto.
fn iniciales(nombre, apellido) = fail("por hacer")

## Una frase que presenta a una persona.
fn presentar(nombre, edad) = fail("por hacer")

# --- Pruebas (no las cambies) ---

test "saludar" = assert_eq(saludar("Ada"), "¡Hola, Ada!")

test "saludar a otra persona" = assert_eq(saludar("Alan"), "¡Hola, Alan!")

test "gritar"
  assert_eq(gritar("hola"), "HOLA!")
  assert_eq(gritar("Alto"), "ALTO!")
end

test "iniciales"
  assert_eq(iniciales("Ada", "Lovelace"), "A.L.")
  assert_eq(iniciales("grace", "hopper"), "g.h.")
end

test "presentar" = assert_eq(presentar("Ada", 36), "Ada tiene 36 años.")
"#,
        ),
        ("03-arithmetic", "03-aritmetica"),
        (
            r#"# Exercise 03 · Arithmetic  ★☆☆
#
# Numbers are whole (Int: 3) or decimal (Float: 3.5). `/` always gives a
# Float (`7 / 2` is 3.5); `//` divides and rounds down (`7 // 2` is 3) and
# `%` gives the remainder (`7 % 2` is 1).
#
# Statement
#   - `square(x)`: x times x;
#   - `average(a, b)`: the average of two numbers;
#   - `fahrenheit(celsius)`: the temperature in degrees Fahrenheit
#     (multiply by 9, divide by 5, add 32);
#   - `hypotenuse(a, b)`: the long side of a right triangle whose two other
#     sides are a and b (the square root of a² + b²);
#   - `minutes_and_seconds(total)`: for a number of seconds, a text like
#     "2 min 5 s" (for 125).
#
# Hints
#   1. `sqrt(16)` is 4.0.
#   2. `125 // 60` is 2 and `125 % 60` is 5.
#   3. `3 == 3.0` is true: an Int and a Float with the same value are equal.
#
# Check your work:  rythme exercise check 03-arithmetic.ry
#              or:  rythme test 03-arithmetic.ry

fn square(x) = fail("to do")

fn average(a, b) = fail("to do")

fn fahrenheit(celsius) = fail("to do")

fn hypotenuse(a, b) = fail("to do")

fn minutes_and_seconds(total) = fail("to do")

# --- Tests (do not change them) ---

test "square"
  assert_eq(square(3), 9)
  assert_eq(square(-4), 16)
  assert_eq(square(1.5), 2.25)
end

test "average"
  assert_eq(average(2, 4), 3)
  assert_eq(average(1, 2), 1.5)
end

test "fahrenheit"
  assert_eq(fahrenheit(0), 32)
  assert_eq(fahrenheit(100), 212)
  assert_eq(fahrenheit(-40), -40)
end

test "hypotenuse"
  assert_eq(hypotenuse(3, 4), 5)
  assert_eq(hypotenuse(5, 12), 13)
end

test "minutes and seconds"
  assert_eq(minutes_and_seconds(125), "2 min 5 s")
  assert_eq(minutes_and_seconds(59), "0 min 59 s")
  assert_eq(minutes_and_seconds(600), "10 min 0 s")
end
"#,
            r#"# Ejercicio 03 · Aritmética  ★☆☆
#
# Los números son enteros (Int: 3) o decimales (Float: 3.5). `/` siempre da
# un Float (`7 / 2` es 3.5); `//` divide y redondea hacia abajo (`7 // 2`
# es 3) y `%` da el resto (`7 % 2` es 1).
#
# Enunciado
#   - `cuadrado(x)`: x por x;
#   - `media(a, b)`: la media de dos números;
#   - `fahrenheit(celsius)`: la temperatura en grados Fahrenheit
#     (multiplica por 9, divide entre 5, suma 32);
#   - `hipotenusa(a, b)`: el lado largo de un triángulo rectángulo cuyos
#     otros dos lados son a y b (la raíz cuadrada de a² + b²);
#   - `minutos_y_segundos(total)`: para un número de segundos, un texto
#     como "2 min 5 s" (para 125).
#
# Pistas
#   1. `sqrt(16)` es 4.0.
#   2. `125 // 60` es 2 y `125 % 60` es 5.
#   3. `3 == 3.0` es verdadero: un Int y un Float con el mismo valor son
#      iguales.
#
# Comprueba tu trabajo:  rythme exercise check 03-aritmetica.ry
#                    o:  rythme test 03-aritmetica.ry

fn cuadrado(x) = fail("por hacer")

fn media(a, b) = fail("por hacer")

fn fahrenheit(celsius) = fail("por hacer")

fn hipotenusa(a, b) = fail("por hacer")

fn minutos_y_segundos(total) = fail("por hacer")

# --- Pruebas (no las cambies) ---

test "cuadrado"
  assert_eq(cuadrado(3), 9)
  assert_eq(cuadrado(-4), 16)
  assert_eq(cuadrado(1.5), 2.25)
end

test "media"
  assert_eq(media(2, 4), 3)
  assert_eq(media(1, 2), 1.5)
end

test "fahrenheit"
  assert_eq(fahrenheit(0), 32)
  assert_eq(fahrenheit(100), 212)
  assert_eq(fahrenheit(-40), -40)
end

test "hipotenusa"
  assert_eq(hipotenusa(3, 4), 5)
  assert_eq(hipotenusa(5, 12), 13)
end

test "minutos y segundos"
  assert_eq(minutos_y_segundos(125), "2 min 5 s")
  assert_eq(minutos_y_segundos(59), "0 min 59 s")
  assert_eq(minutos_y_segundos(600), "10 min 0 s")
end
"#,
        ),
        ("04-choosing", "04-elegir"),
        (
            r#"# Exercise 04 · Choosing with `if`  ★☆☆
#
# `if condition then value else other_value` is an expression: it gives a
# value, so `else` is always needed. `else if` chains several conditions.
#
# Statement
#   Without using the built-in functions `abs`, `min` and `max`:
#   - `absolute(n)`: n without its sign (`absolute(-5)` is 5);
#   - `larger(a, b)`: the larger of two numbers;
#   - `grade(score)`: for a score from 0 to 100, "A" from 90, "B" from 80,
#     "C" from 70, "D" from 60, and "F" below 60;
#   - `ticket_price(age)`: 0 under 4 years, 5 under 12, 12 from 65, and 10
#     otherwise.
#
# Hints
#   1. `if n < 0 then -n else n`
#   2. Test the highest grade first: `if score >= 90 then "A" else if ...`.
#   3. A long `if` can continue on the next lines, each starting with
#      `else if`.
#
# Check your work:  rythme exercise check 04-choosing.ry
#              or:  rythme test 04-choosing.ry

fn absolute(n) = fail("to do")

fn larger(a, b) = fail("to do")

fn grade(score) = fail("to do")

fn ticket_price(age) = fail("to do")

# --- Tests (do not change them) ---

test "absolute"
  assert_eq(absolute(-5), 5)
  assert_eq(absolute(7), 7)
  assert_eq(absolute(0), 0)
end

test "larger"
  assert_eq(larger(3, 8), 8)
  assert_eq(larger(8, 3), 8)
  assert_eq(larger(-1, -2), -1)
end

test "grade"
  assert_eq(grade(95), "A")
  assert_eq(grade(90), "A")
  assert_eq(grade(85), "B")
  assert_eq(grade(70), "C")
  assert_eq(grade(61), "D")
  assert_eq(grade(12), "F")
end

test "ticket price"
  assert_eq(ticket_price(2), 0)
  assert_eq(ticket_price(4), 5)
  assert_eq(ticket_price(11), 5)
  assert_eq(ticket_price(30), 10)
  assert_eq(ticket_price(65), 12)
end
"#,
            r#"# Ejercicio 04 · Elegir con `if`  ★☆☆
#
# `if condición then valor else otro_valor` es una expresión: da un valor,
# así que `else` siempre hace falta. `else if` encadena varias condiciones.
#
# Enunciado
#   Sin usar las funciones predefinidas `abs`, `min` y `max`:
#   - `absoluto(n)`: n sin su signo (`absoluto(-5)` es 5);
#   - `mayor(a, b)`: el mayor de dos números;
#   - `calificacion(puntos)`: para una puntuación de 0 a 100, "A" desde 90,
#     "B" desde 80, "C" desde 70, "D" desde 60, y "F" por debajo de 60;
#   - `precio_entrada(edad)`: 0 por debajo de 4 años, 5 por debajo de 12,
#     12 desde 65, y 10 en los demás casos.
#
# Pistas
#   1. `if n < 0 then -n else n`
#   2. Prueba primero la calificación más alta:
#      `if puntos >= 90 then "A" else if ...`.
#   3. Un `if` largo puede seguir en las líneas siguientes, cada una
#      empezando con `else if`.
#
# Comprueba tu trabajo:  rythme exercise check 04-elegir.ry
#                    o:  rythme test 04-elegir.ry

fn absoluto(n) = fail("por hacer")

fn mayor(a, b) = fail("por hacer")

fn calificacion(puntos) = fail("por hacer")

fn precio_entrada(edad) = fail("por hacer")

# --- Pruebas (no las cambies) ---

test "absoluto"
  assert_eq(absoluto(-5), 5)
  assert_eq(absoluto(7), 7)
  assert_eq(absoluto(0), 0)
end

test "mayor"
  assert_eq(mayor(3, 8), 8)
  assert_eq(mayor(8, 3), 8)
  assert_eq(mayor(-1, -2), -1)
end

test "calificación"
  assert_eq(calificacion(95), "A")
  assert_eq(calificacion(90), "A")
  assert_eq(calificacion(85), "B")
  assert_eq(calificacion(70), "C")
  assert_eq(calificacion(61), "D")
  assert_eq(calificacion(12), "F")
end

test "precio de la entrada"
  assert_eq(precio_entrada(2), 0)
  assert_eq(precio_entrada(4), 5)
  assert_eq(precio_entrada(11), 5)
  assert_eq(precio_entrada(30), 10)
  assert_eq(precio_entrada(65), 12)
end
"#,
        ),
        ("05-logic", "05-logica"),
        (
            r#"# Exercise 05 · True or false  ★☆☆
#
# Comparisons (`==`, `!=`, `<`, `<=`, `>`, `>=`) give a Bool: `true` or
# `false`. `and`, `or` and `not` combine Bools.
#
# Statement
#   Each function gives a Bool, and none of them needs `if`:
#   - `is_multiple(n, d)`: is n a multiple of d?
#   - `is_between(x, low, high)`: is x from low to high (both included)?
#   - `is_leap(year)`: a year is a leap year when it is a multiple of
#     4, except the multiples of 100 that are not multiples of 400 (2000 is
#     a leap year, 1900 is not); the library has it too, as
#     `is_leap_year`, but here you write the rule yourself;
#   - `exclusive_or(a, b)`: is exactly one of a and b true?
#
# Hints
#   1. n is a multiple of d when the remainder `n % d` is 0.
#   2. `x >= low and x <= high`
#   3. Write the rule for leap years with the first function:
#      `is_multiple(year, 4) and (...)`.
#   4. Two Bools can be compared with `!=` too.
#
# Check your work:  rythme exercise check 05-logic.ry
#              or:  rythme test 05-logic.ry

fn is_multiple(n, d) = fail("to do")

fn is_between(x, low, high) = fail("to do")

fn is_leap(year) = fail("to do")

fn exclusive_or(a, b) = fail("to do")

# --- Tests (do not change them) ---

test "multiples"
  assert(is_multiple(12, 3), "12 is a multiple of 3")
  assert(not is_multiple(12, 5), "12 is not a multiple of 5")
  assert(is_multiple(0, 7), "0 is a multiple of every number")
end

test "between"
  assert(is_between(5, 1, 10), "5 is between 1 and 10")
  assert(is_between(1, 1, 10), "the bounds are included")
  assert(is_between(10, 1, 10), "the bounds are included")
  assert(not is_between(11, 1, 10), "11 is too big")
  assert(not is_between(0, 1, 10), "0 is too small")
end

test "leap years"
  assert_eq(is_leap(2024), true)
  assert_eq(is_leap(2023), false)
  assert_eq(is_leap(1900), false)
  assert_eq(is_leap(2000), true)
end

test "exclusive or"
  assert_eq(exclusive_or(true, false), true)
  assert_eq(exclusive_or(false, true), true)
  assert_eq(exclusive_or(true, true), false)
  assert_eq(exclusive_or(false, false), false)
end
"#,
            r#"# Ejercicio 05 · Verdadero o falso  ★☆☆
#
# Las comparaciones (`==`, `!=`, `<`, `<=`, `>`, `>=`) dan un Bool: `true`
# o `false`. `and`, `or` y `not` combinan Bools.
#
# Enunciado
#   Cada función da un Bool, y ninguna necesita `if`:
#   - `es_multiplo(n, d)`: ¿es n un múltiplo de d?
#   - `esta_entre(x, bajo, alto)`: ¿está x entre bajo y alto (ambos
#     incluidos)?
#   - `es_bisiesto(anio)`: un año es bisiesto cuando es múltiplo de 4,
#     salvo los múltiplos de 100 que no son múltiplos de 400 (2000 es
#     bisiesto, 1900 no); la biblioteca también lo tiene, como
#     `is_leap_year`, pero aquí escribes la regla tú mismo;
#   - `o_exclusivo(a, b)`: ¿es verdadero exactamente uno de a y b?
#
# Pistas
#   1. n es múltiplo de d cuando el resto `n % d` es 0.
#   2. `x >= bajo and x <= alto`
#   3. Escribe la regla de los años bisiestos con la primera función:
#      `es_multiplo(anio, 4) and (...)`.
#   4. Dos Bools también se pueden comparar con `!=`.
#
# Comprueba tu trabajo:  rythme exercise check 05-logica.ry
#                    o:  rythme test 05-logica.ry

fn es_multiplo(n, d) = fail("por hacer")

fn esta_entre(x, bajo, alto) = fail("por hacer")

fn es_bisiesto(anio) = fail("por hacer")

fn o_exclusivo(a, b) = fail("por hacer")

# --- Pruebas (no las cambies) ---

test "múltiplos"
  assert(es_multiplo(12, 3), "12 es múltiplo de 3")
  assert(not es_multiplo(12, 5), "12 no es múltiplo de 5")
  assert(es_multiplo(0, 7), "0 es múltiplo de todos los números")
end

test "entre"
  assert(esta_entre(5, 1, 10), "5 está entre 1 y 10")
  assert(esta_entre(1, 1, 10), "los límites están incluidos")
  assert(esta_entre(10, 1, 10), "los límites están incluidos")
  assert(not esta_entre(11, 1, 10), "11 es demasiado grande")
  assert(not esta_entre(0, 1, 10), "0 es demasiado pequeño")
end

test "años bisiestos"
  assert_eq(es_bisiesto(2024), true)
  assert_eq(es_bisiesto(2023), false)
  assert_eq(es_bisiesto(1900), false)
  assert_eq(es_bisiesto(2000), true)
end

test "o exclusivo"
  assert_eq(o_exclusivo(true, false), true)
  assert_eq(o_exclusivo(false, true), true)
  assert_eq(o_exclusivo(true, true), false)
  assert_eq(o_exclusivo(false, false), false)
end
"#,
        ),
        ("06-cases", "06-casos"),
        (
            r#"# Exercise 06 · One definition per case  ★☆☆
#
# A function can be written as several definitions, tried in order. A
# parameter can be a value (`fn f(0) = ...`), and a guard (`if ...` before
# the `=`) adds a condition:
#   fn describe(0) = "zero"
#   fn describe(n) if n < 0 = "negative"
#   fn describe(_) = "positive"
#
# Statement
#   - `plural(n, word)`: "1 cat" for 1, but "0 cats", "2 cats"...
#   - `fizzbuzz(n)`: "FizzBuzz" if n is a multiple of 3 and of 5, "Fizz"
#     for the other multiples of 3, "Buzz" for the other multiples of 5,
#     and the number as a text otherwise;
#   - `day_kind(day)`: "weekend" for "Saturday" and "Sunday", "weekday"
#     for any other text.
#
# Hints
#   1. Start with the special case: `fn plural(1, word) = ...`, then the
#      general case on the next line.
#   2. The order matters: test "FizzBuzz" before "Fizz" and "Buzz".
#   3. `text(15)` is "15".
#   4. `_` accepts anything, and is usually the last case.
#
# Check your work:  rythme exercise check 06-cases.ry
#              or:  rythme test 06-cases.ry

fn plural(n, word) = fail("to do")

fn fizzbuzz(n) = fail("to do")

fn day_kind(day) = fail("to do")

# --- Tests (do not change them) ---

test "plural"
  assert_eq(plural(1, "cat"), "1 cat")
  assert_eq(plural(2, "cat"), "2 cats")
  assert_eq(plural(0, "dog"), "0 dogs")
end

test "fizzbuzz"
  assert_eq(fizzbuzz(1), "1")
  assert_eq(fizzbuzz(3), "Fizz")
  assert_eq(fizzbuzz(10), "Buzz")
  assert_eq(fizzbuzz(30), "FizzBuzz")
  assert_eq(fizzbuzz(31), "31")
end

test "the first fifteen" = assert_eq(1..15 |> map(fizzbuzz) |> join(" "), "1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz")

test "day kind"
  assert_eq(day_kind("Saturday"), "weekend")
  assert_eq(day_kind("Sunday"), "weekend")
  assert_eq(day_kind("Monday"), "weekday")
end
"#,
            r#"# Ejercicio 06 · Una definición por caso  ★☆☆
#
# Una función puede escribirse en varias definiciones, que se prueban en
# orden. Un parámetro puede ser un valor (`fn f(0) = ...`), y una guarda
# (`if ...` antes del `=`) añade una condición:
#   fn describir(0) = "cero"
#   fn describir(n) if n < 0 = "negativo"
#   fn describir(_) = "positivo"
#
# Enunciado
#   - `plural(n, palabra)`: "1 gato" para 1, pero "0 gatos", "2 gatos"...
#   - `fizzbuzz(n)`: "FizzBuzz" si n es múltiplo de 3 y de 5, "Fizz" para
#     los demás múltiplos de 3, "Buzz" para los demás múltiplos de 5, y el
#     número como texto en los demás casos;
#   - `tipo_de_dia(dia)`: "fin de semana" para "sábado" y "domingo",
#     "entre semana" para cualquier otro texto.
#
# Pistas
#   1. Empieza por el caso particular: `fn plural(1, palabra) = ...`, y
#      luego el caso general en la línea siguiente.
#   2. El orden importa: prueba "FizzBuzz" antes que "Fizz" y "Buzz".
#   3. `text(15)` es "15".
#   4. `_` acepta cualquier cosa, y suele ser el último caso.
#
# Comprueba tu trabajo:  rythme exercise check 06-casos.ry
#                    o:  rythme test 06-casos.ry

fn plural(n, palabra) = fail("por hacer")

fn fizzbuzz(n) = fail("por hacer")

fn tipo_de_dia(dia) = fail("por hacer")

# --- Pruebas (no las cambies) ---

test "plural"
  assert_eq(plural(1, "gato"), "1 gato")
  assert_eq(plural(2, "gato"), "2 gatos")
  assert_eq(plural(0, "perro"), "0 perros")
end

test "fizzbuzz"
  assert_eq(fizzbuzz(1), "1")
  assert_eq(fizzbuzz(3), "Fizz")
  assert_eq(fizzbuzz(10), "Buzz")
  assert_eq(fizzbuzz(30), "FizzBuzz")
  assert_eq(fizzbuzz(31), "31")
end

test "los quince primeros" = assert_eq(1..15 |> map(fizzbuzz) |> join(" "), "1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz")

test "tipo de día"
  assert_eq(tipo_de_dia("sábado"), "fin de semana")
  assert_eq(tipo_de_dia("domingo"), "fin de semana")
  assert_eq(tipo_de_dia("lunes"), "entre semana")
end
"#,
        ),
        ("07-recursion", "07-recursion"),
        (
            r#"# Exercise 07 · Recursion  ★★☆
#
# Rythme has no loops: to repeat, a function calls itself. Every recursive
# function needs a base case, which stops, and a recursive case, which
# brings the problem closer to the base case:
#   fn countdown(0) = "liftoff!"
#   fn countdown(n) = "{n} " + countdown(n - 1)
#
# Statement
#   - `factorial(n)`: 1 × 2 × ... × n, and `factorial(0)` is 1;
#   - `sum_to(n)`: 1 + 2 + ... + n, and `sum_to(0)` is 0;
#   - `power(x, n)`: x to the power n, for n ≥ 0, without `**` or `pow`.
#
# Hints
#   1. `factorial(5)` is `5 * factorial(4)`.
#   2. What is `power(x, 0)`? And `power(x, n)` compared with
#      `power(x, n - 1)`?
#   3. A function that never reaches its base case stops with error E014:
#      check that n gets smaller at each call.
#
# Check your work:  rythme exercise check 07-recursion.ry
#              or:  rythme test 07-recursion.ry

fn factorial(n) = fail("to do")

fn sum_to(n) = fail("to do")

fn power(x, n) = fail("to do")

# --- Tests (do not change them) ---

test "factorial"
  assert_eq(factorial(0), 1)
  assert_eq(factorial(1), 1)
  assert_eq(factorial(5), 120)
  assert_eq(factorial(10), 3628800)
end

test "sum to"
  assert_eq(sum_to(0), 0)
  assert_eq(sum_to(1), 1)
  assert_eq(sum_to(10), 55)
  assert_eq(sum_to(100), 5050)
end

test "power"
  assert_eq(power(2, 0), 1)
  assert_eq(power(2, 10), 1024)
  assert_eq(power(3, 4), 81)
  assert_eq(power(1.5, 2), 2.25)
end
"#,
            r#"# Ejercicio 07 · Recursión  ★★☆
#
# Rythme no tiene bucles: para repetir, una función se llama a sí misma.
# Toda función recursiva necesita un caso base, que se detiene, y un caso
# recursivo, que acerca el problema al caso base:
#   fn cuenta_atras(0) = "¡despegue!"
#   fn cuenta_atras(n) = "{n} " + cuenta_atras(n - 1)
#
# Enunciado
#   - `factorial(n)`: 1 × 2 × ... × n, y `factorial(0)` es 1;
#   - `suma_hasta(n)`: 1 + 2 + ... + n, y `suma_hasta(0)` es 0;
#   - `potencia(x, n)`: x elevado a n, para n ≥ 0, sin `**` ni `pow`.
#
# Pistas
#   1. `factorial(5)` es `5 * factorial(4)`.
#   2. ¿Cuánto es `potencia(x, 0)`? ¿Y `potencia(x, n)` comparado con
#      `potencia(x, n - 1)`?
#   3. Una función que nunca llega a su caso base se detiene con el error
#      E014: comprueba que n se hace más pequeño en cada llamada.
#
# Comprueba tu trabajo:  rythme exercise check 07-recursion.ry
#                    o:  rythme test 07-recursion.ry

fn factorial(n) = fail("por hacer")

fn suma_hasta(n) = fail("por hacer")

fn potencia(x, n) = fail("por hacer")

# --- Pruebas (no las cambies) ---

test "factorial"
  assert_eq(factorial(0), 1)
  assert_eq(factorial(1), 1)
  assert_eq(factorial(5), 120)
  assert_eq(factorial(10), 3628800)
end

test "suma hasta"
  assert_eq(suma_hasta(0), 0)
  assert_eq(suma_hasta(1), 1)
  assert_eq(suma_hasta(10), 55)
  assert_eq(suma_hasta(100), 5050)
end

test "potencia"
  assert_eq(potencia(2, 0), 1)
  assert_eq(potencia(2, 10), 1024)
  assert_eq(potencia(3, 4), 81)
  assert_eq(potencia(1.5, 2), 2.25)
end
"#,
        ),
        ("08-digits", "08-cifras"),
        (
            r#"# Exercise 08 · The digits of a number  ★★☆
#
# `n % 10` is the last digit of n, and `n // 10` is n without its last
# digit: for 472, they are 2 and 47.
#
# Statement
#   For whole numbers n ≥ 0, and with recursion (without turning the number
#   into a text):
#   - `digit_count(n)`: how many digits n has (0 has one digit);
#   - `digit_sum(n)`: the sum of the digits of n;
#   - `reverse_number(n)`: the digits in the opposite order (1230 gives 321).
#
# Hints
#   1. A number below 10 has a single digit: that is the base case.
#   2. `digit_sum(472)` is `2 + digit_sum(47)`.
#   3. For `reverse_number`, write a helper with one more parameter, the
#      number built so far: `reverse_from(n, built)`, with
#      `reverse_from(0, built) = built`. Each step moves the last digit of n
#      to the end of `built`: `built * 10 + n % 10`.
#
# Check your work:  rythme exercise check 08-digits.ry
#              or:  rythme test 08-digits.ry

fn digit_count(n) = fail("to do")

fn digit_sum(n) = fail("to do")

fn reverse_number(n) = fail("to do")

# --- Tests (do not change them) ---

test "digit count"
  assert_eq(digit_count(0), 1)
  assert_eq(digit_count(7), 1)
  assert_eq(digit_count(472), 3)
  assert_eq(digit_count(1000000), 7)
end

test "digit sum"
  assert_eq(digit_sum(0), 0)
  assert_eq(digit_sum(472), 13)
  assert_eq(digit_sum(99999), 45)
end

test "reverse number"
  assert_eq(reverse_number(472), 274)
  assert_eq(reverse_number(1230), 321)
  assert_eq(reverse_number(5), 5)
end
"#,
            r#"# Ejercicio 08 · Las cifras de un número  ★★☆
#
# `n % 10` es la última cifra de n, y `n // 10` es n sin su última cifra:
# para 472, son 2 y 47.
#
# Enunciado
#   Para números enteros n ≥ 0, y con recursión (sin convertir el número
#   en texto):
#   - `cantidad_de_cifras(n)`: cuántas cifras tiene n (0 tiene una cifra);
#   - `suma_de_cifras(n)`: la suma de las cifras de n;
#   - `invertir_numero(n)`: las cifras en el orden contrario (1230 da 321).
#
# Pistas
#   1. Un número menor que 10 tiene una sola cifra: ese es el caso base.
#   2. `suma_de_cifras(472)` es `2 + suma_de_cifras(47)`.
#   3. Para `invertir_numero`, escribe una función auxiliar con un
#      parámetro más, el número construido hasta ahora:
#      `invertir_desde(n, construido)`, con
#      `invertir_desde(0, construido) = construido`. Cada paso lleva la
#      última cifra de n al final de `construido`:
#      `construido * 10 + n % 10`.
#
# Comprueba tu trabajo:  rythme exercise check 08-cifras.ry
#                    o:  rythme test 08-cifras.ry

fn cantidad_de_cifras(n) = fail("por hacer")

fn suma_de_cifras(n) = fail("por hacer")

fn invertir_numero(n) = fail("por hacer")

# --- Pruebas (no las cambies) ---

test "cantidad de cifras"
  assert_eq(cantidad_de_cifras(0), 1)
  assert_eq(cantidad_de_cifras(7), 1)
  assert_eq(cantidad_de_cifras(472), 3)
  assert_eq(cantidad_de_cifras(1000000), 7)
end

test "suma de cifras"
  assert_eq(suma_de_cifras(0), 0)
  assert_eq(suma_de_cifras(472), 13)
  assert_eq(suma_de_cifras(99999), 45)
end

test "invertir un número"
  assert_eq(invertir_numero(472), 274)
  assert_eq(invertir_numero(1230), 321)
  assert_eq(invertir_numero(5), 5)
end
"#,
        ),
        ("09-accumulators", "09-acumuladores"),
        (
            r#"# Exercise 09 · Accumulators and tail calls  ★★☆
#
# In `n + sum_to(n - 1)`, the addition waits for the call to finish: a
# million nested calls stop with error E014 ("too many nested calls"). When
# the recursive call is the very last thing the function does (a tail
# call), nothing waits, and there is no limit. An extra parameter, the
# accumulator, carries the result built so far:
#   fn sum_from(0, total) = total
#   fn sum_from(n, total) = sum_from(n - 1, total + n)
#
# Statement
#   - `big_sum(n)`: 1 + 2 + ... + n, which must work for n = 1000000;
#   - `collatz_steps(n)`: for n ≥ 1, the number of steps to reach 1, where
#     a step divides an even number by 2 and turns an odd number into
#     3 × n + 1 (6 → 3 → 10 → 5 → 16 → 8 → 4 → 2 → 1 takes 8 steps);
#   - `fib(n)`: the n-th Fibonacci number (0, 1, 1, 2, 3, 5, 8...: each
#     one is the sum of the two before), fast even for n = 90.
#
# Hints
#   1. `big_sum(n)` can simply call a helper: `sum_from(n, 0)`.
#   2. `collatz_from(n, steps)`: when n is 1, the answer is `steps`.
#   3. `fib(n) = fib(n - 1) + fib(n - 2)` is correct but very slow, because
#      it computes the same numbers again and again. Carry the two last
#      numbers instead: `fib_from(n, a, b)`, where a and b are two
#      neighbouring Fibonacci numbers, starting with `fib_from(n, 0, 1)`.
#
# Check your work:  rythme exercise check 09-accumulators.ry
#              or:  rythme test 09-accumulators.ry

fn big_sum(n) = fail("to do")

fn collatz_steps(n) = fail("to do")

fn fib(n) = fail("to do")

# --- Tests (do not change them) ---

test "big sum"
  assert_eq(big_sum(10), 55)
  assert_eq(big_sum(1000000), 500000500000)
end

test "collatz steps"
  assert_eq(collatz_steps(1), 0)
  assert_eq(collatz_steps(6), 8)
  assert_eq(collatz_steps(27), 111)
end

test "fibonacci"
  assert_eq(0..10 |> map(fib), [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55])
  assert_eq(fib(90), 2880067194370816120)
end
"#,
            r#"# Ejercicio 09 · Acumuladores y llamadas de cola  ★★☆
#
# En `n + suma_hasta(n - 1)`, la suma espera a que termine la llamada: un
# millón de llamadas anidadas se detiene con el error E014 ("demasiadas
# llamadas anidadas"). Cuando la llamada recursiva es lo último que hace la
# función (una llamada de cola), nada espera, y no hay límite. Un
# parámetro extra, el acumulador, lleva el resultado construido hasta
# ahora:
#   fn suma_desde(0, total) = total
#   fn suma_desde(n, total) = suma_desde(n - 1, total + n)
#
# Enunciado
#   - `suma_grande(n)`: 1 + 2 + ... + n, que debe funcionar para
#     n = 1000000;
#   - `pasos_collatz(n)`: para n ≥ 1, el número de pasos para llegar a 1,
#     donde un paso divide un número par entre 2 y convierte un número
#     impar en 3 × n + 1 (6 → 3 → 10 → 5 → 16 → 8 → 4 → 2 → 1 lleva 8
#     pasos);
#   - `fib(n)`: el n-ésimo número de Fibonacci (0, 1, 1, 2, 3, 5, 8...: cada
#     uno es la suma de los dos anteriores), rápido incluso para n = 90.
#
# Pistas
#   1. `suma_grande(n)` puede simplemente llamar a una función auxiliar:
#      `suma_desde(n, 0)`.
#   2. `collatz_desde(n, pasos)`: cuando n es 1, la respuesta es `pasos`.
#   3. `fib(n) = fib(n - 1) + fib(n - 2)` es correcto pero muy lento,
#      porque calcula los mismos números una y otra vez. Lleva en cambio
#      los dos últimos números: `fib_desde(n, a, b)`, donde a y b son dos
#      números de Fibonacci vecinos, empezando por `fib_desde(n, 0, 1)`.
#
# Comprueba tu trabajo:  rythme exercise check 09-acumuladores.ry
#                    o:  rythme test 09-acumuladores.ry

fn suma_grande(n) = fail("por hacer")

fn pasos_collatz(n) = fail("por hacer")

fn fib(n) = fail("por hacer")

# --- Pruebas (no las cambies) ---

test "suma grande"
  assert_eq(suma_grande(10), 55)
  assert_eq(suma_grande(1000000), 500000500000)
end

test "pasos de collatz"
  assert_eq(pasos_collatz(1), 0)
  assert_eq(pasos_collatz(6), 8)
  assert_eq(pasos_collatz(27), 111)
end

test "fibonacci"
  assert_eq(0..10 |> map(fib), [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55])
  assert_eq(fib(90), 2880067194370816120)
end
"#,
        ),
        ("10-divisors", "10-divisores"),
        (
            r#"# Exercise 10 · Divisors and prime numbers  ★★☆
#
# Recursion can also try every candidate, one after the other, until an
# answer is found.
#
# Statement
#   - `my_gcd(a, b)`: the greatest common divisor of two whole numbers
#     ≥ 0, with Euclid's method: the gcd of a and 0 is a, and the gcd of a
#     and b is the gcd of b and `a % b` (without the built-in `gcd`);
#   - `is_prime(n)`: is n a prime number (a number ≥ 2 whose only divisors
#     are 1 and itself)?
#   - `next_prime(n)`: the smallest prime number strictly greater than n.
#
# Hints
#   1. Euclid's method is already written as two cases: translate it.
#   2. For `is_prime`, write a helper `no_divisor_from(n, d)` that tries d,
#      then d + 1... It can stop as soon as `d * d > n`.
#   3. `next_prime(n)` tries n + 1: if it is prime, that is the answer;
#      otherwise, the answer is `next_prime(n + 1)`.
#
# Check your work:  rythme exercise check 10-divisors.ry
#              or:  rythme test 10-divisors.ry

fn my_gcd(a, b) = fail("to do")

fn is_prime(n) = fail("to do")

fn next_prime(n) = fail("to do")

# --- Tests (do not change them) ---

test "gcd"
  assert_eq(my_gcd(12, 18), 6)
  assert_eq(my_gcd(17, 5), 1)
  assert_eq(my_gcd(7, 0), 7)
  assert_eq(my_gcd(0, 9), 9)
end

test "prime numbers"
  assert_eq(1..30 |> filter(is_prime), [2, 3, 5, 7, 11, 13, 17, 19, 23, 29])
  assert_eq(is_prime(0), false)
  assert_eq(is_prime(7919), true)
  assert_eq(is_prime(7917), false)
end

test "next prime"
  assert_eq(next_prime(1), 2)
  assert_eq(next_prime(13), 17)
  assert_eq(next_prime(100), 101)
end
"#,
            r#"# Ejercicio 10 · Divisores y números primos  ★★☆
#
# La recursión también puede probar cada candidato, uno tras otro, hasta
# encontrar una respuesta.
#
# Enunciado
#   - `mi_mcd(a, b)`: el máximo común divisor de dos números enteros ≥ 0,
#     con el método de Euclides: el mcd de a y 0 es a, y el mcd de a y b
#     es el mcd de b y `a % b` (sin la función predefinida `gcd`);
#   - `es_primo(n)`: ¿es n un número primo (un número ≥ 2 cuyos únicos
#     divisores son 1 y él mismo)?
#   - `siguiente_primo(n)`: el menor número primo estrictamente mayor que
#     n.
#
# Pistas
#   1. El método de Euclides ya está escrito como dos casos: tradúcelo.
#   2. Para `es_primo`, escribe una función auxiliar
#      `sin_divisor_desde(n, d)` que prueba d, luego d + 1... Puede parar
#      en cuanto `d * d > n`.
#   3. `siguiente_primo(n)` prueba n + 1: si es primo, esa es la
#      respuesta; si no, la respuesta es `siguiente_primo(n + 1)`.
#
# Comprueba tu trabajo:  rythme exercise check 10-divisores.ry
#                    o:  rythme test 10-divisores.ry

fn mi_mcd(a, b) = fail("por hacer")

fn es_primo(n) = fail("por hacer")

fn siguiente_primo(n) = fail("por hacer")

# --- Pruebas (no las cambies) ---

test "mcd"
  assert_eq(mi_mcd(12, 18), 6)
  assert_eq(mi_mcd(17, 5), 1)
  assert_eq(mi_mcd(7, 0), 7)
  assert_eq(mi_mcd(0, 9), 9)
end

test "números primos"
  assert_eq(1..30 |> filter(es_primo), [2, 3, 5, 7, 11, 13, 17, 19, 23, 29])
  assert_eq(es_primo(0), false)
  assert_eq(es_primo(7919), true)
  assert_eq(es_primo(7917), false)
end

test "siguiente primo"
  assert_eq(siguiente_primo(1), 2)
  assert_eq(siguiente_primo(13), 17)
  assert_eq(siguiente_primo(100), 101)
end
"#,
        ),
        ("11-list-patterns", "11-patrones-de-listas"),
        (
            r#"# Exercise 11 · Taking lists apart  ★★☆
#
# A list is either empty, `[]`, or a first element followed by the rest of
# the list, `[first, ..rest]`. A function on lists usually has one
# definition for each of these two cases:
#   fn count_items([]) = 0
#   fn count_items([_, ..rest]) = 1 + count_items(rest)
#
# Statement
#   With recursion, and without the built-in functions `len`, `sum`, `max`
#   and `contains`:
#   - `total(list)`: the sum of the numbers of the list;
#   - `largest(list)`: the largest element of a list that is not empty;
#   - `has_value(list, x)`: is x an element of the list?
#   - `last_one(list)`: the last element of a list that is not empty.
#
# Hints
#   1. The total of an empty list is 0.
#   2. A list of one element, `[x]`, is a good base case for `largest` and
#      `last_one`.
#   3. The largest of `[x, ..rest]` is the larger of x and of the largest of
#      rest.
#   4. `has_value`: an empty list contains nothing; otherwise, either the
#      first element is x, or x is in the rest (`or`).
#   5. `rythme check` warns (W002) about a forgotten case, here the empty
#      list. It has no largest element: say so with
#      `fn largest([]) = fail("the list is empty")`.
#
# Check your work:  rythme exercise check 11-list-patterns.ry
#              or:  rythme test 11-list-patterns.ry

fn total(list) = fail("to do")

fn largest(list) = fail("to do")

fn has_value(list, x) = fail("to do")

fn last_one(list) = fail("to do")

# --- Tests (do not change them) ---

test "total"
  assert_eq(total([]), 0)
  assert_eq(total([4]), 4)
  assert_eq(total([1, 2, 3, 4]), 10)
end

test "largest"
  assert_eq(largest([7]), 7)
  assert_eq(largest([3, 9, 2]), 9)
  assert_eq(largest([-3, -1, -2]), -1)
end

test "has value"
  assert_eq(has_value([1, 2, 3], 2), true)
  assert_eq(has_value([1, 2, 3], 5), false)
  assert_eq(has_value([], 1), false)
  assert_eq(has_value(["a", "b"], "b"), true)
end

test "last one"
  assert_eq(last_one([1, 2, 3]), 3)
  assert_eq(last_one(["only"]), "only")
end
"#,
            r#"# Ejercicio 11 · Desarmar listas  ★★☆
#
# Una lista es o bien vacía, `[]`, o bien un primer elemento seguido del
# resto de la lista, `[primero, ..resto]`. Una función sobre listas suele
# tener una definición para cada uno de estos dos casos:
#   fn contar_elementos([]) = 0
#   fn contar_elementos([_, ..resto]) = 1 + contar_elementos(resto)
#
# Enunciado
#   Con recursión, y sin las funciones predefinidas `len`, `sum`, `max` y
#   `contains`:
#   - `total(lista)`: la suma de los números de la lista;
#   - `el_mayor(lista)`: el mayor elemento de una lista no vacía;
#   - `contiene_valor(lista, x)`: ¿es x un elemento de la lista?
#   - `el_ultimo(lista)`: el último elemento de una lista no vacía.
#
# Pistas
#   1. El total de una lista vacía es 0.
#   2. Una lista de un elemento, `[x]`, es un buen caso base para
#      `el_mayor` y `el_ultimo`.
#   3. El mayor de `[x, ..resto]` es el más grande entre x y el mayor de
#      resto.
#   4. `contiene_valor`: una lista vacía no contiene nada; si no, o el
#      primer elemento es x, o x está en el resto (`or`).
#   5. `rythme check` avisa (W002) de un caso olvidado, aquí la lista
#      vacía. No tiene mayor elemento: dilo con
#      `fn el_mayor([]) = fail("la lista está vacía")`.
#
# Comprueba tu trabajo:  rythme exercise check 11-patrones-de-listas.ry
#                    o:  rythme test 11-patrones-de-listas.ry

fn total(lista) = fail("por hacer")

fn el_mayor(lista) = fail("por hacer")

fn contiene_valor(lista, x) = fail("por hacer")

fn el_ultimo(lista) = fail("por hacer")

# --- Pruebas (no las cambies) ---

test "total"
  assert_eq(total([]), 0)
  assert_eq(total([4]), 4)
  assert_eq(total([1, 2, 3, 4]), 10)
end

test "el mayor"
  assert_eq(el_mayor([7]), 7)
  assert_eq(el_mayor([3, 9, 2]), 9)
  assert_eq(el_mayor([-3, -1, -2]), -1)
end

test "contiene el valor"
  assert_eq(contiene_valor([1, 2, 3], 2), true)
  assert_eq(contiene_valor([1, 2, 3], 5), false)
  assert_eq(contiene_valor([], 1), false)
  assert_eq(contiene_valor(["a", "b"], "b"), true)
end

test "el último"
  assert_eq(el_ultimo([1, 2, 3]), 3)
  assert_eq(el_ultimo(["único"]), "único")
end
"#,
        ),
        ("12-building-lists", "12-construir-listas"),
        (
            r#"# Exercise 12 · Building lists  ★★☆
#
# `[x, ..rest]` also builds a list: x followed by the elements of rest.
# Lists never change: a function gives back a new list.
#   fn zeros(0) = []
#   fn zeros(n) = [0, ..zeros(n - 1)]
#
# Statement
#   With recursion, and without `map`, `reverse` or ranges (`1..n`):
#   - `countdown(n)`: the list [n, n - 1, ..., 1, 0];
#   - `repeat_value(x, n)`: a list with n times the value x;
#   - `double_all(list)`: each number of the list doubled;
#   - `reversed(list)`: the elements in the opposite order.
#
# Hints
#   1. `countdown(0)` is `[0]`.
#   2. `double_all([x, ..rest])` starts with `x * 2`.
#   3. For `reversed`, use an accumulator: `reversed_onto(list, done)` moves
#      the first element of list to the front of done, until list is empty.
#      `reversed_onto([1, 2], [])` → `reversed_onto([2], [1])` →
#      `reversed_onto([], [2, 1])`.
#
# Check your work:  rythme exercise check 12-building-lists.ry
#              or:  rythme test 12-building-lists.ry

fn countdown(n) = fail("to do")

fn repeat_value(x, n) = fail("to do")

fn double_all(list) = fail("to do")

fn reversed(list) = fail("to do")

# --- Tests (do not change them) ---

test "countdown"
  assert_eq(countdown(0), [0])
  assert_eq(countdown(3), [3, 2, 1, 0])
end

test "repeat value"
  assert_eq(repeat_value("a", 3), ["a", "a", "a"])
  assert_eq(repeat_value(1, 0), [])
end

test "double all"
  assert_eq(double_all([1, 2, 3]), [2, 4, 6])
  assert_eq(double_all([]), [])
end

test "reversed"
  assert_eq(reversed([1, 2, 3]), [3, 2, 1])
  assert_eq(reversed([]), [])
  assert_eq(reversed(["x"]), ["x"])
end

test "the original list does not change"
  numbers = [1, 2, 3]
  reversed(numbers)
  assert_eq(numbers, [1, 2, 3])
end
"#,
            r#"# Ejercicio 12 · Construir listas  ★★☆
#
# `[x, ..resto]` también construye una lista: x seguido de los elementos de
# resto. Las listas nunca cambian: una función devuelve una lista nueva.
#   fn ceros(0) = []
#   fn ceros(n) = [0, ..ceros(n - 1)]
#
# Enunciado
#   Con recursión, y sin `map`, `reverse` ni rangos (`1..n`):
#   - `cuenta_atras(n)`: la lista [n, n - 1, ..., 1, 0];
#   - `repetir_valor(x, n)`: una lista con n veces el valor x;
#   - `duplicar_todos(lista)`: cada número de la lista multiplicado por 2;
#   - `al_reves(lista)`: los elementos en el orden contrario.
#
# Pistas
#   1. `cuenta_atras(0)` es `[0]`.
#   2. `duplicar_todos([x, ..resto])` empieza con `x * 2`.
#   3. Para `al_reves`, usa un acumulador: `al_reves_sobre(lista, hecho)`
#      pasa el primer elemento de lista al principio de hecho, hasta que
#      lista quede vacía.
#      `al_reves_sobre([1, 2], [])` → `al_reves_sobre([2], [1])` →
#      `al_reves_sobre([], [2, 1])`.
#
# Comprueba tu trabajo:  rythme exercise check 12-construir-listas.ry
#                    o:  rythme test 12-construir-listas.ry

fn cuenta_atras(n) = fail("por hacer")

fn repetir_valor(x, n) = fail("por hacer")

fn duplicar_todos(lista) = fail("por hacer")

fn al_reves(lista) = fail("por hacer")

# --- Pruebas (no las cambies) ---

test "cuenta atrás"
  assert_eq(cuenta_atras(0), [0])
  assert_eq(cuenta_atras(3), [3, 2, 1, 0])
end

test "repetir un valor"
  assert_eq(repetir_valor("a", 3), ["a", "a", "a"])
  assert_eq(repetir_valor(1, 0), [])
end

test "duplicar todos"
  assert_eq(duplicar_todos([1, 2, 3]), [2, 4, 6])
  assert_eq(duplicar_todos([]), [])
end

test "al revés"
  assert_eq(al_reves([1, 2, 3]), [3, 2, 1])
  assert_eq(al_reves([]), [])
  assert_eq(al_reves(["x"]), ["x"])
end

test "la lista original no cambia"
  numeros = [1, 2, 3]
  al_reves(numeros)
  assert_eq(numeros, [1, 2, 3])
end
"#,
        ),
        ("13-library", "13-biblioteca"),
        (
            r#"# Exercise 13 · The list functions  ★★☆
#
# Most work on lists needs no recursion: the built-in functions do it.
# `1..5` is the list [1, 2, 3, 4, 5]; `map(list, f)` applies f to each
# element; `filter(list, f)` keeps the elements for which f gives true;
# `sum`, `len`, `sort`, `reverse`, `take`... `rythme help lists` shows them.
#
# Statement
#   - `squares(n)`: the list [1, 4, 9, ..., n × n];
#   - `evens(list)`: the even numbers of the list, in the same order;
#   - `sum_of_multiples(limit)`: the sum of the numbers from 1 to limit - 1
#     that are multiples of 3 or of 5;
#   - `mean(list)`: the average of the numbers of a list that is not empty;
#   - `smallest_three(list)`: the three smallest numbers, from the smallest.
#
# Hints
#   1. `map(1..n, square)` with a function `square` written by you, or
#      `map(1..n, fn(x) = x * x)` (an anonymous function, see exercise 14).
#   2. `is_even` is a built-in function: `filter(list, is_even)`.
#   3. `1..0` is the empty list `[]`.
#   4. `sort` then `take`.
#
# Check your work:  rythme exercise check 13-library.ry
#              or:  rythme test 13-library.ry

fn squares(n) = fail("to do")

fn evens(list) = fail("to do")

fn sum_of_multiples(limit) = fail("to do")

fn mean(list) = fail("to do")

fn smallest_three(list) = fail("to do")

# --- Tests (do not change them) ---

test "squares"
  assert_eq(squares(5), [1, 4, 9, 16, 25])
  assert_eq(squares(0), [])
end

test "evens" = assert_eq(evens([5, 2, 8, 1, 4]), [2, 8, 4])

test "sum of multiples"
  assert_eq(sum_of_multiples(10), 23)
  assert_eq(sum_of_multiples(1000), 233168)
end

test "mean"
  assert_eq(mean([1, 2, 3, 4]), 2.5)
  assert_eq(mean([7]), 7)
end

test "smallest three"
  assert_eq(smallest_three([5, 1, 4, 2, 3]), [1, 2, 3])
  assert_eq(smallest_three([9, 8]), [8, 9])
end
"#,
            r#"# Ejercicio 13 · Las funciones de listas  ★★☆
#
# La mayor parte del trabajo con listas no necesita recursión: las
# funciones predefinidas lo hacen. `1..5` es la lista [1, 2, 3, 4, 5];
# `map(lista, f)` aplica f a cada elemento; `filter(lista, f)` conserva los
# elementos para los que f da true; `sum`, `len`, `sort`, `reverse`,
# `take`... `rythme help lists` las muestra.
#
# Enunciado
#   - `cuadrados(n)`: la lista [1, 4, 9, ..., n × n];
#   - `pares(lista)`: los números pares de la lista, en el mismo orden;
#   - `suma_de_multiplos(limite)`: la suma de los números de 1 a
#     limite - 1 que son múltiplos de 3 o de 5;
#   - `promedio(lista)`: la media de los números de una lista no vacía;
#   - `los_tres_menores(lista)`: los tres números más pequeños, del más
#     pequeño al más grande.
#
# Pistas
#   1. `map(1..n, cuadrado)` con una función `cuadrado` escrita por ti, o
#      `map(1..n, fn(x) = x * x)` (una función anónima, mira el
#      ejercicio 14).
#   2. `is_even` es una función predefinida: `filter(lista, is_even)`.
#   3. `1..0` es la lista vacía `[]`.
#   4. `sort` y luego `take`.
#
# Comprueba tu trabajo:  rythme exercise check 13-biblioteca.ry
#                    o:  rythme test 13-biblioteca.ry

fn cuadrados(n) = fail("por hacer")

fn pares(lista) = fail("por hacer")

fn suma_de_multiplos(limite) = fail("por hacer")

fn promedio(lista) = fail("por hacer")

fn los_tres_menores(lista) = fail("por hacer")

# --- Pruebas (no las cambies) ---

test "cuadrados"
  assert_eq(cuadrados(5), [1, 4, 9, 16, 25])
  assert_eq(cuadrados(0), [])
end

test "pares" = assert_eq(pares([5, 2, 8, 1, 4]), [2, 8, 4])

test "suma de múltiplos"
  assert_eq(suma_de_multiplos(10), 23)
  assert_eq(suma_de_multiplos(1000), 233168)
end

test "promedio"
  assert_eq(promedio([1, 2, 3, 4]), 2.5)
  assert_eq(promedio([7]), 7)
end

test "los tres menores"
  assert_eq(los_tres_menores([5, 1, 4, 2, 3]), [1, 2, 3])
  assert_eq(los_tres_menores([9, 8]), [8, 9])
end
"#,
        ),
        ("14-anonymous-functions", "14-funciones-anonimas"),
        (
            r#"# Exercise 14 · Anonymous functions  ★★☆
#
# `fn(x) = x * 2` is a function without a name, written right where it is
# needed. It can use the names around it:
#   fn add_to_all(list, n) = map(list, fn(x) = x + n)
#
# Statement
#   - `with_tax(prices, percent)`: each price increased by `percent` %;
#   - `long_words(words, n)`: the words that have more than n characters;
#   - `lengths(words)`: the length of each word;
#   - `all_positive(list)`: are all the numbers greater than 0?
#   - `count_above(list, limit)`: how many numbers are greater than limit.
#
# Hints
#   1. A price p increased by 20 % is `p + p * 20 / 100`.
#   2. `len("word")` is 4.
#   3. `all(list, f)` is true when f gives true for every element, and
#      `count(list, f)` counts the elements for which f gives true.
#
# Check your work:  rythme exercise check 14-anonymous-functions.ry
#              or:  rythme test 14-anonymous-functions.ry

fn with_tax(prices, percent) = fail("to do")

fn long_words(words, n) = fail("to do")

fn lengths(words) = fail("to do")

fn all_positive(list) = fail("to do")

fn count_above(list, limit) = fail("to do")

# --- Tests (do not change them) ---

test "with tax"
  assert_eq(with_tax([100, 50], 20), [120, 60])
  assert_eq(with_tax([10], 0), [10])
end

test "long words" = assert_eq(long_words(["a", "tree", "is", "green"], 3), ["tree", "green"])

test "lengths" = assert_eq(lengths(["a", "tree", "is"]), [1, 4, 2])

test "all positive"
  assert_eq(all_positive([1, 5, 3]), true)
  assert_eq(all_positive([1, -5, 3]), false)
  assert_eq(all_positive([]), true)
end

test "count above" = assert_eq(count_above([3, 10, 7, 12], 5), 3)
"#,
            r#"# Ejercicio 14 · Funciones anónimas  ★★☆
#
# `fn(x) = x * 2` es una función sin nombre, escrita justo donde se
# necesita. Puede usar los nombres que la rodean:
#   fn sumar_a_todos(lista, n) = map(lista, fn(x) = x + n)
#
# Enunciado
#   - `con_impuesto(precios, porcentaje)`: cada precio aumentado en
#     `porcentaje` %;
#   - `palabras_largas(palabras, n)`: las palabras que tienen más de n
#     caracteres;
#   - `longitudes(palabras)`: la longitud de cada palabra;
#   - `todos_positivos(lista)`: ¿son todos los números mayores que 0?
#   - `cuantos_por_encima(lista, limite)`: cuántos números son mayores que
#     limite.
#
# Pistas
#   1. Un precio p aumentado en un 20 % es `p + p * 20 / 100`.
#   2. `len("casa")` es 4.
#   3. `all(lista, f)` es true cuando f da true para todos los elementos, y
#      `count(lista, f)` cuenta los elementos para los que f da true.
#
# Comprueba tu trabajo:  rythme exercise check 14-funciones-anonimas.ry
#                    o:  rythme test 14-funciones-anonimas.ry

fn con_impuesto(precios, porcentaje) = fail("por hacer")

fn palabras_largas(palabras, n) = fail("por hacer")

fn longitudes(palabras) = fail("por hacer")

fn todos_positivos(lista) = fail("por hacer")

fn cuantos_por_encima(lista, limite) = fail("por hacer")

# --- Pruebas (no las cambies) ---

test "con impuesto"
  assert_eq(con_impuesto([100, 50], 20), [120, 60])
  assert_eq(con_impuesto([10], 0), [10])
end

test "palabras largas" = assert_eq(palabras_largas(["el", "árbol", "es", "verde"], 3), ["árbol", "verde"])

test "longitudes" = assert_eq(longitudes(["a", "casa", "es"]), [1, 4, 2])

test "todos positivos"
  assert_eq(todos_positivos([1, 5, 3]), true)
  assert_eq(todos_positivos([1, -5, 3]), false)
  assert_eq(todos_positivos([]), true)
end

test "cuántos por encima" = assert_eq(cuantos_por_encima([3, 10, 7, 12], 5), 3)
"#,
        ),
        ("15-functions-as-values", "15-funciones-como-valores"),
        (
            r#"# Exercise 15 · Functions are values  ★★☆
#
# A function can be given to another function, and a function can give
# back a new function:
#   fn twice(f) = fn(x) = f(f(x))
#   add_two = twice(fn(x) = x + 1)      # add_two(5) is 7
#
# Statement
#   - `apply_twice(f, x)`: f applied to x, then f applied to the result;
#   - `compose(f, g)`: a new function that applies g, then f;
#   - `adder(n)`: a new function that adds n to its argument;
#   - `my_map(list, f)` and `my_filter(list, f)`: your own versions of
#     `map` and `filter`, with recursion.
#
# Hints
#   1. `compose(f, g)` gives `fn(x) = ...`.
#   2. `my_map([x, ..rest], f)` starts with `f(x)`.
#   3. `my_filter([x, ..rest], f)`: keep x only if `f(x)` is true, then
#      continue with rest.
#
# Check your work:  rythme exercise check 15-functions-as-values.ry
#              or:  rythme test 15-functions-as-values.ry

fn apply_twice(f, x) = fail("to do")

fn compose(f, g) = fail("to do")

fn adder(n) = fail("to do")

fn my_map(list, f) = fail("to do")

fn my_filter(list, f) = fail("to do")

# --- Tests (do not change them) ---

test "apply twice"
  assert_eq(apply_twice(fn(x) = x * 3, 2), 18)
  assert_eq(apply_twice(upper, "hi"), "HI")
end

test "compose"
  plus_one_then_double = compose(fn(x) = x * 2, fn(x) = x + 1)
  assert_eq(plus_one_then_double(4), 10)
  assert_eq(compose(len, trim)("  abc  "), 3)
end

test "adder"
  add10 = adder(10)
  assert_eq(add10(5), 15)
  assert_eq(map([1, 2, 3], adder(100)), [101, 102, 103])
end

test "my map"
  assert_eq(my_map([1, 2, 3], fn(x) = x * x), [1, 4, 9])
  assert_eq(my_map([], fn(x) = x), [])
end

test "my filter"
  assert_eq(my_filter([1, 2, 3, 4, 5, 6], is_even), [2, 4, 6])
  assert_eq(my_filter(["a", "bb", "ccc"], fn(w) = len(w) > 1), ["bb", "ccc"])
end
"#,
            r#"# Ejercicio 15 · Las funciones son valores  ★★☆
#
# Una función se puede dar a otra función, y una función puede devolver una
# función nueva:
#   fn dos_veces(f) = fn(x) = f(f(x))
#   sumar_dos = dos_veces(fn(x) = x + 1)      # sumar_dos(5) es 7
#
# Enunciado
#   - `aplicar_dos_veces(f, x)`: f aplicada a x, y luego f aplicada al
#     resultado;
#   - `componer(f, g)`: una función nueva que aplica g, y luego f;
#   - `sumador(n)`: una función nueva que suma n a su argumento;
#   - `mi_map(lista, f)` y `mi_filter(lista, f)`: tus propias versiones de
#     `map` y `filter`, con recursión.
#
# Pistas
#   1. `componer(f, g)` da `fn(x) = ...`.
#   2. `mi_map([x, ..resto], f)` empieza con `f(x)`.
#   3. `mi_filter([x, ..resto], f)`: conserva x solo si `f(x)` es true, y
#      luego sigue con resto.
#
# Comprueba tu trabajo:  rythme exercise check 15-funciones-como-valores.ry
#                    o:  rythme test 15-funciones-como-valores.ry

fn aplicar_dos_veces(f, x) = fail("por hacer")

fn componer(f, g) = fail("por hacer")

fn sumador(n) = fail("por hacer")

fn mi_map(lista, f) = fail("por hacer")

fn mi_filter(lista, f) = fail("por hacer")

# --- Pruebas (no las cambies) ---

test "aplicar dos veces"
  assert_eq(aplicar_dos_veces(fn(x) = x * 3, 2), 18)
  assert_eq(aplicar_dos_veces(upper, "hola"), "HOLA")
end

test "componer"
  mas_uno_y_doble = componer(fn(x) = x * 2, fn(x) = x + 1)
  assert_eq(mas_uno_y_doble(4), 10)
  assert_eq(componer(len, trim)("  abc  "), 3)
end

test "sumador"
  sumar10 = sumador(10)
  assert_eq(sumar10(5), 15)
  assert_eq(map([1, 2, 3], sumador(100)), [101, 102, 103])
end

test "mi map"
  assert_eq(mi_map([1, 2, 3], fn(x) = x * x), [1, 4, 9])
  assert_eq(mi_map([], fn(x) = x), [])
end

test "mi filter"
  assert_eq(mi_filter([1, 2, 3, 4, 5, 6], is_even), [2, 4, 6])
  assert_eq(mi_filter(["a", "bb", "ccc"], fn(p) = len(p) > 1), ["bb", "ccc"])
end
"#,
        ),
        ("16-fold", "16-fold"),
        (
            r#"# Exercise 16 · Folding a list into one value  ★★☆
#
# `fold(list, start, f)` combines all the elements into one value: it
# starts with `start`, then computes `f(result, element)` for each element.
#   fold([1, 2, 3], 0, fn(total, x) = total + x)     # 6
#
# Statement
#   With `fold`, and without recursion:
#   - `product_of(list)`: the product of the numbers (1 for an empty list);
#   - `longest_word(words)`: the longest word (the first one if several
#     have the same length; "" for an empty list);
#   - `to_number(digits)`: the number written with these digits
#     ([1, 2, 3] gives 123);
#   - `count_where(list, f)`: how many elements give true with f (without
#     `count` or `filter`).
#
# Hints
#   1. Choose `start` well: it is the answer for the empty list.
#   2. For `longest_word`, the result so far is the longest word seen so
#      far: keep it, or take the new word if it is strictly longer.
#   3. `to_number`: with 12 so far and the digit 3, the new result is
#      12 * 10 + 3.
#
# Check your work:  rythme exercise check 16-fold.ry
#              or:  rythme test 16-fold.ry

fn product_of(list) = fail("to do")

fn longest_word(words) = fail("to do")

fn to_number(digits) = fail("to do")

fn count_where(list, f) = fail("to do")

# --- Tests (do not change them) ---

test "product"
  assert_eq(product_of([2, 3, 4]), 24)
  assert_eq(product_of([]), 1)
end

test "longest word"
  assert_eq(longest_word(["a", "tree", "is", "green"]), "green")
  assert_eq(longest_word(["one", "two"]), "one")
  assert_eq(longest_word([]), "")
end

test "to number"
  assert_eq(to_number([1, 2, 3]), 123)
  assert_eq(to_number([0, 7]), 7)
  assert_eq(to_number([]), 0)
end

test "count where"
  assert_eq(count_where([1, 2, 3, 4, 5], is_odd), 3)
  assert_eq(count_where(["a", "bb"], fn(w) = len(w) > 5), 0)
end
"#,
            r#"# Ejercicio 16 · Plegar una lista en un solo valor  ★★☆
#
# `fold(lista, inicio, f)` combina todos los elementos en un solo valor:
# empieza con `inicio`, y luego calcula `f(resultado, elemento)` para cada
# elemento.
#   fold([1, 2, 3], 0, fn(total, x) = total + x)     # 6
#
# Enunciado
#   Con `fold`, y sin recursión:
#   - `producto_de(lista)`: el producto de los números (1 para una lista
#     vacía);
#   - `palabra_mas_larga(palabras)`: la palabra más larga (la primera si
#     varias tienen la misma longitud; "" para una lista vacía);
#   - `a_numero(cifras)`: el número escrito con esas cifras
#     ([1, 2, 3] da 123);
#   - `contar_si(lista, f)`: cuántos elementos dan true con f (sin `count`
#     ni `filter`).
#
# Pistas
#   1. Elige bien `inicio`: es la respuesta para la lista vacía.
#   2. Para `palabra_mas_larga`, el resultado hasta ahora es la palabra más
#      larga vista hasta ahora: consérvala, o toma la nueva palabra si es
#      estrictamente más larga.
#   3. `a_numero`: con 12 hasta ahora y la cifra 3, el nuevo resultado es
#      12 * 10 + 3.
#
# Comprueba tu trabajo:  rythme exercise check 16-fold.ry
#                    o:  rythme test 16-fold.ry

fn producto_de(lista) = fail("por hacer")

fn palabra_mas_larga(palabras) = fail("por hacer")

fn a_numero(cifras) = fail("por hacer")

fn contar_si(lista, f) = fail("por hacer")

# --- Pruebas (no las cambies) ---

test "producto"
  assert_eq(producto_de([2, 3, 4]), 24)
  assert_eq(producto_de([]), 1)
end

test "palabra más larga"
  assert_eq(palabra_mas_larga(["el", "árbol", "es", "verde"]), "árbol")
  assert_eq(palabra_mas_larga(["uno", "dos"]), "uno")
  assert_eq(palabra_mas_larga([]), "")
end

test "a número"
  assert_eq(a_numero([1, 2, 3]), 123)
  assert_eq(a_numero([0, 7]), 7)
  assert_eq(a_numero([]), 0)
end

test "contar si"
  assert_eq(contar_si([1, 2, 3, 4, 5], is_odd), 3)
  assert_eq(contar_si(["a", "bb"], fn(p) = len(p) > 5), 0)
end
"#,
        ),
        ("17-pipelines", "17-pipelines"),
        (
            r#"# Exercise 17 · Pipelines  ★★☆
#
# `x |> f(a)` means `f(x, a)`: the value on the left becomes the first
# argument. Steps are read from left to right, in the order they happen:
#   [3, 1, 2] |> sort |> map(fn(x) = x * 10)        # [10, 20, 30]
# A line that starts with `|>` continues the line before.
#
# Statement
#   Write each function as one pipeline:
#   - `sum_of_odd_squares(n)`: the sum of the squares of the odd numbers
#     from 1 to n;
#   - `top_three(scores)`: the three best scores, the best first;
#   - `shout_words(sentence)`: each word in uppercase, separated by one
#     space ("hello  world" gives "HELLO WORLD");
#   - `initials_of(sentence)`: the first letter of each word, in uppercase
#     ("portable network graphics" gives "PNG").
#
# Hints
#   1. `filter`, then `map`, then `sum`.
#   2. `sort`, `reverse`, `take(3)`.
#   3. `split(text)` cuts a text at the spaces (several spaces count as
#      one); `join(words, " ")` glues words back together.
#   4. `word[0]` is the first character of `word`.
#
# Check your work:  rythme exercise check 17-pipelines.ry
#              or:  rythme test 17-pipelines.ry

fn sum_of_odd_squares(n) = fail("to do")

fn top_three(scores) = fail("to do")

fn shout_words(sentence) = fail("to do")

fn initials_of(sentence) = fail("to do")

# --- Tests (do not change them) ---

test "sum of odd squares"
  assert_eq(sum_of_odd_squares(5), 35)
  assert_eq(sum_of_odd_squares(1), 1)
end

test "top three"
  assert_eq(top_three([12, 18, 9, 15, 20]), [20, 18, 15])
  assert_eq(top_three([4]), [4])
end

test "shout words" = assert_eq(shout_words("hello  world"), "HELLO WORLD")

test "initials of"
  assert_eq(initials_of("portable network graphics"), "PNG")
  assert_eq(initials_of("Rythme"), "R")
end
"#,
            r#"# Ejercicio 17 · Tuberías  ★★☆
#
# `x |> f(a)` significa `f(x, a)`: el valor de la izquierda se convierte en
# el primer argumento. Los pasos se leen de izquierda a derecha, en el
# orden en que ocurren:
#   [3, 1, 2] |> sort |> map(fn(x) = x * 10)        # [10, 20, 30]
# Una línea que empieza con `|>` continúa la línea anterior.
#
# Enunciado
#   Escribe cada función como una sola tubería:
#   - `suma_de_cuadrados_impares(n)`: la suma de los cuadrados de los
#     números impares de 1 a n;
#   - `los_tres_mejores(puntos)`: las tres mejores puntuaciones, la mejor
#     primero;
#   - `gritar_palabras(frase)`: cada palabra en mayúsculas, separadas por
#     un espacio ("hola  mundo" da "HOLA MUNDO");
#   - `iniciales_de(frase)`: la primera letra de cada palabra, en
#     mayúsculas ("portable network graphics" da "PNG").
#
# Pistas
#   1. `filter`, luego `map`, luego `sum`.
#   2. `sort`, `reverse`, `take(3)`.
#   3. `split(texto)` corta un texto en los espacios (varios espacios
#      cuentan como uno); `join(palabras, " ")` vuelve a pegar las
#      palabras.
#   4. `palabra[0]` es el primer carácter de `palabra`.
#
# Comprueba tu trabajo:  rythme exercise check 17-pipelines.ry
#                    o:  rythme test 17-pipelines.ry

fn suma_de_cuadrados_impares(n) = fail("por hacer")

fn los_tres_mejores(puntos) = fail("por hacer")

fn gritar_palabras(frase) = fail("por hacer")

fn iniciales_de(frase) = fail("por hacer")

# --- Pruebas (no las cambies) ---

test "suma de cuadrados impares"
  assert_eq(suma_de_cuadrados_impares(5), 35)
  assert_eq(suma_de_cuadrados_impares(1), 1)
end

test "los tres mejores"
  assert_eq(los_tres_mejores([12, 18, 9, 15, 20]), [20, 18, 15])
  assert_eq(los_tres_mejores([4]), [4])
end

test "gritar palabras" = assert_eq(gritar_palabras("hola  mundo"), "HOLA MUNDO")

test "iniciales de"
  assert_eq(iniciales_de("portable network graphics"), "PNG")
  assert_eq(iniciales_de("Rythme"), "R")
end
"#,
        ),
        ("18-text-processing", "18-procesar-textos"),
        (
            r#"# Exercise 18 · Working with texts  ★★☆
#
# A text can be turned into the list of its characters with `chars`, and
# the list functions do the rest. `lower`, `upper`, `replace`, `split`,
# `join`, `contains`... are described by `rythme help text`.
#
# Statement
#   - `is_palindrome(text)`: does the text read the same from both ends,
#     ignoring uppercase and spaces? ("Never odd or even" does.)
#   - `count_vowels(text)`: how many characters are vowels (a, e, i, o, u
#     or y, in lowercase or uppercase)?
#   - `title_case(text)`: each word starts with an uppercase letter, the
#     rest in lowercase ("the LORD of the rings" gives
#     "The Lord Of The Rings").
#
# Hints
#   1. `replace(text, " ", "")` removes the spaces, and `reverse` works on
#      texts too.
#   2. `contains("aeiouy", c)` tells whether the character c is a vowel.
#   3. `capitalize("lord")` is "Lord".
#
# Check your work:  rythme exercise check 18-text-processing.ry
#              or:  rythme test 18-text-processing.ry

fn is_palindrome(text) = fail("to do")

fn count_vowels(text) = fail("to do")

fn title_case(text) = fail("to do")

# --- Tests (do not change them) ---

test "palindromes"
  assert_eq(is_palindrome("kayak"), true)
  assert_eq(is_palindrome("Never odd or even"), true)
  assert_eq(is_palindrome("rythme"), false)
end

test "vowels"
  assert_eq(count_vowels("Rythme"), 2)
  assert_eq(count_vowels("AEIOU"), 5)
  assert_eq(count_vowels("xkcd"), 0)
end

test "title case" = assert_eq(title_case("the LORD of the rings"), "The Lord Of The Rings")
"#,
            r#"# Ejercicio 18 · Trabajar con textos  ★★☆
#
# Un texto se puede convertir en la lista de sus caracteres con `chars`, y
# las funciones de listas hacen el resto. `lower`, `upper`, `replace`,
# `split`, `join`, `contains`... se describen en `rythme help text`.
#
# Enunciado
#   - `es_palindromo(texto)`: ¿se lee igual el texto desde los dos
#     extremos, sin contar mayúsculas ni espacios? ("Anita lava la tina"
#     sí.)
#   - `contar_vocales(texto)`: ¿cuántos caracteres son vocales (a, e, i, o
#     o u, en minúscula o en mayúscula)?
#   - `tipo_titulo(texto)`: cada palabra empieza con mayúscula, y el resto
#     en minúsculas ("el SEÑOR de los anillos" da
#     "El Señor De Los Anillos").
#
# Pistas
#   1. `replace(texto, " ", "")` quita los espacios, y `reverse` también
#      funciona con textos.
#   2. `contains("aeiou", c)` dice si el carácter c es una vocal.
#   3. `capitalize("señor")` es "Señor".
#
# Comprueba tu trabajo:  rythme exercise check 18-procesar-textos.ry
#                    o:  rythme test 18-procesar-textos.ry

fn es_palindromo(texto) = fail("por hacer")

fn contar_vocales(texto) = fail("por hacer")

fn tipo_titulo(texto) = fail("por hacer")

# --- Pruebas (no las cambies) ---

test "palíndromos"
  assert_eq(es_palindromo("reconocer"), true)
  assert_eq(es_palindromo("Anita lava la tina"), true)
  assert_eq(es_palindromo("rythme"), false)
end

test "vocales"
  assert_eq(contar_vocales("Murcielago"), 5)
  assert_eq(contar_vocales("AEIOU"), 5)
  assert_eq(contar_vocales("xkcd"), 0)
end

test "tipo título" = assert_eq(tipo_titulo("el SEÑOR de los anillos"), "El Señor De Los Anillos")
"#,
        ),
        ("19-sorting", "19-ordenar"),
        (
            r#"# Exercise 19 · Sorting  ★★★
#
# Two classic ways to sort a list, written with recursion (without the
# built-in `sort`).
#
# Statement
#   - `insert(sorted, x)`: x placed at its position in a list that is
#     already sorted;
#   - `insertion_sort(list)`: sorts the list by inserting each element, one
#     after the other, into a sorted list that starts empty;
#   - `merge(a, b)`: one sorted list from two sorted lists;
#   - `merge_sort(list)`: cuts the list in two halves, sorts each half,
#     and merges them.
#
# Hints
#   1. `insert([], x)` is `[x]`. For `insert([first, ..rest], x)`: if
#      x <= first, x goes in front; otherwise first stays first.
#   2. `insertion_sort([x, ..rest])` is `insert(insertion_sort(rest), x)`
#      (or use `fold` with `insert`).
#   3. `merge` compares the first elements of the two lists: the smaller one
#      comes first, followed by the merge of what remains.
#   4. `take(list, n)` and `drop(list, n)` give the two halves, with
#      `n = len(list) // 2`. A list of 0 or 1 element is already sorted.
#
# Check your work:  rythme exercise check 19-sorting.ry
#              or:  rythme test 19-sorting.ry

fn insert(sorted, x) = fail("to do")

fn insertion_sort(list) = fail("to do")

fn merge(a, b) = fail("to do")

fn merge_sort(list) = fail("to do")

# --- Tests (do not change them) ---

test "insert"
  assert_eq(insert([1, 3, 5], 4), [1, 3, 4, 5])
  assert_eq(insert([], 2), [2])
  assert_eq(insert([1, 2], 9), [1, 2, 9])
end

test "insertion sort"
  assert_eq(insertion_sort([5, 2, 9, 1, 5, 6]), [1, 2, 5, 5, 6, 9])
  assert_eq(insertion_sort([]), [])
end

test "merge"
  assert_eq(merge([1, 4, 6], [2, 3, 7]), [1, 2, 3, 4, 6, 7])
  assert_eq(merge([], [1]), [1])
  assert_eq(merge([1], []), [1])
end

test "merge sort"
  assert_eq(merge_sort([5, 2, 9, 1, 5, 6]), [1, 2, 5, 5, 6, 9])
  assert_eq(merge_sort(["pear", "apple", "fig"]), ["apple", "fig", "pear"])
  assert_eq(merge_sort(reverse(1..100)), 1..100)
end
"#,
            r#"# Ejercicio 19 · Ordenar  ★★★
#
# Dos formas clásicas de ordenar una lista, escritas con recursión (sin la
# función predefinida `sort`).
#
# Enunciado
#   - `insertar(ordenada, x)`: x colocado en su lugar en una lista que ya
#     está ordenada;
#   - `ordenar_por_insercion(lista)`: ordena la lista insertando cada
#     elemento, uno tras otro, en una lista ordenada que empieza vacía;
#   - `mezclar(a, b)`: una lista ordenada a partir de dos listas ordenadas;
#   - `ordenar_por_mezcla(lista)`: corta la lista en dos mitades, ordena
#     cada mitad, y las mezcla.
#
# Pistas
#   1. `insertar([], x)` es `[x]`. Para `insertar([primero, ..resto], x)`:
#      si x <= primero, x va delante; si no, primero sigue primero.
#   2. `ordenar_por_insercion([x, ..resto])` es
#      `insertar(ordenar_por_insercion(resto), x)` (o usa `fold` con
#      `insertar`).
#   3. `mezclar` compara los primeros elementos de las dos listas: el más
#      pequeño va primero, seguido de la mezcla de lo que queda.
#   4. `take(lista, n)` y `drop(lista, n)` dan las dos mitades, con
#      `n = len(lista) // 2`. Una lista de 0 o 1 elemento ya está ordenada.
#
# Comprueba tu trabajo:  rythme exercise check 19-ordenar.ry
#                    o:  rythme test 19-ordenar.ry

fn insertar(ordenada, x) = fail("por hacer")

fn ordenar_por_insercion(lista) = fail("por hacer")

fn mezclar(a, b) = fail("por hacer")

fn ordenar_por_mezcla(lista) = fail("por hacer")

# --- Pruebas (no las cambies) ---

test "insertar"
  assert_eq(insertar([1, 3, 5], 4), [1, 3, 4, 5])
  assert_eq(insertar([], 2), [2])
  assert_eq(insertar([1, 2], 9), [1, 2, 9])
end

test "ordenar por inserción"
  assert_eq(ordenar_por_insercion([5, 2, 9, 1, 5, 6]), [1, 2, 5, 5, 6, 9])
  assert_eq(ordenar_por_insercion([]), [])
end

test "mezclar"
  assert_eq(mezclar([1, 4, 6], [2, 3, 7]), [1, 2, 3, 4, 6, 7])
  assert_eq(mezclar([], [1]), [1])
  assert_eq(mezclar([1], []), [1])
end

test "ordenar por mezcla"
  assert_eq(ordenar_por_mezcla([5, 2, 9, 1, 5, 6]), [1, 2, 5, 5, 6, 9])
  assert_eq(ordenar_por_mezcla(["pera", "manzana", "higo"]), ["higo", "manzana", "pera"])
  assert_eq(ordenar_por_mezcla(reverse(1..100)), 1..100)
end
"#,
        ),
        ("20-records", "20-registros"),
        (
            r#"# Exercise 20 · Records  ★★☆
#
# A record groups named values: `{name: "Ada", born: 1815}`. `ada.name`
# reads a field. A record never changes: `{..ada, born: 1816}` is a new
# record, a copy of ada with another `born`. A parameter can take a record
# apart: `fn hello({name}) = "Hello, {name}"`.
#
# Statement
#   A person is a record `{name, born}`; a point is a record `{x, y}`.
#   - `age_in(person, year)`: the age of the person at the end of the year;
#   - `label(person)`: a text like "Ada (1815)";
#   - `renamed(person, name)`: the same person with another name;
#   - `moved(point, dx, dy)`: the point moved by dx horizontally and dy
#     vertically;
#   - `distance(a, b)`: the distance between two points.
#
# Hints
#   1. `person.born`, or a parameter `{born}`.
#   2. `{..person, name: name}`, which can be written `{..person, name}`.
#   3. The distance is the square root of (xb - xa)² + (yb - ya)².
#
# Check your work:  rythme exercise check 20-records.ry
#              or:  rythme test 20-records.ry

fn age_in(person, year) = fail("to do")

fn label(person) = fail("to do")

fn renamed(person, name) = fail("to do")

fn moved(point, dx, dy) = fail("to do")

fn distance(a, b) = fail("to do")

# --- Tests (do not change them) ---

ada = {name: "Ada", born: 1815}

test "age" = assert_eq(age_in(ada, 1852), 37)

test "label" = assert_eq(label(ada), "Ada (1815)")

test "renamed"
  assert_eq(renamed(ada, "Augusta"), {name: "Augusta", born: 1815})
  assert_eq(ada.name, "Ada")
end

test "moved"
  assert_eq(moved({x: 1, y: 2}, 3, -1), {x: 4, y: 1})
  assert_eq(moved({x: 0, y: 0}, 0, 0), {x: 0, y: 0})
end

test "distance"
  assert_eq(distance({x: 0, y: 0}, {x: 3, y: 4}), 5)
  assert_eq(distance({x: 1, y: 1}, {x: 1, y: 1}), 0)
end
"#,
            r#"# Ejercicio 20 · Registros  ★★☆
#
# Un registro agrupa valores con nombre: `{nombre: "Ada", nacimiento: 1815}`.
# `ada.nombre` lee un campo. Un registro nunca cambia:
# `{..ada, nacimiento: 1816}` es un registro nuevo, una copia de ada con
# otro `nacimiento`. Un parámetro puede desarmar un registro:
# `fn hola({nombre}) = "Hola, {nombre}"`.
#
# Enunciado
#   Una persona es un registro `{nombre, nacimiento}`; un punto es un
#   registro `{x, y}`.
#   - `edad_en(persona, anio)`: la edad de la persona al final del año;
#   - `etiqueta(persona)`: un texto como "Ada (1815)";
#   - `renombrada(persona, nombre)`: la misma persona con otro nombre;
#   - `desplazado(punto, dx, dy)`: el punto desplazado dx en horizontal y
#     dy en vertical;
#   - `distancia(a, b)`: la distancia entre dos puntos.
#
# Pistas
#   1. `persona.nacimiento`, o un parámetro `{nacimiento}`.
#   2. `{..persona, nombre: nombre}`, que se puede escribir
#      `{..persona, nombre}`.
#   3. La distancia es la raíz cuadrada de (xb - xa)² + (yb - ya)².
#
# Comprueba tu trabajo:  rythme exercise check 20-registros.ry
#                    o:  rythme test 20-registros.ry

fn edad_en(persona, anio) = fail("por hacer")

fn etiqueta(persona) = fail("por hacer")

fn renombrada(persona, nombre) = fail("por hacer")

fn desplazado(punto, dx, dy) = fail("por hacer")

fn distancia(a, b) = fail("por hacer")

# --- Pruebas (no las cambies) ---

ada = {nombre: "Ada", nacimiento: 1815}

test "edad" = assert_eq(edad_en(ada, 1852), 37)

test "etiqueta" = assert_eq(etiqueta(ada), "Ada (1815)")

test "renombrada"
  assert_eq(renombrada(ada, "Augusta"), {nombre: "Augusta", nacimiento: 1815})
  assert_eq(ada.nombre, "Ada")
end

test "desplazado"
  assert_eq(desplazado({x: 1, y: 2}, 3, -1), {x: 4, y: 1})
  assert_eq(desplazado({x: 0, y: 0}, 0, 0), {x: 0, y: 0})
end

test "distancia"
  assert_eq(distancia({x: 0, y: 0}, {x: 3, y: 4}), 5)
  assert_eq(distancia({x: 1, y: 1}, {x: 1, y: 1}), 0)
end
"#,
        ),
        ("21-lists-of-records", "21-listas-de-registros"),
        (
            r#"# Exercise 21 · Lists of records  ★★☆
#
# Real data is often a list of records: a class of students, a list of
# orders... `map`, `filter`, `sort_by`, `max_by`, `group_by` and `sum` work
# on them like on any list.
#
# Statement
#   A student is a record `{name, score, group}` (a score from 0 to 100).
#   - `names_of_passed(students)`: the names of the students whose score is
#     at least 50, in the same order;
#   - `average_score(students)`: the average of the scores;
#   - `best(students)`: the name of the student with the best score;
#   - `ranking(students)`: the names, from the best score to the lowest;
#   - `group_sizes(students)`: a record that gives, for each group, how
#     many students are in it (`{A: 2, B: 1}`).
#
# Hints
#   1. `filter` then `map(fn(s) = s.name)`.
#   2. `max_by(list, key)` gives the element whose key is the largest.
#   3. `sort_by(list, fn(s) = -s.score)` sorts from the best score.
#   4. `group_by(list, key)` gives a record of lists, and
#      `map_values(record, f)` applies f to each of its values.
#
# Check your work:  rythme exercise check 21-lists-of-records.ry
#              or:  rythme test 21-lists-of-records.ry

fn names_of_passed(students) = fail("to do")

fn average_score(students) = fail("to do")

fn best(students) = fail("to do")

fn ranking(students) = fail("to do")

fn group_sizes(students) = fail("to do")

# --- Tests (do not change them) ---

classroom = [
  {name: "Ada", score: 92, group: "A"},
  {name: "Alan", score: 45, group: "B"},
  {name: "Grace", score: 78, group: "A"},
  {name: "Edsger", score: 50, group: "B"},
  {name: "Barbara", score: 85, group: "A"},
]

test "passed" = assert_eq(names_of_passed(classroom), ["Ada", "Grace", "Edsger", "Barbara"])

test "average" = assert_eq(average_score(classroom), 70)

test "best" = assert_eq(best(classroom), "Ada")

test "ranking" = assert_eq(ranking(classroom), ["Ada", "Barbara", "Grace", "Edsger", "Alan"])

test "group sizes" = assert_eq(group_sizes(classroom), {A: 3, B: 2})
"#,
            r#"# Ejercicio 21 · Listas de registros  ★★☆
#
# Los datos reales suelen ser una lista de registros: una clase de
# alumnos, una lista de pedidos... `map`, `filter`, `sort_by`, `max_by`,
# `group_by` y `sum` funcionan con ellos como con cualquier lista.
#
# Enunciado
#   Un alumno es un registro `{nombre, nota, grupo}` (una nota de 0 a
#   100).
#   - `nombres_de_aprobados(alumnos)`: los nombres de los alumnos cuya
#     nota es al menos 50, en el mismo orden;
#   - `nota_media(alumnos)`: la media de las notas;
#   - `mejor(alumnos)`: el nombre del alumno con la mejor nota;
#   - `clasificacion(alumnos)`: los nombres, de la mejor nota a la más
#     baja;
#   - `tamanos_de_grupo(alumnos)`: un registro que da, para cada grupo,
#     cuántos alumnos tiene (`{A: 2, B: 1}`).
#
# Pistas
#   1. `filter` y luego `map(fn(a) = a.nombre)`.
#   2. `max_by(lista, clave)` da el elemento cuya clave es la mayor.
#   3. `sort_by(lista, fn(a) = -a.nota)` ordena desde la mejor nota.
#   4. `group_by(lista, clave)` da un registro de listas, y
#      `map_values(registro, f)` aplica f a cada uno de sus valores.
#
# Comprueba tu trabajo:  rythme exercise check 21-listas-de-registros.ry
#                    o:  rythme test 21-listas-de-registros.ry

fn nombres_de_aprobados(alumnos) = fail("por hacer")

fn nota_media(alumnos) = fail("por hacer")

fn mejor(alumnos) = fail("por hacer")

fn clasificacion(alumnos) = fail("por hacer")

fn tamanos_de_grupo(alumnos) = fail("por hacer")

# --- Pruebas (no las cambies) ---

clase = [
  {nombre: "Ada", nota: 92, grupo: "A"},
  {nombre: "Alan", nota: 45, grupo: "B"},
  {nombre: "Grace", nota: 78, grupo: "A"},
  {nombre: "Edsger", nota: 50, grupo: "B"},
  {nombre: "Barbara", nota: 85, grupo: "A"},
]

test "aprobados" = assert_eq(nombres_de_aprobados(clase), ["Ada", "Grace", "Edsger", "Barbara"])

test "media" = assert_eq(nota_media(clase), 70)

test "mejor" = assert_eq(mejor(clase), "Ada")

test "clasificación" = assert_eq(clasificacion(clase), ["Ada", "Barbara", "Grace", "Edsger", "Alan"])

test "tamaños de grupo" = assert_eq(tamanos_de_grupo(clase), {A: 3, B: 2})
"#,
        ),
        ("22-own-types", "22-tipos-propios"),
        (
            r#"# Exercise 22 · Your own types  ★★☆
#
# `type` defines a new kind of value, with one constructor for each
# possible form:
#   type Pet = Dog(name) | Cat(name, lives) | Fish
# A function can have one definition per constructor:
#   fn sound(Dog(_)) = "woof"
#
# Statement
#   The type `Shape` below has three forms.
#   - `area(shape)`: the area of the shape;
#   - `perimeter(shape)`: the length of its border;
#   - `name_of(shape)`: "circle", "rectangle" or "square";
#   - `total_area(shapes)`: the sum of the areas of a list of shapes.
#
# Hints
#   1. `fn area(Circle(r)) = pi * r * r`, then one definition for each
#      other constructor.
#   2. The perimeter of a circle is 2 × π × r.
#   3. If you forget a constructor, `rythme check` tells you which one
#      (warning W002).
#
# Check your work:  rythme exercise check 22-own-types.ry
#              or:  rythme test 22-own-types.ry

type Shape = Circle(radius) | Rect(width, height) | Square(side)

fn area(shape) = fail("to do")

fn perimeter(shape) = fail("to do")

fn name_of(shape) = fail("to do")

fn total_area(shapes) = fail("to do")

# --- Tests (do not change them) ---

test "area"
  assert_eq(area(Rect(2, 3)), 6)
  assert_eq(area(Square(4)), 16)
  assert_eq(round(area(Circle(1)), 2), 3.14)
end

test "perimeter"
  assert_eq(perimeter(Rect(2, 3)), 10)
  assert_eq(perimeter(Square(4)), 16)
  assert_eq(round(perimeter(Circle(1)), 2), 6.28)
end

test "name" = assert_eq([Circle(1), Rect(1, 2), Square(3)] |> map(name_of), ["circle", "rectangle", "square"])

test "total area" = assert_eq(total_area([Rect(2, 3), Square(2), Rect(1, 1)]), 11)
"#,
            r#"# Ejercicio 22 · Tus propios tipos  ★★☆
#
# `type` define una nueva clase de valor, con un constructor para cada
# forma posible:
#   type Mascota = Perro(nombre) | Gato(nombre, vidas) | Pez
# Una función puede tener una definición por constructor:
#   fn sonido(Perro(_)) = "guau"
#
# Enunciado
#   El tipo `Figura` de abajo tiene tres formas.
#   - `area(figura)`: el área de la figura;
#   - `perimetro(figura)`: la longitud de su borde;
#   - `nombre_de(figura)`: "círculo", "rectángulo" o "cuadrado";
#   - `area_total(figuras)`: la suma de las áreas de una lista de figuras.
#
# Pistas
#   1. `fn area(Circulo(r)) = pi * r * r`, y luego una definición para
#      cada uno de los otros constructores.
#   2. El perímetro de un círculo es 2 × π × r.
#   3. Si olvidas un constructor, `rythme check` te dice cuál (aviso W002).
#
# Comprueba tu trabajo:  rythme exercise check 22-tipos-propios.ry
#                    o:  rythme test 22-tipos-propios.ry

type Figura = Circulo(radio) | Rectangulo(ancho, alto) | Cuadrado(lado)

fn area(figura) = fail("por hacer")

fn perimetro(figura) = fail("por hacer")

fn nombre_de(figura) = fail("por hacer")

fn area_total(figuras) = fail("por hacer")

# --- Pruebas (no las cambies) ---

test "área"
  assert_eq(area(Rectangulo(2, 3)), 6)
  assert_eq(area(Cuadrado(4)), 16)
  assert_eq(round(area(Circulo(1)), 2), 3.14)
end

test "perímetro"
  assert_eq(perimetro(Rectangulo(2, 3)), 10)
  assert_eq(perimetro(Cuadrado(4)), 16)
  assert_eq(round(perimetro(Circulo(1)), 2), 6.28)
end

test "nombre" = assert_eq([Circulo(1), Rectangulo(1, 2), Cuadrado(3)] |> map(nombre_de), ["círculo", "rectángulo", "cuadrado"])

test "área total" = assert_eq(area_total([Rectangulo(2, 3), Cuadrado(2), Rectangulo(1, 1)]), 11)
"#,
        ),
        ("23-match", "23-match"),
        (
            r#"# Exercise 23 · Choosing with `match`  ★★☆
#
# `match` compares a value with patterns, in order, and gives the result of
# the first one that fits:
#   fn describe(n) = match n
#     0 -> "zero"
#     n if n < 0 -> "negative"
#     _ -> "positive"
#   end
# Patterns can be numbers, texts, constructors, lists (`[]`, `[x]`,
# `[x, ..rest]`) or records (`{name}`).
#
# Statement
#   - `next_light(light)`: the light that comes after: Green, then Orange,
#     then Red, then Green again;
#   - `can_go(light)`: true only for Green;
#   - `describe(list)`: "empty" for [], "one: 7" for [7], "two: 1 and 2"
#     for [1, 2], and "many, starting with 5" for longer lists;
#   - `greeting(person)`: for a record with a `title` field,
#     "Hello, Dr Hopper"; otherwise, with only `name`, "Hello, Grace".
#
# Hints
#   1. Each of the three lights is one case of the `match`.
#   2. `[a, b]` matches lists of exactly two elements, `[first, ..]` lists
#      of at least one element: put the most precise patterns first.
#   3. `{title, last}` matches records that have at least these fields.
#
# Check your work:  rythme exercise check 23-match.ry
#              or:  rythme test 23-match.ry

type Light = Green | Orange | Red

fn next_light(light) = fail("to do")

fn can_go(light) = fail("to do")

fn describe(list) = fail("to do")

fn greeting(person) = fail("to do")

# --- Tests (do not change them) ---

test "next light"
  assert_eq(next_light(Green), Orange)
  assert_eq(next_light(Orange), Red)
  assert_eq(next_light(Red), Green)
end

test "can go"
  assert_eq(can_go(Green), true)
  assert_eq(can_go(Orange), false)
  assert_eq(can_go(Red), false)
end

test "describe"
  assert_eq(describe([]), "empty")
  assert_eq(describe([7]), "one: 7")
  assert_eq(describe([1, 2]), "two: 1 and 2")
  assert_eq(describe([5, 6, 7]), "many, starting with 5")
end

test "greeting"
  assert_eq(greeting({title: "Dr", last: "Hopper", name: "Grace"}), "Hello, Dr Hopper")
  assert_eq(greeting({name: "Grace"}), "Hello, Grace")
end
"#,
            r#"# Ejercicio 23 · Elegir con `match`  ★★☆
#
# `match` compara un valor con patrones, en orden, y da el resultado del
# primero que encaja:
#   fn describir(n) = match n
#     0 -> "cero"
#     n if n < 0 -> "negativo"
#     _ -> "positivo"
#   end
# Los patrones pueden ser números, textos, constructores, listas (`[]`,
# `[x]`, `[x, ..resto]`) o registros (`{nombre}`).
#
# Enunciado
#   - `siguiente_luz(luz)`: la luz que viene después: Verde, luego Ambar,
#     luego Rojo, y luego Verde otra vez;
#   - `puede_pasar(luz)`: true solo para Verde;
#   - `describir(lista)`: "vacía" para [], "uno: 7" para [7],
#     "dos: 1 y 2" para [1, 2], y "varios, empezando por 5" para listas más
#     largas;
#   - `saludo(persona)`: para un registro con un campo `titulo`,
#     "Hola, Dra. Hopper"; si no, solo con `nombre`, "Hola, Grace".
#
# Pistas
#   1. Cada una de las tres luces es un caso del `match`.
#   2. `[a, b]` encaja con listas de exactamente dos elementos,
#      `[primero, ..]` con listas de al menos un elemento: pon primero los
#      patrones más precisos.
#   3. `{titulo, apellido}` encaja con los registros que tienen al menos
#      esos campos.
#
# Comprueba tu trabajo:  rythme exercise check 23-match.ry
#                    o:  rythme test 23-match.ry

type Luz = Verde | Ambar | Rojo

fn siguiente_luz(luz) = fail("por hacer")

fn puede_pasar(luz) = fail("por hacer")

fn describir(lista) = fail("por hacer")

fn saludo(persona) = fail("por hacer")

# --- Pruebas (no las cambies) ---

test "siguiente luz"
  assert_eq(siguiente_luz(Verde), Ambar)
  assert_eq(siguiente_luz(Ambar), Rojo)
  assert_eq(siguiente_luz(Rojo), Verde)
end

test "puede pasar"
  assert_eq(puede_pasar(Verde), true)
  assert_eq(puede_pasar(Ambar), false)
  assert_eq(puede_pasar(Rojo), false)
end

test "describir"
  assert_eq(describir([]), "vacía")
  assert_eq(describir([7]), "uno: 7")
  assert_eq(describir([1, 2]), "dos: 1 y 2")
  assert_eq(describir([5, 6, 7]), "varios, empezando por 5")
end

test "saludo"
  assert_eq(saludo({titulo: "Dra.", apellido: "Hopper", nombre: "Grace"}), "Hola, Dra. Hopper")
  assert_eq(saludo({nombre: "Grace"}), "Hola, Grace")
end
"#,
        ),
        ("24-trees", "24-arboles"),
        (
            r#"# Exercise 24 · Trees  ★★★
#
# A type can contain itself. A binary tree is either empty (`Leaf`), or a
# node with a value and two smaller trees, on its left and on its right.
# Functions on trees follow the shape of the type: one case for `Leaf`, one
# for `Node`, which calls itself on both sides.
#
# Statement
#   - `size(tree)`: how many values the tree holds;
#   - `total(tree)`: the sum of its values;
#   - `depth(tree)`: the number of levels (0 for a Leaf);
#   - `insert(tree, x)`: a search tree is a tree where the values on the
#     left of a node are smaller than its value, and the others on the
#     right; give the search tree with x added (a new Node in place of a
#     Leaf);
#   - `to_list(tree)`: the values, from left to right.
#
# Hints
#   1. `fn size(Leaf) = 0` and `fn size(Node(left, _, right)) = ...`.
#   2. `max(a, b)` gives the larger of two numbers.
#   3. `insert` gives a new tree: `Node(insert(left, x), value, right)`
#      when x goes to the left.
#   4. `to_list` of a node is: the list of the left, then the value, then
#      the list of the right (`+` joins lists).
#
# Check your work:  rythme exercise check 24-trees.ry
#              or:  rythme test 24-trees.ry

type Tree = Leaf | Node(left, value, right)

fn size(tree) = fail("to do")

fn total(tree) = fail("to do")

fn depth(tree) = fail("to do")

fn insert(tree, x) = fail("to do")

fn to_list(tree) = fail("to do")

# --- Tests (do not change them) ---

#       5
#      / \
#     3   8
#    /
#   1
small = Node(Node(Node(Leaf, 1, Leaf), 3, Leaf), 5, Node(Leaf, 8, Leaf))

test "size"
  assert_eq(size(Leaf), 0)
  assert_eq(size(small), 4)
end

test "total" = assert_eq(total(small), 17)

test "depth"
  assert_eq(depth(Leaf), 0)
  assert_eq(depth(small), 3)
end

test "insert"
  assert_eq(insert(Leaf, 4), Node(Leaf, 4, Leaf))
  assert_eq(insert(Node(Leaf, 4, Leaf), 2), Node(Node(Leaf, 2, Leaf), 4, Leaf))
end

test "to list" = assert_eq(to_list(small), [1, 3, 5, 8])

test "sorting with a tree" = assert_eq([7, 2, 9, 4, 1] |> fold(Leaf, insert) |> to_list, [1, 2, 4, 7, 9])
"#,
            r#"# Ejercicio 24 · Árboles  ★★★
#
# Un tipo puede contenerse a sí mismo. Un árbol binario es o bien vacío
# (`Hoja`), o bien un nodo con un valor y dos árboles más pequeños, a su
# izquierda y a su derecha. Las funciones sobre árboles siguen la forma del
# tipo: un caso para `Hoja`, otro para `Nodo`, que se llama a sí mismo en
# los dos lados.
#
# Enunciado
#   - `tamano(arbol)`: cuántos valores contiene el árbol;
#   - `total(arbol)`: la suma de sus valores;
#   - `profundidad(arbol)`: el número de niveles (0 para una Hoja);
#   - `insertar(arbol, x)`: un árbol de búsqueda es un árbol donde los
#     valores a la izquierda de un nodo son menores que su valor, y los
#     demás a la derecha; da el árbol de búsqueda con x añadido (un Nodo
#     nuevo en lugar de una Hoja);
#   - `a_lista(arbol)`: los valores, de izquierda a derecha.
#
# Pistas
#   1. `fn tamano(Hoja) = 0` y `fn tamano(Nodo(izquierda, _, derecha)) = ...`.
#   2. `max(a, b)` da el mayor de dos números.
#   3. `insertar` da un árbol nuevo:
#      `Nodo(insertar(izquierda, x), valor, derecha)` cuando x va a la
#      izquierda.
#   4. `a_lista` de un nodo es: la lista de la izquierda, luego el valor,
#      luego la lista de la derecha (`+` une listas).
#
# Comprueba tu trabajo:  rythme exercise check 24-arboles.ry
#                    o:  rythme test 24-arboles.ry

type Arbol = Hoja | Nodo(izquierda, valor, derecha)

fn tamano(arbol) = fail("por hacer")

fn total(arbol) = fail("por hacer")

fn profundidad(arbol) = fail("por hacer")

fn insertar(arbol, x) = fail("por hacer")

fn a_lista(arbol) = fail("por hacer")

# --- Pruebas (no las cambies) ---

#       5
#      / \
#     3   8
#    /
#   1
pequeno = Nodo(Nodo(Nodo(Hoja, 1, Hoja), 3, Hoja), 5, Nodo(Hoja, 8, Hoja))

test "tamaño"
  assert_eq(tamano(Hoja), 0)
  assert_eq(tamano(pequeno), 4)
end

test "total" = assert_eq(total(pequeno), 17)

test "profundidad"
  assert_eq(profundidad(Hoja), 0)
  assert_eq(profundidad(pequeno), 3)
end

test "insertar"
  assert_eq(insertar(Hoja, 4), Nodo(Hoja, 4, Hoja))
  assert_eq(insertar(Nodo(Hoja, 4, Hoja), 2), Nodo(Nodo(Hoja, 2, Hoja), 4, Hoja))
end

test "a lista" = assert_eq(a_lista(pequeno), [1, 3, 5, 8])

test "ordenar con un árbol" = assert_eq([7, 2, 9, 4, 1] |> fold(Hoja, insertar) |> a_lista, [1, 2, 4, 7, 9])
"#,
        ),
        ("25-expressions", "25-expresiones"),
        (
            r#"# Exercise 25 · A small calculator  ★★★
#
# A calculation like (1 + 2) × 3 is a tree: the multiplication has two
# parts, the addition and 3. With a type, a program can hold calculations,
# compute them and show them.
#
# Statement
#   - `evaluate(expr)`: the value of the calculation;
#   - `show(expr)`: the calculation as a text, with parentheses around every
#     addition and multiplication: "((1 + 2) * 3)", and a negation written
#     "-x" ("-4", "-(1 + 2)");
#   - `count_numbers(expr)`: how many numbers the calculation contains;
#   - `simplify(expr)`: the same calculation, where `Add(e, Num(0))`,
#     `Mul(e, Num(1))` (and the same with the number on the left) are
#     replaced by e, at every level.
#
# Hints
#   1. One case per constructor: `fn evaluate(Num(n)) = n`,
#      `fn evaluate(Add(a, b)) = evaluate(a) + evaluate(b)`...
#   2. `show(Neg(e))` is "-" followed by `show(e)`.
#   3. In `simplify`, first simplify both parts, then look at the result
#      with a `match`.
#
# Check your work:  rythme exercise check 25-expressions.ry
#              or:  rythme test 25-expressions.ry

type Expr = Num(value) | Add(left, right) | Mul(left, right) | Neg(inner)

fn evaluate(expr) = fail("to do")

fn show(expr) = fail("to do")

fn count_numbers(expr) = fail("to do")

fn simplify(expr) = fail("to do")

# --- Tests (do not change them) ---

calculation = Mul(Add(Num(1), Num(2)), Num(3))

test "evaluate"
  assert_eq(evaluate(Num(4)), 4)
  assert_eq(evaluate(calculation), 9)
  assert_eq(evaluate(Neg(calculation)), -9)
end

test "show"
  assert_eq(show(calculation), "((1 + 2) * 3)")
  assert_eq(show(Neg(Num(4))), "-4")
  assert_eq(show(Neg(Add(Num(1), Num(2)))), "-(1 + 2)")
end

test "count numbers" = assert_eq(count_numbers(Add(calculation, Neg(Num(5)))), 4)

test "simplify"
  assert_eq(simplify(Add(Num(7), Num(0))), Num(7))
  assert_eq(simplify(Mul(Num(1), Add(Num(0), Num(2)))), Num(2))
  assert_eq(simplify(calculation), calculation)
  assert_eq(simplify(Neg(Mul(Num(3), Num(1)))), Neg(Num(3)))
end
"#,
            r#"# Ejercicio 25 · Una pequeña calculadora  ★★★
#
# Un cálculo como (1 + 2) × 3 es un árbol: la multiplicación tiene dos
# partes, la suma y 3. Con un tipo, un programa puede guardar cálculos,
# calcularlos y mostrarlos.
#
# Enunciado
#   - `evaluar(expr)`: el valor del cálculo;
#   - `mostrar(expr)`: el cálculo como texto, con paréntesis alrededor de
#     cada suma y cada multiplicación: "((1 + 2) * 3)", y una negación
#     escrita "-x" ("-4", "-(1 + 2)");
#   - `cantidad_de_numeros(expr)`: cuántos números contiene el cálculo;
#   - `simplificar(expr)`: el mismo cálculo, donde `Suma(e, Num(0))`,
#     `Producto(e, Num(1))` (y lo mismo con el número a la izquierda) se
#     reemplazan por e, en todos los niveles.
#
# Pistas
#   1. Un caso por constructor: `fn evaluar(Num(n)) = n`,
#      `fn evaluar(Suma(a, b)) = evaluar(a) + evaluar(b)`...
#   2. `mostrar(Opuesto(e))` es "-" seguido de `mostrar(e)`.
#   3. En `simplificar`, simplifica primero las dos partes, y luego mira el
#      resultado con un `match`.
#
# Comprueba tu trabajo:  rythme exercise check 25-expresiones.ry
#                    o:  rythme test 25-expresiones.ry

type Expr = Num(valor) | Suma(izquierda, derecha) | Producto(izquierda, derecha) | Opuesto(interior)

fn evaluar(expr) = fail("por hacer")

fn mostrar(expr) = fail("por hacer")

fn cantidad_de_numeros(expr) = fail("por hacer")

fn simplificar(expr) = fail("por hacer")

# --- Pruebas (no las cambies) ---

calculo = Producto(Suma(Num(1), Num(2)), Num(3))

test "evaluar"
  assert_eq(evaluar(Num(4)), 4)
  assert_eq(evaluar(calculo), 9)
  assert_eq(evaluar(Opuesto(calculo)), -9)
end

test "mostrar"
  assert_eq(mostrar(calculo), "((1 + 2) * 3)")
  assert_eq(mostrar(Opuesto(Num(4))), "-4")
  assert_eq(mostrar(Opuesto(Suma(Num(1), Num(2)))), "-(1 + 2)")
end

test "cantidad de números" = assert_eq(cantidad_de_numeros(Suma(calculo, Opuesto(Num(5)))), 4)

test "simplificar"
  assert_eq(simplificar(Suma(Num(7), Num(0))), Num(7))
  assert_eq(simplificar(Producto(Num(1), Suma(Num(0), Num(2)))), Num(2))
  assert_eq(simplificar(calculo), calculo)
  assert_eq(simplificar(Opuesto(Producto(Num(3), Num(1)))), Opuesto(Num(3)))
end
"#,
        ),
        ("26-option", "26-option"),
        (
            r#"# Exercise 26 · A value that may be missing: Option  ★★☆
#
# Rythme has no `null`. A function that may find nothing returns an Option:
# `Some(value)` when there is a value, `None` otherwise. `match` handles
# both cases, and `unwrap_or(option, default)` gives the value or a default.
#   fn first_of([]) = None
#   fn first_of([x, ..]) = Some(x)
#
# Statement
#   - `safe_divide(a, b)`: `Some(a / b)`, or `None` when b is 0;
#   - `find_age(people, name)`: the age of the person with this name, as an
#     Option (people is a list of records `{name, age}`);
#   - `age_or_unknown(people, name)`: a text, "36" for a known person and
#     "unknown" otherwise;
#   - `first_long_word(words)`: the first word with more than 5
#     characters, as an Option;
#   - `add_options(a, b)`: `Some(x + y)` when a is `Some(x)` and b is
#     `Some(y)`, `None` otherwise.
#
# Hints
#   1. `find(list, f)` gives the first element for which f is true, as an
#      Option; `map(option, f)` applies f to the value inside, if any.
#   2. `match find_age(people, name)` with the cases `Some(age)` and `None`.
#   3. `match [a, b]` with a case `[Some(x), Some(y)]` and a case `_`.
#
# Check your work:  rythme exercise check 26-option.ry
#              or:  rythme test 26-option.ry

fn safe_divide(a, b) = fail("to do")

fn find_age(people, name) = fail("to do")

fn age_or_unknown(people, name) = fail("to do")

fn first_long_word(words) = fail("to do")

fn add_options(a, b) = fail("to do")

# --- Tests (do not change them) ---

people = [{name: "Ada", age: 36}, {name: "Alan", age: 41}]

test "safe divide"
  assert_eq(safe_divide(10, 4), Some(2.5))
  assert_eq(safe_divide(1, 0), None)
end

test "find age"
  assert_eq(find_age(people, "Alan"), Some(41))
  assert_eq(find_age(people, "Grace"), None)
end

test "age or unknown"
  assert_eq(age_or_unknown(people, "Ada"), "36")
  assert_eq(age_or_unknown(people, "Grace"), "unknown")
end

test "first long word"
  assert_eq(first_long_word(["a", "functional", "language"]), Some("functional"))
  assert_eq(first_long_word(["short", "words"]), None)
end

test "add options"
  assert_eq(add_options(Some(2), Some(3)), Some(5))
  assert_eq(add_options(Some(2), None), None)
  assert_eq(add_options(None, None), None)
end
"#,
            r#"# Ejercicio 26 · Un valor que puede faltar: Option  ★★☆
#
# Rythme no tiene `null`. Una función que puede no encontrar nada devuelve
# un Option: `Some(valor)` cuando hay un valor, `None` si no. `match`
# maneja los dos casos, y `unwrap_or(opcion, por_defecto)` da el valor o un
# valor por defecto.
#   fn primero_de([]) = None
#   fn primero_de([x, ..]) = Some(x)
#
# Enunciado
#   - `division_segura(a, b)`: `Some(a / b)`, o `None` cuando b es 0;
#   - `buscar_edad(personas, nombre)`: la edad de la persona con ese
#     nombre, como Option (personas es una lista de registros
#     `{nombre, edad}`);
#   - `edad_o_desconocida(personas, nombre)`: un texto, "36" para una
#     persona conocida y "desconocida" si no;
#   - `primera_palabra_larga(palabras)`: la primera palabra con más de 5
#     caracteres, como Option;
#   - `sumar_opciones(a, b)`: `Some(x + y)` cuando a es `Some(x)` y b es
#     `Some(y)`, `None` si no.
#
# Pistas
#   1. `find(lista, f)` da el primer elemento para el que f es true, como
#      Option; `map(opcion, f)` aplica f al valor de dentro, si lo hay.
#   2. `match buscar_edad(personas, nombre)` con los casos `Some(edad)` y
#      `None`.
#   3. `match [a, b]` con un caso `[Some(x), Some(y)]` y un caso `_`.
#
# Comprueba tu trabajo:  rythme exercise check 26-option.ry
#                    o:  rythme test 26-option.ry

fn division_segura(a, b) = fail("por hacer")

fn buscar_edad(personas, nombre) = fail("por hacer")

fn edad_o_desconocida(personas, nombre) = fail("por hacer")

fn primera_palabra_larga(palabras) = fail("por hacer")

fn sumar_opciones(a, b) = fail("por hacer")

# --- Pruebas (no las cambies) ---

personas = [{nombre: "Ada", edad: 36}, {nombre: "Alan", edad: 41}]

test "división segura"
  assert_eq(division_segura(10, 4), Some(2.5))
  assert_eq(division_segura(1, 0), None)
end

test "buscar la edad"
  assert_eq(buscar_edad(personas, "Alan"), Some(41))
  assert_eq(buscar_edad(personas, "Grace"), None)
end

test "edad o desconocida"
  assert_eq(edad_o_desconocida(personas, "Ada"), "36")
  assert_eq(edad_o_desconocida(personas, "Grace"), "desconocida")
end

test "primera palabra larga"
  assert_eq(primera_palabra_larga(["un", "lenguaje", "funcional"]), Some("lenguaje"))
  assert_eq(primera_palabra_larga(["solo", "esto"]), None)
end

test "sumar opciones"
  assert_eq(sumar_opciones(Some(2), Some(3)), Some(5))
  assert_eq(sumar_opciones(Some(2), None), None)
  assert_eq(sumar_opciones(None, None), None)
end
"#,
        ),
        ("27-result", "27-result"),
        (
            r#"# Exercise 27 · An operation that may fail: Result  ★★☆
#
# Rythme has no exceptions. A function that may fail returns a Result:
# `Ok(value)` when it worked, `Err(message)` otherwise. The caller must
# look at both cases: no error can be forgotten.
#   parse_int("42")       # Ok(42)
#   parse_int("forty")    # Err("\"forty\" is not a whole number")
#
# Statement
#   - `checked_divide(a, b)`: `Ok(a / b)`, or `Err("division by zero")`;
#   - `parse_age(text)`: the age written in the text, as a Result:
#     `Err("not a number")` if the text is not a whole number,
#     `Err("an age cannot be negative")` below 0,
#     `Err("too old")` above 150, and `Ok(age)` otherwise;
#   - `describe(result)`: "ok: 42" for `Ok(42)` and "error: too old" for
#     `Err("too old")`;
#   - `check_code(code)`: `Ok(code)` if it has at least 8
#     characters and contains a digit; otherwise an Err with the first
#     problem found: "too short" or "no digit".
#
# Hints
#   1. `match parse_int(text)` with the cases `Err(_)`, `Ok(n) if n < 0`,
#      `Ok(n) if n > 150` and `Ok(n)`.
#   2. `any(chars(code), is_digit)` tells whether there is a digit.
#
# Check your work:  rythme exercise check 27-result.ry
#              or:  rythme test 27-result.ry

fn checked_divide(a, b) = fail("to do")

fn parse_age(text) = fail("to do")

fn describe(result) = fail("to do")

fn check_code(code) = fail("to do")

# --- Tests (do not change them) ---

test "checked divide"
  assert_eq(checked_divide(9, 3), Ok(3))
  assert_eq(checked_divide(1, 0), Err("division by zero"))
end

test "parse age"
  assert_eq(parse_age("36"), Ok(36))
  assert_eq(parse_age(" 7 "), Ok(7))
  assert_eq(parse_age("seven"), Err("not a number"))
  assert_eq(parse_age("-1"), Err("an age cannot be negative"))
  assert_eq(parse_age("200"), Err("too old"))
end

test "describe"
  assert_eq(describe(Ok(42)), "ok: 42")
  assert_eq(describe(Err("too old")), "error: too old")
  assert_eq(describe(parse_age("-5")), "error: an age cannot be negative")
end

test "check code"
  assert_eq(check_code("rythme2024"), Ok("rythme2024"))
  assert_eq(check_code("abc1"), Err("too short"))
  assert_eq(check_code("functional"), Err("no digit"))
end
"#,
            r#"# Ejercicio 27 · Una operación que puede fallar: Result  ★★☆
#
# Rythme no tiene excepciones. Una función que puede fallar devuelve un
# Result: `Ok(valor)` cuando funcionó, `Err(mensaje)` si no. Quien la llama
# debe mirar los dos casos: ningún error se puede olvidar.
#   parse_int("42")         # Ok(42)
#   parse_int("cuarenta")   # Err("\"cuarenta\" is not a whole number")
#
# Enunciado
#   - `division_comprobada(a, b)`: `Ok(a / b)`, o
#     `Err("división entre cero")`;
#   - `leer_edad(texto)`: la edad escrita en el texto, como Result:
#     `Err("no es un número")` si el texto no es un número entero,
#     `Err("una edad no puede ser negativa")` por debajo de 0,
#     `Err("demasiado mayor")` por encima de 150, y `Ok(edad)` si no;
#   - `describir(resultado)`: "ok: 42" para `Ok(42)` y
#     "error: demasiado mayor" para `Err("demasiado mayor")`;
#   - `comprobar_codigo(codigo)`: `Ok(codigo)` si tiene al menos 8
#     caracteres y contiene una cifra; si no, un Err con el primer problema
#     encontrado: "demasiado corto" o "ninguna cifra".
#
# Pistas
#   1. `match parse_int(texto)` con los casos `Err(_)`, `Ok(n) if n < 0`,
#      `Ok(n) if n > 150` y `Ok(n)`.
#   2. `any(chars(codigo), is_digit)` dice si hay una cifra.
#
# Comprueba tu trabajo:  rythme exercise check 27-result.ry
#                    o:  rythme test 27-result.ry

fn division_comprobada(a, b) = fail("por hacer")

fn leer_edad(texto) = fail("por hacer")

fn describir(resultado) = fail("por hacer")

fn comprobar_codigo(codigo) = fail("por hacer")

# --- Pruebas (no las cambies) ---

test "división comprobada"
  assert_eq(division_comprobada(9, 3), Ok(3))
  assert_eq(division_comprobada(1, 0), Err("división entre cero"))
end

test "leer la edad"
  assert_eq(leer_edad("36"), Ok(36))
  assert_eq(leer_edad(" 7 "), Ok(7))
  assert_eq(leer_edad("siete"), Err("no es un número"))
  assert_eq(leer_edad("-1"), Err("una edad no puede ser negativa"))
  assert_eq(leer_edad("200"), Err("demasiado mayor"))
end

test "describir"
  assert_eq(describir(Ok(42)), "ok: 42")
  assert_eq(describir(Err("demasiado mayor")), "error: demasiado mayor")
  assert_eq(describir(leer_edad("-5")), "error: una edad no puede ser negativa")
end

test "comprobar el código"
  assert_eq(comprobar_codigo("rythme2024"), Ok("rythme2024"))
  assert_eq(comprobar_codigo("abc1"), Err("demasiado corto"))
  assert_eq(comprobar_codigo("funcional"), Err("ninguna cifra"))
end
"#,
        ),
        ("28-question-mark", "28-signo-de-interrogacion"),
        (
            r#"# Exercise 28 · Passing errors on with `?`  ★★★
#
# Writing a `match` after each step that may fail is long. `?` after a
# Result (or an Option) takes the value out of `Ok(...)` (or `Some(...)`),
# or stops the function at once and returns the `Err(...)` (or `None`):
#   fn double_text(t)
#     n = parse_int(t)?          # stops here if t is not a number
#     Ok(n * 2)
#   end
#
# Statement
#   - `add_texts(a, b)`: the sum of the two numbers written in a and b,
#     as `Ok(sum)`, or the error of `parse_int`;
#   - `parse_point(text)`: for "3,4", `Ok({x: 3, y: 4})`; the error
#     `Err("expected x,y")` if the text does not have exactly two parts
#     separated by a comma, or the error of `parse_int` for a part that is
#     not a number;
#   - `sum_of_texts(texts)`: the sum of the numbers written in a list of
#     texts, or the first error;
#   - `ratio_of_texts(a, b)`: a / b for two numbers written in texts, or the
#     first error, with `Err("division by zero")` when b is 0.
#
# Hints
#   1. `Ok(parse_int(a)? + parse_int(b)?)`
#   2. `match split(text, ",")` with the cases `[x, y]` and `_`.
#   3. For a list, use recursion: `sum_of_texts([])` is `Ok(0)`; for
#      `[t, ..rest]`, take the number of t and the sum of rest, both with
#      `?`.
#   4. `?` can be used on any function that gives a Result, your own ones
#      too.
#
# Check your work:  rythme exercise check 28-question-mark.ry
#              or:  rythme test 28-question-mark.ry

fn add_texts(a, b) = fail("to do")

fn parse_point(text) = fail("to do")

fn sum_of_texts(texts) = fail("to do")

fn ratio_of_texts(a, b) = fail("to do")

# --- Tests (do not change them) ---

test "add texts"
  assert_eq(add_texts("2", "40"), Ok(42))
  assert(is_err(add_texts("2", "forty")), "forty is not a number")
end

test "parse point"
  assert_eq(parse_point("3,4"), Ok({x: 3, y: 4}))
  assert_eq(parse_point("3"), Err("expected x,y"))
  assert_eq(parse_point("1,2,3"), Err("expected x,y"))
  assert(is_err(parse_point("3,four")), "four is not a number")
end

test "sum of texts"
  assert_eq(sum_of_texts(["1", "2", "3"]), Ok(6))
  assert_eq(sum_of_texts([]), Ok(0))
  assert_eq(sum_of_texts(["1", "x", "y"]), parse_int("x"))
end

test "ratio of texts"
  assert_eq(ratio_of_texts("9", "2"), Ok(4.5))
  assert_eq(ratio_of_texts("9", "0"), Err("division by zero"))
  assert(is_err(ratio_of_texts("nine", "0")), "nine is not a number")
end
"#,
            r#"# Ejercicio 28 · Transmitir los errores con `?`  ★★★
#
# Escribir un `match` después de cada paso que puede fallar es largo. `?`
# después de un Result (o de un Option) saca el valor de `Ok(...)` (o de
# `Some(...)`), o detiene la función enseguida y devuelve el `Err(...)` (o
# `None`):
#   fn doble_texto(t)
#     n = parse_int(t)?          # se detiene aquí si t no es un número
#     Ok(n * 2)
#   end
#
# Enunciado
#   - `sumar_textos(a, b)`: la suma de los dos números escritos en a y b,
#     como `Ok(suma)`, o el error de `parse_int`;
#   - `leer_punto(texto)`: para "3,4", `Ok({x: 3, y: 4})`; el error
#     `Err("se esperaba x,y")` si el texto no tiene exactamente dos partes
#     separadas por una coma, o el error de `parse_int` para una parte que
#     no es un número;
#   - `suma_de_textos(textos)`: la suma de los números escritos en una
#     lista de textos, o el primer error;
#   - `cociente_de_textos(a, b)`: a / b para dos números escritos en
#     textos, o el primer error, con `Err("división entre cero")` cuando b
#     es 0.
#
# Pistas
#   1. `Ok(parse_int(a)? + parse_int(b)?)`
#   2. `match split(texto, ",")` con los casos `[x, y]` y `_`.
#   3. Para una lista, usa la recursión: `suma_de_textos([])` es `Ok(0)`;
#      para `[t, ..resto]`, toma el número de t y la suma de resto, los dos
#      con `?`.
#   4. `?` se puede usar con cualquier función que dé un Result, también
#      con las tuyas.
#
# Comprueba tu trabajo:  rythme exercise check 28-signo-de-interrogacion.ry
#                    o:  rythme test 28-signo-de-interrogacion.ry

fn sumar_textos(a, b) = fail("por hacer")

fn leer_punto(texto) = fail("por hacer")

fn suma_de_textos(textos) = fail("por hacer")

fn cociente_de_textos(a, b) = fail("por hacer")

# --- Pruebas (no las cambies) ---

test "sumar textos"
  assert_eq(sumar_textos("2", "40"), Ok(42))
  assert(is_err(sumar_textos("2", "cuarenta")), "cuarenta no es un número")
end

test "leer un punto"
  assert_eq(leer_punto("3,4"), Ok({x: 3, y: 4}))
  assert_eq(leer_punto("3"), Err("se esperaba x,y"))
  assert_eq(leer_punto("1,2,3"), Err("se esperaba x,y"))
  assert(is_err(leer_punto("3,cuatro")), "cuatro no es un número")
end

test "suma de textos"
  assert_eq(suma_de_textos(["1", "2", "3"]), Ok(6))
  assert_eq(suma_de_textos([]), Ok(0))
  assert_eq(suma_de_textos(["1", "x", "y"]), parse_int("x"))
end

test "cociente de textos"
  assert_eq(cociente_de_textos("9", "2"), Ok(4.5))
  assert_eq(cociente_de_textos("9", "0"), Err("división entre cero"))
  assert(is_err(cociente_de_textos("nueve", "0")), "nueve no es un número")
end
"#,
        ),
        ("29-word-counts", "29-contar-palabras"),
        (
            r#"# Exercise 29 · A small project: counting words  ★★★
#
# Everything together: texts, lists, records, pipelines and Options, for a
# program that analyses a text.
#
# Statement
#   - `words_of(text)`: the words of the text, in lowercase, without the
#     punctuation `.`, `,`, `!` and `?`;
#   - `frequencies(text)`: a list of `[word, count]` pairs, from the most
#     frequent word to the least frequent, and in alphabetical order for
#     words that appear the same number of times;
#   - `most_frequent(text)`: the most frequent word, as an Option (`None`
#     for a text without words);
#   - `report(text, n)`: the n most frequent words, one per line, each as
#     "word: count".
#
# Hints
#   1. `replace` four times (or `fold` over the list [".", ",", "!", "?"]),
#      then `lower` and `split`.
#   2. `group_by(words, fn(w) = w)` gives a record `{word: [word, word]}`,
#      and `entries(record)` the list of its `[name, value]` pairs.
#   3. `sort_by` is stable: sort by word first, then by count (the largest
#      first, with `fn([_, n]) = -n`).
#   4. `first(list)` gives an Option, and `map(option, f)` changes the value
#      inside.
#   5. `join(lines, "\n")` puts each element on its own line.
#
# Check your work:  rythme exercise check 29-word-counts.ry
#              or:  rythme test 29-word-counts.ry

fn words_of(text) = fail("to do")

fn frequencies(text) = fail("to do")

fn most_frequent(text) = fail("to do")

fn report(text, n) = fail("to do")

# --- Tests (do not change them) ---

poem = "The cat sat. The dog sat! Did the cat see the dog?"

test "words" = assert_eq(words_of("Hello, World! Hello?"), ["hello", "world", "hello"])

test "frequencies"
  assert_eq(frequencies("b a b c a b"), [["b", 3], ["a", 2], ["c", 1]])
  assert_eq(frequencies(poem) |> take(3), [["the", 4], ["cat", 2], ["dog", 2]])
end

test "most frequent"
  assert_eq(most_frequent(poem), Some("the"))
  assert_eq(most_frequent(" ... "), None)
end

test "report" = assert_eq(report(poem, 3), "the: 4\ncat: 2\ndog: 2")
"#,
            r#"# Ejercicio 29 · Un pequeño proyecto: contar palabras  ★★★
#
# Todo junto: textos, listas, registros, tuberías y Options, para un
# programa que analiza un texto.
#
# Enunciado
#   - `palabras_de(texto)`: las palabras del texto, en minúsculas, sin la
#     puntuación `.`, `,`, `!` y `?`;
#   - `frecuencias(texto)`: una lista de pares `[palabra, cantidad]`, de la
#     palabra más frecuente a la menos frecuente, y en orden alfabético
#     para las palabras que aparecen el mismo número de veces;
#   - `la_mas_frecuente(texto)`: la palabra más frecuente, como Option
#     (`None` para un texto sin palabras);
#   - `informe(texto, n)`: las n palabras más frecuentes, una por línea,
#     cada una como "palabra: cantidad".
#
# Pistas
#   1. `replace` cuatro veces (o `fold` sobre la lista
#      [".", ",", "!", "?"]), y luego `lower` y `split`.
#   2. `group_by(palabras, fn(p) = p)` da un registro
#      `{palabra: [palabra, palabra]}`, y `entries(registro)` la lista de
#      sus pares `[nombre, valor]`.
#   3. `sort_by` es estable: ordena primero por palabra, y luego por
#      cantidad (la mayor primero, con `fn([_, n]) = -n`).
#   4. `first(lista)` da un Option, y `map(opcion, f)` cambia el valor de
#      dentro.
#   5. `join(lineas, "\n")` pone cada elemento en su propia línea.
#
# Comprueba tu trabajo:  rythme exercise check 29-contar-palabras.ry
#                    o:  rythme test 29-contar-palabras.ry

fn palabras_de(texto) = fail("por hacer")

fn frecuencias(texto) = fail("por hacer")

fn la_mas_frecuente(texto) = fail("por hacer")

fn informe(texto, n) = fail("por hacer")

# --- Pruebas (no las cambies) ---

poema = "El gato vio al perro. El perro vio al gato! Y el gato? El gato duerme."

test "palabras" = assert_eq(palabras_de("Hola, Mundo! Hola?"), ["hola", "mundo", "hola"])

test "frecuencias"
  assert_eq(frecuencias("b a b c a b"), [["b", 3], ["a", 2], ["c", 1]])
  assert_eq(frecuencias(poema) |> take(3), [["el", 4], ["gato", 4], ["al", 2]])
end

test "la más frecuente"
  assert_eq(la_mas_frecuente(poema), Some("el"))
  assert_eq(la_mas_frecuente(" ... "), None)
end

test "informe" = assert_eq(informe(poema, 3), "el: 4\ngato: 4\nal: 2")
"#,
        ),
        ("30-testing", "30-pruebas"),
        (
            r#"# Exercise 30 · Tests find bugs  ★★★
#
# A test checks one piece of code: `test "name" = assert_eq(actual,
# expected)`. When it fails, `rythme test` shows the two values: the bug is
# somewhere between them.
#
# Statement
#   The functions below are already written, but each one has a bug.
#   1. Run the tests and read what fails.
#   2. Fix the functions (not the tests!) until every test passes.
#   3. Then write a test of your own for `clamp`, at the end of the file:
#      what should `clamp(5, 5, 5)` give?
#
#   - `clamp(x, low, high)`: x brought back between low and high;
#   - `median(list)`: the middle value of the sorted list, or the average of
#     the two middle values when the list has an even length;
#   - `count_words(text)`: how many words the text has.
#
# Hints
#   1. Read the `got:` and `expected:` lines of the first failure, and
#      compute by hand what the function does with these arguments.
#   2. `debug(value)` shows a value while the program runs, and gives it
#      back unchanged: `debug(sorted)`.
#   3. What does `split` without a separator do with several spaces?
#
# Check your work:  rythme exercise check 30-testing.ry
#              or:  rythme test 30-testing.ry

fn clamp(x, low, high) = max(high, min(x, low))

fn median(list)
  middle = len(list) // 2
  if is_even(len(list)) then (list[middle - 1] + list[middle]) / 2 else list[middle]
end

fn count_words(text) = len(split(text, " "))

# --- Tests (do not change them) ---

test "clamp"
  assert_eq(clamp(15, 0, 10), 10)
  assert_eq(clamp(-5, 0, 10), 0)
  assert_eq(clamp(5, 0, 10), 5)
end

test "median of an odd number of values"
  assert_eq(median([1, 2, 3]), 2)
  assert_eq(median([3, 1, 2]), 2)
end

test "median of an even number of values" = assert_eq(median([4, 1, 3, 2]), 2.5)

test "count words"
  assert_eq(count_words("one two three"), 3)
  assert_eq(count_words("  one   two "), 2)
end

# Your test:
"#,
            r#"# Ejercicio 30 · Las pruebas encuentran errores  ★★★
#
# Una prueba comprueba un trozo de código: `test "nombre" = assert_eq(obtenido,
# esperado)`. Cuando falla, `rythme test` muestra los dos valores: el error
# está en algún lugar entre ellos.
#
# Enunciado
#   Las funciones de abajo ya están escritas, pero cada una tiene un error.
#   1. Ejecuta las pruebas y lee lo que falla.
#   2. Corrige las funciones (¡no las pruebas!) hasta que pasen todas las
#      pruebas.
#   3. Después escribe una prueba propia para `limitar`, al final del
#      archivo: ¿qué debería dar `limitar(5, 5, 5)`?
#
#   - `limitar(x, bajo, alto)`: x devuelto entre bajo y alto;
#   - `mediana(lista)`: el valor del medio de la lista ordenada, o la media
#     de los dos valores del medio cuando la lista tiene una longitud par;
#   - `contar_palabras(texto)`: cuántas palabras tiene el texto.
#
# Pistas
#   1. Lee las líneas `obtenido:` y `esperado:` del primer fallo, y
#      calcula a mano lo que hace la función con esos argumentos.
#   2. `debug(valor)` muestra un valor mientras el programa se ejecuta, y
#      lo devuelve sin cambios: `debug(ordenada)`.
#   3. ¿Qué hace `split` sin separador con varios espacios?
#
# Comprueba tu trabajo:  rythme exercise check 30-pruebas.ry
#                    o:  rythme test 30-pruebas.ry

fn limitar(x, bajo, alto) = max(alto, min(x, bajo))

fn mediana(lista)
  medio = len(lista) // 2
  if is_even(len(lista)) then (lista[medio - 1] + lista[medio]) / 2 else lista[medio]
end

fn contar_palabras(texto) = len(split(texto, " "))

# --- Pruebas (no las cambies) ---

test "limitar"
  assert_eq(limitar(15, 0, 10), 10)
  assert_eq(limitar(-5, 0, 10), 0)
  assert_eq(limitar(5, 0, 10), 5)
end

test "mediana de un número impar de valores"
  assert_eq(mediana([1, 2, 3]), 2)
  assert_eq(mediana([3, 1, 2]), 2)
end

test "mediana de un número par de valores" = assert_eq(mediana([4, 1, 3, 2]), 2.5)

test "contar palabras"
  assert_eq(contar_palabras("uno dos tres"), 3)
  assert_eq(contar_palabras("  uno   dos "), 2)
end

# Tu prueba:
"#,
        ),
    ],
}
