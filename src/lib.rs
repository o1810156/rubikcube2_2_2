use num_traits::{Num, One, Zero};
use std::cmp::{Eq, PartialEq};
use std::convert::From;
use std::fmt;
use std::fmt::Display;
use std::ops;

#[derive(Debug, Clone)]
pub struct Replacement {
    original: Vec<Vec<usize>>,
    table: Vec<Vec<usize>>,
    k: usize,
}

impl Replacement {
    pub fn e() -> Self {
        Replacement {
            original: vec![],
            table: vec![],
            k: 0,
        }
    }

    pub fn new(table: Vec<Vec<usize>>) -> Self {
        let original = table.clone();
        if table.len() == 0 {
            return Self::e();
        }

        for v in table.iter() {
            if v.len() == 0 {
                panic!("empty vector");
            }
        }

        let &k = table.iter().map(|v| v.iter().max().unwrap()).max().unwrap();

        let table = table
            .iter()
            .rev()
            .map(|v| {
                let mut w = vec![0; k + 1];
                for i in 0..(v.len() - 1) {
                    w[v[i]] = v[i + 1];
                }
                w[v[v.len() - 1]] = v[0];
                w
            })
            .collect::<Vec<_>>();

        Self { original, table, k }
    }

    pub fn get_k(&self) -> usize {
        self.k
    }

    pub fn get_correct_k(&self) -> Option<usize> {
        for i in (1..=self.k).rev() {
            if self.replace(i) != i {
                return Some(i);
            }
        }
        None
    }

    pub fn replace(&self, i: usize) -> usize {
        if i == 0 {
            panic!("index 0 is not allowed");
        }

        if i > self.k {
            return i;
        }

        let mut dist = i;
        for v in self.table.iter() {
            let next = v[dist];
            dist = if next > 0 { next } else { dist };
        }
        dist
    }

    pub fn concat_before(&self, other: &Self) -> Self {
        let mut new_table = self.original.clone();
        new_table.extend_from_slice(&other.original);
        Self::new(new_table)
    }

    pub fn rev_find(&self, val: usize) -> Option<usize> {
        if val == 0 {
            panic!("Dist 0 is not allowed");
        }

        for i in 1..=self.k {
            if self.replace(i) == val {
                return Some(i);
            }
        }
        None
    }

    pub fn rearrange(&self) -> Self {
        let k = match self.get_correct_k() {
            Some(k) => k,
            None => return Self::e(),
        };

        let mut new_table = vec![];
        let mut book = vec![true; k + 1];
        book[0] = false;
        for i in 1..=k {
            if book[i] {
                book[i] = false;
                let mut dist = self.replace(i);

                if dist == i {
                    continue;
                }

                let mut chain = vec![i];
                while dist != chain[0] {
                    chain.push(dist);
                    book[dist] = false;
                    dist = self.replace(dist);
                }
                new_table.push(chain);

                continue;
            }
        }

        Replacement::new(new_table)
    }

    pub fn from_correspond_book(corr_book: &Vec<usize>) -> Self {
        let &k = corr_book.iter().max().unwrap();

        let mut table = vec![];
        let mut book = vec![true; k + 1];
        book[0] = false;
        for i in 1..=k {
            if book[i] {
                book[i] = false;
                let mut dist = corr_book[i - 1];

                if dist == i {
                    continue;
                }

                let mut chain = vec![i];
                while dist != chain[0] {
                    chain.push(dist);
                    book[dist] = false;
                    dist = corr_book[dist - 1];
                }
                table.push(chain);

                continue;
            }
        }
        Replacement::new(table)
    }
}

impl PartialEq for Replacement {
    fn eq(&self, other: &Self) -> bool {
        let k = self.get_k().max(other.get_k());
        for i in 1..=k {
            // println!("{}: {} | {}", i, self.replace(i), other.replace(i));
            if self.replace(i) != other.replace(i) {
                return false;
            }
        }
        true
    }
}

impl Eq for Replacement {}

impl Display for Replacement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.k == 0 {
            return write!(f, "e");
        }

        let mut res = vec![];
        for v in self.original.iter() {
            let s = format!(
                "({})",
                v.iter()
                    .map(|&i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            );
            res.push(s);
        }
        write!(f, "{}", res.join(""))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Mod3 {
    Zero,
    One,
    Two,
}

impl Mod3 {
    pub fn add_inv(&self) -> Self {
        match self {
            &Mod3::Zero => Mod3::Zero,
            &Mod3::One => Mod3::Two,
            &Mod3::Two => Mod3::One,
        }
    }

    pub fn mul_inv(&self) -> Self {
        match self {
            &Mod3::Zero => panic!("0 is not allowed"),
            &Mod3::One => Mod3::One,
            &Mod3::Two => Mod3::Two,
        }
    }
}

impl fmt::Display for Mod3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Mod3::Zero => "1  ",
                Mod3::One => "w  ",
                Mod3::Two => "w^2",
            }
        )
    }
}

impl From<Mod3> for usize {
    fn from(m: Mod3) -> Self {
        match m {
            Mod3::Zero => 0,
            Mod3::One => 1,
            Mod3::Two => 2,
        }
    }
}

impl From<usize> for Mod3 {
    fn from(n: usize) -> Self {
        match n % 3 {
            0 => Mod3::Zero,
            1 => Mod3::One,
            _ => Mod3::Two,
        }
    }
}

impl Num for Mod3 {
    type FromStrRadixErr = <usize as Num>::FromStrRadixErr;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        let res = <usize as Num>::from_str_radix(s, radix)?;

        Ok(Self::from(res))
    }
}

impl Zero for Mod3 {
    fn zero() -> Self {
        Self::Zero
    }

    fn is_zero(&self) -> bool {
        self == &Self::Zero
    }

    fn set_zero(&mut self) {
        *self = Self::Zero;
    }
}

impl One for Mod3 {
    fn one() -> Self {
        Self::One
    }
    fn is_one(&self) -> bool {
        self == &Self::One
    }
    fn set_one(&mut self) {
        *self = Self::One;
    }
}

impl ops::Add for Mod3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Mod3::Zero, _) => other,
            (_, Mod3::Zero) => self,
            (Mod3::One, Mod3::One) => Mod3::Two,
            (Mod3::One, Mod3::Two) => Mod3::Zero,
            (Mod3::Two, Mod3::One) => Mod3::Zero,
            (Mod3::Two, Mod3::Two) => Mod3::One,
        }
    }
}

impl ops::AddAssign for Mod3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl ops::Mul for Mod3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Mod3::Zero, _) => Mod3::Zero,
            (_, Mod3::Zero) => Mod3::Zero,
            (Mod3::One, Mod3::One) => Mod3::One,
            (Mod3::One, Mod3::Two) => Mod3::Two,
            (Mod3::Two, Mod3::One) => Mod3::Two,
            (Mod3::Two, Mod3::Two) => Mod3::One,
        }
    }
}

impl ops::MulAssign for Mod3 {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl ops::Sub for Mod3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Mod3::Zero, _) => other.add_inv(),
            (_, Mod3::Zero) => self.add_inv(),
            (Mod3::One, Mod3::One) => Mod3::Zero,
            (Mod3::One, Mod3::Two) => Mod3::Two,
            (Mod3::Two, Mod3::One) => Mod3::One,
            (Mod3::Two, Mod3::Two) => Mod3::Zero,
        }
    }
}

impl ops::SubAssign for Mod3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl ops::Div for Mod3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if other.is_zero() {
            panic!("0 division occured.");
        }

        self * other.mul_inv()
    }
}

impl ops::DivAssign for Mod3 {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl ops::Rem for Mod3 {
    type Output = Self;

    fn rem(self, _other: Self) -> Self {
        todo!()
    }
}
