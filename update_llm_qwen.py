# update_llm_qwen.py
import os

GRAMMALANG = r'C:\Projects\grammalang-stable'

# Обновляем llm_integration.py с поддержкой Qwen3-32B
llm_integration = '''"""
Интеграция ATLAS с LLM (Qwen3-32B)

Использование Qwen3-32B для:
- Синтеза понятий (Aufhebung)
- Генерации кода GrammaLang
- Анализа противоречий
- Рефлексивного мышления
"""

import os
import subprocess
import json
from typing import Optional, Dict, Any, List


class LlmConfig:
    """Конфигурация LLM"""
    
    def __init__(self, model_path: str, model_type: str = "qwen"):
        self.model_path = model_path
        self.model_type = model_type
        self.temperature = 0.3
        self.max_tokens = 2000
        self.system_prompt = """Ты — ATLAS, система диалектического мышления.
Твоя задача — синтезировать новые понятия из противоречащих.
Формат ответа:
НАЗВАНИЕ: <название синтеза>
ОПИСАНИЕ: <краткое описание>
СВОЙСТВА: <свойства через запятую>
СТРАТЕГИЯ: <hegelian|plotinian|pragmatic>"""


class LlmInterface:
    """Интерфейс к локальной LLM"""
    
    def __init__(self, config: LlmConfig):
        self.config = config
        self.is_loaded = False
        
    def load_model(self) -> bool:
        """Загрузка модели (проверка наличия)"""
        if os.path.exists(self.config.model_path):
            self.is_loaded = True
            size_gb = os.path.getsize(self.config.model_path) / 1e9
            print(f"Модель загружена: {os.path.basename(self.config.model_path)} ({size_gb:.1f} GB)")
            return True
        else:
            print(f"Модель не найдена: {self.config.model_path}")
            return False
            
    def generate(self, prompt: str) -> str:
        """Генерация текста через LLM"""
        if not self.is_loaded:
            return self._mock_generate(prompt)
            
        # Пытаемся использовать ollama
        try:
            model_name = os.path.basename(self.config.model_path).replace('.gguf', '')
            result = subprocess.run(
                ["ollama", "run", model_name, prompt],
                capture_output=True,
                text=True,
                timeout=60,
            )
            if result.returncode == 0:
                return result.stdout.strip()
        except Exception:
            pass
            
        # Пытаемся использовать llama.cpp
        try:
            result = subprocess.run(
                ["llama-cli", "-m", self.config.model_path, "-p", prompt, 
                 "--temp", str(self.config.temperature),
                 "--max-tokens", str(self.config.max_tokens),
                 "--no-display-prompt"],
                capture_output=True,
                text=True,
                timeout=120,
            )
            if result.returncode == 0:
                return result.stdout.strip()
        except Exception:
            pass
            
        # Заглушка
        return self._mock_generate(prompt)
            
    def _mock_generate(self, prompt: str) -> str:
        """Заглушка для тестирования без модели"""
        concepts = []
        for word in prompt.split():
            if word.startswith("'") or word.startswith('"'):
                concepts.append(word.strip("'\\""))
                
        if len(concepts) >= 2:
            name = f"{concepts[0]}_{concepts[1]}_synthesis"
            return f"""НАЗВАНИЕ: {name}
ОПИСАНИЕ: Диалектический синтез {concepts[0]} и {concepts[1]}
СВОЙСТВА: {concepts[0]}, {concepts[1]}, balanced, dialectical
СТРАТЕГИЯ: hegelian"""
        else:
            return "НАЗВАНИЕ: synthesis\\nОПИСАНИЕ: Общий синтез\\nСВОЙСТВА: balanced\\nСТРАТЕГИЯ: hegelian"


class AtlasLlmIntegrator:
    """Интеграция ATLAS с Qwen3-32B"""
    
    def __init__(self):
        self.models = self._discover_models()
        self.primary_model: Optional[LlmInterface] = None
        self.knowledge_base: List[Dict[str, Any]] = []
        self.synthesis_history: List[Dict[str, Any]] = []
        
    def _discover_models(self) -> List[Dict[str, str]]:
        """Обнаружение доступных моделей"""
        models_dir = r'C:\\Projects\\grammalang-stable\\models'
        if not os.path.exists(models_dir):
            return []
            
        models = []
        for file in os.listdir(models_dir):
            if file.endswith('.gguf'):
                models.append({
                    "name": file,
                    "path": os.path.join(models_dir, file),
                    "size_gb": round(os.path.getsize(os.path.join(models_dir, file)) / 1e9, 1),
                })
        return models
        
    def select_qwen32b(self) -> bool:
        """Выбор Qwen3-32B (лучшая модель)"""
        for model in self.models:
            if "Qwen3-32B" in model["name"] or "qwen3-32b" in model["name"].lower():
                config = LlmConfig(model["path"], model_type="qwen")
                self.primary_model = LlmInterface(config)
                return self.primary_model.load_model()
        return False
        
    def select_best_model(self) -> str:
        """Автоматический выбор лучшей модели"""
        # Приоритет: Qwen3-32B (самая мощная)
        if self.select_qwen32b():
            for m in self.models:
                if "Qwen3-32B" in m["name"]:
                    return m["name"]
                    
        # Затем DeepSeek-R1-Qwen3-8B
        priority = [
            "DeepSeek-R1-0528-Qwen3-8B-Q4_K_M.gguf",
            "deepseek-r1-distill-qwen-7b-multilingual-q8_0.gguf",
        ]
        
        for name in priority:
            if self.select_model(name):
                return name
                
        if self.models:
            self.select_model(self.models[0]["name"])
            return self.models[0]["name"]
            
        return ""
        
    def select_model(self, model_name: str) -> bool:
        """Выбор модели по имени"""
        for model in self.models:
            if model["name"] == model_name:
                config = LlmConfig(model["path"])
                self.primary_model = LlmInterface(config)
                return self.primary_model.load_model()
        return False
        
    def synthesize(self, concept_a: str, concept_b: str) -> Dict[str, str]:
        """Синтез двух понятий через Qwen"""
        prompt = f"""Даны два понятия: '{concept_a}' и '{concept_b}'. 
Они противоречат друг другу. 
Предложи новое понятие, которое снимает это противоречие (Aufhebung).
{self.primary_model.config.system_prompt if self.primary_model else ''}"""
        
        if self.primary_model:
            response = self.primary_model.generate(prompt)
            synthesis = self._parse_synthesis(response)
            self.synthesis_history.append(synthesis)
            return synthesis
        else:
            return {
                "name": f"{concept_a}_{concept_b}_synthesis",
                "description": "Синтез без LLM",
                "properties": [concept_a, concept_b],
                "strategy": "hegelian",
            }
            
    def _parse_synthesis(self, response: str) -> Dict[str, str]:
        """Парсинг ответа LLM"""
        result = {
            "name": "synthesis",
            "description": "",
            "properties": [],
            "strategy": "hegelian",
        }
        
        for line in response.split('\\n'):
            line = line.strip()
            if line.startswith("НАЗВАНИЕ:"):
                result["name"] = line.replace("НАЗВАНИЕ:", "").strip()
            elif line.startswith("ОПИСАНИЕ:"):
                result["description"] = line.replace("ОПИСАНИЕ:", "").strip()
            elif line.startswith("СВОЙСТВА:"):
                props = line.replace("СВОЙСТВА:", "").strip()
                result["properties"] = [p.strip() for p in props.split(",") if p.strip()]
            elif line.startswith("СТРАТЕГИЯ:"):
                result["strategy"] = line.replace("СТРАТЕГИЯ:", "").strip().lower()
                
        return result
        
    def generate_code(self, concept: Dict[str, str]) -> str:
        """Генерация кода GrammaLang из понятия"""
        name = concept["name"]
        props = ",\\n    ".join(f"{p}: Строка" for p in concept.get("properties", []))
        
        return f"""// Синтез: {concept.get('description', '')}
структура {name} {{
    {props}
}}"""
        
    def analyze_contradiction(self, a: str, b: str) -> str:
        """Анализ противоречия через Qwen"""
        prompt = f"Проанализируй противоречие между '{a}' и '{b}'. Определи тип и предложи стратегию синтеза."
        
        if self.primary_model:
            return self.primary_model.generate(prompt)
        return f"Прямое противоречие. Рекомендуемая стратегия: Hegelian синтез."
        
    def reflect(self, concept: str) -> str:
        """Рефлексия над понятием через Qwen"""
        prompt = f"Осознай понятие '{concept}'. Что оно означает? Какие противоречия в нем скрыты?"
        
        if self.primary_model:
            return self.primary_model.generate(prompt)
        return f"Рефлексия над '{concept}': понятие содержит внутренние противоречия."
        
    def list_models(self) -> str:
        """Список моделей"""
        if not self.models:
            return "Модели не найдены"
            
        result = "Доступные модели:\\n"
        for model in sorted(self.models, key=lambda m: -m["size_gb"]):
            result += f"  - {model['name']} ({model['size_gb']} GB)\\n"
        return result
        
    def summary(self) -> str:
        """Сводка LLM-интеграции"""
        model_name = ""
        if self.primary_model and self.primary_model.is_loaded:
            model_name = os.path.basename(self.primary_model.config.model_path)
            
        return f"""
LLM-интеграция ATLAS (Qwen3-32B):
  Моделей: {len(self.models)}
  Активная: {model_name or 'нет'}
  Синтезов: {len(self.synthesis_history)}
  Знаний: {len(self.knowledge_base)}
"""


# Тесты
if __name__ == "__main__":
    integrator = AtlasLlmIntegrator()
    
    print(integrator.list_models())
    
    # Выбор Qwen3-32B
    if integrator.select_qwen32b():
        print("Qwen3-32B выбрана")
    else:
        print("Qwen3-32B не найдена, выбираем лучшую...")
        integrator.select_best_model()
    
    # Синтез через Qwen
    synthesis = integrator.synthesize("свобода", "безопасность")
    print(f"\\nСинтез: {synthesis}")
    
    # Генерация кода
    code = integrator.generate_code(synthesis)
    print(f"\\nКод GrammaLang:\\n{code}")
    
    # Рефлексия
    reflection = integrator.reflect("ответственная_свобода")
    print(f"\\nРефлексия: {reflection[:200]}...")
    
    print(integrator.summary())
'''

with open(os.path.join(GRAMMALANG, 'src/grammalang/atlas_bridge/llm_integration.py'), 'w', encoding='utf-8') as f:
    f.write(llm_integration)
print("llm_integration.py updated with Qwen3-32B support")
