"""
Cvičení 03 — Úloha 3.5
Název: Trojúhelník v rovině
Body: 1 bod; 1 bod; 2 body; 1 bod; 2 body (v pořadí podúloh)

Zadání:
Vytvořte strukturu bod v rovině. Vytvořte strukturu trojúhelník, obsahující tři body. Za použití
euklidovské metriky napište následující funkce. V euklidovské metrice se vzdálenost mezi dvěma body
rovná odmocnině ze součtu druhých mocnin rozdílů odpovídajících souřadnic daných bodů.

a) Funkci, která porovná, zda dvě reálná čísla se v absolutní hodnotě liší nejvýše o 10^-13.  Tuto
funkci použijte kdykoliv porovnáváte dvě reálná čísla na rovnost. [1 bod]

b) Funkci, která spočte délku úsečky. [1 bod]

c) Funkci, která rozhodne, zda zadané tři body tvoří trojúhelník. Body tvoří trojúhelník, pokud
neleží na přímce. Tři body vytvoří tři úsečky. Body neleží na přímce, pokud součet délek dvou
kratších úseček je delší než délka nejdelší úsečky. [2 body]

d) Funkci, která spočte obvod trojúhelníku. Obvod trojúhelníku je roven součtu délek stran
trojúhelníku. [1 bod]

e) Funkci, která rozhodne, zda zadaný trojúhelník je pravoúhlý. Podle Pythagorovy věty se
v pravoúhlém trojúhelníku součet druhých mocnin kratších stran rovná druhé mocnině delší strany. [2
body]




Ukázka:

b1.x = 7.2; b1.y = -2;

b2.x = -2.9; b2.y = -6.23;

b3.x = 2; b3.y = 5;

b4.x = -1; b4.y = -2.5;

b5.x = 3; b5.y = 7.5;

b6.x = 3; b6.y = -2.6;

b7.x = 5.2; b7.y = -2.6;

t1.a = b1; t1.b = b2; t1.c = b3;

t2.a = b5; t2.b = b6; t2.c = b7;

je1 = PribliznaRovnost(5, 4.99999999999992); // true

delka = DelkaUsecky(b1, b2); // 10.95

je2 = JeTrojuhelnik(b1, b2, b3); // true

je3 = JeTrojuhelnik(b4, b3, b5); //false

obvod1 = ObvodTrojuhelniku(t1); // 31.923

obvod2 = ObvodTrojuhelniku(t2); // 22.637

je4 = TrojuhelnikJePravouhly(t2); // true

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

from math import sqrt


def approximately_equal(a, b):
    return abs(a - b) <= 1e-13


def segment_length(a, b):
    dx, dy = b[0] - a[0], b[1] - a[1]
    return sqrt(dx * dx + dy * dy)


def ordered_sides(triangle):
    a, b, c = triangle
    ab, bc, ca = segment_length(a, b), segment_length(b, c), segment_length(c, a)
    if ab >= bc and ab >= ca:
        return ab, bc, ca
    if bc >= ab and bc >= ca:
        return bc, ab, ca
    return ca, ab, bc


def is_triangle(a, b, c):
    longest, short1, short2 = ordered_sides((a, b, c))
    return short1 + short2 > longest and not approximately_equal(short1 + short2, longest)


def triangle_perimeter(triangle):
    longest, short1, short2 = ordered_sides(triangle)
    return longest + short1 + short2


def is_right_triangle(triangle):
    # Вырожденный треугольник не может быть прямоугольным.
    if not is_triangle(*triangle):
        return False
    longest, short1, short2 = ordered_sides(triangle)
    return approximately_equal(short1 * short1 + short2 * short2, longest * longest)


def main():
    b1, b2, b3 = (7.2, -2.0), (-2.9, -6.23), (2.0, 5.0)
    b4, b5, b6, b7 = (-1.0, -2.5), (3.0, 7.5), (3.0, -2.6), (5.2, -2.6)
    print(f"Approximate equality: {approximately_equal(5, 4.99999999999992)}")
    print(f"Length b1-b2: {segment_length(b1, b2):.3f}")
    print(f"b1, b2, b3 form a triangle: {is_triangle(b1, b2, b3)}")
    print(f"b4, b3, b5 form a triangle: {is_triangle(b4, b3, b5)}")
    print(f"Perimeter t1: {triangle_perimeter((b1, b2, b3)):.3f}")
    print(f"Perimeter t2: {triangle_perimeter((b5, b6, b7)):.3f}")
    print(f"t2 is right-angled: {is_right_triangle((b5, b6, b7))}")

if __name__ == "__main__":
    main()
