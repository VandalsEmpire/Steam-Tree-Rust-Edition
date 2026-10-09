#!/usr/bin/env python3
"""Desenha o gráfico altura x elementos a partir do CSV gerado pelo programa Rust.

Uso (na raiz do projeto):
    python3 scripts/gerar_graficos.py
    python3 scripts/gerar_graficos.py data/balanceamento.csv graficos/balanceamento.png

Dependência: matplotlib  ->  pip install matplotlib
"""
import csv
import os
import sys

import matplotlib

matplotlib.use("Agg")  # desenha direto em arquivo, sem abrir janela
import matplotlib.pyplot as plt

entrada = sys.argv[1] if len(sys.argv) > 1 else "data/balanceamento.csv"
saida = sys.argv[2] if len(sys.argv) > 2 else "graficos/balanceamento.png"

# 1) lê o CSV: n,bst,aa,avl
n, bst, aa, avl = [], [], [], []
with open(entrada, newline="") as f:
    for linha in csv.DictReader(f):
        n.append(int(linha["n"]))
        bst.append(int(linha["bst"]))
        aa.append(int(linha["aa"]))
        avl.append(int(linha["avl"]))

# 2) desenha uma linha por árvore (mesmas cores usadas nos outros gráficos)
fig, ax = plt.subplots(figsize=(8, 5), dpi=200)
ax.plot(n, bst, color="#2a78d6", linewidth=2, label="BST")
ax.plot(n, aa, color="#eb6834", linewidth=2, label="AA-Tree")
ax.plot(n, avl, color="#1baf7a", linewidth=2, label="AVL")

ax.set_title("Altura da árvore conforme os jogos são inseridos")
ax.set_xlabel("Elementos inseridos")
ax.set_ylabel("Altura")
ax.grid(axis="y", color="#e1e0d9")
ax.legend(frameon=False)
for lado in ("top", "right"):
    ax.spines[lado].set_visible(False)

# 3) salva a imagem (cria a pasta se não existir)
pasta = os.path.dirname(saida)
if pasta:
    os.makedirs(pasta, exist_ok=True)
fig.tight_layout()
fig.savefig(saida)
print(f"Gráfico salvo em {saida}")
