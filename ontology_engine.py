"""
ontology_engine.py — ядро GrammaLang v0.6.0
Мост между OntologicalContext (v0.3) и архитектурой Книги III.
"""

from __future__ import annotations
from typing import List, Dict, Optional, Any, Union
from enum import Enum
from datetime import datetime
from pydantic import BaseModel, Field
import uuid


# ==================== БАЗОВЫЕ ТИПЫ ====================

class ReactorPhase(str, Enum):
    """Фазы семантического реактора (из тома v0.6.0)."""
    INJECTION = "injection"
    IRRADIATION = "irradiation"
    PLASMA = "plasma"
    CRYSTALLIZATION = "crystallization"


class RelationType(str, Enum):
    STABILIZES = "стабилизирует"
    SUPPRESSES = "подавляет"
    REQUIRES = "требует"
    CONTRADICTS = "противоречит"
    RETROACTIVE = "ретроактивно_определяет"


class GestureOfDestruction(str, Enum):
    NULL = "null"
    COPULA = "destruction_of_copula"
    SUBJECTIVITY = "destruction_of_subjectivity"
    QUESTION = "destruction_of_question"
    DEFINITION = "destruction_of_definition"
    SPATIALITY = "destruction_of_spatiality"


class DaseinMode(str, Enum):
    FALLENNESS = "fallenness"
    AHEAD_OF_ITSELF = "ahead-of-itself"
    BEING_IN_THE_WORLD = "being-in-the-world"
    QUESTIONING = "questioning"


# ==================== УЗЕЛ ====================

class OntologicalNode(BaseModel):
    """Узел онтологической машины."""
    id: str = Field(default_factory=lambda: str(uuid.uuid4()))
    type: str = "Generic"
    mode: str = "initial"
    property: str = ""
    invariants: Dict[str, Any] = Field(default_factory=dict)
    cost: float = Field(default=0.0, ge=0.0)
    is_active: bool = True

    def enforce_invariants(self) -> bool:
        """Проверяет все инварианты. При нарушении — исключение."""
        for key, expected in self.invariants.items():
            current = getattr(self, key, None)
            if current != expected:
                raise ValueError(
                    f"Инвариант узла '{self.id}' нарушен: "
                    f"{key} = {current}, ожидается {expected}"
                )
        return True

    def change_mode(self, new_mode: str) -> None:
        """Меняет режим и перепроверяет инварианты."""
        self.mode = new_mode
        self.enforce_invariants()

    def check_stability(self) -> float:
        """Оценка стабильности узла 0.0–1.0."""
        if not self.is_active:
            return 0.0
        score = 1.0
        for key, val in self.invariants.items():
            if hasattr(self, key) and getattr(self, key, None) != val:
                score -= 0.3
        score -= min(self.cost / 10.0, 0.5)
        return max(0.0, score)

    def to_dict(self) -> Dict[str, Any]:
        return self.model_dump()


# ==================== СВЯЗЬ ====================

class OntologicalEdge(BaseModel):
    """Связь между узлами онтологической машины."""
    source: str
    target: str
    relation_type: RelationType
    weight: float = Field(default=0.5, ge=0.0, le=1.0)
    condition: Optional[str] = None

    def is_active(self, machine_state: Optional[Dict[str, Any]] = None) -> bool:
        """Проверяет, активна ли связь в данном состоянии машины."""
        if not self.condition:
            return True
        try:
            return bool(eval(self.condition, {"__builtins__": {}}, machine_state or {}))
        except Exception:
            return True

    def update_weight(self, delta: float) -> None:
        self.weight = max(0.0, min(1.0, self.weight + delta))

    def to_dict(self) -> Dict[str, Any]:
        return self.model_dump()


# ==================== ОПЕРАТОР ====================

class Operator(BaseModel):
    """Базовый оператор онтологической машины."""
    name: str
    params: Dict[str, Any] = Field(default_factory=dict)
    active: bool = False

    def activate(self) -> None:
        self.active = True

    def deactivate(self) -> None:
        self.active = False

    def apply(self, machine: OntologicalMachine) -> None:
        """Применить оператор к машине. Переопределяется в подклассах."""
        pass

    def to_dict(self) -> Dict[str, Any]:
        return self.model_dump()


class HierarchyOperator(Operator):
    """Оператор иерархии: блокирует связи, пересекающие уровни."""
    levels: List[str] = Field(default_factory=lambda: ["верхний", "средний", "нижний"])

    def apply(self, machine: OntologicalMachine) -> None:
        for edge in machine.edges:
            src = machine.get_node(edge.source)
            tgt = machine.get_node(edge.target)
            if src and tgt:
                try:
                    src_idx = self.levels.index(src.mode)
                    tgt_idx = self.levels.index(tgt.mode)
                    if src_idx > tgt_idx and edge.relation_type == RelationType.STABILIZES:
                        edge.weight = 0.0
                except ValueError:
                    pass


class PolyphonyOperator(Operator):
    """Оператор полифонии: подавляет доминантные голоса."""
    threshold: float = Field(default=0.6, ge=0.0, le=1.0)
    voice_diversity_index: float = Field(default=0.5, ge=0.0, le=1.0)

    def apply(self, machine: OntologicalMachine) -> None:
        voices = set(n.property for n in machine.nodes.values() if n.is_active)
        self.voice_diversity_index = len(voices) / max(1, len(machine.nodes))
        for node in machine.nodes.values():
            if node.check_stability() > 0.95:
                node.is_active = False
                break


class VirusTemplate(Operator):
    """Вирусный оператор: заражает узлы и меняет их режим."""
    target_property: str = ""
    pressure: float = Field(default=0.5, ge=0.0, le=1.0)
    conversion_rate: float = Field(default=0.0, ge=0.0, le=1.0)

    def apply(self, machine: OntologicalMachine) -> None:
        infected = [n for n in machine.nodes.values() if n.property == self.target_property]
        self.conversion_rate = len(infected) / len(machine.nodes) if machine.nodes else 0
        if self.conversion_rate > 0.8:
            for node in machine.nodes.values():
                node.mode = "infected"


# ==================== МАШИНА ====================

class OntologicalMachine(BaseModel):
    """Центральный класс: онтологическая машина."""
    id: str = Field(default_factory=lambda: str(uuid.uuid4()))
    nodes: Dict[str, OntologicalNode] = Field(default_factory=dict)
    edges: List[OntologicalEdge] = Field(default_factory=list)
    operators: List[Operator] = Field(default_factory=list)
    axis: Optional[str] = None  # ID узла-оси
    metrics: Dict[str, float] = Field(default_factory=lambda: {
        "stability_ratio": 1.0,
        "contradiction_index": 0.0,
        "abstraction_cost": 0.0
    })
    history: List[Dict[str, Any]] = Field(default_factory=list)  # TemporalMap

    def add_node(self, node: OntologicalNode) -> None:
        self.nodes[node.id] = node
        self.calculate_metrics()

    def get_node(self, node_id: str) -> Optional[OntologicalNode]:
        return self.nodes.get(node_id)

    def add_edge(self, edge: OntologicalEdge) -> None:
        self.edges.append(edge)
        self.calculate_metrics()

    def set_axis(self, node_id: str) -> None:
        if node_id in self.nodes:
            self.axis = node_id
            self.calculate_metrics()

    def calculate_metrics(self) -> None:
        if not self.nodes:
            self.metrics = {"stability_ratio": 1.0, "contradiction_index": 0.0, "abstraction_cost": 0.0}
            return
        node_stabilities = [n.check_stability() for n in self.nodes.values()]
        avg_stability = sum(node_stabilities) / len(node_stabilities)
        contradictions = [e for e in self.edges if e.relation_type == RelationType.CONTRADICTS and e.is_active()]
        self.metrics["contradiction_index"] = min(1.0, len(contradictions) * 0.2)
        self.metrics["abstraction_cost"] = sum(n.cost for n in self.nodes.values())
        self.metrics["stability_ratio"] = max(0.0, avg_stability - self.metrics["contradiction_index"] * 0.5)

    def step(self) -> Dict[str, float]:
        """Один такт симуляции."""
        self.calculate_metrics()
        for op in self.operators:
            if op.active:
                op.apply(self)
        snapshot = {
            "timestamp": datetime.now().isoformat(),
            "metrics": {**self.metrics},
            "node_count": len(self.nodes),
            "edge_count": len(self.edges),
            "axis": self.axis,
            "active_operators": [op.name for op in self.operators if op.active]
        }
        self.history.append(snapshot)
        return self.metrics

    def run_simulation(self, steps: int, interventions: List[Dict[str, Any]] = None) -> List[Dict[str, Any]]:
        """Запускает симуляцию на steps тактов с возможными вмешательствами."""
        interventions = interventions or []
        for i in range(steps):
            for intervention in interventions:
                if intervention.get("step") == i:
                    action = intervention.get("action")
                    params = intervention.get("params", {})
                    if action == "add_node":
                        self.add_node(OntologicalNode(**params))
            self.step()
        return self.history

    def check_hold_break(self) -> bool:
        """Проверяет, достигнут ли HOLD_BREAK."""
        return self.metrics["stability_ratio"] < 0.2

    def to_dict(self) -> Dict[str, Any]:
        return {
            "id": self.id,
            "axis": self.axis,
            "nodes": {k: v.to_dict() for k, v in self.nodes.items()},
            "edges": [e.to_dict() for e in self.edges],
            "operators": [op.to_dict() for op in self.operators],
            "metrics": self.metrics,
            "history": self.history
        }

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> OntologicalMachine:
        m = cls()
        for n_id, n_data in data.get("nodes", {}).items():
            m.nodes[n_id] = OntologicalNode(**n_data)
        for e_data in data.get("edges", []):
            m.edges.append(OntologicalEdge(**e_data))
        m.axis = data.get("axis")
        m.calculate_metrics()
        return m


# ==================== СОВМЕСТИМОСТЬ С v0.3 ====================

def context_to_machine(context: Any) -> OntologicalMachine:
    """Конвертирует OntologicalContext (v0.3) в OntologicalMachine (v0.6.0)."""
    machine = OntologicalMachine()
    # substance → node
    for sub in context.substances.values():
        node = OntologicalNode(id=sub.id, type="Substance", property=sub.name, cost=1.0 - sub.energy)
        machine.add_node(node)
    # tension → edge
    for tension in context.tensions:
        if tension.pole_a in machine.nodes and tension.pole_b in machine.nodes:
            edge = OntologicalEdge(
                source=tension.pole_a,
                target=tension.pole_b,
                relation_type=RelationType.CONTRADICTS if tension.status == "held" else RelationType.STABILIZES,
                weight=0.8
            )
            machine.add_edge(edge)
    return machine


# ==================== ТЕСТ ====================

if __name__ == "__main__":
    print("=== Тест OntologicalEngine v0.6.0 ===\n")

    # 1. Создание машины
    machine = OntologicalMachine()
    print("1. Машина создана")

    # 2. Добавление узлов
    thesis = OntologicalNode(type="Концепт", property="Тезис", invariants={"is_active": True})
    antithesis = OntologicalNode(type="Концепт", property="Антитезис", invariants={"is_active": True})
    synthesis = OntologicalNode(type="Концепт", property="Синтез", invariants={"is_active": True})
    machine.add_node(thesis)
    machine.add_node(antithesis)
    machine.add_node(synthesis)
    print(f"2. Добавлены узлы: {list(machine.nodes.keys())}")

    # 3. Добавление связей
    e1 = OntologicalEdge(source=thesis.id, target=antithesis.id, relation_type=RelationType.CONTRADICTS, weight=0.9)
    e2 = OntologicalEdge(source=antithesis.id, target=synthesis.id, relation_type=RelationType.REQUIRES, weight=0.7)
    machine.add_edge(e1)
    machine.add_edge(e2)
    print(f"3. Добавлены связи: {len(machine.edges)}")

    # 4. Установка оси
    machine.set_axis(thesis.id)
    print(f"4. Ось установлена: {machine.axis}")

    # 5. Метрики
    print(f"5. Метрики: {machine.metrics}")

    # 6. Симуляция
    history = machine.run_simulation(5)
    print(f"6. Симуляция 5 тактов: записей в истории {len(history)}")

    # 7. HOLD_BREAK
    print(f"7. HOLD_BREAK: {machine.check_hold_break()}")

    # 8. Добавление оператора
    poly = PolyphonyOperator(name="Polyphony")
    poly.activate()
    machine.operators.append(poly)
    machine.step()
    print(f"8. Оператор Polyphony добавлен, voice_diversity_index = {poly.voice_diversity_index}")

    print("\n=== Тест пройден ===")
