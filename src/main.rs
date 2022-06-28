use rubikcube::{Mod3, Replacement};
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone)]
struct WreathElm {
    replacement: Replacement,
    vector: Vec<Mod3>,
}

const WSIZE: usize = 8;
type Matrix = Vec<Vec<Option<Mod3>>>;

impl WreathElm {
    fn new(replacement: Replacement, vector: Vec<Mod3>) -> Self {
        Self {
            replacement,
            vector,
        }
    }

    fn get_matrix(&self) -> Matrix {
        let mut res = vec![vec![None; WSIZE]; WSIZE];
        for i in 0..8 {
            let j = self.replacement.rev_find(i + 1).unwrap_or(i + 1);
            res[i][j - 1] = Some(self.vector[i]);
        }

        res
    }

    fn rev(&self) -> Matrix {
        let m = self.get_matrix();
        let two = rotate(&m, &m);
        rotate(&two, &m)
    }
}

impl Display for WreathElm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{Replacement: {}, Vector: [{}]}}",
            self.replacement,
            self.vector
                .iter()
                .map(|&m| {
                    let n: usize = m.into();
                    n.to_string()
                })
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

fn rotate(now: &Matrix, other: &Matrix) -> Matrix {
    let mut res = vec![vec![None; WSIZE]; WSIZE];

    for i in 0..WSIZE {
        for j in 0..WSIZE {
            if let Some(m) = other[i][j] {
                let target_line = &now[j];
                for k in 0..WSIZE {
                    if let Some(n) = target_line[k] {
                        res[i][k] = Some(n + m);
                        break;
                    }
                }
                break;
            }
        }
    }

    res
}

fn print_matrix(name: &str, m: &Matrix) {
    println!("=== {} ===", name);
    for line in m.iter() {
        println!(
            "{}",
            line.iter()
                .map(|n| if let Some(n) = n {
                    n.to_string()
                } else {
                    "_  ".to_string()
                })
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
    println!("=========");
}

fn decode(m: &Matrix) -> WreathElm {
    let mut corr_book = vec![0; WSIZE];

    for i in 0..8 {
        for j in 0..8 {
            if let Some(_) = m[j][i] {
                corr_book[i] = j + 1;
                break;
            }
        }
    }

    let mut vector = vec![];
    for i in 0..8 {
        for j in 0..8 {
            if let Some(val) = m[i][j] {
                vector.push(val);
                break;
            }
        }
    }

    let replacement = Replacement::from_correspond_book(&corr_book);
    WreathElm::new(replacement, vector)
}

#[derive(Debug, Clone)]
struct Operation {
    name: String,
    w: WreathElm,
    m: Matrix,
}

impl Operation {
    fn new(name: &str, w: WreathElm) -> Self {
        let m = w.get_matrix();
        Operation {
            name: name.to_string(),
            w,
            m,
        }
    }

    fn rev(&self) -> Self {
        let m = self.w.rev();
        let w = decode(&m);
        Operation {
            name: format!("{}^{{-1}}", self.name),
            w,
            m,
        }
    }
}

fn main() {
    let u = WreathElm::new(
        Replacement::new(vec![vec![1, 2, 3, 4]]),
        vec![Mod3::Zero; WSIZE],
    );

    let d = WreathElm::new(
        Replacement::new(vec![vec![5, 6, 7, 8]]),
        vec![Mod3::Zero; WSIZE],
    );

    let l = WreathElm::new(
        Replacement::new(vec![vec![2, 7, 6, 3]]),
        vec![0, 2, 1, 0, 0, 2, 1, 0]
            .into_iter()
            .map(|n| n.into())
            .collect(),
    );

    let r = WreathElm::new(
        Replacement::new(vec![vec![1, 4, 5, 8]]),
        vec![1, 0, 0, 2, 1, 0, 0, 2]
            .into_iter()
            .map(|n| n.into())
            .collect(),
    );

    let f = WreathElm::new(
        Replacement::new(vec![vec![1, 8, 7, 2]]),
        vec![2, 1, 0, 0, 0, 0, 2, 1]
            .into_iter()
            .map(|n| n.into())
            .collect(),
    );

    let b = WreathElm::new(
        Replacement::new(vec![vec![3, 6, 5, 4]]),
        vec![0, 0, 2, 1, 2, 1, 0, 0]
            .into_iter()
            .map(|n| n.into())
            .collect(),
    );

    let u = Operation::new("U", u);
    let d = Operation::new("D", d);
    let l = Operation::new("L", l);
    let r = Operation::new("R", r);
    let f = Operation::new("F", f);
    let b = Operation::new("B", b);
    let u_rev = u.rev();
    let d_rev = d.rev();
    let l_rev = l.rev();
    let r_rev = r.rev();
    let f_rev = f.rev();
    let b_rev = b.rev();

    let wreath = vec![
        u.clone(),
        d.clone(),
        l.clone(),
        r.clone(),
        f.clone(),
        b.clone(),
    ];

    for ope in wreath.iter() {
        print_matrix(&ope.name, &ope.m);
    }

    let ur = rotate(&r.m, &u.m);
    print_matrix("UR", &ur);
    println!("{}", decode(&ur));

    let fr = rotate(&r.m, &f.m);
    print_matrix("FR", &fr);
    println!("{}", decode(&fr));

    let bu = rotate(&u.m, &b.m);
    print_matrix("BU", &bu);
    println!("{}", decode(&bu));

    let binv_u = rotate(&u.m, &b_rev.m);
    print_matrix("B^{-1}U", &binv_u);
    println!("{}", decode(&binv_u));

    let test_case = vec![&f.m, &d.m, &f.m, &d.m, &f.m, &d.m, &f.m, &d.m, &f.m];
    let mut res = d.m.clone();
    for m in test_case.into_iter().rev() {
        res = rotate(&res, m);
    }
    print_matrix("test", &res);
    println!("{}", decode(&res));

    // (1)

    let problem_1 = vec![
        &d.m, &d.m, &b.m, &b.m, &d_rev.m, &f.m, &f.m, &d.m, &b.m, &b.m, &d_rev.m, &f.m,
        &f.m, // &dinv,
    ];

    let mut res = d_rev.m.clone();
    for m in problem_1.into_iter().rev() {
        res = rotate(&res, &m);
    }
    print_matrix("(1) (i)", &res);
    println!("{}", decode(&res));

    let problem_2 = vec![
        &r_rev.m, &f_rev.m, &u.m, &f.m, &r.m, &d_rev.m, &r.m, &r.m, &f_rev.m, &r.m, &u_rev.m,
        &r_rev.m, &f.m, &r.m, &r.m, // &d.m,
    ];

    let mut res = d.m.clone();
    for m in problem_2.into_iter().rev() {
        res = rotate(&res, &m);
    }
    print_matrix("(1) (ii)", &res);
    println!("{}", decode(&res));

    // (2)
    let problem_3 = WreathElm::new(
        Replacement::new(vec![vec![1, 6, 8, 7], vec![2, 4, 5]]),
        vec![0, 2, 0, 2, 1, 2, 0, 2]
            .into_iter()
            .map(|n| n.into())
            .collect(),
    );

    let alter_book = vec![
        (
            "(1 2 3)",
            (
                "[RF]U[RF]U^{-1}",
                vec![&r_rev, &f_rev, &r, &f, &u, &r_rev, &f_rev, &r, &f, &u_rev],
            ),
        ),
        (
            "(2 3 4)",
            (
                "[LF]U[LF]U^{-1}",
                vec![&l_rev, &f_rev, &l, &f, &u, &l_rev, &f_rev, &l, &f, &u_rev],
            ),
        ),
        (
            "(3 4 5)",
            (
                "[UR]B^{-1}[UR]B",
                vec![&u_rev, &r_rev, &u, &r, &b_rev, &u_rev, &r_rev, &u, &r, &b],
            ),
        ),
        (
            "(4 5 6)",
            (
                "[DR]B^{-1}[DR]B",
                vec![&d_rev, &r_rev, &d, &r, &b_rev, &d_rev, &r_rev, &d, &r, &b],
            ),
        ),
        (
            "(5 6 7)",
            (
                "[RB]D[RB]D^{-1}",
                vec![&r_rev, &b_rev, &r, &b, &d, &r_rev, &b_rev, &r, &b, &d_rev],
            ),
        ),
        (
            "(6 7 8)",
            (
                "[LB]D[LB]D^{-1}",
                vec![&l_rev, &b_rev, &l, &b, &d, &l_rev, &b_rev, &l, &b, &d_rev],
            ),
        ),
    ]
    .into_iter()
    .map(|(name, (label, vec))| {
        (
            name.to_string(),
            (
                label.to_string(),
                vec.into_iter().map(|o| o.clone()).collect::<Vec<_>>(),
            ),
        )
    })
    .collect::<HashMap<_, _>>();

    let replace_part_alters = vec![
        "(1 2 3)", "(2 3 4)", "(2 3 4)", "(3 4 5)", "(2 3 4)", "(2 3 4)", "(4 5 6)", "(4 5 6)",
        "(5 6 7)", "(4 5 6)", "(3 4 5)", "(2 3 4)", "(2 3 4)", "(6 7 8)",
    ];
    let mut replace_part_opes = vec![];
    let mut replace_part_vec = vec![];

    for &alt in replace_part_alters.iter() {
        let alter = alter_book.get(alt).unwrap();
        replace_part_opes.push(alter.0.clone());
        replace_part_vec.extend_from_slice(alter.1.as_slice());
    }

    replace_part_vec.push(u.clone());
    let mut res = problem_3.get_matrix();
    for o in replace_part_vec.into_iter().rev() {
        res = rotate(&res, &o.m);
    }
    print_matrix("(2) replacement", &res);
    println!("{}", decode(&res));

    println!("About operation: {:?}\n", replace_part_opes);

    let problem_4 = res;

    #[rustfmt::skip]
    let twist_book = 
    {
        vec![
            // x_1
            ("(BU^{-1})^3(B^{-1}U^{-1})^5", vec![
                &b, &u_rev, &b, &u_rev, &b, &u_rev,
                &b_rev, &u_rev, &b_rev, &u_rev, &b_rev, &u_rev, &b_rev, &u_rev, &b_rev, &u_rev,
            ]),
            // x_2
            ("(U^{-1}F)^3(U^{-1}F^{-1})^5", vec![
                &u_rev, &f, &u_rev, &f, &u_rev, &f,
                &u_rev, &f_rev, &u_rev, &f_rev, &u_rev, &f_rev, &u_rev, &f_rev, &u_rev, &f_rev,
            ]),
            // x_3
            ("(B^{-1}D)^3(B^{-1}D^{-1})^5", vec![
                &b_rev, &d, &b_rev, &d, &b_rev, &d,
                &b_rev, &d_rev, &b_rev, &d_rev, &b_rev, &d_rev, &b_rev, &d_rev, &b_rev, &d_rev,
            ]),
            // x_4
            ("(B^{-1}L)^3(B^{-1}L^{-1})^5", vec![
                &b_rev, &l, &b_rev, &l, &b_rev, &l,
                &b_rev, &l_rev, &b_rev, &l_rev, &b_rev, &l_rev, &b_rev, &l_rev, &b_rev, &l_rev,
            ]),
            // x_5
            ("(D^{-1}R)^3(D^{-1}R^{-1})^5", vec![
                &d_rev, &r, &d_rev, &r, &d_rev, &r,
                &d_rev, &r_rev, &d_rev, &r_rev, &d_rev, &r_rev, &d_rev, &r_rev, &d_rev, &r_rev,
            ]),
            // x_6
            ("(D^{-1}B)^3(D^{-1}B^{-1})^5", vec![
                &d_rev, &b, &d_rev, &b, &d_rev, &b,
                &d_rev, &b_rev, &d_rev, &b_rev, &d_rev, &b_rev, &d_rev, &b_rev, &d_rev, &b_rev, 
            ]),
            // y
            ("(FD)^5(F^{-1}D)^3(D^{-1}R)^3(D^{-1}R^{-1})^5", vec![
                &f, &d, &f, &d, &f, &d, &f, &d, &f, &d,
                &f_rev, &d, &f_rev, &d, &f_rev, &d,
                &d_rev, &r, &d_rev, &r, &d_rev, &r,
                &d_rev, &r_rev, &d_rev, &r_rev, &d_rev, &r_rev, &d_rev, &r_rev, &d_rev, &r_rev, 
            ])
        ]
        .into_iter()
        .map(|(label, vec)| {
            (
                label.to_string(),
                vec.into_iter().map(|o| o.clone()).collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>()
    };

    let mut v_inv = vec![];
    let mut opes = vec![];

    // v_inv = 2 x_1 + x_3 + 2 x_4 + 2 x_5 + x_6 + y

    v_inv.extend_from_slice(&twist_book[0].1);
    opes.push(twist_book[0].0.clone());
    v_inv.extend_from_slice(&twist_book[0].1);
    opes.push(twist_book[0].0.clone());

    v_inv.extend_from_slice(&twist_book[2].1);
    opes.push(twist_book[2].0.clone());

    v_inv.extend_from_slice(&twist_book[3].1);
    opes.push(twist_book[3].0.clone());
    v_inv.extend_from_slice(&twist_book[3].1);
    opes.push(twist_book[3].0.clone());

    v_inv.extend_from_slice(&twist_book[4].1);
    opes.push(twist_book[4].0.clone());
    v_inv.extend_from_slice(&twist_book[4].1);
    opes.push(twist_book[4].0.clone());

    v_inv.extend_from_slice(&twist_book[5].1);
    opes.push(twist_book[5].0.clone());

    v_inv.extend_from_slice(&twist_book[6].1);
    opes.push(twist_book[6].0.clone());

    let mut res = problem_4;
    for o in v_inv.into_iter().rev() {
        res = rotate(&res, &o.m);
        // print_matrix("dump", &res);
    }
    print_matrix("(2) twist", &res);
    println!("{}", decode(&res));

    println!("About operation: {:?}\n", opes);
}
