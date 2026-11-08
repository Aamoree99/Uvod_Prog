"""Úloha: Největší prvek

Vraťte největší hodnotu v neprázdném poli.

Napište samostatnou funkci s parametry. Nepoužívejte foreach, kolekce, Console, Math ani jiné hotové knihovní algoritmy. 
Pracujte s pevně dlouhým polem int[] či bool[] a se základními cykly a podmínkami.

Níže jsou tři zkušební příklady. Před potvrzením vyzkoušejte všechny:

Příklad 1 — vstup:
pole = [-5, 0, 4, -2, 4, 9, 0, 3]
Očekávaný výsledek:
```text
9
```

Příklad 2 — vstup:
pole = [7, 7, -1, 2, 0, 7, 2]
Očekávaný výsledek:
```text
7
```

Příklad 3 — vstup:
pole = [-3, -1, -6, -2, 0]
Očekávaný výsledek:
```text
0
```"""

def maximum(pole):
    max = pole[0]

    for i in range(1, len(pole)):
        if pole[i] > max:
            max = pole[i]

    return max

print(maximum([-5, 0, 4, -2, 4, 9, 0, 3]))  # 9
print(maximum([7, 7, -1, 2, 0, 7, 2]))      # 7
print(maximum([-3, -1, -6, -2, 0]))         # 0