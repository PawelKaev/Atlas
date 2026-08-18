// Atlas IDE - WebSocket клиент

class WsClient {
    constructor() {
        this.ws = null;
        this.connected = false;
        this.reconnectInterval = 3000;
        this.listeners = {};
        
        this.connect();
    }
    
    connect() {
        // Пытаемся подключиться к Rust WebSocket серверу
        // В реальном IDE будет: ws://localhost:8080/ws
        // Сейчас используем симуляцию
        
        console.log('WebSocket: attempting connection...');
        
        // Симуляция подключения
        setTimeout(() => {
            this.connected = true;
            console.log('WebSocket: connected');
            this.emit('connected', {});
            this.startSimulation();
        }, 1000);
    }
    
    startSimulation() {
        // Отправляем данные каждые 500мс
        setInterval(() => {
            const data = {
                stability: 0.5 + Math.random() * 0.5,
                contradiction: Math.random() * 0.6,
                awareness: 0.3 + Math.random() * 0.5,
                pulse: 60 + Math.random() * 80,
            };
            
            this.emit('metrics', data);
        }, 500);
    }
    
    on(event, callback) {
        if (!this.listeners[event]) {
            this.listeners[event] = [];
        }
        this.listeners[event].push(callback);
    }
    
    emit(event, data) {
        if (this.listeners[event]) {
            this.listeners[event].forEach(callback => callback(data));
        }
    }
    
    send(data) {
        if (this.connected) {
            console.log('WebSocket send:', data);
        }
    }
}

// Инициализация
const wsClient = new WsClient();

// Интеграция с кардиограммой
wsClient.on('metrics', (data) => {
    if (typeof cardiogram !== 'undefined') {
        cardiogram.setStability(data.stability);
        cardiogram.setContradiction(data.contradiction);
        cardiogram.setAwareness(data.awareness);
        
        // Обновляем статус-бар
        document.getElementById('stability').textContent = 
            'Стабильность: ' + data.stability.toFixed(2);
        document.getElementById('contradiction').textContent = 
            'Противоречие: ' + data.contradiction.toFixed(2);
        document.getElementById('awareness').textContent = 
            'Самосознание: ' + data.awareness.toFixed(2);
        document.getElementById('pulse').textContent = 
            'Пульс: ' + data.pulse.toFixed(1);
    }
});
