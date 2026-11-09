/*
Cvičení 09 — Úloha 9.1
Název: Matice
Body: 1 bod; 1 bod; 1 bod; 1 bod; 1 bod; 1 bod; 1 bod; 1 bod; 1 bod; 1 bod (v pořadí podúloh)

Zadání:
Napište následující funkce pracující s maticí celých čísel long:

a) Funkci, která vytvoří matici zadaného rozměru naplněnou prvky s náhodnými hodnotami. Hodnoty
volte z intervalu [-4,+4], je to výhodné pro použití v dalších příkladech. [1 bod]

b) Funkci, která zadanou matici uloží do textového souboru. Na prvním řádku bude uložen rozměr
matice. Na dalších řádcích budou uloženy jednotlivé řádky matice [1 bod]

c) Funkci, která z textového souboru načte matici (zadanou formou uvedenou v 1.b). [1 bod]

d) Funkci, která vytvoří transponovanou matici (= záměna řádků za sloupce). [1 bod]

e) Funkci testující, zda zadaná matice je symetrická. Matice je symetrická, pokud pro každý prvek
nacházející se v řádku r a sloupci s, platí, že jeho hodnota je rovna hodnotě prvku v řádku s a
sloupci r. [1 bod].

f) Funkce vracející pole obsahující diagonálu matice. Diagonála matice je tvořena prvky, které se
nacházejí v řádku se stejným pořadovým číslem, jako je pořadové číslo sloupce, ve kterém se
nacházejí. [1 bod].

g) Funkci testující, zda matice je horní trojúhelníková. Tj. Matice je čtvercová a všechny prvky
nacházející se pod diagonálou mají nulovou hodnotu. [1 bod].

h) Funkci vracející pořadové číslo sloupce, ve kterém se na zadaném řádku nachází první nenulový
prvek. Tj. Na zadaném řádku určí počet prvků s nulovou hodnotou nacházejících se nalevo od prvního
nenulového prvku zleva [1 bod].

i) Funkci testující, zda matice je odstupňovaná. V odstupňované matici je na každém řádku počet nul
zleva alespoň o jednu nulu větší než na předchozím řádku. [1 bod].

j) Funkci testující, zda matice je jednotková. Jednotková matice je čtvercová matice, která má na
diagonále prvky s hodnotou jedna, ostatní prvky mají nulovou hodnotu. [1 bod].


Ukázka:

long[,] M1 = NahodnaMatice(2, 3);

UlozMatici(M1, "matice1.txt");

long[,] M2 = NactiMatici("matice1.txt");

long[,] M3 = { { 2, 3, 1 }, { 7, 4, 6 } };

long[,] M4 = TransponovanaMatice(M3);

// { {2, 7 }, { 3, 4 }, { 1, 6 } }

long[,] M5 = { { 2, 6, 1 }, { 6, 4, 3 }, { 1, 3, 7 } };

bool b1 = MaticeJeSymetricka(M5); // true

long[] v1 = DiagonalaMatice(M5); // 2, 4, 7

long[,] M6 = { { 4, 5, 2 }, { 0, 3, 6 }, { 0, 0, 9 } };

bool b2 = MaticeJeHorniTrojuhelnikova(M6); // true

int i1 = PocetNulZleva(M6, 2); // 2

long[,] M7 = { { 7, 1, 2 }, { 0, 0, 6 } };

bool b3 = MaticeJeOdstupnovana(M7); // true

bool b4 = MaticeJeJednotkova(M6); // false

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

use rand::Rng;
use std::fs;
use std::fs::File;
use std::io::Write;

// Тип Matrix — просто короткое название
// для двумерного массива 10 на 10.
//
// Все элементы имеют тип i64,
// который соответствует long из C#.
type Matrix = [[i64; 10]; 10];

// --------------------------------------------------
// A) RANDOM MATRIX
// --------------------------------------------------

// Создаём матрицу и заполняем случайными числами.
fn random_matrix(rows: usize, cols: usize) -> Matrix {
    // Проверяем, что размеры допустимы.
    assert!(rows > 0 && rows <= 10);
    assert!(cols > 0 && cols <= 10);

    // Создаём массив 10 на 10, заполненный нулями.
    let mut matrix = [[0i64; 10]; 10];

    // Создаём генератор случайных чисел.
    let mut rng = rand::thread_rng();

    // Начинаем с первой строки.
    let mut row = 0;

    // Перебираем все используемые строки.
    while row < rows {
        // В каждой строке начинаем с первого столбца.
        let mut col = 0;

        // Перебираем столбцы текущей строки.
        while col < cols {
            // Записываем случайное число от -4 до 4.
            matrix[row][col] = rng.gen_range(-4..=4);

            // Переходим к следующему столбцу.
            col += 1;
        }

        // После всех столбцов переходим к следующей строке.
        row += 1;
    }

    // Возвращаем готовую матрицу.
    matrix
}

// --------------------------------------------------
// PRINT MATRIX
// --------------------------------------------------

// Выводим матрицу в терминал.
fn print_matrix(matrix: &Matrix, rows: usize, cols: usize) {
    let mut row = 0;

    while row < rows {
        let mut col = 0;

        while col < cols {
            // :4 — выделяем 4 позиции под число,
            // чтобы столбцы выглядели ровно.
            print!("{:4}", matrix[row][col]);

            col += 1;
        }

        // Завершаем текущую строку матрицы.
        println!();

        row += 1;
    }
}

// --------------------------------------------------
// B) SAVE MATRIX
// --------------------------------------------------

// Сохраняем матрицу в текстовый файл.
fn save_matrix(matrix: &Matrix, rows: usize, cols: usize, filename: &str) {
    // Создаём или перезаписываем файл.
    let mut file = File::create(filename).expect("Cannot create file");

    // Первая строка — размеры матрицы.
    writeln!(file, "{} {}", rows, cols).expect("Cannot write dimensions");

    let mut row = 0;

    while row < rows {
        let mut col = 0;

        while col < cols {
            // Между соседними числами записываем пробел.
            if col > 0 {
                write!(file, " ").expect("Cannot write separator");
            }

            // Записываем число.
            write!(file, "{}", matrix[row][col]).expect("Cannot write number");

            col += 1;
        }

        // Следующая строка файла — следующая строка матрицы.
        writeln!(file).expect("Cannot write newline");

        row += 1;
    }
}

// --------------------------------------------------
// C) LOAD MATRIX
// --------------------------------------------------

// Читаем очередное целое число из текста.
//
// position — позиция, с которой продолжаем чтение.
//
// Например, из текста "2 3\n-4 5"
// последовательно получим 2, 3, -4, 5.
fn read_number(text: &str, position: &mut usize) -> i64 {
    let bytes = text.as_bytes();

    // Пропускаем пробелы, табуляцию и переводы строк.
    while *position < bytes.len()
        && (bytes[*position] == b' '
            || bytes[*position] == b'\n'
            || bytes[*position] == b'\r'
            || bytes[*position] == b'\t')
    {
        *position += 1;
    }

    // Проверяем, что число существует.
    assert!(*position < bytes.len(), "Missing number");

    // Запоминаем начало числа.
    let start = *position;

    // Пропускаем необязательный знак.
    if bytes[*position] == b'-' || bytes[*position] == b'+' {
        *position += 1;
    }

    // Запоминаем позицию первой цифры.
    let first_digit = *position;

    // Проходим все цифры.
    while *position < bytes.len() && bytes[*position] >= b'0' && bytes[*position] <= b'9' {
        *position += 1;
    }

    // После знака должна быть хотя бы одна цифра.
    assert!(*position > first_digit, "Invalid number");

    // После числа должен находиться пробельный символ
    // либо конец текста.
    if *position < bytes.len() {
        assert!(
            bytes[*position] == b' '
                || bytes[*position] == b'\n'
                || bytes[*position] == b'\r'
                || bytes[*position] == b'\t',
            "Invalid separator"
        );
    }

    // Преобразуем найденный фрагмент в i64.
    text[start..*position]
        .parse::<i64>()
        .expect("Cannot parse integer")
}

// Загружаем матрицу из файла.
//
// Возвращаем одновременно:
// матрицу, количество строк, количество столбцов.
fn load_matrix(filename: &str) -> (Matrix, usize, usize) {
    // Читаем весь файл в строку.
    let text = fs::read_to_string(filename).expect("Cannot read file");

    // Текущая позиция в тексте.
    let mut position = 0;

    // Первые два числа — размеры матрицы.
    let rows = read_number(&text, &mut position) as usize;
    let cols = read_number(&text, &mut position) as usize;

    // Проверяем размеры.
    assert!(rows > 0 && rows <= 10);
    assert!(cols > 0 && cols <= 10);

    // Создаём пустую матрицу.
    let mut matrix = [[0i64; 10]; 10];

    let mut row = 0;

    while row < rows {
        let mut col = 0;

        while col < cols {
            // Считываем очередное число из файла
            // и записываем его в текущую ячейку.
            matrix[row][col] = read_number(&text, &mut position);

            col += 1;
        }

        row += 1;
    }

    // Возвращаем три значения одновременно.
    (matrix, rows, cols)
}

// --------------------------------------------------
// D) TRANSPOSE MATRIX
// --------------------------------------------------

// Меняем строки и столбцы местами.
fn transpose_matrix(matrix: &Matrix, rows: usize, cols: usize) -> Matrix {
    // Создаём новую пустую матрицу.
    let mut result = [[0i64; 10]; 10];

    let mut row = 0;

    while row < rows {
        let mut col = 0;

        while col < cols {
            // Ключевая операция:
            // строка становится столбцом,
            // столбец становится строкой.
            result[col][row] = matrix[row][col];

            col += 1;
        }

        row += 1;
    }

    result
}

// --------------------------------------------------
// E) SYMMETRIC MATRIX
// --------------------------------------------------

// Проверяем симметричность матрицы.
fn is_symmetric(matrix: &Matrix, rows: usize, cols: usize) -> bool {
    // Симметричная матрица обязательно квадратная.
    if rows != cols {
        return false;
    }

    let mut row = 0;

    while row < rows {
        let mut col = 0;

        while col < cols {
            // Сравниваем зеркальные элементы.
            if matrix[row][col] != matrix[col][row] {
                return false;
            }

            col += 1;
        }

        row += 1;
    }

    // Все соответствующие элементы совпадают.
    true
}

// --------------------------------------------------
// F) MATRIX DIAGONAL
// --------------------------------------------------

// Возвращаем элементы главной диагонали.
//
// Например:
// 2 6 1
// 6 4 3
// 1 3 7
//
// Диагональ: 2, 4, 7.
fn matrix_diagonal(matrix: &Matrix, rows: usize, cols: usize) -> ([i64; 10], usize) {
    // Создаём массив для диагонали.
    let mut diagonal = [0i64; 10];

    // Длина диагонали равна меньшему размеру.
    let length = if rows < cols { rows } else { cols };

    let mut i = 0;

    while i < length {
        // У элементов диагонали одинаковые индексы.
        diagonal[i] = matrix[i][i];

        i += 1;
    }

    // Возвращаем массив и количество нужных элементов.
    (diagonal, length)
}

// --------------------------------------------------
// G) UPPER TRIANGULAR MATRIX
// --------------------------------------------------

// Проверяем верхнюю треугольную матрицу.
//
// Все элементы ниже диагонали должны быть нулями.
fn is_upper_triangular(matrix: &Matrix, rows: usize, cols: usize) -> bool {
    if rows != cols {
        return false;
    }

    let mut row = 0;

    while row < rows {
        let mut col = 0;

        while col < cols {
            // row > col означает, что элемент
            // находится ниже главной диагонали.
            if row > col && matrix[row][col] != 0 {
                return false;
            }

            col += 1;
        }

        row += 1;
    }

    true
}

// --------------------------------------------------
// H) COUNT LEFT ZEROS
// --------------------------------------------------

// Считаем нули слева в заданной строке.
//
// Например:
// [0, 0, 5, 2] -> 2
//
// Если вся строка нулевая, возвращаем cols.
fn count_left_zeros(matrix: &Matrix, row: usize, rows: usize, cols: usize) -> usize {
    assert!(row < rows, "Invalid row index");

    let mut col = 0;

    // Считаем нули только до первого
    // ненулевого элемента.
    while col < cols && matrix[row][col] == 0 {
        col += 1;
    }

    col
}

// --------------------------------------------------
// I) ECHELON MATRIX
// --------------------------------------------------

// Проверяем ступенчатую матрицу.
//
// На каждой следующей строке начальных нулей
// должно быть больше, чем на предыдущей.
fn is_echelon(matrix: &Matrix, rows: usize, cols: usize) -> bool {
    // В одной строке условие выполняется автоматически.
    let mut row = 1;

    while row < rows {
        // Нули в предыдущей строке.
        let previous = count_left_zeros(matrix, row - 1, rows, cols);

        // Нули в текущей строке.
        let current = count_left_zeros(matrix, row, rows, cols);

        // Следующая строка должна начинаться
        // с большего количества нулей.
        if current <= previous {
            return false;
        }

        row += 1;
    }

    true
}

// --------------------------------------------------
// J) IDENTITY MATRIX
// --------------------------------------------------

// Проверяем единичную матрицу.
//
// На диагонали должны быть единицы,
// все остальные элементы должны быть нулями.
fn is_identity(matrix: &Matrix, rows: usize, cols: usize) -> bool {
    // Единичная матрица всегда квадратная.
    if rows != cols {
        return false;
    }

    let mut row = 0;

    while row < rows {
        let mut col = 0;

        while col < cols {
            if row == col {
                // На диагонали должна находиться единица.
                if matrix[row][col] != 1 {
                    return false;
                }
            } else {
                // Вне диагонали должны находиться нули.
                if matrix[row][col] != 0 {
                    return false;
                }
            }

            col += 1;
        }

        row += 1;
    }

    true
}

// --------------------------------------------------
// MAIN
// --------------------------------------------------

fn main() {
    println!("MATRIX OPERATIONS");
    println!("========================================");

    // A) Создаём случайную матрицу 2 на 3.
    let m1 = random_matrix(2, 3);

    println!("A) Random matrix:");
    print_matrix(&m1, 2, 3);

    // B) Сохраняем её в файл.
    save_matrix(&m1, 2, 3, "matice1.txt");

    println!();
    println!("B) Matrix saved to matice1.txt");

    // C) Загружаем матрицу из файла.
    let (m2, rows, cols) = load_matrix("matice1.txt");

    println!();
    println!("C) Loaded matrix:");
    print_matrix(&m2, rows, cols);

    // Проверяем правильность сохранения и загрузки.
    let mut row = 0;

    while row < rows {
        let mut col = 0;

        while col < cols {
            assert!(m1[row][col] == m2[row][col]);
            col += 1;
        }

        row += 1;
    }

    // D) Транспонирование.
    let mut m3 = [[0i64; 10]; 10];

    m3[0][0] = 2;
    m3[0][1] = 3;
    m3[0][2] = 1;

    m3[1][0] = 7;
    m3[1][1] = 4;
    m3[1][2] = 6;

    let m4 = transpose_matrix(&m3, 2, 3);

    println!();
    println!("D) Transposed matrix:");
    print_matrix(&m4, 3, 2);

    // E + F) Симметричность и диагональ.
    let mut m5 = [[0i64; 10]; 10];

    m5[0][0] = 2;
    m5[0][1] = 6;
    m5[0][2] = 1;

    m5[1][0] = 6;
    m5[1][1] = 4;
    m5[1][2] = 3;

    m5[2][0] = 1;
    m5[2][1] = 3;
    m5[2][2] = 7;

    println!();
    println!("E) Symmetric: {}", is_symmetric(&m5, 3, 3));

    let (diagonal, length) = matrix_diagonal(&m5, 3, 3);

    print!("F) Diagonal: ");

    let mut i = 0;

    while i < length {
        print!("{} ", diagonal[i]);
        i += 1;
    }

    println!();

    // G + H + J) Треугольная матрица.
    let mut m6 = [[0i64; 10]; 10];

    m6[0][0] = 4;
    m6[0][1] = 5;
    m6[0][2] = 2;

    m6[1][0] = 0;
    m6[1][1] = 3;
    m6[1][2] = 6;

    m6[2][0] = 0;
    m6[2][1] = 0;
    m6[2][2] = 9;

    println!();
    println!("G) Upper triangular: {}", is_upper_triangular(&m6, 3, 3));

    println!("H) Left zeros in row 2: {}", count_left_zeros(&m6, 2, 3, 3));

    // I) Ступенчатая матрица.
    let mut m7 = [[0i64; 10]; 10];

    m7[0][0] = 7;
    m7[0][1] = 1;
    m7[0][2] = 2;

    m7[1][0] = 0;
    m7[1][1] = 0;
    m7[1][2] = 6;

    println!();
    println!("I) Echelon: {}", is_echelon(&m7, 2, 3));

    // J) Проверка единичной матрицы.
    println!();
    println!("J) Identity (M6): {}", is_identity(&m6, 3, 3));

    // Дополнительная проверка настоящей
    // единичной матрицы.
    let mut identity = [[0i64; 10]; 10];

    identity[0][0] = 1;
    identity[1][1] = 1;
    identity[2][2] = 1;

    println!("   Identity (test): {}", is_identity(&identity, 3, 3));
}
