import json, sys
from pathlib import Path
from integrate_refraction import run_refraction_simulation

def load_profile(name):
    path = Path("philosophers") / f"{name}.json"
    if not path.exists():
        raise FileNotFoundError(f"Profile not found: {path}")
    return json.loads(path.read_text(encoding="utf-8"))

def analyze_as_philosopher(name, text, source_file="input.txt", steps=20):
    profile = load_profile(name)
    result = run_refraction_simulation(text, source_file=source_file,
        window_size=profile.get("window_size", 200), step=profile.get("step", 100),
        sensitivity=profile.get("sensitivity", 0.5), simulation_steps=steps)
    result["philosopher"] = name
    result["profile"] = profile
    return result

def print_report(result):
    ref = result["refraction_report"]
    sim = result["simulation"]
    p = result["profile"]
    print()
    print("=" * 50)
    print("PHILOSOPHER:", result["philosopher"])
    print("Mode:", p.get("mode", "standard"))
    print("=" * 50)
    print("Refractions:", ref["total_refractions"])
    print("Nodes:", len(result["machine"]["nodes"]))
    print("HOLD_BREAK:", sim["hold_break_detected"])
    print("Stability:", round(sim["final_metrics"]["stability_ratio"], 3))
    print()
    for i, snap in enumerate(sim["history"]):
        sr = snap["metrics"]["stability_ratio"]
        bar = "#" * int(sr * 20)
        print(f"  {i:2d}: {sr:.3f} {bar}")
    print()
    top = sorted(ref["refractions"], key=lambda x: x["measurement"], reverse=True)[:5]
    for m in top:
        op = m["operator"] or "-"
        txt = (m["fragment_text"] or "")[:60]
        print(f"  [{op:10}] {m['measurement']:.3f} | {txt}")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Philosophers:", [f.stem for f in Path("philosophers").glob("*.json")])
        sys.exit(1)
    philosopher = sys.argv[1]
    if len(sys.argv) > 2:
        text = Path(sys.argv[2]).read_text(encoding="utf-8")
    else:
        text = "Being is. Non-being is not."
    result = analyze_as_philosopher(philosopher, text)
    print_report(result)
