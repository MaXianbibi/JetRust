use axum::response::Html;
use std::time::Instant;

// Importer Instant pour chronométrer
use crate::element::HtmlElement;
use rayon::iter::ParallelIterator;
use rayon::prelude::*; // Import rayon's parallel iterator traits // Explicitly import ParallelIterator

pub async fn root() -> Html<String> {
    let start = Instant::now(); // Démarrer le chronométrage

    let mut body: HtmlElement = HtmlElement::new("body");
    body.add_class("bg-neutral-800 w-screen h-screen flex justify-center flex-col h-fit");
    let header: &mut HtmlElement = body.add_child(HtmlElement::new("header"))
        .add_class("flex flex-col items-center justify-center min-h-[300px] w-full bg-neutral-900 border-b-2 border-neutral-700");

    let mut h1: HtmlElement = HtmlElement::new("h1");
    h1.add_text("JetRust")
        .add_class("text-8xl text-white font-bold");
    header.add_child(h1);

    let mut div: HtmlElement = HtmlElement::new("div");
    div.add_class("
    text-white
    flex flex-col items-center justify-start  w-full bg-neutral-900 border-b-2 border-neutral-700 overflow-y-scroll");
    // for i in 0..10_000 {
    //     let mut p = HtmlElement::new("p");
    //     p.add_text(&format!("Ceci est le paragraphe numéro {}", i));
    //     body.add_child(p);
    // }

    body.get_children().reserve(1000);

    let children: Vec<_> = (0..1000)
        .into_par_iter()
        .map(|i| {
            let mut p = HtmlElement::new("p");
            p.add_text(&format!("Ceci est le paragraphe numéro {}", i));
            p
        })
        .collect();
    div.add_children(children);
    body.add_child(div);

    // let n_children = 1; // Nombre d'enfants à créer

    // let mut children = body.get_children();
    // children.reserve(n_children );

    // (0..n_children)
    // .into_par_iter()
    // .map(|i| {
    //     let mut div = HtmlElement::new("div");
    //     div.add_class("bg-neutral-900 border-b-2 ").add_text("where am i ?");
    //     div
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
        <link rel="stylesheet" href="/static/css/output.css">
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
