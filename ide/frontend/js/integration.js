// Atlas IDE - Полная интеграция

class AtlasIntegration {
    constructor() {
        this.field = null;
        this.cardiogram = null;
        this.editor = null;
        this.wsClient = null;
        this.machine = {
            nodes: [],
            contradictions: [],
            syntheses: [],
            stability: 1.0,
            contradiction: 0.0,
            awareness: 0.0,
        };
        
        this.init();
    }
    
    init() {
        // Получаем компоненты
        this.field = window.field;
        this.cardiogram = window.cardiogram;
        this.editor = window.atlasEditor;
        this.wsClient = window.wsClient;
        
        // Подписываемся на события
        this.setupEventListeners();
        
        // Запускаем автоматическую симуляцию
        this.startAutoSimulation();
        
        console.log('Atlas Integration initialized');
    }
    
    setupEventListeners() {
        // WebSocket события
        if (this.wsClient) {
            this.wsClient.on('metrics', (data) => {
                this.updateMachineState(data);
            });
            
            this.wsClient.on('connected', () => {
                this.log('WebSocket подключен');
            });
        }
        
        // Кнопка запуска
        const runBtn = document.getElementById('run-btn');
        if (runBtn) {
            runBtn.addEventListener('click', () => {
                this.runCode();
            });
        }
        
        // Кнопка синтеза
        const synthesisBtn = document.getElementById('synthesis-btn');
        if (synthesisBtn) {
            synthesisBtn.addEventListener('click', () => {
                this.performSynthesis();
            });
        }
        
        // Кнопка противоречия
        const contradictionBtn = document.getElementById('contradiction-btn');
        if (contradictionBtn) {
            contradictionBtn.addEventListener('click', () => {
                this.performContradiction();
            });
        }
    }
    
    runCode() {
        const code = this.editor ? this.editor.getCode() : '';
        
        if (!code.trim()) {
            this.log('Пустой код');
            return;
        }
        
        this.log('Выполнение кода...');
        
        // Определяем операторы в коде
        if (code.includes('~::~')) {
            this.performContradiction();
            this.log('Обнаружена апория ~::~');
        }
        
        if (code.includes(':::')) {
            this.log('Каскад ::: обнаружен');
            this.performSynthesis();
        }
        
        if (code.includes('~>')) {
            this.log('Рефлексия ~> обнаружена');
            this.machine.awareness += 0.1;
        }
        
        this.updateUI();
    }
    
    performSynthesis() {
        const nodeA = this.machine.nodes[0] || 'Свобода';
        const nodeB = this.machine.nodes[1] || 'Безопасность';
        const synthesisName = 'Синтез_' + (this.machine.syntheses.length + 1);
        
        this.machine.syntheses.push(synthesisName);
        this.machine.stability = Math.min(1, this.machine.stability + 0.1);
        this.machine.contradiction = Math.max(0, this.machine.contradiction - 0.1);
        this.machine.awareness = Math.min(1, this.machine.awareness + 0.05);
        
        // Визуализация
        if (this.field) {
            this.field.addSynthesisNode(synthesisName, nodeA, nodeB);
        }
        
        // Кардиограмма
        if (this.cardiogram) {
            this.cardiogram.synthesisSpike();
        }
        
        // WebSocket
        if (this.wsClient) {
            this.wsClient.send({ type: 'synthesis', name: synthesisName });
        }
        
        this.log(`Синтез: ${nodeA} + ${nodeB} = ${synthesisName}`);
        this.updateUI();
    }
    
    performContradiction() {
        const nodeA = this.machine.nodes[0] || 'Свобода';
        const nodeB = this.machine.nodes[1] || 'Безопасность';
        const severity = 0.5 + Math.random() * 0.4;
        
        this.machine.contradictions.push({ nodeA, nodeB, severity });
        this.machine.stability = Math.max(0, this.machine.stability - 0.1);
        this.machine.contradiction = Math.min(1, this.machine.contradiction + 0.1);
        
        // Кардиограмма
        if (this.cardiogram) {
            this.cardiogram.contradictionDip();
        }
        
        // WebSocket
        if (this.wsClient) {
            this.wsClient.send({ type: 'contradiction', severity });
        }
        
        this.log(`Противоречие: ${nodeA} ~::~ ${nodeB} (${severity.toFixed(2)})`);
        this.updateUI();
    }
    
    updateMachineState(data) {
        this.machine.stability = data.stability;
        this.machine.contradiction = data.contradiction;
        this.machine.awareness = data.awareness;
    }
    
    updateUI() {
        // Обновляем статус-бар
        document.getElementById('stability').textContent = 
            'Стабильность: ' + this.machine.stability.toFixed(2);
        document.getElementById('contradiction').textContent = 
            'Противоречие: ' + this.machine.contradiction.toFixed(2);
        document.getElementById('awareness').textContent = 
            'Самосознание: ' + this.machine.awareness.toFixed(2);
        document.getElementById('pulse').textContent = 
            'Пульс: ' + (60 + this.machine.contradiction * 100).toFixed(1);
        
        // Обновляем рефлексивный вид
        this.updateReflexiveView();
    }
    
    updateReflexiveView() {
        const view = document.getElementById('reflexive-view');
        if (!view) return;
        
        let html = '<div class="reflexive-level">';
        html += `<div>Синтезов: ${this.machine.syntheses.length}</div>`;
        html += `<div>Противоречий: ${this.machine.contradictions.length}</div>`;
        html += `<div>Самосознание: ${(this.machine.awareness * 100).toFixed(0)}%</div>`;
        html += '</div>';
        
        // Спираль рефлексии
        html += '<svg width="100%" height="150" viewBox="0 0 200 150">';
        const spirals = Math.floor(this.machine.awareness * 5);
        for (let i = 0; i < spirals; i++) {
            const cx = 100;
            const cy = 75;
            const r = 10 + i * 15;
            const color = ['#4CAF50', '#3498db', '#9b59b6', '#e94560', '#f1c40f'][i % 5];
            html += `<circle cx="${cx}" cy="${cy}" r="${r}" fill="none" stroke="${color}" stroke-width="2" opacity="${0.3 + i * 0.15}"/>`;
        }
        html += '</svg>';
        
        view.innerHTML = html;
    }
    
    startAutoSimulation() {
        // Автоматическая симуляция каждые 3 секунды
        setInterval(() => {
            if (Math.random() > 0.5) {
                this.performSynthesis();
            } else {
                this.performContradiction();
            }
        }, 5000);
    }
    
    log(message) {
        const consoleEl = document.getElementById('trace-console');
        if (!consoleEl) return;
        
        const entry = document.createElement('div');
        entry.textContent = new Date().toLocaleTimeString() + ' - ' + message;
        consoleEl.appendChild(entry);
        consoleEl.scrollTop = consoleEl.scrollHeight;
        
        // Ограничиваем количество записей
        while (consoleEl.children.length > 50) {
            consoleEl.removeChild(consoleEl.firstChild);
        }
    }
}

// Инициализация после загрузки всех компонентов
window.addEventListener('load', () => {
    setTimeout(() => {
        window.atlasIntegration = new AtlasIntegration();
    }, 1000);
});
