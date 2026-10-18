# Rythme

**Más sencillo que Python y Ruby, tan agradable como Ruby: un lenguaje de
programación puramente funcional, con ayuda en todas partes.**
Los programas son archivos que terminan en `.ry`. El intérprete está escrito en Rust.

*[English version →](README.md) · [Version française →](README.fr.md) · [Auf Deutsch →](README.de.md) · [Em português →](README.pt.md) · [中文 →](README.zh.md)*

Este README es un resumen. Los mensajes, la ayuda y las explicaciones de
Rythme existen en español (`--lang es`); la documentación completa está en
inglés y en francés: el [manual de referencia](docs/reference.md) y la
[guía para docentes](docs/teaching.md).

```rythme
# Los valores nunca cambian. Todo es una función. No hay bucles.
fn cuadrado(x) = x * x

[1, 2, 3, 4]
  |> filter(is_even)      # quedarse con los números pares
  |> map(cuadrado)        # elevar cada uno al cuadrado
  |> sum                  # sumarlos
  |> print                # 20
```

## ¿Por qué Rythme?

- **Más sencillo que Python o Ruby.** 15 palabras clave. Una forma de hacer
  cada cosa. Sin clases, sin `null`, sin excepciones, sin bucles, sin
  mutación: menos conceptos que aprender, menos fallos que buscar.
- **Tan agradable como Ruby.** Bloques (`xs.map do |x| x * 2 end`), cadenas de
  métodos, llamadas sin paréntesis (`puts "hola"`), nombres como `even?`,
  `unless`, símbolos (`:ok`) y los nombres de Ruby de la biblioteca.
- **Seguro como Rust.** Un valor que puede faltar es una `Option` (`Some(x)` o
  `None`); una operación que puede fallar devuelve un `Result` (`Ok(x)` o
  `Err(e)`), y `?` pasa los errores hacia arriba. El programa se comprueba
  **antes** de ejecutarse.
- **Ayuda en todas partes.** Cada error dice *dónde* está, *por qué*, y *cómo
  corregirlo*. `rythme explain E003` explica cada código de error,
  `rythme help` y `rythme learn` (una visita guiada) enseñan el lenguaje, y el
  servidor de lenguaje (`rythme lsp`) trae la misma ayuda a tu editor.

Así se ve un error, en español:

```
error[E005]: no se puede usar `+` con Text y Int
 --> informe.ry:2:16
  |
2 | print("edad: " + 36)
  |                ^ Text "edad: " + Int 36
  |
  = ayuda: para construir un texto con un valor dentro, usa llaves: "total: {x}"
      o convierte antes el valor: text(x)
  = más sobre este error: `rythme explain E005`
```

```
error[E001]: `for` no se usa en Rythme
 --> bucle.ry:1:1
  |
1 | for x in numeros
  | ^^^ viene de otro lenguaje
  |
  = ayuda: Rythme no tiene bucles. Trabaja con la lista entera en su lugar:
        1..5 |> each(fn(i) = print(i))
        [1, 2, 3] |> map(fn(x) = x * 2)
      o escribe una función recursiva. Consulta `rythme help functional`.
```

## Instalar

**Linux y macOS:**

```sh
curl -fsSL https://raw.githubusercontent.com/maxlestage/Rythme/master/install.sh | sh
```

**Windows** (PowerShell):

```powershell
irm https://raw.githubusercontent.com/maxlestage/Rythme/master/install.ps1 | iex
```

**Con [Rust](https://rustup.rs):** `cargo install --git https://github.com/maxlestage/Rythme`.
Más opciones en el [README en inglés](README.md#install).

## Usar

| Comando                     | Qué hace                                         |
|-----------------------------|--------------------------------------------------|
| `rythme`                    | modo interactivo: escribe código, mira el resultado |
| `rythme hola.ry`            | ejecutar un programa                             |
| `rythme check hola.ry`      | encontrar los problemas sin ejecutar el programa |
| `rythme test hola.ry`       | ejecutar las pruebas escritas en el archivo      |
| `rythme fmt hola.ry`        | dar al código su formato oficial                 |
| `rythme new hola`           | un proyecto nuevo: `rythme.toml`, `src/`, `tests/` |
| `rythme help [tema]`        | ayuda sobre un tema o una función (`rythme help listas`) |
| `rythme explain E003`       | explicar un código de error                      |
| `rythme learn`              | visita guiada del lenguaje                       |
| `rythme steps fib.ry "fib(4)"` | el cálculo paso a paso                        |

Los mensajes están en español con `--lang es` antes del comando, con
`RYTHME_LANG=es`, o automáticamente cuando el idioma del sistema es el español
(`LANG=es_ES.UTF-8`). Las palabras clave, los nombres de las funciones
integradas y los comandos son los mismos en todos los idiomas. En el modo
interactivo, `:lang es` cambia el idioma.

## Licencia

[MIT](LICENSE).
