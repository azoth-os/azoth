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


🏛️ Classique (Monolithique)

```mermaid
graph TD
    %% --- Styles (Identiques à Azoth pour la cohérence) ---
    classDef userland fill:#e1f5fe,stroke:#01579b,stroke-width:2px;
    classDef kernel fill:#f3e5f5,stroke:#4a148c,stroke-width:2px;
    classDef hardware fill:#424242,stroke:#000000,stroke-width:2px,color:#fff;
    classDef danger fill:#ffcdd2,stroke:#c62828,stroke-width:2px,stroke-dasharray: 5 5;

    %% --- Espace Utilisateur ---
    subgraph UserSpace [📱 ESPACE UTILISATEUR]
        direction LR
        App[App 'A'<br/>(Binaire Natif)]:::userland
        Browser[Navigateur]:::userland
    end

    %% --- Barrière Coûteuse ---
    ContextSwitch(🐌 Context Switch / Syscalls):::danger

    %% --- Noyau Monolithique ---
    subgraph KernelSpace [🛡️ NOYAU MONOLITHIQUE (Ring 0)]
        direction TB
        
        %% Le Cœur
        subgraph Core [Core Kernel]
            Sched[Ordonnanceur]:::kernel
            Mem[Gestion Mémoire]:::kernel
        end

        %% Le Danger : Les pilotes sont DANS le noyau
        subgraph Drivers [⚠️ PILOTES & SERVICES]
            GPU[Pilote GPU<br/>(C / C++)]:::kernel
            FS[Système de<br/>Fichiers]:::kernel
            Net[Réseau]:::kernel
        end
    end

    %% --- Matériel ---
    subgraph HW [💻 MATÉRIEL]
        CPU[CPU]:::hardware
    end

    %% --- Connexions ---
    App <--> ContextSwitch
    Browser <--> ContextSwitch
    
    ContextSwitch <--> Core
    
    Core --- Drivers
    Drivers <--> CPU
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
