<div align="center" >
<img src="https://github.com/azoth-os/azoth/blob/2ea2423a5db80ae3ff16b2ab9b0407db13230069/.github/icons/azoth-icon.png" width="35%" alt="azoth-icon">
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


## ⚡ Azoth vs Architecture Classique


```mermaid
graph TD
    %% --- Styles ---
    classDef userland fill:#e1f5fe,stroke:#01579b,stroke-width:2px;
    classDef kernel fill:#f3e5f5,stroke:#4a148c,stroke-width:2px;
    classDef hardware fill:#424242,stroke:#000000,stroke-width:2px,color:#fff;
    classDef ipc fill:#fff9c4,stroke:#fbc02d,stroke-width:2px,stroke-dasharray: 5 5;

    %% --- Espace Utilisateur ---
    subgraph UserSpace [📱 ESPACE UTILISATEUR (Wasm Modules)]
        direction LR
        App[App 'A'<br/>(Rust)]:::userland
        Driver[Pilote GPU<br/>(Zig)]:::userland
        FS[Système de<br/>Fichiers]:::userland
    end

    %% --- Couche IPC ---
    IPC(🚀 Zéro-Copie IPC / Shared Memory):::ipc

    %% --- Noyau Athanor ---
    subgraph KernelSpace [🛡️ NOYAU ATHANOR (Kernel Space)]
        direction TB
        Mem[Gestion Mémoire]:::kernel
        Sched[Ordonnanceur]:::kernel
        Disp[IPC Dispatcher]:::kernel
        Int[Interrupts / IDT]:::kernel
    end

    %% --- Matériel ---
    subgraph HW [💻 MATÉRIEL]
        CPU[CPU: x86_64 / ARM64 / RISC-V]:::hardware
    end

    %% --- Connexions ---
    App <--> IPC
    Driver <--> IPC
    FS <--> IPC
    
    IPC <--> Disp
    
    Disp --- Mem
    Disp --- Sched
    Disp --- Int
    
    KernelSpace <--> CPU
```

  
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
