// TODO:
//  Разработайте функцию, которая принимает целое число в качестве аргумента и возвращает строку,
//  содержащую это число и слово "компьютер" в нужном склонении по падежам в зависимости от числа.
//  Например, при вводе числа 25 функция должна возвращать "25 компьютеров",
//  для числа 41 — "41 компьютер", а для числа 1048 — "1048 компьютеров".
//  -2^31 <= number <= 2^31 - 1
pub fn get_computer_string(number: i32) -> String {
    // делаем положительным число
    let positive_number = number.abs();

    // берем остаток от деления на 100
    let num = positive_number % 100;

    if num > 10 && num < 20 {
        format!("{positive_number} компьютеров")
    }
    else if num % 10 > 1 && num % 10 < 5 { format!("{positive_number} компьютера")
    }
    else if num % 10 == 1 {
        format!("{positive_number} компьютер")
    }
    else {
        format!("{positive_number} компьютеров")
    }
}

// Тесты
#[cfg(test)]
mod tests {
    use crate::exercise_1::get_computer_string;
    #[test]
    fn get_computer_string_test() {
        let num_1 = -1001;
        let num_2 = 1;
        let num_3 = 3;
        let num_4 = -1048;
        let num_5 = 934;

        assert_eq!(get_computer_string(num_1), "1001 компьютер".to_string());
        assert_eq!(get_computer_string(num_2), "1 компьютер".to_string());
        assert_eq!(get_computer_string(num_3), "3 компьютера".to_string());
        assert_eq!(get_computer_string(num_4), "1048 компьютеров".to_string());
        assert_eq!(get_computer_string(num_5), "934 компьютера".to_string());
    }
}
