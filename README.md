**PerfumeLedger — Inventario de Perfumes en Solana**

PerfumeLedger es un **programa on-chain construido en Rust utilizando el framework Anchor** sobre la blockchain de Solana. Su objetivo es permitir que una perfumería o negocio registre y administre su inventario de perfumes de forma **descentralizada, segura y permanente en la blockchain**.

Este sistema guarda la información directamente en cuentas del programa, lo que significa que los datos no dependen de un servidor tradicional.

* Objetivo del proyecto

PerfumeLedger funciona como un **sistema de gestión de inventario en blockchain** que permite realizar operaciones básicas sobre perfumes registrados.

Las principales funciones del sistema son:

• Crear el perfil de una perfumería asociado a una wallet
• Registrar nuevos perfumes con su información
• Actualizar información del inventario
• Activar o desactivar disponibilidad de un perfume
• Eliminar perfumes del sistema

Toda la información queda registrada de manera transparente y segura en la red de Solana.

---

## Estructura del programa

El programa organiza los datos en una jerarquía simple dentro de la blockchain.

Wallet (Owner)
│
└── Cuenta de Perfumería
    │
    ├── Perfume 1
    ├── Perfume 2
    └── Perfume 3

Cada perfumería tiene su propio inventario vinculado a la wallet del propietario.

---

## Estructuras de datos principales

### Perfumería

Campo | Tipo | Descripción
owner | Pubkey | Dirección de la wallet propietaria
nombre | String | Nombre del negocio o perfumería
inventario | Vec | Lista de perfumes almacenados

---

### Perfume

Campo | Tipo | Descripción
nombre | String | Nombre del perfume
marca | String | Marca del producto
precio | u32 | Precio del perfume
activo | bool | Indica si está disponible en el inventario

---

## Funciones del programa

El contrato inteligente incluye varias instrucciones para interactuar con los datos:

**crear_perfumeria(nombre)**
Crea una nueva cuenta de perfumería vinculada al propietario.

**registrar_perfume(nombre, marca, precio)**
Agrega un perfume nuevo al inventario.

**actualizar_precio(nombre, nuevo_precio)**
Permite modificar el precio de un perfume existente.

**cambiar_estado(nombre)**
Activa o desactiva la disponibilidad de un perfume.

**eliminar_perfume(nombre)**
Remueve un perfume del inventario almacenado.

---

## Direcciones derivadas (PDA)

El programa utiliza **Program Derived Addresses (PDA)** para generar cuentas únicas dentro de Solana.

Cuenta | Seeds utilizadas
Perfumería | ["perfumeria", owner_pubkey]

Esto asegura que:

• Cada wallet puede tener solo una perfumería registrada
• Los datos no pueden ser modificados por terceros
• Solo el propietario de la cuenta puede actualizar su inventario

---

## Cómo ejecutar el proyecto

Para probar el programa puedes utilizar **Solana Playground**.

Pasos básicos:

1. Abrir Solana Playground
2. Crear o pegar el código dentro del archivo `src/lib.rs`
3. Conectar tu wallet en la red **Devnet**
4. Presionar **Build** para compilar el programa
5. Presionar **Deploy** para publicarlo
6. Ejecutar las funciones desde el panel de pruebas

---

## Ejemplo de uso

Flujo típico de interacción con el programa:

1. crear_perfumeria("Aroma Elite")
2. registrar_perfume("Sauvage", "Dior", 2500)
3. registrar_perfume("Invictus", "Paco Rabanne", 2200)
4. cambiar_estado("Sauvage")
5. actualizar_precio("Invictus", 2400)
6. eliminar_perfume("Sauvage")

---

##Tecnologías utilizadas

Tecnología | Uso dentro del proyecto
Solana | Blockchain donde se ejecuta el programa
Anchor | Framework para desarrollar smart contracts
Rust | Lenguaje principal del programa

---
Autor Proyecto desarrollado como parte de la certificación de Solana, implementando un smart contract en Solana para la gestión descentralizada de inventario de perfumes.
