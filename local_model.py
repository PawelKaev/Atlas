import os
import sys
from llama_cpp import Llama


class LocalModel:
    def __init__(self, model_path, n_ctx=2048, n_threads=4, n_gpu_layers=0, mmproj_path=None):
        self.model_path = model_path
        self.n_ctx = n_ctx
        self.n_threads = n_threads
        self.n_gpu_layers = n_gpu_layers
        self.mmproj_path = mmproj_path
        
        if not os.path.exists(model_path):
            raise FileNotFoundError(f"Модель не найдена: {model_path}")
        
        print(f"Загрузка модели: {os.path.basename(model_path)}")
        print(f"  Контекст: {n_ctx}, Потоки: {n_threads}, GPU слои: {n_gpu_layers}")
        
        llm_kwargs = {
            'model_path': model_path,
            'n_ctx': n_ctx,
            'n_threads': n_threads,
            'n_gpu_layers': n_gpu_layers,
            'verbose': False,
        }
        
        if mmproj_path and os.path.exists(mmproj_path):
            print(f"  Мультимодальная модель, mmproj: {os.path.basename(mmproj_path)}")
            llm_kwargs['mmproj'] = mmproj_path
        
        try:
            self.llm = Llama(**llm_kwargs)
            print(f"  Модель загружена успешно")
        except Exception as e:
            print(f"  Ошибка загрузки модели: {e}")
            raise
    
    def chat(self, messages, system_prompt=None, temperature=0.7, max_tokens=2000, top_p=0.9, top_k=40):
        try:
            if system_prompt:
                prompt = f"System: {system_prompt}\n\n"
            else:
                prompt = ""
            
            for msg in messages:
                role = msg.get('role', 'user')
                content = msg.get('content', '')
                
                if isinstance(content, list):
                    text_parts = []
                    for part in content:
                        if isinstance(part, dict):
                            if part.get('type') == 'text':
                                text_parts.append(part['text'])
                            elif part.get('type') == 'image_url':
                                text_parts.append('[image]')
                        else:
                            text_parts.append(str(part))
                    content = ' '.join(text_parts)
                
                if role == 'user':
                    prompt += f"User: {content}\n"
                elif role == 'assistant':
                    prompt += f"Assistant: {content}\n"
            
            prompt += "Assistant: "
            
            response = self.llm(
                prompt,
                max_tokens=max_tokens,
                temperature=temperature,
                top_p=top_p,
                top_k=top_k,
                stop=["User:", "\nUser:", "\n\nUser:"],
                echo=False
            )
            
            if isinstance(response, dict):
                return response.get('choices', [{}])[0].get('text', '').strip()
            else:
                return str(response).strip()
                
        except Exception as e:
            print(f"Ошибка при генерации ответа: {e}")
            return f"[Ошибка: {e}]"
    
    def chat_with_image(self, messages, temperature=0.7, max_tokens=2000):
        return "[Анализ изображений отключён. Используйте вкладку 'Книги' для OCR.]"
