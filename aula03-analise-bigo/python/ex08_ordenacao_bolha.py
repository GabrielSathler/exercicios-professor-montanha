def ordenacao_bolha(lista):
    n = len(lista)
    for i in range(n):
        for j in range(0, n - i - 1):
            if lista[j] > lista[j + 1]:
                lista[j], lista[j + 1] = lista[j + 1], lista[j]
    return lista


if __name__ == "__main__":
    exemplo = [5, 2, 9, 1, 7, 3]
    print("Lista ordenada:", ordenacao_bolha(exemplo))
