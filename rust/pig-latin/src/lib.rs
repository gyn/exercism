#[derive(Copy, Clone, Debug)]
pub enum FSMState {
    Begin,
    FirstX,
    FirstY,
    SpeicalVowel,
    VowelDone,
    FirstQ,
    FirstS,
    Consonant,
    SecondY,
    ConsonantDigraph,
    ConsonantCluster,
    ConsonantDone,
    End,
}

fn fsm(current: FSMState, ch: u8) -> FSMState {
    match current {
        FSMState::Begin => match ch {
            b'a' | b'e' | b'i' | b'o' | b'u' => FSMState::VowelDone,
            b'q' => FSMState::FirstQ,
            b's' => FSMState::FirstS,
            b'y' => FSMState::FirstY,
            b'x' => FSMState::FirstX,
            _ => FSMState::Consonant,
        },
        FSMState::FirstX => match ch {
            b'a' | b'e' | b'i' | b'o' | b'u' => FSMState::ConsonantDone,
            b'r' => FSMState::SpeicalVowel,
            _ => FSMState::Consonant,
        },
        FSMState::FirstY => match ch {
            b'a' | b'e' | b'i' | b'o' | b'u' => FSMState::ConsonantDone,
            b't' => FSMState::SpeicalVowel,
            _ => FSMState::Consonant,
        },
        FSMState::SpeicalVowel | FSMState::VowelDone => match ch {
            0 => FSMState::End,
            _ => FSMState::VowelDone,
        },
        FSMState::FirstQ => match ch {
            b'u' => FSMState::ConsonantDigraph,
            b'a' | b'e' | b'i' | b'o' => FSMState::ConsonantDone,
            _ => FSMState::Consonant,
        },
        FSMState::FirstS => match ch {
            b'q' => FSMState::FirstQ,
            b'a' | b'e' | b'i' | b'o' | b'u' => FSMState::ConsonantDone,
            _ => FSMState::Consonant,
        },
        FSMState::ConsonantDigraph => match ch {
            b'a' | b'e' | b'i' | b'o' | b'u' => FSMState::ConsonantDone,
            _ => FSMState::Consonant,
        },
        FSMState::Consonant => match ch {
            b'a' | b'e' | b'i' | b'o' | b'u' => FSMState::ConsonantDone,
            b'y' => FSMState::SecondY,
            _ => FSMState::ConsonantCluster,
        },
        FSMState::SecondY => match ch {
            0 => FSMState::End,
            b'a' | b'e' | b'i' | b'o' | b'u' => FSMState::ConsonantDone,
            _ => FSMState::ConsonantCluster,
        },
        FSMState::ConsonantCluster => match ch {
            b'a' | b'e' | b'i' | b'o' | b'u' => FSMState::ConsonantDone,
            b'y' => FSMState::ConsonantDone,
            _ => FSMState::ConsonantCluster,
        },
        FSMState::ConsonantDone => FSMState::ConsonantDone,
        _ => FSMState::Begin,
    }
}

fn translate_word(input: &[u8]) -> String {
    if input.is_empty() {
        return String::new();
    }

    let mut current = FSMState::Begin;

    let mut result = Vec::new();
    let mut consonant = Vec::new();

    for &ch in input {
        let next = fsm(current, ch);

        match next {
            FSMState::SpeicalVowel => {
                result.append(&mut consonant);
                result.push(ch)
            }
            FSMState::SecondY | FSMState::VowelDone | FSMState::ConsonantDone => result.push(ch),
            FSMState::FirstX
            | FSMState::FirstY
            | FSMState::FirstQ
            | FSMState::FirstS
            | FSMState::Consonant
            | FSMState::ConsonantDigraph
            | FSMState::ConsonantCluster => consonant.push(ch),
            _ => {}
        };

        current = next;
    }

    fsm(current, 0);

    result.append(&mut consonant);

    result.push(b'a');
    result.push(b'y');

    String::from_utf8(result).unwrap()
}

pub fn translate(input: &str) -> String {
    let mut result = String::new();

    let mut word = Vec::new();

    for c in input.chars() {
        match c {
            ' ' => {
                let v = translate_word(&word);
                result.push_str(&v);
                word.clear();

                result.push(' ');
            }
            m if c.is_ascii() => word.push(m as u8),
            _ => {}
        }
    }

    let v = translate_word(&word);
    result.push_str(&v);

    result
}
