# create_atlas_grammalang_integration.py
import os

ATLAS = r'C:\Projects\Atlas'
GRAMMALANG = r'C:\Projects\grammalang-stable'

# Создаем мост в GrammaLang
os.makedirs(os.path.join(GRAMMALANG, 'src/grammalang/atlas_bridge'), exist_ok=True)

# 1. Мост Atlas ↔ GrammaLang
bridge = '''"""
Мост между ATLAS (диалектическое мышление) и GrammaLang (практический язык).

Позволяет:
- Использовать ATLAS для синтеза понятий
- Использовать GrammaLang для компиляции
- Объединять диалектику и практичность
"""

from typing import Optional, List, Dict, Any
import json


class AtlasBridge:
    """Мост между ATLAS и GrammaLang"""
    
    def __init__(self):
        self.atlas_concepts: Dict[str, Any] = {}
        self.grammalang_functions: Dict[str, Any] = {}
        self.syntheses: List[Dict[str, Any]] = []
        self.contradictions: List[Dict[str, Any]] = []
        
    def load_atlas_concepts(self, concepts: Dict[str, Any]):
        """Загрузка понятий из ATLAS"""
        self.atlas_concepts.update(concepts)
        print(f"Загружено понятий ATLAS: {len(concepts)}")
        
    def load_grammalang_functions(self, functions: Dict[str, Any]):
        """Загрузка функций из GrammaLang"""
        self.grammalang_functions.update(functions)
        print(f"Загружено функций GrammaLang: {len(functions)}")
        
    def synthesize(self, concept_a: str, concept_b: str, strategy: str = "hegelian") -> str:
        """Синтез двух понятий через ATLAS"""
        synthesis_name = f"{concept_a}_{concept_b}_synthesis"
        
        self.syntheses.append({
            "concept_a": concept_a,
            "concept_b": concept_b,
            "strategy": strategy,
            "result": synthesis_name,
        })
        
        # Регистрируем синтез как понятие
        self.atlas_concepts[synthesis_name] = {
            "parents": [concept_a, concept_b],
            "strategy": strategy,
            "genealogy": [concept_a, concept_b],
        }
        
        print(f"Синтез: {concept_a} + {concept_b} → {synthesis_name}")
        return synthesis_name
        
    def detect_contradiction(self, a: str, b: str, severity: float = 0.7) -> Dict[str, Any]:
        """Обнаружение противоречия"""
        contradiction = {
            "node_a": a,
            "node_b": b,
            "severity": severity,
            "ready_for_synthesis": severity >= 0.6,
        }
        
        self.contradictions.append(contradiction)
        
        if contradiction["ready_for_synthesis"]:
            print(f"Противоречие {a} ~::~ {b} готово к синтезу")
        else:
            print(f"Противоречие {a} ~::~ {b} (недостаточно зрелое)")
            
        return contradiction
        
    def export_to_grammalang(self) -> str:
        """Экспорт синтезов в код GrammaLang"""
        code_lines = []
        
        for synthesis in self.syntheses:
            code_lines.append(f"// Синтез: {synthesis['concept_a']} + {synthesis['concept_b']}")
            code_lines.append(f"структура {synthesis['result']} {{")
            code_lines.append(f"    {synthesis['concept_a'].lower()}: {synthesis['concept_a']},")
            code_lines.append(f"    {synthesis['concept_b'].lower()}: {synthesis['concept_b']},")
            code_lines.append("}")
            code_lines.append("")
            
        return "\\n".join(code_lines)
        
    def export_to_atlas(self) -> str:
        """Экспорт функций GrammaLang в формат ATLAS"""
        result = {
            "concepts": self.atlas_concepts,
            "functions": list(self.grammalang_functions.keys()),
            "syntheses": self.syntheses,
            "contradictions": self.contradictions,
        }
        return json.dumps(result, ensure_ascii=False, indent=2)
        
    def summary(self) -> str:
        """Сводка интеграции"""
        return f"""
Интеграция ATLAS ↔ GrammaLang:
  Понятий ATLAS: {len(self.atlas_concepts)}
  Функций GrammaLang: {len(self.grammalang_functions)}
  Синтезов: {len(self.syntheses)}
  Противоречий: {len(self.contradictions)}
"""


class AtlasPipeline:
    """Интеграция ATLAS в конвейер GrammaLang"""
    
    def __init__(self):
        self.bridge = AtlasBridge()
        
    def process(self, source: str) -> Dict[str, Any]:
        """Обработка исходного кода через ATLAS + GrammaLang"""
        
        # 1. Анализ исходного кода
        contradictions = self._extract_contradictions(source)
        
        # 2. Синтез понятий
        syntheses = []
        for c in contradictions:
            if c["ready_for_synthesis"]:
                synthesis = self.bridge.synthesize(
                    c["node_a"],
                    c["node_b"],
                    strategy="hegelian",
                )
                syntheses.append(synthesis)
                
        # 3. Генерация кода
        code = self.bridge.export_to_grammalang()
        
        return {
            "contradictions": contradictions,
            "syntheses": syntheses,
            "generated_code": code,
        }
        
    def _extract_contradictions(self, source: str) -> List[Dict[str, Any]]:
        """Извлечение противоречий из исходного кода"""
        contradictions = []
        
        for line in source.split("\\n"):
            if "~::~" in line:
                parts = line.split("~::~")
                if len(parts) == 2:
                    contradictions.append({
                        "node_a": parts[0].strip(),
                        "node_b": parts[1].strip(),
                        "severity": 0.8,
                        "ready_for_synthesis": True,
                    })
                    
        return contradictions
'''

with open(os.path.join(GRAMMALANG, 'src/grammalang/atlas_bridge/__init__.py'), 'w', encoding='utf-8') as f:
    f.write(bridge)
print("atlas_bridge/__init__.py created")

# 2. Пример использования
example = '''"""
Пример интеграции ATLAS с GrammaLang
"""

from grammalang.atlas_bridge import AtlasBridge, AtlasPipeline


def demo():
    # Создаем мост
    bridge = AtlasBridge()
    
    # Загружаем понятия из ATLAS
    bridge.load_atlas_concepts({
        "свобода": {"category": "concept"},
        "безопасность": {"category": "concept"},
        "ответственность": {"category": "synthesis"},
    })
    
    # Загружаем функции из GrammaLang
    bridge.load_grammalang_functions({
        "привет": {"params": ["имя"], "return": "Строка"},
        "сложить": {"params": ["a", "b"], "return": "Целое"},
    })
    
    # Обнаруживаем противоречие
    contradiction = bridge.detect_contradiction("свобода", "безопасность", 0.8)
    
    # Синтезируем
    synthesis = bridge.synthesize("свобода", "безопасность", "hegelian")
    
    # Экспортируем
    grammalang_code = bridge.export_to_grammalang()
    atlas_json = bridge.export_to_atlas()
    
    print(bridge.summary())
    print("Сгенерированный код GrammaLang:")
    print(grammalang_code)
    
    # Конвейер
    pipeline = AtlasPipeline()
    result = pipeline.process("свобода ~::~ безопасность")
    print(f"Конвейер: {len(result['syntheses'])} синтезов")


if __name__ == "__main__":
    demo()
'''

with open(os.path.join(GRAMMALANG, 'examples_atlas_integration.py'), 'w', encoding='utf-8') as f:
    f.write(example)
print("example created")

print("\nIntegration files created!")
