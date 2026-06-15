def fibonacci_recursivo(n):
    if n <= 1:
        return n
    return fibonacci_recursivo(n - 1) + fibonacci_recursivo(n - 2)


if __name__ == "__main__":
    print("fibonacci_recursivo(10):", fibonacci_recursivo(10))
