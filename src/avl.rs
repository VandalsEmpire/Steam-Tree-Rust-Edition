use crate::arvore::{Arvore, Ordem};
use crate::jogo::Jogo;
use std::cmp::Ordering;

struct No {
    chave: u32,
    dados: Jogo,
    altura: i32,
    esq: Option<Box<No>>,
    dir: Option<Box<No>>,
}

impl No {
    fn novo(chave: u32, dados: Jogo) -> Self {
        No {
            chave,
            dados,
            altura: 1,
            esq: None,
            dir: None,
        }
    }
}

pub struct Avl {
    raiz: Option<Box<No>>,
    tamanho: usize,
    comparacoes: u64,
}

impl Avl {
    pub fn nova() -> Self {
        Avl {
            raiz: None,
            tamanho: 0,
            comparacoes: 0,
        }
    }
}

impl Default for Avl {
    fn default() -> Self {
        Self::nova()
    }
}

// Funções auxiliares de balanceamento

fn altura_no(no: Option<&No>) -> i32 {
    no.map_or(0, |n| n.altura)
}

fn atualizar_altura(no: &mut No) {
    let altura_esq = altura_no(no.esq.as_deref());
    let altura_dir = altura_no(no.dir.as_deref());
    no.altura = altura_esq.max(altura_dir) + 1;
}

fn fator_balanceamento(no: &No) -> i32 {
    altura_no(no.esq.as_deref()) - altura_no(no.dir.as_deref())
}


fn rotacao_direita(mut y: Box<No>) -> Box<No> {
    let mut x = y
        .esq
        .take()
        .expect("rotação à direita exige filho esquerdo");

    y.esq = x.dir.take();
    atualizar_altura(&mut y);

    x.dir = Some(y);
    atualizar_altura(&mut x);

    x
}



fn rotacao_esquerda(mut x: Box<No>) -> Box<No> {
    let mut y = x
        .dir
        .take()
        .expect("rotação à esquerda exige filho direito");

    x.dir = y.esq.take();
    atualizar_altura(&mut x);

    y.esq = Some(x);
    atualizar_altura(&mut y);

    y
}

/// Atualiza a altura e, faz uma das quatro correções AVL:
/// LL, RR, LR ou RL.
fn rebalancear(mut no: Box<No>) -> Box<No> {
    atualizar_altura(&mut no);

    let fator = fator_balanceamento(&no);

    // LL ou LR.
    if fator > 1 {
        let fator_esq = no
            .esq
            .as_deref()
            .map_or(0, fator_balanceamento);


        // Caso LR: primeiro gira o filho para a esquerda.
        if fator_esq < 0 {
            let esq = no.esq.take().expect("filho esquerdo ausente");
            no.esq = Some(rotacao_esquerda(esq));
        }

        // Caso LL.
        return rotacao_direita(no);
    }

    // Caso RR ou RL.
    if fator < -1 {
        let fator_dir = no
            .dir
            .as_deref()
            .map_or(0, fator_balanceamento);

        // Caso RL: primeiro gira o filho para a direita.
        if fator_dir > 0 {
            let dir = no.dir.take().expect("filho direito ausente");
            no.dir = Some(rotacao_direita(dir));
        }

        // Caso RR.
        return rotacao_esquerda(no);
    }

    no
}

/// Remove e devolve o menor nó da sub deixando a subárvore resultante balanceada.
fn extrair_minimo(mut no: Box<No>) -> (Box<No>, Option<Box<No>>) {
    match no.esq.take() {
        None => {
            let direita = no.dir.take();
            (no, direita)
        }
        Some(esq) => {
            let (minimo, nova_esq) = extrair_minimo(esq);
            no.esq = nova_esq;
            let no = rebalancear(no);
            (minimo, Some(no))
        }
    }
}

// Implementação do trait Arvore
impl Arvore for Avl {
    fn nome(&self) -> &'static str {
        "AVL"
    }

    fn inserir(&mut self, chave: u32, dados: Jogo) {
        fn inserir_recursivo(
            no: Option<Box<No>>,
            chave: u32,
            dados: Jogo,
            comparacoes: &mut u64,
            inserido: &mut bool,
        ) -> Box<No> {
            let mut no = match no {
                None => {
                    *inserido = true;
                    return Box::new(No::novo(chave, dados));
                }
                Some(no) => no,
            };

            *comparacoes += 1;

            match chave.cmp(&no.chave) {
                Ordering::Less => {
                    no.esq = Some(inserir_recursivo(
                        no.esq.take(),
                        chave,
                        dados,
                        comparacoes,
                        inserido,
                    ));
                }
                Ordering::Greater => {
                    no.dir = Some(inserir_recursivo(
                        no.dir.take(),
                        chave,
                        dados,
                        comparacoes,
                        inserido,
                    ));
                }
                Ordering::Equal => {
                    // Mesma regra usada na BST e na AA: chave repetida atualiza os dados e não aumenta o tamanho da árvore.
                    no.dados = dados;
                    return no;
                }
            }

            rebalancear(no)
        }

        let mut inserido = false;
        self.raiz = Some(inserir_recursivo(
            self.raiz.take(),
            chave,
            dados,
            &mut self.comparacoes,
            &mut inserido,
        ));

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
        fn remover_recursivo(
            no: Option<Box<No>>,
            chave: u32,
            comparacoes: &mut u64,
            removido: &mut bool,
        ) -> Option<Box<No>> {
            let mut no = no?;

            *comparacoes += 1;

            match chave.cmp(&no.chave) {
                Ordering::Less => {
                    no.esq = remover_recursivo(
                        no.esq.take(),
                        chave,
                        comparacoes,
                        removido,
                    );
                }
                Ordering::Greater => {
                    no.dir = remover_recursivo(
                        no.dir.take(),
                        chave,
                        comparacoes,
                        removido,
                    );
                }
                Ordering::Equal => {
                    *removido = true;

                    // Caso 1: folha.
                    if no.esq.is_none() && no.dir.is_none() {
                        return None;
                    }

                    // Caso 2: somente filho direito.
                    if no.esq.is_none() {
                        return no.dir.take();
                    }

                    // Caso 2: somente filho esquerdo.
                    if no.dir.is_none() {
                        return no.esq.take();
                    }

                    // Caso 3: dois filhos.
                    // O sucessor (menor da subárvore direita) assume o lugar
                    // do nó removido, preservando a propriedade da BST.
                    let direita = no.dir.take().expect("filho direito ausente");
                    let (sucessor, nova_direita) = extrair_minimo(direita);
                    no.chave = sucessor.chave;
                    no.dados = sucessor.dados;
                    no.dir = nova_direita;
                }
            }

            // Depois de alterar qualquer subárvore, o caminho de retorno é
            // rebalanceado até a raiz.
            Some(rebalancear(no))
        }

        let mut removido = false;
        self.raiz = remover_recursivo(
            self.raiz.take(),
            chave,
            &mut self.comparacoes,
            &mut removido,
        );

        if removido {
            self.tamanho -= 1;
        }

        removido
    }

    fn altura(&self) -> usize {
        altura_no(self.raiz.as_deref()) as usize
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

                    // Direita primeiro para a esquerda sair primeiro.
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
                // Gera raiz-direita-esquerda e depois inverte para obter
                // esquerda-direita-raiz.
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

/// Destruição iterativa para evitar depender da profundidade da pilha de
/// chamadas de `drop` dos `Box`. Na AVL isso não é tão crítico quanto na BST,
/// mas mantém a mesma estratégia do restante do projeto.
impl Drop for Avl {
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
    fn suite_comum_avl() {
        suite_comum(Avl::nova());
    }

    #[test]
    fn rotacao_ll() {
        let mut a = Avl::nova();
        for k in [30, 20, 10] {
            a.inserir(k, jogo_falso(k));
        }

        assert_eq!(a.percorrer(Ordem::PreOrdem), vec![20, 10, 30]);
        assert_eq!(a.altura(), 2);
    }

    #[test]
    fn rotacao_rr() {
        let mut a = Avl::nova();
        for k in [10, 20, 30] {
            a.inserir(k, jogo_falso(k));
        }

        assert_eq!(a.percorrer(Ordem::PreOrdem), vec![20, 10, 30]);
        assert_eq!(a.altura(), 2);
    }

    #[test]
    fn rotacao_lr() {
        let mut a = Avl::nova();
        for k in [30, 10, 20] {
            a.inserir(k, jogo_falso(k));
        }

        assert_eq!(a.percorrer(Ordem::PreOrdem), vec![20, 10, 30]);
        assert_eq!(a.altura(), 2);
    }

    #[test]
    fn rotacao_rl() {
        let mut a = Avl::nova();
        for k in [10, 30, 20] {
            a.inserir(k, jogo_falso(k));
        }

        assert_eq!(a.percorrer(Ordem::PreOrdem), vec![20, 10, 30]);
        assert_eq!(a.altura(), 2);
    }

    #[test]
    fn remocao_rebalanceia() {
        let mut a = Avl::nova();
        for k in [50, 30, 70, 20, 40, 60, 80, 10, 25, 35, 45, 65, 75, 85, 90] {
            a.inserir(k, jogo_falso(k));
        }

        for k in [10, 20, 30, 50] {
            assert!(a.remover(k));
            assert!(a.buscar(k).is_none());
        }

        let esperado = vec![25, 35, 40, 45, 60, 65, 70, 75, 80, 85, 90];
        assert_eq!(a.percorrer(Ordem::EmOrdem), esperado);
        assert!(a.altura() <= 5);
        assert_eq!(a.tamanho(), esperado.len());
    }

    #[test]
    fn insercao_ordenada_mantem_altura_logaritmica() {
        let mut a = Avl::nova();

        for k in 1..=20_000u32 {
            a.inserir(k, jogo_falso(k));
        }

        // Para 20.000 nós, uma AVL precisa continuar com altura muito menor
        // que a BST degenerada (20.000). O limite é folgado para o teste.
        assert!(a.altura() <= 20);
        assert_eq!(a.tamanho(), 20_000);
        assert_eq!(a.percorrer(Ordem::EmOrdem).len(), 20_000);
    }

    #[test]
    fn chave_duplicada_atualiza_dados() {
        let mut a = Avl::nova();
        a.inserir(10, jogo_falso(10));

        let mut novo = jogo_falso(10);
        novo.name = "Atualizado".to_string();
        a.inserir(10, novo);

        assert_eq!(a.tamanho(), 1);
        assert_eq!(a.buscar(10).unwrap().name, "Atualizado");
    }

    #[test]
    fn contador_de_comparacoes_funciona() {
        let mut a = Avl::nova();
        for k in [50, 30, 70] {
            a.inserir(k, jogo_falso(k));
        }

        a.zerar_contador();
        assert_eq!(a.buscar(70).map(|j| j.app_id), Some(70));
        assert_eq!(a.contador_comparacoes(), 2);

        a.zerar_contador();
        assert!(a.buscar(999).is_none());
        assert!(a.contador_comparacoes() > 0);
    }
}
