"""
Cvičení 03 — Úloha 3.1
Název: DPH
Body: 1 bod; 1 bod (v pořadí podúloh)

Zadání:
a) Napište funkci, která pro zadanou cenu zboží a číselně zadanou sazbu DPH spočte cenu zboží
s DPH. [1 bod]

b) Napište funkci, která pro sazbu DPH použije výčtový typ. Existují tři sazby DPH: základní 21%,
první snížená 15% a druhá snížená 10% [1 bod]:


Ukázka:

x = SpoctiDPH1(56.20, 21); // 68.002

y = SpoctiDPH2(115, DPH.PrvniSnizena); // 132.25

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

def price_with_vat(price, vat):
    return price + price * vat / 100.0


def price_with_vat_rate(price, rate):
    # Ставка представлена индексом в массиве вместо объекта перечисления.
    rates = (21.0, 15.0, 10.0)
    return price_with_vat(price, rates[rate])


def main():
    print(f"The price 56.20 CZK with 21 % VAT is {price_with_vat(56.20, 21):.2f} CZK.")
    i = 0
    while i < 3:
        print(f"The price 115.00 CZK with VAT rate {i} is {price_with_vat_rate(115, i):.2f} CZK.")
        i += 1

if __name__ == "__main__":
    main()
