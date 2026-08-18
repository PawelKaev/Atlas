# update_text_analysis.py
import os

# Обновляем text_analysis.js с глубоким анализом
text_analysis_js = '''// Atlas IDE - Загрузка текста и анализ (расширенный)

class TextAnalyzer {
    constructor() {
        this.analysisResults = [];
        this.philosophicalConcepts = {
            'denken': 'мышление',
            'Denken': 'Мышление',
            'geschichtliche': 'историческое',
            'geschichtlicheDenken': 'историческое мышление',
            'Erklären': 'объяснение',
            'Besinnung': 'осмысление',
            'Bedenken': 'сомнения',
            'Vorhaben': 'намерение',
            'Seite': 'сторона',
            'lange': 'долго',
            'hinaus': 'вперёд',
            'vermögen': 'мочь',
            'vielleicht': 'возможно',
            'unterscheiden': 'различать',
            'behalten': 'сохранять',
            'versuchte': 'попытка',
            'weder': 'ни',
            'noch': 'ни',
        };
        
        this.dialecticalPairs = [
            ['Erklären', 'Denken'],
            ['philosophisch', 'historisch'],
            ['systematisch', 'geschichtlich'],
            ['Bedenken', 'Vorhaben'],
            ['auflösen', 'klären'],
        ];
        
        this.init();
    }
    
    init() {
        this.createPanel();
    }
    
    createPanel() {
        const panel = document.createElement('div');
        panel.className = 'text-analysis-panel';
        panel.innerHTML = `
            <h3>Анализ текста</h3>
            <textarea id="text-input" placeholder="Вставьте текст для анализа..."></textarea>
            <button id="analyze-btn" class="analyze-btn">🔍 Анализировать</button>
            <div id="analysis-results"></div>
        `;
        
        const leftPanel = document.querySelector('.panel-left');
        leftPanel.appendChild(panel);
        
        document.getElementById('analyze-btn').addEventListener('click', () => {
            this.analyze();
        });
        
        // Загрузка файла
        const fileInput = document.createElement('input');
        fileInput.type = 'file';
        fileInput.accept = '.txt,.at,.md';
        fileInput.style.display = 'none';
        panel.appendChild(fileInput);
        
        const loadBtn = document.createElement('button');
        loadBtn.className = 'load-btn';
        loadBtn.textContent = '📂 Загрузить файл';
        loadBtn.addEventListener('click', () => fileInput.click());
        panel.querySelector('.analyze-btn').after(loadBtn);
        
        fileInput.addEventListener('change', (e) => {
            const file = e.target.files[0];
            if (file) {
                const reader = new FileReader();
                reader.onload = (event) => {
                    document.getElementById('text-input').value = event.target.result;
                };
                reader.readAsText(file);
            }
        });
    }
    
    analyze() {
        const text = document.getElementById('text-input').value;
        if (!text.trim()) {
            this.showResults([{ type: 'error', label: 'Ошибка', detail: 'Пустой текст', color: '#e94560' }]);
            return;
        }
        
        const results = this.performDeepAnalysis(text);
        this.analysisResults.push(...results);
        this.showResults(results);
        this.updateField(results);
        this.log(`Анализ: ${text.length} символов, ${results.length} результатов`);
    }
    
    performDeepAnalysis(text) {
        const results = [];
        
        // 1. Операторы Atlas
        const contradictions = text.match(/[\\w\\s]+~::~[\\w\\s]+/g);
        if (contradictions) {
            contradictions.forEach(c => {
                const parts = c.split('~::~');
                results.push({
                    type: 'contradiction',
                    label: 'Противоречие',
                    detail: `${parts[0].trim()} ~::~ ${parts[1].trim()}`,
                    color: '#e94560',
                });
            });
        }
        
        // 2. Диалектические пары (немецкий)
        this.dialecticalPairs.forEach(([a, b]) => {
            if (text.includes(a) && text.includes(b)) {
                results.push({
                    type: 'dialectical_pair',
                    label: 'Диалектическая пара',
                    detail: `${a} ↔ ${b}`,
                    color: '#e94560',
                });
            }
        });
        
        // 3. Философские понятия (немецкий)
        Object.entries(this.philosophicalConcepts).forEach(([de, ru]) => {
            if (text.includes(de)) {
                results.push({
                    type: 'concept',
                    label: 'Понятие',
                    detail: `${de} → ${ru}`,
                    color: '#9b59b6',
                });
            }
        });
        
        // 4. Противопоставления (weder...noch, nicht...sondern)
        if (text.includes('weder') && text.includes('noch')) {
            results.push({
                type: 'opposition',
                label: 'Противопоставление',
                detail: 'weder...noch (ни...ни)',
                color: '#e67e22',
            });
        }
        
        if (text.includes('nicht') && text.includes('sondern')) {
            results.push({
                type: 'opposition',
                label: 'Противопоставление',
                detail: 'nicht...sondern (не...а)',
                color: '#e67e22',
            });
        }
        
        // 5. Русские понятия
        const russianConcepts = ['мышление', 'историческое', 'объяснение', 'осмысление', 'сомнение'];
        russianConcepts.forEach(rc => {
            if (text.toLowerCase().includes(rc.toLowerCase())) {
                results.push({
                    type: 'russian_concept',
                    label: 'Русское понятие',
                    detail: rc,
                    color: '#4CAF50',
                });
            }
        });
        
        // 6. Статистика
        const words = text.split(/\\s+/).filter(w => w.length > 0);
        const sentences = text.split(/[.!?]\\s/).filter(s => s.length > 0);
        results.push({
            type: 'stats',
            label: 'Статистика',
            detail: `${words.length} слов, ${text.length} символов, ${sentences.length} предложений`,
            color: '#f1c40f',
        });
        
        // 7. Язык текста
        const isGerman = /[äöüßÄÖÜ]/.test(text);
        const isRussian = /[а-яА-Я]/.test(text);
        const language = isGerman && isRussian ? 'смешанный' : isGerman ? 'немецкий' : isRussian ? 'русский' : 'неопределён';
        results.push({
            type: 'language',
            label: 'Язык',
            detail: language,
            color: '#3498db',
        });
        
        return results;
    }
    
    showResults(results) {
        const container = document.getElementById('analysis-results');
        let html = '';
        
        results.forEach(r => {
            html += `<div class="result-item" style="border-left: 3px solid ${r.color}">
                <span class="result-label">${r.label}</span>
                <span class="result-detail">${r.detail}</span>
            </div>`;
        });
        
        container.innerHTML = html;
    }
    
    updateField(results) {
        // Добавляем узлы на поле для диалектических пар
        const pairs = results.filter(r => r.type === 'dialectical_pair');
        
        pairs.forEach(p => {
            if (window.field) {
                const match = p.detail.match(/(.+?) ↔ (.+)/);
                if (match) {
                    const a = match[1].trim();
                    const b = match[2].trim();
                    
                    if (!window.field.nodes.find(n => n.label === a)) {
                        window.field.addNode(a, Math.random(), Math.random(), '#9b59b6', 16, 'philosopher');
                    }
                    if (!window.field.nodes.find(n => n.label === b)) {
                        window.field.addNode(b, Math.random(), Math.random(), '#9b59b6', 16, 'philosopher');
                    }
                    
                    window.field.addContradiction(a, b, 0.7);
                }
            }
        });
    }
    
    log(message) {
        const consoleEl = document.getElementById('trace-console');
        if (consoleEl) {
            const entry = document.createElement('div');
            entry.textContent = new Date().toLocaleTimeString() + ' - ' + message;
            consoleEl.appendChild(entry);
            consoleEl.scrollTop = consoleEl.scrollHeight;
        }
    }
}

const textAnalyzer = new TextAnalyzer();
'''

with open('ide/frontend/js/text_analysis.js', 'w', encoding='utf-8') as f:
    f.write(text_analysis_js)
print("text_analysis.js updated with deep analysis")
