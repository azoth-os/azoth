<div align="center" >
<img src="https://raw.githubusercontent.com/azoth-os/azoth/refs/heads/main/.github/icons/azoth-icon.png?token=GHSAT0AAAAAADRW4JFMX4VVHW4QMKYOJU7E2KIUOSQ" width="35%" alt="azoth-icon">
</div>

<p align="center">
<!--- BADGE GITHUB --->
</p>

> [!NOTE]
>
> This project is currently under development ! :hammer_and_wrench:
>
> *By [@jclermonttt]*.

## 🎯 Project Objectives

The goal of [Azoth] is to resolve the historical trade-off between **speed** and **security**.

1. **Context Latency Elimination:** - Use Software Isolation (SFI) via WebAssembly to avoid expensive hardware context switches.

- Achieve inter-process communication (IPC) speed close to native function call speed.

2. **Security through Proof:**

- Guarantee memory integrity using Rust's ownership system.

- Reduce the attack surface by maintaining a minimalist **[Athanor]** kernel (Principle of Least Privilege).

3. **Hardware Independence:**

- Maintain complete abstraction, allowing the same OS to boot on a PC (x86_64) or a mobile device (ARM).

4.  **Modular Ecosystem:**
    Enable the development of drivers and services in any language that can be compiled into Wasm, while ensuring they cannot crash the system. 5. **Sovereignty and Privacy (Privacy by Design):**

- **Granular Isolation:** Each sensor (microphone, camera, GPS) has its own isolated module. Access is physically revoked by [Athanor] as soon as it is no longer needed.

- **Hardware Anti-Tracking:** Minimize side-channel information leakage between applications through Wasm isolation.

- **Zero Unwanted Persistence:** Ability to instantly reset the memory state of a suspicious service without restarting the system.

## 🏗️ Architecture Système

Azoth-OS abandonne l'architecture monolithique classique (comme Linux) pour un design modulaire strict.

+---------------------------------------------------------------+
| 📱 ESPACE UTILISATEUR (User Space) |
| |
| +-------------+ +-------------+ +-------------+ |
| | App "A" | | Pilote GPU | | Système de | |
| | (Wasm) | | (Wasm) | | Fichiers | |
| +------+------+ +------+------+ +------+------+ |
| | | | |
| v v v |
+---------|------------------|------------------|---------------+
| 🚀 Zéro-Copie IPC (Shared Memory) |
+---------|------------------|------------------|---------------+
| 🛡️ NOYAU ATHANOR (Kernel Space) |
| |
| [ Gestionnaire de Mémoire ] [ Ordonnanceur (Scheduler) ] |
| [ IPC Dispatcher ] [ Gestion des interruptions ] |
| |
+---------------------------------------------------------------+
| 💻 MATÉRIEL (Hardware) |
| (x86_64 / ARM64 / RISC-V) |
+---------------------------------------------------------------+

## ⚡ Azoth vs Architecture Classique

| Fonctionnalité        | Noyau Monolithique (Linux)                                   | Azoth-OS (Athanor)                                                    |
| :-------------------- | :----------------------------------------------------------- | :-------------------------------------------------------------------- |
| **Pilotes**           | Exécutés en mode privilège (Ring 0). Un bug = Crash système. | Exécutés en espace utilisateur (Wasm). Un bug = Crash du pilote seul. |
| **Isolation**         | Processus lourds, coûteux en contexte.                       | Modules Wasm légers (SFI), isolation mémoire parfaite.                |
| **Langage**           | Majoritairement C (Gestion mémoire manuelle).                | Rust (Sûreté mémoire garantie à la compilation).                      |
| **Surface d'attaque** | Immense (Millions de lignes de code en mode root).           | Minime (Seul Athanor a les pleins pouvoirs).                          |

## 🛤️ Roadmap

### Phase 1: La Genèse (Athanor)

- [x] Configuration de l'environnement Rust (no_std).
- [ ] Bootloader (UEFI/BIOS) minimal.
- [ ] Gestion des interruptions (IDT) et exceptions.
- [ ] Allocateur de mémoire physique et virtuelle.

### Phase 2: L'Écosystème (Wasm)

- [ ] Intégration du runtime Wasm (ex: Wasm3 ou interpréteur maison).
- [ ] Système d'appels système (Syscalls) pour les modules Wasm.
- [ ] Premier pilote "Hello World" en Wasm.

### Phase 3: L'Interface (Userland)

- [ ] Système de fichiers virtuel (VFS).
- [ ] Shell graphique minimal.

[Azoth]: https://github.com/azoth-os/azoth/
[Athanor]: https://github.com/azoth-os/athanor/
[@jclermonttt]: http://github.com/jclermonttt
