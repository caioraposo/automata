use std::collections::HashMap;

/// Pushdown Automata
/// A HashMap a seguir possui as seguintes caracteristicas:
/// como chave uma tupla: (estado_atual, char_atual)
/// e valores associados: (desempilhar, proximo_estado, andar_para)
/// A linguagem será validada se a pilha estiver vazia
fn main() {
    // Read input file and ignore empty spaces
    let tape = include_str!("../input")
        .trim()
        .chars()
        .filter(|x| *x != ' ')
        .collect();

    let initial_state = "q0";
    let final_state = "q0";
    let mut PDA = HashMap::new();
    // estado_atual char_lido   desempilhar prox_estado empilhar
    PDA.insert(("q0", 'a'), (' ', "q0", ' '));
    PDA.insert(("q0", 'b'), (' ', "q0", ' '));
    PDA.insert(("q0", 'e'), (' ', "q2", 'E'));
    PDA.insert(("q0", 'i'), (' ', "q1", 'C'));
    PDA.insert(("q0", 'u'), ('U', "q2", ' '));
    PDA.insert(("q0", 'r'), (' ', "q5", 'U'));

    PDA.insert(("q1", 'a'), (' ', "q1", ' '));
    PDA.insert(("q1", 'b'), (' ', "q1", ' '));
    PDA.insert(("q1", ';'), (' ', "q1", ' '));
    PDA.insert(("q1", 't'), ('C', "q0", ' '));

    PDA.insert(("q2", '['), (' ', "q2", 'B'));
    PDA.insert(("q2", 'x'), (' ', "q3", ' '));
    PDA.insert(("q2", 'y'), (' ', "q3", ' '));

    PDA.insert(("q3", ']'), ('B', "q3", ' '));
    PDA.insert(("q3", '+'), (' ', "q4", ' '));
    PDA.insert(("q3", 'r'), (' ', "q5", 'U'));
    PDA.insert(("q3", 'f'), ('E', "q5", ' '));
    PDA.insert(("q3", 'a'), (' ', "q0", ' '));
    PDA.insert(("q3", 'b'), (' ', "q0", ' '));
    PDA.insert(("q3", 'i'), (' ', "q1", 'C'));

    PDA.insert(("q4", '['), (' ', "q4", 'B'));
    PDA.insert(("q4", 'x'), (' ', "q3", ' '));
    PDA.insert(("q4", 'y'), (' ', "q3", ' '));

    // Duplicate from q3
    PDA.insert(("q5", 'a'), (' ', "q0", ' '));
    PDA.insert(("q5", 'b'), (' ', "q0", ' '));
    PDA.insert(("q5", 'i'), (' ', "q1", 'C'));
    
    compute(tape, PDA, initial_state, final_state, 0);
}

// Será imprimido:
// [fita]
// <estado_atual> <posicao_na_fita> <ação>
//
// Ex: [' ', 'A', 'A', 'B', 'b', ' ']
//     "q1" 3 ('B', "q1", Right)
fn compute(
    mut tape: Vec<char>,
    PDA: HashMap<(&str, char), (char, &str, char)>,
    initial_state: &str,
    final_state: &str,
    start_from: usize,
) {
    let mut stack = Vec::new();
    let mut pos = start_from;
    let mut current_state = initial_state;
    let mut current_char = tape[pos];

    // Enquanto houver transicao entre o estado atual e o próximo
    while let Some(transition) = PDA.get(&(current_state, current_char)) {
        println!("{:?}", tape);
        println!("{:?} {} {:?}", current_state, pos, transition);
        println!("Pilha: {:?}", stack);

        current_state = transition.1;
        if transition.0 != ' ' {
            stack.pop();
        }
        if transition.2 != ' ' {
            stack.push(transition.2);
        }
        pos += 1;
        // Posicao na fita maior que o tamanho do vetor
        if pos >= tape.len() {
            break;
        }
        current_char = tape[pos];
        println!();
    }
    println!();
    if stack.len() == 0 {
        println!("Aceita!");
    } else {
        println!("Não aceita! pilha: {:?}", stack);
    }
}
