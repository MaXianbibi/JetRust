use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;
use uuid::Uuid;



#[derive(Debug, Clone)]
/// Represents an HTML element with its associated properties.
///
/// # Fields
/// - `tag_name`: The name of the HTML tag (e.g., `div`, `span`, `p`).
/// - `text`: The inner text content of the HTML element.
/// - `attributes`: A collection of key-value pairs representing the attributes of the element (e.g., `class`, `style`).
/// - `children`: A list of child `HtmlElement` objects nested within this element.
/// - `id`: The unique identifier for the HTML element.
pub struct HtmlElement {
    id: String,
    tag_name: String,
    text: String,
    attributes: HashMap<String, String>, 
    children: Vec<HtmlElement>,         
}

impl HtmlElement {
    pub fn new(tag_name: &str) -> Self {
        HtmlElement {
            id: Uuid::new_v4().to_string(),
            tag_name: tag_name.to_string(),
            text: String::new(),
            attributes: HashMap::new(), // Initialisé à vide
            children: Vec::new(),       // Initialisé à vide
        }
    }

    pub fn render(&self) -> String {
        let attributes_str = self
            .attributes
            .iter()
            .map(|(key, value)| format!(r#"{}="{}" "#, key, value))
            .collect::<String>();

        let children_render: String = self
            .children
            .par_iter() 
            .map(|child| child.render())
            .collect();

        format!(
            "<{} {}>{} {}</{}>",
            self.tag_name, attributes_str, self.text, children_render, self.tag_name
        )
    }

    pub fn add_text(&mut self, text: &str) -> &mut Self {
        self.text = text.to_string();
        self
    }

    pub fn add_event(&mut self, event: &str, callback: &str) -> &mut Self {
        self.attributes.insert(event.to_string(), callback.to_string());
        self
    }



    pub fn add_class(&mut self, class: &str) -> &mut Self {
        self.attributes
            .insert("class".to_string(), class.split_whitespace().collect::<Vec<_>>().join(" "));
        self
    }

    pub fn add_style(&mut self, style: &str) -> &mut Self {
        self.attributes
            .insert("style".to_string(), style.to_string());
        self
    }

    pub fn add_child(&mut self, child: HtmlElement) -> &mut HtmlElement {
        self.children.push(child);
        self.children.last_mut().unwrap()
    }

    pub fn add_children(&mut self, children: Vec<HtmlElement>) {
        self.children.extend(children); // Utilise extend pour ajouter plusieurs enfants
    }

    pub fn get_child_mut(&mut self, child_id: &str) -> &mut HtmlElement {
        // Cherche d'abord l'enfant
        if let Some(child) = self.children.iter_mut().find(|child| child.id == child_id) {
            return child; // Si trouvé, retourne la référence mutable
        }
        panic!("L'enfant avec l'ID {} n'existe pas", child_id); // Si l'enfant n'est pas trouvé, on panique
        // Si l'enfant n'est pas trouvé, on crée et ajoute un nouvel enfant
    }

    pub fn get_children(&mut self) -> &mut Vec<HtmlElement> {
        &mut self.children
    }
}
