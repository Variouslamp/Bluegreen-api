# Bluegreen-api

`Bluegreen-api` es un proyecto de practica enfocado en el despliegue de una API sencilla sobre AWS usando una estrategia Blue/Green. El repositorio combina una aplicacion minima en Rust, contenedorizacion con Docker, despliegue en ECS/Fargate, publicacion de imagenes en ECR y automatizacion con GitHub Actions.

## Objetivo

El objetivo de este proyecto es servir como demostracion practica de un flujo de despliegue automatizado en AWS. La API se usa como caso de prueba para validar cambios de version sin interrumpir el servicio, aplicando Blue/Green como estrategia de publicacion.

## Arquitectura general

La solucion esta compuesta por los siguientes elementos:

- API escrita en Rust con `axum`
- Imagen Docker construida desde `api-code/`
- Registro de imagenes en Amazon ECR
- Ejecucion del contenedor en Amazon ECS con Fargate
- Despliegue automatizado con GitHub Actions
- Script Bash para verificacion continua del endpoint

## Tecnologias utilizadas

- Rust
- Axum
- Tokio
- Serde
- Chrono
- Docker
- Docker Compose
- AWS ECS
- AWS Fargate
- Amazon ECR
- GitHub Actions

## Estructura del repositorio

```text
.
├── api-code/
│   ├── Cargo.toml
│   ├── Dockerfile
│   ├── docker-compose.yaml
│   └── src/
│       └── main.rs
├── check-script/
│   └── check-script
├── .github/
│   └── workflows/
│       └── deploy.yaml
├── informe/
│   └── despliegue_bluegreen.pdf
├── task-definition.json
└── README.md
```

## API

La API responde a una peticion `GET` en la ruta `/` y retorna un JSON con la siguiente informacion:

- `mensaje`: estado general de la respuesta
- `version`: version activa de la API
- `hora`: fecha y hora de la solicitud

Ejemplo de respuesta:

```json
{
  "mensaje": "ok",
  "version": "V2.0",
  "hora": "28-06-2026 10:30:00"
}
```

## Script de verificacion

El directorio `check-script/` contiene un script Bash sencillo para hacer solicitudes repetidas al endpoint de la API. Se recomienda ejecutarlo desde la carpeta donde quieras generar el archivo de salida, porque el script crea `salida.txt` en el directorio actual y guarda alli toda la informacion recibida.

Mientras corre, el script imprime en pantalla cada respuesta obtenida con `curl` y tambien la almacena en `salida.txt`. Su ejecucion permanece activa en bucle hasta que el usuario la detiene manualmente con `Ctrl + C`.

Sintaxis:

```bash
./check-script <url-o-ip> <intervalo-en-segundos>
```

Comportamiento de los argumentos:

- El primer argumento es obligatorio y debe ser la URL o IP del servicio al que se quiere hacer `curl`
- El segundo argumento es opcional y define la cantidad de segundos entre cada solicitud
- Si no se indica el segundo argumento, el script usa `1` segundo por defecto

Ejemplo:

```bash
./check-script http://localhost:8080 2
```

## Flujo de despliegue Blue/Green

El proyecto fue pensado para validar despliegues sin interrupcion del servicio. El flujo general es el siguiente:

1. Se realiza un cambio en la rama principal.
2. GitHub Actions construye una nueva imagen Docker.
3. La imagen se publica en Amazon ECR.
4. ECS toma la nueva definicion de tarea.
5. El servicio despliega la nueva version en AWS.
6. El trafico se transfiere de forma controlada a la nueva revision.

## Pipeline automatizado

El workflow ubicado en `.github/workflows/deploy.yaml` se ejecuta con cada `push` sobre `main`.

Este pipeline realiza las siguientes tareas:

- Configura credenciales de AWS
- Inicia sesion en Amazon ECR
- Construye la imagen Docker
- Asigna tags con `latest` y con el hash del commit
- Publica la imagen en ECR
- Actualiza la definicion de tarea de ECS
- Despliega la nueva revision en el servicio configurado

## Ejecucion local

Para pruebas locales se puede usar Docker Compose desde `api-code/`.

```bash
docker compose up --build
```

Luego la API queda disponible en el puerto expuesto por el compose y puede ser consultada con `curl` o con el `check-script`.

## Informe tecnico

El informe con mayor detalle del proceso, la arquitectura y el despliegue realizado esta disponible aqui:

- [Informe tecnico](informe/despliegue_bluegreen.pdf)

## Estado actual

El proyecto ya tiene un despliegue funcional en AWS. Al hacer `push` sobre `main`, el pipeline se ejecuta automaticamente, genera una nueva imagen de la API y despliega el codigo actualizado en un contenedor administrado por AWS.

## Proximos pasos

- Mantener actualizada la documentacion conforme evolucione la infraestructura
- Agregar capturas o evidencia del pipeline si se considera necesario
- Refinar la observabilidad del despliegue y las pruebas de verificacion
