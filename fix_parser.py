# fix_parser.py
content = open(r'C:\Projects\grammalang-stable\src\grammalang\atlas_bridge\llm_integration.py', 'r', encoding='utf-8').read()

# Исправляем _mock_generate
content = content.replace(
    '''concepts = []
        for word in prompt.split():
            if word.startswith("'") or word.startswith('"'):
                concepts.append(word.strip("'\\""))
                
        if len(concepts) >= 2:
            name = f"{concepts[0]}_{concepts[1]}_synthesis"
            return f"""НАЗВАНИЕ: {name}
ОПИСАНИЕ: Диалектический синтез {concepts[0]} и {concepts[1]}
СВОЙСТВА: {concepts[0]}, {concepts[1]}, balanced, dialectical
СТРАТЕГИЯ: hegelian"""''',
    '''concepts = []
        import re
        # Извлекаем понятия из кавычек
        matches = re.findall(r"['\\"]([^'\\"]+)['\\"]", prompt)
        concepts = matches[:2]
                
        if len(concepts) >= 2:
            name = f"{concepts[0]}_{concepts[1]}_synthesis"
            return f"""НАЗВАНИЕ: {name}
ОПИСАНИЕ: Диалектический синтез {concepts[0]} и {concepts[1]}
СВОЙСТВА: {concepts[0]}, {concepts[1]}, balanced, dialectical
СТРАТЕГИЯ: hegelian"""'''
)

# Исправляем парсинг
content = content.replace(
    '''    def _parse_synthesis(self, response: str) -> Dict[str, str]:
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
                
        return result''',
    '''    def _parse_synthesis(self, response: str) -> Dict[str, str]:
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
                name = line.replace("НАЗВАНИЕ:", "").strip()
                # Очищаем имя от кавычек и лишних символов
                name = name.replace("'", "").replace('"', "").strip()
                result["name"] = name
            elif line.startswith("ОПИСАНИЕ:"):
                desc = line.replace("ОПИСАНИЕ:", "").strip()
                desc = desc.replace("'", "").replace('"', "").strip()
                result["description"] = desc
            elif line.startswith("СВОЙСТВА:"):
                props = line.replace("СВОЙСТВА:", "").strip()
                result["properties"] = [
                    p.strip().replace("'", "").replace('"', "").strip() 
                    for p in props.split(",") if p.strip()
                ]
            elif line.startswith("СТРАТЕГИЯ:"):
                result["strategy"] = line.replace("СТРАТЕГИЯ:", "").strip().lower()
                
        return result'''
)

open(r'C:\Projects\grammalang-stable\src\grammalang\atlas_bridge\llm_integration.py', 'w', encoding='utf-8').write(content)
print('Parser fixed')
