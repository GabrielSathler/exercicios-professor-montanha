def pares_com_soma(lista, alvo):
    for i in range(len(lista)):
        for j in range(i + 1, len(lista)):
            if lista[i] + lista[j] == alvo:
                print(lista[i], lista[j])


if __name__ == "__main__":
    exemplo = [1, 2, 3, 4, 5]
    print("Pares cuja soma e 6:")
    pares_com_soma(exemplo, 6)
