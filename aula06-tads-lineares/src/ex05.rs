pub fn avaliar_rpn(expr: &str) -> Option<f64> {
    let mut pilha: Vec<f64> = Vec::new();

    for token in expr.split_whitespace() {
        match token {
            "+" | "-" | "*" | "/" => {
                // a direita sai primeiro do topo, depois a esquerda
                let direita = pilha.pop()?;
                let esquerda = pilha.pop()?;
                let resultado = match token {
                    "+" => esquerda + direita,
                    "-" => esquerda - direita,
                    "*" => esquerda * direita,
                    "/" => {
                        if direita == 0.0 {
                            return None;
                        }
                        esquerda / direita
                    }
                    _ => unreachable!(),
                };
                pilha.push(resultado);
            }
            outro => {
                let n: f64 = outro.parse().ok()?;
                pilha.push(n);
            }
        }
    }

    if pilha.len() == 1 {
        pilha.pop()
    } else {
        None
    }
}

pub fn demo() {
    let expr = "3 4 + 2 *";
    println!("  \"{}\" = {:?}", expr, avaliar_rpn(expr));
    let expr2 = "5 1 2 + 4 * + 3 -";
    println!("  \"{}\" = {:?}", expr2, avaliar_rpn(expr2));
    let invalida = "3 +";
    println!("  \"{}\" = {:?} (inválida)", invalida, avaliar_rpn(invalida));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemplo_basico() {
        assert_eq!(avaliar_rpn("3 4 + 2 *"), Some(14.0));
    }

    #[test]
    fn expressao_complexa() {
        // (5 + ((1+2)*4)) - 3 = 14
        assert_eq!(avaliar_rpn("5 1 2 + 4 * + 3 -"), Some(14.0));
    }

    #[test]
    fn divisao() {
        assert_eq!(avaliar_rpn("10 2 /"), Some(5.0));
    }

    #[test]
    fn invalida_operandos_insuficientes() {
        assert_eq!(avaliar_rpn("3 +"), None);
    }

    #[test]
    fn invalida_token_desconhecido() {
        assert_eq!(avaliar_rpn("3 x 4"), None);
    }

    #[test]
    fn divisao_por_zero() {
        assert_eq!(avaliar_rpn("3 0 /"), None);
    }
}
