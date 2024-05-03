// Разделитель для символов внутри строки консольного вывода
const SEP: &str = " ";

// Символ-разделитель для верхней границы таблицы
const SEP_HIGH: &str = "—";

// Длина отступа для выравнивания символов внутри строки консольного вывода
const SEP_LEN: usize = 4;

// unit структура
struct Solution;
impl Solution {

    // Метод для инициализации и заполнения двумерного массива
    pub fn print_table(num: u32) -> Result<(), String> {
        // Проверяем не равно ли число нулю, иначе возвращаем ошибку
        if num == 0 {
            return Err("num must be more than zero".to_string())
        }

        // Создаем двумерный массив для таблицы
        let mut table = vec![vec![0u32; (num + 1) as usize]; (num + 1) as usize];

        // Инициализируем таблицу
        for i in 1..=num {
            table[0][i as usize] = i;
            table[i as usize][0] = i;
        }

        // Заполняем таблицу перемножениями
        for row in 1..=num as usize{
            for col in 1..=num as usize{
                table[row][col] = table[row][0] * table[0][col];
            }
        }

        // Вызываем функцию красивого консольного вывода
        pretty_table_output(table, num);

        Ok(())
    }
}

// Функция для консольного вывода матрицы в таблицу
pub fn pretty_table_output(table: Vec<Vec<u32>>, num: u32) {
    // Выводим верхнюю границу таблицы
    print!("{} ", SEP_HIGH.repeat((num * 4 + num) as usize));

    println!();

    // Итерируемся по строкам таблицы
    for row in table.iter() {
        // Выводим левую границу таблицы
        print!("| ");

        // Итерируемся по элементам строк
        for col in row.iter() {
            // Рассчитываем выравнивание по левому краю и выводим строку
            print!("{col}{}", SEP.repeat(SEP_LEN - col.to_string().len()))
        }
        println!()
    }
}

// Тесты
#[cfg(test)]
mod tests {
    use crate::exercise_4::Solution;
    use pretty_assertions::assert_eq;
    #[test]
    fn print_table_test() {
        let num_1 = 5;
        let num_2 = 2;
        let num_3 = 6;
        let num_4 = 7;
        let num_5 = 0;

        assert_eq!(Solution::print_table(num_1), Ok(()));
        assert_eq!(Solution::print_table(num_2), Ok(()));
        assert_eq!(Solution::print_table(num_3), Ok(()));
        assert_eq!(Solution::print_table(num_4), Ok(()));
        assert_eq!(Solution::print_table(num_5), Err("num must be more than zero".to_string()));
    }
}