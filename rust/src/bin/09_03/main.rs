/*
Cvičení 09 — Úloha 9.3
Název: ANSII Art
Body: 1 bod; 1 bod; 1 bod; 1 bod; 1 bod; 1 bod; 1 bod; 1 bod; 0 bodů; 1 bod; 1 bod; 1 bod; 2 body; 3 body; 2 body; 2 body (v pořadí podúloh)

Zadání:
Pomocí znaků hvězdičky '*' a mezery ' ' vytvořte následující obrazce pro zadanou délku hrany
obrazce. Délka hrany obrazce bude liché číslo. Vzhledem k rozdílné výšce a šířce znaků budou
obrazce oproti předloze vypadat mírně deformované. Složitější obrazce lze vytvořit postupy, které
se naučíte na jednodušších obrazcích.

a) Obrazec z předlohy na pozici [0,0] – okraje čtverce. [1 bod]

b) Obrazec z předlohy na pozici [0,1] – okraje čtverce a svislá střední příčka. [1 bod]

c) Obrazec z předlohy na pozici [0,2] – okraje čtverce a úhlopříčka z levého horního rohu do
pravého dolního rohu. [1 bod]

d) Obrazec z předlohy na pozici [0,3] – okraje čtverce a obě dvě úhlopříčky. [1 bod]

e) Obrazec z předlohy na pozici [1,0] – okraje čtverce a obě dvě střední příčky. [1 bod]

f) Obrazec z předlohy na pozici [1,1] – okraje čtverce a obě dvě střední příčky a obě dvě
úhlopříčky. [1 bod]

g) Obrazec z předlohy na pozici [1, 2] – Okraje čtverce a ve čtverci se střídají prázdné a plné
řádky. [1 bod]

h) Obrazec z předlohy na pozici [1, 3] – Mřížka. [1 bod]

i) Obrazec z předlohy na pozici [2, 0] – Střídání plných a prázdných soustředných čtverců. Obrazec
byl probírán na přednášce, proto není bodován. [0 bodů]

j) Obrazec z předlohy na pozici [2, 1] – Pravá horní polovina čtverce. [1 bod]

k) Obrazec z předlohy na pozici [2, 2] – Horní čtvrtina čtverce. [1 bod]

l) Obrazec z předlohy na pozici [2, 3] – Přesýpací hodiny. [1 bod]

m) Obrazec z předlohy na pozici [3, 0] – Složený obrazec ze 4 samostatných [2 body]

n) Obrazec z předlohy na pozici [3, 1] – Složený obrazec ze 4 samostatných [3 body]

o) Obrazec z předlohy na pozici [3, 2] – Složený obrazec ze 4 samostatných [2 body]

p) Obrazec z předlohy na pozici [3, 3] – Složený obrazec ze 4 samostatných [2 body]



Textový popis obrazové předlohy ze strany 30:
Předloha obsahuje 4 řádky a 4 sloupce obrazců. Pozice [řádek, sloupec]
jsou číslovány od nuly. Černé části odpovídají hvězdičkám, bílé mezerám.
Pozice [0,0] až [2,3] jsou popsány v podúlohách a) až l).

[3,0]: Čtyři čtverce ve dvou řádcích a dvou sloupcích. V každém z nich
se střídají plné a prázdné soustředné čtverce jako na pozici [2,0].

[3,1]: Čtyři čtverce ve dvou řádcích a dvou sloupcích. Levý horní má
okraje a obě úhlopříčky, pravý horní má okraje a střídající se plné
a prázdné svislé pruhy, levý dolní obsahuje mřížku a pravý dolní
střídající se plné a prázdné soustředné čtverce.

[3,2]: Čtyři prázdné čtverce s plnými okraji ve dvou řádcích a dvou
sloupcích. Mezi sousedními čtverci jsou bílé mezery; uprostřed obrazce
je tak dvojice svislých a dvojice vodorovných okrajů oddělených mezerou.

[3,3]: Vnější plný čtvercový okraj. Uvnitř jsou čtyři plné čtverce
ve dvou řádcích a dvou sloupcích, oddělené bílou svislou a vodorovnou
střední příčkou. Od vnějšího okraje jsou odděleny bílým lemem.

Ukázka:

char[,] obrA = Obrazec00(25);

char[,] obrB = Obrazec01(25);

//...

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

// --------------------------------------------------
// BASIC SETTINGS
// --------------------------------------------------

// Размер буфера для хранения рисунка.
//
// В Rust размер обычного массива фиксирован.
// Используем буфер 101x101, но выводим только
// реально используемую часть.
//
// Для составных рисунков требуется 2 * size + 1.
const BUFFER_SIZE: usize = 101;

type Picture = [[char; BUFFER_SIZE]; BUFFER_SIZE];

// --------------------------------------------------
// A-L) BASIC PATTERNS
// --------------------------------------------------

// Определяем, должна ли в заданной позиции
// находиться звёздочка.
//
// kind — номер рисунка:
// 0 = a, 1 = b, ... 11 = l.
//
// kind = 12 используется для вертикальных полос
// в составном рисунке n.
//
// Функция ничего не рисует сама.
// Она только возвращает true или false.
fn is_star(kind: usize, row: usize, col: usize, size: usize) -> bool {
    // Индекс центральной строки и столбца.
    let middle = size / 2;

    // Находим, является ли ячейка частью рамки.
    let border = row == 0 || col == 0 || row == size - 1 || col == size - 1;

    // Находим расстояние до ближайшего края.
    //
    // Оно понадобится для концентрических квадратов.
    let mut distance = row;

    if col < distance {
        distance = col;
    }

    if size - 1 - row < distance {
        distance = size - 1 - row;
    }

    if size - 1 - col < distance {
        distance = size - 1 - col;
    }

    match kind {
        // A) Только рамка квадрата.
        0 => border,

        // B) Рамка + вертикальная линия посередине.
        1 => border || col == middle,

        // C) Рамка + диагональ сверху слева вниз вправо.
        2 => border || row == col,

        // D) Рамка + две диагонали.
        3 => border || row == col || row + col == size - 1,

        // E) Рамка + вертикальная и горизонтальная линии.
        4 => border || row == middle || col == middle,

        // F) Рамка + обе средние линии + обе диагонали.
        5 => border || row == middle || col == middle || row == col || row + col == size - 1,

        // G) Рамка + чередующиеся полные и пустые строки.
        6 => border || row % 2 == 0,

        // H) Сетка из горизонтальных и вертикальных линий.
        7 => row % 2 == 0 || col % 2 == 0,

        // I) Концентрические заполненные и пустые квадраты.
        //
        // Ближайший край на расстоянии:
        // 0 — звёздочки;
        // 1 — пробелы;
        // 2 — звёздочки;
        // 3 — пробелы.
        8 => distance % 2 == 0,

        // J) Верхняя правая половина квадрата.
        //
        // Всё, что находится на главной диагонали
        // или справа/сверху от неё.
        9 => col >= row,

        // K) Верхняя четверть:
        // треугольник от верхнего края к центру.
        10 => row <= middle && col >= row && row + col < size,

        // L) Песочные часы.
        //
        // Верхняя половина сужается к центру,
        // нижняя расширяется обратно.
        11 => {
            if row <= middle {
                col >= row && row + col < size
            } else {
                col <= row && row + col >= size - 1
            }
        }

        // Вспомогательный рисунок:
        // рамка + чередующиеся вертикальные полосы.
        12 => border || col % 2 == 0,

        // Неизвестный тип рисунка.
        _ => false,
    }
}

// --------------------------------------------------
// M-P) COMPOSITE PATTERNS
// --------------------------------------------------

// Определяем звёздочки составных рисунков.
//
// В каждом составном рисунке четыре части,
// расположенные квадратом 2x2.
//
// Между частями оставляем один пустой ряд
// и один пустой столбец.
fn is_composite_star(kind: usize, row: usize, col: usize, size: usize) -> bool {
    // Размер составного рисунка.
    let total = 2 * size + 1;

    // ------------------------------------------
    // P) OUTER BORDER AND FOUR FILLED SQUARES
    // ------------------------------------------

    if kind == 15 {
        // Внешняя рамка полностью заполнена.
        if row == 0 || col == 0 || row == total - 1 || col == total - 1 {
            return true;
        }

        // Между внешней рамкой и квадратами
        // находится пустой отступ в один символ.
        //
        // Центральная строка и столбец
        // также полностью пустые.
        if row == 1
            || col == 1
            || row == total - 2
            || col == total - 2
            || row == size
            || col == size
        {
            return false;
        }

        // Остальное пространство внутри занято
        // четырьмя заполненными квадратами.
        return true;
    }

    // Центральные разделители между квадратами.
    if row == size || col == size {
        return false;
    }

    // Пересчитываем координаты составного рисунка
    // в координаты одного маленького квадрата.
    //
    // Например, нижний квадрат начинается
    // со строки size + 1.
    let local_row = if row > size { row - size - 1 } else { row };

    let local_col = if col > size { col - size - 1 } else { col };

    // ------------------------------------------
    // M) FOUR CONCENTRIC SQUARE PATTERNS
    // ------------------------------------------

    if kind == 12 {
        // Все четыре квадрата одинаковые.
        return is_star(8, local_row, local_col, size);
    }

    // ------------------------------------------
    // N) FOUR DIFFERENT PATTERNS
    // ------------------------------------------

    if kind == 13 {
        // Верхний левый:
        // рамка с обеими диагоналями.
        if row < size && col < size {
            return is_star(3, local_row, local_col, size);
        }

        // Верхний правый:
        // рамка с вертикальными полосами.
        if row < size && col > size {
            return is_star(12, local_row, local_col, size);
        }

        // Нижний левый:
        // сетка.
        if row > size && col < size {
            return is_star(7, local_row, local_col, size);
        }

        // Нижний правый:
        // концентрические квадраты.
        return is_star(8, local_row, local_col, size);
    }

    // ------------------------------------------
    // O) FOUR EMPTY SQUARES WITH BORDERS
    // ------------------------------------------

    if kind == 14 {
        return is_star(0, local_row, local_col, size);
    }

    false
}

// --------------------------------------------------
// CREATE PICTURE
// --------------------------------------------------

// Создаём двумерный массив символов.
//
// size — длина стороны маленького квадрата.
// kind — номер рисунка от 0 до 15.
//
// Для a-l результат имеет размер size x size.
//
// Для m-p результат имеет размер:
// (2 * size + 1) x (2 * size + 1).
fn create_picture(size: usize, kind: usize) -> Picture {
    // По условию длина стороны должна быть нечётной.
    assert!(size >= 3, "Size must be at least 3");
    assert!(size % 2 == 1, "Size must be odd");

    // Буфер должен вмещать и составные рисунки.
    assert!(2 * size + 1 <= BUFFER_SIZE, "Picture is too large");

    assert!(kind < 16, "Invalid picture type");

    // Создаём полностью пустой рисунок.
    let mut picture = [[' '; BUFFER_SIZE]; BUFFER_SIZE];

    // Для составных рисунков используем
    // удвоенную сторону с разделителем.
    let total = if kind >= 12 { 2 * size + 1 } else { size };

    // Перебираем каждую строку рисунка.
    let mut row = 0;

    while row < total {
        // Начинаем с первого столбца.
        let mut col = 0;

        while col < total {
            // Определяем, должна ли здесь быть звёздочка.
            let filled = if kind < 12 {
                is_star(kind, row, col, size)
            } else {
                is_composite_star(kind, row, col, size)
            };

            // Если условие выполнено,
            // заменяем пробел звёздочкой.
            if filled {
                picture[row][col] = '*';
            }

            // Следующий столбец.
            col += 1;
        }

        // Следующая строка.
        row += 1;
    }

    // Возвращаем готовый рисунок.
    picture
}

// --------------------------------------------------
// INDIVIDUAL FUNCTIONS A-P
// --------------------------------------------------

// Каждая функция создаёт конкретный рисунок.
//
// Используем одну общую реализацию,
// чтобы не копировать 16 раз одинаковые циклы.

fn obrazec_00(size: usize) -> Picture {
    create_picture(size, 0)
}

fn obrazec_01(size: usize) -> Picture {
    create_picture(size, 1)
}

fn obrazec_02(size: usize) -> Picture {
    create_picture(size, 2)
}

fn obrazec_03(size: usize) -> Picture {
    create_picture(size, 3)
}

fn obrazec_10(size: usize) -> Picture {
    create_picture(size, 4)
}

fn obrazec_11(size: usize) -> Picture {
    create_picture(size, 5)
}

fn obrazec_12(size: usize) -> Picture {
    create_picture(size, 6)
}

fn obrazec_13(size: usize) -> Picture {
    create_picture(size, 7)
}

fn obrazec_20(size: usize) -> Picture {
    create_picture(size, 8)
}

fn obrazec_21(size: usize) -> Picture {
    create_picture(size, 9)
}

fn obrazec_22(size: usize) -> Picture {
    create_picture(size, 10)
}

fn obrazec_23(size: usize) -> Picture {
    create_picture(size, 11)
}

fn obrazec_30(size: usize) -> Picture {
    create_picture(size, 12)
}

fn obrazec_31(size: usize) -> Picture {
    create_picture(size, 13)
}

fn obrazec_32(size: usize) -> Picture {
    create_picture(size, 14)
}

fn obrazec_33(size: usize) -> Picture {
    create_picture(size, 15)
}

// --------------------------------------------------
// PRINT PICTURE
// --------------------------------------------------

// Печатаем рисунок в терминал.
//
// total — сколько строк и столбцов
// действительно нужно вывести.
//
// Пробелы сохраняются, чтобы рисунки
// имели правильную форму.
fn print_picture(name: &str, picture: &Picture, total: usize) {
    println!();
    println!("{}", name);
    println!("----------------------------------------");

    let mut row = 0;

    while row < total {
        let mut col = 0;

        while col < total {
            print!("{}", picture[row][col]);

            col += 1;
        }

        println!();
        row += 1;
    }
}

// --------------------------------------------------
// MAIN
// --------------------------------------------------

fn main() {
    // Длина стороны из примера преподавателя.
    let size = 25;

    // Размер составных рисунков.
    let composite_size = 2 * size + 1;

    println!("ASCII ART PATTERNS");
    println!("========================================");
    println!("Square size: {}", size);

    // Первая строка рисунков.
    print_picture("A) [0,0] Square border", &obrazec_00(size), size);
    print_picture("B) [0,1] Vertical center", &obrazec_01(size), size);
    print_picture("C) [0,2] Main diagonal", &obrazec_02(size), size);
    print_picture("D) [0,3] Both diagonals", &obrazec_03(size), size);

    // Вторая строка рисунков.
    print_picture("E) [1,0] Center cross", &obrazec_10(size), size);
    print_picture("F) [1,1] Cross and diagonals", &obrazec_11(size), size);
    print_picture("G) [1,2] Alternating rows", &obrazec_12(size), size);
    print_picture("H) [1,3] Grid", &obrazec_13(size), size);

    // Третья строка рисунков.
    print_picture("I) [2,0] Concentric squares", &obrazec_20(size), size);
    print_picture("J) [2,1] Upper right half", &obrazec_21(size), size);
    print_picture("K) [2,2] Upper quarter", &obrazec_22(size), size);
    print_picture("L) [2,3] Hourglass", &obrazec_23(size), size);

    // Четвёртая строка — составные рисунки.
    print_picture(
        "M) [3,0] Four concentric patterns",
        &obrazec_30(size),
        composite_size,
    );

    print_picture(
        "N) [3,1] Four different patterns",
        &obrazec_31(size),
        composite_size,
    );

    print_picture(
        "O) [3,2] Four bordered squares",
        &obrazec_32(size),
        composite_size,
    );

    print_picture(
        "P) [3,3] Four filled squares",
        &obrazec_33(size),
        composite_size,
    );

    println!();
    println!("All 16 patterns created successfully.");
}
