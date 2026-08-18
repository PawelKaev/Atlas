# Atlas v0.7 - Дерево файлов

## Статус: ЗАГРУЖЕНО НА GITHUB (коммит cdc11d4)

## Легенда:
- [NEW] - новый файл
- [MOD] - измененный файл
- [DEL] - удаленный файл
- [UNCHANGED] - без изменений

---

C:\Projects\Atlas\
│   .gitignore                              [UNCHANGED]
│   main.py                                 [UNCHANGED]
│   tree.txt                                [NEW]
│   create_phase2.py                        [NEW]
│   create_phase3.py                        [NEW]
│   create_phase4.py                        [NEW]
│   create_phase5.py                        [NEW]
│   create_phase6.py                        [NEW]
│   create_phase7.py                        [NEW]
│   fix_integration.py                      [NEW]
│   fix_test.py                             [NEW]
│   create_tree_v07.py                      [NEW]
│
├───docs/
│   └───v0.7/
│       ├───README.md                       [NEW]
│       ├───final_documentation.md          [NEW]
│       ├───phase1_summary.md               [NEW]
│       ├───strategy_selector_spec.md       [NEW]
│       ├───synthesis_detector_spec.md      [NEW]
│       ├───target_ontology_spec.md         [NEW]
│       └───testing_guide.md                [NEW]
│
├───grammalang-core/
│   ├───Cargo.toml                          [UNCHANGED]
│   ├───Cargo.lock                          [MOD]
│   ├───create_docs.py                      [NEW]
│   │
│   ├───src/
│   │   ├───lib.rs                          [MOD] (pub mod ontology, pub mod modes)
│   │   ├───ontology.rs                     [DEL] (перемещен)
│   │   │
│   │   ├───modes/                          [NEW]
│   │   │   ├───mod.rs                      [NEW]
│   │   │   ├───plato_mode.rs               [NEW]
│   │   │   └───architect_mode.rs           [NEW]
│   │   │
│   │   └───ontology/                       [NEW]
│   │       ├───mod.rs                      [NEW]
│   │       ├───engine.rs                   [NEW] (из ontology.rs)
│   │       ├───target_ontology.rs          [NEW]
│   │       ├───contradiction.rs            [NEW]
│   │       ├───synthesis_detector.rs       [NEW]
│   │       ├───synthesis_strategy_selector.rs  [NEW]
│   │       ├───synthesis_generator.rs      [NEW]
│   │       ├───synthesis_generator_llm.rs  [NEW]
│   │       ├───synthesis_generator_diffusion.rs [NEW]
│   │       ├───synthesis_generator_evolutionary.rs [NEW]
│   │       ├───synthesis_integrator.rs     [NEW]
│   │       ├───axis_proposer.rs            [NEW]
│   │       ├───synthesis_validator.rs      [NEW]
│   │       ├───synthesis_rollback.rs       [NEW]
│   │       └───integration_layer.rs        [NEW]
│   │
│   └───tests/
│       ├───ontology_test.rs                [NEW]
│       ├───simple_ontology_test.rs         [NEW]
│       │
│       └───ontology/                       [NEW]
│           ├───mod.rs                      [NEW]
│           ├───target_ontology_tests.rs    [NEW]
│           ├───contradiction_tests.rs      [NEW]
│           ├───synthesis_detector_tests.rs [NEW]
│           ├───phase1_tests.rs             [NEW]
│           ├───phase2_tests.rs             [NEW]
│           ├───phase3_tests.rs             [NEW]
│           ├───phase4_tests.rs             [NEW]
│           ├───phase5_tests.rs             [NEW]
│           ├───phase6_tests.rs             [NEW]
│           ├───phase7_tests.rs             [NEW]
│           └───integration_test.rs         [NEW]
│
└───src/
    └───ontology/
        └───synthesis_generator.rs          [NEW]

---

## Итого:
- Новых файлов: 50+
- Измененных: 2 (lib.rs, Cargo.lock)
- Удаленных: 1 (ontology.rs)
- Тестов: 51