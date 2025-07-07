use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::env;
use std::time::Instant;

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_complete_term: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_complete_term: false,
        }
    }

    fn insert(&mut self, term: &str) {
        let mut node = self;
        for ch in term.chars() {
            node = node.children.entry(ch).or_insert(TrieNode::new());
        }
        node.is_complete_term = true;
    }

    fn search(&self, term: &str) -> bool {
        let mut node = self;
        let mut path = String::new();
        let mut path_size = 0;
    
        for ch in term.chars() {
            path.push(ch);
            path.push(' ');
            path_size += 1;

    
            match node.children.get(&ch) {
                Some(next) => node = next,
                None => {
                    println!("Símbolo '{}' não encontrado. Termo não existe.", ch);
                    println!("Caminho percorrido até falha: {}", path);
                    return false;
                }
            }
        }
        println!("Termo encontrado:  {}", node.is_complete_term);
        println!("Caminho percorrido: {}, Passos no caminho: {}", path, path_size);
        node.is_complete_term
    }

    fn starts_with(&self, prefix: &str) -> bool {
        let mut node = self;
        for ch in prefix.chars() {
            match node.children.get(&ch) {
                Some(next) => node = next,
                None => {
                    println!("Prefixo '{}' não encontrado na Trie.", prefix);
                    return false;
                }
            }
        }

        println!("Termos que começam com '{}':", prefix);
        Self::print_all_from_node(node, prefix.to_string());

        true
    }

    fn print_all_from_node(node: &TrieNode, prefix: String) {
        if node.is_complete_term {
            let spaced: String = prefix.chars().map(|c| c.to_string()).collect::<Vec<_>>().join(" ");
            println!("{}", spaced);
            // format!("{}", spaced);
        }

        for (ch, child_node) in &node.children {
            let mut new_prefix = prefix.clone();
            new_prefix.push(*ch);
            Self::print_all_from_node(child_node, new_prefix);
        }
    }

    fn insert_from_file<P: AsRef<Path>>(path: P, trie: &mut TrieNode) -> io::Result<()> {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
    
        for line_result in reader.lines() {
            let line = line_result?;
            let term: String = line.split_whitespace().collect();
            if !term.is_empty() {
                trie.insert(&term);
            }
        }
    
        Ok(())
    }
}

fn insert_into_vec<P: AsRef<Path>>(path: P, list: &mut Vec<String>) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;

        let term: String = line.split_whitespace().collect();

        if !term.is_empty() {
            list.push(term);
        }
    }

    Ok(())
}

fn search_in_vec(list: &[String], term: &str) -> bool {

    for (i, t) in list.iter().enumerate() {
        if t == term {
            println!("Termo '{}' encontrado na posição {}", term, i);
            return true;
        }
    }

    println!("Termo '{}' não encontrado", term);
    false
}

fn search_prefix_in_vec(list: &[String], prefix: &str) -> Vec<String> {
    let mut resultados = Vec::new();

    println!("Buscando termos que começam com '{}':", prefix);
    // format!("Buscando termos que começam com '{}':", prefix);

    for t in list {
        if t.starts_with(prefix) {
            println!("{}", t);
            // format!("{}", t);

            resultados.push(t.clone());
        }
    }

    if resultados.is_empty() {
        println!("Nenhum termo encontrado com prefixo '{}'.", prefix);
    }

    // println!("Quantidade resultados: {}" resultados.len());
    resultados
}




fn main() {
    let mut root = TrieNode::new();
    let mut vec: Vec<String> = Vec::new();

    let search_term : String = "h g h h c d d f b b h c d b h a c a b".split_whitespace().collect();
    let search_prefix : String = "f g h d a ".split_whitespace().collect();

    TrieNode::insert_from_file("../../termos.txt", &mut root);
    insert_into_vec("../../termos.txt", &mut vec);
    


    let inicio_busca_unica_trie = Instant::now();
    root.search(&search_term);
    let duracao_busca_unica_trie = inicio_busca_unica_trie.elapsed();


    let inicio_busca_unica_vec = Instant::now();
    search_in_vec(&vec, &search_term);
    let duracao_busca_unica_vec = inicio_busca_unica_vec.elapsed();

    let inicio_busca_prefixo_trie= Instant::now();
    root.starts_with(&search_prefix);
    let duracao_busca_prefixo_trie = inicio_busca_unica_trie.elapsed();



    let inicio_busca_prefixo_vec = Instant::now();
    search_prefix_in_vec(&vec, &search_prefix);
    let duracao_busca_prefixo_vec = inicio_busca_prefixo_vec.elapsed();

    println!("Testes busca única: ");

    println!("(Trie)Tempo decorrido: {:?}", duracao_busca_unica_trie.as_micros());
    println!("(Trie)Tempo decorrido: {:?}", duracao_busca_unica_trie.as_millis());


    println!("(Vec)Tempo decorrido: {:?}", duracao_busca_unica_vec.as_micros());
    println!("(Vec)Tempo decorrido: {:?}", duracao_busca_unica_vec.as_millis());

    println!("############################################");

    println!("Testes busca com prefixo: ");

    println!("(Trie)Tempo decorrido: {:?}", duracao_busca_prefixo_trie.as_micros());
    println!("(Trie)Tempo decorrido: {:?}", duracao_busca_prefixo_trie.as_millis());


    println!("(Vec)Tempo decorrido: {:?}", duracao_busca_prefixo_vec.as_micros());
    println!("(Vec)Tempo decorrido: {:?}", duracao_busca_prefixo_vec.as_millis());

    println!("############################################");





}
