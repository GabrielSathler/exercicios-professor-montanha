pub struct Navegador {
    atual: Option<String>,
    back: Vec<String>,
    forward: Vec<String>,
}

impl Navegador {
    pub fn novo() -> Self {
        Navegador {
            atual: None,
            back: Vec::new(),
            forward: Vec::new(),
        }
    }

    // ao visitar uma nova url, não dá mais pra avançar: limpa o forward
    pub fn visitar(&mut self, url: &str) {
        if let Some(anterior) = self.atual.take() {
            self.back.push(anterior);
        }
        self.atual = Some(url.to_string());
        self.forward.clear();
    }

    pub fn voltar(&mut self) -> Option<String> {
        let anterior = self.back.pop()?;
        if let Some(atual) = self.atual.take() {
            self.forward.push(atual);
        }
        self.atual = Some(anterior.clone());
        Some(anterior)
    }

    pub fn avancar(&mut self) -> Option<String> {
        let proxima = self.forward.pop()?;
        if let Some(atual) = self.atual.take() {
            self.back.push(atual);
        }
        self.atual = Some(proxima.clone());
        Some(proxima)
    }

    pub fn atual(&self) -> Option<&str> {
        self.atual.as_deref()
    }
}

pub fn demo() {
    let mut nav = Navegador::novo();
    nav.visitar("a.com");
    nav.visitar("b.com");
    nav.visitar("c.com");
    println!("  atual           = {:?}", nav.atual());
    println!("  voltar          = {:?}", nav.voltar());
    println!("  voltar          = {:?}", nav.voltar());
    println!("  avancar         = {:?}", nav.avancar());
    nav.visitar("d.com");
    println!("  após visitar d, avancar = {:?}", nav.avancar()); // None: forward limpo
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fluxo_voltar_avancar() {
        let mut nav = Navegador::novo();
        nav.visitar("a.com");
        nav.visitar("b.com");
        nav.visitar("c.com");
        assert_eq!(nav.atual(), Some("c.com"));
        assert_eq!(nav.voltar(), Some("b.com".to_string()));
        assert_eq!(nav.voltar(), Some("a.com".to_string()));
        assert_eq!(nav.voltar(), None); // nada antes de a.com
        assert_eq!(nav.avancar(), Some("b.com".to_string()));
    }

    #[test]
    fn visitar_limpa_forward() {
        let mut nav = Navegador::novo();
        nav.visitar("a.com");
        nav.visitar("b.com");
        nav.voltar(); // atual = a.com, forward = [b.com]
        nav.visitar("z.com"); // limpa forward
        assert_eq!(nav.avancar(), None);
        assert_eq!(nav.atual(), Some("z.com"));
    }
}
