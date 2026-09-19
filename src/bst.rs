use crate::arvore::{Arvore, Ordem};
use crate::jogo::Jogo;
use std::cmp::Ordering;

struct No {
    chave: u32,
    dados: Jogo,
    esq: Option<Box<No>>,
    dir: Option<Box<No>>,
}

impl No {
    fn novo(chave: u32, dados: Jogo) -> Self {
        No {
            chave,
            dados,
            esq: None,
            dir: None,
        }
    }
}

pub struct Bst {
    raiz: Option<Box<No>>,
    tamanho: usize,
    comparacoes: u64,
}

impl Bst {
    pub fn nova() -> Self {
        Bst {
            raiz: None,
            tamanho: 0,
            comparacoes: 0,
        }
    }
}

impl Default for Bst {
    fn default() -> Self {
        Self::nova()
    }
}

impl Arvore for Bst {
    fn nome(&self) -> &'static str {
        "BST"
    }

    fn inserir(&mut self, chave: u32, dados: Jogo) {
        // `atual` aponta para o "elo" onde estamos: começa na raiz.
        let mut atual = &mut self.raiz;

        // Enquanto o elo tiver um nó, comparamos e descemos.
        while let Some(no) = atual {
            self.comparacoes += 1;
            match chave.cmp(&no.chave) {
                Ordering::Less => atual = &mut no.esq,
                Ordering::Greater => atual = &mut no.dir,
                Ordering::Equal => {
                    no.dados = dados; // chave repetida: só troca os dados
                    return;
                }
            }
        }

        *atual = Some(Box::new(No::novo(chave, dados)));
        self.tamanho += 1;
    }

    fn buscar(&mut self, chave: u32) -> Option<&Jogo> {
        let mut atual = self.raiz.as_ref();
        while let Some(no) = atual {
            self.comparacoes += 1;
            match chave.cmp(&no.chave) {
                Ordering::Less => atual = no.esq.as_ref(),
                Ordering::Greater => atual = no.dir.as_ref(),
                Ordering::Equal => return Some(&no.dados),
            }
        }
        None
    }

    fn remover(&mut self, chave: u32) -> bool {
        // 1) Achar o elo que aponta para o nó com essa chave.
        let mut atual = &mut self.raiz;
        loop {
            match atual {
                None => return false,
                Some(no) if no.chave == chave => {
                    self.comparacoes += 1;
                    break;
                }
                Some(no) => {
                    self.comparacoes += 1;
                    atual = if chave < no.chave {
                        &mut no.esq
                    } else {
                        &mut no.dir
                    };
                }
            }
        }

        let mut removido = atual.take().unwrap();
        match (removido.esq.take(), removido.dir.take()) {
            // folha: o elo já ficou vazio
            (None, None) => {}
            // um só filho: ele sobe para o lugar do nó
            (Some(filho), None) | (None, Some(filho)) => *atual = Some(filho),
            // dois filhos: o sucessor (menor da direita) assume o lugar
            (Some(esq), Some(dir)) => {
                let mut dir = Some(dir);
                let (chave_s, dados_s) = extrair_minimo(&mut dir);
                *atual = Some(Box::new(No {
                    chave: chave_s,
                    dados: dados_s,
                    esq: Some(esq),
                    dir,
                }));
            }
        }
        self.tamanho -= 1;
        true
    }

    fn altura(&self) -> usize {
        // Percurso com pilha explícita (sem recursão) guardando (nó, nível).
        let mut maior = 0;
        let mut pilha: Vec<(&No, usize)> = Vec::new();
        if let Some(r) = &self.raiz {
            pilha.push((r, 1));
        }
        while let Some((no, nivel)) = pilha.pop() {
            maior = maior.max(nivel);
            if let Some(e) = &no.esq {
                pilha.push((e, nivel + 1));
            }
            if let Some(d) = &no.dir {
                pilha.push((d, nivel + 1));
            }
        }
        maior
    }

    fn tamanho(&self) -> usize {
        self.tamanho
    }

    fn percorrer(&self, ordem: Ordem) -> Vec<u32> {
        let mut saida = Vec::with_capacity(self.tamanho);
        match ordem {
            Ordem::PreOrdem => {
                let mut pilha: Vec<&No> = Vec::new();
                if let Some(r) = &self.raiz {
                    pilha.push(r);
                }
                while let Some(no) = pilha.pop() {
                    saida.push(no.chave);
                    // direita entra primeiro para a esquerda sair primeiro
                    if let Some(d) = &no.dir {
                        pilha.push(d);
                    }
                    if let Some(e) = &no.esq {
                        pilha.push(e);
                    }
                }
            }
            Ordem::EmOrdem => {
                let mut pilha: Vec<&No> = Vec::new();
                let mut atual = self.raiz.as_deref();
                while atual.is_some() || !pilha.is_empty() {
                    // desce o máximo possível pela esquerda
                    while let Some(no) = atual {
                        pilha.push(no);
                        atual = no.esq.as_deref();
                    }
                    let no = pilha.pop().unwrap();
                    saida.push(no.chave);
                    atual = no.dir.as_deref();
                }
            }
            Ordem::PosOrdem => {
                // raiz-direita-esquerda, depois inverte => esquerda-direita-raiz
                let mut pilha: Vec<&No> = Vec::new();
                if let Some(r) = &self.raiz {
                    pilha.push(r);
                }
                while let Some(no) = pilha.pop() {
                    saida.push(no.chave);
                    if let Some(e) = &no.esq {
                        pilha.push(e);
                    }
                    if let Some(d) = &no.dir {
                        pilha.push(d);
                    }
                }
                saida.reverse();
            }
        }
        saida
    }

    fn contador_comparacoes(&self) -> u64 {
        self.comparacoes
    }

    fn zerar_contador(&mut self) {
        self.comparacoes = 0;
    }
}

fn extrair_minimo(raiz: &mut Option<Box<No>>) -> (u32, Jogo) {
    let mut atual = raiz;
    while atual.as_ref().unwrap().esq.is_some() {
        atual = &mut atual.as_mut().unwrap().esq;
    }
    let mut minimo = atual.take().unwrap();
    *atual = minimo.dir.take(); // o filho direito do mínimo ocupa o lugar dele
    (minimo.chave, minimo.dados)
}

impl Drop for Bst {
    fn drop(&mut self) {
        let mut pilha: Vec<Box<No>> = Vec::new();
        if let Some(r) = self.raiz.take() {
            pilha.push(r);
        }
        while let Some(mut no) = pilha.pop() {
            if let Some(e) = no.esq.take() {
                pilha.push(e);
            }
            if let Some(d) = no.dir.take() {
                pilha.push(d);
            }
        }
    }
}

#[cfg(test)]
mod testes {
    use super::*;
    use crate::arvore::testes::{jogo_falso, suite_comum};

    #[test]
    fn suite_comum_bst() {
        suite_comum(Bst::nova());
    }

    #[test]
    fn ordens_de_percurso_da_bst() {
        let mut b = Bst::nova();
        for k in [50, 30, 70, 20, 40, 60, 80] {
            b.inserir(k, jogo_falso(k));
        }
        assert_eq!(
            b.percorrer(Ordem::PreOrdem),
            vec![50, 30, 20, 40, 70, 60, 80]
        );
        assert_eq!(
            b.percorrer(Ordem::EmOrdem),
            vec![20, 30, 40, 50, 60, 70, 80]
        );
        assert_eq!(
            b.percorrer(Ordem::PosOrdem),
            vec![20, 40, 30, 60, 80, 70, 50]
        );
        assert_eq!(b.altura(), 3);
    }

    #[test]
    fn contador_conta_uma_por_no_visitado() {
        let mut b = Bst::nova();
        for k in [50, 30, 70] {
            b.inserir(k, jogo_falso(k));
        }
        b.zerar_contador();
        b.buscar(70); // compara com 50 e depois com 70
        assert_eq!(b.contador_comparacoes(), 2);
    }

    #[test]
    fn insercao_ordenada_degenera_em_lista_sem_estourar_a_pilha() {
        let mut b = Bst::nova();
        for k in 1..=20_000u32 {
            b.inserir(k, jogo_falso(k));
        }
        assert_eq!(b.altura(), 20_000);
        assert_eq!(b.percorrer(Ordem::EmOrdem).len(), 20_000);
        // ao sair do teste, o Drop iterativo destrói tudo sem recursão
    }
}
