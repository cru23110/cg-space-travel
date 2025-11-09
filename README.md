# Space Travel - Software Renderer

**Juan Cruz - 23110**

## Descripción

Este proyecto es un renderizador de software 3D (sin usar OpenGL/Vulkan) que simula un viaje espacial por un sistema solar. Está desarrollado en Rust usando únicamente CPU para todos los cálculos de gráficos.

## Características

-   Sistema solar con 8 planetas orbitando alrededor de una estrella central
-   3 tipos de shaders procedurales para planetas: rocosos, gaseosos y de lava
-   Shader animado para el sol con efecto de turbulencia
-   Nave espacial que sigue la cámara en tercera persona
-   Efecto warp (hiperespacio) activable con la tecla F
-   Skybox con 1000 estrellas generadas proceduralmente
-   Sistema de colisiones que previene atravesar planetas y el sol
-   Visualización de órbitas planetarias
-   Z-buffer para renderizado correcto de profundidad

## Requisitos

-   Rust (versión estable)
-   Cargo

## Cómo ejecutar

1. Clonar el repositorio
2. Navegar a la carpeta del proyecto
3. Compilar en modo release:

```bash
cargo build --release
```

4. Ejecutar:

```bash
cargo run --release
```

## Controles

-   **W/S**: Mover cámara adelante/atrás
-   **A/D**: Mover cámara izquierda/derecha
-   **Q/E**: Mover cámara arriba/abajo
-   **F**: Activar/desactivar efecto warp
-   **ESC**: Salir del programa

## Videos y Capturas

- Video de demostración del sistema solar: `/screenshots/space-travel-demo.mp4`
- Capturas de pantalla en `/screenshots/`

## Rama Experimental: game-mode

Este proyecto incluye una rama adicional llamada **`game-mode`** que transforma el simulador de sistema solar en un juego arcade estilo shoot'em up espacial.

### Características de game-mode:

- **Controles arcade**: La nave se mueve solo en el plano 2D (arriba/abajo, izquierda/derecha)
- **Vista cenital fija**: Cámara desde arriba mirando hacia abajo
- **Obstáculos procedurales**: Cubos rojos que se generan automáticamente y vienen hacia la nave
- **Sistema de disparos enemigos**: Los obstáculos disparan proyectiles que debes esquivar
- **Física de vuelo suave**: Inclinación de la nave al moverse lateralmente
- **Límites de movimiento**: ±4 horizontal, ±3 vertical para mantener la jugabilidad

### Controles en game-mode:

- **W**: Mover nave arriba
- **S**: Mover nave abajo
- **A**: Mover nave izquierda
- **D**: Mover nave derecha
- **F**: Activar dash warp (boost temporal)
- **ESC**: Salir

### Para probar game-mode:

```bash
git checkout game-mode
cargo run --release
```

Esta rama es experimental y demuestra cómo el mismo engine puede ser adaptado para diferentes tipos de juegos espaciales.

## Estructura del proyecto

-   `src/main.rs`: Loop principal del juego
-   `src/framebuffer.rs`: Manejo del framebuffer y z-buffer
-   `src/camera.rs`: Sistema de cámara
-   `src/celestial/`: Planetas, estrella y nave
-   `src/shaders/`: Shaders procedurales para planetas y estrella
-   `src/geometry/`: Generación de geometría (esferas) y carga de OBJ
-   `src/pipeline.rs`: Pipeline de renderizado 3D
-   `src/warp.rs`: Sistema de partículas para efecto warp
-   `src/skybox.rs`: Fondo estrellado

## Tecnologías utilizadas

-   **minifb**: Ventana y framebuffer
-   **nalgebra-glm**: Operaciones matemáticas 3D
-   **noise**: Generación de ruido Perlin para shaders
-   **rand**: Generación aleatoria para skybox y partículas
-   **tobj**: Carga de modelos OBJ
