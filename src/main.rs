use std::collections::HashMap;
use std::fmt::{Debug};


/// Eine Node in einem Baum
#[derive(Debug)]
struct Node {
    value: Option<char>,        // kann ein Buchstabe sein oder None
    amount: i32,                // wie oft kommt der Buchstabe vor
    right: Option<Box<Node>>,   // kann eine Node sein oder None
    left: Option<Box<Node>>     // kann eine Node sein oder None
}

impl Node {
    fn is_leaf(&self) -> bool { // hat die Node unter sich Nodes?
        self.right.is_none() && self.left.is_none()
    }
}

/// Zählt die Anzahl der Buchstaben in einem String und gibt eine Reihung von Nodes zurück
fn count_chars(input: &str) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();
    for char in input.chars() { // für jeden Buchstaben
        let mut found = false;  // wurde der Buchstabe schon gefunden?
        for node in &mut nodes { // für jede Node in der Reihung
            if Some(char) == node.value { // wenn der Buchstabe der Node entspricht
                node.amount += 1;
                found = true;
                break;
            }
        }
        if !found { // wenn der Buchstabe noch nicht gefunden wurde
            nodes.push(Node { value: Some(char), amount: 1, right: None, left: None })
        }
    }

    nodes
}

/// Erstellt einen Huffman-Baum aus einer Reihung von Nodes
fn create_tree(nodes: &mut Vec<Node>) -> Node {
    while nodes.len() != 1 { // solange es mehr als eine Node gibt
        nodes.sort_by(|a, b| b.amount.cmp(&a.amount)); // sortiere die Nodes nach der Anzahl der Buchstaben
        let first = nodes.pop().unwrap(); // nimm die Node mit der geringsten Anzahl
        let second = nodes.pop().unwrap(); // nimm die Node mit der zweitgeringsten Anzahl
        nodes.push(Node { value: None, amount: first.amount + second.amount, right: Some(Box::new(first)), left: Some(Box::new(second))})
        // Neue Node über den beiden Nodes
    }
    nodes.pop().unwrap() // am Ende gibt es nur eine Node, die den ganzen Baum enthält
}

fn encode_text<T: AsRef<str>>(text: T) -> (Node, String) {
    let root = create_tree(&mut count_chars(text.as_ref()));
    let mut result: String = String::new();
    let mut values: HashMap<char, String> = HashMap::new();
    generate_huffman_tree_map(&root, &mut values, String::new()); // rekursiv die Werte für jeden Buchstaben generieren
    for char in text.as_ref().chars() {
        result.push_str(values.get(&char).unwrap()); // den Wert für jeden Buchstaben anhängen
    }
    (root, result)
}

/// Rekusive Funktion, die die Werte für jeden Buchstaben generiert
fn generate_huffman_tree_map(node: &Node, values: &mut HashMap<char, String>, current: String) {
    if node.is_leaf() {
        values.insert(node.value.unwrap(), current);
    } else {
        if let Some(left) = &node.left {
            generate_huffman_tree_map(left, values, current.clone() + "0"); // links ist 0
        }
        if let Some(right) = &node.right {
            generate_huffman_tree_map(right, values, current + "1"); // rechts ist 1
        }
    }
}

/// Dekodiert eine Eingabe mit einem Huffman-Baum
fn decode_text<T: AsRef<str>>(tree: &Node, input: &T) -> String {
    let mut output = String::new();
    let mut current_node = tree;
    for char in input.as_ref().chars() { // für jeden Buchstaben
        if char == '1'{
            if let Some(node) = &current_node.right { // wenn rechts eine Node ist
                current_node = node; // gehe nach rechts
            }
        } else if let Some(node) = &current_node.left {
            current_node = node; // gehe nach links
        }
        if current_node.is_leaf() {
            output.push(current_node.value.unwrap()); // Buchstaben zur Ausgabe hinzufügen
            current_node = &tree; // zurück zum Anfang
        }
    }
    output
}


fn main() {
    //let raw = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!\"§$%&/()=?`´*+~#'-_.:,;<>|@€{[]}\\";
    let  raw = "Hello World!";
    let encoded = encode_text(raw);
    let root = encoded.0;
    let encoded_text = encoded.1;
    let ascii: String = raw.chars().map(|c| format!("{:b}", c as u8)).collect::<Vec<String>>().join("");
    println!("Encoded: {} {}", encoded_text, encoded_text.len());
    println!("ASCII: {} {}", ascii, ascii.len());
    println!("Saved: {}%", 100.0 - (encoded_text.len() as f32 / ascii.len() as f32 * 100.0));
    println!("Decoded: {}", decode_text(&root, &encoded_text));
}
