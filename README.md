📚 Compilador 
-
Un compilador completo en Rust que transforma código fuente en lenguaje ensamblador x86-64 para Windows y Unix. 

📋 Descripción 
-
* Este proyecto implementa un compilador completo con todas las fases tradicionales de compilación: 

   + Análisis léxico
   + Análisis sintáctico
   + Análisis semántico
   + Generación de código intermedio
   + Optimización
   + Generación de código ensamblador
     

🚀 Características 
-
Lenguaje Soportado 

    Tipos de datos: int, bool, string, arrays
    Variables: Declaración con let y tipado opcional
    Funciones: Con parámetros, valores de retorno y verificación de tipos
    Estructuras de control: if/else, while, for
    Operaciones: Aritméticas, lógicas y de comparación
    Arrays: Unidimensionales y multidimensionales
    Strings: Con operaciones completas
     

Optimizaciones 

    Constant Folding: Evaluación de expresiones constantes en tiempo de compilación
    Dead Code Elimination: Eliminación de código no utilizado
    Common Subexpression Elimination: Eliminación de subexpresiones repetidas
    Loop Optimization: Optimización de bucles
     

Multiplataforma 

    Windows: Genera código compatible con Microsoft Visual C++
    Unix/Linux/macOS: Genera código compatible con sistemas Unix

🛠️ Requisitos 
-
    Rust: Versión 1.56 o superior
    NASM: Ensamblador para generar código objeto
    GCC/Clang (Unix) o Visual Studio (Windows): Para enlazar el código
     
