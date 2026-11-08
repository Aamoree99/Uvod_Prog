"""6. Palindrom

Napište je_palindrom(a). Použijte dva indexy pohybující se ke středu. 
Prázdné i jednoprvkové pole považujte za palindrom.

Pracujte v Pythonu. Logiku napište do funkcí, cykly veďte přes indexy. 
Nová pole vytvářejte s pevnou velikostí pomocí [0] * n. 
Nepoužívejte append, sum, max, min, sorted, .count, .index, slicing ani reversed.

Příklad: [1, 2, 3, 2, 1] → True; [1, 2, 2, 3] → False
"""

def palindrom(a):
    left = 0
    right = len(a) - 1

    while left < right:
        if a[left] != a[right]:
            return False

        left += 1
        right -=1

    return True

print(palindrom([1, 2, 3, 2, 1]))
print(palindrom([1, 2, 2, 3]))
print(palindrom([]))
print(palindrom([5]))