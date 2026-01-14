# Guessing Game en Rust 🦀

Este es un juego de adivinanzas desarrollado en **Rust** como parte de mi aprendizaje del lenguaje. El proyecto demuestra conceptos fundamentales de Rust, como la gestión de memoria (ownership), el manejo de errores robusto y la modularidad.

## 🚀 Características

- **Modularidad:** El código está dividido en funciones específicas para entrada de datos, lógica de negocio y salida.
- **Manejo de Errores:** Implementación de `Result` y `match` para evitar fallos (panics) si el usuario ingresa datos no numéricos.
- **Sintaxis Moderna:** Uso de la versión más reciente de la crate `rand` (0.9+), empleando `random_range`.
- **Lógica de Comparación:** Uso de `std::cmp::Ordering` para una comparación de tipos idiomática.

## 🛠️ Conceptos de Rust Aplicados

- **Variables Mutables:** Uso de `let mut` para el manejo de strings y generadores aleatorios.
- **Control de Flujo:** Implementación de `loop`, `break` y `continue`.
- **Pattern Matching:** Uso intensivo de `match` para manejar resultados `Ok` y `Err`.
- **Tipado Fuerte:** Conversión segura de tipos de `String` a `u32`.
