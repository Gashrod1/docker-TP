# Projet Docker (Postgres + API Rust + Front + Motor Admin)

## Prérequis
- Docker + Docker Compose

## Démarrage
Depuis la racine du projet :
```
docker compose up -d --build
```

Services exposés :
- API Rust : http://localhost:3000
- Front : http://localhost:8080
- Motor Admin : http://localhost:3001

## Configuration BDD
Postgres (dans docker-compose) :
- DB : `app_db`
- User : `app_user`
- Password : `secretpassword`
- Host (dans le réseau Docker) : `postgres`
- Port : `5432`

La table créée par l’API est `counter`.

## Motor Admin
Connexion :
- URL : http://localhost:3001
- Email : `tp@epsi.com`
- Mot de passe : `Tp1234!`

Pour voir la donnée :
- Se connecter à la base `app_db`
- Chercher la table `counters`
- La ligne `id = 1` contient la valeur du compteur

## API Rust
Endpoints :
- `GET /count` : retourne la valeur du compteur
- `POST /inc` : incrémente le compteur
- `POST /reset` : remet le compteur à 0