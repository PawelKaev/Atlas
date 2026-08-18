# Atlas v0.8 - Дерево файлов

## Новые файлы (v0.8)
src/social/
├── mod.rs - объявление модулей
├── knowledge_base.rs - базы знаний
├── kb_connectors.rs - коннекторы (Wikidata, DBPedia, JSON-LD)
├── collective_trace.rs - коллективный trace
├── social_reactor.rs - социальный реактор
├── federation.rs - федерация
└── integration.rs - интеграция с v0.7

tests/social/
├── mod.rs - тестовый модуль
├── phase0_tests.rs - тесты инфраструктуры (4)
├── phase1_tests.rs - тесты коннекторов (4)
├── phase2_tests.rs - тесты trace (7)
├── phase3_tests.rs - тесты реактора (6)
├── phase4_tests.rs - тесты федерации (7)
├── phase5_tests.rs - тесты интеграции (6)
└── phase6_tests.rs - комплексные тесты (7)

docs/v0.8/
├── README.md
└── final_summary.md

## Измененные файлы

- src/lib.rs - добавлен pub mod social
