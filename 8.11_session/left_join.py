"""Úloha: Posun doleva

Posuňte všechny prvky neprázdného pole cyklicky o jednu pozici doleva.

Napište samostatnou funkci s parametry. 
Nepoužívejte foreach, kolekce, Console, Math ani jiné hotové knihovní algoritmy. 
Pracujte s pevně dlouhým polem int[] či bool[] a se základními cykly a podmínkami.

Níže jsou tři zkušební příklady. Před potvrzením vyzkoušejte všechny:

Příklad 1 — vstup:
pole = [-5, 0, 4, -2, 4, 9, 0, 3]
Očekávaný výsledek:
```text
[0, 4, -2, 4, 9, 0, 3, -5]
```

Příklad 2 — vstup:
pole = [7, 7, -1, 2, 0, 7, 2]
Očekávaný výsledek:
```text
[7, -1, 2, 0, 7, 2, 7]
```

Příklad 3 — vstup:
pole = [-3, -1, -6, -2, 0]
Očekávaný výsledek:
```text
[-1, -6, -2, 0, -3]
```"""

def left_join(pole):
    first = pole[0]

    for i in range(len(pole) - 1): #for(int i = 0; i < pole.Lenght;i++)
        pole[i] = pole[i+1] 

    pole[len(pole)-1] = first #pole[pole.Lenght - 1] = first
    



a = [-5, 0, 4, -2, 4, 9, 0, 3]
left_join(a)
print(a)  # [0, 4, -2, 4, 9, 0, 3, -5]

b = [7, 7, -1, 2, 0, 7, 2]
left_join(b)
print(b)  # [7, -1, 2, 0, 7, 2, 7]

c = [-3, -1, -6, -2, 0]
left_join(c)
print(c)  # [-1, -6, -2, 0, -3]