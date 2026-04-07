#!/usr/bin/env python3
"""
Простое веб-приложение на Flask для демонстрации Docker
"""

from flask import Flask, jsonify
import os
import socket

app = Flask(__name__)

@app.route('/')
def home():
    """Главная страница с приветствием"""
    return jsonify({
        'message': 'Добро пожаловать в Python-приложение в Docker!',
        'version': '1.0.0',
        'hostname': socket.gethostname(),
        'environment': os.getenv('ENV', 'development')
    })

@app.route('/health')
def health():
    """Эндпоинт для проверки здоровья приложения"""
    return jsonify({'status': 'healthy'}), 200

@app.route('/info')
def info():
    """Информация о системе"""
    return jsonify({
        'python_version': os.sys.version,
        'platform': os.sys.platform,
        'working_directory': os.getcwd(),
        'user': os.getenv('USER', 'unknown')
    })

if __name__ == '__main__':
    port = int(os.getenv('PORT', 5000))
    host = os.getenv('HOST', '0.0.0.0')
    debug = os.getenv('DEBUG', 'False').lower() == 'true'
    
    print(f"Запуск приложения на {host}:{port} (debug={debug})")
    app.run(host=host, port=port, debug=debug)