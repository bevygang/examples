use crate::code::code_string;

pub fn next_input(inputs: &[(Vec<char>, Vec<char>)]) -> Option<Vec<char>> {
    let mut step = strategy();

    for (input, eval) in inputs {
        if *input != step.input {
            println!("Invalid input: {}", code_string(input));
            return None;
        }

        let (i, j) = next_indices(eval);

        if i >= step.next.len() || j >= step.next[i].len() {
            return None;
        }

        step = step.next[i][j].clone();

        if step.count == 0 {
            println!("Solver error: invalid state");
            return None;
        }

        if step.count <= 2 {
            return None;
        }
    }

    if step.input.len() == 4 {
        Some(step.input)
    } else {
        None
    }
}

fn next_indices(eval: &[char]) -> (usize, usize) {
    let black_count = eval.iter().filter(|c| { **c == 'S' }).count();
    let white_count = eval.len() - black_count;

    (black_count, 4-black_count-white_count)
}

#[derive(Clone)]
struct Step {
    count: usize,
    input: Vec<char>,
    next: Vec<Vec<Step>>
}

impl Step {
    fn new(count: usize, input: &str, next: Vec<Vec<Step>>) -> Step {
        Step { count, input: input.chars().collect(), next}
    }

    fn empty(count: usize) -> Step {
        Step { count, input: vec![], next: vec![]}
    }

    fn last(count: usize, input: &str) -> Step {
        Step { count, input: input.chars().collect(), next: vec![] }
    }

    fn lastx(count: usize, input: &str) -> Step {
        Step {
            count,
            input: input.chars().collect(),
            next: vec![
                vec![Step::empty(2);5],
                vec![Step::empty(2);4],
                vec![Step::empty(2);3],
                vec![Step::empty(2);2],
                vec![Step::empty(2)],
            ]
        }
    }
}

fn strategy() -> Step {
    Step{
        count: 1296,
        input: "1122".chars().collect(),
        next: vec![
            vec![
                Step::empty(1),
                Step::new(16, "1213", vec![
                    vec![Step::empty(0); 5],
                    vec![
                        Step::empty(1),
                        Step::last(4, "1415"),
                        Step::last(3, "1145"),
                        Step::empty(0),
                    ],
                    vec![
                        Step::empty(1),
                        Step::last(3, "4115"),
                        Step::last(3, "1145"),
                    ],
                    vec![Step::empty(0), Step::empty(1)],
                    vec![Step::empty(0)],
                ]),
                strategy_a(96),
                strategy_b(256),
                strategy_c(256),
            ],
            vec![
                Step::empty(0),
                strategy_d(36),
                strategy_e(208),
                strategy_f(256),
            ],
            vec![
                Step::last(4,"1213"),
                strategy_g(32),
                strategy_h(114),
            ],
            vec![
                Step::empty(0),
                strategy_i(20),
            ],
            vec![Step::empty(1)]
        ]
    }
}

fn strategy_a(count: usize) -> Step {
    Step::new(count, "2344", vec![
        vec![
            Step::empty(0),
            Step::empty(2),
            Step::new(16, "3215", vec![
                vec![Step::empty(0); 5],
                vec![
                    Step::empty(1),
                    Step::empty(2),
                    Step::empty(1),
                    Step::empty(1),
                ],
                vec![
                    Step::empty(2),
                    Step::last(3, "3231"),
                    Step::empty(2),
                ],
                vec![
                    Step::empty(0),
                    Step::last(3, "3213"),
                ],
                vec![Step::empty(1)],
            ]),
            Step::new(14, "5215",vec![
                vec![Step::empty(0); 5],
                vec![
                    Step::empty(0),
                    Step::empty(1),
                    Step::last(3, "3511"),
                    Step::last(3, "3611"),
                ],
                vec![
                   Step::empty(1),
                   Step::empty(1),
                   Step::empty(2),
                ],
                vec![
                    Step::empty(0),
                    Step::empty(2),
                ],
                vec![Step::empty(1)],
            ]),
            Step::last(4, "1515"),
        ],
        vec![
           Step::empty(0),
           Step::last(6, "2413"),
           Step::new(18, "2415", vec![
              vec![
                  Step::empty(1),
                  Step::empty(1),
                  Step::empty(0),
                  Step::empty(0),
                  Step::empty(0),
              ],
              vec![
                  Step::empty(1),
                  Step::empty(2),
                  Step::last(3, "2253"),
                  Step::last(3, "2236"),
              ],
              vec![
                  Step::empty(1),
                  Step::empty(2),
                  Step::empty(2),
              ],
              vec![
                  Step::empty(0),
                  Step::empty(1),
              ],
              vec![Step::empty(1)],
           ]),
           Step::lastx(15, "2256"),
        ],
        vec![
            Step::empty(0),
            Step::last(4, "2234"),
            Step::lastx(14, "3315"),
        ],
        vec![
            Step::empty(0),
            Step::last(3, "2314"),
        ],
        vec![Step::empty(0)],
    ])
}

fn strategy_b(count: usize) -> Step {
    Step::new(count, "2344", vec![])
}

fn strategy_c(count: usize) -> Step {
    Step::new(count, "3345", vec![])
}

fn strategy_d(count: usize) -> Step {
    Step::new(count, "1213", vec![])
}

fn strategy_e(count: usize) -> Step {
    Step::new(count, "1134", vec![])
}

fn strategy_f(count: usize) -> Step {
    Step::new(count, "1344", vec![])
}

fn strategy_g(count: usize) -> Step {
    Step::new(count, "1223", vec![])
}

fn strategy_h(count: usize) -> Step {
    Step::new(count, "1234", vec![])
}

fn strategy_i(count: usize) -> Step {
    Step::new(count, "1223", vec![])
}
