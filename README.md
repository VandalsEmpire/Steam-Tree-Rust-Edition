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

## Estrutura do repositório

```
steam/
├── Cargo.toml
├── README.md
├── .gitignore
├── data/
│   └── games_enxuto.csv     (versionado; games.csv original fica fora do Git)
└── src/
    ├── main.rs              (programa das árvores: em desenvolvimento)
    └── bin/
        └── filtrar.rs       (filtro que gera o games_enxuto.csv)
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

*(a preencher quando o programa estiver pronto)*

```bash
cargo run --release
```

### 3. Gerar os gráficos

*(a preencher: dependências e comandos)*

## Métodos implementados

Todas as árvores implementam:

- `inserir(chave, dados)`
- `buscar(chave) -> dados`
- `remover(chave)` (nas balanceadas, com reequilíbrio)
- `altura() -> inteiro`
- `percorrer(ordem) -> lista de chaves` (pré-ordem, em-ordem e pós-ordem)
- `contador_comparacoes() -> inteiro`

*(a preencher: status de cada árvore)*

## Métricas e análise


## Roadmap

- [x] Escolha do dataset
- [x] Filtro que gera o CSV reduzido
- [ ] Loader do CSV reduzido em Rust
- [x] BST
- [ ] Árvore balanceada 1
- [ ] Árvore balanceada 2
- [ ] Métricas e gráficos
- [ ] Slides e apresentação

## Referências

- [Steam Games Dataset (fronkongames), Kaggle](https://www.kaggle.com/datasets/fronkongames/steam-games-dataset)
- [Implementando uma Binary Search Tree](https://www.tabnews.com.br/Programmer404/implementando-uma-binary-search-tree)

## Licença

- **Código:** [GNU AGPL-3.0](LICENSE)
- **Dataset:** Steam Games Dataset (fronkongames), licença MIT, usado em versão reduzida (`data/games_enxuto.csv`).

![Static_Badge](https://redlib.catsarch.com/img/h4u0ezhy2ioh1.jpeg)





