# Bluegreen-api

`Bluegreen-api` es un proyecto sencillo enfocado en una estrategia de despliegue Blue/Green sobre infraestructura AWS. La idea es utilizar una API pequeña como punto de validación para cambios de versión y pruebas de despliegue.

El servicio será implementado en AWS utilizando ECS con Fargate, lo que permitirá desplegar la aplicación de forma administrada y escalable para este tipo de pruebas.

La API responderá a peticiones `GET` con la siguiente información:

- La hora exacta de la solicitud
- Un estado `ok`
- La versión de la API que se encuentre activa en ese momento

Este comportamiento servirá como base para validar despliegues Blue/Green, comparando respuestas entre distintas versiones de la aplicación antes de realizar el cambio definitivo de tráfico.

## Objetivo

El objetivo principal de este proyecto es servir como laboratorio o ejemplo práctico para probar despliegues Blue/Green en AWS, verificando el estado de una aplicación de forma simple y controlada.

## Estructura del proyecto

```text
.
├── api-code/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── check-script/
│   └── check-script
└── README.md
```

## Componentes

### `api-code/`
Contiene la base de la API desarrollada en Rust.  
Por ahora funciona como la base del servicio que más adelante responderá con la hora, el estado y la versión activa.

### `check-script/`
Script sencillo para consultar el endpoint de la API de forma repetida y guardar las respuestas en un archivo local.  
Además, su función es servir como evidencia de que no existe pérdida de conectividad para los usuarios durante el cambio de versión del código, que es justamente uno de los objetivos principales de los despliegues Blue/Green.

## Estado actual

El proyecto se encuentra en una fase inicial.  
Por ahora solo existe la estructura base del repositorio, el script de verificación y el inicio del servicio en Rust.

## Próximos pasos

- Implementar la API en Rust
- Definir el formato exacto de la respuesta
- Agregar la versión de la aplicación en la respuesta
- Desplegar la solución en AWS ECS con Fargate
- Integrar el flujo Blue/Green en AWS
- Documentar el proceso de despliegue cuando la infraestructura esté lista
