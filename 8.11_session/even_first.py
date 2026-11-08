"""8. Nejdřív sudá

Napište nejdriv_suda(a): vraťte nové pole, nejdřív sudá a pak lichá čísla. 
V obou skupinách zachovejte původní pořadí; vstup neměňte. Nejprve spočítejte sudé prvky.

Pracujte v Pythonu. Logiku napište do funkcí, cykly veďte přes indexy. 
Nová pole vytvářejte s pevnou velikostí pomocí [0] * n. 
Nepoužívejte append, sum, max, min, sorted, .count, .index, slicing ani reversed.

Příklad: [5, 2, 8, 1, 4, 7] → [2, 8, 4, 5, 1, 7]
"""

def even_first(pole):
    even_count = 0

    for i in range(len(pole)):
        if pole[i] % 2 == 0:
            even_count += 1

    result = [0] * len(pole)
    odd = 0
    even = even_count

    for i in range(len(pole)):
        if pole[i] % 2 == 0:
            result[odd] = pole[i]
            odd += 1
        else:
            result[even] = pole[i]
            even += 1

    return result


pole = [5, 2, 8, 1, 4, 7]
print(even_first(pole))
print(pole)