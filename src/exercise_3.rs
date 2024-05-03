// TODO: Написать функцию/метод, которая возвращает массив простых чисел в диапазоне (2 числа - минимальное и максимальное) заданных чисел.
//  Например, на вход переданы 2 числа: от 11 до 20  (диапазон считается включая граничные значения).
//  На выходе программа должна выдать [11, 13 , 17, 19]
//  0 <= low <= 2^32
//  0 <= high <= 2^32
//  0 <= array[i] <= 2^32

pub fn get_prime_numbers(low: u32, high: u32) -> Result<Vec<u32>, String> {
    // Создаем результирующий массив
    let mut res_vec = Vec::<u32>::new();

    // Флаг для условия добавления переменной в массив
    let mut flag;

    // Проверяем правильность нижней и верхней границ
    if low >= high {
        return Err("Error: low must be less than high".to_string());
    }

    // Идем в цикле включительно от нижней до верхней границы
    for i in low..=high {
        // Обнуляем флаг
        flag = true;

        // Идем в цикле по делителям до квадратного корня числа
        for j in 2..=f32::sqrt(i as f32) as u32 {
            // Проверяем делится ли без остатка число на делитель
            if i % j == 0 {
                flag = false;
            }
        }

        // Если не нашли числа, которые делились бы без остатка - добавляем в результирующий массив
        if flag {
            res_vec.push(i);
        }
    }

    // Если массив пустой - мы не нашли ни одного простого числа, возвращаем ошибку
    if res_vec.is_empty() {
        return Err("Error: there are no prime numbers in the range ".to_string());
    }

    // Возвращаем результирующий массив
    Ok(res_vec)
}

// Тесты
#[cfg(test)]
mod tests {
    use crate::exercise_3::get_prime_numbers;
    use pretty_assertions::assert_eq;
    #[test]
    fn get_prime_numbers_test() {
        let (low_1, high_1) = (11, 20);
        let (low_2, high_2) = (3, 47);
        let (low_3, high_3) = (0, 0);
        let (low_4, high_4) = (121, 12);
        let (low_5, high_5) = (54, 94);
        let (low_6, high_6) = (8, 9);

        assert_eq!(get_prime_numbers(low_1, high_1), Ok(vec![11, 13, 17, 19]));
        assert_eq!(get_prime_numbers(low_2, high_2), Ok(vec![3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]));
        assert_eq!(get_prime_numbers(low_3, high_3), Err("Error: low must be less than high".to_string()));
        assert_eq!(get_prime_numbers(low_4, high_4), Err("Error: low must be less than high".to_string()));
        assert_eq!(get_prime_numbers(low_5, high_5), Ok(vec![59, 61, 67, 71, 73, 79, 83, 89]));
        assert_eq!(get_prime_numbers(low_6, high_6), Err("Error: there are no prime numbers in the range ".to_string()));
    }
}