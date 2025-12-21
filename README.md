<div align="center" >
<img src="./icons/azoth-icon.png" width="35%" alt="azoth-icon">
</div>

> [!NOTE]
>
> This project is currently under development ! :hammer_and_wrench:
>
> *By **[jclermonttt]***.

## 🎯 Project Objectives

The goal of [Azoth] is to resolve the historical trade-off between **speed** and **security**.

1. **Context Latency Elimination:** - Use Software Isolation (SFI) via WebAssembly to avoid expensive hardware context switches.

- Achieve inter-process communication (IPC) speed close to native function call speed.

2. **Security through Proof:**

- Guarantee memory integrity using Rust's ownership system.

- Reduce the attack surface by maintaining a minimalist **[Athanor]** kernel (Principle of Least Privilege).

3. **Hardware Independence:**

- Maintain complete abstraction, allowing the same OS to boot on a PC (x86_64) or a mobile device (ARM).

 4. **Modular Ecosystem:**
Enable the development of drivers and services in any language that can be compiled into Wasm, while ensuring they cannot crash the system. 5. **Sovereignty and Privacy (Privacy by Design):**

- **Granular Isolation:** Each sensor (microphone, camera, GPS) has its own isolated module. Access is physically revoked by [Athanor] as soon as it is no longer needed.

- **Hardware Anti-Tracking:** Minimize side-channel information leakage between applications through Wasm isolation.

- **Zero Unwanted Persistence:** Ability to instantly reset the memory state of a suspicious service without restarting the system.

## 🏗️ Architecture Système

Azoth-OS abandonne l'architecture monolithique classique (comme Linux) pour un design modulaire strict.

+---------------------------------------------------------------+
|  📱  ESPACE UTILISATEUR (User Space)                          |
|                                                               |
|  +-------------+    +-------------+    +-------------+        |
|  |  App "A"    |    |  Pilote GPU |    | Système de  |        |
|  |   (Wasm)    |    |   (Wasm)    |    | Fichiers    |        |
|  +------+------+    +------+------+    +------+------+        |
|         |                  |                  |               |
|         v                  v                  v               |
+---------|------------------|------------------|---------------+
          |  🚀 Zéro-Copie IPC (Shared Memory)  |
+---------|------------------|------------------|---------------+
|  🛡️  NOYAU ATHANOR (Kernel Space)                             |
|                                                               |
|  [ Gestionnaire de Mémoire ]  [ Ordonnanceur (Scheduler) ]    |
|  [     IPC Dispatcher      ]  [ Gestion des interruptions ]   |
|                                                               |
+---------------------------------------------------------------+
|  💻  MATÉRIEL (Hardware)                                      |
|  (x86_64 / ARM64 / RISC-V)                                    |
+---------------------------------------------------------------+


[Azoth]: <https://github.com/azoth-os/azoth/>
[Athanor]: <https://github.com/azoth-os/athanor/>
[jclermonttt]: <http://github.com/jclermonttt>
