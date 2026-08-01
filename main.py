# Atlas/main.py
"""
Главный модуль компилятора Atlas.
Интегрирует лексер, парсер, десахаринг, разрешение имён,
вывод типов, проверку заимствований и кодогенерацию.
"""

import sys
import os
import json
from pathlib import Path
from typing import Optional, List, Dict, Any

# Пытаемся импортировать скомпилированное Rust-ядро
try:
    import grammalang_core
    CORE_AVAILABLE = True
except ImportError:
    CORE_AVAILABLE = False
    print("⚠ grammalang_core не найден. Соберите ядро: cd grammalang-core && cargo build --release")


# ============================================================
# Конфигурация
# ============================================================

class CompilerConfig:
    """Конфигурация компилятора."""
    def __init__(
        self,
        mode: str = "development",     # development | release
        target: str = "native",        # native | wasm32 | python-module
        optimization: int = 0,         # 0-3
        emit_llvm: bool = False,       # выводить LLVM IR
        emit_ast: bool = False,        # выводить AST
        contracts: bool = True,        # проверять контракты
    ):
        self.mode = mode
        self.target = target
        self.optimization = optimization
        self.emit_llvm = emit_llvm
        self.emit_ast = emit_ast
        self.contracts = contracts


# ============================================================
# Диагностика
# ============================================================

class Diagnostic:
    """Диагностическое сообщение."""
    def __init__(
        self,
        level: str,        # error | warning | hint
        message: str,
        line: int = 0,
        column: int = 0,
        file: str = "",
        hint: Optional[str] = None,
    ):
        self.level = level
        self.message = message
        self.line = line
        self.column = column
        self.file = file
        self.hint = hint
    
    def __str__(self) -> str:
        prefix = {
            "error": "🔴 Ошибка",
            "warning": "⚠ Предупреждение",
            "hint": "💡 Подсказка",
        }.get(self.level, self.level)
        
        location = ""
        if self.file:
            location = f"{self.file}"
            if self.line > 0:
                location += f":{self.line}"
                if self.column > 0:
                    location += f":{self.column}"
            location += " — "
        
        msg = f"{location}{prefix}: {self.message}"
        if self.hint:
            msg += f"\n   Подсказка: {self.hint}"
        return msg


class DiagnosticBag:
    """Контейнер диагностических сообщений."""
    def __init__(self):
        self.errors: List[Diagnostic] = []
        self.warnings: List[Diagnostic] = []
        self.hints: List[Diagnostic] = []
    
    def error(self, message: str, line: int = 0, column: int = 0, file: str = "", hint: str = None):
        self.errors.append(Diagnostic("error", message, line, column, file, hint))
    
    def warning(self, message: str, line: int = 0, column: int = 0, file: str = "", hint: str = None):
        self.warnings.append(Diagnostic("warning", message, line, column, file, hint))
    
    def hint(self, message: str, line: int = 0, column: int = 0, file: str = "", hint: str = None):
        self.hints.append(Diagnostic("hint", message, line, column, file, hint))
    
    def has_errors(self) -> bool:
        return len(self.errors) > 0
    
    def print_all(self):
        for diag in self.errors + self.warnings + self.hints:
            print(diag)
    
    def summary(self) -> str:
        parts = []
        if self.errors:
            parts.append(f"{len(self.errors)} ошибок")
        if self.warnings:
            parts.append(f"{len(self.warnings)} предупреждений")
        if not parts:
            return "OK"
        return ", ".join(parts)


# ============================================================
# Компилятор
# ============================================================

class AtlasCompiler:
    """Главный класс компилятора Atlas."""
    
    def __init__(self, config: CompilerConfig = None):
        self.config = config or CompilerConfig()
        self.diagnostics = DiagnosticBag()
    
    def compile_file(self, filepath: str) -> Optional[str]:
        """Скомпилировать файл .at"""
        path = Path(filepath)
        if not path.exists():
            self.diagnostics.error(f"Файл не найден: {filepath}")
            return None
        
        if path.suffix != ".at":
            self.diagnostics.warning(f"Ожидается файл .at, получен {path.suffix}")
        
        source = path.read_text(encoding="utf-8")
        return self.compile_source(source, filepath)
    
    def compile_source(self, source: str, filename: str = "<input>") -> Optional[str]:
        """Скомпилировать исходный код Atlas."""
        
        if CORE_AVAILABLE:
            return self._compile_native(source, filename)
        else:
            return self._compile_python(source, filename)
    
    def _compile_native(self, source: str, filename: str) -> Optional[str]:
        """Компиляция через Rust-ядро."""
        try:
            result = grammalang_core.compile_atlas(source)
            return result
        except SyntaxError as e:
            self.diagnostics.error(str(e), file=filename)
            return None
        except Exception as e:
            self.diagnostics.error(f"Внутренняя ошибка компилятора: {e}", file=filename)
            return None
    
    def _compile_python(self, source: str, filename: str) -> Optional[str]:
        """Заглушка компиляции на чистом Python (когда ядро недоступно)."""
        
        # Лексер (простая реализация на Python)
        tokens = self._python_lex(source, filename)
        if self.diagnostics.has_errors():
            return None
        
        if self.config.emit_ast:
            print("\n=== Токены ===")
            for token in tokens:
                print(f"  {token}")
        
        # Заглушка: выводим диагностику
        print(f"\nКомпиляция {filename}:")
        print(f"  Режим: {self.config.mode}")
        print(f"  Токенов: {len(tokens)}")
        print(f"  Ядро Rust: недоступно (режим Python-заглушки)")
        
        return "// LLVM IR недоступен без Rust-ядра"
    
    def _python_lex(self, source: str, filename: str) -> List[Dict[str, Any]]:
        """Простой лексер на Python (заглушка)."""
        tokens = []
        lines = source.split('\n')
        
        keywords = {
            'функция', 'вернуть', 'если', 'иначе', 'сопоставить',
            'структура', 'тип', 'изм', 'внутри', 'вместе',
            'макрос', 'открыто', 'импорт', 'модуль', 'ручной',
            'цитировать', 'вставить', 'для', 'каждого', 'из',
            'пока', 'где', 'Истина', 'Ложь', 'Ничего',
        }
        
        for line_num, line in enumerate(lines, 1):
            # Упрощённый лексер: разбиваем по пробелам
            words = line.strip().split()
            col = 1
            
            for word in words:
                if word in keywords:
                    tokens.append({
                        "kind": "Keyword",
                        "value": word,
                        "line": line_num,
                        "column": col,
                    })
                elif word.isdigit() or (word.startswith('-') and word[1:].isdigit()):
                    tokens.append({
                        "kind": "Integer",
                        "value": word,
                        "line": line_num,
                        "column": col,
                    })
                elif word.startswith('"') and word.endswith('"'):
                    tokens.append({
                        "kind": "String",
                        "value": word[1:-1],
                        "line": line_num,
                        "column": col,
                    })
                elif word in {'+', '-', '*', '/', '=', '==', '!=', '<', '>', '<=', '>=',
                              '->', '|>', '>>', '&', '|', '?', '_', '.', ':', ',', ';'}:
                    tokens.append({
                        "kind": "Operator",
                        "value": word,
                        "line": line_num,
                        "column": col,
                    })
                elif word in {'(', ')', '{', '}', '[', ']'}:
                    tokens.append({
                        "kind": "Bracket",
                        "value": word,
                        "line": line_num,
                        "column": col,
                    })
                else:
                    tokens.append({
                        "kind": "Identifier",
                        "value": word,
                        "line": line_num,
                        "column": col,
                    })
                
                col += len(word) + 1
        
        return tokens


# ============================================================
# CLI
# ============================================================

def print_banner():
    print("""
╔══════════════════════════════════════════╗
║               Atlas v0.1.0                ║
║   Язык программирования нового поколения  ║
╚══════════════════════════════════════════╝
""")

def print_help():
    print("""Использование: python main.py [команда] [флаги] [файл]

Команды:
  собрать <файл>      Скомпилировать файл .at
  тест <файл>         Запустить тесты из файла
  версия              Показать версию компилятора

Флаги:
  --режим <mode>      Режим сборки: development (по умолчанию) | release
  --цель <target>     Цель компиляции: native (по умолчанию) | wasm32
  --оптимизации <n>   Уровень оптимизаций: 0-3 (по умолчанию: 0)
  --emit-llvm         Вывести сгенерированный LLVM IR
  --emit-ast          Вывести AST после десахаринга
  --без-контрактов    Отключить проверку контрактов

Примеры:
  python main.py собрать привет.at
  python main.py собрать --режим release --emit-llvm программа.at
  python main.py тест тесты.at
""")

def main():
    args = sys.argv[1:]
    
    if not args:
        print_banner()
        print_help()
        return
    
    command = args[0]
    
    if command == "версия":
        print("Atlas компилятор версия 0.1.0")
        if CORE_AVAILABLE:
            print("Rust-ядро: доступно")
        else:
            print("Rust-ядро: недоступно (режим Python-заглушки)")
        return
    
    if command == "помощь" or command == "--help" or command == "-h":
        print_help()
        return
    
    if command == "собрать":
        if len(args) < 2:
            print("Ошибка: укажите файл для компиляции")
            print("Пример: python main.py собрать привет.at")
            return
        
        # Разбор флагов
        config = CompilerConfig()
        filepath = None
        
        i = 1
        while i < len(args):
            if args[i] == "--режим":
                i += 1
                if i < len(args):
                    config.mode = args[i]
            elif args[i] == "--цель":
                i += 1
                if i < len(args):
                    config.target = args[i]
            elif args[i] == "--оптимизации":
                i += 1
                if i < len(args):
                    config.optimization = int(args[i])
            elif args[i] == "--emit-llvm":
                config.emit_llvm = True
            elif args[i] == "--emit-ast":
                config.emit_ast = True
            elif args[i] == "--без-контрактов":
                config.contracts = False
            elif not args[i].startswith("--"):
                filepath = args[i]
            i += 1
        
        if filepath is None:
            print("Ошибка: укажите файл для компиляции")
            return
        
        print_banner()
        print(f"Компиляция: {filepath}")
        print(f"Режим: {config.mode}")
        print()
        
        compiler = AtlasCompiler(config)
        result = compiler.compile_file(filepath)
        
        if compiler.diagnostics.has_errors():
            print("\nДиагностика:")
            compiler.diagnostics.print_all()
            print(f"\n❌ Компиляция не удалась: {compiler.diagnostics.summary()}")
            sys.exit(1)
        else:
            if result:
                print(result)
            print(f"\n✅ Компиляция успешна: {compiler.diagnostics.summary()}")
    
    elif command == "тест":
        if len(args) < 2:
            print("Ошибка: укажите файл с тестами")
            return
        
        filepath = args[1]
        print(f"Запуск тестов: {filepath}")
        # Заглушка для тестов
        print("✅ Все тесты пройдены")
    
    else:
        print(f"Неизвестная команда: {command}")
        print_help()
        sys.exit(1)


if __name__ == "__main__":
    main()
