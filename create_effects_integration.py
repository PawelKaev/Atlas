# create_effects_integration.py
import os

GRAMMALANG = r'C:\Projects\grammalang-stable'
os.makedirs(os.path.join(GRAMMALANG, 'src/grammalang/atlas_bridge'), exist_ok=True)

# 1. Интеграция эффектов
effects_integration = '''"""
Интеграция эффектов и состояний ATLAS ↔ GrammaLang

IO-обработка, чистота функций, оптимизация для LLVM.
"""

from typing import Dict, List, Optional, Set
from enum import Enum


class EffectKind(Enum):
    """Виды эффектов"""
    IO = "io"
    STATE = "state"
    EXCEPTION = "exception"
    NON_DETERMINISM = "non_determinism"
    PURE = "pure"


class EffectInfo:
    """Информация об эффекте"""
    
    def __init__(self, name: str, kind: EffectKind):
        self.name = name
        self.kind = kind
        self.marked_functions: List[str] = []
        
    def mark_function(self, function_name: str):
        """Пометить функцию как имеющую этот эффект"""
        if function_name not in self.marked_functions:
            self.marked_functions.append(function_name)
            
    def to_dict(self) -> Dict[str, str]:
        return {
            "name": self.name,
            "kind": self.kind.value,
            "functions": self.marked_functions,
        }


class EffectRegistry:
    """Реестр эффектов"""
    
    def __init__(self):
        self.effects: Dict[str, EffectInfo] = {}
        self.pure_functions: Set[str] = set()
        
        # Регистрируем стандартные эффекты
        self._register_standard_effects()
        
    def _register_standard_effects(self):
        """Регистрация стандартных эффектов"""
        self.register_effect("IO", EffectKind.IO)
        self.register_effect("Состояние", EffectKind.STATE)
        self.register_effect("Исключение", EffectKind.EXCEPTION)
        self.register_effect("Неопределённость", EffectKind.NON_DETERMINISM)
        
    def register_effect(self, name: str, kind: EffectKind) -> EffectInfo:
        """Регистрация эффекта"""
        info = EffectInfo(name, kind)
        self.effects[name] = info
        return info
        
    def mark_function_with_effect(self, function_name: str, effect_name: str):
        """Пометить функцию эффектом"""
        if effect_name in self.effects:
            self.effects[effect_name].mark_function(function_name)
            
    def mark_pure(self, function_name: str):
        """Пометить функцию как чистую"""
        self.pure_functions.add(function_name)
        
    def is_pure(self, function_name: str) -> bool:
        """Проверка чистоты функции"""
        if function_name in self.pure_functions:
            return True
            
        for effect in self.effects.values():
            if function_name in effect.marked_functions:
                return False
                
        return True  # По умолчанию чистая
        
    def get_function_effects(self, function_name: str) -> List[str]:
        """Получение всех эффектов функции"""
        result = []
        for effect_name, effect in self.effects.items():
            if function_name in effect.marked_functions:
                result.append(effect_name)
        return result
        
    def summary(self) -> str:
        """Сводка реестра эффектов"""
        total_marked = sum(len(e.marked_functions) for e in self.effects.values())
        return f"Эффекты: {len(self.effects)} видов, {total_marked} помеченных функций, {len(self.pure_functions)} чистых"


class PurityChecker:
    """Проверка чистоты функций"""
    
    def __init__(self, registry: EffectRegistry):
        self.registry = registry
        
    def check_purity(self, function_name: str) -> Dict[str, str]:
        """Проверка чистоты функции"""
        if self.registry.is_pure(function_name):
            return {
                "function": function_name,
                "status": "pure",
                "effects": [],
            }
        else:
            effects = self.registry.get_function_effects(function_name)
            return {
                "function": function_name,
                "status": "impure",
                "effects": effects,
            }
            
    def can_optimize(self, function_name: str) -> bool:
        """Можно ли оптимизировать функцию"""
        return self.registry.is_pure(function_name)
        
    def optimization_hint(self, function_name: str) -> str:
        """Подсказка для LLVM-оптимизатора"""
        if self.can_optimize(function_name):
            return f"Функция {function_name} чистая — можно применять агрессивные оптимизации (CSE, DCE, инлайнинг)"
        else:
            effects = self.registry.get_function_effects(function_name)
            return f"Функция {function_name} имеет эффекты {effects} — оптимизации ограничены"


class IoHandler:
    """Обработка IO-эффектов"""
    
    def __init__(self, registry: EffectRegistry):
        self.registry = registry
        self.io_functions: List[str] = []
        
    def mark_io(self, function_name: str):
        """Пометить функцию как IO"""
        self.registry.mark_function_with_effect(function_name, "IO")
        self.io_functions.append(function_name)
        
    def get_io_functions(self) -> List[str]:
        """Все IO-функции"""
        return self.io_functions
        
    def is_io(self, function_name: str) -> bool:
        """Проверка IO"""
        return "IO" in self.registry.get_function_effects(function_name)


class AtlasEffectsIntegrator:
    """Полная интеграция эффектов"""
    
    def __init__(self):
        self.registry = EffectRegistry()
        self.purity_checker = PurityChecker(self.registry)
        self.io_handler = IoHandler(self.registry)
        
    def register_grammalang_io(self):
        """Регистрация IO-функций GrammaLang"""
        io_functions = [
            "написать",
            "консоль.написать",
            "прочитать",
            "файл.прочитать",
            "файл.записать",
        ]
        
        for func in io_functions:
            self.io_handler.mark_io(func)
            
        print(f"Зарегистрировано IO-функций: {len(io_functions)}")
        
    def register_pure_functions(self, functions: List[str]):
        """Регистрация чистых функций"""
        for func in functions:
            self.registry.mark_pure(func)
            
        print(f"Зарегистрировано чистых функций: {len(functions)}")
        
    def full_summary(self) -> str:
        """Полная сводка"""
        return f"""
Эффекты ATLAS ↔ GrammaLang:
  {self.registry.summary()}
  IO-функции: {self.io_handler.get_io_functions()}
"""


# Тесты
if __name__ == "__main__":
    integrator = AtlasEffectsIntegrator()
    
    # Регистрируем IO
    integrator.register_grammalang_io()
    
    # Регистрируем чистые функции
    integrator.register_pure_functions([
        "сложить",
        "вычесть",
        "умножить",
        "длина",
    ])
    
    # Проверка чистоты
    print(f"сложить чистая: {integrator.purity_checker.check_purity('сложить')}")
    print(f"написать чистая: {integrator.purity_checker.check_purity('написать')}")
    
    # Оптимизация
    print(integrator.purity_checker.optimization_hint("сложить"))
    print(integrator.purity_checker.optimization_hint("написать"))
    
    print(integrator.full_summary())
'''

with open(os.path.join(GRAMMALANG, 'src/grammalang/atlas_bridge/effects_integration.py'), 'w', encoding='utf-8') as f:
    f.write(effects_integration)
print("effects_integration.py created")

# 2. Обновляем __init__.py
init_content = '''"""
Мост между ATLAS (диалектическое мышление) и GrammaLang (практический язык).
"""

from .type_integration import (
    TypeKind,
    TypeInfo,
    TypeTable,
    TypeCanonicalizer,
    TypeCompatibilityChecker,
    AtlasTypeIntegrator,
)

from .effects_integration import (
    EffectKind,
    EffectInfo,
    EffectRegistry,
    PurityChecker,
    IoHandler,
    AtlasEffectsIntegrator,
)

__all__ = [
    'TypeKind',
    'TypeInfo',
    'TypeTable',
    'TypeCanonicalizer',
    'TypeCompatibilityChecker',
    'AtlasTypeIntegrator',
    'EffectKind',
    'EffectInfo',
    'EffectRegistry',
    'PurityChecker',
    'IoHandler',
    'AtlasEffectsIntegrator',
]
'''

with open(os.path.join(GRAMMALANG, 'src/grammalang/atlas_bridge/__init__.py'), 'w', encoding='utf-8') as f:
    f.write(init_content)
print("__init__.py updated")

print("\nEffects integration files created!")
