# Contexte maître — moteur indépendant

Répondre en français. God Hand USA / SLUS_215.03 : **moteur indépendant Rust + Bevy
0.20.1**, MIT pour le code original. Décision utilisateur du 2026-11-03 : code public
original, aucune traduction du binaire PS2. L'ISO personnelle est requise au lancement.

Lire AGENTS.md, docs/workflow.md, docs/clean-room.md, docs/architecture.md et ROADMAP.md.
Les prompts 01–05 sont des références historiques de recherche locale uniquement.

Socle : crates/disc (ISO en lecture seule), crates/engine (simulation originale),
crates/app (CLI et sandbox Bevy optionnel). Le jeu n'est pas jouable. Y = hauteur.
Tests synthétiques seulement en CI. Ne pas importer de code décompilé ou traduit.

Chaque session met à jour docs/dashboard/data.json : tâches, modifications, tests
réels et limites. Régénérer la roadmap avec python scripts/dashboard.py.
Capturer le dashboard après modifications visuelles. Vérifier l'index avec
python scripts/check_public_tree.py. Le dashboard se publie sur Pages gratuit
depuis main:/docs, sans hébergement externe payant.
