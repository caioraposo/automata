use std::collections::HashMap;

/// Pushdown Automata
/// A HashMap a seguir possui as seguintes caracteristicas:
/// como chave uma tupla: (estado_atual, char_atual)
/// e valores associados: (desempilhar, proximo_estado, andar_para)
/// A linguagem será validada se a pilha estiver vazia
fn main() {
    let initial_state = "q0";
    let final_state = "q0";
    // Read input file and ignore empty spaces
    let tape: Vec<char> = include_str!("../input")
        .trim()
        .chars()
        .filter(|x| *x != ' ')
        .collect();

    let mut pda = HashMap::new();
    let pda_file = include_str!("../pda");
    let lines: Vec<Vec<&str>> = pda_file
        .lines()
        .map(|l| l.split(" ").collect())
        .collect();

    println!("Pushdown Automata:");
    println!("estado_atual char_lido   desempilhar prox_estado empilhar");
    for line in lines.iter() {
        println!("{:?}", line);
        pda.insert((line[0], line[1].chars().next().unwrap()),
                   (line[2].chars().next().unwrap(), line[3],
                        line[4].chars().next().unwrap()));
    }
    compute(tape, pda, initial_state, final_state, 0);
}

// Será imprimido:
// [fita]
// <estado_atual> <posicao_na_fita> <ação>
//
// Ex: ['A', 'A', 'B', 'b']
//     "q1" 3 ('B', "q1", Right)
fn compute(
    tape: Vec<char>,
    pda: HashMap<(&str, char), (char, &str, char)>,
    initial_state: &str,
    _final_state: &str,
    start_from: usize,
) {
    let mut stack = Vec::new();
    let mut pos = start_from;
    let mut current_state = initial_state;
    let mut current_char = tape[pos];

    // Enquanto houver transicao entre o estado atual e o próximo
    while let Some(transition) = pda.get(&(current_state, current_char)) {
        println!("{:?}", tape);
        println!("{:?} {} {:?}", current_state, pos, transition);
        println!("Pilha: {:?}", stack);

        current_state = transition.1;
        if transition.0 != 'λ' {
            stack.pop();
        }
        if transition.2 != 'λ' {
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
    if stack.is_empty() {
        println!("Aceita!");
    } else {
        println!("Não aceita! pilha: {:?}", stack);
    }
}
