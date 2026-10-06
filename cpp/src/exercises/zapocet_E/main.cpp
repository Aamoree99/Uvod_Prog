/*
Zápočet — E
Název: Druhé největší číslo v poli

Тест на оценку 3: найти второе максимальное число в массиве.
Это учебное решение по описанию теста, а не копия полного условия.
Здесь второй максимум — второй элемент по убыванию с учётом повторов:
для [9, 9, 5] ответ 9. Если нужны разные значения, правило будет другим.
*/

#include <iostream>
#include <vector>
#include <string>
#include <sstream>
#include <optional>

template<class T>
std::string array_text(const T& values) {
    std::ostringstream out;
    out << "[";
    std::size_t i = 0;
    while (i < values.size()) {
        if (i) out << ", ";
        out << values[i++];
    }
    out << "]";
    return out.str();
}

// Второй максимум учитывает повторы: для [9,9,5] ответ 9.
// Отсутствующий результат отличаем от любого целого числа, включая минимальное.
static std::optional<int> second_maximum(const std::vector<int>& numbers) {
    if (static_cast<int>(numbers.size()) < 2) return std::nullopt;
    // Начинаем с реальных элементов, поэтому отрицательные значения тоже работают.
    int maximum=numbers[0], second=numbers[1];
    if (second > maximum) { int temp=maximum; maximum=second; second=temp; }
    int index=2;
    while (index < static_cast<int>(numbers.size())) {
        int current=numbers[index];
        if (current > maximum) {
            // Сначала сохраняем старого лидера, потом заменяем его новым.
            second=maximum; maximum=current;
        } else if (current > second) second=current;
        index += 1;
    }
    // Один проход: O(n) времени, O(1) дополнительной памяти.
    return second;
}
int main() {
    std::cout << std::boolalpha;
    std::vector<int> numbers = {2,5,7,1,4,6,9,8,2};
    std::cout << "Массив: " << array_text(numbers) << '\n';
    std::optional<int> value=second_maximum(numbers);
    if (value.has_value()) std::cout << "Второе максимальное число: " << value.value() << '\n';
    else std::cout << "Для второго максимума нужны хотя бы два элемента." << '\n';
}
