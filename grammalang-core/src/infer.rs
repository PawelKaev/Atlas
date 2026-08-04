// grammalang-core/src/infer.rs
// Версия 2.0 — полноценная проверка образцов (patcheck)

use std::collections::{HashMap, HashSet};
use crate::ast::*;
use crate::error::{Diagnostic, DiagnosticKind};
use crate::token::Span;
use crate::types::{Constraint, Substitution, fresh_var, occurs_in};

type TypeContext = HashMap<String, Тип>;

#[derive(Debug, Clone, Default)]
pub struct BindingsMap {
    bindings: Vec<(String, Тип)>,
}

impl BindingsMap {
    pub fn new() -> Self { BindingsMap { bindings: Vec::new() } }
    pub fn singleton(name: String, typ: Тип) -> Self { BindingsMap { bindings: vec![(name, typ)] } }
    pub fn empty() -> Self { BindingsMap::new() }
    
    pub fn insert(&mut self, name: String, typ: Тип) {
        self.bindings.push((name, typ));
    }
    
    pub fn merge(&mut self, other: BindingsMap) {
        self.bindings.extend(other.bindings);
    }
    
    pub fn names(&self) -> Vec<String> {
        self.bindings.iter().map(|(n, _)| n.clone()).collect()
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &(String, Тип)> {
        self.bindings.iter()
    }
    
    pub fn into_context(self, ctx: &mut TypeContext) {
        for (name, typ) in self.bindings {
            ctx.insert(name, typ);
        }
    }
}

pub struct Inferrer {
    context: TypeContext,
    constraints: Vec<Constraint>,
    errors: Vec<Diagnostic>,
    expected_return_type: Option<Тип>,
    concepts: HashMap<String, Vec<Тип>>,
    struct_schemas: HashMap<String, Vec<(String, Тип)>>,
    sum_schemas: HashMap<String, Vec<(String, Option<Тип>)>>,
}

impl Inferrer {
    pub fn new() -> Self {
        let mut concepts = HashMap::new();
        concepts.insert("Число".to_string(), vec![Тип::Примитивный(ПримитивныйТип::Целое), Тип::Примитивный(ПримитивныйТип::Десятичное)]);
        concepts.insert("Сравнимый".to_string(), vec![Тип::Примитивный(ПримитивныйТип::Целое), Тип::Примитивный(ПримитивныйТип::Десятичное), Тип::Примитивный(ПримитивныйТип::Строка), Тип::Примитивный(ПримитивныйТип::Булево), Тип::Примитивный(ПримитивныйТип::Символ)]);
        concepts.insert("Строковый".to_string(), vec![Тип::Примитивный(ПримитивныйТип::Строка)]);
        concepts.insert("Итерируемый".to_string(), vec![Тип::Массив { тип: Box::new(Тип::Переменная("T".to_string())), размер: None }, Тип::Срез { тип: Box::new(Тип::Переменная("T".to_string())) }, Тип::Диапазон]);
        Inferrer { context: HashMap::new(), constraints: Vec::new(), errors: Vec::new(), expected_return_type: None, concepts, struct_schemas: HashMap::new(), sum_schemas: HashMap::new() }
    }

    pub fn register_struct_schema(&mut self, name: &str, fields: Vec<(String, Тип)>) {
        self.struct_schemas.insert(name.to_string(), fields);
    }

    pub fn register_sum_schema(&mut self, name: &str, variants: Vec<(String, Option<Тип>)>) {
        self.sum_schemas.insert(name.to_string(), variants);
    }

    pub fn infer(&mut self, ast: &Ast) -> (Option<Ast>, Vec<Diagnostic>) {
        // Сначала собираем схемы из объявлений
        self.collect_schemas(ast);
        let typed = self.infer_node(ast);
        let mut typed = match typed { Some(ast) => ast, None => return (None, std::mem::take(&mut self.errors)) };
        match self.solve() {
            Ok(mut sub) => { sub.compress_all(); apply_substitution_to_ast(&mut typed, &sub); (Some(typed), std::mem::take(&mut self.errors)) }
            Err(mut solve_errors) => { self.errors.append(&mut solve_errors); (Some(typed), std::mem::take(&mut self.errors)) }
        }
    }

    fn collect_schemas(&mut self, ast: &Ast) {
        match ast {
            Ast::Модуль { объявления, .. } => { for d in объявления { self.collect_schemas(d); } }
            Ast::ОбъявлениеСтруктуры { имя, поля, .. } => {
                self.register_struct_schema(имя, поля.clone());
            }
            Ast::ОбъявлениеСуммы { имя, варианты, .. } => {
                let vars: Vec<(String, Option<Тип>)> = варианты.iter().map(|v| (v.имя.clone(), v.тип_данных.clone())).collect();
                self.register_sum_schema(имя, vars);
            }
            _ => {}
        }
    }

    fn solve(&mut self) -> Result<Substitution, Vec<Diagnostic>> {
        let mut sub = Substitution::new();
        for constraint in std::mem::take(&mut self.constraints) {
            match constraint {
                Constraint::Равенство(t1, t2, span) => { let t1 = sub.apply_mut(&t1); let t2 = sub.apply_mut(&t2); if let Err(mut errors) = self.unify(&t1, &t2, &mut sub) { for err in &mut errors { if err.span.line == 0 { err.span = span; } } return Err(errors); } }
                Constraint::Концепт(typ, концепт, span) => { let resolved = sub.apply_mut(&typ); if !self.check_concept(&resolved, &концепт) { return Err(vec![Diagnostic { kind: DiagnosticKind::Ошибка, message: format!("Тип '{}' не удовлетворяет концепту '{}'", self.type_to_string(&resolved), концепт), span, hint: Some(format!("Концепт '{}' требует: {}", концепт, self.concept_types_string(&концепт))) }]); } }
                _ => continue,
            }
        }
        Ok(sub)
    }

    fn unify(&mut self, t1: &Тип, t2: &Тип, sub: &mut Substitution) -> Result<(), Vec<Diagnostic>> {
        let t1 = sub.apply_mut(t1); let t2 = sub.apply_mut(t2);
        if t1 == t2 { return Ok(()); }
        match (&t1, &t2) {
            (Тип::Переменная(v), other) | (other, Тип::Переменная(v)) => { if occurs_in(v, other) { return Err(vec![Diagnostic { kind: DiagnosticKind::Ошибка, message: format!("Бесконечный тип: '{}' содержит '{}'", v, self.type_to_string(other)), span: Span { line: 0, column: 0, offset: 0 }, hint: None }]); } sub.insert(v.clone(), other.clone()); Ok(()) }
            (Тип::Примитивный(p1), Тип::Примитивный(p2)) => if p1 == p2 { Ok(()) } else { Err(vec![self.type_mismatch(&t1, &t2)]) }
            (Тип::Функция { аргументы: a1, результат: r1 }, Тип::Функция { аргументы: a2, результат: r2 }) => { if a1.len() != a2.len() { return Err(vec![Diagnostic { kind: DiagnosticKind::Ошибка, message: format!("Арность: {} vs {}", a1.len(), a2.len()), span: Span { line: 0, column: 0, offset: 0 }, hint: None }]); } for (x, y) in a1.iter().zip(a2) { self.unify(x, y, sub)?; } self.unify(r1, r2, sub) }
            (Тип::Запись(f1), Тип::Запись(f2)) => { if f1.len() != f2.len() { return Err(vec![Diagnostic { kind: DiagnosticKind::Ошибка, message: format!("Размер записи: {} vs {}", f1.len(), f2.len()), span: Span { line: 0, column: 0, offset: 0 }, hint: None }]); } for ((n1, t1), (n2, t2)) in f1.iter().zip(f2) { if n1 != n2 { return Err(vec![Diagnostic { kind: DiagnosticKind::Ошибка, message: format!("Поле '{}' vs '{}'", n1, n2), span: Span { line: 0, column: 0, offset: 0 }, hint: None }]); } self.unify(t1, t2, sub)?; } Ok(()) }
            _ => Err(vec![self.type_mismatch(&t1, &t2)]),
        }
    }

    fn check_concept(&self, typ: &Тип, концепт: &str) -> bool { self.concepts.get(концепт).map_or(true, |allowed| allowed.iter().any(|a| self.types_match(typ, a))) }
    fn types_match(&self, t1: &Тип, t2: &Тип) -> bool { match (t1, t2) { (Тип::Примитивный(p1), Тип::Примитивный(p2)) => p1 == p2, (Тип::Переменная(_), _) | (_, Тип::Переменная(_)) => true, _ => false } }
    fn concept_types_string(&self, c: &str) -> String { self.concepts.get(c).map_or("любой".into(), |t| t.iter().map(|x| self.type_to_string(x)).collect::<Vec<_>>().join(", ")) }

    // ==================== check_pattern ====================

    fn check_pattern(&mut self, pattern: &Образец, expected_type: &Тип, span: Span) -> Result<BindingsMap, Vec<Diagnostic>> {
        match pattern {
            Образец::Переменная(name) => {
                Ok(BindingsMap::singleton(name.clone(), expected_type.clone()))
            }
            
            Образец::Подчёркивание => {
                Ok(BindingsMap::empty())
            }
            
            Образец::Литерал(val) => {
                let lit_type = self.literal_type(val);
                match self.unify_silent(expected_type, &lit_type) {
                    Ok(_) => Ok(BindingsMap::empty()),
                    Err(e) => Err(vec![e]),
                }
            }
            
            Образец::Конструктор { имя, вложенный } => {
                let variants = self.resolve_sum_variants(expected_type);
                
                let variant = variants.iter().find(|(n, _)| n == имя)
                    .ok_or_else(|| vec![Diagnostic {
                        kind: DiagnosticKind::Ошибка,
                        message: format!("Вариант '{}' не найден в типе '{}'", имя, self.type_to_string(expected_type)),
                        span,
                        hint: None,
                    }])?;
                
                match (&variant.1, вложенный) {
                    (Some(var_type), Some(inner_pattern)) => {
                        self.check_pattern(inner_pattern, var_type, span)
                    }
                    (None, None) => Ok(BindingsMap::empty()),
                    (Some(_), None) => Err(vec![Diagnostic {
                        kind: DiagnosticKind::Ошибка,
                        message: format!("Вариант '{}' требует вложенный образец", имя),
                        span,
                        hint: None,
                    }]),
                    (None, Some(_)) => Err(vec![Diagnostic {
                        kind: DiagnosticKind::Ошибка,
                        message: format!("Вариант '{}' не принимает вложенный образец", имя),
                        span,
                        hint: None,
                    }]),
                }
            }
            
            Образец::Структура { имя, поля, открытый: _ } => {
                let struct_fields = self.resolve_struct_fields(expected_type, имя);
                
                let mut bindings = BindingsMap::empty();
                
                for (field_name, field_pattern) in поля {
                    let field_type = struct_fields.iter()
                        .find(|(n, _)| n == field_name)
                        .map(|(_, t)| t.clone())
                        .ok_or_else(|| vec![Diagnostic {
                            kind: DiagnosticKind::Ошибка,
                            message: format!("Поле '{}' не найдено в структуре '{}'", field_name, имя),
                            span,
                            hint: None,
                        }])?;
                    
                    bindings.merge(self.check_pattern(field_pattern, &field_type, span)?);
                }
                
                Ok(bindings)
            }
            
            Образец::Или(left, right) => {
                let left_bindings = self.check_pattern(left, expected_type, span)?;
                let right_bindings = self.check_pattern(right, expected_type, span)?;
                
                let left_names: HashSet<_> = left_bindings.names().into_iter().collect();
                let right_names: HashSet<_> = right_bindings.names().into_iter().collect();
                
                if left_names != right_names {
                    return Err(vec![Diagnostic {
                        kind: DiagnosticKind::Ошибка,
                        message: "Ветви Или-образца должны связывать одинаковые переменные".to_string(),
                        span,
                        hint: None,
                    }]);
                }
                
                Ok(left_bindings)
            }
            
            Образец::Привязка { имя, образец } => {
                let mut bindings = self.check_pattern(образец, expected_type, span)?;
                bindings.insert(имя.clone(), expected_type.clone());
                Ok(bindings)
            }
            
            Образец::Кортеж(элементы) => {
                let tuple_types = match expected_type {
                    Тип::Кортеж(types) => types.clone(),
                    _ => return Err(vec![Diagnostic {
                        kind: DiagnosticKind::Ошибка,
                        message: "Кортежный образец требует кортежный тип".to_string(),
                        span,
                        hint: None,
                    }]),
                };
                
                if элементы.len() != tuple_types.len() {
                    return Err(vec![Diagnostic {
                        kind: DiagnosticKind::Ошибка,
                        message: format!("Несовпадение длины кортежа: {} vs {}", элементы.len(), tuple_types.len()),
                        span,
                        hint: None,
                    }]);
                }
                
                let mut bindings = BindingsMap::empty();
                for (elem_pattern, elem_type) in элементы.iter().zip(tuple_types.iter()) {
                    bindings.merge(self.check_pattern(elem_pattern, elem_type, span)?);
                }
                Ok(bindings)
            }
            
            _ => Err(vec![Diagnostic {
                kind: DiagnosticKind::Ошибка,
                message: format!("Неподдерживаемый образец: {:?}", pattern),
                span,
                hint: None,
            }]),
        }
    }

    fn resolve_sum_variants(&self, expected_type: &Тип) -> Vec<(String, Option<Тип>)> {
        match expected_type {
            Тип::Сумма(v) => v.clone(),
            Тип::Переменная(name) => self.sum_schemas.get(name).cloned().unwrap_or_default(),
            Тип::Параметризованный { имя, .. } => self.sum_schemas.get(имя).cloned().unwrap_or_default(),
            _ => Vec::new(),
        }
    }

    fn resolve_struct_fields(&self, expected_type: &Тип, _struct_name: &str) -> Vec<(String, Тип)> {
        match expected_type {
            Тип::Запись(f) => f.clone(),
            Тип::Переменная(name) => self.struct_schemas.get(name).cloned().unwrap_or_default(),
            Тип::Параметризованный { имя, .. } => self.struct_schemas.get(имя).cloned().unwrap_or_default(),
            _ => Vec::new(),
        }
    }

    fn unify_silent(&self, t1: &Тип, t2: &Тип) -> Result<(), Diagnostic> {
        if t1 == t2 { return Ok(()); }
        match (t1, t2) {
            (Тип::Переменная(_), _) | (_, Тип::Переменная(_)) => Ok(()),
            (Тип::Примитивный(p1), Тип::Примитивный(p2)) if p1 == p2 => Ok(()),
            _ => Err(self.type_mismatch(t1, t2)),
        }
    }

    fn literal_type(&self, val: &Значение) -> Тип {
        match val {
            Значение::Целое(_) => Тип::Примитивный(ПримитивныйТип::Целое),
            Значение::Десятичное(_) => Тип::Примитивный(ПримитивныйТип::Десятичное),
            Значение::Строка(_) => Тип::Примитивный(ПримитивныйТип::Строка),
            Значение::Булево(_) => Тип::Примитивный(ПримитивныйТип::Булево),
            Значение::Символ(_) => Тип::Примитивный(ПримитивныйТип::Символ),
            Значение::Ничего => Тип::Пустой,
        }
    }

    // ==================== infer_node ====================

    fn infer_node(&mut self, node: &Ast) -> Option<Ast> {
        match node {
            Ast::Модуль { имя, объявления, span } => { Some(Ast::Модуль { имя: имя.clone(), объявления: объявления.iter().filter_map(|d| self.infer_node(d)).collect(), span: *span }) }
            Ast::ОбъявлениеФункции { имя, параметры_типа, параметры, возвращаемый_тип, тело, открыто, span } => {
                let saved = self.expected_return_type.clone(); self.expected_return_type = возвращаемый_тип.clone();
                let mut type_param_map = HashMap::new();
                for tp in параметры_типа { let v = fresh_var(); type_param_map.insert(tp.имя.clone(), v.clone()); self.context.insert(tp.имя.clone(), v); }
                let resolved_params: Vec<Параметр> = параметры.iter().map(|p| Параметр { имя: p.имя.clone(), тип: substitute_type_vars(&p.тип, &type_param_map), изменяемый: p.изменяемый }).collect();
                let mut saved_vars = Vec::new();
                for p in &resolved_params { let t = if p.тип != Тип::Пустой { p.тип.clone() } else { fresh_var() }; saved_vars.push((p.имя.clone(), self.context.insert(p.имя.clone(), t))); }
                let typed_body = self.infer_node(тело)?;
                for (name, old) in saved_vars { if let Some(t) = old { self.context.insert(name, t); } else { self.context.remove(&name); } }
                self.expected_return_type = saved;
                Some(Ast::ОбъявлениеФункции { имя: имя.clone(), параметры_типа: параметры_типа.clone(), параметры: resolved_params, возвращаемый_тип: возвращаемый_тип.clone(), тело: Box::new(typed_body), открыто: *открыто, span: *span })
            }
            Ast::Блок { выражения, span } => { Some(Ast::Блок { выражения: выражения.iter().filter_map(|e| self.infer_node(e)).collect(), span: *span }) }
            Ast::Пусть { имя, тип_аннотация, изменяемая, значение, span } | Ast::Присваивание { имя, тип_аннотация, изменяемая, значение, span } => {
                let typed_val = self.infer_node(значение)?; let _vt = self.get_type(&typed_val).unwrap_or_else(fresh_var);
                if let (Some(annot), Some(ref vtp)) = (тип_аннотация, &self.get_type(&typed_val)) { self.constraints.push(Constraint::Равенство(vtp.clone(), annot.clone(), *span)); }
                let ft = тип_аннотация.clone().or_else(|| self.get_type(&typed_val)).unwrap_or_else(fresh_var);
                self.context.insert(имя.clone(), ft.clone());
                Some(Ast::Пусть { имя: имя.clone(), тип_аннотация: Some(ft.clone()), изменяемая: *изменяемая, значение: Box::new(typed_val), span: *span })
            }
            Ast::ПрисваиваниеСОперацией { имя, оператор, значение, span } => {
                let typed_val = self.infer_node(значение)?; let vt = self.get_type(&typed_val).unwrap_or_else(fresh_var);
                if let Some(var_type) = self.context.get(имя) { self.constraints.push(Constraint::Равенство(var_type.clone(), vt, *span)); }
                Some(Ast::ПрисваиваниеСОперацией { имя: имя.clone(), оператор: оператор.clone(), значение: Box::new(typed_val), span: *span })
            }
            Ast::ПрисваиваниеОбразца { образец, значение, span } => {
                let typed_val = self.infer_node(значение)?;
                let vt = self.get_type(&typed_val).unwrap_or_else(fresh_var);
                if let Ok(bindings) = self.check_pattern(образец, &vt, *span) {
                    bindings.into_context(&mut self.context);
                }
                Some(Ast::ПрисваиваниеОбразца { образец: образец.clone(), значение: Box::new(typed_val), span: *span })
            }

            // Обновление структуры
            Ast::ОбновлениеСтруктуры { объект, поля, span, .. } => {
                let typed_obj = self.infer_node(объект)?;
                let obj_type = self.get_type(&typed_obj).unwrap_or_else(fresh_var);
                if let Тип::Запись(existing_fields) = &obj_type {
                    for (name, _) in поля {
                        if !existing_fields.iter().any(|(n, _)| n == name) {
                            self.errors.push(Diagnostic { kind: DiagnosticKind::Ошибка, message: format!("Поле '{}' не найдено в структуре", name), span: *span, hint: None });
                            return None;
                        }
                    }
                    let typed_fields: Vec<(String, Ast)> = поля.iter().map(|(n, v)| self.infer_node(v).map(|ast| (n.clone(), ast))).collect::<Option<Vec<_>>>()?;
                    Some(Ast::ОбновлениеСтруктуры { объект: Box::new(typed_obj), поля: typed_fields, тип: Some(obj_type.clone()), span: *span })
                } else {
                    self.errors.push(Diagnostic { kind: DiagnosticKind::Ошибка, message: "Обновление структуры возможно только для записи".to_string(), span: *span, hint: None });
                    None
                }
            }

            // Сопоставление с проверкой образцов
            Ast::Сопоставление { значение, ветки, span, .. } => {
                let typed_val = self.infer_node(значение)?;
                let val_type = self.get_type(&typed_val).unwrap_or_else(fresh_var);
                
                let mut typed_branches = Vec::new();
                let mut result_type = fresh_var();
                let mut first_branch_type: Option<Тип> = None;
                
                for ветка in ветки {
                    // Проверяем образец
                    match self.check_pattern(&ветка.образец, &val_type, *span) {
                        Ok(bindings) => {
                            // Сохраняем текущий контекст
                            let saved_context = self.context.clone();
                            
                            // Добавляем привязки в контекст
                            bindings.into_context(&mut self.context);
                            
                            // Проверяем условие-охранник
                            let typed_guard = ветка.условие.as_ref()
                                .and_then(|g| self.infer_node(g))
                                .map(Box::new);
                            
                            // Выводим тип тела
                            let typed_body = self.infer_node(&ветка.тело)?;
                            let body_type = self.get_type(&typed_body).unwrap_or_else(fresh_var);
                            
                            // Унифицируем с результатом
                            if let Some(ref first) = first_branch_type {
                                self.constraints.push(Constraint::Равенство(body_type.clone(), first.clone(), *span));
                            } else {
                                first_branch_type = Some(body_type.clone());
                            }
                            
                            // Восстанавливаем контекст
                            self.context = saved_context;
                            
                            typed_branches.push(ВеткаСопоставления {
                                образец: ветка.образец.clone(),
                                условие: typed_guard,
                                тело: Box::new(typed_body),
                            });
                        }
                        Err(mut errs) => {
                            self.errors.append(&mut errs);
                            return None;
                        }
                    }
                }
                
                result_type = first_branch_type.unwrap_or(Тип::Пустой);
                Some(Ast::Сопоставление { значение: Box::new(typed_val), ветки: typed_branches, тип: Some(result_type), span: *span })
            }

            Ast::ДвоичноеВыражение { левое, оператор, правое, span, .. } => {
                let tl = self.infer_node(левое)?; let tr = self.infer_node(правое)?;
                let lt = self.get_type(&tl).unwrap_or_else(fresh_var); let rt = self.get_type(&tr).unwrap_or_else(fresh_var);
                let result = match оператор {
                    БинарныйОператор::Сложение | БинарныйОператор::Вычитание | БинарныйОператор::Умножение | БинарныйОператор::Деление | БинарныйОператор::Остаток => { self.constraints.push(Constraint::Равенство(lt.clone(), rt.clone(), *span)); lt.clone() }
                    БинарныйОператор::Равно | БинарныйОператор::НеРавно | БинарныйОператор::Меньше | БинарныйОператор::Больше | БинарныйОператор::МеньшеРавно | БинарныйОператор::БольшеРавно => { self.constraints.push(Constraint::Равенство(lt.clone(), rt.clone(), *span)); Тип::Примитивный(ПримитивныйТип::Булево) }
                    БинарныйОператор::И | БинарныйОператор::Или => Тип::Примитивный(ПримитивныйТип::Булево),
                    _ => lt.clone(),
                };
                Some(Ast::ДвоичноеВыражение { левое: Box::new(tl), оператор: оператор.clone(), правое: Box::new(tr), тип: Some(result), span: *span })
            }
            Ast::Если { условие, то, иначе, span, .. } => {
                let tc = self.infer_node(условие)?; let tt = self.infer_node(то)?;
                let te = иначе.as_ref().and_then(|e| self.infer_node(e));
                let ct = self.get_type(&tc).unwrap_or_else(fresh_var); self.constraints.push(Constraint::Равенство(ct, Тип::Примитивный(ПримитивныйТип::Булево), *span));
                let tt_type = self.get_type(&tt).unwrap_or_else(fresh_var);
                let result = if let Some(ref te_ast) = te { let et = self.get_type(te_ast).unwrap_or_else(fresh_var); self.constraints.push(Constraint::Равенство(tt_type.clone(), et, *span)); tt_type } else { Тип::Пустой };
                Some(Ast::Если { условие: Box::new(tc), то: Box::new(tt), иначе: te.map(Box::new), тип: Some(result), span: *span })
            }
            Ast::Пока { условие, тело, span } | Ast::ЦиклПока { условие, тело, span, .. } => {
                let tc = self.infer_node(условие)?; let tb = self.infer_node(тело)?;
                self.constraints.push(Constraint::Равенство(self.get_type(&tc).unwrap_or_else(fresh_var), Тип::Примитивный(ПримитивныйТип::Булево), *span));
                Some(Ast::Пока { условие: Box::new(tc), тело: Box::new(tb), span: *span })
            }
            Ast::Вызов { функция, аргументы, span, .. } => {
                let tf = self.infer_node(функция)?; let ta: Vec<Ast> = аргументы.iter().filter_map(|a| self.infer_node(a)).collect();
                let result = fresh_var();
                let arg_types: Vec<Тип> = ta.iter().map(|a| self.get_type(a).unwrap_or_else(fresh_var)).collect();
                let ft = Тип::Функция { аргументы: arg_types, результат: Box::new(result.clone()) };
                self.constraints.push(Constraint::Равенство(self.get_type(&tf).unwrap_or_else(fresh_var), ft, *span));
                Some(Ast::Вызов { функция: Box::new(tf), аргументы: ta, тип: Some(result), span: *span })
            }
            Ast::Переменная { имя, span, .. } => { let t = self.context.get(имя).cloned().unwrap_or_else(fresh_var); Some(Ast::Переменная { имя: имя.clone(), тип: Some(t), span: *span }) }
            Ast::Литерал { значение, span } => { Some(Ast::Литерал { значение: значение.clone(), span: *span }) }
            Ast::КонструкторСтруктуры { имя, поля, span, .. } => {
                let tf: Vec<(String, Ast)> = поля.iter().filter_map(|(n, v)| self.infer_node(v).map(|ast| (n.clone(), ast))).collect();
                let ft: Vec<(String, Тип)> = tf.iter().map(|(n, v)| (n.clone(), self.get_type(v).unwrap_or_else(fresh_var))).collect();
                Some(Ast::КонструкторСтруктуры { имя: имя.clone(), поля: tf, тип: Some(Тип::Запись(ft)), span: *span })
            }
            Ast::КонструкторСуммы { имя, значение, span, .. } => {
                let typed_val = значение.as_ref().and_then(|v| self.infer_node(v));
                let inner_type = typed_val.as_ref().and_then(|v| self.get_type(v));
                let sum_type = Тип::Сумма(vec![(имя.clone(), inner_type.clone())]);
                Some(Ast::КонструкторСуммы { имя: имя.clone(), значение: typed_val.map(Box::new), тип: Some(sum_type), span: *span })
            }
            _ => Some(node.clone()),
        }
    }
    
    fn get_type(&self, node: &Ast) -> Option<Тип> {
        match node {
            Ast::ДвоичноеВыражение { тип, .. } | Ast::УнарноеВыражение { тип, .. } | Ast::Вызов { тип, .. } | Ast::Если { тип, .. } | Ast::Переменная { тип, .. } | Ast::КонструкторСтруктуры { тип, .. } | Ast::КонструкторСуммы { тип, .. } | Ast::ДоступКПолю { тип, .. } | Ast::ОбновлениеСтруктуры { тип, .. } | Ast::Сопоставление { тип, .. } => тип.clone(),
            Ast::Литерал { значение, .. } => Some(self.literal_type(значение)),
            _ => None,
        }
    }

    fn type_mismatch(&self, expected: &Тип, found: &Тип) -> Diagnostic { Diagnostic { kind: DiagnosticKind::Ошибка, message: format!("Несоответствие типов: ожидался '{}', получен '{}'", self.type_to_string(expected), self.type_to_string(found)), span: Span { line: 0, column: 0, offset: 0 }, hint: None } }

    fn type_to_string(&self, typ: &Тип) -> String {
        match typ {
            Тип::Примитивный(ПримитивныйТип::Целое) => "Целое".into(),
            Тип::Примитивный(ПримитивныйТип::Десятичное) => "Десятичное".into(),
            Тип::Примитивный(ПримитивныйТип::Булево) => "Булево".into(),
            Тип::Примитивный(ПримитивныйТип::Строка) => "Строка".into(),
            Тип::Переменная(v) => v.clone(),
            Тип::Функция { аргументы, результат } => format!("({}) -> {}", аргументы.iter().map(|a| self.type_to_string(a)).collect::<Vec<_>>().join(", "), self.type_to_string(результат)),
            Тип::Запись(поля) => format!("{{ {} }}", поля.iter().map(|(n, t)| format!("{}: {}", n, self.type_to_string(t))).collect::<Vec<_>>().join(", ")),
            Тип::Сумма(варианты) => format!("enum {{ {} }}", варианты.iter().map(|(n, t)| match t { Some(tt) => format!("{}({})", n, self.type_to_string(tt)), None => n.clone() }).collect::<Vec<_>>().join(" | ")),
            Тип::Пустой => "Пустой".into(),
            Тип::Кортеж(типы) => format!("({})", типы.iter().map(|t| self.type_to_string(t)).collect::<Vec<_>>().join(", ")),
            _ => format!("{:?}", typ),
        }
    }
}

pub fn substitute_type_vars(typ: &Тип, map: &HashMap<String, Тип>) -> Тип {
    match typ {
        Тип::Переменная(name) => map.get(name).cloned().unwrap_or(typ.clone()),
        Тип::Функция { аргументы, результат } => Тип::Функция { аргументы: аргументы.iter().map(|a| substitute_type_vars(a, map)).collect(), результат: Box::new(substitute_type_vars(результат, map)) },
        Тип::Запись(поля) => Тип::Запись(поля.iter().map(|(n, t)| (n.clone(), substitute_type_vars(t, map))).collect()),
        _ => typ.clone(),
    }
}

pub fn apply_substitution_to_ast(ast: &mut Ast, sub: &Substitution) {
    match ast {
        Ast::Модуль { объявления, .. } => { for d in объявления { apply_substitution_to_ast(d, sub); } }
        Ast::ОбъявлениеФункции { параметры, возвращаемый_тип, тело, .. } => { for p in параметры { p.тип = sub.apply(&p.тип); } if let Some(ref mut r) = возвращаемый_тип { *r = sub.apply(r); } apply_substitution_to_ast(тело, sub); }
        Ast::Блок { выражения, .. } => { for e in выражения { apply_substitution_to_ast(e, sub); } }
        Ast::Пусть { тип_аннотация, значение, .. } | Ast::Присваивание { тип_аннотация, значение, .. } => { if let Some(ref mut t) = тип_аннотация { *t = sub.apply(t); } apply_substitution_to_ast(значение, sub); }
        Ast::ПрисваиваниеСОперацией { значение, .. } => { apply_substitution_to_ast(значение, sub); }
        Ast::ПрисваиваниеОбразца { значение, .. } => { apply_substitution_to_ast(значение, sub); }
        Ast::ОбновлениеСтруктуры { объект, поля, тип, .. } => { apply_substitution_to_ast(объект, sub); for (_, v) in поля { apply_substitution_to_ast(v, sub); } if let Some(ref mut t) = тип { *t = sub.apply(t); } }
        Ast::Сопоставление { значение, ветки, тип, .. } => { apply_substitution_to_ast(значение, sub); for в in ветки { if let Some(ref mut g) = в.условие { apply_substitution_to_ast(g, sub); } apply_substitution_to_ast(&mut в.тело, sub); } if let Some(ref mut t) = тип { *t = sub.apply(t); } }
        Ast::ДвоичноеВыражение { левое, правое, тип, .. } => { if let Some(ref mut t) = тип { *t = sub.apply(t); } apply_substitution_to_ast(левое, sub); apply_substitution_to_ast(правое, sub); }
        Ast::Вызов { функция, аргументы, тип, .. } => { if let Some(ref mut t) = тип { *t = sub.apply(t); } apply_substitution_to_ast(функция, sub); for a in аргументы { apply_substitution_to_ast(a, sub); } }
        Ast::Если { условие, то, иначе, тип, .. } => { if let Some(ref mut t) = тип { *t = sub.apply(t); } apply_substitution_to_ast(условие, sub); apply_substitution_to_ast(то, sub); if let Some(ref mut e) = иначе { apply_substitution_to_ast(e, sub); } }
        Ast::Пока { условие, тело, .. } => { apply_substitution_to_ast(условие, sub); apply_substitution_to_ast(тело, sub); }
        Ast::ЦиклПока { условие, тело, .. } => { apply_substitution_to_ast(условие, sub); apply_substitution_to_ast(тело, sub); }
        Ast::Переменная { тип, .. } => { if let Some(ref mut t) = тип { *t = sub.apply(t); } }
        Ast::КонструкторСтруктуры { поля, тип, .. } => { if let Some(ref mut t) = тип { *t = sub.apply(t); } for (_, v) in поля { apply_substitution_to_ast(v, sub); } }
        Ast::КонструкторСуммы { тип, значение, .. } => { if let Some(ref mut t) = тип { *t = sub.apply(t); } if let Some(ref mut v) = значение { apply_substitution_to_ast(v, sub); } }
        Ast::ДоступКПолю { объект, тип, .. } => { if let Some(ref mut t) = тип { *t = sub.apply(t); } apply_substitution_to_ast(объект, sub); }
        _ => {}
    }
}
