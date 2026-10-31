# Enseñar con Rythme

*[English version →](teaching.md) · [Version française →](enseigner.md) · [Auf Deutsch →](teaching.de.md) · [Em português →](teaching.pt.md) · [中文 →](teaching.zh.md) · [In italiano →](teaching.it.md) · [日本語 →](teaching.ja.md)*

Rythme se diseñó para el aula: un lenguaje pequeño, con mensajes que
enseñan, para que los estudiantes dediquen su tiempo a las ideas y no a la
sintaxis. Esta guía cubre lo que necesita un curso: un plan, ejercicios
corregidos y calificados automáticamente, maneras de mostrar cómo se
ejecuta el código, y una política para los asistentes de IA.

## Por qué sirve para un curso

- **Poco que aprender antes de escribir programas de verdad.** 15 palabras
  clave, una sola manera de definir una función, una sola manera de elegir
  (`if`/`match`), sin clases, sin bucles, sin modificación. Toda la sintaxis
  cabe en una página (`rythme help syntax`).
- **El intérprete es un asistente de enseñanza.** Los errores señalan el
  lugar exacto, explican el problema y proponen una corrección. Las
  costumbres que vienen de otros lenguajes (`for`, `return`, `x += 1`,
  `null`...) reciben una explicación de la manera funcional de hacerlo.
  `rythme explain E0xx` da la versión larga con ejemplos. `rythme check`
  encuentra los problemas sin ejecutar el programa, incluidas las
  operaciones que fallarían siempre, como `"edad: " + 36`.
- **Pensamiento funcional desde el primer día.** Inmutabilidad, funciones
  como valores, recursión, patrones, tipos algebraicos, errores como valores
  (`Option`, `Result`): los conceptos de Haskell, OCaml, Elm, Elixir o Rust,
  sin su sintaxis ni un sistema de tipos contra el que luchar el primer
  día. Los tipos son opcionales: el verificador los encuentra solo, y
  cuando el curso llega ahí, los estudiantes los escriben
  (`fn area(w: Int, h: Int) -> Int`, `type Tree(a)`) y el verificador los
  comprueba.
- **Las pruebas vienen integradas.** `test "..." = assert_eq(obtenido, esperado)`
  junto al código, `rythme test archivo.ry` para ejecutarlas. Los
  ejercicios pueden venir con sus pruebas.
- **Una biblioteca que los estudiantes reconocen.** Textos, listas,
  diccionarios, fechas, CSV, expresiones regulares, y vectores,
  estadística y pendientes para una unidad sobre aprendizaje automático
  (`rythme help ml`). Quienes conocen Ruby o Python encuentran muchas
  funciones con los nombres que ya conocen (`upcase`, `strip`,
  `each_slice`...): ver `rythme help ruby` y
  [la biblioteca, comparada](stdlib-audit.md) (en inglés).
- **En el idioma de tus estudiantes**, y **fácil de instalar, gratuito,
  abierto**: un solo programa, sin dependencias, licencia MIT, para Linux,
  macOS y Windows, o nada que instalar con el playground del sitio web.

## Un curso posible (12 sesiones)

| # | Tema | Conceptos de Rythme | Ejercicios | Ejemplo |
|---|------|---------------------|------------|---------|
| 1 | Expresiones y valores | números, textos, `print`, `{...}` en los textos, el modo interactivo | 01, 03 | [`hello.ry`](../examples/hello.ry) |
| 2 | Funciones | `fn f(x) = ...`, bloques con `end`, llamadas, `help(f)` | 02 | |
| 3 | Elegir | `if ... then ... else`, booleanos, `and`/`or`/`not` | 04–06 | |
| 4 | Recursión | caso base, definiciones por casos `fn fib(0) = 0`, guardas | 07–10 | [`fibonacci.ry`](../examples/fibonacci.ry) |
| 5 | Listas | literales, rangos `1..10`, patrones `[x, ..rest]` | 11–13 | [`quicksort.ry`](../examples/quicksort.ry) |
| 6 | Funciones de orden superior | `map`, `filter`, `fold`, funciones anónimas, `\|>` | 14–19 | [`fizzbuzz.ry`](../examples/fizzbuzz.ry) |
| 7 | Registros | `{name: ...}`, acceso a los campos, `{..r, x: 1}` | 20–21 | [`students.ry`](../examples/students.ry) |
| 8 | Tus propios tipos | `type`, constructores, `match`, casos olvidados (W002), tipos escritos en el código | 22–23 | [`shapes.ry`](../examples/shapes.ry) |
| 9 | Tipos recursivos | árboles, recursión estructural, tipos con parámetros (`type Tree(a)`) | 24–25 | [`tree.ry`](../examples/tree.ry) |
| 10 | Los errores como valores | `Option`, `Result`, `?`, `unwrap_or` | 26–28 | [`results.ry`](../examples/results.ry) |
| 11 | Pruebas y módulos | `test`, `assert_eq`, `use` | 30 | [`testing.ry`](../examples/testing.ry) |
| 12 | Proyecto | procesamiento de texto, análisis de datos, un pequeño juego | 29 | [`words.ry`](../examples/words.ry) |

La recursión de cola (sesión 4 o 9) merece un momento: el error E014
("demasiadas llamadas anidadas") la explica, y `rythme explain E014`
muestra la técnica del acumulador.

Después de la sesión 7, dos o tres sesiones pueden formar una **unidad
sobre inteligencia artificial**: vectores, estadística, una neurona
artificial, el descenso de gradiente, los vecinos más cercanos y el
minimax (ejercicios 31 a 36, `rythme help ml`, y los ejemplos
[`perceptron.ry`](../examples/perceptron.ry) y [`knn.ry`](../examples/knn.ry)).

## Ejercicios con corrección automática

Da a los estudiantes un archivo con las pruebas ya escritas, y cada función
por escribir en la forma `fail("por hacer")`. Aquí hay uno que un
estudiante completó, con un error clásico:

```rythme
## Devuelve el elemento más grande de una lista no vacía.
fn largest(list) = fold(list, 0, max)

test "un elemento" = assert_eq(largest([7]), 7)
test "varios" = assert_eq(largest([3, 9, 2]), 9)
test "negativos" = assert_eq(largest([-3, -1, -2]), -1)
```

`rythme test largest.ry` indica lo que pasa y, para cada fallo, los
valores esperado y obtenido:

```text
Ejecutando 3 prueba(s)
  ✓ un elemento
  ✓ varios
  ✗ negativos
error[E015]: aserción fallida: los valores son distintos
 --> largest.ry:6:20
  |
6 | test "negativos" = assert_eq(largest([-3, -1, -2]), -1)
  |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: obtenido: 0
      esperado: -1
  = más sobre este error: `rythme explain E015`

3 prueba(s): 2 pasada(s), 1 fallida(s) (1 ms)
```

Hay treinta y seis ejercicios así listos: [**los ejercicios**](exercises/README.md)
(en inglés), graduados de ★☆☆ a ★★★ y que siguen el curso de arriba, cada
uno con un enunciado, pistas y pruebas, en cada idioma de los mensajes,
con soluciones de referencia para los docentes. Vienen integrados en
`rythme`:

```sh
rythme exercise list                     # los ejercicios
rythme exercise start 7                  # copia 07-recursion.ry en la carpeta actual
rythme exercise check 07-recursion.ry    # ejecuta sus pruebas, con consejos, y propone el siguiente
```

`rythme exercise check` también dice cuándo se cambiaron las pruebas de un
ejercicio integrado: el ejercicio solo está resuelto cuando pasan sus
propias pruebas. Tus propios ejercicios pueden seguir el mismo modelo (un
enunciado y pistas en comentarios, `fail("por hacer")` en las funciones,
las pruebas después de una línea `# --- Pruebas (no las cambies) ---`);
`rythme exercise check` funciona con cualquier archivo que tenga pruebas.

## Calificar con `--format json`

`rythme test` acepta una carpeta: con un archivo (o una carpeta) por
estudiante, un solo comando ejecuta todas las pruebas de la clase.

```sh
rythme test --sandbox --format json submissions/ > results.json
```

- **`--format json`** imprime un solo documento JSON en lugar de los
  mensajes para personas: para cada archivo, `tests`, `failed` y `broken`
  (un archivo que no compila, o se detiene antes de sus pruebas); para cada
  prueba, su `name`, su `file`, `passed` y `failure`; y luego un `summary`.
  Las claves nunca cambian ([el esquema](rythme-output.schema.json)); los
  mensajes siguen a `--lang`.
- **`--sandbox`** mantiene los programas lejos de los archivos, las
  carpetas, las variables de entorno y la red. No hay límite de tiempo:
  ejecuta el comando con uno (en Linux, `timeout 60 rythme test ...`).
- **Los estudiantes pueden cambiar las pruebas** de su archivo. Para una
  nota, ejecuta tu propia copia: `rythme exercise check` dice cuándo se
  cambiaron las pruebas de un ejercicio integrado, y las soluciones de
  referencia son públicas (en el repositorio): para un trabajo calificado,
  escribe tus propios ejercicios.
- `rythme check --format json` añade las advertencias, cada una con su
  código estable (W001 un nombre sin usar, W002 un caso olvidado...), y
  `rythme fmt --check` dice si el código tiene el formato oficial.

Rythme lee JSON por sí mismo. Este programa convierte los resultados en
notas:

```rythme
# grades.ry
fn grade(f) = if f.broken then "{f.file}: no se ejecuta" else "{f.file}: {f.tests - f.failed}/{f.tests}"

results = read_file(args()[0]) |> unwrap |> parse_json |> unwrap
each(results.files, fn(f) = print(grade(f)))
```

`rythme grades.ry results.json` imprime:

```text
submissions/ada.ry: 3/3
submissions/alan.ry: 2/3
submissions/grace.ry: no se ejecuta
```

## Mostrar cómo se ejecuta el código

`rythme steps` muestra un cálculo paso a paso, en el modelo de
sustitución: cada llamada se reemplaza por el cuerpo de su función, hasta
que solo queda el resultado. La recursión, los acumuladores y las llamadas
de cola se vuelven visibles.

```rythme
fn fib(n) = if n < 2 then n else fib(n - 1) + fib(n - 2)

fib(4)
```

`rythme steps fib.ry` muestra:

```text
fib(4)
  → fib(3) + fib(2)
  → fib(2) + fib(1) + fib(2)
  → fib(1) + fib(0) + fib(1) + fib(2)
  → 1 + fib(0) + fib(1) + fib(2)
  → 1 + 0 + fib(1) + fib(2)
  → 1 + fib(1) + fib(2)
  → 1 + 1 + fib(2)
  → 2 + fib(2)
  → 2 + (fib(1) + fib(0))
  → 2 + (1 + fib(0))
  → 2 + (1 + 0)
  → 2 + 1
  → 3
```

`rythme steps fib.ry "fib(6)"` muestra otra expresión, `:steps fib(6)` lo
hace en el modo interactivo, y el botón **Pasos** en el playground, donde
*Compartir* crea un enlace que se abre en el mismo paso: un enlace para
proyectar, o para enviar a la clase.

`rythme debug archivo.ry` pausa un programa donde lo pidas (`break 12`,
`break fib`) y muestra los valores de sus nombres. `where` lista las
llamadas en curso con sus argumentos, y dice cuántas llamadas de la misma
función están esperando: la pila de una recursión, en directo. Una llamada
en posición final dice que ocupó el lugar de quien la llamó. En VS Code,
`F5` sobre un archivo `.ry` inicia el mismo depurador, con los puntos de
interrupción en el margen. El
[manual](reference.md#79-the-debugger-rythme-debug) (en inglés) muestra una
sesión completa.

## Sin instalar nada: el playground

El playground del [sitio web](https://maxlestage.github.io/Rythme/es/)
ejecuta el intérprete real en el navegador, con los mismos mensajes y la
misma ayuda: los estudiantes pueden empezar en cualquier ordenador,
tableta o teléfono.

- **Ejecutar**, **Comprobar**, **Probar**, **Formatear** y **Pasos**
  funcionan como los comandos; pega un ejercicio y pulsa **Probar**.
- **Compartir** crea un enlace que contiene el programa mismo (nada se
  envía a un servidor): reparte un punto de partida, o el programa de la
  clase. **Semilla** da a cada estudiante los mismos sorteos de `random`.
- Instalado como aplicación (**Instalar la app** en el menú, o *Añadir a
  pantalla de inicio* en iPhone y iPad), el sitio y su playground
  funcionan sin conexión, lo que ayuda en un aula con mala red. El código
  que se está escribiendo se queda en el navegador.
- En el navegador no se pueden leer ni escribir archivos, `input()`
  devuelve un texto vacío y las llamadas anidadas se detienen en 600 (las
  llamadas de cola no tienen límite).

## En el idioma de tus estudiantes

- Cada mensaje, tema de ayuda, explicación y página de `rythme learn`
  existe en inglés, francés, español, alemán, portugués, chino, italiano y
  japonés. Rythme
  sigue el idioma del sistema; `--lang es` o la variable `RYTHME_LANG=es`
  eligen otro.
- Las palabras clave, los nombres de las funciones y los comandos son los
  mismos en todos los idiomas: un programa escrito en una clase funciona
  en todas partes, y `rythme help` acepta los nombres de los temas en cada
  idioma (`rythme help listas`).
- Los ejercicios también siguen el idioma:
  `rythme --lang de exercise start 7` escribe `07-rekursion.ry`, en alemán.
- El sitio web, su playground y esta guía existen en los mismos idiomas.

## Los asistentes de IA en clase

Los estudiantes tienen asistentes a mano. Rythme no intenta detectarlos;
esto es lo que ofrece, y una política que un curso puede adaptar.

**Lo que ofrece Rythme.** Los asistentes se entrenaron antes de que Rythme
existiera: solos, escriben costumbres de Python o JavaScript (`for`,
`return`, `x += 1`), que Rythme rechaza con un mensaje que dice qué
escribir. Con `rythme help llms`, todo el lenguaje en un texto (también en
la página *IA* del sitio web), escriben Rythme correcto. `rythme mcp`
permite a un asistente ejecutar, comprobar y probar código por sí mismo,
en un entorno aislado. En el playground, *Copiar para IA* copia el
programa y su salida o sus errores, listos para una conversación.
`rythme fix` aplica las correcciones seguras (`&&` → `and`,
`let x = 3` → `x = 3`), y `rythme fix --dry-run` solo las muestra.

**Una política posible.**

- **Primeras semanas: los estudiantes escriben el código.** Se trata de
  construir los reflejos, y los mensajes y `rythme explain` ya explican
  cada error. Un asistente puede explicar un mensaje o una idea, no
  escribir la solución.
- **`rythme fix` una vez leído el mensaje**: solo cambia lo que es seguro,
  la misma corrección que propone el mensaje.
- **Proyectos: permitido, y dicho.** Da `rythme help llms` al asistente,
  pide a los estudiantes que digan para qué lo usaron, y que expliquen su
  código.
- **Evaluar la comprensión, no solo los resultados.** Las pruebas
  comprueban que un programa funciona, no quién lo escribió. Añade
  comprobaciones cortas en clase: predecir los pasos de `rythme steps`, la
  salida de un programa, o la corrección de un mensaje, sin ordenador; o
  un ejercicio hecho en el momento en el playground.
- **Para tu propio trabajo**, un asistente al que se le dio
  `rythme help llms` puede preparar borradores de ejercicios nuevos en el
  formato de arriba. Comprueba cada uno con una solución propia y
  `rythme test` antes de repartirlo.

## Consejos prácticos

- Empieza con el modo interactivo (`rythme`): cada línea muestra su valor.
  `:doc map` muestra la documentación de una función, Tab completa los
  nombres.
- `rythme learn` es una visita guiada que los estudiantes pueden seguir
  solos.
- `rythme help builtins` lista todas las funciones integradas por
  categoría, y cada comando responde a `--help` (`rythme test --help`).
  `rythme completions bash` (o `zsh`, `fish`, `powershell`) completa los
  comandos en la terminal.
- Instala la [extensión de VS Code](../editors/vscode): colores, fragmentos
  de código, los problemas mostrados mientras se escribe con sus
  correcciones rápidas, la documentación bajo el ratón, y el depurador.
  Otros editores también pueden usar el servidor de lenguaje
  ([`editors/README.md`](../editors/README.md), en inglés).
- `rythme fmt` da a todos los archivos el mismo formato: los estudiantes
  pueden ejecutarlo (o `rythme fmt --check .`) antes de entregar su
  trabajo.
- Para un proyecto más grande, `rythme new NOMBRE` crea una carpeta con
  `src/`, `tests/` y `rythme.toml`; `rythme test` solo ejecuta entonces
  todas las pruebas del proyecto, y `rythme add` añade un paquete del
  [índice de paquetes](reference.md#711-the-package-index) (en inglés).
