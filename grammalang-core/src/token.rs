// grammalang-core/src/token.rs

use serde::{Deserialize, Serialize};
use std::fmt;

/// Позиция в исходном файле
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    pub line: usize,   // строка, начиная с 1
    pub column: usize, // столбец, начиная с 1
    pub offset: usize, // смещение в байтах от начала файла
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "строка {}, столбец {}", self.line, self.column)
    }
}

/// Вид токена
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenKind {
    // Ключевые слова
    Функция,
    Вернуть,
    Если,
    Иначе,
    Сопоставить,
    Структура,
    Тип,
    Изм,
    Внутри,
    Вместе,
    Макрос,
    Открыто,
    Импорт,
    Модуль,
    Ручной,
    Цитировать,
    Вставить,
    Для,
    Каждого,
    Из,
    Пока,
    Где,
    Истина,
    Ложь,
    Ничего,
    Значение,
    Провал,
    Успех,

    // Идентификаторы и литералы
    Идентификатор(String),
    Целое(i64),
    Десятичное(f64),
    Строка(String),

    // Операторы
    Плюс,           // +
    Минус,          // -
    Звёздочка,      // *
    Слэш,           // /
    Процент,        // %
    Равно,          // =
    ДваРавно,       // ==
    НеРавно,        // !=
    Меньше,         // <
    Больше,         // >
    МеньшеРавно,    // <=
    БольшеРавно,    // >=
    Стрелка,        // ->
    Конвейер,       // |>
    Композиция,     // >>
    Амперсанд,      // &
    ВертикальнаяЧерта, // |
    Вопрос,         // ?
    Подчёркивание,  // _
    Точка,          // .
    Двоеточие,      // :
    Запятая,        // ,
    ТочкаСЗапятой,  // ;
    Многоточие,     // ...

    // Скобки
    КруглаяОткрыто,
    КруглаяЗакрыто,
    ФигурнаяОткрыто,
    ФигурнаяЗакрыто,
    КвадратнаяОткрыто,
    КвадратнаяЗакрыто,

    // Специальные
    Отступ,            // увеличение отступа
    ОтменаОтступа,     // уменьшение отступа
    КонецФайла,
    Комментарий(String),
    Документация(String),
    Ошибка(String),
}

/// Токен — атомарная единица языка
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,  // исходный текст токена
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: impl Into<String>, span: Span) -> Self {
        Token {
            kind,
            lexeme: lexeme.into(),
            span,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ('{}')", self.kind, self.lexeme)
    }
}
