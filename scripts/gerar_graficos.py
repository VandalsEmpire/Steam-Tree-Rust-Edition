#!/usr/bin/env python3
"""Gera TODOS os gráficos do trabalho a partir dos CSVs que o programa Rust grava.

Uso (na raiz do projeto):
    cargo run --release --bin balanceamento     # grava data/balanceamento.csv
    cargo run --release --bin metricas          # grava data/metricas.csv
    python3 scripts/gerar_graficos.py           # lê os CSVs e salva os PNGs em graficos/

Gráficos gerados:
  - graficos/balanceamento*.png   altura x elementos inseridos (um por data/balanceamento*.csv)
  - graficos/desempenho_<cenario>_tempo.png         tempo médio por operação
  - graficos/desempenho_<cenario>_comparacoes.png   comparações médias por operação

Dependência: matplotlib  ->  pip install matplotlib
"""
import csv
import glob
import os

import matplotlib

matplotlib.use("Agg")  # desenha direto em arquivo, sem abrir janela
import matplotlib.pyplot as plt
from matplotlib.ticker import FuncFormatter

PASTA_SAIDA = "graficos"

# Uma cor fixa por árvore, em todos os gráficos
CORES = {"BST": "#2a78d6", "AA-Tree": "#eb6834", "AVL": "#1baf7a"}
ARVORES = ["BST", "AA-Tree", "AVL"]
OPERACOES = ["inserir", "buscar", "remover"]
NOMES_CENARIO = {
    "arquivo": "ordem do arquivo (125.855 jogos)",
    "ordenado": "chaves ordenadas, pior caso (20.000 jogos)",
}


def br(valor, casas=0):
    """Número no formato brasileiro: 1234.5 -> 1.234,5"""
    texto = f"{valor:,.{casas}f}"
    return texto.replace(",", "X").replace(".", ",").replace("X", ".")


def formatar_eixo(valor, _posicao):
    return br(valor, 0) if valor >= 1 else br(valor, 1)


def acabamento(ax):
    """Estilo limpo: só grade horizontal e sem bordas desnecessárias."""
    ax.grid(axis="y", color="#e1e0d9", linewidth=0.8)
    ax.set_axisbelow(True)
    for lado in ("top", "right"):
        ax.spines[lado].set_visible(False)
    ax.yaxis.set_major_formatter(FuncFormatter(formatar_eixo))


def salvar(fig, caminho):
    os.makedirs(os.path.dirname(caminho), exist_ok=True)
    fig.tight_layout()
    fig.savefig(caminho)
    plt.close(fig)
    print(f"Gráfico salvo em {caminho}")


# ---------------------------------------------------------------------------
# 1) Balanceamento: altura x elementos inseridos
# ---------------------------------------------------------------------------
def grafico_balanceamento(entrada):
    n, bst, aa, avl = [], [], [], []
    with open(entrada, newline="") as f:
        for linha in csv.DictReader(f):
            n.append(int(linha["n"]))
            bst.append(int(linha["bst"]))
            aa.append(int(linha["aa"]))
            avl.append(int(linha["avl"]))

    fig, ax = plt.subplots(figsize=(8, 5), dpi=200)
    ax.plot(n, bst, color=CORES["BST"], linewidth=2, label="BST")
    ax.plot(n, aa, color=CORES["AA-Tree"], linewidth=2, label="AA-Tree")
    ax.plot(n, avl, color=CORES["AVL"], linewidth=2, label="AVL")
    ax.set_title("Altura da árvore conforme os jogos são inseridos")
    ax.set_xlabel("Elementos inseridos")
    ax.set_ylabel("Altura")
    ax.xaxis.set_major_formatter(FuncFormatter(lambda v, _: br(v)))
    acabamento(ax)
    ax.legend(frameon=False)

    nome = os.path.splitext(os.path.basename(entrada))[0]  # ex.: balanceamento_ordenado
    salvar(fig, f"{PASTA_SAIDA}/{nome}.png")


# ---------------------------------------------------------------------------
# 2) Desempenho: barras agrupadas por operação, uma cor por árvore
# ---------------------------------------------------------------------------
def ler_metricas(entrada):
    """Devolve {cenario: {(arvore, operacao): linha_do_csv}}"""
    dados = {}
    with open(entrada, newline="") as f:
        for linha in csv.DictReader(f):
            dados.setdefault(linha["cenario"], {})[(linha["arvore"], linha["operacao"])] = linha
    return dados


def grafico_desempenho(cenario, medicoes, coluna, titulo, unidade, sufixo):
    largura = 0.26
    fig, ax = plt.subplots(figsize=(9, 5.5), dpi=200)
    valores_todos = []

    for i, arvore in enumerate(ARVORES):
        valores = [float(medicoes[(arvore, op)][coluna]) for op in OPERACOES]
        valores_todos += valores
        posicoes = [j + (i - 1) * largura for j in range(len(OPERACOES))]
        barras = ax.bar(posicoes, valores, width=largura * 0.92, color=CORES[arvore],
                        label=arvore, zorder=3)
        for barra, v in zip(barras, valores):
            ax.annotate(br(v, 1 if v < 100 else 0), (barra.get_x() + barra.get_width() / 2, v),
                        xytext=(0, 3), textcoords="offset points", ha="center",
                        fontsize=7.5, fontweight="bold")

    # se a diferença entre o menor e o maior valor é enorme, usa escala logarítmica
    usa_log = max(valores_todos) / max(min(valores_todos), 1e-9) > 50
    if usa_log:
        ax.set_yscale("log")
        ax.set_ylim(min(valores_todos) / 3, max(valores_todos) * 6)
    else:
        ax.set_ylim(0, max(valores_todos) * 1.15)

    ax.set_xticks(range(len(OPERACOES)))
    ax.set_xticklabels(OPERACOES)
    ax.set_ylabel(unidade + (" (escala log)" if usa_log else ""))
    ax.set_title(f"{titulo}\n{NOMES_CENARIO.get(cenario, cenario)}", fontsize=11)
    acabamento(ax)
    ax.legend(frameon=False, ncol=3, loc="upper left")
    salvar(fig, f"{PASTA_SAIDA}/desempenho_{cenario}_{sufixo}.png")


def graficos_desempenho(entrada):
    for cenario, medicoes in ler_metricas(entrada).items():
        grafico_desempenho(cenario, medicoes, "tempo_us_por_op",
                           "Tempo médio por operação", "microssegundos por operação", "tempo")
        grafico_desempenho(cenario, medicoes, "comparacoes_por_op",
                           "Comparações médias por operação", "comparações por operação",
                           "comparacoes")


# ---------------------------------------------------------------------------
def main():
    csvs_balanceamento = sorted(glob.glob("data/balanceamento*.csv"))
    if not csvs_balanceamento:
        print("Nenhum data/balanceamento*.csv. Rode: cargo run --release --bin balanceamento")
    for caminho in csvs_balanceamento:
        grafico_balanceamento(caminho)

    if os.path.exists("data/metricas.csv"):
        graficos_desempenho("data/metricas.csv")
    else:
        print("data/metricas.csv não existe. Rode: cargo run --release --bin metricas")


if __name__ == "__main__":
    main()
