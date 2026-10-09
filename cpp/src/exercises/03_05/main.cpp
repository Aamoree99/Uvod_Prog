/*
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
*/

#include <iostream>
#include <iomanip>
#include <string>
#include <cmath>

// Возвращаем английское представление логического результата.
static std::string bool_text(bool boolean_result) { return boolean_result ? "true" : "false"; }
// Точка хранит две координаты; треугольник — три точки.
struct Point { double x; double y; };
struct Triangle { Point a; Point b; Point c; };
// Абсолютная погрешность позволяет сравнивать результаты вычислений double.
static bool approximately_equal(double a, double b) { return std::abs(a-b) <= 1e-13; }
// Расстояние между точками: квадратный корень из суммы квадратов разностей координат.
static double segment_length(Point a, Point b) {
    double difference_x = b.x - a.x, difference_y = b.y - a.y;
    return std::sqrt(difference_x*difference_x + difference_y*difference_y);
}
// Проверяем, образуют ли точки a, b и c невырожденный треугольник.
static bool is_triangle(Point a, Point b, Point c) {
    double ab = segment_length(a,b), bc = segment_length(b,c), ca = segment_length(c,a);
    double longest, short_side1, short_side2;
    // Выбираем самую длинную сторону; остальные две должны в сумме быть больше неё.
    if (ab >= bc && ab >= ca) { longest=ab; short_side1=bc; short_side2=ca; }
    else if (bc >= ab && bc >= ca) { longest=bc; short_side1=ab; short_side2=ca; }
    else { longest=ca; short_side1=ab; short_side2=bc; }
    // Почти равная сумма означает три точки на одной прямой.
    if (approximately_equal(short_side1+short_side2, longest)) return false;
    return short_side1+short_side2 > longest;
}
// Периметр — сумма трёх сторон.
static double triangle_perimeter(Triangle t) { return segment_length(t.a,t.b)+segment_length(t.b,t.c)+segment_length(t.c,t.a); }
// Проверяем равенство суммы квадратов коротких сторон квадрату длинной.
static bool is_right_triangle(Triangle t) {
    double ab=segment_length(t.a,t.b), bc=segment_length(t.b,t.c), ca=segment_length(t.c,t.a);
    double longest, short_side1, short_side2;
    if (ab >= bc && ab >= ca) { longest=ab; short_side1=bc; short_side2=ca; }
    else if (bc >= ab && bc >= ca) { longest=bc; short_side1=ab; short_side2=ca; }
    else { longest=ca; short_side1=ab; short_side2=bc; }
    // Проверяем теорему Пифагора с той же абсолютной погрешностью, что в Rust.
    return approximately_equal(short_side1*short_side1+short_side2*short_side2, longest*longest);
}
// Запускаем пример из упражнения и выводим результаты.
int main() {
    std::cout << std::boolalpha;
    Point b1=Point{7.2,-2.0}, b2=Point{-2.9,-6.23}, b3=Point{2.0,5.0};
    Point b4=Point{-1.0,-2.5}, b5=Point{3.0,7.5}, b6=Point{3.0,-2.6}, b7=Point{5.2,-2.6};
    Triangle t1=Triangle{b1,b2,b3}, t2=Triangle{b5,b6,b7};
    std::cout << "Approximate equality: " << bool_text(approximately_equal(5.0,4.99999999999992)) << '\n';
    std::cout << "Length b1-b2: " << std::fixed << std::setprecision(3) << segment_length(b1,b2) << std::defaultfloat << '\n';
    std::cout << "b1, b2, b3 form a triangle: " << bool_text(is_triangle(b1,b2,b3)) << '\n';
    std::cout << "b4, b3, b5 form a triangle: " << bool_text(is_triangle(b4,b3,b5)) << '\n';
    std::cout << "Perimeter t1: " << std::fixed << std::setprecision(3) << triangle_perimeter(t1) << std::defaultfloat << '\n'; std::cout << "Perimeter t2: " << std::fixed << std::setprecision(3) << triangle_perimeter(t2) << std::defaultfloat << '\n';
    std::cout << "t2 is right-angled: " << bool_text(is_right_triangle(t2)) << '\n';
}
