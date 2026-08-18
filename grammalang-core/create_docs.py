# create_docs_simple.py
import os

os.makedirs('docs/v0.7', exist_ok=True)

# Простые текстовые файлы без markdown-блоков
readme = """Atlas v0.7 - Документация

Фаза 1: Детектор синтеза
Статус: Завершено

Документы:
1. synthesis_detector_spec.md
2. strategy_selector_spec.md
3. testing_guide.md
4. phase1_summary.md

Быстрый старт:
cargo test --test ontology_test -- --nocapture

Статус фаз:
[x] Фаза 0: Подготовка
[x] Фаза 1: Детектор синтеза
[ ] Фаза 2: Генерация
[ ] Фаза 3: Пересборка
"""

with open('docs/v0.7/README.md', 'w', encoding='utf-8') as f:
    f.write(readme)

summary = """Фаза 1: Детектор синтеза - Итоговый отчет

Статус: Завершено

Достижения:
- Полный детектор синтеза
- Три селектора стратегий
- Контекстный анализ

Метрики:
- Тестов: 21
- Покрытие: 100%

Философия:
- Гегельянство: прямые противоречия
- Неоплатонизм: рекурсивные
- Прагматизм: опосредованные
- Марксизм: адаптивный выбор
"""

with open('docs/v0.7/phase1_summary.md', 'w', encoding='utf-8') as f:
    f.write(summary)

detector = """Спецификация детектора синтеза

Основные компоненты:
- SynthesisDetector
- SynthesisCandidate
- MetricsSnapshot

Алгоритм:
1. Проверка готовности
2. Определение типа
3. Выбор стратегии
4. Расчет метрик
5. Сортировка

Типы и стратегии:
- Direct -> Hegelian
- Mediated -> Pragmatic
- Recursive -> Plotinian
"""

with open('docs/v0.7/synthesis_detector_spec.md', 'w', encoding='utf-8') as f:
    f.write(detector)

selector = """Спецификация селекторов стратегий

1. HeuristicSelector - быстрый выбор
2. ContextualSelector - контекстный выбор
3. AdaptiveSelector - обучаемый выбор

Рекомендации:
- Начало: HeuristicSelector
- Настройка: ContextualSelector
- Продакшн: AdaptiveSelector
"""

with open('docs/v0.7/strategy_selector_spec.md', 'w', encoding='utf-8') as f:
    f.write(selector)

testing = """Руководство по тестированию

Запуск:
cargo test --test ontology_test -- --nocapture

Структура:
- target_ontology_tests.rs (3 теста)
- contradiction_tests.rs (6 тестов)
- synthesis_detector_tests.rs (5 тестов)
- phase1_tests.rs (5 тестов)
- integration_test.rs (2 теста)

Результат: 21 тест пройден
"""

with open('docs/v0.7/testing_guide.md', 'w', encoding='utf-8') as f:
    f.write(testing)

print("Все документы созданы!")
