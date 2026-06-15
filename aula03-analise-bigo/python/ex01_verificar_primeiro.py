def verificar_primeiro(lista):
    if len(lista) == 0:
        return None
    return lista[0]


if __name__ == "__main__":
    exemplo = [10, 20, 30, 40]
    print("Primeiro elemento:", verificar_primeiro(exemplo))
    print("Lista vazia:", verificar_primeiro([]))
