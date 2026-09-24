use crate::arvore::{Arvore, Ordem};
use crate::jogo::Jogo;
use std::cmp::Ordering;

struct No {
    chave: u32,
    dados: Jogo,
    nivel: u32, // nivel do nó na árvore, útil para controlar o balanceamento
    esq: Option<Box<No>>,
    dir: Option<Box<No>>,
}

impl No {
    fn novo(chave: u32, dados: Jogo) -> Self {
        No {
            chave,
            dados,
            nivel: 1, // nível inicial do nó é 1, sempre 1
            esq: None,
            dir: None,
        }
    }
}

pub struct Aa {
    raiz: Option<Box<No>>,
    tamanho: usize,
    comparacoes: u64,
}

impl Aa {
    pub fn nova() -> Self {
        Aa {
            raiz: None,
            tamanho: 0,
            comparacoes: 0,
        }
    }
}

impl Default for Aa {
    fn default() -> Self {
        Self::nova()
    }
}

//funcoes da arvore aa

//rotaciona para a direita
fn skew(mut no: Box<No>) -> Box<No> {
    if let Some(esq_nivel) = no.esq.as_ref().map(|n|n.nivel){
        if esq_nivel == no.nivel {
            let mut nova_raiz = no.esq.take().unwrap();
            no.esq = nova_raiz.dir.take();
            nova_raiz.dir = Some(no);
            return nova_raiz;
        }
    }
    no
}

//rotaciona para a esquerda e sobe o nível(quebra a linha em uma arvore)
fn split(mut no: Box<No>) -> Box<No> {
    let neto_direito_mesmo_nivel = no.dir.as_ref().and_then(|d| d.dir.as_ref())
    .map(|neto| neto.nivel == no.nivel).unwrap_or(false);

    if neto_direito_mesmo_nivel {
        let mut nova_raiz = no.dir.take().unwrap();
        no.dir = nova_raiz.esq.take();
        nova_raiz.nivel += 1;
        nova_raiz.esq = Some(no);
        return nova_raiz;
    }
    no
}

fn menor_no(mut no: &No) -> (u32, &Jogo) {
    while let Some(ref prox) = no.esq {
        no = prox;
    }
    (no.chave, &no.dados)
}

fn maior_no(mut no: &No) -> (u32, &Jogo) {
    while let Some(ref prox) = no.dir {
        no = prox;
    }
    (no.chave, &no.dados)
}

fn reajustar_nivel(no: &mut No) {
    let nivel_esq = no.esq.as_ref().map_or(0, |e| e.nivel);
    let nivel_dir = no.dir.as_ref().map_or(0, |d| d.nivel);
    let nivel_ideal = nivel_esq.min(nivel_dir) + 1;

    if nivel_ideal < no.nivel {
        no.nivel = nivel_ideal;
        if let Some(ref mut dir) = no.dir {
            if nivel_ideal < dir.nivel {
                dir.nivel = nivel_ideal;
            }
        }
    }
}


// Implementação do trait Arvore para a árvore AA

impl Arvore for Aa {
    fn nome(&self) -> &'static str {
        "AA-Tree"
    }

    fn inserir(&mut self, chave: u32, dados: Jogo) {
        fn inserir_recursivo(no: Option<Box<No>>, chave: u32, dados: Jogo,
             comparacoes: &mut u64, inserido: &mut bool,) -> Box<No> {
                let mut no = match no {
                    None => {
                        *inserido = true;
                        return Box::new(No::novo(chave, dados));
                    }
                    Some(n_) => n_,
                };
                *comparacoes += 1;
                match chave.cmp(&no.chave) {
                    Ordering::Less => {
                        no.esq = Some(inserir_recursivo(no.esq.take(), chave, dados, comparacoes, inserido));
                    }
                    Ordering::Greater => {
                        no.dir = Some(inserir_recursivo(no.dir.take(), chave, dados, comparacoes, inserido));
                    }
                    Ordering::Equal => {
                        no.dados = dados; // Chave duplicada: apenas atualiza
                        return no;
                    }
                }
            let no = skew(no);
            split(no)
        }
        let mut inserido = false;
        self.raiz = Some(inserir_recursivo(self.raiz.take(), chave,
    dados, &mut self.comparacoes, &mut inserido));
        if inserido {
            self.tamanho += 1;
        }
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
        fn remover_recursivo(no: Option<Box<No>>, chave: u32, comparacoes: &mut u64,
            removido: &mut bool) -> Option<Box<No>> {
            let mut no = no?;

            *comparacoes += 1;
            match chave.cmp(&no.chave) {
                Ordering::Less => {
                    no.esq = remover_recursivo(no.esq, chave, comparacoes, removido);
                }
                Ordering::Greater => {
                    no.dir = remover_recursivo(no.dir, chave, comparacoes, removido);
                }
                Ordering::Equal => {
                    *removido = true;
                    if no.esq.is_none() && no.dir.is_none() {
                        return None;
                    } else if no.esq.is_none() {
                        let (sub_chave, sub_dados) = menor_no(no.esq.as_ref().unwrap());
                        no.chave = sub_chave;
                        no.dados = sub_dados.clone();
                        no.dir = remover_recursivo(no.dir.take(), sub_chave, comparacoes, &mut false);
                    } else {
                        let (sub_chave, sub_dados) = menor_no(no.dir.as_ref().unwrap());
                        no.chave = sub_chave;
                        no.dados = sub_dados.clone();
                        no.esq = remover_recursivo(no.esq.take(), sub_chave, comparacoes, &mut false);
                    }
                }
            }
            // Reajusta os níveis e aplica os passos de rebalanceamento
            reajustar_nivel(&mut no);
            let mut no = skew(no);
            if let Some(mut dir) = no.dir.take() {
                dir = skew(dir);
                if let Some(neto) = dir.dir.take() {
                    dir.dir = Some(skew(neto));
                }
                no.dir = Some(dir);
            }

            let mut no = split(no);
            if let Some(dir) = no.dir.take() {
                no.dir = Some(split(dir));
            }
            Some(no)
        }
        let mut removido = false;
        self.raiz = remover_recursivo(self.raiz.take(), chave, &mut self.comparacoes, &mut removido);
        if removido {
            self.tamanho -= 1;
        }
        removido
    }

    //nao a alteracao em relacao ao bst
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

    //nao a alteracao em relacao ao bst
    fn tamanho(&self) -> usize {
        self.tamanho
    }

    //nao a alteracao em relacao ao bst
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

impl Drop for Aa {
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
    fn suite_comum_aa() {
        suite_comum(Aa::nova());
    }

    #[test]
    fn ordens_de_percurso_da_aa() {
        let mut a = Aa::nova();
        for k in [50, 30, 70, 20, 40, 60, 80] {
            a.inserir(k, jogo_falso(k));
        }
        assert_eq!(
            a.percorrer(Ordem::PreOrdem),
            vec![50, 30, 20, 40, 70, 60, 80]
        );
        assert_eq!(
            a.percorrer(Ordem::EmOrdem),
            vec![20, 30, 40, 50, 60, 70, 80]
        );
        assert_eq!(
            a.percorrer(Ordem::PosOrdem),
            vec![20, 40, 30, 60, 80, 70, 50]
        );
    }

    #[test]
    fn contador_conta_uma_por_no_visitado() {
        let mut a = Aa::nova();
        for k in [50, 30, 70] {
            a.inserir(k, jogo_falso(k));
        }
        a.zerar_contador();
        a.buscar(70);
        assert!(a.contador_comparacoes() > 0);
    }

    #[test]
    fn insercao_ordenada_degenera_em_lista_sem_estourar_a_pilha() {
        let mut a = Aa::nova();
        for k in 1..=20_000u32 {
            a.inserir(k, jogo_falso(k));
        }
        // Na Árvore aa, a altura fica limitada a O(log n) -> em torno de 15 a 30.
        assert!(a.altura() <= 32);
        assert_eq!(a.tamanho(), 20_000);
    }
}
