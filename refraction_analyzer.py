import json
import hashlib
from collections import Counter
from typing import List, Dict, Any, Optional, Set
from dataclasses import dataclass, field
from enum import Enum

# ==================== КОНФИГУРАЦИЯ И КОНСТАНТЫ ====================

SEMANTIC_WEIGHTS = {
    "воля": 3.0, "will": 3.0,
    "вирус": 3.0, "virus": 3.0,
    "время": 3.0, "time": 3.0,
    "удерживать": 2.5, "удержание": 2.5, "удерживается": 2.5,
    "разрыв": 2.5, "напряжение": 2.5,
    "смысл": 2.0, "бытие": 2.0, "сущее": 2.0,
    "интенция": 2.0, "субъект": 2.0,
    "должен": 2.0, "необходимо": 2.0, "следует": 2.0,
    "не": 2.0, "ни": 2.0,
}

PRONOUNS: Set[str] = {"я", "ты", "он", "она", "оно", "мы", "вы", "они"}


def detect_operator(text: str) -> Optional[str]:
    """Автоматически определяет оператор GrammaLang по наличию ключевых слов."""
    words = text.lower().split()
    for w in words:
        if w in SEMANTIC_WEIGHTS:
            return w
    return None


class MediumType(str, Enum):
    TRANSLATION = "translation"
    PERCEPTION = "perception"
    POLYPHONY = "polyphony"
    TEMPORAL = "temporal_structure"
    ONTOLOGY = "ontology"
    SEMANTIC = "semantic"


class RefractionType(str, Enum):
    SEMANTIC_SHIFT = "semantic_shift"
    STRUCTURAL_BREAK = "structural_break"
    PERCEPTUAL_FLATTENING = "perceptual_flattening"
    TEMPORAL_DRIFT = "temporal_drift"
    POLYPHONIC_COLLAPSE = "polyphonic_collapse"
    SIMULACRUM_SMOOTHNESS = "simulacrum_smoothness"


@dataclass
class RefractionMarker:
    fragment_id: str
    fragment_text: str
    source_file: Optional[str] = None
    start_offset: Optional[int] = None
    end_offset: Optional[int] = None
    medium: MediumType = MediumType.SEMANTIC
    refraction_type: RefractionType = RefractionType.STRUCTURAL_BREAK
    measurement: float = 0.0
    measurement_method: str = "unknown"
    operator: Optional[str] = None
    relations: List[str] = field(default_factory=list)
    notes: Optional[str] = None

    def to_dict(self) -> Dict[str, Any]:
        return {
            "fragment_id": self.fragment_id,
            "fragment_text": self.fragment_text,
            "source_file": self.source_file,
            "start_offset": self.start_offset,
            "end_offset": self.end_offset,
            "medium": self.medium.value,
            "refraction_type": self.refraction_type.value,
            "measurement": self.measurement,
            "measurement_method": self.measurement_method,
            "operator": self.operator,
            "relations": self.relations,
            "notes": self.notes,
        }


@dataclass
class ArchitectureGap:
    fragment_id: str
    text: str
    gap_dimension: str
    operator: str
    human_anchor: str
    measurement: float
    detection_source: str
    notes: Optional[str] = None

    def to_dict(self) -> Dict[str, Any]:
        return {
            "fragment_id": self.fragment_id,
            "text": self.text,
            "gap_dimension": self.gap_dimension,
            "operator": self.operator,
            "human_anchor": self.human_anchor,
            "measurement": self.measurement,
            "detection_source": self.detection_source,
            "notes": self.notes,
        }


# ==================== УТИЛИТЫ ====================

def make_id(prefix: str, content: str) -> str:
    h = hashlib.sha256(content.encode("utf-8")).hexdigest()[:12]
    return f"{prefix}_{h}"


def sliding_windows(text: str, window_size: int = 200, step: int = 100) -> List[Dict[str, Any]]:
    words = text.split()
    windows = []
    word_positions = []
    current_char_index = 0
    for w in words:
        word_positions.append(current_char_index)
        current_char_index += len(w) + 1

    for start_word_idx in range(0, len(words), step):
        end_word_idx = min(start_word_idx + window_size, len(words))
        if start_word_idx >= len(words):
            break
        start_offset = word_positions[start_word_idx]
        win_text = " ".join(words[start_word_idx:end_word_idx])
        end_offset = start_offset + len(win_text)
        windows.append({
            "text": win_text,
            "start_offset": start_offset,
            "end_offset": end_offset,
        })
    return windows


def _weighted_loss(orig_text: str, resp_text: str) -> float:
    orig_words = orig_text.lower().split()
    resp_words = resp_text.lower().split()
    lost_words = [w for w in orig_words if w not in resp_words]
    total_weight = sum(SEMANTIC_WEIGHTS.get(w, 1.0) for w in orig_words)
    lost_weight = sum(SEMANTIC_WEIGHTS.get(w, 1.0) for w in lost_words)
    if total_weight == 0:
        return 0.0
    return min(1.0, lost_weight / total_weight)


# ==================== АНАЛИЗАТОР ====================

class RefractionAnalyzer:
    def __init__(self, sensitivity: float = 0.5):
        self.sensitivity = sensitivity
        self.markers: List[RefractionMarker] = []
        self.gaps: List[ArchitectureGap] = []

    @staticmethod
    def link_markers_by_window(markers: List[RefractionMarker]) -> None:
        if len(markers) < 2:
            return
        for i in range(len(markers) - 1):
            markers[i].relations.append(markers[i + 1].fragment_id)
            markers[i + 1].relations.append(markers[i].fragment_id)

    def analyze_translation_gap(
        self, original: str, translation: str,
        fragment_id: Optional[str] = None, source_file: Optional[str] = None
    ) -> RefractionMarker:
        fid = fragment_id or make_id("trans", original)
        orig_sentences = [s.strip() for s in original.replace("!", ".").replace("?", ".").split(".") if s.strip()]
        trans_sentences = [s.strip() for s in translation.replace("!", ".").replace("?", ".").split(".") if s.strip()]
        sent_count_diff = abs(len(orig_sentences) - len(trans_sentences)) / max(1, max(len(orig_sentences), len(trans_sentences)))
        orig_avg_len = sum(len(s.split()) for s in orig_sentences) / max(1, len(orig_sentences))
        trans_avg_len = sum(len(s.split()) for s in trans_sentences) / max(1, len(trans_sentences))
        len_diff = abs(orig_avg_len - trans_avg_len) / max(1, max(orig_avg_len, trans_avg_len))
        measurement = round((sent_count_diff * 0.4 + len_diff * 0.6), 3)
        measurement = min(1.0, measurement)
        op = detect_operator(original)
        marker = RefractionMarker(
            fragment_id=fid, fragment_text=original[:100], source_file=source_file,
            start_offset=0, end_offset=len(original),
            medium=MediumType.TRANSLATION, refraction_type=RefractionType.SEMANTIC_SHIFT,
            measurement=measurement, measurement_method="sentence_structure_divergence",
            operator=op, notes=f"Расхождение предложений: {sent_count_diff:.2f}, расхождение длины: {len_diff:.2f}"
        )
        self.markers.append(marker)
        return marker

    def analyze_structural_gap(
        self, text: str, fragment_id_prefix: str = "struct",
        window_size: int = 200, step: int = 100, source_file: Optional[str] = None
    ) -> List[RefractionMarker]:
        windows = sliding_windows(text, window_size, step)
        markers = []
        for win in windows:
            win_text = win["text"]
            win_start_offset = win["start_offset"]
            sentences = [s.strip() for s in win_text.replace("!", ".").replace("?", ".").split(".") if s.strip()]
            if len(sentences) < 2:
                continue
            win_char_pos = 0
            sentence_offsets = []
            for s in sentences:
                sentence_offsets.append(win_char_pos)
                win_char_pos += len(s) + 1
            for j, sent in enumerate(sentences):
                words = sent.split()
                if len(words) < 3:
                    continue
                op = detect_operator(sent)
                # 1. Перепад длины
                if j > 0:
                    prev_len = len(sentences[j - 1].split())
                    curr_len = len(words)
                    if prev_len > 0:
                        ratio = abs(curr_len - prev_len) / max(prev_len, curr_len)
                        if ratio > self.sensitivity:
                            fid = make_id(f"{fragment_id_prefix}_len", sent)
                            marker = RefractionMarker(
                                fragment_id=fid, fragment_text=sent[:100], source_file=source_file,
                                start_offset=win_start_offset + sentence_offsets[j],
                                end_offset=win_start_offset + sentence_offsets[j] + len(sent),
                                medium=MediumType.SEMANTIC, refraction_type=RefractionType.STRUCTURAL_BREAK,
                                measurement=round(ratio, 3), measurement_method="sentence_length_ratio",
                                operator=op, notes=f"Перепад длины: {prev_len} → {curr_len} слов (ratio={ratio:.2f})"
                            )
                            markers.append(marker)
                # 2. Смена местоимения
                found_pronouns = [w.lower() for w in words if w.lower() in PRONOUNS]
                if j > 0 and found_pronouns:
                    prev_words = sentences[j - 1].lower().split()
                    prev_pronouns = [w for w in prev_words if w in PRONOUNS]
                    if prev_pronouns and found_pronouns[0] != prev_pronouns[0]:
                        fid = make_id(f"{fragment_id_prefix}_pron", sent)
                        marker = RefractionMarker(
                            fragment_id=fid, fragment_text=sent[:100], source_file=source_file,
                            start_offset=win_start_offset + sentence_offsets[j],
                            end_offset=win_start_offset + sentence_offsets[j] + len(sent),
                            medium=MediumType.ONTOLOGY, refraction_type=RefractionType.STRUCTURAL_BREAK,
                            measurement=0.6, measurement_method="pronoun_shift_detection",
                            operator=op, notes=f"Смена лица: {prev_pronouns[0]} → {found_pronouns[0]}"
                        )
                        markers.append(marker)
                # 3. Эллипсис
                if len(words) <= 5 and j > 0 and len(sentences[j - 1].split()) > 12:
                    fid = make_id(f"{fragment_id_prefix}_ellipsis", sent)
                    marker = RefractionMarker(
                        fragment_id=fid, fragment_text=sent[:100], source_file=source_file,
                        start_offset=win_start_offset + sentence_offsets[j],
                        end_offset=win_start_offset + sentence_offsets[j] + len(sent),
                        medium=MediumType.POLYPHONY, refraction_type=RefractionType.PERCEPTUAL_FLATTENING,
                        measurement=0.7, measurement_method="ellipsis_detection",
                        operator=op, notes=f"Эллипсис: резкое сокращение с {len(sentences[j-1].split())} до {len(words)} слов"
                    )
                    markers.append(marker)
                # 4. Пунктуационное напряжение
                punct_density = sent.count("—") + sent.count("-") + sent.count(":") + sent.count(";")
                if punct_density >= 2 and len(words) >= 8:
                    fid = make_id(f"{fragment_id_prefix}_punct", sent)
                    marker = RefractionMarker(
                        fragment_id=fid, fragment_text=sent[:100], source_file=source_file,
                        start_offset=win_start_offset + sentence_offsets[j],
                        end_offset=win_start_offset + sentence_offsets[j] + len(sent),
                        medium=MediumType.TEMPORAL, refraction_type=RefractionType.STRUCTURAL_BREAK,
                        measurement=round(min(1.0, punct_density * 0.2), 3),
                        measurement_method="punctuation_density",
                        operator=op, notes=f"Пунктуационное напряжение: {punct_density} знаков"
                    )
                    markers.append(marker)
        self.link_markers_by_window(markers)
        self.markers.extend(markers)
        return markers

    def prepare_annotation_windows(
        self, text: str, window_size: int = 200, step: int = 100,
        source_file: Optional[str] = None
    ) -> List[RefractionMarker]:
        windows = sliding_windows(text, window_size, step)
        markers = []
        for win in windows:
            win_text = win["text"]
            fid = make_id("ann", win_text)
            op = detect_operator(win_text)
            marker = RefractionMarker(
                fragment_id=fid, fragment_text=win_text[:150], source_file=source_file,
                start_offset=win["start_offset"], end_offset=win["end_offset"],
                medium=MediumType.PERCEPTION, refraction_type=RefractionType.SIMULACRUM_SMOOTHNESS,
                measurement=0.0, measurement_method="window_generation",
                operator=op, notes=f"Окно разметки: длина {len(win_text)} символов, оператор: {op or 'нет'}"
            )
            markers.append(marker)
        self.link_markers_by_window(markers)
        self.markers.extend(markers)
        return markers

    def detect_architecture_gap(
        self, text: str, llm_response: str, gap_dimension: str,
        human_anchor: str, operator: str,
        fragment_id: Optional[str] = None, source_file: Optional[str] = None
    ) -> ArchitectureGap:
        fid = fragment_id or make_id("gap", text)
        measurement = _weighted_loss(text, llm_response)
        gap = ArchitectureGap(
            fragment_id=fid, text=text[:150], gap_dimension=gap_dimension,
            operator=operator, human_anchor=human_anchor,
            measurement=round(measurement, 3), detection_source="weighted_semantic_loss",
            notes=f"Семантическая потеря: {measurement:.2f} (ключевые термины учтены)"
        )
        self.gaps.append(gap)
        return gap

    def get_refraction_report(self) -> Dict[str, Any]:
        if not self.markers and not self.gaps:
            return {"status": "no_refractions", "refractions": [], "architecture_gaps": [], "summary": {}}
        measurements = [m.measurement for m in self.markers]
        operators = [m.operator for m in self.markers if m.operator]
        return {
            "status": "refractions_detected",
            "total_refractions": len(self.markers),
            "total_architecture_gaps": len(self.gaps),
            "average_measurement": round(sum(measurements) / len(measurements), 3) if measurements else 0.0,
            "max_measurement": max(measurements) if measurements else 0.0,
            "min_measurement": min(measurements) if measurements else 0.0,
            "operator_distribution": {op: operators.count(op) for op in set(operators)},
            "refractions": [m.to_dict() for m in self.markers],
            "architecture_gaps": [g.to_dict() for g in self.gaps],
            "summary": {},
        }

    def to_ontology_nodes(self) -> List[Dict[str, Any]]:
        nodes = []
        for marker in self.markers:
            nodes.append({
                "type": "RefractionPoint",
                "property": f"{marker.medium.value}_{marker.refraction_type.value}",
                "mode": "active" if marker.measurement > 0.5 else "latent",
                "invariants": {
                    "measurement": marker.measurement,
                    "medium": marker.medium.value,
                    "operator": marker.operator or "unknown"
                },
                "cost": marker.measurement,
                "source_file": marker.source_file,
                "start_offset": marker.start_offset,
                "end_offset": marker.end_offset,
                "relations": marker.relations,
            })
        return nodes

    def print_human_summary(self):
        """Выводит отчёт в формате, готовом для вставки в книгу."""
        print("\n=== ОТЧЁТ ДЛЯ КНИГИ (GrammaLang v0.6.0) ===")
        print(f"Всего маркеров: {len(self.markers)}")
        strong = sum(1 for m in self.markers if m.measurement > 0.5)
        print(f"Сильных изломов (meas > 0.5): {strong}")

        operators = [m.operator for m in self.markers if m.operator]
        if operators:
            print("Доминирующие операторы:", Counter(operators).most_common(5))

        gaps = [g for g in self.gaps if g.measurement > 0.3]
        if gaps:
            print(f"Критичные семантические потери (>0.3): {len(gaps)}")
            for g in gaps:
                print(f"  - {g.operator}: потеря {g.measurement:.2f} — {g.human_anchor}")
        else:
            print("Критичных семантических потерь не обнаружено.")
        print("==========================================\n")


# ==================== ТЕСТ ====================

if __name__ == "__main__":
    sample_dostoevsky = (
        "Он остановился у окна. Молчание. Всё внутри сжалось. "
        "И вдруг — резкий шаг вперёд, будто кто-то толкнул его в спину. "
        "Время будто остановилось, но сердце билось всё быстрее. "
        "«Я должен это сделать», — прошептал он, и голос его дрожал."
    )
    sample_heidegger = (
        "Бытие и время: вопрос о смысле бытия раскрывается как напряжение между присутствием и отсутствием. "
        "Здесь — не метафора, а структура: разрыв между тем, что дано, и тем, что удерживается. "
        "Воля к смыслу — это не интенция субъекта, а онтологический сдвиг, который требует пересмотра оснований."
    )

    analyzer = RefractionAnalyzer(sensitivity=0.5)

    print("=== Тест 1: структурные изломы (режим Достоевского) ===")
    markers_d = analyzer.analyze_structural_gap(sample_dostoevsky, window_size=120, step=60)
    print(f"Найдено изломов: {len(markers_d)}")
    for m in markers_d:
        print(f"  - [{m.medium.value}] {m.refraction_type.value}: {m.notes} (meas={m.measurement}, op={m.operator})")

    print("\n=== Тест 2: сетка окон (режим Хайдеггера) ===")
    windows_h = analyzer.prepare_annotation_windows(sample_heidegger, window_size=15, step=10)
    print(f"Сгенерировано окон: {len(windows_h)}")
    for w in windows_h[:5]:
        print(f"  - [{w.start_offset}-{w.end_offset}] {w.fragment_text[:80]}... (оператор: {w.operator or 'нет'})")

    print("\n=== Тест 3: архитектурный зазор (семантическая потеря) ===")
    original = "Воля удерживает разрыв между смыслом и исполнением."
    llm_short = "Воля между смыслом и исполнением."
    gap = analyzer.detect_architecture_gap(
        original, llm_short,
        gap_dimension="semantic_density",
        human_anchor="удержание разрыва как оператор",
        operator="will",
    )
    print(f"Зазор: {gap.measurement:.3f} — {gap.notes}")

    # Книжный отчёт
    analyzer.print_human_summary()


# ==================== ПРИМЕР ПРОМПТА ДЛЯ АГЕНТА ====================
"""
Пример промпта для Qwen 3 (32B) на основе окон из prepare_annotation_windows():

---
Ты — философский аналитик в парадигме GrammaLang.
Проанализируй фрагмент (offset 65-160):
«между присутствием и отсутствием. Здесь — не метафора, а структура: разрыв между…»

Выдели оператор (если есть) и укажи, какой тип онтологического сдвига здесь происходит:
- разрыв
- удержание
- напряжение
- смена модальности

Верни JSON:
{
  "offset": "65-160",
  "operator": "разрыв",
  "shift_type": "онтологический разрыв между данностью и структурой",
  "notes": "пунктуация усиливает временной разрыв"
}
---
"""
