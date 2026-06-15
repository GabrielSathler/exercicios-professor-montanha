pub struct FilaCircular {
    buf: Vec<Option<i32>>,
    inicio: usize,
    tamanho: usize,
    capacidade: usize,
}

impl FilaCircular {
    pub fn nova(capacidade: usize) -> Self {
        let capacidade = capacidade.max(1);
        FilaCircular {
            buf: vec![None; capacidade],
            inicio: 0,
            tamanho: 0,
            capacidade,
        }
    }

    pub fn inserir(&mut self, x: i32) {
        let pos = (self.inicio + self.tamanho) % self.capacidade;
        self.buf[pos] = Some(x);
        if self.tamanho == self.capacidade {
            // cheia: o slot escrito era o mais antigo, entao avanca o inicio
            self.inicio = (self.inicio + 1) % self.capacidade;
        } else {
            self.tamanho += 1;
        }
    }

    pub fn itens(&self) -> Vec<i32> {
        let mut saida: Vec<i32> = Vec::with_capacity(self.tamanho);
        for k in 0..self.tamanho {
            let pos = (self.inicio + k) % self.capacidade;
            if let Some(v) = self.buf[pos] {
                saida.push(v);
            }
        }
        saida
    }

    pub fn len(&self) -> usize {
        self.tamanho
    }

    pub fn is_empty(&self) -> bool {
        self.tamanho == 0
    }
}

pub fn demo() {
    let mut fila = FilaCircular::nova(3);
    println!("  vazia? {} (len = {})", fila.is_empty(), fila.len());
    for x in [1, 2, 3] {
        fila.inserir(x);
    }
    println!("  após inserir 1,2,3 = {:?} (len = {})", fila.itens(), fila.len());
    fila.inserir(4);
    println!("  após inserir 4     = {:?}", fila.itens());
    fila.inserir(5);
    println!("  após inserir 5     = {:?}", fila.itens());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preenche_sem_sobrescrever() {
        let mut fila = FilaCircular::nova(3);
        fila.inserir(1);
        fila.inserir(2);
        assert_eq!(fila.itens(), vec![1, 2]);
        assert_eq!(fila.len(), 2);
    }

    #[test]
    fn sobrescreve_o_mais_antigo() {
        let mut fila = FilaCircular::nova(3);
        for x in [1, 2, 3, 4] {
            fila.inserir(x);
        }
        assert_eq!(fila.itens(), vec![2, 3, 4]);
    }

    #[test]
    fn varias_sobrescritas() {
        let mut fila = FilaCircular::nova(2);
        for x in [1, 2, 3, 4, 5] {
            fila.inserir(x);
        }
        assert_eq!(fila.itens(), vec![4, 5]);
    }

    #[test]
    fn capacidade_minima_um() {
        let mut fila = FilaCircular::nova(0);
        fila.inserir(9);
        fila.inserir(10);
        assert_eq!(fila.itens(), vec![10]);
    }
}
