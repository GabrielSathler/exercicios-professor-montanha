pub fn balanceado(s: &str) -> bool {
    let mut pilha: Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            // empilho já o fechamento esperado pra cada abertura
            '(' => pilha.push(')'),
            '[' => pilha.push(']'),
            '{' => pilha.push('}'),
            ')' | ']' | '}' => {
                if pilha.pop() != Some(c) {
                    return false;
                }
            }
            _ => {}
        }
    }
    pilha.is_empty()
}

pub fn demo() {
    for caso in ["{[()]}", "([)]", "(((", "a(b[c]{d})e", ""] {
        println!("  balanceado({:?}) = {}", caso, balanceado(caso));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bem_formado() {
        assert!(balanceado("{[()]}"));
        assert!(balanceado("()[]{}"));
        assert!(balanceado("")); // vazio é balanceado
    }

    #[test]
    fn ordem_errada() {
        assert!(!balanceado("([)]"));
    }

    #[test]
    fn aberturas_sem_fechar() {
        assert!(!balanceado("((("));
    }

    #[test]
    fn fechamento_sobrando() {
        assert!(!balanceado("())"));
    }

    #[test]
    fn ignora_outros_caracteres() {
        assert!(balanceado("a(b[c]{d})e"));
    }
}
