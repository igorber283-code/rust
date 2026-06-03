inp = int(input("Введите чилсо"))
a = 1
b = 1
for _ in range(10):
    a, b = b, a + b
    print(b)
