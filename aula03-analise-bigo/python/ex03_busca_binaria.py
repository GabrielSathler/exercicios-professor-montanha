def busca_binaria(lista, alvo):
    esquerda, direita = 0, len(lista) - 1
    while esquerda <= direita:
        # divisao inteira pra nao estourar indice
        meio = (esquerda + direita) // 2
        if lista[meio] == alvo:
            return meio
        elif lista[meio] < alvo:
            esquerda = meio + 1
        else:
            direita = meio - 1
    return -1


if __name__ == "__main__":
    ordenada = [1, 3, 5, 7, 9, 11, 13]
    print("Indice do 7:", busca_binaria(ordenada, 7))
    print("Indice do 8 (ausente):", busca_binaria(ordenada, 8))
