# Contribuer

Lire docs/clean-room.md et NOTICE.md. Contributions sous MIT, en code original
ou issu de sources compatibles explicitement identifiées. Déclarer la provenance
du contrat, les tests et les limites dans la PR. Ne pas joindre de code traduit,
assets, dumps, sauvegardes ou captures du jeu.

Fixtures synthétiques uniquement. Le lecteur reste en lecture seule avec bornes
et allocations limitées. Le code Bevy reste séparé de la simulation sans GPU.

```sh
cargo fmt --all -- --check
cargo test --workspace --locked
cargo check -p godhand-lab --features viewer --locked
python scripts/dashboard.py --check
```

Après indexation : python scripts/check_public_tree.py. Actualiser le journal et
les tâches avec les résultats réellement observés. Un test d'un backend tiers
n'est pas un résultat du moteur.

Le dashboard utilise Pages gratuit. Ne pas ajouter de service payant, secret côté
navigateur, artefact volumineux ou données du jeu à la CI publique.
