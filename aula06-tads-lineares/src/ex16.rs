use std::collections::VecDeque;

pub struct GerenciadorTarefas {
    fila: VecDeque<String>,
}

impl GerenciadorTarefas {
    pub fn novo() -> Self {
        GerenciadorTarefas {
            fila: VecDeque::new(),
        }
    }

    pub fn urgente(&mut self, t: &str) {
        self.fila.push_front(t.to_string());
    }

    pub fn normal(&mut self, t: &str) {
        self.fila.push_back(t.to_string());
    }

    pub fn proxima(&mut self) -> Option<String> {
        self.fila.pop_front()
    }

    pub fn pendentes(&self) -> usize {
        self.fila.len()
    }
}

impl Default for GerenciadorTarefas {
    fn default() -> Self {
        Self::novo()
    }
}

pub fn demo() {
    let mut g = GerenciadorTarefas::novo();
    g.normal("escrever relatorio");
    g.normal("responder e-mails");
    g.urgente("corrigir bug em producao");
    g.normal("revisar PR");
    g.urgente("atender chamado critico");

    println!("  tarefas pendentes: {}", g.pendentes());
    print!("  ordem de atendimento:");
    while let Some(tarefa) = g.proxima() {
        print!(" [{}]", tarefa);
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urgentes_atendidas_antes_das_normais() {
        let mut g = GerenciadorTarefas::novo();
        g.normal("a");
        g.normal("b");
        g.urgente("u");
        assert_eq!(g.proxima(), Some("u".to_string()));
        assert_eq!(g.proxima(), Some("a".to_string()));
        assert_eq!(g.proxima(), Some("b".to_string()));
        assert_eq!(g.proxima(), None);
    }

    #[test]
    fn duas_urgentes_a_ultima_fica_na_frente() {
        let mut g = GerenciadorTarefas::novo();
        g.urgente("u1");
        g.urgente("u2");
        assert_eq!(g.proxima(), Some("u2".to_string()));
        assert_eq!(g.proxima(), Some("u1".to_string()));
    }

    #[test]
    fn contagem_de_pendentes() {
        let mut g = GerenciadorTarefas::novo();
        assert_eq!(g.pendentes(), 0);
        g.normal("x");
        g.urgente("y");
        assert_eq!(g.pendentes(), 2);
    }
}
