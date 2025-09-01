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
    Step::new(count, "2344", vec![
        vec![
            Step::empty(0),
            Step::last(7, "2335"),
            Step::new(41, "3235", vec![
                vec![
                    Step::empty(0),
                    Step::empty(0),
                    Step::empty(2),
                    Step::last(3, "4613"),
                    Step::empty(2),
                ],
                vec![
                    Step::empty(0),
                    Step::last(3, "5263"),
                    Step::last(6, "3413"),
                    Step::last(6, "3416"),
                ],
                vec![
                    Step::empty(2),
                    Step::last(4, "3256"),
                    Step::last(6, "1336"),
                ],
                vec![
                    Step::empty(0),
                    Step::last(6, "1536"),
                ],
                vec![
                    Step::empty(1),
                ],
            ]),
            Step::new(44, "3516", vec![
               vec![
                   Step::empty(1),
                   Step::last(4, "4651"),
                   Step::last(6, "6255"),
                   Step::empty(1),
                   Step::empty(0),
               ],
               vec![
                   Step::last(3, "5613"),
                   Step::last(7, "1461"),
                   Step::last(5, "4551"),
                   Step::empty(1),
               ],
               vec![
                   Step::last(3, "1113"),
                   Step::last(5, "3551"),
                   Step::last(3, "4515"),
               ],
               vec![
                   Step::empty(0),
                   Step::last(4, "1145"),
               ],
               vec![
                   Step::empty(1),
               ],
            ]),
            Step::new(16, "5515", vec![
                vec![
                    Step::empty(0),
                    Step::empty(0),
                    Step::empty(1),
                    Step::empty(1),
                    Step::empty(0),
                ],
                vec![
                    Step::empty(0),
                    Step::empty(2),
                    Step::empty(2),
                    Step::empty(1),
                ],
                vec![
                    Step::empty(1),
                    Step::empty(1),
                    Step::last(3, "1516"),
                ],
                vec![
                    Step::empty(0),
                    Step::last(3, "1516"),
                ],
                vec![
                    Step::empty(1),
                ],
            ]),
        ],
        vec![
            Step::empty(2),
            Step::new(21, "3245", vec![
                vec![
                    Step::empty(1),
                    Step::last(3, "2436"),
                    Step::empty(0),
                    Step::empty(0),
                    Step::empty(0),
                ],
                vec![
                    Step::empty(2),
                    Step::empty(2),
                    Step::empty(2),
                    Step::empty(0),
                ],
                vec![
                    Step::empty(2),
                    Step::last(3, "3234"),
                    Step::empty(2),
                ],
                vec![
                    Step::empty(0),
                    Step::last(3, "3243"),
                ],
                vec![
                    Step::empty(1),
                ],
            ]),
            Step::new(42, "4514", vec![
                vec![
                    Step::empty(1),
                    Step::empty(1),
                    Step::last(7, "2456"),
                    Step::last(4, "2635"),
                    Step::last(3, "2636"),
                ],
                vec![
                    Step::empty(0),
                    Step::last(4, "1356"),
                    Step::last(5, "4361"),
                    Step::last(6, "1635"),
                ],
                vec![
                    Step::empty(2),
                    Step::empty(2),
                    Step::last(3, "3614"),
                ],
                vec![
                    Step::empty(0),
                    Step::last(3, "4414"),
                ],
                vec![
                    Step::empty(1),
                ],
            ]),
            Step::new(34, "3315", vec![
                vec![
                    Step::empty(0),
                    Step::empty(0),
                    Step::last(3, "5641"),
                    Step::last(4, "2566"),
                    Step::empty(1),
                ],
                vec![
                    Step::empty(1),
                    Step::last(4, "5361"),
                    Step::last(4, "5614"),
                    Step::last(5, "6614"),
                ],
                vec![
                    Step::empty(2),
                    Step::last(4, "3331"),
                    Step::empty(1),
                ],
                vec![
                    Step::empty(0),
                    Step::last(4, "3316"),
                ],
                vec![
                    Step::empty(1),
                ],
            ]),
        ],
        vec![
            Step::last(3, "2434"),
            Step::lastx(13, "2425"),
            Step::new(23, "1545", vec![
                vec![
                    Step::empty(0),
                    Step::empty(1),
                    Step::last(3, "2654"),
                    Step::last(3, "2353"),
                    Step::last(4, "1136"),
                ],
                vec![
                    Step::empty(0),
                    Step::empty(2),
                    Step::last(4, "2564"),
                    Step::last(3, "2335"),
                ],
                vec![
                    Step::empty(0),
                    Step::empty(0),
                    Step::empty(2),
                ],
                vec![
                    Step::empty(0),
                    Step::empty(1),
                ],
                vec![
                    Step::empty(0),
                ],
            ]),
        ],
        vec![
            Step::empty(0),
            Step::lastx(9, "1335"),
        ],
        vec![
            Step::empty(1),
        ],
    ])
}

fn strategy_c(count: usize) -> Step {
    Step::new(count, "3345", vec![
        vec![
            Step::empty(2),
            Step::new(20, "4653", vec![
                vec![
                    Step::empty(2),
                    Step::empty(2),
                    Step::empty(0),
                    Step::empty(0),
                    Step::empty(0),
                ],
                vec![
                    Step::last(3, "4536"),
                    Step::last(3, "4534"),
                    Step::empty(1),
                    Step::empty(0),
                ],
                vec![
                    Step::empty(2),
                    Step::empty(2),
                    Step::empty(1),
                ],
                vec![
                    Step::empty(0),
                    Step::last(3, "4453"),
                ],
                vec![
                    Step::empty(1),
                ],
            ]),
            Step::new(42, "6634", vec![
                vec![
                    Step::empty(0),
                    Step::last(3, "4566"),
                    Step::last(4, "4556"),
                    Step::empty(1),
                    Step::empty(0),
                ],
                vec![
                    Step::empty(2),
                    Step::last(5, "4656"),
                    Step::last(6, "5653"),
                    Step::last(4, "1444"),
                ],
                vec![
                    Step::empty(2),
                    Step::last(5, "5636"),
                    Step::last(5, "4654"),
                ],
                vec![
                    Step::empty(0),
                    Step::last(4, "1413"),
                ],
                vec![
                    Step::empty(1),
                ],
            ]),
            Step::new(16, "6646", vec![
                vec![
                    Step::empty(0),
                    Step::empty(0),
                    Step::empty(1),
                    Step::empty(0),
                    Step::empty(0),
                ],
                vec![
                    Step::empty(0),
                    Step::last(3, "1416"),
                    Step::empty(1),
                    Step::empty(1),
                ],
                vec![
                    Step::last(3, "1416"),
                    Step::last(3, "5666"),
                    Step::empty(2),
                ],
                vec![
                    Step::empty(0),
                    Step::empty(2),
                ],
                vec![
                    Step::empty(0),
                ],
            ]),
            Step::empty(1),
        ],
        vec![
            Step::last(4, "3453"),
            Step::new(40, "3454", vec![
                vec![
                    Step::empty(1),
                    Step::last(5, "4535"),
                    Step::last(6, "1436"),
                    Step::empty(0),
                    Step::empty(0),
                ],
                vec![
                    Step::empty(2),
                    Step::last(5, "4356"),
                    Step::last(6, "3536"),
                    Step::empty(0),
                ],
                vec![
                    Step::empty(1),
                    Step::last(3, "3564"),
                    Step::last(6, "3463"),
                ],
                vec![
                    Step::empty(0),
                    Step::last(4, "3456"),
                ],
                vec![
                    Step::empty(1),
                ],
            ]),
            Step::new(46, "3636", vec![
                vec![
                    Step::empty(1),
                    Step::empty(1),
                    Step::last(3, "4364"),
                    Step::last(6, "4565"),
                    Step::last(6, "4544"),
                ],
                vec![
                    Step::empty(0),
                    Step::last(5, "4366"),
                    Step::last(6, "1565"),
                    Step::last(6, "4546"),
                ],
                vec![
                    Step::empty(2),
                    Step::last(4, "3466"),
                    Step::last(3, "3556"),
                ],
                vec![
                    Step::empty(0),
                    Step::empty(2),
                ],
                vec![
                    Step::empty(1),
                ],
            ]),
            Step::new(18, "3656", vec![
                vec![
                    Step::empty(0),
                    Step::empty(1),
                    Step::empty(1),
                    Step::empty(1),
                    Step::empty(1),
                ],
                vec![
                    Step::empty(0),
                    Step::last(3, "5665"),
                    Step::last(3, "6443"),
                    Step::last(3, "4446"),
                ],
                vec![
                    Step::empty(0),
                    Step::empty(1),
                    Step::last(3, "4646"),
                ],
                vec![
                    Step::empty(0),
                    Step::empty(1),
                ],
                vec![
                    Step::empty(0),
                ],
            ]),
        ],
        vec![
            Step::lastx(5, "3435"),
            Step::new(20, "3443", vec![
                vec![
                    Step::empty(0),
                    Step::empty(0),
                    Step::last(4, "4355"),
                    Step::empty(0),
                    Step::empty(0),
                ],
                vec![
                    Step::empty(0),
                    Step::last(3, "3334"),
                    Step::last(4, "3356"),
                    Step::empty(0),
                ],
                vec![
                    Step::empty(1),
                    Step::empty(2),
                    Step::last(4, "3455"),
                ],
                vec![
                    Step::empty(0),
                    Step::empty(1),
                ],
                vec![
                    Step::empty(1),
                ],
            ]),
            Step::new(29, "3636", vec![
                vec![
                    Step::empty(0),
                    Step::empty(1),
                    Step::last(3, "5365"),
                    Step::last(4, "6445"),
                    Step::last(4, "1444"),
                ],
                vec![
                    Step::empty(0),
                    Step::empty(2),
                    Step::last(3, "3565"),
                    Step::last(4, "4645"),
                ],
                vec![
                    Step::empty(1),
                    Step::empty(1),
                    Step::last(4, "3446"),
                ],
                vec![
                    Step::empty(0),
                    Step::empty(2),
                ],
                vec![
                    Step::empty(0),
                ],
            ]),
        ],
        vec![
            Step::empty(0),
            Step::lastx(12, "3446"),
        ],
        vec![
            Step::empty(1),
        ],
    ])
}

fn strategy_d(count: usize) -> Step {
    Step::new(count, "1213", vec![
        vec![
            Step::empty(1),
            Step::last(4, "1145"),
            Step::last(3, "1415"),
            Step::empty(0),
            Step::empty(0),
        ],
        vec![
            Step::empty(0),
            Step::lastx(6, "1114"),
            Step::lastx(7, "2412"),
            Step::empty(0),
        ],
        vec![
            Step::empty(2),
            Step::last(4, "1145"),
            Step::lastx(4, "1145"),
        ],
        vec![
            Step::empty(0),
            Step::lastx(4, "1114"),
        ],
        vec![
            Step::empty(1),
        ],
    ])
}

fn strategy_e(count: usize) -> Step {
    Step::new(count, "1134", vec![
        vec![
            Step::empty(0),
            Step::last(4, "1312"),
            Step::new(24, "3521", vec![
                vec![
                    Step::empty(1),
                    Step::empty(2),
                    Step::last(4, "4612"),
                    Step::empty(0),
                    Step::empty(0),
                ],
                vec![
                    Step::empty(0),
                    Step::last(3, "3312"),
                    Step::last(3, "2423"),
                    Step::empty(0),
                ],
                vec![
                    Step::empty(2),
                    Step::empty(2),
                    Step::last(3, "4621"),
                ],
                vec![
                    Step::empty(0),
                    Step::last(3, "3321"),
                ],
                vec![
                    Step::empty(1),
                ],
            ]),
            Step::new(38, "2352", vec![
                vec![
                    Step::empty(2),
                    Step::last(4, "3226"),
                    Step::last(4, "5621"),
                    Step::empty(1),
                    Step::empty(0),
                ],
                vec![
                    Step::empty(1),
                    Step::last(5, "2223"),
                    Step::last(7, "6242"),
                    Step::empty(1),
                ],
                vec![
                    Step::empty(2),
                    Step::last(4, "2323"),
                    Step::last(4, "2462"),
                ],
                vec![
                    Step::empty(0),
                    Step::empty(2),
                ],
                vec![
                    Step::empty(1),
                ],
            ]),
            Step::new(20, "2525", vec![
                vec![
                    Step::empty(1),
                    Step::empty(2),
                    Step::empty(1),
                    Step::empty(0),
                    Step::empty(0),
                ],
                vec![
                    Step::empty(0),
                    Step::last(3, "2252"),
                    Step::last(3, "2262"),
                    Step::empty(0),
                ],
                vec![
                    Step::empty(2),
                    Step::empty(2),
                    Step::empty(2),
                ],
                vec![
                    Step::empty(0),
                    Step::last(3, "2225"),
                ],
                vec![
                    Step::empty(1),
                ],
            ]),
        ],
        vec![
            Step::last(4, "1341"),
            Step::new(34, "1315", vec![
                vec![
                    Step::empty(1),
                    Step::last(3, "4151"),
                    Step::last(4, "4161"),
                    Step::empty(0),
                    Step::empty(0),
                ],
                vec![
                    Step::empty(1),
                    Step::last(6, "6451"),
                    Step::last(6, "1461"),
                    Step::empty(0),
                ],
                vec![
                    Step::last(3, "1351"),
                    Step::last(3, "1361"),
                    Step::empty(2),
                ],
                vec![
                    Step::empty(0),
                    Step::last(4, "1113"),
                ],
                vec![
                    Step::empty(1),
                ],
            ]),
            Step::new(32, "1516", vec![
                vec![
                    Step::empty(2),
                    Step::empty(2),
                    Step::last(3, "2145"),
                    Step::empty(0),
                    Step::last(4, "2324"),
                ],
                vec![
                    Step::empty(2),
                    Step::last(4, "1661"),
                    Step::last(4, "1245"),
                    Step::empty(0),
                ],
                vec![
                    Step::last(3, "1561"),
                    Step::last(3, "1551"),
                    Step::empty(1),
                ],
                vec![
                    Step::empty(0),
                    Step::last(3, "1511"),
                ],
                vec![
                    Step::empty(1),
                ],
            ]),
            Step::new(22, "1256", vec![
                vec![
                    Step::empty(1),
                    Step::empty(0),
                    Step::last(4, "2524"),
                    Step::empty(2),
                    Step::empty(0),
                ],
                vec![
                    Step::empty(0),
                    Step::empty(2),
                    Step::last(4, "5224"),
                    Step::last(4, "2224"),
                ],
                vec![
                    Step::empty(2),
                    Step::empty(0),
                    Step::empty(0),
                ],
                vec![
                    Step::empty(0),
                    Step::empty(2),
                ],
                vec![
                    Step::empty(1),
                ],
            ]),
        ],
        vec![
            Step::last(4, "1314"),
            Step::lastx(12, "1315"),
            Step::lastx(12, "1235"),
        ],
        vec![
            Step::empty(0),
            Step::empty(2),
        ],
        vec![
            Step::empty(0),
        ],
    ])
}

fn strategy_f(count: usize) -> Step {
    Step::new(count, "1344", vec![
        vec![
            Step::empty(0),
            Step::last(7, "1335"),
            Step::new(41, "3135", vec![
                vec![
                    Step::empty(0),
                    Step::empty(0),
                    Step::empty(2),
                    Step::last(3, "4623"),
                    Step::empty(2),
                ],
                vec![
                    Step::empty(0),
                    Step::last(3, "5163"),
                    Step::last(6, "3423"),
                    Step::last(6, "3426"),
                ],
                vec![
                    Step::empty(2),
                    Step::last(4, "3156"),
                    Step::last(6, "1436"),
                ],
                vec![
                    Step::empty(0),
                    Step::last(6, "1536"),
                ],
                vec![
                    Step::empty(1),
                ],
            ]),
            Step::new(44, "3526", vec![
                vec![
                    Step::empty(1),
                    Step::last(4, "4652"),
                    Step::last(6, "6155"),
                    Step::empty(1),
                    Step::empty(0),
                ],
                vec![
                    Step::last(3, "5623"),
                    Step::last(7, "1462"),
                    Step::last(5, "4552"),
                    Step::empty(1),
                ],
                vec![
                    Step::last(3, "1123"),
                    Step::last(5, "3552"),
                    Step::last(3, "4525"),
                ],
                vec![
                    Step::empty(0),
                    Step::last(4, "1145"),
                ],
                vec![
                    Step::empty(1),
                ],
            ]),
            Step::new(16, "5525", vec![
                vec![
                    Step::empty(0),
                    Step::empty(0),
                    Step::empty(1),
                    Step::empty(1),
                    Step::empty(0),
                ],
                vec![
                    Step::empty(0),
                    Step::empty(2),
                    Step::empty(2),
                    Step::empty(1),
                ],
                vec![
                    Step::empty(1),
                    Step::empty(1),
                    Step::last(3, "1516"),
                ],
                vec![
                    Step::empty(0),
                    Step::last(3, "1516"),
                ],
                vec![
                    Step::empty(1),
                ],
            ]),
        ],
        vec![
            Step::empty(2),
            Step::new(21, "3145", vec![
                vec![
                    Step::empty(1),
                    Step::last(3, "1436"),
                    Step::empty(0),
                    Step::empty(0),
                    Step::empty(0),
                ],
                vec![
                    Step::empty(2),
                    Step::empty(2),
                    Step::empty(2),
                    Step::empty(0),
                ],
                vec![
                    Step::empty(2),
                    Step::last(3, "3134"),
                    Step::empty(2),
                ],
                vec![
                    Step::empty(0),
                    Step::last(3, "3143"),
                ],
                vec![
                    Step::empty(1),
                ],
            ]),
            Step::new(42, "4524", vec![
                vec![
                    Step::empty(1),
                    Step::empty(1),
                    Step::last(7, "1456"),
                    Step::last(4, "1635"),
                    Step::last(3, "1636"),
                ],
                vec![
                    Step::empty(0),
                    Step::last(4, "1356"),
                    Step::last(5, "4362"),
                    Step::last(6, "1336"),
                ],
                vec![
                    Step::empty(2),
                    Step::empty(2),
                    Step::last(3, "3624"),
                ],
                vec![
                    Step::empty(0),
                    Step::last(3, "4424"),
                ],
                vec![
                    Step::empty(1),
                ],
            ]),
            Step::new(34, "3325", vec![
                vec![
                    Step::empty(0),
                    Step::empty(0),
                    Step::last(3, "5642"),
                    Step::last(4, "1566"),
                    Step::empty(1),
                ],
                vec![
                    Step::empty(1),
                    Step::last(4, "5362"),
                    Step::last(4, "5624"),
                    Step::last(5, "6624"),
                ],
                vec![
                    Step::empty(2),
                    Step::last(4, "3332"),
                    Step::empty(1),
                ],
                vec![
                    Step::empty(0),
                    Step::last(4, "3326"),
                ],
                vec![
                    Step::empty(1),
                ],
            ]),
        ],
        vec![
            Step::last(3, "1434"),
            Step::lastx(13, "1415"),
            Step::new(23, "1415", vec![
                vec![
                    Step::empty(0),
                    Step::empty(0),
                    Step::empty(2),
                    Step::last(4, "3324"),
                    Step::empty(0),
                ],
                vec![
                    Step::empty(0),
                    Step::last(4, "1546"),
                    Step::last(4, "1356"),
                    Step::last(4, "1136"),
                ],
                vec![
                    Step::empty(0),
                    Step::empty(2),
                    Step::last(3, "1136"),
                ],
                vec![
                    Step::empty(0),
                    Step::empty(0),
                ],
                vec![
                    Step::empty(0),
                ],
            ]),
        ],
        vec![
            Step::empty(0),
            Step::lastx(9, "1335"),
        ],
        vec![
            Step::empty(1),
        ],
    ])
}

fn strategy_g(count: usize) -> Step {
    Step::new(count, "1223", vec![
        vec![
            Step::empty(1),
            Step::last(4, "2145"),
            Step::last(3, "4115"),
            Step::empty(0),
            Step::empty(0),
        ],
        vec![
            Step::empty(0),
            Step::last(5, "2145"),
            Step::last(6, "4512"),
            Step::empty(0),
        ],
        vec![
            Step::empty(2),
            Step::last(4, "1245"),
            Step::last(3, "1415"),
        ],
        vec![
            Step::empty(0),
            Step::last(3, "1145"),
        ],
        vec![
            Step::empty(1),
        ],
    ])
}

fn strategy_h(count: usize) -> Step {
    Step::new(count, "1234", vec![
        vec![
            Step::empty(2),
            Step::new(16, "1325", vec![
                vec![
                    Step::empty(1),
                    Step::last(3, "4152"),
                    Step::last(3, "4162"),
                    Step::empty(0),
                    Step::empty(0),
                ],
                vec![
                    Step::empty(1),
                    Step::last(3, "3126"),
                    Step::empty(2),
                    Step::empty(0),
                ],
                vec![
                    Step::empty(1),
                    Step::empty(1),
                    Step::empty(1),
                ],
                vec![
                    Step::empty(0),
                    Step::empty(0),
                ],
                vec![
                    Step::empty(0),
                ],
            ]),
            Step::new(20, "1325", vec![
                vec![
                    Step::empty(0),
                    Step::last(3, "5162"),
                    Step::empty(1),
                    Step::empty(0),
                    Step::empty(0),
                ],
                vec![
                    Step::empty(0),
                    Step::empty(2),
                    Step::last(4, "4522"),
                    Step::last(4, "4622"),
                ],
                vec![
                    Step::empty(0),
                    Step::last(3, "5125"),
                    Step::last(3, "2116"),
                ],
                vec![
                    Step::empty(0),
                    Step::empty(0),
                ],
                vec![
                    Step::empty(0),
                ],
            ]),
            Step::last(6, "2515"),
            Step::empty(0),
        ],
        vec![
            Step::last(4, "1323"),
            Step::new(21, "1352", vec![
                vec![
                    Step::empty(0),
                    Step::empty(1),
                    Step::empty(2),
                    Step::empty(0),
                    Step::empty(0),
                ],
                vec![
                    Step::empty(2),
                    Step::last(4, "1623"),
                    Step::empty(2),
                    Step::empty(0),
                ],
                vec![
                    Step::empty(1),
                    Step::last(3, "1323"),
                    Step::last(3, "1462"),
                ],
                vec![
                    Step::empty(0),
                    Step::empty(2),
                ],
                vec![
                    Step::empty(1),
                ],
            ]),
            Step::lastx(16, "2156"),
            Step::lastx(12, "1315"),
        ],
        vec![
            Step::empty(2),
            Step::last(6, "3526"),
            Step::lastx(8, "1536"),
        ],
        vec![
            Step::empty(0),
            Step::empty(1),
        ],
        vec![
            Step::empty(0),
        ],
    ])
}

fn strategy_i(count: usize) -> Step {
    Step::new(count, "1223", vec![
        vec![
            Step::empty(0),
            Step::empty(0),
            Step::empty(0),
            Step::empty(0),
            Step::empty(0),
        ],
        vec![
            Step::empty(1),
            Step::lastx(5, "1145"),
            Step::lastx(4, "1114"),
            Step::empty(0),
        ],
        vec![
            Step::empty(1),
            Step::last(3, "1415"),
            Step::lastx(4, "1114"),
        ],
        vec![
            Step::empty(0),
            Step::empty(2),
        ],
        vec![
            Step::empty(0),
        ],
    ])
}
