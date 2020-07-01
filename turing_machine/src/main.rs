use std::collections::HashMap;

/// Máquina de Turing
/// A HashMap a seguir possui as seguintes caracteristicas:
/// como chave uma tupla: (estado_atual, char_atual)
/// e valores associados: (char_escrever, proximo_estado, andar_para)
fn main() {
    let tape = vec![' ', 'a', 'a', 'b', ' '];
    let initial_state = "q0";
    let final_state = "q4";
    let mut machine = HashMap::new();
    machine.insert(("q0", 'A'), ('A', "q0", Direction::Right));
    machine.insert(("q0", 'B'), ('B', "q0", Direction::Right));
    machine.insert(("q0", 'a'), ('A', "q1", Direction::Right));
    machine.insert(("q0", 'b'), ('B', "q3", Direction::Right));
    machine.insert(("q0", ' '), (' ', "q4", Direction::Right));

    machine.insert(("q1", 'B'), ('B', "q1", Direction::Right));
    machine.insert(("q1", 'a'), ('a', "q1", Direction::Right));
    machine.insert(("q1", 'b'), ('B', "q2", Direction::Left));

    machine.insert(("q2", 'b'), ('b', "q2", Direction::Left));
    machine.insert(("q2", 'B'), ('B', "q2", Direction::Left));
    machine.insert(("q2", 'a'), ('a', "q2", Direction::Left));
    machine.insert(("q2", 'A'), ('A', "q2", Direction::Left));
    machine.insert(("q2", ' '), (' ', "q0", Direction::Right));

    machine.insert(("q3", 'b'), ('b', "q3", Direction::Right));
    machine.insert(("q3", 'A'), ('A', "q3", Direction::Right));
    machine.insert(("q3", 'a'), ('A', "q2", Direction::Left));

    compute(tape, machine, initial_state, final_state, 1);
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
}

// Será imprimido:
// [fita]
// <estado_atual> <posicao_na_fita> <ação>
//
// Ex: [' ', 'A', 'A', 'B', 'b', ' ']
//     "q1" 3 ('B', "q1", Right)
fn compute(
    mut tape: Vec<char>,
    machine: HashMap<(&str, char), (char, &str, Direction)>,
    initial_state: &str,
    final_state: &str,
    start_from: usize,
) {
    let mut pos = start_from;
    let mut current_state = initial_state;
    let mut current_char = tape[pos];

    // Enquanto houver transicao entre o estado atual e o próximo
    while let Some(transition) = machine.get(&(current_state, current_char)) {
        println!("{:?}", tape);
        println!("{:?} {} {:?}", current_state, pos, transition);
        tape[pos] = transition.0;
        current_state = transition.1;
        match transition.2 {
            Direction::Right => pos += 1,
            Direction::Left => pos -= 1,
        }
        // Posicao na fita maior que o tamanho do vetor
        if pos >= tape.len() {
            break;
        }
        current_char = tape[pos];
        println!();
    }
    println!();
    if current_state == final_state {
        println!("Aceita!");
    } else {
        println!("Não aceita! estado atual: {:?}", current_state);
    }
}
