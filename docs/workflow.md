# Workflow du moteur indépendant

Lire clean-room.md et architecture.md. L'ancien script ghidra_analyze_retail.sh et
les prompts 01–05 concernent la recherche locale ; leurs traductions n'entrent pas
dans le moteur public.

## Spécifier puis implémenter

Choisir une tâche de docs/dashboard/data.json, écrire son contrat dans specs/ avec
provenance et critères. Ne pas transmettre le code du jeu à l'implémenteur.

```sh
cargo test --workspace --locked
cargo check -p godhand-lab --features viewer --locked
cargo run -p godhand-lab -- --iso "/chemin/God Hand (USA).iso"
cargo run -p godhand-lab --features viewer -- --iso "/chemin/God Hand (USA).iso" --sandbox
```

Tests synthétiques uniquement. Le lancement lit l'ISO sans la modifier, reconnaît le
numéro de série USA et n'exécute pas l'ELF. Le sandbox ne contient que nos primitives.

Machine actuelle : utiliser cargo +stable-x86_64-pc-windows-gnu avec MinGW sur le PATH.
L'ancien chemin MSVC est absent. Les backends de recherche sont épinglés dans
toolchains.lock.json ; le moteur utilise Cargo.lock.

## Mettre à jour le suivi

Modifier tâches et critères dans docs/dashboard/data.json. Ajouter au journal les
changements, validations réellement effectuées et limites.

```sh
python scripts/dashboard.py
python scripts/dashboard.py --check
```

Après un changement visuel, capturer le dashboard affiché puis renseigner la date,
la provenance et la légende dans captures. Les images du jeu restent locales.
Ne pas confondre capture statique, résultat d'un backend tiers et gameplay du moteur.

## Publier

Indexer explicitement les sources publiques, puis :

```sh
python scripts/check_public_tree.py
git diff --cached --stat
git push origin main
```

GitHub Pages gratuit publie main:/docs, avec .nojekyll. Aucun service Sites, domaine
payant ou artefact personnalisé n'est utilisé. Les contrôles tournent sur Ubuntu
standard public, sans cache ni upload. Vérifier CI et URL après chaque push.

Disques, extractions, outils, Ghidra, sorties de recomp et captures du jeu restent
locaux. Avant prototype/diffing, lire docs/prototype-tcrf-notes.md (local, non publié).
