# create_ide_phase2.py
import os

os.makedirs('ide/frontend/js', exist_ok=True)

# 1. Визуализация полифонического поля
field_js = '''// Atlas IDE - Визуализация полифонического поля

class PolyphonicField {
    constructor(canvasId) {
        this.canvas = document.getElementById(canvasId);
        this.ctx = this.canvas.getContext('2d');
        this.nodes = [];
        this.edges = [];
        this.contradictions = [];
        this.animationFrame = null;
        
        this.resize();
        window.addEventListener('resize', () => this.resize());
        
        this.initDemoNodes();
        this.startAnimation();
    }
    
    resize() {
        this.canvas.width = this.canvas.offsetWidth;
        this.canvas.height = this.canvas.offsetHeight;
    }
    
    initDemoNodes() {
        // Демонстрационные узлы
        this.addNode('Свобода', 0.3, 0.3, '#4CAF50', 20);
        this.addNode('Безопасность', 0.7, 0.3, '#e94560', 20);
        this.addNode('Ответственность', 0.5, 0.5, '#f1c40f', 15);
        this.addNode('Платон', 0.2, 0.7, '#9b59b6', 18);
        this.addNode('Ницше', 0.8, 0.7, '#e67e22', 18);
        
        // Связи
        this.addEdge('Свобода', 'Безопасность', 'contradiction', 0.8);
        this.addEdge('Свобода', 'Ответственность', 'synthesis', 0.6);
        this.addEdge('Безопасность', 'Ответственность', 'synthesis', 0.6);
        this.addEdge('Платон', 'Ницше', 'contradiction', 0.7);
        
        // Противоречия
        this.addContradiction('Свобода', 'Безопасность', 0.8);
        this.addContradiction('Платон', 'Ницше', 0.7);
    }
    
    addNode(label, x, y, color, size) {
        this.nodes.push({
            label,
            x,
            y,
            color,
            size,
            vx: 0,
            vy: 0,
            contradictionLevel: Math.random() * 0.5,
        });
    }
    
    addEdge(from, to, type, weight) {
        this.edges.push({ from, to, type, weight });
    }
    
    addContradiction(nodeA, nodeB, severity) {
        this.contradictions.push({ nodeA, nodeB, severity });
    }
    
    startAnimation() {
        const animate = () => {
            this.update();
            this.draw();
            this.animationFrame = requestAnimationFrame(animate);
        };
        animate();
    }
    
    update() {
        // Простая физика - отталкивание узлов
        for (let i = 0; i < this.nodes.length; i++) {
            for (let j = i + 1; j < this.nodes.length; j++) {
                const dx = this.nodes[j].x - this.nodes[i].x;
                const dy = this.nodes[j].y - this.nodes[i].y;
                const dist = Math.sqrt(dx * dx + dy * dy);
                
                if (dist < 0.15 && dist > 0) {
                    const force = (0.15 - dist) * 0.001;
                    const fx = dx / dist * force;
                    const fy = dy / dist * force;
                    
                    this.nodes[i].vx -= fx;
                    this.nodes[i].vy -= fy;
                    this.nodes[j].vx += fx;
                    this.nodes[j].vy += fy;
                }
            }
        }
        
        // Применяем скорость
        this.nodes.forEach(node => {
            node.x += node.vx;
            node.y += node.vy;
            node.vx *= 0.95;
            node.vy *= 0.95;
            
            // Ограничения
            node.x = Math.max(0.05, Math.min(0.95, node.x));
            node.y = Math.max(0.05, Math.min(0.95, node.y));
        });
    }
    
    draw() {
        const w = this.canvas.width;
        const h = this.canvas.height;
        
        // Фон
        const gradient = this.ctx.createRadialGradient(w/2, h/2, 0, w/2, h/2, w/2);
        gradient.addColorStop(0, '#1a1a2e');
        gradient.addColorStop(1, '#0f3460');
        this.ctx.fillStyle = gradient;
        this.ctx.fillRect(0, 0, w, h);
        
        // Рисуем поле (сетку)
        this.drawFieldGrid(w, h);
        
        // Рисуем связи
        this.drawEdges(w, h);
        
        // Рисуем противоречия (пульсация)
        this.drawContradictions(w, h);
        
        // Рисуем узлы
        this.drawNodes(w, h);
        
        // Рисуем легенду
        this.drawLegend();
    }
    
    drawFieldGrid(w, h) {
        this.ctx.strokeStyle = 'rgba(255,255,255,0.03)';
        this.ctx.lineWidth = 1;
        
        const gridSize = 40;
        for (let x = 0; x < w; x += gridSize) {
            this.ctx.beginPath();
            this.ctx.moveTo(x, 0);
            this.ctx.lineTo(x, h);
            this.ctx.stroke();
        }
        for (let y = 0; y < h; y += gridSize) {
            this.ctx.beginPath();
            this.ctx.moveTo(0, y);
            this.ctx.lineTo(w, y);
            this.ctx.stroke();
        }
    }
    
    drawEdges(w, h) {
        this.edges.forEach(edge => {
            const from = this.nodes.find(n => n.label === edge.from);
            const to = this.nodes.find(n => n.label === edge.to);
            
            if (!from || !to) return;
            
            const x1 = from.x * w;
            const y1 = from.y * h;
            const x2 = to.x * w;
            const y2 = to.y * h;
            
            const color = edge.type === 'contradiction' ? '#e94560' : '#4CAF50';
            
            this.ctx.beginPath();
            this.ctx.moveTo(x1, y1);
            this.ctx.lineTo(x2, y2);
            this.ctx.strokeStyle = color;
            this.ctx.lineWidth = edge.weight * 3;
            this.ctx.globalAlpha = edge.weight * 0.7;
            this.ctx.stroke();
            this.ctx.globalAlpha = 1;
        });
    }
    
    drawContradictions(w, h) {
        const time = Date.now() / 1000;
        
        this.contradictions.forEach(contradiction => {
            const nodeA = this.nodes.find(n => n.label === contradiction.nodeA);
            const nodeB = this.nodes.find(n => n.label === contradiction.nodeB);
            
            if (!nodeA || !nodeB) return;
            
            const x1 = nodeA.x * w;
            const y1 = nodeA.y * h;
            const x2 = nodeB.x * w;
            const y2 = nodeB.y * h;
            
            const mx = (x1 + x2) / 2;
            const my = (y1 + y2) / 2;
            
            // Пульсация противоречия
            const pulse = 1 + Math.sin(time * 3) * 0.3 * contradiction.severity;
            const radius = 20 * contradiction.severity * pulse;
            
            const gradient = this.ctx.createRadialGradient(mx, my, 0, mx, my, radius);
            gradient.addColorStop(0, `rgba(233, 69, 96, ${contradiction.severity * 0.5})`);
            gradient.addColorStop(1, 'rgba(233, 69, 96, 0)');
            
            this.ctx.fillStyle = gradient;
            this.ctx.beginPath();
            this.ctx.arc(mx, my, radius, 0, Math.PI * 2);
            this.ctx.fill();
            
            // Символ апории
            this.ctx.fillStyle = '#e94560';
            this.ctx.font = '12px Arial';
            this.ctx.textAlign = 'center';
            this.ctx.fillText('~::~', mx, my - radius - 5);
        });
    }
    
    drawNodes(w, h) {
        this.nodes.forEach(node => {
            const x = node.x * w;
            const y = node.y * h;
            
            // Свечение
            const glow = this.ctx.createRadialGradient(x, y, 0, x, y, node.size * 2);
            glow.addColorStop(0, node.color + '40');
            glow.addColorStop(1, 'transparent');
            this.ctx.fillStyle = glow;
            this.ctx.beginPath();
            this.ctx.arc(x, y, node.size * 2, 0, Math.PI * 2);
            this.ctx.fill();
            
            // Узел
            this.ctx.beginPath();
            this.ctx.arc(x, y, node.size, 0, Math.PI * 2);
            this.ctx.fillStyle = node.color;
            this.ctx.fill();
            this.ctx.strokeStyle = '#fff';
            this.ctx.lineWidth = 2;
            this.ctx.stroke();
            
            // Метка
            this.ctx.fillStyle = '#fff';
            this.ctx.font = '11px Arial';
            this.ctx.textAlign = 'center';
            this.ctx.fillText(node.label, x, y + node.size + 15);
        });
    }
    
    drawLegend() {
        const w = this.canvas.width;
        const h = this.canvas.height;
        
        this.ctx.fillStyle = 'rgba(22,33,62,0.9)';
        this.ctx.fillRect(w - 160, 10, 150, 80);
        
        this.ctx.fillStyle = '#fff';
        this.ctx.font = '10px Arial';
        this.ctx.textAlign = 'left';
        
        this.ctx.fillStyle = '#4CAF50';
        this.ctx.fillRect(w - 150, 20, 10, 10);
        this.ctx.fillStyle = '#fff';
        this.ctx.fillText('Синтез', w - 135, 30);
        
        this.ctx.fillStyle = '#e94560';
        this.ctx.fillRect(w - 150, 40, 10, 10);
        this.ctx.fillStyle = '#fff';
        this.ctx.fillText('Противоречие', w - 135, 50);
        
        this.ctx.fillStyle = '#f1c40f';
        this.ctx.fillRect(w - 150, 60, 10, 10);
        this.ctx.fillStyle = '#fff';
        this.ctx.fillText('Пульсация', w - 135, 70);
    }
    
    // Публичные методы
    addSynthesisNode(label, fromA, fromB) {
        const a = this.nodes.find(n => n.label === fromA);
        const b = this.nodes.find(n => n.label === fromB);
        
        if (!a || !b) return;
        
        const x = (a.x + b.x) / 2;
        const y = (a.y + b.y) / 2;
        
        this.addNode(label, x, y, '#f1c40f', 15);
        this.addEdge(fromA, label, 'synthesis', 0.6);
        this.addEdge(fromB, label, 'synthesis', 0.6);
    }
    
    updateContradictionLevel(label, level) {
        const node = this.nodes.find(n => n.label === label);
        if (node) {
            node.contradictionLevel = level;
        }
    }
}

// Инициализация
const field = new PolyphonicField('field-canvas');
'''

with open('ide/frontend/js/field.js', 'w', encoding='utf-8') as f:
    f.write(field_js)
print("field.js created")

# 2. Обновляем index.html
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
                <canvas id="field-canvas" style="width:100%;height:calc(100% - 40px);"></canvas>
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
    <script src="js/field.js"></script>
</body>
</html>
'''

with open('ide/frontend/index.html', 'w', encoding='utf-8') as f:
    f.write(index_html)
print("index.html updated")

print("\nAll IDE Phase 2 files created!")
