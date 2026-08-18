# create_incremental.py
import os

GRAMMALANG = r'C:\Projects\grammalang-stable'
os.makedirs(os.path.join(GRAMMALANG, 'src/grammalang/atlas_bridge'), exist_ok=True)

# 1. Инкрементальная компиляция
incremental = '''"""
Инкрементальная компиляция ATLAS ↔ GrammaLang

Точечное обновление графа, оперативная память, динамическая перекомпиляция.
"""

import os
import time
import hashlib
from typing import Dict, List, Optional, Set, Tuple
from dataclasses import dataclass, field


@dataclass
class CompilationUnit:
    """Единица компиляции"""
    name: str
    source_hash: str
    dependencies: Set[str] = field(default_factory=set)
    dependents: Set[str] = field(default_factory=set)
    compiled: bool = False
    last_compiled: float = 0.0
    llvm_ir: Optional[str] = None
    object_code: Optional[bytes] = None


class IncrementalCompiler:
    """Инкрементальный компилятор"""
    
    def __init__(self):
        self.units: Dict[str, CompilationUnit] = {}
        self.cache: Dict[str, str] = {}
        self.compilation_count = 0
        self.recompilation_count = 0
        
    def add_unit(self, name: str, source: str) -> CompilationUnit:
        """Добавление единицы компиляции"""
        source_hash = hashlib.sha256(source.encode()).hexdigest()
        
        if name in self.units:
            # Проверяем изменения
            old_hash = self.units[name].source_hash
            if old_hash != source_hash:
                # Источник изменился — нужна перекомпиляция
                self.units[name].source_hash = source_hash
                self.units[name].compiled = False
                print(f"Изменен: {name} → перекомпиляция")
        else:
            # Новая единица
            self.units[name] = CompilationUnit(
                name=name,
                source_hash=source_hash,
            )
            print(f"Добавлен: {name}")
            
        return self.units[name]
        
    def add_dependency(self, unit: str, depends_on: str):
        """Добавление зависимости"""
        if unit in self.units and depends_on in self.units:
            self.units[unit].dependencies.add(depends_on)
            self.units[depends_on].dependents.add(unit)
            
    def compile_all(self):
        """Полная компиляция"""
        for name in self.units:
            self._compile_unit(name)
            
    def compile_incremental(self, changed_units: List[str]):
        """Инкрементальная компиляция — только измененные"""
        # Находим все затронутые единицы
        affected = self._find_affected(changed_units)
        
        for name in affected:
            self._compile_unit(name)
            
        print(f"Инкрементальная: {len(changed_units)} изменено → {len(affected)} затронуто")
        
    def _find_affected(self, changed: List[str]) -> Set[str]:
        """Поиск всех затронутых единиц (транзитивно)"""
        affected = set(changed)
        queue = list(changed)
        
        while queue:
            unit = queue.pop(0)
            if unit in self.units:
                for dependent in self.units[unit].dependents:
                    if dependent not in affected:
                        affected.add(dependent)
                        queue.append(dependent)
                        
        return affected
        
    def _compile_unit(self, name: str):
        """Компиляция одной единицы"""
        if name not in self.units:
            return
            
        unit = self.units[name]
        
        # Проверяем, нужно ли компилировать
        if unit.compiled:
            return
            
        # Проверяем зависимости
        for dep in unit.dependencies:
            if not self.units[dep].compiled:
                self._compile_unit(dep)
                
        # Имитация компиляции
        time.sleep(0.001)  # Задержка для реалистичности
        
        # Генерируем LLVM IR
        unit.llvm_ir = self._generate_llvm_ir(unit)
        unit.object_code = hashlib.sha256(unit.llvm_ir.encode()).digest()
        unit.compiled = True
        unit.last_compiled = time.time()
        
        self.compilation_count += 1
        
    def _generate_llvm_ir(self, unit: CompilationUnit) -> str:
        """Генерация LLVM IR (упрощенно)"""
        return f"""
; Module: {unit.name}
define i64 @{unit.name}() {{
entry:
    ret i64 0
}}
"""
        
    def get_status(self, name: str) -> str:
        """Статус единицы"""
        if name not in self.units:
            return "Не найдена"
            
        unit = self.units[name]
        status = "Скомпилирована" if unit.compiled else "Требует компиляции"
        return f"{name}: {status} (hash: {unit.source_hash[:8]}...)"
        
    def summary(self) -> str:
        """Сводка компилятора"""
        compiled = sum(1 for u in self.units.values() if u.compiled)
        total = len(self.units)
        
        return f"""
Инкрементальный компилятор:
  Единиц: {total}
  Скомпилировано: {compiled}
  Требует: {total - compiled}
  Компиляций: {self.compilation_count}
  Перекомпиляций: {self.recompilation_count}
"""


class RamCache:
    """Оперативная память / кэш"""
    
    def __init__(self, max_size_mb: int = 100):
        self.max_size_mb = max_size_mb
        self.cache: Dict[str, Tuple[bytes, float]] = {}
        self.hits = 0
        self.misses = 0
        
    def get(self, key: str) -> Optional[bytes]:
        """Получение из кэша"""
        if key in self.cache:
            data, timestamp = self.cache[key]
            self.hits += 1
            return data
        self.misses += 1
        return None
        
    def put(self, key: str, data: bytes):
        """Сохранение в кэш"""
        self.cache[key] = (data, time.time())
        
        # Очистка при превышении размера
        self._evict_if_needed()
        
    def _evict_if_needed(self):
        """Вытеснение старых записей"""
        if len(self.cache) > 1000:  # Простая эвристика
            # Удаляем самые старые
            sorted_items = sorted(self.cache.items(), key=lambda x: x[1][1])
            for key, _ in sorted_items[:100]:
                del self.cache[key]
                
    def hit_rate(self) -> float:
        """Процент попаданий"""
        total = self.hits + self.misses
        if total == 0:
            return 0.0
        return self.hits / total
        
    def summary(self) -> str:
        """Сводка кэша"""
        return f"Кэш: {len(self.cache)} записей, hit rate: {self.hit_rate():.1%}"


class AtlasIncrementalIntegrator:
    """Полная интеграция инкрементальной компиляции"""
    
    def __init__(self):
        self.compiler = IncrementalCompiler()
        self.ram_cache = RamCache()
        
    def add_source(self, name: str, source: str):
        """Добавление исходного кода"""
        unit = self.compiler.add_unit(name, source)
        
        # Кэшируем исходник
        self.ram_cache.put(f"src_{name}", source.encode())
        
        return unit
        
    def compile_project(self, sources: Dict[str, str]):
        """Компиляция проекта"""
        # Добавляем все источники
        for name, source in sources.items():
            self.add_source(name, source)
            
        # Полная компиляция
        self.compiler.compile_all()
        
        # Кэшируем LLVM IR
        for name, unit in self.compiler.units.items():
            if unit.llvm_ir:
                self.ram_cache.put(f"llvm_{name}", unit.llvm_ir.encode())
                
    def update_source(self, name: str, new_source: str) -> List[str]:
        """Обновление исходного кода — инкрементальная перекомпиляция"""
        # Проверяем кэш
        cached = self.ram_cache.get(f"src_{name}")
        if cached and cached == new_source.encode():
            print(f"{name}: без изменений (из кэша)")
            return []
            
        # Обновляем
        self.compiler.add_unit(name, new_source)
        
        # Инкрементальная компиляция
        self.compiler.compile_incremental([name])
        
        return list(self.compiler._find_affected([name]))
        
    def full_summary(self) -> str:
        """Полная сводка"""
        return f"""
Инкрементальная компиляция ATLAS ↔ GrammaLang:
  {self.compiler.summary()}
  {self.ram_cache.summary()}
"""


# Тесты
if __name__ == "__main__":
    integrator = AtlasIncrementalIntegrator()
    
    # Начальный проект
    sources = {
        "модуль_1": "функция привет() { вернуть \\"Привет\\"; }",
        "модуль_2": "функция сложить(a, b) { вернуть a + b; }",
        "модуль_3": "функция использовать() { вернуть сложить(1, 2); }",
    }
    
    print("=== Начальная компиляция ===")
    integrator.compile_project(sources)
    print(integrator.full_summary())
    
    print("=== Изменение модуль_1 ===")
    affected = integrator.update_source("модуль_1", "функция привет(имя) { вернуть \\"Привет, \\" + имя; }")
    print(f"Затронуты: {affected}")
    print(integrator.full_summary())
    
    print("=== Без изменений ===")
    affected = integrator.update_source("модуль_1", "функция привет(имя) { вернуть \\"Привет, \\" + имя; }")
    print(f"Затронуты: {affected}")
'''

with open(os.path.join(GRAMMALANG, 'src/grammalang/atlas_bridge/incremental_integration.py'), 'w', encoding='utf-8') as f:
    f.write(incremental)
print("incremental_integration.py created")

# 2. Обновляем __init__.py
init_content = '''"""
Мост между ATLAS (диалектическое мышление) и GrammaLang (практический язык).
"""

from .type_integration import (
    TypeKind, TypeInfo, TypeTable,
    TypeCanonicalizer, TypeCompatibilityChecker, AtlasTypeIntegrator,
)

from .effects_integration import (
    EffectKind, EffectInfo, EffectRegistry,
    PurityChecker, IoHandler, AtlasEffectsIntegrator,
)

from .llm_integration import (
    LlmConfig, LlmInterface, AtlasLlmIntegrator,
)

from .incremental_integration import (
    CompilationUnit, IncrementalCompiler, RamCache, AtlasIncrementalIntegrator,
)

__all__ = [
    'TypeKind', 'TypeInfo', 'TypeTable',
    'TypeCanonicalizer', 'TypeCompatibilityChecker', 'AtlasTypeIntegrator',
    'EffectKind', 'EffectInfo', 'EffectRegistry',
    'PurityChecker', 'IoHandler', 'AtlasEffectsIntegrator',
    'LlmConfig', 'LlmInterface', 'AtlasLlmIntegrator',
    'CompilationUnit', 'IncrementalCompiler', 'RamCache', 'AtlasIncrementalIntegrator',
]
'''

with open(os.path.join(GRAMMALANG, 'src/grammalang/atlas_bridge/__init__.py'), 'w', encoding='utf-8') as f:
    f.write(init_content)
print("__init__.py updated")

print("\nIncremental compilation files created!")
