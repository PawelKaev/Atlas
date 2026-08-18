// Atlas IDE - Кардиограмма (пульс машины)

class Cardiogram {
    constructor(canvasId) {
        this.canvas = document.getElementById(canvasId);
        this.ctx = this.canvas.getContext('2d');
        this.data = [];
        this.maxPoints = 200;
        this.time = 0;
        this.pulseRate = 60;
        this.stability = 1.0;
        this.contradiction = 0.0;
        this.awareness = 0.0;
        
        this.resize();
        window.addEventListener('resize', () => this.resize());
        
        this.start();
    }
    
    resize() {
        this.canvas.width = this.canvas.offsetWidth;
        this.canvas.height = this.canvas.offsetHeight;
    }
    
    start() {
        setInterval(() => {
            this.update();
            this.draw();
        }, 50);
    }
    
    update() {
        this.time += 0.05;
        
        // Имитация пульса
        this.pulseRate = 60 + this.contradiction * 100;
        
        // Генерация ECG-подобного сигнала
        const ecg = this.generateECG(this.time, this.pulseRate);
        
        this.data.push({
            ecg,
            stability: this.stability,
            contradiction: this.contradiction,
            awareness: this.awareness,
            timestamp: Date.now(),
        });
        
        if (this.data.length > this.maxPoints) {
            this.data.shift();
        }
    }
    
    generateECG(time, rate) {
        // ECG-подобная волна
        const cycle = time * rate / 60;
        const phase = cycle % 1;
        
        let value = 0;
        
        // P-волна (предсердие)
        if (phase < 0.1) {
            value = Math.sin(phase / 0.1 * Math.PI) * 0.15;
        }
        // QRS-комплекс (желудочки)
        else if (phase < 0.2) {
            const qrs = (phase - 0.1) / 0.1;
            if (qrs < 0.2) value = -0.3;
            else if (qrs < 0.35) value = 1.0;
            else if (qrs < 0.45) value = -0.5;
            else value = 0.2;
        }
        // T-волна (восстановление)
        else if (phase < 0.4) {
            value = Math.sin((phase - 0.2) / 0.2 * Math.PI) * 0.25;
        }
        // Базовая линия
        else {
            value = 0;
        }
        
        // Добавляем шум от противоречий
        value += (Math.random() - 0.5) * this.contradiction * 0.3;
        
        // Амплитуда зависит от стабильности
        value *= (0.5 + this.stability * 0.5);
        
        return value;
    }
    
    draw() {
        const w = this.canvas.width;
        const h = this.canvas.height;
        
        // Фон
        this.ctx.fillStyle = '#0a0a1a';
        this.ctx.fillRect(0, 0, w, h);
        
        // Сетка
        this.drawGrid(w, h);
        
        // ECG линия
        this.drawECG(w, h);
        
        // Линия стабильности
        this.drawStability(w, h);
        
        // Линия противоречия
        this.drawContradiction(w, h);
        
        // Информация
        this.drawInfo(w, h);
    }
    
    drawGrid(w, h) {
        this.ctx.strokeStyle = 'rgba(255,255,255,0.05)';
        this.ctx.lineWidth = 1;
        
        for (let x = 0; x < w; x += 50) {
            this.ctx.beginPath();
            this.ctx.moveTo(x, 0);
            this.ctx.lineTo(x, h);
            this.ctx.stroke();
        }
        for (let y = 0; y < h; y += 25) {
            this.ctx.beginPath();
            this.ctx.moveTo(0, y);
            this.ctx.lineTo(w, y);
            this.ctx.stroke();
        }
    }
    
    drawECG(w, h) {
        const midY = h / 2;
        const amplitude = h / 3;
        
        this.ctx.beginPath();
        this.ctx.strokeStyle = '#00ff88';
        this.ctx.lineWidth = 2;
        this.ctx.shadowColor = '#00ff88';
        this.ctx.shadowBlur = 10;
        
        this.data.forEach((point, i) => {
            const x = (i / this.maxPoints) * w;
            const y = midY - point.ecg * amplitude;
            
            if (i === 0) {
                this.ctx.moveTo(x, y);
            } else {
                this.ctx.lineTo(x, y);
            }
        });
        
        this.ctx.stroke();
        this.ctx.shadowBlur = 0;
    }
    
    drawStability(w, h) {
        const midY = h / 2;
        
        this.ctx.beginPath();
        this.ctx.strokeStyle = 'rgba(76, 175, 80, 0.3)';
        this.ctx.lineWidth = 1;
        
        this.data.forEach((point, i) => {
            const x = (i / this.maxPoints) * w;
            const y = h - point.stability * h * 0.3;
            
            if (i === 0) {
                this.ctx.moveTo(x, y);
            } else {
                this.ctx.lineTo(x, y);
            }
        });
        
        this.ctx.stroke();
    }
    
    drawContradiction(w, h) {
        this.ctx.beginPath();
        this.ctx.strokeStyle = 'rgba(233, 69, 96, 0.3)';
        this.ctx.lineWidth = 1;
        
        this.data.forEach((point, i) => {
            const x = (i / this.maxPoints) * w;
            const y = point.contradiction * h * 0.3;
            
            if (i === 0) {
                this.ctx.moveTo(x, y);
            } else {
                this.ctx.lineTo(x, y);
            }
        });
        
        this.ctx.stroke();
    }
    
    drawInfo(w, h) {
        this.ctx.fillStyle = 'rgba(10,10,26,0.8)';
        this.ctx.fillRect(10, 10, 130, 70);
        
        this.ctx.font = '10px Arial';
        this.ctx.textAlign = 'left';
        
        this.ctx.fillStyle = '#00ff88';
        this.ctx.fillText(`Пульс: ${this.pulseRate.toFixed(0)} BPM`, 20, 25);
        
        this.ctx.fillStyle = '#4CAF50';
        this.ctx.fillText(`Стабильность: ${this.stability.toFixed(2)}`, 20, 40);
        
        this.ctx.fillStyle = '#e94560';
        this.ctx.fillText(`Противоречие: ${this.contradiction.toFixed(2)}`, 20, 55);
        
        this.ctx.fillStyle = '#f1c40f';
        this.ctx.fillText(`Самосознание: ${this.awareness.toFixed(2)}`, 20, 70);
    }
    
    // Публичные методы
    setStability(value) {
        this.stability = Math.max(0, Math.min(1, value));
    }
    
    setContradiction(value) {
        this.contradiction = Math.max(0, Math.min(1, value));
    }
    
    setAwareness(value) {
        this.awareness = Math.max(0, Math.min(1, value));
    }
    
    // Симуляция синтеза (всплеск)
    synthesisSpike() {
        this.stability = Math.min(1, this.stability + 0.15);
        this.contradiction = Math.max(0, this.contradiction - 0.1);
        this.awareness = Math.min(1, this.awareness + 0.05);
    }
    
    // Симуляция противоречия (спад)
    contradictionDip() {
        this.stability = Math.max(0, this.stability - 0.1);
        this.contradiction = Math.min(1, this.contradiction + 0.15);
    }
}

// Инициализация
const cardiogram = new Cardiogram('cardio-canvas');
