use std::collections::HashMap;

pub fn contar_letras(frase: &[char]) -> HashMap<char, usize> {
    let mut mapa: HashMap<char, usize> = HashMap::new();
    for c in frase {
        *mapa.entry(*c).or_insert(0) += 1;
    }
    mapa
}

pub fn demo() {
    let frase: Vec<char> = "banana".chars().collect();
    let mapa = contar_letras(&frase);
    // HashMap não tem ordem fixa, então ordeno pra impressão sair sempre igual
    let mut pares: Vec<(char, usize)> = mapa.into_iter().collect();
    pares.sort();
    println!("  frase = \"banana\"");
    for (c, n) in pares {
        println!("    '{}' -> {}", c, n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conta_banana() {
        let frase: Vec<char> = "banana".chars().collect();
        let mapa = contar_letras(&frase);
        assert_eq!(mapa.get(&'b'), Some(&1));
        assert_eq!(mapa.get(&'a'), Some(&3));
        assert_eq!(mapa.get(&'n'), Some(&2));
        assert_eq!(mapa.get(&'z'), None);
    }

    #[test]
    fn conta_vazio() {
        let frase: Vec<char> = Vec::new();
        let mapa = contar_letras(&frase);
        assert!(mapa.is_empty());
    }
}
