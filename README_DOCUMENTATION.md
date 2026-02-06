<!-- Page de garde -->

# Documentation projet Docker Swarm

**Client :** Projet pédagogique — Déploiement multi‑services

**Équipe :** Auteur unique

**Date :** 06/02/2026

**Version :** 1.0

---

**Page 1/3**

## Sommaire

1. [Contexte et objectifs](#contexte-et-objectifs)
2. [Architecture et composants](#architecture-et-composants)
3. [Déploiement et vérifications](#déploiement-et-vérifications)
4. [Captures d’écran](#captures-décran)
5. [Fonctionnel / Non fonctionnel](#fonctionnel--non-fonctionnel)
6. [Notes d’exploitation](#notes-dexploitation)

---

# Contexte et objectifs

Mettre en place un cluster Docker Swarm composé de **1 manager** et **2 workers** (Docker‑in‑Docker), puis déployer une application multi‑services (API Rust, front statique, PostgreSQL, Motor Admin) avec accès depuis l’hôte.

**Objectifs opérationnels** :
- Provisionner un Swarm fonctionnel (manager + workers).
- Publier les services web (API, front, admin) vers l’hôte.
- Documenter clairement les limites rencontrées et les contournements appliqués.

---

**Page 2/3**

# Architecture et composants

## Vue d’ensemble

- **Swarm** : 1 manager + 2 workers via conteneurs Docker‑in‑Docker.
- **Services déployés** :
  - **API** (Rust) — port 3000
  - **Front** (Nginx statique) — port 8080
  - **PostgreSQL** — base applicative
  - **Motor Admin** — port 3001
  - **Portainer** (admin Swarm) — port 9000

## Réseaux et volumes

- Réseau Swarm dédié : `swarmnet` (10.10.0.0/24)
- Volumes persistants :
  - `postgres_data`
  - `portainer_data`

---

# Déploiement et vérifications

## Étapes clés

1. Création des nœuds manager/worker avec ports publiés.
2. Initialisation du Swarm sur le manager.
3. Déploiement de Portainer pour l’administration.
4. Build des images (API, front) **sur l’hôte** puis publication sur Docker Hub.
5. Déploiement de la stack via `docker stack deploy`.

## Vérifications d’accès

- API : http://localhost:3000
- Front : http://localhost:8080
- Motor Admin : http://localhost:3001
- Portainer : http://localhost:9000

---

# Captures d’écran

Les captures sont disponibles dans le dossier [screens/](screens/) :

| Élément | Fichier | Commentaire |
|---|---|---|
| Swarm | [screens/swarm](screens/swarm) | État des nœuds et du cluster |
| Swarm déployé | [screens/swarm_deployed](screens/swarm_deployed) | Services et réplicas |
| Portainer | [screens/portainer](screens/portainer) | Interface d’administration |
| Motor Admin | [screens/motoradmin](screens/motoradmin) | Console d’admin BDD |
| Compteur | [screens/compteur](screens/compteur) | Front applicatif |

---

**Page 3/3**

# Fonctionnel / Non fonctionnel

## Ce qui fonctionne

- Swarm opérationnel avec 1 manager + 2 workers.
- Déploiement des services via une stack.
- Accès au front, à l’API, à Motor Admin et à Portainer depuis l’hôte.
- Persistance des données PostgreSQL via volume.

## Ce qui ne fonctionne pas (ou nécessite contournement)

- **`build:` ignoré en Swarm** : le déploiement échoue si l’image n’est pas fournie.
  - **Contournement** : build local + push Docker Hub + `image:` dans la stack.
- **CORS / API inaccessible** si les ports ne sont pas publiés côté manager.
  - **Contournement** : publier explicitement `3000/8080/3001/9000` sur le manager.

---

# Notes d’exploitation

- Les identifiants Portainer sont ceux de la mise en place initiale (à renouveler en production).
- Les services exposés doivent être sécurisés si le déploiement est public.
- Le guide technique détaillé est disponible dans [readme.md](readme.md).
