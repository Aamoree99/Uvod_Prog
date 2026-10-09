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

// Copy нужен, чтобы Point можно было передавать в функции несколько раз.
// Иначе Rust по умолчанию может считать значение "перемещённым".
#[derive(Clone, Copy)]
struct Point {
    // Координата X
    x: f64,

    // Координата Y
    y: f64,
}

// Треугольник состоит из трёх точек.
#[derive(Clone, Copy)]
struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

// a)
// Проверяем, считаются ли два вещественных числа равными.
//
// Например:
// 5.0
// 4.99999999999992
//
// Они немного отличаются,
// но разница меньше 10^-13.
fn approximately_equal(a: f64, b: f64) -> bool {
    // .abs() убирает знак.
    //
    // Например:
    // 5 - 4 = 1
    // 4 - 5 = -1
    //
    // abs(-1) = 1
    //
    // Поэтому порядок a и b не имеет значения.
    (a - b).abs() <= 1e-13
}

// b)
// Считаем расстояние между двумя точками.
fn segment_length(a: Point, b: Point) -> f64 {
    // Разница координат X.
    let difference_x = b.x - a.x;

    // Разница координат Y.
    let difference_y = b.y - a.y;

    // Формула:
    //
    // sqrt(dx² + dy²)
    //
    // sqrt = квадратный корень.
    (difference_x * difference_x + difference_y * difference_y).sqrt()
}

// c)
// Проверяем, образуют ли три точки треугольник.
fn is_triangle(a: Point, b: Point, c: Point) -> bool {
    // Считаем три стороны.
    let ab = segment_length(a, b);
    let bc = segment_length(b, c);
    let ca = segment_length(c, a);

    // Эти переменные пока только объявляем.
    // Значения дадим ниже.
    let longest;
    let short_side1;
    let short_side2;

    // Определяем, какая сторона самая длинная.

    if ab >= bc && ab >= ca {
        // AB самая длинная.
        longest = ab;

        // Оставшиеся две короткие.
        short_side1 = bc;
        short_side2 = ca;
    } else if bc >= ab && bc >= ca {
        // BC самая длинная.
        longest = bc;

        short_side1 = ab;
        short_side2 = ca;
    } else {
        // Если не AB и не BC,
        // значит самая длинная CA.
        longest = ca;

        short_side1 = ab;
        short_side2 = bc;
    }

    // Если:
    //
    // короткая1 + короткая2 == длинная
    //
    // точки находятся на одной прямой.
    //
    // Для сравнения double используем
    // нашу функцию approximately_equal().
    if approximately_equal(short_side1 + short_side2, longest) == true {
        return false;
    }

    // Для настоящего треугольника должно быть:
    //
    // короткая1 + короткая2 > длинная
    //
    // Эта строка одновременно является return,
    // потому что это последнее выражение без ;
    short_side1 + short_side2 > longest
}

// d)
// Считаем периметр треугольника.
fn triangle_perimeter(t: Triangle) -> f64 {
    // Считаем длину каждой стороны.
    let ab = segment_length(t.a, t.b);
    let bc = segment_length(t.b, t.c);
    let ca = segment_length(t.c, t.a);

    // Периметр = сумма трёх сторон.
    //
    // Опять без ; в конце,
    // поэтому значение возвращается из функции.
    ab + bc + ca
}

// e)
// Проверяем, является ли треугольник прямоугольным.
fn is_right_triangle(t: Triangle) -> bool {
    // Снова считаем три стороны.
    let ab = segment_length(t.a, t.b);
    let bc = segment_length(t.b, t.c);
    let ca = segment_length(t.c, t.a);

    let longest;
    let short_side1;
    let short_side2;

    // Снова ищем гипотенузу,
    // то есть самую длинную сторону.
    if ab >= bc && ab >= ca {
        longest = ab;
        short_side1 = bc;
        short_side2 = ca;
    } else if bc >= ab && bc >= ca {
        longest = bc;
        short_side1 = ab;
        short_side2 = ca;
    } else {
        longest = ca;
        short_side1 = ab;
        short_side2 = bc;
    }

    // Теорема Пифагора:
    //
    // a² + b² = c²
    //
    // Здесь short_side1 и short_side2 — катеты.
    let left_side = short_side1 * short_side1 + short_side2 * short_side2;

    // longest — гипотенуза.
    let right_side = longest * longest;

    // Нельзя нормально сравнивать f64 через ==,
    // поэтому опять используем нашу функцию.
    approximately_equal(left_side, right_side)
}

// Запускаем пример из упражнения и выводим результаты.
fn main() {
    // Создаём точки из задания.

    let b1 = Point {
        x: 7.2,
        y: -2.0_f64,
    };

    let b2 = Point {
        x: -2.9_f64,
        y: -6.23_f64,
    };

    let b3 = Point { x: 2.0, y: 5.0 };

    let b4 = Point {
        x: -1.0_f64,
        y: -2.5_f64,
    };

    let b5 = Point { x: 3.0, y: 7.5 };

    let b6 = Point {
        x: 3.0,
        y: -2.6_f64,
    };

    let b7 = Point {
        x: 5.2,
        y: -2.6_f64,
    };

    // Создаём первый треугольник.
    let t1 = Triangle {
        a: b1,
        b: b2,
        c: b3,
    };

    // Второй треугольник.
    let t2 = Triangle {
        a: b5,
        b: b6,
        c: b7,
    };

    // Проверяем пункт a).
    println!(
        "Approximate equality: {}",
        approximately_equal(5.0, 4.99999999999992)
    );

    // Проверяем длину отрезка.
    println!("Length b1-b2: {:.3}", segment_length(b1, b2));

    // Должно быть true.
    println!("b1, b2, b3 form a triangle: {}", is_triangle(b1, b2, b3));

    // Должно быть false.
    println!("b4, b3, b5 form a triangle: {}", is_triangle(b4, b3, b5));

    // Периметры.
    println!("Perimeter t1: {:.3}", triangle_perimeter(t1));

    println!("Perimeter t2: {:.3}", triangle_perimeter(t2));

    // Проверка прямоугольности.
    println!("t2 is right-angled: {}", is_right_triangle(t2));
}
