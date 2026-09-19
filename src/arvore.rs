use crate::jogo::Jogo;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ordem {
    PreOrdem,
    EmOrdem,
    PosOrdem,
}

pub trait Arvore {
    fn nome(&self) -> &'static str;

    fn inserir(&mut self, chave: u32, dados: Jogo);

    fn buscar(&mut self, chave: u32) -> Option<&Jogo>;

    fn remover(&mut self, chave: u32) -> bool;

    fn altura(&self) -> usize;

    fn tamanho(&self) -> usize;

    fn percorrer(&self, ordem: Ordem) -> Vec<u32>;

    fn contador_comparacoes(&self) -> u64;

    fn zerar_contador(&mut self);
}

#[cfg(test)]
pub mod testes {
    use super::*;

    pub fn jogo_falso(id: u32) -> Jogo {
        Jogo {
            app_id: id,
            name: format!("Jogo {id}"),
            release_date: String::new(),
            peak_ccu: 0,
            price: 0.0,
            positive: 0,
            negative: 0,
            recommendations: 0,
            genres: String::new(),
        }
    }

    pub fn suite_comum<A: Arvore>(mut a: A) {
        assert_eq!(a.tamanho(), 0);
        assert_eq!(a.altura(), 0);
        assert!(a.percorrer(Ordem::EmOrdem).is_empty());
        assert!(a.buscar(1).is_none());
        assert!(!a.remover(1));

        let chaves = [50, 30, 70, 20, 40, 60, 80, 10, 25, 35, 45, 65, 75, 85, 90];
        for &k in &chaves {
            a.inserir(k, jogo_falso(k));
        }
        assert_eq!(a.tamanho(), chaves.len());

        let mut esperado = chaves.to_vec();
        esperado.sort();
        assert_eq!(a.percorrer(Ordem::EmOrdem), esperado);

        let mut pre = a.percorrer(Ordem::PreOrdem);
        let mut pos = a.percorrer(Ordem::PosOrdem);
        pre.sort();
        pos.sort();
        assert_eq!(pre, esperado);
        assert_eq!(pos, esperado);

        a.zerar_contador();
        assert_eq!(a.buscar(45).map(|j| j.app_id), Some(45));
        assert!(a.contador_comparacoes() > 0);
        assert!(a.buscar(999).is_none());

        let mut novo = jogo_falso(40);
        novo.name = "Trocado".to_string();
        a.inserir(40, novo);
        assert_eq!(a.tamanho(), chaves.len());
        assert_eq!(a.buscar(40).unwrap().name, "Trocado");

        for &k in &[10, 20, 30, 50] {
            assert!(a.remover(k));
            assert!(a.buscar(k).is_none());
        }
        assert!(!a.remover(10));
        let mut restantes: Vec<u32> = esperado
            .iter()
            .copied()
            .filter(|k| ![10, 20, 30, 50].contains(k))
            .collect();
        restantes.sort();
        assert_eq!(a.percorrer(Ordem::EmOrdem), restantes);
        assert_eq!(a.tamanho(), restantes.len());
    }
}
