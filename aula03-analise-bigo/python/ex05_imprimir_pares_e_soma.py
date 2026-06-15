def imprimir_pares_e_soma(lista):
    for i in range(len(lista)):
        print(lista[i])
    for i in range(len(lista)):
        for j in range(len(lista)):
            print(lista[i], lista[j])


if __name__ == "__main__":
    exemplo = [1, 2, 3]
    imprimir_pares_e_soma(exemplo)
