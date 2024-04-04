/// Обобщённая структура данных стек, реализованная с использованием вектора.
pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    /// Создаёт новый пустой стек.
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    /// Возвращает количество элементов в стеке.
    pub fn length(&self) -> usize {
        self.stack.len()
    }

    /// Удаляет и возвращает верхний элемент из стека, если он есть.
    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    /// Добавляет элемент на вершину стека.
    pub fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    /// Возвращает ссылку на верхний элемент стека, если он есть.
    pub fn peek(&self) -> Option<&T> {
        self.stack.last()
    }

    /// Проверяет, пуст ли стек.
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

fn main() {
    // Создание нового пустого стека типа `isize`.
    let mut stack: Stack<isize> = Stack::new();

    // Добавление элемента со значением 1 в стек.
    stack.push(1);

    // Проверка, что стек не пустой.
    assert!(!stack.is_empty());

    // Проверка, что длина стека равна 1.
    assert_eq!(stack.length(), 1);

    // Добавление элемента со значением 2 в стек.
    stack.push(2);

    // Получение ссылки на верхний элемент стека и проверка его значения.
    let item = stack.peek();
    assert_eq!(*item.unwrap(), 2);

    // Удаление верхнего элемента из стека и проверка его значения.
    let item = stack.pop();
    assert_eq!(item.unwrap(), 2);
}
