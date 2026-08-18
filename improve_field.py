# improve_field.py
import os

os.makedirs('ide/frontend/js', exist_ok=True)

# Улучшенная визуализация
field_js = '''// Atlas IDE - Улучшенная визуализация полифонического поля

class PolyphonicField {
    constructor(canvasId) {
        this.canvas = document.getElementById(canvasId);
        this.ctx = this.canvas.getContext('2d');
        this.nodes = [];
        this.edges = [];
        this.contradictions = [];
        this.particles = [];
        this.animationFrame = null;
        this.mousePos = { x: -1, y: -1 };
        this.selectedNode = null;
        this.time = 0;
        
        this.resize();
        window.addEventListener('resize', () => this.resize());
        
        this.canvas.addEventListener('mousemove', (e) => {
            const rect = this.canvas.getBoundingClientRect();
            this.mousePos.x = e.clientX - rect.left;
            this.mousePos.y = e.clientY - rect.top;
        });
        
        this.canvas.addEventListener('mouseleave', () => {
            this.mousePos.x = -1;
            this.mousePos.y = -1;
        });
        
        this.canvas.addEventListener('click', (e) => {
            const rect = this.canvas.getBoundingClientRect();
            const x = e.clientX - rect.left;
            const y = e.clientY - rect.top;
            this.selectNode(x, y);
        });
        
        this.initDemoNodes();
        this.startAnimation();
    }
    
    resize() {
        this.canvas.width = this.canvas.offsetWidth;
        this.canvas.height = this.canvas.offsetHeight;
    }
    
    initDemoNodes() {
        // Демонстрационные узлы с разными цветами и размерами
        this.addNode('Свобода', 0.3, 0.3, '#4CAF50', 22, 'concept');
        this.addNode('Безопасность', 0.7, 0.3, '#e94560', 22, 'concept');
        this.addNode('Ответственность', 0.5, 0.5, '#f1c40f', 18, 'synthesis');
        this.addNode('Платон', 0.2, 0.7, '#9b59b6', 20, 'philosopher');
        this.addNode('Ницше', 0.8, 0.7, '#e67e22', 20, 'philosopher');
        this.addNode('Рефлексия', 0.5, 0.2, '#3498db', 16, 'reflection');
        this.addNode('Самосознание', 0.5, 0.85, '#f1c40f', 14, 'awareness');
        
        // Связи
        this.addEdge('Свобода', 'Безопасность', 'contradiction', 0.9);
        this.addEdge('Свобода', 'Ответственность', 'synthesis', 0.7);
        this.addEdge('Безопасность', 'Ответственность', 'synthesis', 0.7);
        this.addEdge('Платон', 'Ницше', 'contradiction', 0.8);
        this.addEdge('Ответственность', 'Рефлексия', 'reflection', 0.5);
        this.addEdge('Рефлексия', 'Самосознание', 'awareness', 0.4);
        this.addEdge('Платон', 'Свобода', 'influence', 0.3);
        this.addEdge('Ницше', 'Безопасность', 'influence', 0.3);
        
        // Противоречия
        this.addContradiction('Свобода', 'Безопасность', 0.9);
        this.addContradiction('Платон', 'Ницше', 0.8);
        this.addContradiction('Свобода', 'Ницше', 0.5);
        this.addContradiction('Безопасность', 'Платон', 0.4);
    }
    
    addNode(label, x, y, color, size, type) {
        this.nodes.push({
            label,
            x,
            y,
            color,
            size,
            type,
            vx: 0,
            vy: 0,
            contradictionLevel: 0,
            pulse: Math.random() * Math.PI * 2,
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
            this.time += 0.01;
            this.update();
            this.draw();
            this.animationFrame = requestAnimationFrame(animate);
        };
        animate();
    }
    
    update() {
        // Притяжение связанных узлов
        this.edges.forEach(edge => {
            const from = this.nodes.find(n => n.label === edge.from);
            const to = this.nodes.find(n => n.label === edge.to);
            if (!from || !to) return;
            
            const dx = to.x - from.x;
            const dy = to.y - from.y;
            const dist = Math.sqrt(dx * dx + dy * dy);
            
            if (dist > 0.2 && dist < 0.5) {
                const force = 0.0005 * edge.weight;
                from.vx += dx * force;
                from.vy += dy * force;
                to.vx -= dx * force;
                to.vy -= dy * force;
            }
        });
        
        // Отталкивание всех узлов
        for (let i = 0; i < this.nodes.length; i++) {
            for (let j = i + 1; j < this.nodes.length; j++) {
                const dx = this.nodes[j].x - this.nodes[i].x;
                const dy = this.nodes[j].y - this.nodes[i].y;
                const dist = Math.sqrt(dx * dx + dy * dy);
                
                if (dist < 0.15 && dist > 0) {
                    const force = (0.15 - dist) * 0.002;
                    const fx = dx / dist * force;
                    const fy = dy / dist * force;
                    
                    this.nodes[i].vx -= fx;
                    this.nodes[i].vy -= fy;
                    this.nodes[j].vx += fx;
                    this.nodes[j].vy += fy;
                }
            }
        }
        
        // Притяжение к центру (слабое)
        this.nodes.forEach(node => {
            const dx = 0.5 - node.x;
            const dy = 0.5 - node.y;
            node.vx += dx * 0.0001;
            node.vy += dy * 0.0001;
        });
        
        // Отталкивание от мыши
        if (this.mousePos.x > 0) {
            const mx = this.mousePos.x / this.canvas.width;
            const my = this.mousePos.y / this.canvas.height;
            
            this.nodes.forEach(node => {
                const dx = node.x - mx;
                const dy = node.y - my;
                const dist = Math.sqrt(dx * dx + dy * dy);
                
                if (dist < 0.1 && dist > 0) {
                    const force = (0.1 - dist) * 0.05;
                    node.vx += dx / dist * force;
                    node.vy += dy / dist * force;
                }
            });
        }
        
        // Применяем скорость
        this.nodes.forEach(node => {
            node.x += node.vx;
            node.y += node.vy;
            node.vx *= 0.92;
            node.vy *= 0.92;
            
            node.x = Math.max(0.05, Math.min(0.95, node.x));
            node.y = Math.max(0.05, Math.min(0.95, node.y));
        });
        
        // Обновляем частицы
        this.updateParticles();
    }
    
    updateParticles() {
        // Удаляем старые частицы
        this.particles = this.particles.filter(p => p.life > 0);
        
        // Обновляем оставшиеся
        this.particles.forEach(p => {
            p.x += p.vx;
            p.y += p.vy;
            p.life -= 0.02;
            p.size *= 0.98;
        });
        
        // Создаем новые частицы от противоречий
        this.contradictions.forEach(c => {
            const nodeA = this.nodes.find(n => n.label === c.nodeA);
            const nodeB = this.nodes.find(n => n.label === c.nodeB);
            if (!nodeA || !nodeB) return;
            
            if (Math.random() < c.severity * 0.3) {
                const mx = ((nodeA.x + nodeB.x) / 2) * this.canvas.width;
                const my = ((nodeA.y + nodeB.y) / 2) * this.canvas.height;
                
                this.particles.push({
                    x: mx,
                    y: my,
                    vx: (Math.random() - 0.5) * 2,
                    vy: (Math.random() - 0.5) * 2,
                    life: Math.random() * 0.5 + 0.3,
                    size: Math.random() * 4 + 2,
                    color: '#e94560',
                });
            }
        });
    }
    
    draw() {
        const w = this.canvas.width;
        const h = this.canvas.height;
        
        // Фон с градиентом
        const gradient = this.ctx.createRadialGradient(w/2, h/2, 0, w/2, h/2, w/1.5);
        gradient.addColorStop(0, '#1a1a2e');
        gradient.addColorStop(0.5, '#16213e');
        gradient.addColorStop(1, '#0a0a1a');
        this.ctx.fillStyle = gradient;
        this.ctx.fillRect(0, 0, w, h);
        
        // Рисуем сетку
        this.drawFieldGrid(w, h);
        
        // Рисуем связи
        this.drawEdges(w, h);
        
        // Рисуем противоречия
        this.drawContradictions(w, h);
        
        // Рисуем частицы
        this.drawParticles();
        
        // Рисуем узлы
        this.drawNodes(w, h);
        
        // Рисуем подсказку
        this.drawTooltip(w, h);
        
        // Рисуем легенду
        this.drawLegend();
    }
    
    drawFieldGrid(w, h) {
        this.ctx.strokeStyle = 'rgba(255,255,255,0.02)';
        this.ctx.lineWidth = 1;
        
        const gridSize = 30;
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
            
            // Цвет связи
            let color;
            switch (edge.type) {
                case 'contradiction': color = '#e94560'; break;
                case 'synthesis': color = '#4CAF50'; break;
                case 'reflection': color = '#9b59b6'; break;
                case 'awareness': color = '#f1c40f'; break;
                case 'influence': color = '#555'; break;
                default: color = '#888';
            }
            
            // Градиентная линия
            const gradient = this.ctx.createLinearGradient(x1, y1, x2, y2);
            gradient.addColorStop(0, from.color);
            gradient.addColorStop(0.5, color);
            gradient.addColorStop(1, to.color);
            
            this.ctx.beginPath();
            this.ctx.moveTo(x1, y1);
            this.ctx.lineTo(x2, y2);
            this.ctx.strokeStyle = gradient;
            this.ctx.lineWidth = edge.weight * 2;
            this.ctx.globalAlpha = edge.weight * 0.5;
            this.ctx.stroke();
            this.ctx.globalAlpha = 1;
        });
    }
    
    drawContradictions(w, h) {
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
            
            // Пульсация
            const pulse = 1 + Math.sin(this.time * 3) * 0.3 * contradiction.severity;
            const radius = 25 * contradiction.severity * pulse;
            
            // Внешнее свечение
            const glow = this.ctx.createRadialGradient(mx, my, 0, mx, my, radius * 1.5);
            glow.addColorStop(0, `rgba(233, 69, 96, ${contradiction.severity * 0.3})`);
            glow.addColorStop(1, 'rgba(233, 69, 96, 0)');
            this.ctx.fillStyle = glow;
            this.ctx.beginPath();
            this.ctx.arc(mx, my, radius * 1.5, 0, Math.PI * 2);
            this.ctx.fill();
            
            // Внутреннее ядро
            const core = this.ctx.createRadialGradient(mx, my, 0, mx, my, radius);
            core.addColorStop(0, `rgba(233, 69, 96, ${contradiction.severity * 0.7})`);
            core.addColorStop(1, 'rgba(233, 69, 96, 0)');
            this.ctx.fillStyle = core;
            this.ctx.beginPath();
            this.ctx.arc(mx, my, radius, 0, Math.PI * 2);
            this.ctx.fill();
            
            // Символ апории
            this.ctx.fillStyle = '#fff';
            this.ctx.font = 'bold 10px Arial';
            this.ctx.textAlign = 'center';
            this.ctx.fillText('~::~', mx, my - radius - 3);
        });
    }
    
    drawParticles() {
        this.particles.forEach(p => {
            this.ctx.beginPath();
            this.ctx.arc(p.x, p.y, p.size, 0, Math.PI * 2);
            this.ctx.fillStyle = p.color;
            this.ctx.globalAlpha = p.life;
            this.ctx.fill();
            this.ctx.globalAlpha = 1;
        });
    }
    
    drawNodes(w, h) {
        this.nodes.forEach(node => {
            const x = node.x * w;
            const y = node.y * h;
            const pulse = 1 + Math.sin(this.time * 2 + node.pulse) * 0.05;
            const size = node.size * pulse;
            
            // Свечение
            const glow = this.ctx.createRadialGradient(x, y, 0, x, y, size * 2.5);
            glow.addColorStop(0, node.color + '60');
            glow.addColorStop(1, 'transparent');
            this.ctx.fillStyle = glow;
            this.ctx.beginPath();
            this.ctx.arc(x, y, size * 2.5, 0, Math.PI * 2);
            this.ctx.fill();
            
            // Основной круг
            this.ctx.beginPath();
            this.ctx.arc(x, y, size, 0, Math.PI * 2);
            this.ctx.fillStyle = node.color;
            this.ctx.fill();
            
            // Обводка
            this.ctx.strokeStyle = '#fff';
            this.ctx.lineWidth = this.selectedNode === node ? 3 : 1.5;
            this.ctx.stroke();
            
            // Иконка типа
            this.drawNodeIcon(node, x, y, size);
            
            // Метка
            this.ctx.fillStyle = '#fff';
            this.ctx.font = '11px Arial';
            this.ctx.textAlign = 'center';
            this.ctx.fillText(node.label, x, y + size + 18);
        });
    }
    
    drawNodeIcon(node, x, y, size) {
        this.ctx.fillStyle = '#fff';
        this.ctx.font = `${size}px Arial`;
        this.ctx.textAlign = 'center';
        this.ctx.textBaseline = 'middle';
        
        let icon = '●';
        switch (node.type) {
            case 'synthesis': icon = '⊕'; break;
            case 'philosopher': icon = 'Φ'; break;
            case 'reflection': icon = '↻'; break;
            case 'awareness': icon = '☀'; break;
            case 'concept': icon = '○'; break;
        }
        
        this.ctx.fillText(icon, x, y);
        this.ctx.textBaseline = 'alphabetic';
    }
    
    drawTooltip(w, h) {
        // Найти узел под мышью
        const mx = this.mousePos.x / w;
        const my = this.mousePos.y / h;
        
        let hoveredNode = null;
        for (const node of this.nodes) {
            const dx = node.x - mx;
            const dy = node.y - my;
            const dist = Math.sqrt(dx * dx + dy * dy);
            
            if (dist < node.size / w) {
                hoveredNode = node;
                break;
            }
        }
        
        if (hoveredNode && this.mousePos.x > 0) {
            const x = this.mousePos.x;
            const y = this.mousePos.y;
            
            this.ctx.fillStyle = 'rgba(22,33,62,0.95)';
            this.ctx.fillRect(x + 15, y - 10, 120, 40);
            this.ctx.strokeStyle = hoveredNode.color;
            this.ctx.lineWidth = 1;
            this.ctx.strokeRect(x + 15, y - 10, 120, 40);
            
            this.ctx.fillStyle = '#fff';
            this.ctx.font = 'bold 11px Arial';
            this.ctx.textAlign = 'left';
            this.ctx.fillText(hoveredNode.label, x + 20, y + 5);
            
            this.ctx.font = '10px Arial';
            this.ctx.fillStyle = '#999';
            this.ctx.fillText(hoveredNode.type, x + 20, y + 20);
        }
    }
    
    drawLegend() {
        const w = this.canvas.width;
        
        this.ctx.fillStyle = 'rgba(22,33,62,0.9)';
        this.ctx.fillRect(w - 170, 10, 160, 110);
        
        this.ctx.font = '10px Arial';
        this.ctx.textAlign = 'left';
        
        const items = [
            { color: '#4CAF50', label: 'Синтез', x: w - 160, y: 25 },
            { color: '#e94560', label: 'Противоречие', x: w - 160, y: 45 },
            { color: '#9b59b6', label: 'Рефлексия', x: w - 160, y: 65 },
            { color: '#f1c40f', label: 'Самосознание', x: w - 160, y: 85 },
            { color: '#555', label: 'Влияние', x: w - 160, y: 105 },
        ];
        
        items.forEach(item => {
            this.ctx.fillStyle = item.color;
            this.ctx.fillRect(item.x, item.y - 8, 12, 12);
            this.ctx.fillStyle = '#fff';
            this.ctx.fillText(item.label, item.x + 18, item.y + 2);
        });
    }
    
    selectNode(x, y) {
        const w = this.canvas.width;
        const h = this.canvas.height;
        const mx = x / w;
        const my = y / h;
        
        this.selectedNode = null;
        for (const node of this.nodes) {
            const dx = node.x - mx;
            const dy = node.y - my;
            const dist = Math.sqrt(dx * dx + dy * dy);
            
            if (dist < node.size / w * 2) {
                this.selectedNode = node;
                
                // Показать информацию в консоли
                const consoleEl = document.getElementById('trace-console');
                const entry = document.createElement('div');
                entry.textContent = `Выбран узел: ${node.label} (${node.type})`;
                consoleEl.appendChild(entry);
                
                break;
            }
        }
    }
    
    // Публичные методы
    addSynthesisNode(label, fromA, fromB) {
        const a = this.nodes.find(n => n.label === fromA);
        const b = this.nodes.find(n => n.label === fromB);
        
        if (!a || !b) return;
        
        const x = (a.x + b.x) / 2;
        const y = (a.y + b.y) / 2;
        
        this.addNode(label, x, y, '#f1c40f', 15, 'synthesis');
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
print("field.js improved")

