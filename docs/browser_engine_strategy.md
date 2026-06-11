# BezotCorp Browser — stratégie moteur, rendu et optimisation

## 1. Vision

BezotCorp Browser n’est pas seulement un navigateur qui affiche des pages web.
L’objectif est de construire un navigateur Rust contrôlé, performant, sécurisé, assisté par IA, capable de réparer et optimiser les sites sans les dénaturer.

La logique principale est :

```txt
Le site fournit une intention.
Le navigateur choisit l’exécution optimale.
```

On ne cherche pas à modifier le sens du site, ni à trahir le travail du développeur.
On cherche à rendre le site meilleur côté utilisateur :

- plus rapide ;
- plus stable ;
- moins gourmand ;
- plus lisible ;
- plus sûr ;
- plus accessible ;
- plus compréhensible par l’IA ;
- plus respectueux des ressources machine.

Le navigateur doit pouvoir compenser les mauvais choix techniques d’un site sans exiger que chaque développeur code lui-même toutes les optimisations.

## 2. Position sur la publicité

BezotCorp Browser ne doit pas être pensé comme un bloqueur de publicité agressif.

La publicité peut faire vivre :

- des créateurs ;
- des médias ;
- des outils gratuits ;
- des plateformes ;
- potentiellement BezotCorp elle-même.

La bonne approche n’est donc pas :

```txt
tout bloquer
```

mais plutôt :

```txt
autoriser la publicité acceptable,
neutraliser l’abus,
optimiser l’exécution,
empêcher le tracking excessif,
éviter les scripts destructeurs de performance.
```

Le navigateur peut donc viser un modèle de publicité propre :

- publicité non intrusive ;
- pas d’autoplay agressif ;
- pas de layout shift violent ;
- pas de tracking massif ;
- pas de scripts publicitaires lourds hors viewport ;
- rendu différé ou virtualisé des pubs hors écran ;
- priorité au contenu principal ;
- éventuel contrat publicitaire BezotCorp plus tard.

## 3. Problème rencontré avec Servo

Servo est un moteur web Rust intéressant, mais l’intégration actuelle via `servo = "0.2.0"` et `WindowRenderingContext` semble surtout adaptée à un exemple où Servo contrôle pratiquement toute la surface de fenêtre.

Notre tentative avec `softbuffer` a montré un problème concret :

```txt
softbuffer + Servo sur la même fenêtre = composition non fiable
```

Symptôme observé :

- la toolbar ne s’affiche pas durablement ;
- écran brièvement noir ;
- Servo repeint probablement par-dessus.

Conclusion :

```txt
Servo reste utile comme moteur web expérimental/fallback,
mais il ne doit pas forcément être le cœur obligatoire du shell navigateur.
```

Servo est utile pour :

- étudier un moteur web Rust ;
- tester l’embedding ;
- rendre des pages HTML/CSS/JS classiques ;
- servir de backend web possible.

Mais BezotCorp Browser doit garder le contrôle du shell, du chrome, du layout global, de l’IA, des optimisations et du rendu applicatif.

## 4. Nouvelle direction technique

La direction la plus cohérente est :

```txt
BezotCorp Browser
├── shell natif Rust contrôlé
├── chrome navigateur maison
├── renderer maison
├── moteur documentaire progressif
├── pipeline d’optimisation
├── IA intégrée
└── Servo optionnel comme backend web/fallback
```

Le cœur ne doit pas dépendre entièrement du fait que Servo accepte ou non qu’on dessine autour de lui.

L’architecture cible devient :

```txt
Input page / document
    ↓
Parsing
    ↓
DOM / Document Model interne
    ↓
Style Model
    ↓
Layout Model
    ↓
Optimization Pipeline
    ↓
Render Tree BezotCorp
    ↓
Renderer contrôlé
```

## 5. Modes de rendu

Le navigateur doit permettre plusieurs modes.

### 5.1 Faithful mode

Objectif : respecter au maximum le rendu prévu par le site.

Utilisation :

- compatibilité web ;
- sites complexes ;
- sites administratifs ;
- applications web sensibles.

### 5.2 Optimized mode

Objectif : garder le rendu proche de l’original, mais améliorer fortement les performances.

Optimisations possibles :

- virtualisation du DOM hors écran ;
- CSS visible uniquement ;
- scripts différés ;
- images priorisées ;
- animations limitées ;
- iframes hors viewport suspendues ;
- layout stabilisé.

### 5.3 Reader / AI mode

Objectif : extraire l’intention principale du site.

Utilisation :

- lecture ;
- résumé IA ;
- accessibilité ;
- recherche ;
- navigation assistée.

Ce mode peut transformer fortement le rendu, mais doit préserver le sens.

### 5.4 Secure mode

Objectif : exécuter le minimum.

Utilisation :

- sites inconnus ;
- navigation sensible ;
- consultation rapide ;
- protection forte.

## 6. Optimisations prévues

Le navigateur doit pouvoir optimiser automatiquement :

### 6.1 DOM

- ignorer temporairement les nœuds hors écran ;
- virtualiser les longues listes ;
- réduire les sous-arbres invisibles ;
- détecter le contenu principal ;
- différer les composants non critiques.

### 6.2 CSS

- parser les styles ;
- calculer uniquement ce qui est utile ;
- prioriser le above-the-fold ;
- ignorer ou retarder les règles inutiles ;
- stabiliser les layouts instables ;
- produire un style interne optimisé.

### 6.3 JavaScript

- distinguer JS critique et non critique ;
- retarder certains scripts ;
- limiter les boucles lourdes ;
- suspendre les scripts hors viewport ;
- isoler les comportements risqués.

### 6.4 Images et médias

- charger selon priorité viewport ;
- compresser ou réduire si nécessaire ;
- différer les médias hors écran ;
- bloquer autoplay agressif ;
- préserver les contenus utiles.

### 6.5 Publicité

- ne pas bloquer par défaut toute publicité ;
- différer les publicités hors écran ;
- empêcher les comportements intrusifs ;
- limiter tracking et scripts lourds ;
- favoriser une publicité compatible avec la performance.

## 7. Contrat futur pour les développeurs

À terme, BezotCorp pourrait proposer un contrat navigateur + moteur de recherche.

Idée :

```txt
Si votre site respecte le contrat BezotCorp,
il sera mieux compris, mieux optimisé,
mieux indexé et mieux rendu.
```

Ce contrat pourrait inclure :

- structure de contenu claire ;
- métadonnées sémantiques ;
- règles de performance ;
- publicité propre ;
- accessibilité ;
- déclarations d’intention UI ;
- compatibilité IA ;
- ressources critiques explicites.

Le moteur de recherche BezotCorp pourrait alors favoriser les sites :

- rapides ;
- lisibles ;
- optimisables ;
- non abusifs ;
- respectueux de l’utilisateur.

## 8. Rôle de Servo

Servo ne doit pas être supprimé immédiatement.

Rôle possible :

```txt
ServoBackend
→ backend web compatible
→ fallback HTML/CSS/JS
→ laboratoire d’intégration
→ comparaison avec notre pipeline maison
```

Mais il ne doit pas bloquer :

- le chrome maison ;
- le renderer maison ;
- les optimisations ;
- l’IA ;
- le modèle documentaire ;
- la stratégie produit.

Décision actuelle :

```txt
On garde Servo comme backend optionnel.
On n’investit plus dans le chrome Servo si la composition est bloquante.
On construit le vrai shell contrôlé BezotCorp.
```

## 9. Librairies Rust candidates

### Fenêtre et événements

- `winit` : fenêtre, événements clavier/souris, boucle d’application. Winit est une brique bas niveau de création de fenêtre et de gestion d’événements.

### GPU / rendu

- `wgpu` : API graphique Rust cross-platform basée sur WebGPU, utilisable sur Vulkan, Metal, D3D12, OpenGL, WebGPU/WebGL selon les plateformes.
- `vello` : rendu 2D vectoriel GPU, candidat pour chrome natif, formes, textes, panneaux.
- `tiny-skia` : rendu 2D CPU, utile pour fallback ou génération intermédiaire.
- `cosmic-text` : shaping et rendu texte.
- `parley` : layout texte riche.
- `glyphon` : rendu texte GPU avec `wgpu`.

### HTML / document

- `html5ever` : parser HTML5 issu de l’écosystème Servo, conçu pour parser du HTML selon les specs WHATWG/HTML5.
- `markup5ever` : briques communes autour du parsing markup.
- modèle DOM maison : probablement nécessaire pour garder le contrôle.

### CSS

- `lightningcss` : parser, transformeur et minifier CSS écrit en Rust, basé sur `cssparser`, capable de parser règles/propriétés/valeurs CSS dans des structures normalisées.
- `cssparser` : parsing CSS bas niveau.
- `selectors` : matching de sélecteurs CSS.

### JavaScript

- `boa` : moteur JavaScript Rust.
- `rquickjs` : binding QuickJS, utile si l’on accepte une dépendance C via QuickJS.
- phase initiale possible : JS limité ou désactivé selon mode.

### Réseau

- `reqwest` : client HTTP haut niveau.
- `hyper` : HTTP bas niveau.
- `url` : parsing et manipulation d’URL.
- `rustls` : TLS Rust.

### Images / SVG

- `image` : formats d’images raster.
- `resvg` / `usvg` : SVG.
- `fontdb` : découverte de polices.

### Stockage

- fichiers `.ron` : configuration lisible humain.
- fichiers `.bin` : cache rapide.
- `serde` : sérialisation.
- `ron` : configuration humaine.
- `bincode` ou équivalent : format binaire.
- `redb` ou `sled` : stockage local si besoin plus tard.

### IA / analyse

- connecteur IA local ;
- index sémantique de page ;
- extraction contenu principal ;
- résumé ;
- actions navigateur contrôlées ;
- mémoire locale éventuelle.

## 10. Architecture cible en dossiers

Proposition durable :

```txt
src/browser/
├── chrome/
│   ├── browser_layout.rs
│   ├── browser_toolbar.rs
│   ├── toolbar_action.rs
│   └── ...
│
├── document/
│   ├── document_model.rs
│   ├── document_node.rs
│   ├── document_tree.rs
│   └── ...
│
├── parser/
│   ├── html_parser.rs
│   ├── css_parser.rs
│   └── ...
│
├── style/
│   ├── style_rule.rs
│   ├── computed_style.rs
│   ├── selector_matcher.rs
│   └── ...
│
├── layout/
│   ├── layout_tree.rs
│   ├── layout_box.rs
│   ├── viewport.rs
│   └── ...
│
├── render/
│   ├── renderer.rs
│   ├── render_tree.rs
│   ├── render_command.rs
│   └── ...
│
├── optimization/
│   ├── render_policy.rs
│   ├── viewport_policy.rs
│   ├── script_policy.rs
│   ├── media_policy.rs
│   └── ...
│
├── engine/
│   ├── browser_engine.rs
│   ├── servo_engine.rs
│   ├── bezot_engine.rs
│   └── ...
│
├── navigation/
├── shortcuts/
├── state/
└── servo_app/
```

## 11. Règles de code

Règles actuelles à conserver :

- un fichier par struct ou enum ;
- pas de dossier `types/` générique ;
- `pub(crate)` accepté dans le binaire ;
- pas de tests dans les fichiers métier ;
- tests dans sous-dossiers `tests/` ;
- objectif : fichiers de production autour de 100 lignes ;
- refactor léger dès qu’un fichier grossit ;
- `.ron` pour lisibilité humaine ;
- `.bin` plus tard pour cache rapide ;
- ne pas créer de prototype jetable si on sait déjà qu’il faudra le supprimer.

## 12. Décision actuelle

La décision stratégique actuelle est :

```txt
Ne pas abandonner Servo brutalement.
Ne pas dépendre de Servo pour le shell principal.
Construire un moteur navigateur contrôlé, composé de briques Rust.
Garder Servo comme backend expérimental/fallback.
Créer un renderer/chrome maison pour contrôler l’expérience.
```

Priorité immédiate :

```txt
1. stabiliser la doc stratégique ;
2. nettoyer le test softbuffer si non viable ;
3. créer un backend BezotEngine minimal ;
4. poser document/render pipeline maison ;
5. garder ServoBackend séparé.
```

## 13. Résumé court pour reprise de contexte

BezotCorp Browser vise un navigateur Rust contrôlé, performant, sécurisé et assisté par IA.
Servo a permis de valider un rendu web Rust, mais son embedding actuel donne trop de contrôle à Servo sur la surface de fenêtre. Le chrome natif avec `softbuffer` sur la même fenêtre n’a pas composé correctement. La direction devient donc : garder Servo comme backend optionnel/fallback, mais construire le cœur du navigateur autour d’un shell, d’un renderer et d’un pipeline document/layout/optimisation maison en Rust.

Le navigateur ne doit pas bloquer toute publicité par principe. Il doit plutôt favoriser une publicité propre, performante et non intrusive.

Objectif long terme : le site fournit une intention, BezotCorp Browser choisit l’exécution optimale.
