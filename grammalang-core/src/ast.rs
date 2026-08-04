// grammalang-core/src/ast.rs

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
    
    ЦиклДля {
        переменная: String,
        итератор: Box<Ast>,
        тело: Box<Ast>,
        span: Span,
    },
    
    ЦиклПока {
        условие: Box<Ast>,
        тело: Box<Ast>,
        метка: Option<String>,
        span: Span,
    },
    
    Цикл {
        тело: Box<Ast>,
        метка: Option<String>,
        span: Span,
    },
    
    Прервать {
        метка: Option<String>,
        значение: Option<Box<Ast>>,
        span: Span,
    },
    
    Продолжить {
        метка: Option<String>,
        span: Span,
    },
    
    Пусть {
        имя: String,
        тип_аннотация: Option<Тип>,
        изменяемая: bool,
        значение: Box<Ast>,
        span: Span,
    },
    
    БлокОбласти {
        выражения: Vec<Ast>,
        последнее: Option<Box<Ast>>,
        замыкания: Vec<Захват>,
        span: Span,
    },
    
    ПрисваиваниеСОперацией {
        имя: String,
        оператор: БинарныйОператор,
        значение: Box<Ast>,
        span: Span,
    },
    
    ПрисваиваниеОбразца {
        образец: Образец,
        значение: Box<Ast>,
        span: Span,
    },
    
    // ✅ Обновление иммутабельной структуры: объект с { поле = значение, ... }
    ОбновлениеСтруктуры {
        объект: Box<Ast>,
        поля: Vec<(String, Ast)>,
        тип: Option<Тип>,
        span: Span,
    },
    
    // Существующие варианты
    Блок { выражения: Vec<Ast>, span: Span },
    Присваивание { имя: String, тип_аннотация: Option<Тип>, изменяемая: bool, значение: Box<Ast>, span: Span },
    ДвоичноеВыражение { левое: Box<Ast>, оператор: БинарныйОператор, правое: Box<Ast>, тип: Option<Тип>, span: Span },
    УнарноеВыражение { оператор: УнарныйОператор, операнд: Box<Ast>, тип: Option<Тип>, span: Span },
    Вызов { функция: Box<Ast>, аргументы: Vec<Ast>, тип: Option<Тип>, span: Span },
    Лямбда { параметры: Vec<Параметр>, возвращаемый_тип: Option<Тип>, тело: Box<Ast>, span: Span },
    Сопоставление { значение: Box<Ast>, ветки: Vec<ВеткаСопоставления>, тип: Option<Тип>, span: Span },
    Если { условие: Box<Ast>, то: Box<Ast>, иначе: Option<Box<Ast>>, тип: Option<Тип>, span: Span },
    Пока { условие: Box<Ast>, тело: Box<Ast>, span: Span },
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
pub struct Захват {
    pub имя: String,
    pub по_ссылке: bool,
    pub изменяемый: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Диапазон {
    pub начало: Box<Ast>,
    pub конец: Box<Ast>,
    pub включая: bool,
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
    Переменная(String),
    Подчёркивание,
    Литерал(Значение),
    Конструктор { имя: String, вложенный: Option<Box<Образец>> },
    Кортеж(Vec<Образец>),
    Список { элементы: Vec<Образец>, хвост: Option<Box<Образец>> },
    Структура { имя: String, поля: Vec<(String, Образец)>, открытый: bool },
    Диапазон { начало: Значение, конец: Значение },
    // Новые варианты для полноценного сопоставления
    Или(Box<Образец>, Box<Образец>),
    Привязка { имя: String, образец: Box<Образец> },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum БинарныйОператор {
    Сложение, Вычитание, Умножение, Деление, Остаток,
    Равно, НеРавно, Меньше, Больше, МеньшеРавно, БольшеРавно,
    И, Или, Конкатенация,
    ПобитовоеИ, ПобитовоеИли, ПобитовоеИсключающееИли,
    СдвигВлево, СдвигВправо,
    Присвоить,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum УнарныйОператор { 
    Отрицание, Не, Вопрос,
    Ссылка, Разыменование,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Значение { 
    Целое(i64), Десятичное(f64), Строка(String), Булево(bool), Ничего,
    Символ(char),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum СтратегияПараллельности { БыстрыйОтказ, СобратьВсе }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum АргументМакроса {
    Выражение(Box<Ast>), Тип(Тип), Блок(Box<Ast>),
    Идентификатор(String), Образец(Образец), Объявление(Box<Ast>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ПримитивныйТип { 
    Целое, Десятичное, Булево, Строка,
    Символ, Байт, БеззнаковоеЦелое,
}

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
    Массив { тип: Box<Тип>, размер: Option<usize> },
    Срез { тип: Box<Тип> },
    Диапазон,
    Кортеж(Vec<Тип>),
    Указатель { изменяемый: bool, тип: Box<Тип> },
}

// Вспомогательные методы
impl Ast {
    pub fn span(&self) -> &Span {
        match self {
            Ast::Модуль { span, .. } => span,
            Ast::ОбъявлениеФункции { span, .. } => span,
            Ast::ОбъявлениеСтруктуры { span, .. } => span,
            Ast::ОбъявлениеСуммы { span, .. } => span,
            Ast::ОбъявлениеИмпорта { span, .. } => span,
            Ast::ОбъявлениеВнешнейФункции { span, .. } => span,
            Ast::ЦиклДля { span, .. } => span,
            Ast::ЦиклПока { span, .. } => span,
            Ast::Цикл { span, .. } => span,
            Ast::Прервать { span, .. } => span,
            Ast::Продолжить { span, .. } => span,
            Ast::Пусть { span, .. } => span,
            Ast::БлокОбласти { span, .. } => span,
            Ast::ПрисваиваниеСОперацией { span, .. } => span,
            Ast::ПрисваиваниеОбразца { span, .. } => span,
            Ast::ОбновлениеСтруктуры { span, .. } => span,
            Ast::Блок { span, .. } => span,
            Ast::Присваивание { span, .. } => span,
            Ast::ДвоичноеВыражение { span, .. } => span,
            Ast::УнарноеВыражение { span, .. } => span,
            Ast::Вызов { span, .. } => span,
            Ast::Лямбда { span, .. } => span,
            Ast::Сопоставление { span, .. } => span,
            Ast::Если { span, .. } => span,
            Ast::Пока { span, .. } => span,
            Ast::Возврат { span, .. } => span,
            Ast::КонструкторСтруктуры { span, .. } => span,
            Ast::КонструкторСуммы { span, .. } => span,
            Ast::ДоступКПолю { span, .. } => span,
            Ast::Заимствование { span, .. } => span,
            Ast::Перемещение { span, .. } => span,
            Ast::Цитирование { span, .. } => span,
            Ast::Вставка { span, .. } => span,
            Ast::БлокЭффекта { span, .. } => span,
            Ast::ПараллельныйБлок { span, .. } => span,
            Ast::РучнойБлок { span, .. } => span,
            Ast::Переменная { span, .. } => span,
            Ast::Литерал { span, .. } => span,
            Ast::ВызовМакроса { span, .. } => span,
        }
    }
    
    pub fn это_выражение(&self) -> bool {
        matches!(self,
            Ast::Литерал { .. } |
            Ast::Переменная { .. } |
            Ast::ДвоичноеВыражение { .. } |
            Ast::УнарноеВыражение { .. } |
            Ast::Вызов { .. } |
            Ast::Лямбда { .. } |
            Ast::Сопоставление { .. } |
            Ast::Если { .. } |
            Ast::Блок { .. } |
            Ast::БлокОбласти { .. } |
            Ast::Пусть { .. } |
            Ast::Цикл { .. } |
            Ast::ЦиклПока { .. } |
            Ast::ЦиклДля { .. } |
            Ast::КонструкторСтруктуры { .. } |
            Ast::КонструкторСуммы { .. } |
            Ast::ДоступКПолю { .. } |
            Ast::ОбновлениеСтруктуры { .. }
        )
    }

    /// Вспомогательный метод для клонирования оператора из ПрисваиваниеСОперацией
    pub fn clone_operator(&self) -> Option<БинарныйОператор> {
        match self {
            Ast::ПрисваиваниеСОперацией { оператор, .. } => Some(оператор.clone()),
            _ => None,
        }
    }
}
