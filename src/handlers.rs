use axum::{response::Html};
use std::time::Instant; // Importer Instant pour chronométrer
use crate::element::HtmlElement;
use rayon::prelude::*; // Import rayon's parallel iterator traits
use rayon::iter::ParallelIterator; // Explicitly import ParallelIterator


pub async fn root() -> Html<String> {
    let start = Instant::now(); // Démarrer le chronométrage

    let mut body: HtmlElement = HtmlElement::new("body");
    body
        .add_class("bg-blue-500");

    
    // body.get_children().reserve(1000);

    // for i in 0..10_000 {
    //     let mut p = HtmlElement::new("p");
    //     p.add_text(&format!("Ceci est le paragraphe numéro {}", i));
    //     body.add_child(p);
    // }


    // let children: Vec<_> = (0..10_000).into_par_iter()
    // .map(|i| {
    //     let mut p = HtmlElement::new("p");
    //     p.add_text(&format!("Ceci est le paragraphe numéro {}", i));
    //     p
    // })
    // .collect();
    // body.add_children(children);

    // let n_children = 10_000; // Nombre d'enfants à créer

    // let mut children = body.get_children(); 
    // children.reserve(n_children); 

    // (0..n_children)
    // .into_par_iter()
    // .map(|i| {
    //     let mut p = HtmlElement::new("div");
    //     p.add_style("display: flex; flex-direction: column; align-items: center;");
        
    //     let p_children = p.get_children(); // Obtenir le vecteur d'enfants de p
    //     p_children.reserve(3);

    //     (0..3).into_par_iter().map({
    //         |j| {
    //             let mut span = HtmlElement::new("span");
    //             span.add_text(&format!("Ceci est le span numéro {}", j));
    //             span
    //         }
    //     }).collect_into_vec(p_children);

    //     p
    // })
    // .collect_into_vec(&mut children); // Collecte directement dans le vecteur préalloué.
    
    


    println!("New request received");

    let title = "Bienvenue sur mon site";
    
    let response = Html(format!(
        r#"
<!DOCTYPE html>
<html lang="fr">
    <head>
        <title>{}</title>
        <link rel="stylesheet" href="/static/global.css">
    </head>
   {}
</html>
            "#,
        title,
        body.render()
    ));

    let duration = start.elapsed(); // Calculer le temps écoulé
    println!("La requête a pris {:?} secondes", duration); // Affiche la durée

    response
}
