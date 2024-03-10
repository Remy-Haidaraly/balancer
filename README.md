Pour utiliser le projet :

Dans un premier temps, il faut démarrer deux serveurs Python :

```bash
python3 -m http.server 8000
python3 -m http.server 8001
```

(Si l'on souhaite en ajouter d'autres, il faut les placer dans le vecteur des serveurs dans le fichier upstream.rs.)

Ensuite, je peux lancer mon projet :

```bash
cargo run
```

Il demandera de choisir un port, par exemple : 8080.

Ensuite, depuis un terminal, nous pouvons utiliser les commandes suivantes pour tester :

```bash
curl -v http://127.0.0.1:8080
```

(Pour quelques tests)

Ou bien :

```bash
for i in {1..100}; do curl -v http://127.0.0.1:8080; done
```

(Pour effectuer un grand nombre de tests)

Une fois les tests terminés, vous pouvez interrompre le programme en appuyant sur Ctrl+C, ce qui créera un fichier
journal (fxloadbalancer.log) contenant le nombre total de requêtes ainsi que le nombre de requêtes reçues pour chaque serveur Python.
