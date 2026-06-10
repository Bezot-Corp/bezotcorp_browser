# BezotCorp Browser — Architecture

## Principes généraux

- Un seul dépôt Git.
- Un seul crate Rust tant que tout est construit et publié ensemble.
- Une architecture modulaire par domaine métier.
- Aucun découpage artificiel en dizaines de crates sans besoin réel.
- Possibilité d'extraire certains composants plus tard si cela devient pertinent.

## Structure prévue

```text
src/
├── main.rs
├── app/
├── ui/
├── conversation/
├── ai/
├── bridge/
├── storage/
└── browser/
```

### app/

Point d'entrée et orchestration globale de l'application.

### ui/

Gestion de l'interface utilisateur, fenêtres, vues et composants graphiques.

### conversation/

Gestion des conversations, historique, modèles de données et virtualisation des contenus volumineux.

### ai/

Connecteurs vers les différents fournisseurs d'intelligence artificielle et logique d'orchestration.

### bridge/

Pont sécurisé vers les ressources locales :

- système de fichiers ;
- éditeurs de code ;
- outils externes ;
- futures intégrations locales.

### storage/

Persistance locale des données utilisateur et des paramètres.

### browser/

Module réservé aux fonctionnalités directement liées au navigateur et aux futures évolutions du moteur de rendu ou de navigation.

## Principes d'évolution

- Construire uniquement ce qui est nécessaire.
- Garder un couplage faible entre les modules.
- Préserver une compilation et un déploiement unifiés.
- Faire évoluer l'architecture progressivement en fonction des besoins réels.
