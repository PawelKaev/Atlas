"""
integrate_refraction.py — мост между RefractionAnalyzer и OntologicalMachine.
Берёт изломы, строит онтологическую машину, симулирует.
"""

import json
from typing import Dict, Any, List, Optional
from refraction_analyzer import (
    RefractionAnalyzer, RefractionMarker, ArchitectureGap,
    MediumType, RefractionType
)
from ontology_engine import (
    OntologicalMachine, OntologicalNode, OntologicalEdge,
    RelationType, Operator, PolyphonyOperator
)


def markers_to_machine(analyzer: RefractionAnalyzer) -> OntologicalMachine:
    """
    Конвертирует все маркеры изломов в OntologicalMachine.
    Каждый маркер → узел.
    Каждая связь (relations) → ребро.
    Архитектурные зазоры → операторы.
    """
    machine = OntologicalMachine()
    
    # 1. Добавляем узлы из маркеров
    node_ids = []
    for marker in analyzer.markers:
        # Имя узла — фрагмент текста (короткое)
        name = marker.fragment_text[:60] if marker.fragment_text else f"fragment_{marker.fragment_id[:8]}"
        
        # Инварианты из метрик маркера
        invariants = {
            "measurement": marker.measurement,
            "medium": marker.medium.value,
            "operator": marker.operator or "unknown"
        }
        
        node = OntologicalNode(
            id=marker.fragment_id,
            type="RefractionPoint",
            property=name,
            mode="active" if marker.measurement > 0.5 else "latent",
            invariants=invariants,
            cost=marker.measurement
        )
        machine.add_node(node)
        node_ids.append(marker.fragment_id)
    
    # 2. Добавляем связи из relations
    for marker in analyzer.markers:
        for target_id in marker.relations:
            if target_id in machine.nodes:
                edge = OntologicalEdge(
                    source=marker.fragment_id,
                    target=target_id,
                    relation_type=RelationType.REQUIRES,
                    weight=0.7
                )
                machine.add_edge(edge)
    
    # 3. Устанавливаем ось — узел с максимальным measurement
    if node_ids:
        max_node = max(node_ids, key=lambda nid: machine.nodes[nid].cost)
        machine.set_axis(max_node)
    
    # 4. Добавляем оператор Polyphony для удержания противоречий
    poly = PolyphonyOperator(name="Polyphony", threshold=0.5)
    poly.activate()
    machine.operators.append(poly)
    
    # 5. Архитектурные зазоры → дополнительные узлы-операторы
    for gap in analyzer.gaps:
        gap_node = OntologicalNode(
            type="ArchitectureGap",
            property=gap.gap_dimension,
            mode="active" if gap.measurement > 0.5 else "latent",
            invariants={
                "measurement": gap.measurement,
                "operator": gap.operator,
                "human_anchor": gap.human_anchor
            },
            cost=gap.measurement
        )
        machine.add_node(gap_node)
    
    return machine


def run_refraction_simulation(
    text: str,
    source_file: str = "unknown.txt",
    window_size: int = 200,
    step: int = 100,
    sensitivity: float = 0.5,
    simulation_steps: int = 20
) -> Dict[str, Any]:
    """
    Полный цикл: текст → изломы → машина → симуляция → результат.
    """
    # Шаг 1: Анализ изломов
    analyzer = RefractionAnalyzer(sensitivity=sensitivity)
    
    # Структурные изломы
    analyzer.analyze_structural_gap(
        text,
        window_size=window_size,
        step=step,
        source_file=source_file
    )
    
    # Сетка окон
    analyzer.prepare_annotation_windows(
        text,
        window_size=window_size,
        step=step,
        source_file=source_file
    )
    
    # Шаг 2: Построение машины
    machine = markers_to_machine(analyzer)
    
    # Шаг 3: Симуляция
    history = machine.run_simulation(simulation_steps)
    
    # Шаг 4: Результат
    return {
        "source_file": source_file,
        "text_length": len(text),
        "refraction_report": analyzer.get_refraction_report(),
        "machine": machine.to_dict(),
        "simulation": {
            "steps": simulation_steps,
            "hold_break_detected": machine.check_hold_break(),
            "final_metrics": machine.metrics,
            "history": history
        }
    }


def analyze_file(
    filepath: str,
    window_size: int = 200,
    step: int = 100,
    sensitivity: float = 0.5,
    simulation_steps: int = 20,
    output: Optional[str] = None
) -> Dict[str, Any]:
    """Анализирует текстовый файл."""
    with open(filepath, 'r', encoding='utf-8') as f:
        text = f.read()
    
    result = run_refraction_simulation(
        text,
        source_file=filepath,
        window_size=window_size,
        step=step,
        sensitivity=sensitivity,
        simulation_steps=simulation_steps
    )
    
    if output:
        with open(output, 'w', encoding='utf-8') as f:
            json.dump(result, f, ensure_ascii=False, indent=2)
        print(f"Результат сохранён в {output}")
    
    return result


# ==================== ТЕСТ ====================

if __name__ == "__main__":
    # Тестовый текст: фрагмент в духе Достоевского
    test_text = (
        "Он остановился у окна. Молчание. Всё внутри сжалось. "
        "И вдруг — резкий шаг вперёд, будто кто-то толкнул его в спину. "
        "Время будто остановилось, но сердце билось всё быстрее. "
        "«Я должен это сделать», — прошептал он, и голос его дрожал. "
        "Никто не ответил. Тишина давила сильнее слов. "
        "И тут он понял: всё рушится. Всё."
    )
    
    print("=== Интеграционный тест: изломы → машина → симуляция ===\n")
    
    result = run_refraction_simulation(
        test_text,
        source_file="dostoevsky_fragment.txt",
        window_size=120,
        step=60,
        sensitivity=0.5,
        simulation_steps=15
    )
    
    # Краткий вывод
    ref_report = result["refraction_report"]
    sim = result["simulation"]
    
    print(f"Изломов найдено: {ref_report['total_refractions']}")
    print(f"Узлов в машине: {len(result['machine']['nodes'])}")
    print(f"Связей в машине: {len(result['machine']['edges'])}")
    print(f"Ось: {result['machine']['axis'][:20]}...")
    print(f"Тактов симуляции: {sim['steps']}")
    print(f"HOLD_BREAK: {sim['hold_break_detected']}")
    print(f"Итоговая стабильность: {sim['final_metrics']['stability_ratio']:.3f}")
    print(f"Индекс противоречий: {sim['final_metrics']['contradiction_index']:.3f}")
    
    # Динамика по тактам
    print("\nДинамика stability_ratio:")
    for i, snap in enumerate(sim['history']):
        sr = snap['metrics']['stability_ratio']
        bar = '█' * int(sr * 20)
        print(f"  Такт {i:2d}: {sr:.3f} {bar}")
