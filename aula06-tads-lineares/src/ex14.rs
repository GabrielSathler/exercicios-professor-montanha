use std::collections::VecDeque;

pub fn eh_palindromo(s: &str) -> bool {
    let mut deque: VecDeque<char> = s
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    while deque.len() > 1 {
        let frente = deque.pop_front().unwrap();
        let tras = deque.pop_back().unwrap();
        if frente != tras {
            return false;
        }
    }
    true
}

pub fn demo() {
    let exemplos = [
        "A man a plan a canal Panama",
        "arara",
        "Socorram me subi no onibus em marrocos",
        "rust",
        "",
    ];
    for frase in exemplos {
        println!("  \"{}\" -> palindromo? {}", frase, eh_palindromo(frase));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frase_classica_eh_palindromo() {
        assert!(eh_palindromo("A man a plan a canal Panama"));
    }

    #[test]
    fn palavra_simples() {
        assert!(eh_palindromo("arara"));
        assert!(!eh_palindromo("rust"));
    }

    #[test]
    fn vazio_e_unico_sao_palindromos() {
        assert!(eh_palindromo(""));
        assert!(eh_palindromo("x"));
    }
}
