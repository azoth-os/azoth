<div align="center">
<img src="https://raw.githubusercontent.com/azoth-os/.github/main/icons/azoth-v3.jpg" width="50%" alt="Azoth OS">
<h2>☁️ Azoth ☁️</h2>
[Doc] | [Azoth][Azoth Foundation]

</div> 

[Doc]: https://github.com/azoth-os/azoth/docs

---

> [!WARNING]
>
> This project is currently under development ! :hammer_and_wrench:
>
> *By [Azoth Foundation]*.

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

[Azoth Foundation]: https://github.com/azoth-os/
[Athanor]: https://github.com/azoth-os/athanor/
[@jclermonttt]: http://github.com/jclermonttt
