"""
pipeline.py — конвейер GrammaLang v0.6.0
Соединяет WillAnalyzer, OntologicalContext и OntologicalMachine.
"""

import json
from typing import Dict, Any, Optional, List
from will_analyzer import WillAnalyzer
from ontological_choice import OntologicalContext, apply_will_to_context
from ontology_engine import (
    OntologicalMachine, OntologicalNode, OntologicalEdge,
    RelationType, context_to_machine
)


def analyze_text(text: str, steps: int = 10) -> Dict[str, Any]:
    """
    Полный цикл анализа текста:
    1. WillAnalyzer — индекс воли (Парменид/Гераклит)
    2. OntologicalContext — субстанции, модусы, тип онтологии
    3. Конвертация в OntologicalMachine (v0.6.0)
    4. Симуляция
    """
    # Шаг 1: Анализ воли
    will = WillAnalyzer()
    indices, sentences = will.analyze_text(text)
    summary = will.get_summary(indices)
    
    # Шаг 2: Онтологический контекст
    context = OntologicalContext()
    
    # Заполняем субстанции: имя предложения → {energy: ...}
    for i, sent in enumerate(sentences):
        name = sent[:80] if sent else f"sentence_{i}"
        energy = (indices[i] + 1.0) / 2.0 if i < len(indices) else 0.5
        context.substances[name] = {
            "energy": max(0.0, min(1.0, energy)),
            "original_index": i
        }
    
    # Применяем выбор онтологии
    context = apply_will_to_context(context, indices, sentences)
    
    # Шаг 3: Конвертация в машину v0.6.0
    machine = OntologicalMachine()
    
    # Добавляем узлы из субстанций
    for sub_name, sub_data in context.substances.items():
        node = OntologicalNode(
            type="Substance",
            property=sub_name[:50],
            cost=1.0 - sub_data.get("energy", 0.5)
        )
        machine.add_node(node)
    
    node_ids = list(machine.nodes.keys())
    
    # Связываем соседние узлы
    for i in range(len(node_ids) - 1):
        edge = OntologicalEdge(
            source=node_ids[i],
            target=node_ids[i + 1],
            relation_type=RelationType.STABILIZES,
            weight=0.5
        )
        machine.add_edge(edge)
    
    # Устанавливаем первый узел как ось
    if node_ids:
        machine.set_axis(node_ids[0])
    
    # Шаг 4: Симуляция
    history = machine.run_simulation(steps)
    
    return {
        "will_summary": summary,
        "ontology_type": context.ontology_type,
        "sentence_count": len(sentences),
        "substance_count": len(context.substances),
        "node_count": len(machine.nodes),
        "edge_count": len(machine.edges),
        "axis": machine.axis,
        "simulation_steps": steps,
        "hold_break_detected": machine.check_hold_break(),
        "final_metrics": machine.metrics,
        "history": history
    }


def analyze_file(filepath: str, steps: int = 10, output: Optional[str] = None) -> Dict[str, Any]:
    """Анализирует текстовый файл."""
    with open(filepath, 'r', encoding='utf-8') as f:
        text = f.read()
    result = analyze_text(text, steps)
    if output:
        with open(output, 'w', encoding='utf-8') as f:
            json.dump(result, f, ensure_ascii=False, indent=2)
        print(f"Результат сохранён в {output}")
    return result


# ==================== CLI ====================

if __name__ == "__main__":
    import sys
    
    if len(sys.argv) > 1:
        filepath = sys.argv[1]
        steps = int(sys.argv[2]) if len(sys.argv) > 2 else 10
        output = sys.argv[3] if len(sys.argv) > 3 else None
        result = analyze_file(filepath, steps, output)
        print(f"Тип онтологии: {result['ontology_type']}")
        print(f"Средний индекс воли: {result['will_summary']['mean']:.2f}")
        print(f"Парменид: {result['will_summary']['parmenides_share']*100:.0f}%")
        print(f"HOLD_BREAK: {result['hold_break_detected']}")
        print(f"Stability: {result['final_metrics']['stability_ratio']:.2f}")
    else:
        sample = """
        Бытие есть. Небытия нет. Бытие и небытие противоречат друг другу.
        Но без небытия невозможно помыслить бытие. Следовательно, они требуют друг друга.
        Это противоречие держит мысль в напряжении.
        """
        result = analyze_text(sample, steps=10)
        print(json.dumps(result, ensure_ascii=False, indent=2))
