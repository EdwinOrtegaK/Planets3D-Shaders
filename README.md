# Planets3D-Shaders 🌌
Este proyecto se centra en el uso de shaders para simular una variedad de cuerpos celestes. Se implementaron distintos shaders para emular diferentes efectos visuales, incluyendo texturas fracturadas, patrones gaseosos, y sistemas de anillos. Los shaders fueron creados utilizando Rust y una librería de ruido para generar variaciones en los patrones de color.

## Características Implementadas
Las características que se implementaron en este proyecto incluyen:
- Shaders complejos para planetas: Cada planeta tiene shaders personalizados que representan distintas superficies y atmósferas.
- Sistema de anillos: Un planeta gaseoso cuenta con un sistema de anillos.
- Luna orbitando un planeta: Un planeta rocoso incluye una luna que orbita a su alrededor.
- Efectos de superficie y atmósfera: Algunos planetas tienen efectos de superficie y atmósfera que varían con el tiempo.
- Interactividad: El usuario puede cambiar entre diferentes cuerpos celestes mediante el teclado.

## Requisitos
Asegúrate de tener Rust y Cargo instalados en tu sistema. Puedes verificarlo ejecutando:
```bash
    cargo --version
```

## Instalación y Configuración
1. **Clona el repositorio**:
    ```bash
    git clone <repository-url>
    ```
2. **Navega al directorio**:
   ```bash
    cd <repository-name>
    ```
3. **Instala las dependencias**:
    ```bash
    cargo add minifb nalgebra-glm tobj
    ```
3. **Compila y ejecuta el proyecto**:
    ```bash
    cargo run --release
    ```
## Controles
Una vez dentro del programa, puedes interactuar con los planetas utilizando los siguientes controles:
- **Movimiento de Cámara**
  - Flecha Izquierda: Mueve la cámara hacia la izquierda.
  - Flecha Derecha: Mueve la cámara hacia la derecha.
  - Flecha Arriba: Mueve la cámara hacia arriba.
  - Flecha Abajo: Mueve la cámara hacia abajo.
- **Zoom**
  - Q: Acercar (Zoom in).
  - E: Alejar (Zoom out).
- **Rotación del planeta**
  - A: Rotar la nave a la izquierda (eje Y).
  - D: Rotar la nave a la derecha (eje Y).
  - W: Rotar la nave hacia arriba (eje X).
  - S: Rotar la nave hacia abajo (eje X).
- **Salir**
  - Escape: Cierra la aplicación.

 ## Cambio de planetas
 Durante la ejecución del programa, puedes utilizar los numeros de tu teclado del 1 al 8 para navegar entre los diferentes cuerpos celestes y visualizarlos en tiempo real:
1. Estrella
2. Planeta Rocoso
3. Planeta Gigante Gaseoso con Efecto de Atmósfera
4. Planeta Gaseoso con Sistema de Anillos
5. Planeta Colorido 
6. Planeta Exótico
7. Planeta Rojo Oscuro con Efecto de superficie texturizada
8. Planeta Rocoso con Luna Orbitando (detalles en la superficie con fracturas)

## Imágenes de los Planetas
Aquí puedes ver capturas de los planetas renderizados:

1. Estrella con efecto solar

![image](https://github.com/user-attachments/assets/d37fa546-5d58-4287-95c0-c3955e23fcde)

2. Planeta rocoso

![image](https://github.com/user-attachments/assets/f9e0540f-ae1b-4c94-93dd-15338ffee247)

3. Gigante gaseoso

![image](https://github.com/user-attachments/assets/ca1f9301-1edd-458c-9d85-94247c587d0d)

4. Gaseoso con anillos

![image](https://github.com/user-attachments/assets/5fdf4b2d-0365-4234-af98-fa4e56dd3b12)

5. Planeta colorido

![image](https://github.com/user-attachments/assets/edf76bfe-57b7-49ca-ab35-89c60b4306ed)

6. Planeta exótico

![image](https://github.com/user-attachments/assets/fe86eeee-8e6d-4596-ae72-413d34a8d4bd)

7. Planeta rojo oscuro

![image](https://github.com/user-attachments/assets/72b7aedb-f7f1-4a13-b5dc-397c21db9efc)

8. Planeta rocoso con luna en órbita

![image](https://github.com/user-attachments/assets/c6d3f2c2-1a78-4826-a780-aa9963a8dfd6)

