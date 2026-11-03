# Premier essai local de ps2-native-recomp

Date : 3 novembre 2026. Il s'agit d'un test du backend tiers
[ps2-native-recomp](https://github.com/exoteric33/ps2-native-recomp), **pas d'un
résultat du moteur indépendant Rust/Bevy**.

## Résultat observé

Le build local démarre God Hand USA, affiche l'écran titre, lance la nouvelle
partie, joue la cinématique puis atteint le premier combat. Les ennemis sont
actifs et le HUD est présent. Un état du runtime enregistré pendant ce combat
peut être chargé dans un nouveau processus ; le combat continue ensuite.

Ce résultat confirme que le projet mérite un essai sur notre exemplaire du jeu.
Il ne valide ni le casino rapporté par un autre testeur, ni une partie complète.

## Conditions

- Backend inchangé au commit `cba42f57913d181b3ff2fd3064ca48ab0cbeea11`.
- Disque USA fourni localement, boot `SLUS_215.03`, identifiant de patch `6FB69282`.
- Aucun patch PNACH, changement de ratio ou correctif spécifique au jeu appliqué.
- Windows 11, Core i7-14700KF, RTX 5080, 32 Go de RAM.
- Backend graphique Direct3D 12 avec DXC, résolution interne par défaut.
- Outils portables llvm-mingw, CMake et Ninja ; dépendances téléchargées avec les
  sommes de contrôle épinglées dans les scripts amont.

Le PATH du premier essai exposait un ccache externe dont le chemin était mal
interprété sous MinGit. Le build a été relancé avec un PATH isolé, sans ccache,
après suppression de cette option dans le cache CMake local. Aucun fichier source
du backend n'a été modifié pour obtenir le résultat décrit ici.

## Cas exécutés

| Cas | Observation | Limite |
| --- | --- | --- |
| Construction | Traducteurs, runtime et runner compilés ; 265 unités de traduction émises | Le code traduit reste privé |
| Exploration automatique | 1 800 frames, sortie normale ; titre visible dans une capture | Une capture finale noire ne suffisait pas à conclure sur le démarrage |
| Essai en fenêtre avec captures | 7 200 frames en 122,44 s de temps total ; cinématique et premier combat observés | Les captures forcent le mode à un thread de cette version |
| Fenêtre normale, caches VU déjà présents | 4 800 frames en 81,51 s ; état enregistré à la frame 4 500 ; sortie normale | Inclut démarrage, menus et cinématique ; pas un benchmark de combat |
| Reprise et commandes programmées | État chargé à 4 500, poursuite jusqu'à 6 300 ; nouvel état enregistré à 6 200 | Actions analogiques et appuis programmés ; clavier et manette physiques non validés |

Les captures de reprise montrent un changement d'orientation/cadrage après la
commande analogique, des échanges de coups et des variations des jauges de vie.
Cela ne mesure pas l'exactitude des contrôles ou du combat face à la console.

Pendant la première session en fenêtre, le runtime a compilé 31 programmes VU
supplémentaires. Le deuxième démarrage les a chargés depuis son cache. La voie
testée utilise donc bien de la compilation à l'exécution ; elle ne démontre pas
une couverture AOT complète de tous les chemins du jeu.

## Performances et réserves

Les titres de fenêtre observés pendant l'ouverture indiquaient environ 60 FPS,
avec une lecture ponctuelle à 28,6 FPS lors d'une transition. Ce sont des
observations instantanées, pas une courbe de frametimes ni une mesure contrôlée.
Les temps totaux du tableau incluent le lancement et ne prouvent pas un framerate
stable dans tout le jeu. Aucun comparatif PCSX2 n'a encore été effectué.

Le journal audio du test normal signale une seule rupture de file, environ
0,60 seconde de silence au total, et aucun échantillon abandonné. Cela ne remplace
pas une écoute comparative. Des avertissements SIF répétés sont présents malgré
la progression observée. Leur effet sur les étapes ultérieures n'est pas établi.

La pause n'est pas encore validée dans ce bilan. La sauvegarde testée est un
snapshot du runtime, **pas la sauvegarde native du jeu sur carte mémoire** : les
journaux ne montrent aucune écriture de page de carte pendant ces essais.

Deux essais de Start sur l'état rechargé n'ont pas permis d'identifier clairement
le menu pause. Les captures ultérieures montrent une transition et du texte de
continuation après des dégâts subis. Ce test est donc non concluant ; il ne suffit
pas à déclarer un blocage du menu ou un défaut de rendu.

## Conséquence pour la suite

La piste est désormais vérifiée localement jusqu'au premier combat. Elle est
utile comme référence exécutable et comme option de jeu expérimentale. Elle
n'implémente pas un moteur original et ne change pas le périmètre public MIT du
projet Rust/Bevy. Ne pas copier ses traductions dans ce moteur.

Prochains tests utiles : pause et menus, commandes physiques, sauvegarde/chargement
dans le jeu, session plus longue, puis comparaison sur une séquence identique avec
PCSX2. Un seul de ces tests ne vaut pas validation des autres.

## Données conservées localement

L'ISO, les modules extraits, le C++ généré, le runner lié au jeu, les états mémoire,
les journaux bruts et les images de gameplay restent exclus de Git. Le carnet
public ne reçoit que ce bilan textuel. Les captures publiques restent celles du
carnet lui-même.

L'empreinte SHA-256 de l'ISO est identique avant et après les essais : le disque
source n'a pas été modifié.
