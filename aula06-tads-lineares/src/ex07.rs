pub struct Editor {
    texto: String,
    desfazer: Vec<String>,
    refazer: Vec<String>,
}

impl Editor {
    pub fn novo() -> Self {
        Editor {
            texto: String::new(),
            desfazer: Vec::new(),
            refazer: Vec::new(),
        }
    }

    pub fn digitar(&mut self, s: &str) {
        // salva o estado antes de mexer e zera o refazer
        self.desfazer.push(self.texto.clone());
        self.refazer.clear();
        self.texto.push_str(s);
    }

    pub fn desfazer(&mut self) {
        if let Some(anterior) = self.desfazer.pop() {
            self.refazer.push(self.texto.clone());
            self.texto = anterior;
        }
    }

    pub fn refazer(&mut self) {
        if let Some(proximo) = self.refazer.pop() {
            self.desfazer.push(self.texto.clone());
            self.texto = proximo;
        }
    }

    pub fn texto(&self) -> &str {
        &self.texto
    }
}

pub fn demo() {
    let mut ed = Editor::novo();
    ed.digitar("Olá");
    ed.digitar(", mundo");
    println!("  texto         = {:?}", ed.texto());
    ed.desfazer();
    println!("  após desfazer = {:?}", ed.texto());
    ed.refazer();
    println!("  após refazer  = {:?}", ed.texto());
    ed.digitar("!");
    println!("  texto final   = {:?}", ed.texto());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digitar_e_desfazer() {
        let mut ed = Editor::novo();
        ed.digitar("abc");
        ed.digitar("def");
        assert_eq!(ed.texto(), "abcdef");
        ed.desfazer();
        assert_eq!(ed.texto(), "abc");
        ed.desfazer();
        assert_eq!(ed.texto(), "");
    }

    #[test]
    fn refazer_restaura() {
        let mut ed = Editor::novo();
        ed.digitar("abc");
        ed.desfazer();
        ed.refazer();
        assert_eq!(ed.texto(), "abc");
    }

    #[test]
    fn digitar_limpa_refazer() {
        let mut ed = Editor::novo();
        ed.digitar("a");
        ed.desfazer(); // texto = "", refazer = ["a"]
        ed.digitar("z"); // deve limpar refazer
        ed.refazer(); // nada para refazer
        assert_eq!(ed.texto(), "z");
    }

    #[test]
    fn desfazer_sem_historico_nao_quebra() {
        let mut ed = Editor::novo();
        ed.desfazer();
        ed.refazer();
        assert_eq!(ed.texto(), "");
    }
}
