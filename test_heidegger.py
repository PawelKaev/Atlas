from integrate_refraction import run_refraction_simulation

text = (
    "Подручное встречает внутримирно. Бытие этого сущего, подручность, стоит потому в каком-то онтологическом отношении к миру и мирности. "
    "Мир во всем подручном всегда уже «вот». Мир опережающе со всем встречным, хотя нетематически, уже раскрыт. "
    "Он может опять же высвечиваться и в известных способах внутримирного обращения. "
    "Мир есть то, из чего подручное подручно. Как мир может дать подручному встретиться? "
    "Предыдущий анализ показал: внутримирно встречающее отпущено в своем бытии для озаботившегося усмотрения, взятия в расчет."
)

result = run_refraction_simulation(
    text,
    source_file="heidegger_par18.txt",
    window_size=100,
    step=50,
    sensitivity=0.5,
    simulation_steps=20
)

ref = result["refraction_report"]
sim = result["simulation"]
machine = result["machine"]

print("=== Хайдеггер, параграф 18: анализ ===\n")
print(f"Изломов: {ref['total_refractions']}")
print(f"Узлов в машине: {len(machine['nodes'])}")
print(f"Связей: {len(machine['edges'])}")
print(f"Доминирующие операторы: {ref['operator_distribution']}")
print(f"HOLD_BREAK: {sim['hold_break_detected']}")
print(f"Итоговая стабильность: {sim['final_metrics']['stability_ratio']:.3f}")
print(f"Индекс противоречий: {sim['final_metrics']['contradiction_index']:.3f}")
print()

print("Динамика stability_ratio:")
for i, snap in enumerate(sim["history"]):
    sr = snap["metrics"]["stability_ratio"]
    bar = "█" * int(sr * 20)
    print(f"  Такт {i:2d}: {sr:.3f} {bar}")
print()

print("Фрагменты с операторами:")
for m in ref["refractions"][:5]:
    op = m["operator"] or "-"
    txt = m["fragment_text"][:70]
    print(f"  [{op:12}] ({m['medium']:20}) meas={m['measurement']:.3f} | {txt}...")
