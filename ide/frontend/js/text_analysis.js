// Atlas IDE - Загрузка текста и анализ с автоматическим синтезом

class TextAnalyzer {
    constructor() {
        this.analysisResults = [];
        this.syntheses = [];
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
        
        this.synthesisStrategies = {
            'hegelian': {
                'formula': 'A + B → Синтез (снятие)',
                'color': '#4CAF50',
            },
            'plotinian': {
                'formula': 'Единое → Множественность',
                'color': '#9b59b6',
            },
            'pragmatic': {
                'formula': 'Поиск наилучшего объяснения',
                'color': '#f1c40f',
            },
        };
        
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
            <button id="synthesize-btn" class="synthesize-btn" style="display:none;">⚡ Авто-синтез</button>
            <div id="analysis-results"></div>
            <div id="synthesis-results"></div>
        `;
        
        const leftPanel = document.querySelector('.panel-left');
        leftPanel.appendChild(panel);
        
        document.getElementById('analyze-btn').addEventListener('click', () => {
            this.analyze();
        });
        
        document.getElementById('synthesize-btn').addEventListener('click', () => {
            this.autoSynthesize();
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
        this.analysisResults = results;
        this.showResults(results);
        this.updateField(results);
        
        // Показываем кнопку синтеза если есть диалектические пары
        const pairs = results.filter(r => r.type === 'dialectical_pair');
        const synthesizeBtn = document.getElementById('synthesize-btn');
        if (pairs.length > 0) {
            synthesizeBtn.style.display = 'block';
            synthesizeBtn.textContent = `⚡ Авто-синтез (${pairs.length} пар)`;
        } else {
            synthesizeBtn.style.display = 'none';
        }
        
        this.log(`Анализ: ${results.length} результатов, ${pairs.length} диалектических пар`);
    }
    
    autoSynthesize() {
        const pairs = this.analysisResults.filter(r => r.type === 'dialectical_pair');
        
        if (pairs.length === 0) {
            this.log('Нет диалектических пар для синтеза');
            return;
        }
        
        this.syntheses = [];
        
        pairs.forEach((pair, index) => {
            const match = pair.detail.match(/(.+?) ↔ (.+)/);
            if (match) {
                const a = match[1].trim();
                const b = match[2].trim();
                
                // Выбор стратегии
                const strategy = this.selectStrategy(index);
                
                // Создание синтеза
                const synthesis = this.createSynthesis(a, b, strategy);
                this.syntheses.push(synthesis);
                
                // Добавляем на поле
                this.addSynthesisToField(synthesis);
            }
        });
        
        this.showSyntheses();
        this.log(`Авто-синтез: ${this.syntheses.length} новых понятий`);
    }
    
    selectStrategy(index) {
        const strategies = ['hegelian', 'plotinian', 'pragmatic'];
        return strategies[index % strategies.length];
    }
    
    createSynthesis(a, b, strategy) {
        const name = `${a}_${b}_Synthese`;
        
        const descriptions = {
            'hegelian': `Снятие противоречия между ${a} и ${b}`,
            'plotinian': `Эманация из единства ${a} и ${b}`,
            'pragmatic': `Практический синтез ${a} и ${b}`,
        };
        
        return {
            name: name,
            a: a,
            b: b,
            strategy: strategy,
            description: descriptions[strategy],
            color: this.synthesisStrategies[strategy].color,
            confidence: 0.7 + Math.random() * 0.2,
        };
    }
    
    addSynthesisToField(synthesis) {
        if (window.field) {
            const a = synthesis.a;
            const b = synthesis.b;
            
            // Находим или создаем узлы
            let nodeA = window.field.nodes.find(n => n.label === a);
            let nodeB = window.field.nodes.find(n => n.label === b);
            
            if (!nodeA) {
                window.field.addNode(a, Math.random(), Math.random(), '#9b59b6', 16, 'philosopher');
                nodeA = window.field.nodes[window.field.nodes.length - 1];
            }
            if (!nodeB) {
                window.field.addNode(b, Math.random(), Math.random(), '#9b59b6', 16, 'philosopher');
                nodeB = window.field.nodes[window.field.nodes.length - 1];
            }
            
            // Добавляем синтез
            const sx = (nodeA.x + nodeB.x) / 2;
            const sy = (nodeA.y + nodeB.y) / 2;
            window.field.addNode(synthesis.name, sx, sy, synthesis.color, 18, 'synthesis');
            
            // Связи синтеза
            window.field.addEdge(a, synthesis.name, 'synthesis', 0.7);
            window.field.addEdge(b, synthesis.name, 'synthesis', 0.7);
        }
    }
    
    showSyntheses() {
        const container = document.getElementById('synthesis-results');
        
        let html = '<h4>Синтезы:</h4>';
        
        this.syntheses.forEach(s => {
            const formula = this.synthesisStrategies[s.strategy].formula;
            html += `<div class="synthesis-item" style="border-left: 3px solid ${s.color}">
                <div class="synthesis-name">${s.name}</div>
                <div class="synthesis-detail">${s.a} + ${s.b} → ${s.name}</div>
                <div class="synthesis-strategy">${formula} (${(s.confidence * 100).toFixed(0)}%)</div>
            </div>`;
        });
        
        container.innerHTML = html;
    }
    
    performDeepAnalysis(text) {
        const results = [];
        
        // Операторы Atlas
        const contradictions = text.match(/[\w\s]+~::~[\w\s]+/g);
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
        
        // Диалектические пары
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
        
        // Философские понятия
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
        
        // Противопоставления
        if (text.includes('weder') && text.includes('noch')) {
            results.push({
                type: 'opposition',
                label: 'Противопоставление',
                detail: 'weder...noch (ни...ни)',
                color: '#e67e22',
            });
        }
        
        // Статистика
        const words = text.split(/\s+/).filter(w => w.length > 0);
        const sentences = text.split(/[.!?]\s/).filter(s => s.length > 0);
        results.push({
            type: 'stats',
            label: 'Статистика',
            detail: `${words.length} слов, ${text.length} символов, ${sentences.length} предложений`,
            color: '#f1c40f',
        });
        
        // Язык
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
