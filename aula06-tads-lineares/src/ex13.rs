pub struct FilaPrioridade {
    itens: Vec<(u8, i32)>,
}

impl FilaPrioridade {
    pub fn nova() -> Self {
        FilaPrioridade { itens: Vec::new() }
    }

    pub fn inserir(&mut self, prioridade: u8, valor: i32) {
        self.itens.push((prioridade, valor));
    }

    pub fn remover(&mut self) -> Option<i32> {
        if self.itens.is_empty() {
            return None;
        }
        let mut melhor_idx = 0;
        for i in 1..self.itens.len() {
            // estritamente maior pra o primeiro vencer empates (FIFO)
            if self.itens[i].0 > self.itens[melhor_idx].0 {
                melhor_idx = i;
            }
        }
        let (_prioridade, valor) = self.itens.remove(melhor_idx);
        Some(valor)
    }

    pub fn len(&self) -> usize {
        self.itens.len()
    }

    pub fn is_empty(&self) -> bool {
        self.itens.is_empty()
    }
}

pub fn demo() {
    let mut fila = FilaPrioridade::nova();
    fila.inserir(1, 10);
    fila.inserir(3, 30);
    fila.inserir(2, 20);
    fila.inserir(3, 31);
    println!("  itens na fila = {} (vazia? {})", fila.len(), fila.is_empty());
    print!("  ordem de remoção:");
    while let Some(v) = fila.remover() {
        print!(" {}", v);
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_por_prioridade() {
        let mut fila = FilaPrioridade::nova();
        fila.inserir(1, 10);
        fila.inserir(3, 30);
        fila.inserir(2, 20);
        assert_eq!(fila.remover(), Some(30));
        assert_eq!(fila.remover(), Some(20));
        assert_eq!(fila.remover(), Some(10));
        assert_eq!(fila.remover(), None);
    }

    #[test]
    fn empate_resolve_por_fifo() {
        let mut fila = FilaPrioridade::nova();
        fila.inserir(5, 100);
        fila.inserir(5, 200);
        assert_eq!(fila.remover(), Some(100));
        assert_eq!(fila.remover(), Some(200));
    }

    #[test]
    fn fila_vazia() {
        let mut fila = FilaPrioridade::nova();
        assert!(fila.is_empty());
        assert_eq!(fila.remover(), None);
    }
}
