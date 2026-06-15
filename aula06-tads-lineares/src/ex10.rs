use std::collections::VecDeque;

// LCG determinístico pra não depender do crate rand
fn proximo_aleatorio(estado: &mut u64) -> u64 {
    *estado = estado
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    *estado >> 33
}

pub fn simular_banco(intervalos: &[u32], atendimentos: &[u32]) -> f64 {
    let n = intervalos.len().min(atendimentos.len());
    if n == 0 {
        return 0.0;
    }

    let mut chegadas: VecDeque<u64> = VecDeque::with_capacity(n);
    let mut acumulado: u64 = 0;
    for &intervalo in &intervalos[..n] {
        acumulado += intervalo as u64;
        chegadas.push_back(acumulado);
    }

    let mut espera_total: u64 = 0;
    let mut fim_anterior: u64 = 0; // quando o caixa fica livre
    let mut i = 0;
    while let Some(chegada) = chegadas.pop_front() {
        let inicio = chegada.max(fim_anterior);
        let espera = inicio - chegada;
        espera_total += espera;
        fim_anterior = inicio + atendimentos[i] as u64;
        i += 1;
    }

    espera_total as f64 / n as f64
}

pub fn demo() {
    let mut estado: u64 = 42;
    let mut intervalos: Vec<u32> = Vec::new();
    let mut atendimentos: Vec<u32> = Vec::new();
    for _ in 0..8 {
        intervalos.push((proximo_aleatorio(&mut estado) % 5) as u32 + 1);
        atendimentos.push((proximo_aleatorio(&mut estado) % 8) as u32 + 1);
    }
    println!("  intervalos   = {:?}", intervalos);
    println!("  atendimentos = {:?}", atendimentos);
    let media = simular_banco(&intervalos, &atendimentos);
    println!("  tempo médio de espera = {:.2}", media);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sem_espera_quando_caixa_sempre_livre() {
        // Cada cliente chega 10 depois do anterior; atendimentos curtos (3) => sem fila.
        let intervalos = [10, 10, 10];
        let atendimentos = [3, 3, 3];
        assert_eq!(simular_banco(&intervalos, &atendimentos), 0.0);
    }

    #[test]
    fn espera_acumula_quando_chegam_juntos() {
        // Todos chegam no instante 0 (intervalos 0); atendimento de 5 cada.
        // Cliente 0 espera 0, cliente 1 espera 5, cliente 2 espera 10 => média 5.
        let intervalos = [0, 0, 0];
        let atendimentos = [5, 5, 5];
        assert_eq!(simular_banco(&intervalos, &atendimentos), 5.0);
    }

    #[test]
    fn entrada_vazia() {
        assert_eq!(simular_banco(&[], &[]), 0.0);
    }

    #[test]
    fn lcg_e_deterministico() {
        let mut a: u64 = 1;
        let mut b: u64 = 1;
        assert_eq!(proximo_aleatorio(&mut a), proximo_aleatorio(&mut b));
    }
}
