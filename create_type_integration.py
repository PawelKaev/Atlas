# create_type_integration.py
import os

GRAMMALANG = r'C:\Projects\grammalang-stable'
os.makedirs(os.path.join(GRAMMALANG, 'src/grammalang/atlas_bridge'), exist_ok=True)

# 1. Интеграция системы типов
type_integration = '''"""
Интеграция системы типов ATLAS ↔ GrammaLang

Канонизация типов, таблица типов, совместимость.
"""

from typing import Dict, List, Optional, Any
from enum import Enum


class TypeKind(Enum):
    """Виды типов"""
    PRIMITIVE = "primitive"
    STRUCT = "struct"
    ENUM = "enum"
    FUNCTION = "function"
    MONADIC = "monadic"
    ALGEBRAIC = "algebraic"


class TypeInfo:
    """Информация о типе"""
    
    def __init__(self, name: str, kind: TypeKind):
        self.name = name
        self.kind = kind
        self.fields: List[Dict[str, str]] = []
        self.variants: List[Dict[str, str]] = []
        
    def add_field(self, name: str, field_type: str):
        self.fields.append({"name": name, "type": field_type})
        
    def add_variant(self, name: str, payload: Optional[str] = None):
        self.variants.append({"name": name, "payload": payload})
        
    def to_dict(self) -> Dict[str, Any]:
        return {
            "name": self.name,
            "kind": self.kind.value,
            "fields": self.fields,
            "variants": self.variants,
        }


class TypeTable:
    """Централизованная таблица типов"""
    
    def __init__(self):
        self.types: Dict[str, TypeInfo] = {}
        self.aliases: Dict[str, str] = {}
        
        # Регистрируем базовые типы
        self._register_primitives()
        
    def _register_primitives(self):
        """Регистрация примитивных типов"""
        primitives = [
            ("Целое", "i64"),
            ("Целое32", "i32"),
            ("Десятичное", "double"),
            ("Строка", "i8*"),
            ("Булево", "i1"),
            ("Ничего", "void"),
            ("Значение", "i64"),
            ("Провал", "i64"),
            ("Успех", "i64"),
        ]
        
        for name, llvm_type in primitives:
            self.types[name] = TypeInfo(name, TypeKind.PRIMITIVE)
            self.types[name].llvm_type = llvm_type
            
    def register_type(self, name: str, kind: TypeKind) -> TypeInfo:
        """Регистрация нового типа"""
        info = TypeInfo(name, kind)
        self.types[name] = info
        return info
        
    def register_alias(self, alias: str, target: str):
        """Регистрация псевдонима типа"""
        self.aliases[alias] = target
        
    def get_type(self, name: str) -> Optional[TypeInfo]:
        """Получение типа по имени"""
        # Проверяем псевдонимы
        if name in self.aliases:
            name = self.aliases[name]
        return self.types.get(name)
        
    def resolve_type(self, name: str) -> str:
        """Разрешение типа с учетом псевдонимов"""
        if name in self.aliases:
            return self.aliases[name]
        return name
        
    def get_llvm_type(self, name: str) -> str:
        """Получение LLVM-типа"""
        resolved = self.resolve_type(name)
        if resolved in self.types:
            info = self.types[resolved]
            if hasattr(info, 'llvm_type'):
                return info.llvm_type
        return "i64"  # По умолчанию
        
    def all_types(self) -> List[TypeInfo]:
        """Все типы"""
        return list(self.types.values())
        
    def summary(self) -> str:
        """Сводка таблицы типов"""
        return f"Таблица типов: {len(self.types)} типов, {len(self.aliases)} псевдонимов"


class TypeCanonicalizer:
    """Канонизация типов ATLAS ↔ GrammaLang"""
    
    def __init__(self, type_table: TypeTable):
        self.type_table = type_table
        self.canonical_map: Dict[str, str] = {}
        
    def canonicalize(self, type_name: str) -> str:
        """Приведение типа к канонической форме"""
        # Убираем пробелы
        canonical = type_name.strip()
        
        # Разрешаем псевдонимы
        canonical = self.type_table.resolve_type(canonical)
        
        # Сохраняем в карту
        self.canonical_map[type_name] = canonical
        
        return canonical
        
    def canonicalize_atlas_type(self, atlas_type: str) -> str:
        """Преобразование типа ATLAS в тип GrammaLang"""
        atlas_to_grammalang = {
            "NodeId": "Строка",
            "AxisId": "Строка",
            "SynthesisStrategy": "Строка",
            "ContradictionKind": "Строка",
            "f32": "Десятичное",
            "f64": "Десятичное",
            "usize": "Целое",
            "String": "Строка",
            "bool": "Булево",
        }
        return atlas_to_grammalang.get(atlas_type, atlas_type)
        
    def canonicalize_grammalang_type(self, grammalang_type: str) -> str:
        """Преобразование типа GrammaLang в тип ATLAS"""
        grammalang_to_atlas = {
            "Целое": "i64",
            "Десятичное": "f64",
            "Строка": "String",
            "Булево": "bool",
            "Ничего": "()",
        }
        return grammalang_to_atlas.get(grammalang_type, grammalang_type)


class TypeCompatibilityChecker:
    """Проверка совместимости типов"""
    
    def __init__(self, type_table: TypeTable):
        self.type_table = type_table
        
    def check_compatibility(self, type_a: str, type_b: str) -> bool:
        """Проверка совместимости двух типов"""
        resolved_a = self.type_table.resolve_type(type_a)
        resolved_b = self.type_table.resolve_type(type_b)
        
        # Одинаковые типы совместимы
        if resolved_a == resolved_b:
            return True
            
        # Числовые типы совместимы
        numeric = {"Целое", "Целое32", "Десятичное"}
        if resolved_a in numeric and resolved_b in numeric:
            return True
            
        return False
        
    def check_assignment(self, target: str, value: str) -> bool:
        """Проверка присваивания"""
        return self.check_compatibility(target, value)


class AtlasTypeIntegrator:
    """Полная интеграция системы типов"""
    
    def __init__(self):
        self.type_table = TypeTable()
        self.canonicalizer = TypeCanonicalizer(self.type_table)
        self.compatibility = TypeCompatibilityChecker(self.type_table)
        
    def register_atlas_types(self):
        """Регистрация типов ATLAS"""
        self.type_table.register_type("Узел", TypeKind.STRUCT)
        self.type_table.register_type("Противоречие", TypeKind.STRUCT)
        self.type_table.register_type("Синтез", TypeKind.STRUCT)
        self.type_table.register_type("Генеалогия", TypeKind.ALGEBRAIC)
        self.type_table.register_type("Стратегия", TypeKind.ENUM)
        
        print("Типы ATLAS зарегистрированы")
        
    def register_grammalang_types(self):
        """Регистрация типов GrammaLang"""
        self.type_table.register_type("Список", TypeKind.ALGEBRAIC)
        self.type_table.register_type("Maybe", TypeKind.MONADIC)
        self.type_table.register_type("Результат", TypeKind.MONADIC)
        
        print("Типы GrammaLang зарегистрированы")
        
    def full_summary(self) -> str:
        """Полная сводка"""
        return f"""
Система типов ATLAS ↔ GrammaLang:
  {self.type_table.summary()}
  Типы: {', '.join(t.name for t in self.type_table.all_types()[:10])}...
"""


# Тесты
if __name__ == "__main__":
    integrator = AtlasTypeIntegrator()
    integrator.register_atlas_types()
    integrator.register_grammalang_types()
    
    # Канонизация
    print(f"Канонизация 'NodeId' → {integrator.canonicalizer.canonicalize_atlas_type('NodeId')}")
    print(f"Канонизация 'Целое' → {integrator.canonicalizer.canonicalize_grammalang_type('Целое')}")
    
    # Совместимость
    print(f"Целое и Десятичное совместимы: {integrator.compatibility.check_compatibility('Целое', 'Десятичное')}")
    print(f"Целое и Строка совместимы: {integrator.compatibility.check_compatibility('Целое', 'Строка')}")
    
    print(integrator.full_summary())
'''

with open(os.path.join(GRAMMALANG, 'src/grammalang/atlas_bridge/type_integration.py'), 'w', encoding='utf-8') as f:
    f.write(type_integration)
print("type_integration.py created")

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

__all__ = [
    'TypeKind',
    'TypeInfo',
    'TypeTable',
    'TypeCanonicalizer',
    'TypeCompatibilityChecker',
    'AtlasTypeIntegrator',
]
'''

with open(os.path.join(GRAMMALANG, 'src/grammalang/atlas_bridge/__init__.py'), 'w', encoding='utf-8') as f:
    f.write(init_content)
print("__init__.py updated")

# 3. Пример использования
example = '''"""
Пример интеграции системы типов
"""

from grammalang.atlas_bridge import AtlasTypeIntegrator


def demo():
    integrator = AtlasTypeIntegrator()
    
    # Регистрация типов
    integrator.register_atlas_types()
    integrator.register_grammalang_types()
    
    # Канонизация типов
    atlas_type = integrator.canonicalizer.canonicalize_atlas_type("NodeId")
    grammalang_type = integrator.canonicalizer.canonicalize_grammalang_type("Целое")
    
    print(f"ATLAS NodeId → GrammaLang {atlas_type}")
    print(f"GrammaLang Целое → ATLAS {grammalang_type}")
    
    # Проверка совместимости
    print(f"Целое ~ Десятичное: {integrator.compatibility.check_compatibility('Целое', 'Десятичное')}")
    print(f"Целое ~ Строка: {integrator.compatibility.check_compatibility('Целое', 'Строка')}")
    
    # LLVM типы
    print(f"LLVM тип Целое: {integrator.type_table.get_llvm_type('Целое')}")
    print(f"LLVM тип Десятичное: {integrator.type_table.get_llvm_type('Десятичное')}")
    
    print(integrator.full_summary())


if __name__ == "__main__":
    demo()
'''

with open(os.path.join(GRAMMALANG, 'examples_type_integration.py'), 'w', encoding='utf-8') as f:
    f.write(example)
print("example created")

print("\nType integration files created!")
