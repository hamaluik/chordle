days-ago-prefix = il y a
days-ago-number = { $days ->
    [-1] ∞
     *[other] { $days }
}
days-ago-suffix = { $days ->
    [1] jour
     *[other] jours
}
due-today = (à rendre aujourd'hui)
due-ago = (dû { $days ->
        [1] hier
         *[other] il y a { $days } jours
    })
due-in = ({ $days ->
        [1] à rendre demain
         *[other] dû dans { $days } jours
    })
undo = Défaire
redo = Refaire
manage-chores = Gérer les tâches ménagères
invalid-chore-name = Nom de tâche non valide, ne doit pas être vide et ≤ 160 caractères.
invalid-interval = Intervalle non valide, voir { $link } pour obtenir de l'aide sur le formatage.
save = Sauvegarder
delete = Supprimer
name = Nom
name-placeholder = par exemple « Nettoyer la cuisine »
interval = Intervalle
history = Histoire
create = Créer
chore-created = Tâche créée avec succès !
failed-to-create-chore = Échec de la création de la corvée…
new-chore = Nouvelle corvée
chores = Corvées
back-to-chores = ← Retour aux tâches ménagères
chordle-source-code = Code source de chordle ↗
settings = Paramètres
language = Langue
