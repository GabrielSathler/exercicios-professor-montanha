pub fn merge_sort(lista: Vec<i32>) -> Vec<i32> {
    // caso base
    if lista.len() <= 1 {
        return lista;
    }

    let meio = lista.len() / 2;
    let esquerda = merge_sort(lista[..meio].to_vec());
    let direita = merge_sort(lista[meio..].to_vec());

    merge(esquerda, direita)
}

fn merge(esquerda: Vec<i32>, direita: Vec<i32>) -> Vec<i32> {
    let mut resultado: Vec<i32> = Vec::with_capacity(esquerda.len() + direita.len());
    let mut i = 0;
    let mut j = 0;

    while i < esquerda.len() && j < direita.len() {
        if esquerda[i] <= direita[j] {
            resultado.push(esquerda[i]);
            i += 1;
        } else {
            resultado.push(direita[j]);
            j += 1;
        }
    }

    resultado.extend_from_slice(&esquerda[i..]);
    resultado.extend_from_slice(&direita[j..]);

    resultado
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordena_lista_desordenada() {
        let v = vec![5, 2, 9, 1, 5, 6];
        assert_eq!(merge_sort(v), vec![1, 2, 5, 5, 6, 9]);
    }

    #[test]
    fn lista_vazia_e_unitaria() {
        assert_eq!(merge_sort(Vec::<i32>::new()), Vec::<i32>::new());
        assert_eq!(merge_sort(vec![7]), vec![7]);
    }

    #[test]
    fn lista_com_negativos() {
        let v = vec![3, -1, 0, -5, 2];
        assert_eq!(merge_sort(v), vec![-5, -1, 0, 2, 3]);
    }
}
