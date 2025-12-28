<img align="right" src="https://raw.githubusercontent.com/azoth-os/.github/refs/heads/main/icons/azoth-v3.jpg" width="45%" />

# *Azoth* 🔥 The Operating System

> [!WARNING]
>
> Ce répertoire est en cours de développement actif 🛠️ (Pre-alpha) ⏳. Les APIs internes sont sujettes à des modifications fréquentes 🔄.

<a href="https://github.com/azoth-os/athanor/actions/workflows/cargo-build.yml">
  <img src="https://github.com/azoth-os/athanor/actions/workflows/cargo-build.yml/badge.svg" alt="Build Status"/>
</a>

<br clear="all" />

## 📖 Description

**Azoth-OS** est un système d'exploitation de nouvelle génération conçu pour éliminer le compromis historique entre vitesse et sécurité. Il repose sur **Athanor**, un micro-noyau minimaliste écrit en Rust, et utilise l'isolation logicielle (SFI) via WebAssembly pour sécuriser ses pilotes et applications sans sacrifier les performances.

## 🎯 Objectifs du Projet

### 1. Élimination de la Latence (Performance)
* **Zero Hardware Switches :** Utilisation de l'isolation WebAssembly pour éviter les coûteux changements de contexte matériel (Ring 0 <-> Ring 3).
* **IPC Natif :** Communication inter-processus à une vitesse proche de l'appel de fonction direct (Zero-Copy).
* **Green IT :** Réduction drastique des cycles CPU perdus, idéal pour l'embarqué et la consommation d'énergie.

* **Single Address Space (SAS) :** Tous les processus partagent le même espace d'adressage virtuel (rendu sûr par le compilateur et non par le MMU), éliminant le TLB thrashing.

### 2. Sécurité par la Preuve (Security)
* **Memory Safety :** Intégrité mémoire garantie par le système de *Ownership* de Rust.
* **Surface Minimale :** Application stricte du principe de moindre privilège via le noyau **Athanor**.
* **Capability-based Security :** Un processus ne peut accéder à une ressource (fichiers, hardware) que s'il possède un jeton (Capability) explicite.

### 3. Souveraineté et Vie Privée (Privacy)
* **Isolation Granulaire :** Chaque capteur (micro, GPS) est un module isolé. Athanor révoque l'accès physique dès la fin de l'utilisation.
* **Anti-Tracking :** Minimisation des fuites d'informations latérales (Side-Channel) grâce au sandboxing Wasm.
* **État Éphémère :** Capacité de réinitialiser la mémoire d'un service suspect instantanément sans redémarrer l'OS.

* **Data Sovereignty :** Chiffrement transparent des données au repos et en transit entre les modules, avec des clés gérées localement (pas de cloud).

## 4. Résilience et Maintenance (Reliability)
* **Atomic Hot-Swapping :** Capacité de mettre à jour des modules (drivers ou services) à chaud, sans redémarrage du système, grâce à l'architecture modulaire de WebAssembly.
* **Micro-Recovery :** En cas de crash d'un module (panic), le système le redémarre en quelques millisecondes (le temps de réinstancier la VM Wasm) sans affecter le reste de l'OS.
* **Supervision Trees :** Architecture inspirée d'Erlang où des processus superviseurs gèrent les pannes des processus enfants automatiquement.

---
