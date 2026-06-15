def somar_lista(lista):
    total = 0
    for elemento in lista:
        total += elemento
    return total


if __name__ == "__main__":
    exemplo = [1, 2, 3, 4, 5]
    print("Soma da lista:", somar_lista(exemplo))
