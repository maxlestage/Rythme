# Rythme

**Mais simples que Python e Ruby, tão agradável quanto Ruby: uma linguagem de
programação puramente funcional, com ajuda em todo lugar.**
Os programas são arquivos que terminam em `.ry`. O interpretador é escrito em Rust.

*[English version →](README.md) · [Version française →](README.fr.md) · [En español →](README.es.md) · [Auf Deutsch →](README.de.md) · [中文 →](README.zh.md)*

Este README é um resumo. As mensagens, a ajuda e as explicações do Rythme
existem em português (`--lang pt`); a documentação completa está em inglês e
em francês: o [manual de referência](docs/reference.md) e o
[guia para professores](docs/teaching.md).

```rythme
# Os valores nunca mudam. Tudo é uma função. Não há laços.
fn quadrado(x) = x * x

[1, 2, 3, 4]
  |> filter(is_even)      # ficar com os números pares
  |> map(quadrado)        # elevar cada um ao quadrado
  |> sum                  # somá-los
  |> print                # 20
```

## Por que Rythme?

- **Mais simples que Python ou Ruby.** 15 palavras-chave. Um jeito de fazer
  cada coisa. Sem classes, sem `null`, sem exceções, sem laços, sem mutação:
  menos conceitos para aprender, menos bugs para procurar.
- **Tão agradável quanto Ruby.** Blocos (`xs.map do |x| x * 2 end`), cadeias de
  métodos, chamadas sem parênteses (`puts "olá"`), nomes como `even?`,
  `unless`, símbolos (`:ok`) e os nomes Ruby da biblioteca.
- **Seguro como Rust.** Um valor que pode faltar é uma `Option` (`Some(x)` ou
  `None`); uma operação que pode falhar devolve um `Result` (`Ok(x)` ou
  `Err(e)`), e `?` repassa os erros para cima. O programa é verificado
  **antes** de rodar.
- **Ajuda em todo lugar.** Cada erro diz *onde* está, *por quê*, e *como*
  corrigi-lo. `rythme explain E003` explica cada código de erro,
  `rythme help` e `rythme learn` (uma visita guiada) ensinam a linguagem, e o
  servidor de linguagem (`rythme lsp`) leva a mesma ajuda para o seu editor.

É assim que um erro aparece, em português:

```
erro[E005]: não é possível usar `+` com Text e Int
 --> relatorio.ry:2:17
  |
2 | print("idade: " + idade)
  |                 ^ Text "idade: " + Int
  |
  = ajuda: para montar um texto com um valor dentro, use chaves: "total: {x}"
      ou converta o valor primeiro: text(x)
  = mais sobre este erro: `rythme explain E005`
```

```
erro[E001]: `for` não é usado no Rythme
 --> laco.ry:1:1
  |
1 | for x in numeros
  | ^^^ vem de outra linguagem
  |
  = ajuda: o Rythme não tem laços. Trabalhe com a lista inteira em vez disso:
        1..5 |> each(fn(i) = print(i))
        [1, 2, 3] |> map(fn(x) = x * 2)
      ou escreva uma função recursiva. Veja `rythme help functional`.
  = mais sobre este erro: `rythme explain E001`
```

## Instalar

**Linux e macOS:**

```sh
curl -fsSL https://raw.githubusercontent.com/maxlestage/Rythme/master/install.sh | sh
```

**Windows** (PowerShell):

```powershell
irm https://raw.githubusercontent.com/maxlestage/Rythme/master/install.ps1 | iex
```

**Com [Rust](https://rustup.rs):** `cargo install --git https://github.com/maxlestage/Rythme`.
Mais opções no [README em inglês](README.md#install).

## Usar

| Comando                     | O que faz                                        |
|-----------------------------|--------------------------------------------------|
| `rythme`                    | modo interativo: digite código, veja o resultado |
| `rythme ola.ry`             | executar um programa                             |
| `rythme check ola.ry`       | encontrar os problemas sem executar o programa   |
| `rythme test ola.ry`        | executar os testes escritos no arquivo           |
| `rythme fmt ola.ry`         | dar ao código o seu layout oficial               |
| `rythme new ola`            | um projeto novo: `rythme.toml`, `src/`, `tests/` |
| `rythme help [tema]`        | ajuda sobre um tema ou uma função (`rythme help listas`) |
| `rythme explain E003`       | explicar um código de erro                       |
| `rythme learn`              | visita guiada pela linguagem                     |
| `rythme steps fib.ry "fib(4)"` | o cálculo passo a passo                       |

As mensagens ficam em português com `--lang pt` antes do comando, com
`RYTHME_LANG=pt`, ou automaticamente quando o idioma do sistema é o português
(`LANG=pt_BR.UTF-8`). As palavras-chave, os nomes das funções embutidas e os
comandos são os mesmos em todos os idiomas. No modo interativo, `:lang pt`
muda o idioma.

## Licença

[MIT](LICENSE).
