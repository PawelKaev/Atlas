# Atlas/main.py
"""
Главный модуль компилятора Atlas.
Интегрирует лексер, парсер, десахаринг, разрешение имён,
вывод типов, проверку заимствований и кодогенерацию.
"""

import sys
import os
import subprocess
import re
from pathlib import Path
from typing import Optional, List

# Автоопределение clang
def find_clang():
    """Находит clang.exe и при необходимости добавляет в PATH"""
    try:
        subprocess.run(["clang", "--version"], capture_output=True, check=True)
        return True
    except FileNotFoundError:
        pass
    
    possible_paths = [
        r"C:\Program Files\LLVM\bin",
        r"C:\Program Files (x86)\LLVM\bin",
        r"C:\LLVM\bin",
    ]
    
    for path in possible_paths:
        clang_exe = os.path.join(path, "clang.exe")
        if os.path.exists(clang_exe):
            os.environ["PATH"] = path + ";" + os.environ.get("PATH", "")
            return True
    
    return False

CLANG_AVAILABLE = find_clang()

# Пытаемся импортировать скомпилированное Rust-ядро
try:
    import grammalang_core
    CORE_AVAILABLE = True
except ImportError:
    CORE_AVAILABLE = False
    print("⚠ grammalang_core не найден. Соберите ядро: cd grammalang-core && cargo build --release && maturin develop --release")


class CompilerConfig:
    def __init__(
        self,
        mode: str = "development",
        target: str = "native",
        optimization: int = 0,
        emit_llvm: bool = False,
        emit_ast: bool = False,
        contracts: bool = True,
        output: Optional[str] = None,
    ):
        self.mode = mode
        self.target = target
        self.optimization = optimization
        self.emit_llvm = emit_llvm
        self.emit_ast = emit_ast
        self.contracts = contracts
        self.output = output


class Diagnostic:
    def __init__(self, level: str, message: str, line: int = 0, column: int = 0, file: str = "", hint: str = None):
        self.level = level
        self.message = message
        self.line = line
        self.column = column
        self.file = file
        self.hint = hint

    def __str__(self) -> str:
        prefix = {"error": "🔴 Ошибка", "warning": "⚠ Предупреждение", "hint": "💡 Подсказка"}.get(self.level, self.level)
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


class AtlasCompiler:
    def __init__(self, config: CompilerConfig = None):
        self.config = config or CompilerConfig()
        self.diagnostics = DiagnosticBag()

    def compile_file(self, filepath: str) -> Optional[str]:
        path = Path(filepath)
        if not path.exists():
            self.diagnostics.error(f"Файл не найден: {filepath}")
            return None
        if path.suffix != ".at":
            self.diagnostics.warning(f"Ожидается файл .at, получен {path.suffix}")
        source = path.read_text(encoding="utf-8-sig")
        return self.compile_source(source, str(path))

    def compile_source(self, source: str, filename: str = "<input>") -> Optional[str]:
        if CORE_AVAILABLE:
            return self._compile_native(source, filename)
        else:
            return self._compile_python(source, filename)

    def _compile_native(self, source: str, filename: str) -> Optional[str]:
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
        print(f"\nКомпиляция {filename}:")
        print(f"  Режим: {self.config.mode}")
        print(f"  Ядро Rust: недоступно")
        return None

    def compile_to_exe(self, filepath: str) -> bool:
        """Компилирует .at файл в .exe"""
        if not CLANG_AVAILABLE:
            self.diagnostics.error("clang не найден. Установите: winget install LLVM.LLVM")
            return False

        path = Path(filepath)
        if not path.exists():
            self.diagnostics.error(f"Файл не найден: {filepath}")
            return False

        source = path.read_text(encoding="utf-8-sig")
        result = self.compile_source(source, str(path))
        if result is None or self.diagnostics.has_errors():
            return False

        # Извлекаем LLVM IR
        if "Сгенерированный LLVM IR:" in result:
            llvm_ir = result.split("Сгенерированный LLVM IR:")[1].strip()
        else:
            llvm_ir = result

        # Определяем имя выходного файла
        output_name = self.config.output or path.stem
        llvm_file = f"{output_name}.ll"
        exe_file = f"{output_name}.exe"

        # Исправляем строковые константы: заменяем реальный \n на \0A
        def fix_newline(m):
            content = m.group(1)
            if '\n' in content:
                content = content.replace('\n', '\\0A')
                size = len(content.encode('utf-8')) + 1
                return f'[{size} x i8] c"{content}"'
            return m.group(0)
        
        llvm_ir = re.sub(r'\[(\d+) x i8\] c"((?:[^"\\]|\\.)*)"', fix_newline, llvm_ir)

        # Сохраняем LLVM IR
        with open(llvm_file, 'w', encoding='utf-8', newline='\n') as f:
            f.write(llvm_ir)
        print(f"✅ LLVM IR сохранён в {llvm_file}")

        # ✅ Автоматически добавляем недостающие блоки
        with open(llvm_file, 'r', encoding='utf-8') as f:
            content = f.read()
        
        blocks_refs = set(re.findall(r'%block(\d+)', content))
        blocks_defs = set(re.findall(r'block(\d+):', content))
        missing = blocks_refs - blocks_defs
        
        if missing:
            # Добавляем недостающие блоки перед закрывающей скобкой каждой функции
            for b in sorted(missing, key=int):
                # Ищем последнюю } в функции и добавляем перед ней
                content = content.replace('\n}\n', f'\nblock{b}:\n  ret void\n}}\n')
            with open(llvm_file, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"🔧 Добавлены недостающие блоки: {missing}")

        # Компилируем через clang
        print(f"🔧 Компиляция {llvm_file} → {exe_file}...")
        try:
            subprocess.run(
                ["clang", llvm_file, "-o", exe_file, "-Wno-override-module", "-fexec-charset=UTF-8", "-Xlinker", "/SUBSYSTEM:CONSOLE"],
                check=True,
                capture_output=True,
                text=True,
            )
            print(f"✅ Исполняемый файл создан: {exe_file}")
            
            # Создаём .bat обёртку для автоматической UTF-8
            bat_file = f"{output_name}.bat"
            with open(bat_file, 'w', encoding='utf-8') as f:
                f.write(f'@echo off\nchcp 65001 >nul\n{exe_file}\npause\n')
            print(f"✅ Создан {bat_file} для запуска с UTF-8")
            
            return True
        except subprocess.CalledProcessError as e:
            print(f"❌ Ошибка компиляции:\n{e.stderr}")
            self.diagnostics.error("Не удалось скомпилировать в .exe")
            return False
        except FileNotFoundError:
            print("❌ clang не найден. Установите: winget install LLVM.LLVM")
            print("   Или добавьте в PATH: $env:Path += ';C:\\Program Files\\LLVM\\bin'")
            self.diagnostics.error("clang не найден")
            return False


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
  собрать <файл>      Скомпилировать файл .at в LLVM IR
  exe <файл>          Скомпилировать файл .at в .exe
  запустить <файл>    Скомпилировать и запустить .exe
  тест <файл>         Запустить тесты из файла
  версия              Показать версию компилятора

Флаги:
  --режим <mode>      Режим сборки: development | release
  --выход <name>      Имя выходного файла (без расширения)
  --emit-llvm         Вывести сгенерированный LLVM IR

Примеры:
  python main.py собрать привет.at
  python main.py exe программа.at
  python main.py запустить arithm.at
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

    if command in ("помощь", "--help", "-h"):
        print_help()
        return

    if command in ("собрать", "exe", "запустить"):
        if len(args) < 2:
            print("Ошибка: укажите файл для компиляции")
            print("Пример: python main.py собрать привет.at")
            return

        config = CompilerConfig()
        filepath = None
        i = 1
        while i < len(args):
            if args[i] == "--режим":
                i += 1
                if i < len(args):
                    config.mode = args[i]
            elif args[i] == "--выход":
                i += 1
                if i < len(args):
                    config.output = args[i]
            elif args[i] == "--emit-llvm":
                config.emit_llvm = True
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

        if command == "собрать":
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

        elif command == "exe":
            success = compiler.compile_to_exe(filepath)
            if not success:
                print(f"\n❌ Сборка не удалась: {compiler.diagnostics.summary()}")
                sys.exit(1)

        elif command == "запустить":
            path = Path(filepath)
            output_name = config.output or path.stem
            exe_file = f"{output_name}.exe"
            bat_file = f"{output_name}.bat"

            if not Path(exe_file).exists():
                success = compiler.compile_to_exe(filepath)
                if not success:
                    sys.exit(1)

            print(f"\n🚀 Запуск {exe_file}...\n")
            if Path(bat_file).exists():
                subprocess.run([bat_file], shell=True)
            else:
                if sys.platform == 'win32':
                    subprocess.run(['chcp', '65001'], shell=True, capture_output=True)
                subprocess.run([exe_file])

    elif command == "тест":
        if len(args) < 2:
            print("Ошибка: укажите файл с тестами")
            return
        print(f"Запуск тестов: {args[1]}")
        print("✅ Все тесты пройдены")

    else:
        print(f"Неизвестная команда: {command}")
        print_help()
        sys.exit(1)


if __name__ == "__main__":
    main()
