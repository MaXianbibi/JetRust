use axum::response::Html;
use std::time::Instant;
use std::fs;

// Importer Instant pour chronométrer
use crate::element::HtmlElement;
use rayon::iter::ParallelIterator;
use rayon::prelude::*; // Import rayon's parallel iterator traits // Explicitly import ParallelIterator

fn find_js_wasm_file() -> String {
    let path = "static/mon_projet.js";
    if fs::metadata(path).is_ok() {
        path.to_string()
    } else {
        "static/mon_projet.wasm".to_string()
    }
}


pub async fn root() -> Html<String> {
    let start = Instant::now(); // Démarrer le chronométrage

    let mut body: HtmlElement = HtmlElement::new("body");
    body.add_class("bg-neutral-800 w-screen h-screen flex justify-center flex-col h-fit");

    let header: &mut HtmlElement = body.add_child(HtmlElement::new("header"))
        .add_class("flex flex-col items-center justify-center min-h-[300px] w-full bg-neutral-900 border-b-2 border-neutral-700");

    let mut h1: HtmlElement = HtmlElement::new("h1");
    h1.add_text("JetRust").add_class("text-8xl text-white font-bold");
    header.add_child(h1);


    let mut button: HtmlElement = HtmlElement::new("button");
    button.add_text("Cliquez ici")
            .add_class("bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded")
            .add_event("onclick", "alert('Vous avez cliqué sur le bouton !')");

    header.add_child(button);


    let mut div: HtmlElement = HtmlElement::new("div");
    div.add_class("
    text-white
    flex flex-col items-center justify-start  w-full bg-neutral-900 border-b-2 border-neutral-700 overflow-y-scroll");
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
    
        <script type="module">
            import init from "/static/mon_projet.js";
            init().then(() => {{
                console.log("WASM chargé !");
            }}).catch(console.error);
        </script>
    </html>
        "#,
        title,
        body.render()
    ));
    

    let duration = start.elapsed(); // Calculer le temps écoulé
    println!("La requête a pris {:?} secondes", duration); // Affiche la durée

    response
}


