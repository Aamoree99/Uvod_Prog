"""
Cvičení 03 — Úloha 3.2
Název: BMI
Body: 1 bod; 1 bod (v pořadí podúloh)

Zadání:
BMI (Body Mass Index) se počítá: váha v kilogramech vydělená druhou mocninou výšky v metrech.

a) Napište funkci, která pro zadanou hodnotu výšky a váhy spočte BMI. [1 bod]

b) Napište funkci, která místo číselné hodnoty bude vracet výčtový typ. [1 bod]


BMI                výčtový typ

[0; 16,5)          těžká podvýživa

[16,5; 18,5)     podváha

[18,5; 25)        optimální váha

[25; 30)           nadváha

[30; 35)           obezita prvního stupně

[35; 40)           obezita druhého stupně

[40; oo)           obezita třetího stupně


Ukázka:

x = SpoctiBMI1(1.70, 61.5); // 21.28

y = SpoctiBMI2(1.67, 80); // Nadvaha

Požadavky / omezení:
a) K řešení příkladů je možné používat pouze syntaktických konstrukcí a funkcí probraných na
přednáškách. K řešení cvičení s nižším číslem nelze používat učivo hodin s vyšším číslem.

b) Řešení musí být procedurální, nesmí být objektové. Je zakázáno použití cyklu foreach. Je
zakázáno použití kolekcí (Dictionary, List, LinkedList, SortedList, atd.) a jejich obdoby v jiných
programovacích jazycích.

c) Příklady můžete řešit i v jiném jazyku než C#, pokud tento jazyk umožňuje procedurální
programování, např. C, C++, Java. Při této volbě zdůrazňuji platnost pravidla 2a, lze používat
pouze ekvivalenty syntaktických konstrukcí a funkcí probraných na přednášce.

d) Úloha musí být funkční. Otestujte ji minimálně na datech uvedených v zadání příkladu.

e) Zdrojové kódy musí být formátovány jednotným způsobem, zejména odsazování.

f) Proměnné a funkce musejí mít vhodné názvy

g) Zdrojové kódy musí být vhodně členěny do funkcí (platí pro 3. cvičení a následující)

h) Pro každé cvičení vytvořte vlastní Solution (Rešení). Jednotlivé příklady budou jako samostatné
Projekty.

i) Je zakázáno používat složené příkazy { }, pokud obsahují pouze jeden příkaz.
"""

def calculate_bmi(height, weight):
    return weight / (height * height)


def classify_bmi(height, weight):
    bmi = calculate_bmi(height, weight)
    limits = (16.5, 18.5, 25, 30, 35, 40)
    names = ("severe underweight", "underweight", "normal weight", "overweight", "class I obesity", "class II obesity", "class III obesity")
    i = 0
    while i < len(limits):
        if bmi < limits[i]:
            return names[i]
        i += 1
    return names[6]


def main():
    print(f"BMI is {calculate_bmi(1.70, 61.5):.2f}.")
    print(f"BMI category: {classify_bmi(1.67, 80)}.")

if __name__ == "__main__":
    main()
