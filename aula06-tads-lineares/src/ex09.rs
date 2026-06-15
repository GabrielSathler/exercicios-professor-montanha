pub struct StackMin {
    dados: Vec<i32>,
    minimos: Vec<i32>,
}

impl StackMin {
    pub fn nova() -> Self {
        StackMin {
            dados: Vec::new(),
            minimos: Vec::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        // o topo de minimos guarda o menor da pilha até aqui
        let novo_min = match self.minimos.last() {
            Some(&m) if m < x => m,
            _ => x,
        };
        self.dados.push(x);
        self.minimos.push(novo_min);
    }

    pub fn pop(&mut self) -> Option<i32> {
        let valor = self.dados.pop()?;
        self.minimos.pop();
        Some(valor)
    }

    pub fn min(&self) -> Option<i32> {
        self.minimos.last().copied()
    }

    pub fn vazia(&self) -> bool {
        self.dados.is_empty()
    }
}

pub fn demo() {
    let mut p = StackMin::nova();
    println!("  vazia? {}", p.vazia());
    for x in [5, 3, 7, 2, 8] {
        p.push(x);
        println!("  push {} -> min = {:?}", x, p.min());
    }
    println!("  pop {:?} -> min = {:?}", p.pop(), p.min());
    println!("  pop {:?} -> min = {:?}", p.pop(), p.min());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_acompanha_pilha() {
        let mut p = StackMin::nova();
        p.push(5);
        assert_eq!(p.min(), Some(5));
        p.push(3);
        assert_eq!(p.min(), Some(3));
        p.push(7);
        assert_eq!(p.min(), Some(3));
        p.push(2);
        assert_eq!(p.min(), Some(2));
        assert_eq!(p.pop(), Some(2));
        assert_eq!(p.min(), Some(3)); // mínimo volta corretamente
    }

    #[test]
    fn pilha_vazia() {
        let mut p = StackMin::nova();
        assert!(p.vazia());
        assert_eq!(p.pop(), None);
        assert_eq!(p.min(), None);
    }

    #[test]
    fn duplicatas_no_minimo() {
        let mut p = StackMin::nova();
        p.push(2);
        p.push(2);
        assert_eq!(p.min(), Some(2));
        p.pop();
        assert_eq!(p.min(), Some(2));
    }
}
