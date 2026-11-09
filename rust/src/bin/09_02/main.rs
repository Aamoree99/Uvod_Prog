/*
Cvičení 09 — Úloha 9.2
Název: Gaussova eliminační metoda
Body: 6 bodů; 3 body; 1 bod (v pořadí podúloh)

Zadání:
Napište následující funkce pracující s maticí celých čísel long:

a) Napište funkci OdstupnujMatici, která pomocí elementárních řádkových úprav převede zadanou
matici na odstupňovaný tvar. [6 bodů]

b) Napište funkci GaussovaEliminace, která za pomocí funkce OdstupnujMatici z matice reprezentující
nehomogenní soustavu lineárních rovnic vypočte hodnoty jednotlivých neznámých a vrátí je poli
reálných čísel. V případě, že soustava nemá řešení, je vráceno pole nulové délky. V případě, že
soustava má nekonečně mnoho řešení, nastavte volitelné proměnné na hodnoty 0. [3 body]

c) Napište funkci Zkouska. Funkce nejprve dosadí hodnoty ze zadaného pole reprezentujícího hodnoty
neznámých do zadané matice reprezentující nehomogenní soustavu lineárních rovnic. Funkce ověří, zda
pro každou rovnici platí rovnost levé a pravé strany. Tj. hodnoty neznámých nacházející se
v zadaném poli jsou řešením nehomogenní soustavy lineárních rovnic reprezentované zadanou maticí.
[1 bod]


Ukázka:

long[,] M = { { 0, 0, 3, 2 }, { 0, 2, 4, 3 }, { 0, 4, 8, 6 }};

OdstupnujMatici(M);

// { { 0, 2, 4, 3 }, { 0, 0, 3, 2 }, { 0, 0, 0, 0 } };

double[] y = GaussovaEliminace(M); // 0.166666, 0.666666

bool b = Zkouska(M, y); // true

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
// MATRIX TYPE
// --------------------------------------------------

// Как в 09_01, используем массив 10 на 10.
// i64 соответствует long из C#.
//
// rows и cols будут хранить реальные размеры.
type Matrix = [[i64; 10]; 10];

// --------------------------------------------------
// HELPER: PRINT MATRIX
// --------------------------------------------------

// Выводим только используемую часть матрицы.
fn print_matrix(matrix: &Matrix, rows: usize, cols: usize) {
    let mut row = 0;

    // Перебираем строки.
    while row < rows {
        let mut col = 0;

        // Перебираем столбцы текущей строки.
        while col < cols {
            // Выводим элемент, выделяя 5 позиций.
            print!("{:5}", matrix[row][col]);

            col += 1;
        }

        // Завершаем строку матрицы.
        println!();

        row += 1;
    }
}

// --------------------------------------------------
// HELPER: GREATEST COMMON DIVISOR
// --------------------------------------------------

// Находим НОД двух чисел.
//
// Он нужен, чтобы уменьшать множители
// при исключении неизвестных.
//
// Например:
// gcd(2, 4) = 2.
fn gcd(mut a: i128, mut b: i128) -> i128 {
    // Работаем с абсолютными значениями.
    if a < 0 {
        a = -a;
    }

    if b < 0 {
        b = -b;
    }

    // Обычный алгоритм Евклида.
    while b != 0 {
        let remainder = a % b;

        a = b;
        b = remainder;
    }

    a
}

// --------------------------------------------------
// A) ECHELON MATRIX — 6 POINTS
// --------------------------------------------------

// Приводим матрицу к ступенчатому виду.
//
// matrix — исходная матрица, которую изменяем.
// rows — количество строк.
// cols — количество столбцов.
//
// Используем элементарные преобразования строк:
// 1. Перестановка двух строк.
// 2. Умножение строки на ненулевой множитель.
// 3. Прибавление к строке другой строки
//    с некоторым коэффициентом.
//
// Все значения остаются целыми числами.
fn echelon_matrix(matrix: &mut Matrix, rows: usize, cols: usize) {
    // Проверяем размеры матрицы.
    assert!(rows > 0 && rows <= 10);
    assert!(cols > 0 && cols <= 10);

    // Строка, в которой ищем очередной ведущий элемент.
    let mut pivot_row = 0;

    // Столбец, который сейчас обрабатываем.
    let mut pivot_col = 0;

    // Продолжаем, пока есть строки и столбцы.
    while pivot_row < rows && pivot_col < cols {
        // Предполагаем, что ведущий элемент
        // находится в текущей строке.
        let mut selected_row = pivot_row;

        // Если текущий элемент равен нулю,
        // ищем ненулевой элемент ниже.
        while selected_row < rows && matrix[selected_row][pivot_col] == 0 {
            selected_row += 1;
        }

        // Если весь остаток столбца состоит из нулей,
        // пропускаем этот столбец.
        if selected_row == rows {
            pivot_col += 1;
            continue;
        }

        // Если ненулевой элемент нашли в другой строке,
        // меняем строки местами.
        if selected_row != pivot_row {
            let temporary = matrix[pivot_row];

            matrix[pivot_row] = matrix[selected_row];
            matrix[selected_row] = temporary;
        }

        // Получаем ведущий элемент (pivot).
        let pivot = matrix[pivot_row][pivot_col] as i128;

        // Начинаем обрабатывать строки ПОД ведущей.
        let mut row = pivot_row + 1;

        while row < rows {
            // Элемент, который хотим превратить в ноль.
            let factor = matrix[row][pivot_col] as i128;

            // Если он уже нулевой, ничего не делаем.
            if factor != 0 {
                // Находим НОД множителей,
                // чтобы не увеличивать числа без необходимости.
                let divisor = gcd(pivot, factor);

                // Множитель текущей строки.
                let row_multiplier = pivot / divisor;

                // Множитель ведущей строки.
                let pivot_multiplier = factor / divisor;

                // Перебираем элементы текущей строки.
                let mut col = pivot_col;

                while col < cols {
                    // Новая строка:
                    //
                    // current = current * row_multiplier
                    //         - pivot_row * pivot_multiplier
                    //
                    // Это делает элемент под pivot равным нулю.
                    let value = matrix[row][col] as i128 * row_multiplier
                        - matrix[pivot_row][col] as i128 * pivot_multiplier;

                    // Проверяем, помещается ли результат в i64.
                    matrix[row][col] = i64::try_from(value).expect("Matrix coefficient overflow");

                    col += 1;
                }

                // Здесь должен получиться ноль.
                matrix[row][pivot_col] = 0;
            }

            // Переходим к следующей строке.
            row += 1;
        }

        // Следующий ведущий элемент ищем
        // на следующей строке и в следующем столбце.
        pivot_row += 1;
        pivot_col += 1;
    }
}

// --------------------------------------------------
// B) GAUSSIAN ELIMINATION — 3 POINTS
// --------------------------------------------------

// Решаем систему линейных уравнений.
//
// Последний столбец содержит правые части.
//
// Например, матрица 3x4 означает:
// 3 уравнения с 3 неизвестными.
//
// Возвращаем:
// - массив из f64 с найденными значениями;
// - количество неизвестных.
//
// Если решений нет, длина результата равна 0.
//
// Если решений бесконечно много,
// свободным переменным присваиваем 0.
fn gaussian_elimination(matrix: &Matrix, rows: usize, cols: usize) -> ([f64; 10], usize) {
    // Проверяем размеры расширенной матрицы.
    assert!(rows > 0 && rows <= 10);
    assert!(cols >= 2 && cols <= 10);

    // Последний столбец — результаты уравнений.
    // Поэтому неизвестных на одну меньше.
    let variables = cols - 1;

    // Создаём массив ответов, заполненный нулями.
    //
    // Нули также будут значениями свободных переменных.
    let mut solution = [0.0f64; 10];

    // Копируем матрицу, чтобы сохранить оригинал.
    let mut stepped = *matrix;

    // Используем функцию из пункта A.
    echelon_matrix(&mut stepped, rows, cols);

    // Начинаем обратную подстановку
    // с последней строки.
    let mut row = rows;

    while row > 0 {
        row -= 1;

        // Ищем первый ненулевой коэффициент строки.
        let mut pivot_col = 0;

        while pivot_col < variables && stepped[row][pivot_col] == 0 {
            pivot_col += 1;
        }

        // Если все коэффициенты неизвестных равны нулю.
        if pivot_col == variables {
            // Но правая часть не равна нулю:
            //
            // 0*x + 0*y + ... = 5
            //
            // Такое уравнение невозможно.
            if stepped[row][variables] != 0 {
                // Нулевая длина означает отсутствие решений.
                return (solution, 0);
            }

            // Если справа тоже 0, получаем 0 = 0.
            // Это уравнение ничего не ограничивает.
        } else {
            // Берём правую часть уравнения.
            let mut value = stepped[row][variables] as f64;

            // Вычитаем значения уже найденных переменных.
            let mut col = pivot_col + 1;

            while col < variables {
                value -= stepped[row][col] as f64 * solution[col];

                col += 1;
            }

            // Делим остаток на коэффициент
            // перед ведущей неизвестной.
            solution[pivot_col] = value / stepped[row][pivot_col] as f64;
        }
    }

    // Решение найдено.
    //
    // Возвращаем массив и количество неизвестных.
    (solution, variables)
}

// --------------------------------------------------
// C) CHECK SOLUTION — 1 POINT
// --------------------------------------------------

// Проверяем найденное решение подстановкой.
//
// Для каждой строки вычисляем:
//
// a*x + b*y + c*z + ...
//
// Затем сравниваем с правой частью.
//
// Используем небольшую погрешность,
// поскольку f64 не всегда представляет
// дробные числа абсолютно точно.
fn check_solution(
    matrix: &Matrix,
    rows: usize,
    cols: usize,
    solution: &[f64; 10],
    solution_length: usize,
) -> bool {
    assert!(rows > 0 && rows <= 10);
    assert!(cols >= 2 && cols <= 10);

    // Количество неизвестных.
    let variables = cols - 1;

    // Если решений нет или длина неверная,
    // проверить такую подстановку нельзя.
    if solution_length != variables {
        return false;
    }

    // Проверяем уравнения одно за другим.
    let mut row = 0;

    while row < rows {
        // Здесь будем считать левую часть уравнения.
        let mut left = 0.0f64;

        let mut col = 0;

        while col < variables {
            // Коэффициент умножаем на значение переменной.
            left += matrix[row][col] as f64 * solution[col];

            col += 1;
        }

        // Правая часть находится в последнем столбце.
        let right = matrix[row][variables] as f64;

        // Разность между сторонами уравнения.
        let difference = (left - right).abs();

        // Допустимая погрешность сравнения f64.
        let tolerance = 0.00000001 * (1.0 + left.abs() + right.abs());

        // Если равенство не выполняется,
        // решение неверное.
        if difference > tolerance {
            return false;
        }

        row += 1;
    }

    // Все уравнения выполнены.
    true
}

// --------------------------------------------------
// MAIN
// --------------------------------------------------

fn main() {
    println!("GAUSSIAN ELIMINATION");
    println!("========================================");

    // Создаём матрицу 3x4 из условия преподавателя.
    let mut matrix = [[0i64; 10]; 10];

    // Первое уравнение: 3z = 2.
    matrix[0][0] = 0;
    matrix[0][1] = 0;
    matrix[0][2] = 3;
    matrix[0][3] = 2;

    // Второе уравнение: 2y + 4z = 3.
    matrix[1][0] = 0;
    matrix[1][1] = 2;
    matrix[1][2] = 4;
    matrix[1][3] = 3;

    // Третье уравнение: 4y + 8z = 6.
    matrix[2][0] = 0;
    matrix[2][1] = 4;
    matrix[2][2] = 8;
    matrix[2][3] = 6;

    let rows = 3;
    let cols = 4;

    println!("ORIGINAL MATRIX");
    print_matrix(&matrix, rows, cols);

    // A) Приводим копию матрицы к ступенчатому виду.
    let mut stepped = matrix;

    echelon_matrix(&mut stepped, rows, cols);

    println!();
    println!("A) ECHELON MATRIX");
    print_matrix(&stepped, rows, cols);

    // Проверяем ожидаемую матрицу из задания.
    assert!(stepped[0][1] == 2);
    assert!(stepped[0][2] == 4);
    assert!(stepped[0][3] == 3);

    assert!(stepped[1][2] == 3);
    assert!(stepped[1][3] == 2);

    // B) Решаем исходную систему.
    let (solution, length) = gaussian_elimination(&matrix, rows, cols);

    println!();
    println!("B) SOLUTION");

    if length == 0 {
        println!("No solution");
    } else {
        let mut i = 0;

        while i < length {
            println!("x{} = {:.6}", i + 1, solution[i]);

            i += 1;
        }
    }

    // C) Проверяем решение подстановкой.
    let valid = check_solution(&matrix, rows, cols, &solution, length);

    println!();
    println!("C) VERIFICATION");
    println!("Solution valid: {}", valid);

    assert!(valid, "Solution verification failed");

    // --------------------------------------------------
    // ADDITIONAL TEST: NO SOLUTION
    // --------------------------------------------------

    // x + y = 1
    // x + y = 2
    //
    // Система противоречива.
    let mut impossible = [[0i64; 10]; 10];

    impossible[0][0] = 1;
    impossible[0][1] = 1;
    impossible[0][2] = 1;

    impossible[1][0] = 1;
    impossible[1][1] = 1;
    impossible[1][2] = 2;

    let (_, impossible_length) = gaussian_elimination(&impossible, 2, 3);

    println!();
    println!("ADDITIONAL TEST: NO SOLUTION");
    println!("Solution length: {}", impossible_length);

    assert!(impossible_length == 0);

    // --------------------------------------------------
    // ADDITIONAL TEST: INFINITE SOLUTIONS
    // --------------------------------------------------

    // x + y = 3
    // 2x + 2y = 6
    //
    // Второе уравнение дублирует первое.
    // Выбираем свободную переменную y = 0,
    // после чего x = 3.
    let mut infinite = [[0i64; 10]; 10];

    infinite[0][0] = 1;
    infinite[0][1] = 1;
    infinite[0][2] = 3;

    infinite[1][0] = 2;
    infinite[1][1] = 2;
    infinite[1][2] = 6;

    let (infinite_solution, infinite_length) = gaussian_elimination(&infinite, 2, 3);

    println!();
    println!("ADDITIONAL TEST: INFINITE SOLUTIONS");

    println!("x = {:.6}", infinite_solution[0]);
    println!("y = {:.6}", infinite_solution[1]);

    let infinite_valid = check_solution(&infinite, 2, 3, &infinite_solution, infinite_length);

    println!("Solution valid: {}", infinite_valid);

    assert!(infinite_valid);

    println!();
    println!("All tests passed.");
}
