// Atlas IDE - Редактор с подсветкой синтаксиса

class AtlasEditor {
    constructor() {
        this.operators = [
            { pattern: /~::~/, color: '#e94560', description: 'Апория' },
            { pattern: /::Ethics:::/, color: '#ff6b35', description: 'Этика' },
            { pattern: /:::/, color: '#4d96ff', description: 'Каскад' },
            { pattern: /~>/, color: '#9b59b6', description: 'Рефлексия' },
            { pattern: /~@/, color: '#2ecc71', description: 'Автогенеалогия' },
            { pattern: /~\$/, color: '#f1c40f', description: 'Самосознание' },
            { pattern: /<<(.+?)>>/, color: '#1abc9c', description: 'Оператор' },
            { pattern: /praxis/, color: '#e74c3c', description: 'Праксис' },
            { pattern: /revolution/, color: '#c0392b', description: 'Революция' },
            { pattern: /synthesis/, color: '#3498db', description: 'Синтез' },
            { pattern: /contradiction/, color: '#e67e22', description: 'Противоречие' },
            { pattern: /reflection/, color: '#9b59b6', description: 'Рефлексия' },
            { pattern: /\/\/.*$/, color: '#666', description: 'Комментарий' },
            { pattern: /"(?:[^"\\]|\\.)*"/, color: '#f39c12', description: 'Строка' },
            { pattern: /\b(if|else|for|while|fn|let|mut|return|match)\b/, color: '#e74c3c', description: 'Ключевое слово' },
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
        const lines = code.split('\n');
        
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
