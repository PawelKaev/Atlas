// Atlas IDE - Main JavaScript

class AtlasIDE {
    constructor() {
        this.state = {
            stability: 1.0,
            contradiction: 0.0,
            awareness: 0.0,
            pulse: 60.0,
        };
        
        this.nodes = [];
        this.cardioData = [];
        
        this.init();
    }
    
    init() {
        this.fieldCanvas = document.getElementById('field-canvas');
        this.cardioCanvas = document.getElementById('cardio-canvas');
        this.editor = document.getElementById('editor');
        this.traceConsole = document.getElementById('trace-console');
        
        document.getElementById('run-btn').addEventListener('click', () => this.run());
        
        this.startCardiogram();
        this.log('Atlas IDE initialized');
    }
    
    run() {
        const code = this.editor.value;
        this.log('Running: ' + code.substring(0, 50) + '...');
        
        // Имитация выполнения
        this.updateState({
            stability: Math.random() * 0.5 + 0.5,
            contradiction: Math.random() * 0.5,
            awareness: Math.random() * 0.7,
        });
    }
    
    updateState(newState) {
        Object.assign(this.state, newState);
        
        document.getElementById('stability').textContent = 
            'Стабильность: ' + this.state.stability.toFixed(2);
        document.getElementById('contradiction').textContent = 
            'Противоречие: ' + this.state.contradiction.toFixed(2);
        document.getElementById('awareness').textContent = 
            'Самосознание: ' + this.state.awareness.toFixed(2);
        document.getElementById('pulse').textContent = 
            'Пульс: ' + this.state.pulse.toFixed(1);
        
        this.drawField();
    }
    
    startCardiogram() {
        setInterval(() => {
            this.state.pulse = 60 + this.state.contradiction * 120;
            this.cardioData.push({
                stability: this.state.stability,
                contradiction: this.state.contradiction,
            });
            
            if (this.cardioData.length > 100) {
                this.cardioData.shift();
            }
            
            this.drawCardiogram();
        }, 500);
    }
    
    drawField() {
        const ctx = this.fieldCanvas.getContext('2d');
        const w = this.fieldCanvas.width;
        const h = this.fieldCanvas.height;
        
        ctx.clearRect(0, 0, w, h);
        
        // Рисуем фоновое поле
        const gradient = ctx.createRadialGradient(w/2, h/2, 0, w/2, h/2, w/2);
        gradient.addColorStop(0, '#1a1a2e');
        gradient.addColorStop(1, '#0f3460');
        ctx.fillStyle = gradient;
        ctx.fillRect(0, 0, w, h);
        
        // Рисуем узлы
        this.nodes.forEach((node, i) => {
            const x = w/2 + Math.cos(i * 2 * Math.PI / this.nodes.length) * w/4;
            const y = h/2 + Math.sin(i * 2 * Math.PI / this.nodes.length) * h/4;
            
            ctx.beginPath();
            ctx.arc(x, y, node.size || 10, 0, 2 * Math.PI);
            ctx.fillStyle = node.color || '#4CAF50';
            ctx.fill();
            
            ctx.fillStyle = '#fff';
            ctx.font = '12px Arial';
            ctx.textAlign = 'center';
            ctx.fillText(node.label || '', x, y - 15);
        });
    }
    
    drawCardiogram() {
        const ctx = this.cardioCanvas.getContext('2d');
        const w = this.cardioCanvas.width;
        const h = this.cardioCanvas.height;
        
        ctx.clearRect(0, 0, w, h);
        ctx.fillStyle = '#16213e';
        ctx.fillRect(0, 0, w, h);
        
        // Рисуем линию стабильности
        ctx.beginPath();
        ctx.strokeStyle = '#4CAF50';
        ctx.lineWidth = 2;
        
        this.cardioData.forEach((data, i) => {
            const x = (i / 100) * w;
            const y = h - data.stability * h;
            
            if (i === 0) {
                ctx.moveTo(x, y);
            } else {
                ctx.lineTo(x, y);
            }
        });
        
        ctx.stroke();
        
        // Рисуем линию противоречия
        ctx.beginPath();
        ctx.strokeStyle = '#e94560';
        ctx.lineWidth = 1;
        
        this.cardioData.forEach((data, i) => {
            const x = (i / 100) * w;
            const y = h - data.contradiction * h;
            
            if (i === 0) {
                ctx.moveTo(x, y);
            } else {
                ctx.lineTo(x, y);
            }
        });
        
        ctx.stroke();
    }
    
    log(message) {
        const entry = document.createElement('div');
        entry.textContent = new Date().toLocaleTimeString() + ' - ' + message;
        this.traceConsole.appendChild(entry);
        this.traceConsole.scrollTop = this.traceConsole.scrollHeight;
    }
}

// Запуск IDE
const ide = new AtlasIDE();
