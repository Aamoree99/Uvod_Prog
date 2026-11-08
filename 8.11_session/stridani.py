"""Úloha: Střídání hodnot bool

Vraťte true právě tehdy, když se v poli bool[] střídají true a false mezi každými sousedy.

Napište samostatnou funkci s parametry. Nepoužívejte foreach, kolekce, Console, Math ani jiné hotové knihovní algoritmy. Pracujte s pevně dlouhým polem int[] či bool[] a se základními cykly a podmínkami.

Níže jsou tři zkušební příklady. Před potvrzením vyzkoušejte všechny:

Příklad 1 — vstup:
pole = [true, false, true, true, false]
Očekávaný výsledek:
```text
false
```

Příklad 2 — vstup:
pole = [false, true, false, true]
Očekávaný výsledek:
```text
true
```

Příklad 3 — vstup:
pole = [false, false, false]
Očekávaný výsledek:
```text
false
```"""

def stridani_bool(pole):
    #for(int i = 0; i < pole.Lenght; i ++)
    for i in range(len(pole) - 2): #-2 потмоу что сравниваем i и i+1. Что бы не выйти за рамки массива
        if pole[i] == pole[i+1]:
            return False
    return True

print(stridani_bool([True, False, True, True, False])) #False
print(stridani_bool([False, True, False, True])) #TRrue
print(stridani_bool([False, False, False])) #False