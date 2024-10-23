const WIDTH: usize = 22;  // Ширина конверта
const HEIGHT: usize = 22; // Висота конверта
#[test]
fn main() {
    let mut output = String::new();  // Створюємо порожній рядок для виведення

    for y in 0..HEIGHT {  // Висота конверта
        for x in 0..WIDTH {  // Ширина конверта
            if y == 0 || y == HEIGHT - 1 {  // Верхня та нижня межі
                output.push('*');
            } else if x == 0 || x == WIDTH - 1 {  // Ліва та права межі
                output.push('*');
            } else if x == y || x == WIDTH - y - 2 {  // Діагональні лінії
                output.push('*');
            } else if y + x == WIDTH - 2 {  // Умова для другої діагоналі
                output.push('*');
            } else {
                output.push(' ');  // Порожній простір
            }
        }
        output.push('\n');  // Переходимо на новий рядок
    }

    print!("{}", output);  // Виводимо все одним разом
}
