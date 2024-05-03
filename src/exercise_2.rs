// TODO:
//  Написать функцию/метод, которая на вход получает массив положительных целых чисел произвольной длины.
//  Например [42, 12, 18].
//  На выходе возвращает массив чисел, которые являются общими делителями для всех указанных числе.
//  В примере это будет [2, 3, 6].
//  -2^31 <= array[i] <= 2^31 - 1

pub fn get_common_divisor(array: Vec<i32>) -> Result<Vec<i32>, String> {
    // Создаем результирующий массив
    let mut result_vec = Vec::<i32>::new();

    // Флаг для условия добавления переменной в массив
    let mut flag;

    // Возвращаем ошибку, если подан пустой массив, иначе - максимальное число из массива
    let max_num = match array.iter().max() {
        Some(value) => value,
        None => return Err("Error: empty array".to_string())
    };

    // Идем в цикле до половины от максимального числа
    for i in 2..max_num / 2 {
        // Обнуляем флаг
        flag = true;

        // Итерируемся по массиву чисел
        for j in array.iter() {
            // Проверяем, делится ли без остатка число на делитель
            if *j % i != 0 {
                flag = false;
            }
        }

        // Если истина - добавляем в массив делитель
        if flag { result_vec.push(i); }
    }

    // Возвращаем результирующий массив
    Ok(result_vec)
}

// Тесты
#[cfg(test)]
mod tests {
    use crate::exercise_2::get_common_divisor;
    use pretty_assertions::assert_eq;
    #[test]
    fn get_common_divisor_test() {
        let vec_1 = vec![42, 12, 18, 36];
        let vec_2 = vec![];
        let vec_3 = vec![3, 7, 4, 9];
        let vec_4 = vec![144, 12];
        let vec_5 = vec![25, 125, 625];
        let vec_6 = vec![24, 36, 108, 96, 72];

        assert_eq!(get_common_divisor(vec_1), Ok(vec![2, 3, 6]));
        assert_eq!(get_common_divisor(vec_2), Err("Error: empty array".to_string()));
        assert_eq!(get_common_divisor(vec_3), Ok(vec![]));
        assert_eq!(get_common_divisor(vec_4), Ok(vec![2, 3, 4, 6, 12]));
        assert_eq!(get_common_divisor(vec_5), Ok(vec![5, 25]));
        assert_eq!(get_common_divisor(vec_6), Ok(vec![2, 3, 4, 6, 12]));
    }
}