use serde::{Deserialize, Serialize};
use crate::token::Span;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Ast {
    Модуль { имя: String, объявления: Vec<Ast>, span: Span },
    ОбъявлениеФункции {
        имя: String, параметры_типа: Vec<ПараметрТипа>, параметры: Vec<Параметр>,
        возвращаемый_тип: Option<Тип>, тело: Box<Ast>, открыто: bool, span: Span,
    },
    ОбъявлениеСтруктуры {
        имя: String, параметры_типа: Vec<ПараметрТипа>, поля: Vec<(String, Тип)>,
        открыто: bool, span: Span,
    },
    ОбъявлениеСуммы {
        имя: String, параметры_типа: Vec<ПараметрТипа>, варианты: Vec<ВариантСуммы>,
        открыто: bool, span: Span,
    },
    ОбъявлениеИмпорта { путь: Vec<String>, имена: Vec<(String, Option<String>)>, span: Span },
    ОбъявлениеВнешнейФункции { язык: String, имя: String, параметры: Vec<Тип>, возвращаемый_тип: Option<Тип>, span: Span },
    Блок { выражения: Vec<Ast>, span: Span },
    Присваивание { имя: String, тип_аннотация: Option<Тип>, изменяемая: bool, значение: Box<Ast>, span: Span },
    ДвоичноеВыражение { левое: Box<Ast>, оператор: БинарныйОператор, правое: Box<Ast>, тип: Option<Тип>, span: Span },
    УнарноеВыражение { оператор: УнарныйОператор, операнд: Box<Ast>, тип: Option<Тип>, span: Span },
    Вызов { функция: Box<Ast>, аргументы: Vec<Ast>, тип: Option<Тип>, span: Span },
    Лямбда { параметры: Vec<Параметр>, возвращаемый_тип: Option<Тип>, тело: Box<Ast>, span: Span },
    Сопоставление { значение: Box<Ast>, ветки: Vec<ВеткаСопоставления>, тип: Option<Тип>, span: Span },
    Если { условие: Box<Ast>, то: Box<Ast>, иначе: Option<Box<Ast>>, тип: Option<Тип>, span: Span },
    Возврат { значение: Option<Box<Ast>>, span: Span },
    КонструкторСтруктуры { имя: String, поля: Vec<(String, Ast)>, тип: Option<Тип>, span: Span },
    КонструкторСуммы { имя: String, значение: Option<Box<Ast>>, тип: Option<Тип>, span: Span },
    ДоступКПолю { объект: Box<Ast>, поле: String, тип: Option<Тип>, span: Span },
    Заимствование { изменяемое: bool, значение: Box<Ast>, тип: Option<Тип>, span: Span },
    Перемещение { значение: Box<Ast>, span: Span },
    Цитирование { тело: Box<Ast>, span: Span },
    Вставка { значение: Box<Ast>, span: Span },
    БлокЭффекта { эффекты: Vec<String>, тело: Box<Ast>, span: Span },
    ПараллельныйБлок { стратегия: СтратегияПараллельности, тело: Box<Ast>, span: Span },
    РучнойБлок { тело: Box<Ast>, span: Span },
    Переменная { имя: String, тип: Option<Тип>, span: Span },
    Литерал { значение: Значение, span: Span },
    ВызовМакроса { имя: String, аргументы: Vec<АргументМакроса>, span: Span },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ПараметрТипа { pub имя: String, pub ограничения: Vec<String> }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Параметр { pub имя: String, pub тип: Тип, pub изменяемый: bool }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ВариантСуммы { pub имя: String, pub тип_данных: Option<Тип> }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ВеткаСопоставления { pub образец: Образец, pub условие: Option<Box<Ast>>, pub тело: Box<Ast> }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Образец {
    Переменная(String), Подчёркивание, Литерал(Значение),
    Конструктор { имя: String, вложенный: Option<Box<Образец>> },
    Кортеж(Vec<Образец>), Список { элементы: Vec<Образец>, хвост: Option<Box<Образец>> },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum БинарныйОператор {
    Сложение, Вычитание, Умножение, Деление, Остаток,
    Равно, НеРавно, Меньше, Больше, МеньшеРавно, БольшеРавно,
    И, Или, Конкатенация,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum УнарныйОператор { Отрицание, Не, Вопрос }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Значение { Целое(i64), Десятичное(f64), Строка(String), Булево(bool), Ничего }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum СтратегияПараллельности { БыстрыйОтказ, СобратьВсе }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum АргументМакроса {
    Выражение(Box<Ast>), Тип(Тип), Блок(Box<Ast>),
    Идентификатор(String), Образец(Образец), Объявление(Box<Ast>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ПримитивныйТип { Целое, Десятичное, Булево, Строка }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Тип {
    Переменная(String), Примитивный(ПримитивныйТип),
    Параметризованный { имя: String, параметры: Vec<Тип> },
    Функция { аргументы: Vec<Тип>, результат: Box<Тип> },
    Запись(Vec<(String, Тип)>), Сумма(Vec<(String, Option<Тип>)>),
    Ссылка { изменяемая: bool, тип: Box<Тип> },
    Эффект { эффект: String, тип: Box<Тип> },
    Уточнённый { базовый: Box<Тип>, условие: Box<Ast> },
    Единичный, Пустой,
}
