<img align="right" src="https://raw.githubusercontent.com/azoth-os/.github/main/icons/azoth-v3.jpg" width="50%" alt="Azoth OS"/>

# _~Azoth~Operating~System_


> [!WARNING]
>
> This project is currently under development ! :hammer_and_wrench:
>
> *By [@jclermonttt]*.

## 📖 Description

**Azoth-OS** est un système d'exploitation de nouvelle génération conçu pour éliminer le compromis historique entre vitesse et sécurité. Il repose sur **Athanor**, un micro-noyau minimaliste écrit en Rust, et utilise l'isolation logicielle (SFI) via WebAssembly pour sécuriser ses pilotes et applications sans sacrifier les performances.

## 🎯 Objectifs du Projet

### 1. Élimination de la Latence (Performance)
* **Zero Hardware Switches :** Utilisation de l'isolation WebAssembly pour éviter les coûteux changements de contexte matériel (Ring 0 <-> Ring 3).
* **IPC Natif :** Communication inter-processus à une vitesse proche de l'appel de fonction direct (Zero-Copy).
* **Green IT :** Réduction drastique des cycles CPU perdus, idéal pour l'embarqué et la consommation d'énergie.

### 2. Sécurité par la Preuve (Security)
* **Memory Safety :** Intégrité mémoire garantie par le système de *Ownership* de Rust.
* **Surface Minimale :** Application stricte du principe de moindre privilège via le noyau **Athanor**.
* **Capability-based Security :** Un processus ne peut accéder à une ressource (fichiers, hardware) que s'il possède un jeton (Capability) explicite.

### 3. Souveraineté et Vie Privée (Privacy)
* **Isolation Granulaire :** Chaque capteur (micro, GPS) est un module isolé. Athanor révoque l'accès physique dès la fin de l'utilisation.
* **Anti-Tracking :** Minimisation des fuites d'informations latérales (Side-Channel) grâce au sandboxing Wasm.
* **État Éphémère :** Capacité de réinitialiser la mémoire d'un service suspect instantanément sans redémarrer l'OS.

---

## ⚡ Azoth vs Architecture Classique

### 🏛️ Architecture Classique (Monolithique)
*Dans un système classique (type Linux/Windows), les pilotes tournent avec les mêmes privilèges que le noyau. Un bug graphique peut faire planter tout le système.*

```mermaid
graph TD
    %% --- Styles ---
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
[!Azoth]: https://github.com/azoth-os/azoth/
[Athanor]: https://github.com/azoth-os/athanor/
[@jclermonttt]: http://github.com/jclermonttt
