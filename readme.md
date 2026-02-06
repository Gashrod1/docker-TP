# Partie 2

## Objectif
Créer un Swarm **1 manager + 2 workers** sans VMs lourdes, en utilisant 3 conteneurs Docker‑in‑Docker.

---

## Création des 3 nœuds (avec ports publiés)
>  Problème rencontré et important : publier les ports sur le conteneur `mgr`, sinon les services Swarm ne sont pas accessibles depuis l’hôte (erreur CORS côté navigateur).

```
docker network create --subnet 10.10.0.0/24 swarmnet

docker run -d --privileged --name mgr --hostname mgr \
  --network swarmnet --ip 10.10.0.2 \
  -p 3000:3000 -p 8080:8080 -p 3001:3001 -p 9000:9000 \
  -v /var/lib/docker-mgr:/var/lib/docker docker:27-dind

docker run -d --privileged --name w1 --hostname w1 \
  --network swarmnet --ip 10.10.0.3 \
  -v /var/lib/docker-w1:/var/lib/docker docker:27-dind

docker run -d --privileged --name w2 --hostname w2 \
  --network swarmnet --ip 10.10.0.4 \
  -v /var/lib/docker-w2:/var/lib/docker docker:27-dind
```

## Initialiser le Swarm
```
docker exec mgr docker swarm init --advertise-addr 10.10.0.2
```

Joindre les workers (sans token en dur) :
```
WORKER_TOKEN=$(docker exec mgr docker swarm join-token -q worker)
docker exec w1 docker swarm join --token "$WORKER_TOKEN" 10.10.0.2:2377
docker exec w2 docker swarm join --token "$WORKER_TOKEN" 10.10.0.2:2377
```

Vérifier :
```
docker exec mgr docker node ls
```

---

## Portainer (administration du cluster)
```
docker exec mgr docker volume create portainer_data

docker exec mgr docker service create \
  --name portainer \
  --publish 9000:9000 \
  --constraint 'node.role == manager' \
  --mount type=bind,src=/var/run/docker.sock,dst=/var/run/docker.sock \
  --mount type=volume,src=portainer_data,dst=/data \
  portainer/portainer-ce
```

Accès : http://localhost:9000

---

# Déployer l’application sur Swarm

## ⚠️ Problèmes rencontrés / bug Docker
1. **Swarm ignore `build:`**  
   → erreur : `image reference must be provided`  
   **Solution** : builder les images sur l’hôte, puis **les charger dans chaque nœud** et utiliser `image:` dans le stack.

2. **CORS “request did not succeed”**  
   → en fait l’API était **inaccessible**, car le conteneur `mgr` n’exposait pas les ports.  
   **Solution** : lancer `mgr` avec `-p 3000/8080/3001/9000`.

---

## 1) Dockerfile pour le front
Créer [front/Dockerfile](front/Dockerfile) :
```
FROM nginx:alpine
COPY . /usr/share/nginx/html
```

## 2) Build des images (sur l’hôte)
```
docker build -t api-rust:latest ./api-rust
docker build -t front-static:latest ./front
```

## 3) Charger les images dans chaque nœud DIND
```
docker save api-rust:latest | docker exec -i mgr docker load
docker save api-rust:latest | docker exec -i w1 docker load
docker save api-rust:latest | docker exec -i w2 docker load

docker save front-static:latest | docker exec -i mgr docker load
docker save front-static:latest | docker exec -i w1 docker load
docker save front-static:latest | docker exec -i w2 docker load
```

## 4) Stack Swarm (sans build, avec image)
Dans le manager :
```
docker exec -it mgr sh
```

Puis dans `mgr`, créer `stack.yml` :
```
cat > stack.yml <<'YAML'
version: "3.9"

services:
  postgres:
    image: postgres:16
    environment:
      POSTGRES_DB: app_db
      POSTGRES_USER: app_user
      POSTGRES_PASSWORD: secretpassword
    volumes:
      - postgres_data:/var/lib/postgresql/data
    deploy:
      replicas: 1

  api:
    image: api-rust:latest
    environment:
      DATABASE_URL: postgres://app_user:secretpassword@postgres:5432/app_db
    ports:
      - "3000:3000"
    deploy:
      replicas: 2
    depends_on:
      - postgres

  front:
    image: front-static:latest
    ports:
      - "8080:80"
    deploy:
      replicas: 2

  motoradmin:
    image: motoradmin/motoradmin
    ports:
      - "3001:3000"
    deploy:
      replicas: 1
    depends_on:
      - postgres

volumes:
  postgres_data:
YAML
```

Déployer :
```
docker stack deploy -c stack.yml app
```

Vérifier :
```
docker service ls
docker stack ps app
```

---

## Vérifications finales
- API : http://localhost:3000
- Front : http://localhost:8080
- Motor Admin : http://localhost:3001