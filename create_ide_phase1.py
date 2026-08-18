# create_ide_phase1.py
import os

os.makedirs('ide/frontend/js', exist_ok=True)

# 1. Редактор с подсветкой синтаксиса
editor_js = '''// Atlas IDE - Редактор с подсветкой синтаксиса

class AtlasEditor {
    constructor() {
        this.operators = [
            { pattern: /~::~/, color: '#e94560', description: 'Апория' },
            { pattern: /::Ethics:::/, color: '#ff6b35', description: 'Этика' },
            { pattern: /:::/, color: '#4d96ff', description: 'Каскад' },
            { pattern: /~>/, color: '#9b59b6', description: 'Рефлексия' },
            { pattern: /~@/, color: '#2ecc71', description: 'Автогенеалогия' },
            { pattern: /~\\$/, color: '#f1c40f', description: 'Самосознание' },
            { pattern: /<<(.+?)>>/, color: '#1abc9c', description: 'Оператор' },
            { pattern: /praxis/, color: '#e74c3c', description: 'Праксис' },
            { pattern: /revolution/, color: '#c0392b', description: 'Революция' },
            { pattern: /synthesis/, color: '#3498db', description: 'Синтез' },
            { pattern: /contradiction/, color: '#e67e22', description: 'Противоречие' },
            { pattern: /reflection/, color: '#9b59b6', description: 'Рефлексия' },
            { pattern: /\\/\\/.*$/, color: '#666', description: 'Комментарий' },
            { pattern: /"(?:[^"\\\\]|\\\\.)*"/, color: '#f39c12', description: 'Строка' },
            { pattern: /\\b(if|else|for|while|fn|let|mut|return|match)\\b/, color: '#e74c3c', description: 'Ключевое слово' },
        ];
        
        this.init();
    }
    
    init() {
        this.editor = document.getElementById('editor');
        this.lineNumbers = document.createElement('div');
        this.lineNumbers.className = 'line-numbers';
        
        // Создаем контейнер
        const container = document.createElement('div');
        container.className = 'editor-container';
        this.editor.parentNode.insertBefore(container, this.editor);
        container.appendChild(this.lineNumbers);
        container.appendChild(this.editor);
        
        this.editor.addEventListener('input', () => this.update());
        this.editor.addEventListener('scroll', () => this.syncScroll());
        
        this.update();
    }
    
    update() {
        const code = this.editor.value;
        const lines = code.split('\\n');
        
        // Обновляем номера строк
        this.lineNumbers.innerHTML = lines.map((_, i) => i + 1).join('<br>');
        
        // Подсветка (в реальном IDE будет через CodeMirror)
        // Здесь пока просто показываем операторы в консоли
        this.detectOperators(code);
    }
    
    syncScroll() {
        this.lineNumbers.scrollTop = this.editor.scrollTop;
    }
    
    detectOperators(code) {
        const found = [];
        
        for (const op of this.operators) {
            if (op.pattern.test(code)) {
                found.push({ name: op.description, color: op.color });
            }
        }
        
        if (found.length > 0) {
            this.showOperators(found);
        }
    }
    
    showOperators(operators) {
        // Показываем найденные операторы в консоли
        const consoleEl = document.getElementById('trace-console');
        const entry = document.createElement('div');
        entry.innerHTML = operators.map(op => 
            `<span style="color:${op.color}">● ${op.name}</span>`
        ).join(' ');
        consoleEl.appendChild(entry);
    }
    
    highlight(code) {
        // Простая подсветка (возвращает HTML)
        let result = code;
        
        for (const op of this.operators) {
            result = result.replace(op.pattern, match => 
                `<span style="color:${op.color}">${match}</span>`
            );
        }
        
        return result;
    }
    
    // Вставка оператора
    insertOperator(operator) {
        const cursorPos = this.editor.selectionStart;
        const textBefore = this.editor.value.substring(0, cursorPos);
        const textAfter = this.editor.value.substring(cursorPos);
        
        this.editor.value = textBefore + operator + textAfter;
        this.editor.selectionStart = this.editor.selectionEnd = cursorPos + operator.length;
        this.editor.focus();
        this.update();
    }
    
    // Получение кода
    getCode() {
        return this.editor.value;
    }
    
    // Установка кода
    setCode(code) {
        this.editor.value = code;
        this.update();
    }
}

// Создание панели операторов
function createOperatorPanel() {
    const panel = document.createElement('div');
    panel.className = 'operator-panel';
    panel.innerHTML = '<h3>Операторы Atlas</h3>';
    
    const operators = [
        { symbol: '~::~', name: 'Апория', desc: 'Удержание противоречия' },
        { symbol: ':::', name: 'Каскад', desc: 'Версионирование' },
        { symbol: '~>', name: 'Рефлексия', desc: 'Самосознание' },
        { symbol: '~@', name: 'Автогенеалогия', desc: 'Своя история' },
        { symbol: '~$', name: 'Самосознание', desc: 'Знать что знаешь' },
        { symbol: '::Ethics:::', name: 'Этика', desc: 'Переопределение' },
        { symbol: '<<praxis>>', name: 'Праксис', desc: 'Практика' },
        { symbol: '<<revolution>>', name: 'Революция', desc: 'Скачок' },
    ];
    
    operators.forEach(op => {
        const btn = document.createElement('button');
        btn.className = 'operator-btn';
        btn.title = op.desc;
        btn.innerHTML = `<span class="op-symbol">${op.symbol}</span><span class="op-name">${op.name}</span>`;
        btn.addEventListener('click', () => atlasEditor.insertOperator(op.symbol + ' '));
        panel.appendChild(btn);
    });
    
    return panel;
}

// Инициализация
const atlasEditor = new AtlasEditor();
const operatorPanel = createOperatorPanel();
document.querySelector('.panel-left').appendChild(operatorPanel);
'''

with open('ide/frontend/js/editor.js', 'w', encoding='utf-8') as f:
    f.write(editor_js)
print("editor.js created")

# 2. Дополнительный CSS
editor_css = '''
.editor-container {
    display: flex;
    flex: 1;
    gap: 0;
    background: #16213e;
    border: 1px solid #0f3460;
    border-radius: 5px;
    overflow: hidden;
}

.line-numbers {
    padding: 10px 5px;
    background: #0f3460;
    color: #666;
    font-family: 'Consolas', monospace;
    font-size: 14px;
    text-align: right;
    user-select: none;
    overflow: hidden;
}

.line-numbers br {
    line-height: 1.5;
}

#editor {
    flex: 1;
    border: none;
    border-radius: 0;
}

.operator-panel {
    padding: 10px;
    background: #16213e;
    border: 1px solid #0f3460;
    border-radius: 5px;
    max-height: 200px;
    overflow-y: auto;
}

.operator-panel h3 {
    margin-bottom: 10px;
    color: #e94560;
    font-size: 0.9em;
}

.operator-btn {
    display: flex;
    align-items: center;
    gap: 10px;
    width: 100%;
    padding: 5px 10px;
    margin-bottom: 5px;
    background: #0f3460;
    color: #e0e0e0;
    border: none;
    border-radius: 3px;
    cursor: pointer;
    font-size: 0.8em;
    transition: all 0.2s;
}

.operator-btn:hover {
    background: #1a4a7a;
}

.op-symbol {
    font-family: 'Consolas', monospace;
    font-weight: bold;
    min-width: 60px;
    text-align: left;
}

.op-name {
    color: #999;
}
'''

with open('ide/frontend/css/editor.css', 'w', encoding='utf-8') as f:
    f.write(editor_css)
print("editor.css created")

# 3. Обновляем index.html
index_html = '''<!DOCTYPE html>
<html lang="ru">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Atlas IDE - Полифонические поля</title>
    <link rel="stylesheet" href="css/style.css">
    <link rel="stylesheet" href="css/editor.css">
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
            </div>
        </header>
        
        <main>
            <div class="panel-left">
                <h2>Редактор</h2>
                <textarea id="editor" placeholder="Введите код Atlas...
                    
Примеры:
свобода ~::~ безопасность
синтез ::: уровень_2
понятие ~> Meta-понятие"></textarea>
                <button id="run-btn">▶ Запустить</button>
            </div>
            
            <div class="panel-center">
                <h2>Полифоническое поле</h2>
                <canvas id="field-canvas"></canvas>
            </div>
            
            <div class="panel-right">
                <h2>Кардиограмма</h2>
                <canvas id="cardio-canvas"></canvas>
                
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
</body>
</html>
'''

with open('ide/frontend/index.html', 'w', encoding='utf-8') as f:
    f.write(index_html)
print("index.html updated")

print("\nAll IDE Phase 1 files created!")
