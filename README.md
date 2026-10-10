![Static Badge](https://img.shields.io/badge/Free-Palestine-4B0082)
![Static Badge](https://img.shields.io/badge/Anti--Imperialism-710C04)

```
  █████████  ███████████ ███████████   ██████████
 ███░░░░░███░█░░░███░░░█░░███░░░░░███ ░░███░░░░░█
░███    ░░░ ░   ░███  ░  ░███    ░███  ░███  █ ░ 
░░█████████     ░███     ░██████████   ░██████   
 ░░░░░░░░███    ░███     ░███░░░░░███  ░███░░█   
 ███    ░███    ░███     ░███    ░███  ░███ ░   █
░░█████████     █████    █████   █████ ██████████
 ░░░░░░░░░     ░░░░░    ░░░░░   ░░░░░ ░░░░░░░░░░ 
```                                              
                                                 
                                                 

Trabalho  de **Estrutura de Dados** (ECO – Engenharia de Computação), Prof. Lucas Dominguez Cordeiro.

Implementação e comparação de estruturas de dados em árvore em **Rust**, usando um dataset real de jogos da Steam:

- Árvore binária de busca (BST)
- Árvore balanceada 1: *AVL*
- Árvore balanceada 2: *AA*

**Integrantes:** *Miguel*, *Livia* e *Leonardo*

> **Status:** WIP

## Dataset

- **Nome:** Steam Games Dataset
- **Fonte:** Kaggle (autor: fronkongames) — <https://www.kaggle.com/datasets/fronkongames/steam-games-dataset>
- **Data do download:** 19/09/2026
- **Licença:** *MIT*
- **Tamanho usado:** 125.855 registros (bem acima do mínimo de 10.000 exigido)

O arquivo original (`games.csv`, ~383 MB) **não é versionado** neste repositório. O que fica versionado é o arquivo reduzido `data/games_enxuto.csv` (~9,6 MB), gerado pelo filtro descrito abaixo.

### Colunas mantidas

| Coluna | Descrição | Uso |
|---|---|---|
| `AppID` | Identificador único do jogo na Steam | chave |
| `Name` | Nome do jogo | dados |
| `ReleaseDate` | Data de lançamento (texto, ex.: `Aug 1, 2023`) | dados |
| `PeakCCU` | Pico de jogadores simultâneos | dados |
| `Price` | Preço | dados |
| `Positive` | Número de avaliações positivas | dados |
| `Negative` | Número de avaliações negativas | dados |
| `Recommendations` | Número de recomendações | dados |
| `Genres` | Gêneros (texto; pode ser vazio) | dados |

### Observação sobre o CSV original

No CSV do Kaggle, o cabeçalho tem 39 nomes, mas cada linha de dados tem 40 campos: a coluna `DiscountDLC count` corresponde, na verdade, a duas colunas coladas (`Discount` e `DLC count`). Por isso, o filtro **ignora o cabeçalho original e lê os campos pela posição**, escrevendo um cabeçalho novo e correto.

## Requisitos

- [Rust](https://www.rust-lang.org/tools/install) (toolchain estável, com `cargo`)
- A dependência `csv` é baixada automaticamente pelo Cargo
- [Python 3](https://www.python.org/downloads/) com `matplotlib` (**só** para gerar os gráficos): `pip install matplotlib`

## Estrutura do repositório

```
steam/
├── Cargo.toml
├── README.md
├── .gitignore
├── data/
│   ├── games_enxuto.csv     (versionado; games.csv original fica fora do Git)
│   ├── balanceamento*.csv   (gerado: altura de cada árvore a cada N inserções)
│   └── metricas.csv         (gerado: tempo e comparações de inserir/buscar/remover)
├── graficos/
│   └── *.png                (gerados: balanceamento e desempenho)
├── scripts/
│   └── gerar_graficos.py    (lê os CSVs de data/ e desenha todos os gráficos)
└── src/
    ├── main.rs              (insere o dataset nas 3 árvores e mede altura, comparações e tempo)
    ├── jogo.rs              (struct Jogo e loader do CSV)
    ├── arvore.rs            (trait comum às 3 árvores)
    ├── bst.rs / avl.rs / aa.rs
    └── bin/
        ├── filtrar.rs       (filtro que gera o games_enxuto.csv)
        ├── balanceamento.rs (mede a altura a cada N inserções e grava o CSV)
        └── metricas.rs      (mede tempo e comparações de inserir, buscar e remover)
```

## Como executar

### 1. Gerar o CSV reduzido (só é necessário se quiser refazer o filtro)

1. Baixe `games.csv` na página do dataset (link acima) e coloque em `data/games.csv`.
2. Rode, na raiz do projeto:

```bash
cargo run --release --bin filtrar -- data/games.csv data/games_enxuto.csv
```

O programa imprime quantas linhas leu, escreveu e descartou (linhas corrompidas ou sem `AppID` numérico).

Quem apenas clonar o repositório **não precisa** desse passo, pois o `data/games_enxuto.csv` já vem versionado.

### 2. Rodar o programa das árvores


```bash
cargo run --release
```

Imprime, para cada árvore, o número de nós, a altura, as comparações e o tempo de inserção (ordem do arquivo e chaves ordenadas), além de uma busca de exemplo.

### 3. Medir o balanceamento (altura × elementos)

Insere os jogos um por um e anota a altura de cada árvore a cada 1.000 inserções (e no tamanho total):

```bash
cargo run --release --bin balanceamento
```

Isso grava `data/balanceamento.csv` (colunas `n,bst,aa,avl`). Os argumentos são opcionais e vêm nesta ordem: CSV de jogos, passo, modo e CSV de saída.

```bash
# passo de 500 e chaves ordenadas por AppID (pior caso da BST)
cargo run --release --bin balanceamento -- data/games_enxuto.csv 500 ordenado data/balanceamento_ordenado.csv
```

### 4. Medir tempo e comparações de cada operação

Cronometra `inserir`, `buscar` e `remover` em cada árvore, em dois cenários (ordem do arquivo com 125.855 jogos e chaves ordenadas com 20.000 jogos, o pior caso). A busca e a remoção usam uma amostra de 10.000 chaves espalhadas pelo dataset, as mesmas para as 3 árvores:

```bash
cargo run --release --bin metricas
```

Grava `data/metricas.csv`. Argumentos opcionais, nesta ordem: CSV de jogos, CSV de saída e tamanho da amostra.

### 5. Gerar os gráficos

```bash
pip install matplotlib
python3 scripts/gerar_graficos.py
```

Lê os CSVs de `data/` e salva em `graficos/`:

| Arquivo | O que mostra |
|---|---|
| `balanceamento.png` | altura × elementos inseridos (ordem do arquivo) |
| `balanceamento_ordenado.png` | altura × elementos inseridos (chaves ordenadas; precisa do CSV do passo 3 com o modo `ordenado`) |
| `desempenho_<cenario>_tempo.png` | tempo médio por operação |
| `desempenho_<cenario>_comparacoes.png` | comparações médias por operação |

Onde `<cenario>` é `arquivo` ou `ordenado`. O script ignora o gráfico cujo CSV não existir e avisa qual comando rodar.

### 6. Testes

```bash
cargo test
```

> Use sempre `--release` para medir tempo: no modo debug os tempos ficam inflados e não representam o desempenho real. Altura e número de comparações não dependem do modo.

## Métodos implementados

Todas as árvores implementam:

- `inserir(chave, dados)`
- `buscar(chave) -> dados`
- `remover(chave)` (nas balanceadas, com reequilíbrio)
- `altura() -> inteiro`
- `percorrer(ordem) -> lista de chaves` (pré-ordem, em-ordem e pós-ordem)
- `contador_comparacoes() -> inteiro`

## Roadmap

- [x] Escolha do dataset
- [x] Filtro que gera o CSV reduzido
- [x] Loader do CSV reduzido em Rust
- [x] BST
- [x] AA
- [x] AVL
- [x] Medição de altura × elementos e script do gráfico
- [x] Métricas por operação (inserir, buscar, remover) e gráficos de desempenho
- [ ] Slides e apresentação

## Referências

- [Steam Games Dataset (fronkongames), Kaggle](https://www.kaggle.com/datasets/fronkongames/steam-games-dataset)
- [Implementando uma Binary Search Tree](https://www.tabnews.com.br/Programmer404/implementando-uma-binary-search-tree)

## Licença

- **Código:** [GNU AGPL-3.0](LICENSE)
- **Dataset:** Steam Games Dataset (fronkongames), licença MIT, usado em versão reduzida (`data/games_enxuto.csv`).

![Static_Badge](https://redlib.catsarch.com/img/h4u0ezhy2ioh1.jpeg)





