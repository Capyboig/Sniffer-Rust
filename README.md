# 🛰️ Rust Network Packet Sniffer

[![Rust](https://img.shields.io/badge/Rust-2024_Edition-orange?logo=rust&style=flat-square)](https://www.rust-lang.org/)
[![pnet](https://img.shields.io/badge/Library-libpnet_0.35-purple?style=flat-square)](https://docs.rs/pnet/)
[![OS](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux-lightgrey?style=flat-square)](https://github.com/Capyboig/Sniffer-Rust)
[![License](https://img.shields.io/badge/License-MIT-green.svg?style=flat-square)](LICENSE)

Analizador y capturador de tráfico de red a bajo nivel (*Packet Sniffer*) implementado en **Rust** utilizando la biblioteca nativa `pnet`. Capaz de interceptar tramas de enlace de datos (DataLink Layer), desensamblar paquetes IPv4 y decodificar protocolos de transporte TCP/UDP en tiempo real.

---

## 🔍 Características

* **Detección Automática de Interfaces:** Inspecciona el hardware de red del sistema y selecciona de forma automática la interfaz activa conectada (descartando interfaces de loopback).
* **Inspección Multi-Capa (OSI):**
  * **Capa 2 (Enlace de datos):** Decodificación de cabeceras Ethernet (direcciones MAC de origen y destino, EtherType).
  * **Capa 3 (Red):** Desempaquetado de datagramas IPv4 (resolución de IPs de origen y destino).
  * **Capa 4 (Transporte):** Análisis de segmentos TCP y datagramas UDP (identificación de puertos de origen/destino).
* **Filtrado y Análisis de Payloads:** Inspección de tráfico web (puertos 80 HTTP y 443 HTTPS) con decodificación de payload a texto en tiempo real.
* **Soporte Multiplataforma:** Configurado con librerías nativas WinPcap/Npcap (`wpcap.lib`, `Packet.lib`) para Windows y compatibilidad con interfaces POSIX de Linux.

---


🛠️ Stack y Dependencias
Lenguaje: Rust.
Librería de captura: pnet = "0.35.0" (abstracción sobre libpcap / WinPcap).
Enlazado nativo: Packet.lib y wpcap.lib (enlazadas mediante configuración local en .cargo/config.toml).
🚀 Instalación y Uso
Prerrequisitos
En Windows:
Instalar Npcap
 (asegúrate de marcar la casilla "Install Npcap in WinPcap API-compatible Mode" durante la instalación).
Contar con permisos de Administrador al ejecutar el binario (necesarios para abrir interfaces de red en modo promiscuo/raw).


En Linux:

Instalar las dependencias de captura de paquetes:

bash
sudo apt-get install libpcap-dev
Ejecución

Clona el repositorio:

bash
git clone https://github.com/Capyboig/Sniffer-Rust.git
cd Sniffer-Rust

Compila y ejecuta con privilegios elevados:

Windows (PowerShell como Administrador):
powershell
cargo run --release
Linux:
bash
sudo cargo run --release

## 🧠 Flujo de Procesamiento

```mermaid
flowchart LR
    A[Interfaz de Red] -->|DataLink Channel| B[Ethernet Frame]
    B -->|EtherType: IPv4| C[Datagrama IPv4]
    C -->|Protocolo: TCP| D[Segmento TCP]
    C -->|Protocolo: UDP| E[Datagrama UDP]
    D -->|Filtro Puertos 80/443| F[Payload Decodificado]
