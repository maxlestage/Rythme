# Ensinar com Rythme

*[English version →](teaching.md) · [Version française →](enseigner.md) · [En español →](teaching.es.md) · [Auf Deutsch →](teaching.de.md) · [中文 →](teaching.zh.md) · [In italiano →](teaching.it.md) · [日本語 →](teaching.ja.md)*

O Rythme foi pensado para a sala de aula: uma linguagem pequena, com
mensagens que ensinam, para que os alunos passem o tempo com as ideias e
não com a sintaxe. Este guia cobre o que um curso precisa: um plano,
exercícios corrigidos e avaliados automaticamente, maneiras de mostrar
como o código roda, e uma política para os assistentes de IA.

## Por que ele serve para um curso

- **Pouco a aprender antes de escrever programas de verdade.** 15
  palavras-chave, uma só maneira de definir uma função, uma só maneira de
  escolher (`if`/`match`), sem classes, sem laços, sem modificação. Toda a
  sintaxe cabe em uma página (`rythme help syntax`).
- **O interpretador é um assistente de ensino.** Os erros apontam o lugar
  exato, explicam o problema e propõem uma correção. Os hábitos vindos de
  outras linguagens (`for`, `return`, `x += 1`, `null`...) recebem uma
  explicação do jeito funcional. `rythme explain E0xx` dá a versão longa
  com exemplos. `rythme check` encontra os problemas sem rodar o programa,
  inclusive as operações que sempre falhariam, como `"idade: " + 36`.
- **Pensamento funcional desde o primeiro dia.** Imutabilidade, funções
  como valores, recursão, padrões, tipos algébricos, erros como valores
  (`Option`, `Result`): os conceitos de Haskell, OCaml, Elm, Elixir ou
  Rust, sem a sintaxe deles nem um sistema de tipos para combater no
  primeiro dia. Os tipos são opcionais: o verificador os encontra sozinho,
  e quando o curso chega lá, os alunos os escrevem
  (`fn area(w: Int, h: Int) -> Int`, `type Tree(a)`) e o verificador os
  confere.
- **Os testes são integrados.** `test "..." = assert_eq(obtido, esperado)`
  ao lado do código, `rythme test arquivo.ry` para rodá-los. Os exercícios
  podem vir com os seus testes.
- **Uma biblioteca que os alunos reconhecem.** Textos, listas,
  dicionários, datas, CSV, expressões regulares, e vetores, estatística e
  inclinações para uma unidade sobre aprendizado de máquina
  (`rythme help ml`). Quem conhece Ruby ou Python encontra muitas funções
  com os nomes que já conhece (`upcase`, `strip`, `each_slice`...): veja
  `rythme help ruby` e [a biblioteca, comparada](stdlib-audit.md) (em
  inglês).
- **No idioma dos seus alunos**, e **fácil de instalar, gratuito,
  aberto**: um só programa, nenhuma dependência, licença MIT, para Linux,
  macOS e Windows, ou nada para instalar com o playground do site.

## Um curso possível (12 aulas)

| # | Tema | Conceitos de Rythme | Exercícios | Exemplo |
|---|------|---------------------|------------|---------|
| 1 | Expressões e valores | números, textos, `print`, `{...}` nos textos, o modo interativo | 01, 03 | [`hello.ry`](../examples/hello.ry) |
| 2 | Funções | `fn f(x) = ...`, blocos com `end`, chamadas, `help(f)` | 02 | |
| 3 | Escolher | `if ... then ... else`, booleanos, `and`/`or`/`not` | 04–06 | |
| 4 | Recursão | caso base, definições por casos `fn fib(0) = 0`, guardas | 07–10 | [`fibonacci.ry`](../examples/fibonacci.ry) |
| 5 | Listas | literais, intervalos `1..10`, padrões `[x, ..rest]` | 11–13 | [`quicksort.ry`](../examples/quicksort.ry) |
| 6 | Funções de ordem superior | `map`, `filter`, `fold`, funções anônimas, `\|>` | 14–19 | [`fizzbuzz.ry`](../examples/fizzbuzz.ry) |
| 7 | Registros | `{name: ...}`, acesso aos campos, `{..r, x: 1}` | 20–21 | [`students.ry`](../examples/students.ry) |
| 8 | Seus próprios tipos | `type`, construtores, `match`, casos esquecidos (W002), tipos escritos no código | 22–23 | [`shapes.ry`](../examples/shapes.ry) |
| 9 | Tipos recursivos | árvores, recursão estrutural, tipos com parâmetros (`type Tree(a)`) | 24–25 | [`tree.ry`](../examples/tree.ry) |
| 10 | Os erros como valores | `Option`, `Result`, `?`, `unwrap_or` | 26–28 | [`results.ry`](../examples/results.ry) |
| 11 | Testes e módulos | `test`, `assert_eq`, `use` | 30 | [`testing.ry`](../examples/testing.ry) |
| 12 | Projeto | processamento de texto, análise de dados, um pequeno jogo | 29 | [`words.ry`](../examples/words.ry) |

A recursão de cauda (aula 4 ou 9) merece um momento: o erro E014 ("chamadas
aninhadas demais") a explica, e `rythme explain E014` mostra a técnica do
acumulador.

Depois da aula 7, duas ou três aulas podem formar uma **unidade sobre
inteligência artificial**: vetores, estatística, um neurônio artificial,
a descida do gradiente, os vizinhos mais próximos e o minimax (exercícios
31 a 36, `rythme help ml`, e os exemplos
[`perceptron.ry`](../examples/perceptron.ry) e [`knn.ry`](../examples/knn.ry)).

## Exercícios com correção automática

Dê aos alunos um arquivo com os testes já escritos, e cada função a
escrever na forma `fail("a fazer")`. Aqui está um que um aluno completou,
com um erro clássico:

```rythme
## Devolve o maior elemento de uma lista não vazia.
fn largest(list) = fold(list, 0, max)

test "um elemento" = assert_eq(largest([7]), 7)
test "vários" = assert_eq(largest([3, 9, 2]), 9)
test "negativos" = assert_eq(largest([-3, -1, -2]), -1)
```

`rythme test largest.ry` mostra o que passa e, para cada falha, os valores
esperado e obtido:

```text
Executando 3 teste(s)
  ✓ um elemento
  ✓ vários
  ✗ negativos
erro[E015]: asserção falhou: os valores são diferentes
 --> largest.ry:6:20
  |
6 | test "negativos" = assert_eq(largest([-3, -1, -2]), -1)
  |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: obtido:   0
      esperado: -1
  = mais sobre este erro: `rythme explain E015`

3 teste(s): 2 passaram, 1 falharam (1 ms)
```

Trinta e seis exercícios assim estão prontos: [**os exercícios**](exercises/README.md)
(em inglês), graduados de ★☆☆ a ★★★ e que seguem o curso acima, cada um
com um enunciado, dicas e testes, em cada idioma das mensagens, com
soluções de referência para os professores. Eles vêm integrados ao
`rythme`:

```sh
rythme exercise list                     # os exercícios
rythme exercise start 7                  # copia 07-recursao.ry para a pasta atual
rythme exercise check 07-recursao.ry     # roda os testes dele, com conselhos, e propõe o próximo
```

`rythme exercise check` também diz quando os testes de um exercício
integrado foram alterados: o exercício só está resolvido quando os seus
próprios testes passam. Os seus próprios exercícios podem seguir o mesmo
modelo (um enunciado e dicas em comentários, `fail("a fazer")` nas
funções, os testes depois de uma linha
`# --- Testes (não os altere) ---`); `rythme exercise check` funciona com
qualquer arquivo que tenha testes.

## Avaliar com `--format json`

`rythme test` aceita uma pasta: com um arquivo (ou uma pasta) por aluno, um
só comando roda todos os testes da turma.

```sh
rythme test --sandbox --format json submissions/ > results.json
```

- **`--format json`** imprime um só documento JSON no lugar das mensagens
  para pessoas: para cada arquivo, `tests`, `failed` e `broken` (um arquivo
  que não compila, ou para antes dos seus testes); para cada teste, o seu
  `name`, o seu `file`, `passed` e `failure`; depois um `summary`. As
  chaves nunca mudam ([o esquema](rythme-output.schema.json)); as
  mensagens seguem `--lang`.
- **`--sandbox`** mantém os programas longe dos arquivos, das pastas, das
  variáveis de ambiente e da rede. Não há limite de tempo: rode o comando
  com um (no Linux, `timeout 60 rythme test ...`).
- **Os alunos podem alterar os testes** do seu arquivo. Para uma nota, use
  a sua própria cópia: `rythme exercise check` diz quando os testes de um
  exercício integrado foram alterados, e as soluções de referência são
  públicas (no repositório): para um trabalho avaliado, escreva os seus
  próprios exercícios.
- `rythme check --format json` acrescenta os avisos, cada um com o seu
  código estável (W001 um nome não usado, W002 um caso esquecido...), e
  `rythme fmt --check` diz se o código tem a formatação oficial.

O Rythme lê JSON sozinho. Este programa transforma os resultados em notas:

```rythme
# grades.ry
fn grade(f) = if f.broken then "{f.file}: não roda" else "{f.file}: {f.tests - f.failed}/{f.tests}"

results = read_file(args()[0]) |> unwrap |> parse_json |> unwrap
each(results.files, fn(f) = print(grade(f)))
```

`rythme grades.ry results.json` imprime:

```text
submissions/ada.ry: 3/3
submissions/alan.ry: 2/3
submissions/grace.ry: não roda
```

## Mostrar como o código roda

`rythme steps` mostra um cálculo passo a passo, no modelo de substituição:
cada chamada é substituída pelo corpo da sua função, até restar só o
resultado. A recursão, os acumuladores e as chamadas de cauda ficam
visíveis.

```rythme
fn fib(n) = if n < 2 then n else fib(n - 1) + fib(n - 2)

fib(4)
```

`rythme steps fib.ry` mostra:

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

`rythme steps fib.ry "fib(6)"` mostra outra expressão, `:steps fib(6)` faz
o mesmo no modo interativo, e o botão **Passos** no playground, onde
*Compartilhar* cria um link que abre no mesmo passo: um link para projetar,
ou para enviar à turma.

`rythme debug arquivo.ry` pausa um programa onde você pedir (`break 12`,
`break fib`) e mostra os valores dos seus nomes. `where` lista as chamadas
em andamento com os seus argumentos, e diz quantas chamadas da mesma
função estão esperando: a pilha de uma recursão, ao vivo. Uma chamada em
posição final diz que tomou o lugar de quem a chamou. No VS Code, `F5` em
um arquivo `.ry` inicia o mesmo depurador, com os pontos de parada na
margem. O [manual](reference.md#79-the-debugger-rythme-debug) (em inglês)
mostra uma sessão completa.

## Sem instalar nada: o playground

O playground do [site](https://maxlestage.github.io/Rythme/pt/) roda o
interpretador de verdade no navegador, com as mesmas mensagens e a mesma
ajuda: os alunos podem começar em qualquer computador, tablet ou celular.

- **Rodar**, **Verificar**, **Testar**, **Formatar** e **Passos**
  funcionam como os comandos; cole um exercício e aperte **Testar**.
- **Compartilhar** cria um link que contém o próprio programa (nada é
  enviado a um servidor): distribua um ponto de partida, ou o programa da
  aula. **Semente** dá a cada aluno os mesmos sorteios de `random`.
- Instalado como aplicativo (**Instalar o app** no menu, ou *Adicionar à
  Tela de Início* no iPhone e no iPad), o site e o seu playground
  funcionam sem conexão, o que ajuda numa sala com rede ruim. O código que
  está sendo escrito fica no navegador.
- No navegador não é possível ler nem escrever arquivos, `input()` devolve
  um texto vazio, e as chamadas aninhadas param em 600 (chamadas de cauda
  não têm limite).

## No idioma dos seus alunos

- Cada mensagem, tópico de ajuda, explicação e página de `rythme learn`
  existe em inglês, francês, espanhol, alemão, português, chinês, italiano e
  japonês. O
  Rythme segue o idioma do sistema; `--lang es` ou a variável
  `RYTHME_LANG=es` escolhem outro.
- As palavras-chave, os nomes das funções e os comandos são os mesmos em
  todos os idiomas: um programa escrito numa turma roda em qualquer lugar,
  e `rythme help` aceita os nomes dos tópicos em cada idioma
  (`rythme help listas`).
- Os exercícios também seguem o idioma:
  `rythme --lang de exercise start 7` escreve `07-rekursion.ry`, em alemão.
- O site, o seu playground e este guia existem nos mesmos idiomas.

## Os assistentes de IA em aula

Os alunos têm assistentes à mão. O Rythme não tenta detectá-los; eis o que
ele oferece, e uma política que um curso pode adaptar.

**O que o Rythme oferece.** Os assistentes foram treinados antes de o
Rythme existir: sozinhos, escrevem hábitos de Python ou JavaScript (`for`,
`return`, `x += 1`), que o Rythme recusa com uma mensagem que diz o que
escrever. Com `rythme help llms`, a linguagem inteira em um texto (também
na página *IA* do site), eles escrevem Rythme correto. `rythme mcp` deixa
um assistente rodar, verificar e testar código sozinho, numa sandbox. No
playground, *Copiar para IA* copia o programa e a sua saída ou os seus
erros, prontos para uma conversa. `rythme fix` aplica as correções seguras
(`&&` → `and`, `let x = 3` → `x = 3`), e `rythme fix --dry-run` apenas as
mostra.

**Uma política possível.**

- **Primeiras semanas: os alunos escrevem o código.** O objetivo é
  construir os reflexos, e as mensagens e `rythme explain` já explicam
  cada erro. Um assistente pode explicar uma mensagem ou uma ideia, não
  escrever a solução.
- **`rythme fix` depois de ler a mensagem**: ele só muda o que é seguro, a
  mesma correção que a mensagem propõe.
- **Projetos: permitido, e declarado.** Dê `rythme help llms` ao
  assistente, peça aos alunos que digam para que o usaram, e que
  expliquem o seu código.
- **Avaliar a compreensão, não só os resultados.** Os testes verificam que
  um programa funciona, não quem o escreveu. Acrescente verificações
  curtas em aula: prever os passos de `rythme steps`, a saída de um
  programa, ou a correção de uma mensagem, sem computador; ou um exercício
  feito na hora no playground.
- **Para o seu próprio trabalho**, um assistente que recebeu
  `rythme help llms` pode rascunhar exercícios novos no formato acima.
  Confira cada um com uma solução sua e `rythme test` antes de
  distribuí-lo.

## Dicas práticas

- Comece pelo modo interativo (`rythme`): cada linha mostra o seu valor.
  `:doc map` mostra a documentação de uma função, Tab completa os nomes.
- `rythme learn` é uma visita guiada que os alunos podem seguir sozinhos.
- `rythme help builtins` lista todas as funções integradas por categoria,
  e cada comando responde a `--help` (`rythme test --help`).
  `rythme completions bash` (ou `zsh`, `fish`, `powershell`) completa os
  comandos no terminal.
- Instale a [extensão do VS Code](../editors/vscode): cores, snippets, os
  problemas mostrados durante a digitação com as suas correções rápidas, a
  documentação sob o mouse, e o depurador. Outros editores também podem
  usar o servidor de linguagem
  ([`editors/README.md`](../editors/README.md), em inglês).
- `rythme fmt` dá a todos os arquivos a mesma formatação: os alunos podem
  rodá-lo (ou `rythme fmt --check .`) antes de entregar o trabalho.
- Para um projeto maior, `rythme new NOME` cria uma pasta com `src/`,
  `tests/` e `rythme.toml`; `rythme test` sozinho roda então todos os
  testes do projeto, e `rythme add` traz um pacote do
  [índice de pacotes](reference.md#711-the-package-index) (em inglês).
