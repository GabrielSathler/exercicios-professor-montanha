use std::collections::VecDeque;
use std::time::Instant;

pub struct FilaVecIngenua {
    dados: Vec<i32>,
}

impl FilaVecIngenua {
    pub fn nova() -> Self {
        FilaVecIngenua { dados: Vec::new() }
    }
    pub fn enqueue(&mut self, x: i32) {
        self.dados.push(x);
    }
    pub fn dequeue(&mut self) -> Option<i32> {
        if self.dados.is_empty() {
            None
        } else {
            Some(self.dados.remove(0))
        }
    }
}

pub struct FilaDeque {
    dados: VecDeque<i32>,
}

impl FilaDeque {
    pub fn nova() -> Self {
        FilaDeque {
            dados: VecDeque::new(),
        }
    }
    pub fn enqueue(&mut self, x: i32) {
        self.dados.push_back(x);
    }
    pub fn dequeue(&mut self) -> Option<i32> {
        self.dados.pop_front()
    }
}

pub struct FilaCircular {
    buf: Vec<Option<i32>>,
    inicio: usize,
    tamanho: usize,
    capacidade: usize,
}

impl FilaCircular {
    pub fn nova(capacidade: usize) -> Self {
        FilaCircular {
            buf: vec![None; capacidade.max(1)],
            inicio: 0,
            tamanho: 0,
            capacidade: capacidade.max(1),
        }
    }
    pub fn enqueue(&mut self, x: i32) {
        if self.tamanho == self.capacidade {
            return;
        }
        let pos = (self.inicio + self.tamanho) % self.capacidade;
        self.buf[pos] = Some(x);
        self.tamanho += 1;
    }
    pub fn dequeue(&mut self) -> Option<i32> {
        if self.tamanho == 0 {
            return None;
        }
        let valor = self.buf[self.inicio].take();
        self.inicio = (self.inicio + 1) % self.capacidade;
        self.tamanho -= 1;
        valor
    }
}

pub fn demo() {
    const N: i32 = 10_000;

    let mut a = FilaVecIngenua::nova();
    let t0 = Instant::now();
    for i in 0..N {
        a.enqueue(i);
    }
    while a.dequeue().is_some() {}
    let dur_a = t0.elapsed();

    let mut b = FilaDeque::nova();
    let t1 = Instant::now();
    for i in 0..N {
        b.enqueue(i);
    }
    while b.dequeue().is_some() {}
    let dur_b = t1.elapsed();

    let mut c = FilaCircular::nova(N as usize);
    let t2 = Instant::now();
    for i in 0..N {
        c.enqueue(i);
    }
    while c.dequeue().is_some() {}
    let dur_c = t2.elapsed();

    println!("  (a) FilaVecIngenua (remove(0), O(n^2)): {:?}", dur_a);
    println!("  (b) VecDeque       (amortizado O(1)) : {:?}", dur_b);
    println!("  (c) FilaCircular   (O(1))            : {:?}", dur_c);
    println!("  Observacao: a ingenua tende a ser muito mais lenta (deslocamento).");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fila_ingenua_mantem_ordem_fifo() {
        let mut f = FilaVecIngenua::nova();
        f.enqueue(1);
        f.enqueue(2);
        f.enqueue(3);
        assert_eq!(f.dequeue(), Some(1));
        assert_eq!(f.dequeue(), Some(2));
        assert_eq!(f.dequeue(), Some(3));
        assert_eq!(f.dequeue(), None);
    }

    #[test]
    fn fila_deque_mantem_ordem_fifo() {
        let mut f = FilaDeque::nova();
        for i in 0..5 {
            f.enqueue(i);
        }
        for esperado in 0..5 {
            assert_eq!(f.dequeue(), Some(esperado));
        }
        assert_eq!(f.dequeue(), None);
    }

    #[test]
    fn fila_circular_mantem_ordem_e_reaproveita_buffer() {
        let mut f = FilaCircular::nova(3);
        f.enqueue(10);
        f.enqueue(20);
        assert_eq!(f.dequeue(), Some(10));
        f.enqueue(30);
        f.enqueue(40);
        assert_eq!(f.dequeue(), Some(20));
        assert_eq!(f.dequeue(), Some(30));
        assert_eq!(f.dequeue(), Some(40));
        assert_eq!(f.dequeue(), None);
    }
}
