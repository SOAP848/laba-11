#!/usr/bin/env python3
"""
Тесты для Flask-приложения
"""

import unittest
import json
from app import app

class TestFlaskApp(unittest.TestCase):
    """Класс тестов для приложения"""
    
    def setUp(self):
        """Настройка тестового клиента"""
        self.app = app.test_client()
        self.app.testing = True
    
    def test_home_endpoint(self):
        """Тест главной страницы"""
        response = self.app.get('/')
        self.assertEqual(response.status_code, 200)
        
        data = json.loads(response.data)
        self.assertIn('message', data)
        self.assertIn('hostname', data)
        self.assertEqual(data['version'], '1.0.0')
    
    def test_health_endpoint(self):
        """Тест эндпоинта здоровья"""
        response = self.app.get('/health')
        self.assertEqual(response.status_code, 200)
        
        data = json.loads(response.data)
        self.assertEqual(data['status'], 'healthy')
    
    def test_info_endpoint(self):
        """Тест информационного эндпоинта"""
        response = self.app.get('/info')
        self.assertEqual(response.status_code, 200)
        
        data = json.loads(response.data)
        self.assertIn('python_version', data)
        self.assertIn('platform', data)
        self.assertIn('working_directory', data)
    
    def test_nonexistent_endpoint(self):
        """Тест несуществующего эндпоинта"""
        response = self.app.get('/nonexistent')
        self.assertEqual(response.status_code, 404)

if __name__ == '__main__':
    unittest.main()