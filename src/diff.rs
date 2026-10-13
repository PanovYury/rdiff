use std::fmt;

#[derive(Debug, PartialEq)]
pub enum DiffOp<'a, 'b, T> {
    Unchanged(&'a T),
    Removed(&'a T),
    Added(&'b T),
}

impl<'a, 'b, T: fmt::Debug> fmt::Display for DiffOp<'a, 'b, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiffOp::Unchanged(line) => write!(f, "  {:?}", line),
            DiffOp::Removed(line) => write!(f, "- {:?}", line),
            DiffOp::Added(line) => write!(f, "+ {:?}", line),
        }
    }
}

pub fn diff<'a, 'b, T: PartialEq>(x: &'a [T], y: &'b [T]) -> Vec<DiffOp<'a, 'b, T>> {
    let n = x.len();
    let m = y.len();

    // Краевой случай 1: Оба пустые
    if n == 0 && m == 0 {
        return Vec::new();
    }

    let max_d = n + m;
    let max_d_isize = max_d as isize;

    let mut v = vec![0isize; 2 * max_d + 1];

    // Храним полную историю векторов v для каждого шага d.
    // Каждый элемент истории — это копия вектора v на шаге d.
    let mut history: Vec<Vec<isize>> = Vec::with_capacity(max_d + 1);

    'outer: for d in 0..=max_d {
        let d_isize = d as isize;

        for k in (-d_isize..=d_isize).step_by(2) {
            let idx = (k + max_d_isize) as usize;

            let mut x_pos = if k == -d_isize || (k != d_isize && v[idx - 1] < v[idx + 1]) {
                v[idx + 1] // Движение вниз (добавление)
            } else {
                v[idx - 1] + 1 // Движение вправо (удаление)
            };

            let mut y_pos = x_pos - k;

            // «Змейка»: двигаемся бесплатно, пока элементы совпадают
            while x_pos < n as isize && y_pos < m as isize && x[x_pos as usize] == y[y_pos as usize]
            {
                x_pos += 1;
                y_pos += 1;
            }

            v[idx] = x_pos;

            if x_pos >= n as isize && y_pos >= m as isize {
                history.push(v.clone());
                break 'outer;
            }
        }
        history.push(v.clone());
    }

    // --- ИСТИННЫЙ КАНОНИЧЕСКИЙ БЭКТРЕКИНГ ---
    let mut result = Vec::new();
    let mut x_pos = n as isize;
    let mut y_pos = m as isize;

    // Восстанавливаем путь, двигаясь по истории шагов d назад
    for d in (0..history.len()).rev() {
        let d_isize = d as isize;
        let k = x_pos - y_pos;
        let idx = (k + max_d_isize) as usize;

        let v_step = &history[d];

        // Определяем направление на шаге d по каноническому правилу Майерса
        let move_down = if k == -d_isize {
            true // Достигли левой границы — шаг мог быть сделан только сверху
        } else if k == d_isize {
            false // Достигли правой границы — шаг мог быть сделан только слева
        } else {
            v_step[idx - 1] < v_step[idx + 1]
        };

        // Вычисляем k, с которого мы пришли на текущую диагональ
        let prev_k = if move_down { k + 1 } else { k - 1 };
        let prev_idx = (prev_k + max_d_isize) as usize;

        // Предыдущий x — это значение из вектора v_step для предыдущей диагонали.
        // Но если мы шли ВПРАВО (удаление), то к значению v[idx-1] на прямом ходу прибавлялась 1.
        // Значит, исходный x перед змейкой равен v_step[prev_idx] для move_down,
        // и v_step[prev_idx] + 1 для движения вправо.
        let prev_x = if move_down {
            v_step[prev_idx]
        } else {
            v_step[prev_idx] + 1
        };

        let prev_y = prev_x - k;

        // 1. Откатываем змейку (совпадающие элементы) до точки разветвления шага d
        while x_pos > prev_x && y_pos > prev_y {
            result.push(DiffOp::Unchanged(&x[(x_pos - 1) as usize]));
            x_pos -= 1;
            y_pos -= 1;
        }

        // 2. Откатываем само одиночное изменение шага d
        if d > 0 {
            if move_down {
                result.push(DiffOp::Added(&y[(y_pos - 1) as usize]));
                y_pos -= 1;
            } else {
                result.push(DiffOp::Removed(&x[(x_pos - 1) as usize]));
                x_pos -= 1;
            }
        }
    }

    // Зачищаем остатки у координат (0,0) — для асимметричных / пустых массивов
    while x_pos > 0 && y_pos > 0 {
        result.push(DiffOp::Unchanged(&x[(x_pos - 1) as usize]));
        x_pos -= 1;
        y_pos -= 1;
    }
    while y_pos > 0 {
        result.push(DiffOp::Added(&y[(y_pos - 1) as usize]));
        y_pos -= 1;
    }
    while x_pos > 0 {
        result.push(DiffOp::Removed(&x[(x_pos - 1) as usize]));
        x_pos -= 1;
    }

    result.reverse();
    result
}

pub fn print_diff<T: std::fmt::Debug>(diff_ops: &[DiffOp<T>]) {
    for op in diff_ops {
        println!("{op}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Структура стала проще — ей больше не нужны три лайфтайма в объявлении
    struct TestCase {
        x: &'static [&'static str],
        y: &'static [&'static str],
        diff: &'static [DiffOp<'static, 'static, &'static str>],
    }

    // Собираем ВСЕ логические сценарии в одном месте, используя тип &'str
    const STR_TEST_CASES: &[TestCase] = &[
        // 1. Полное совпадение
        TestCase {
            x: &["A", "B", "C"],
            y: &["A", "B", "C"],
            diff: &[
                DiffOp::Unchanged(&"A"),
                DiffOp::Unchanged(&"B"),
                DiffOp::Unchanged(&"C"),
            ],
        },
        // 2. Полное несовпадение
        TestCase {
            x: &["A", "B", "C"],
            y: &["X", "Y", "Z"],
            diff: &[
                DiffOp::Removed(&"A"),
                DiffOp::Removed(&"B"),
                DiffOp::Removed(&"C"),
                DiffOp::Added(&"X"),
                DiffOp::Added(&"Y"),
                DiffOp::Added(&"Z"),
            ],
        },
        // 3. Замена в середине
        TestCase {
            x: &["A", "B", "C"],
            y: &["A", "Y", "C"],
            diff: &[
                DiffOp::Unchanged(&"A"),
                DiffOp::Removed(&"B"),
                DiffOp::Added(&"Y"),
                DiffOp::Unchanged(&"C"),
            ],
        },
        // 4. Перестановка / Сдвиг
        TestCase {
            x: &["A", "B", "C"],
            y: &["B", "A", "C"],
            diff: &[
                DiffOp::Removed(&"A"),
                DiffOp::Unchanged(&"B"),
                DiffOp::Added(&"A"),
                DiffOp::Unchanged(&"C"),
            ],
        },
        // --- КРИТИЧЕСКИЕ КРАЕВЫЕ СЛУЧАИ ДЛЯ МАЙЕРСА ---
        // 5. Оба массива пустые
        TestCase {
            x: &[],
            y: &[],
            diff: &[],
        },
        // 6. Первый массив пустой (только добавления)
        TestCase {
            x: &[],
            y: &["A", "B"],
            diff: &[DiffOp::Added(&"A"), DiffOp::Added(&"B")],
        },
        // 7. Второй массив пустой (только удаления)
        TestCase {
            x: &["A", "B"],
            y: &[],
            diff: &[DiffOp::Removed(&"A"), DiffOp::Removed(&"B")],
        },
        // 8. Одно длиннее другого с общим префиксом (тест границ k-диагоналей)
        TestCase {
            x: &["A"],
            y: &["A", "B", "C", "D"],
            diff: &[
                DiffOp::Unchanged(&"A"),
                DiffOp::Added(&"B"),
                DiffOp::Added(&"C"),
                DiffOp::Added(&"D"),
            ],
        },
    ];

    #[test]
    fn test_diff_logic() {
        for (i, case) in STR_TEST_CASES.iter().enumerate() {
            // Передаем слайсы напрямую без .to_vec(), это быстрее и чище
            let actual = diff(case.x, case.y);
            assert_eq!(actual, case.diff, "Поломка в тест-кейсе номер {}", i);
        }
    }
}
