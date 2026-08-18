# create_ide_text_analysis.py
import os

os.makedirs('ide/frontend/js', exist_ok=True)
os.makedirs('ide/frontend/css', exist_ok=True)

# 1. Модуль анализа текста
text_analysis_js = '''// Atlas IDE - Загрузка текста и анализ

class TextAnalyzer {
    constructor() {
        this.analysisResults = [];
        this.init();
    }
    
    init() {
        this.createPanel();
    }
    
    createPanel() {
        // Создаем панель для текста
        const panel = document.createElement('div');
        panel.className = 'text-analysis-panel';
        panel.innerHTML = `
            <h3>Анализ текста</h3>
            <textarea id="text-input" placeholder="Вставьте текст для анализа..."></textarea>
            <button id="analyze-btn" class="analyze-btn">🔍 Анализировать</button>
            <div id="analysis-results"></div>
        `;
        
        // Вставляем в левую панель после редактора
        const leftPanel = document.querySelector('.panel-left');
        leftPanel.appendChild(panel);
        
        // Обработчики
        document.getElementById('analyze-btn').addEventListener('click', () => {
            this.analyze();
        });
        
        // Загрузка файла
        const fileInput = document.createElement('input');
        fileInput.type = 'file';
        fileInput.accept = '.txt,.at,.md';
        fileInput.style.display = 'none';
        fileInput.id = 'file-input';
        panel.appendChild(fileInput);
        
        const loadBtn = document.createElement('button');
        loadBtn.className = 'load-btn';
        loadBtn.textContent = '📂 Загрузить файл';
        loadBtn.addEventListener('click', () => {
            fileInput.click();
        });
        panel.querySelector('button').after(loadBtn);
        
        fileInput.addEventListener('change', (e) => {
            const file = e.target.files[0];
            if (file) {
                const reader = new FileReader();
                reader.onload = (event) => {
                    document.getElementById('text-input').value = event.target.result;
                    console.log('Файл загружен:', file.name);
                };
                reader.readAsText(file);
            }
        });
    }
    
    analyze() {
        const text = document.getElementById('text-input').value;
        if (!text.trim()) {
            this.showResults([{ type: 'error', message: 'Пустой текст' }]);
            return;
        }
        
        const results = this.performAnalysis(text);
        this.analysisResults.push(...results);
        this.showResults(results);
        
        // Обновляем поле
        this.updateField(results);
        
        // Логируем
        this.log(`Анализ текста: ${text.length} символов, ${results.length} результатов`);
    }
    
    performAnalysis(text) {
        const results = [];
        
        // 1. Поиск противоречий (~::~)
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
        
        // 2. Поиск каскадов (:::)
        const cascades = text.match(/[\\w\\s]+:::[\\w\\s]+/g);
        if (cascades) {
            cascades.forEach(c => {
                results.push({
                    type: 'cascade',
                    label: 'Каскад',
                    detail: c.trim(),
                    color: '#4d96ff',
                });
            });
        }
        
        // 3. Поиск рефлексий (~>)
        const reflections = text.match(/[\\w\\s]+~>[\\w\\s]+/g);
        if (reflections) {
            reflections.forEach(r => {
                results.push({
                    type: 'reflection',
                    label: 'Рефлексия',
                    detail: r.trim(),
                    color: '#9b59b6',
                });
            });
        }
        
        // 4. Поиск ключевых слов
        const keywords = ['свобода', 'безопасность', 'ответственность', 'синтез', 'противоречие'];
        keywords.forEach(kw => {
            if (text.toLowerCase().includes(kw.toLowerCase())) {
                results.push({
                    type: 'keyword',
                    label: 'Ключевое слово',
                    detail: kw,
                    color: '#4CAF50',
                });
            }
        });
        
        // 5. Статистика
        const words = text.split(/\\s+/).filter(w => w.length > 0);
        results.push({
            type: 'stats',
            label: 'Статистика',
            detail: `${words.length} слов, ${text.length} символов, ${text.split(/[.!?]/).length - 1} предложений`,
            color: '#f1c40f',
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
        // Обновляем полифоническое поле на основе результатов
        const contradictions = results.filter(r => r.type === 'contradiction');
        
        contradictions.forEach(c => {
            if (window.field) {
                const match = c.detail.match(/(.+?) ~::~ (.+)/);
                if (match) {
                    const a = match[1].trim();
                    const b = match[2].trim();
                    
                    // Добавляем узлы если их нет
                    if (!window.field.nodes.find(n => n.label === a)) {
                        window.field.addNode(a, Math.random(), Math.random(), '#e94560', 15, 'concept');
                    }
                    if (!window.field.nodes.find(n => n.label === b)) {
                        window.field.addNode(b, Math.random(), Math.random(), '#e94560', 15, 'concept');
                    }
                    
                    // Добавляем противоречие
                    window.field.addContradiction(a, b, 0.8);
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

// Инициализация
const textAnalyzer = new TextAnalyzer();
'''

with open('ide/frontend/js/text_analysis.js', 'w', encoding='utf-8') as f:
    f.write(text_analysis_js)
print("text_analysis.js created")

# 2. CSS
text_analysis_css = '''
.text-analysis-panel {
    padding: 10px;
    background: #16213e;
    border: 1px solid #0f3460;
    border-radius: 5px;
    margin-top: 10px;
}

.text-analysis-panel h3 {
    margin-bottom: 10px;
    color: #e94560;
    font-size: 0.9em;
}

#text-input {
    width: 100%;
    height: 80px;
    background: #0f3460;
    color: #e0e0e0;
    border: 1px solid #0f3460;
    border-radius: 3px;
    padding: 8px;
    font-family: 'Consolas', monospace;
    font-size: 12px;
    resize: vertical;
    margin-bottom: 5px;
}

.analyze-btn, .load-btn {
    width: 100%;
    padding: 8px;
    margin-bottom: 5px;
    background: #4d96ff;
    color: white;
    border: none;
    border-radius: 3px;
    cursor: pointer;
    font-size: 0.85em;
    transition: background 0.2s;
}

.analyze-btn:hover {
    background: #6babff;
}

.load-btn {
    background: #555;
}

.load-btn:hover {
    background: #777;
}

#analysis-results {
    max-height: 150px;
    overflow-y: auto;
    margin-top: 5px;
}

.result-item {
    padding: 5px 8px;
    margin-bottom: 3px;
    background: #0f3460;
    border-radius: 3px;
    display: flex;
    flex-direction: column;
    gap: 2px;
}

.result-label {
    font-size: 0.75em;
    font-weight: bold;
    color: #999;
}

.result-detail {
    font-size: 0.8em;
    color: #e0e0e0;
}
'''

with open('ide/frontend/css/text_analysis.css', 'w', encoding='utf-8') as f:
    f.write(text_analysis_css)
print("text_analysis.css created")

# 3. Обновляем index.html
index_html = '''<!DOCTYPE html>
<html lang="ru">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Atlas IDE - Полифонические поля</title>
    <link rel="stylesheet" href="css/style.css">
    <link rel="stylesheet" href="css/editor.css">
    <link rel="stylesheet" href="css/integration.css">
    <link rel="stylesheet" href="css/text_analysis.css">
</head>
<body>
    <div class="container">
        <header>
            <h1>Atlas IDE</h1>
            <div class="status-bar">
                <span id="stability">Стабильность: 1.00</span>
                <span id="contradiction">Противоречие: 0.00</span>
                <span id="awareness">Самосознание: 0.00</span>
                <span id="pulse">Пульс: 60.0</span>
                <span id="ws-status" style="color:#f1c40f;">WS: подключение...</span>
            </div>
        </header>
        
        <main>
            <div class="panel-left">
                <h2>Редактор</h2>
                <textarea id="editor" placeholder="Введите код Atlas..."></textarea>
                <button id="run-btn">▶ Запустить</button>
                <button id="synthesis-btn" style="background:#4CAF50;margin-top:5px;">⊕ Синтез</button>
                <button id="contradiction-btn" style="background:#e94560;margin-top:5px;">~::~ Противоречие</button>
            </div>
            
            <div class="panel-center">
                <h2>Полифоническое поле</h2>
                <canvas id="field-canvas" style="width:100%;height:calc(100% - 40px);"></canvas>
            </div>
            
            <div class="panel-right">
                <h2>Кардиограмма</h2>
                <canvas id="cardio-canvas" style="width:100%;height:200px;"></canvas>
                
                <h2>Рефлексивный каскад</h2>
                <div id="reflexive-view"></div>
            </div>
        </main>
        
        <footer>
            <div id="trace-console"></div>
        </footer>
    </div>
    
    <script src="js/main.js"></script>
    <script src="js/editor.js"></script>
    <script src="js/field.js"></script>
    <script src="js/cardio.js"></script>
    <script src="js/ws_client.js"></script>
    <script src="js/integration.js"></script>
    <script src="js/text_analysis.js"></script>
    <script>
        wsClient.on('connected', () => {
            document.getElementById('ws-status').textContent = 'WS: подключен';
            document.getElementById('ws-status').style.color = '#4CAF50';
        });
    </script>
</body>
</html>
'''

with open('ide/frontend/index.html', 'w', encoding='utf-8') as f:
    f.write(index_html)
print("index.html updated")

print("\nText analysis files created!")
