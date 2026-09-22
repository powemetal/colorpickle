use chrono::Utc;
use color_picker_lib::palettes::couleur::Couleur;
use color_picker_lib::palettes::palette::Palette;
use color_picker_lib::palettes::palettes::Palettes;
use uuid::Uuid;

// Fonction d'aide pour générer une couleur valide rapidement dans les tests
fn creer_couleur_test(nom: &str, r: u8, g: u8, b: u8) -> Couleur {
    Couleur::new(
        Uuid::new_v4().to_string(),
        nom.to_string(),
        r,
        g,
        b,
        format!("#{:02X}{:02X}{:02X}", r, g, b),
        Utc::now().naive_utc(),
    )
}

#[test]
fn test_creer_palette() {
    let mut palettes = Palettes::new(vec![]);

    let nouvelle = Palette::new("Thème Sombre".to_string(), vec![]);
    let id = nouvelle.id().to_string();
    palettes.ajouter_palette(nouvelle);

    assert_eq!(palettes.get_palettes().len(), 1);
    assert_eq!(palettes.trouver_palette(&id).unwrap().nom(), "Thème Sombre");
}

#[test]
fn test_supprimer_palette() {
    let mut palettes = Palettes::new(vec![]);
    let palette = Palette::new("À Effacer".to_string(), vec![]);
    let id = palette.id().to_string();
    palettes.ajouter_palette(palette);

    // 1. Suppression réussie
    let resultat = palettes.supprimer_palette(&id);
    assert!(resultat.is_ok());
    assert_eq!(palettes.get_palettes().len(), 0);

    // 2. Erreur si on essaie de supprimer une palette inexistante
    let resultat_inexistant = palettes.supprimer_palette(&id);
    assert!(resultat_inexistant.is_err());
}

#[test]
fn test_modifier_nom_palette() {
    let mut palettes = Palettes::new(vec![]);
    let palette = Palette::new("Ancien Nom".to_string(), vec![]);
    let id = palette.id().to_string();
    palettes.ajouter_palette(palette);

    // Modification avec trouver_palette
    let palette_mut = palettes.trouver_palette(&id).expect("Palette introuvable");
    assert!(palette_mut.set_nom("Nouveau Nom").is_ok());

    // Vérification
    assert_eq!(palettes.trouver_palette(&id).unwrap().nom(), "Nouveau Nom");

    // Cas d'erreur : palette inconnue
    let palette_inconnue = palettes.trouver_palette("id-bidon");
    assert!(palette_inconnue.is_none());
}

#[test]
fn test_ajouter_couleur() {
    let mut palettes = Palettes::new(vec![]);
    let palette = Palette::new("Web Design".to_string(), vec![]);
    let id_palette = palette.id().to_string();
    palettes.ajouter_palette(palette);

    let couleur = creer_couleur_test("Bleu Primaire", 0, 122, 255);
    let id_couleur = couleur.id().to_string();

    let target = palettes.trouver_palette(&id_palette).unwrap();
    assert!(target.ajouter_couleur(couleur).is_ok());

    let palette_verif = palettes.trouver_palette(&id_palette).unwrap();
    assert_eq!(palette_verif.couleurs().len(), 1);
    assert_eq!(palette_verif.couleurs()[0].id(), id_couleur);
}

#[test]
fn test_supprimer_couleur() {
    let couleur = creer_couleur_test("Rouge Alerte", 255, 59, 48);
    let id_couleur = couleur.id().to_string();

    let palette = Palette::new("Palette Test".to_string(), vec![couleur]);
    let id_palette = palette.id().to_string();

    let mut palettes = Palettes::new(vec![palette]);

    // Suppression de la couleur existante
    let target = palettes.trouver_palette(&id_palette).unwrap();
    assert!(target.supprimer_couleur(&id_couleur).is_ok());
    assert_eq!(target.couleurs().len(), 0);

    // Tentative de suppression d'une couleur inexistante
    assert!(target.supprimer_couleur(&id_couleur).is_err());
}

#[test]
fn test_simulation_workflow_complet() {
    // Simule la séquence exacte exécutée par tes commandes
    let mut palettes = Palettes::new(vec![]);

    // 1. creer_palette
    let p = Palette::new("Projet Mobile".to_string(), vec![]);
    let id_p = p.id().to_string();
    palettes.ajouter_palette(p);

    // 2. ajouter_couleur
    let c1 = creer_couleur_test("Vert Succès", 52, 199, 89);
    let id_c1 = c1.id().to_string();
    palettes
        .trouver_palette(&id_p)
        .unwrap()
        .ajouter_couleur(c1)
        .unwrap();

    // 3. modifier_nom_palette
    palettes
        .trouver_palette(&id_p)
        .unwrap()
        .set_nom("Projet UI Mobile")
        .unwrap();

    // 4. Verifications
    let palette_finale = palettes.trouver_palette(&id_p).unwrap();
    assert_eq!(palette_finale.nom(), "Projet UI Mobile");
    assert_eq!(palette_finale.couleurs().len(), 1);

    // 5. supprimer_couleur
    palettes
        .trouver_palette(&id_p)
        .unwrap()
        .supprimer_couleur(&id_c1)
        .unwrap();
    assert_eq!(palettes.trouver_palette(&id_p).unwrap().couleurs().len(), 0);

    // 6. supprimer_palette
    palettes.supprimer_palette(&id_p).unwrap();
    assert!(palettes.trouver_palette(&id_p).is_none());
}
