use crate::prelude::*;
use crate::problem::Problem;
use colored::Colorize;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cordinate(usize, usize);

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum CordinateValue {
    Empty,
    Value,
    Origo,
    HorizontalLine,
    VerticalLine,
    Average,
    Median,
}

#[derive(Debug, Clone)]
pub struct Graph<'a> {
    pub matrix: Vec<Vec<CordinateValue>>,
    origo: Option<Cordinate>,
    problem: &'a Problem,
    def_end: i64,
    def_start: i64,
    min_value: f64,
    max_value: f64,
    average_value: Option<f64>,
    median: i64,
}

impl<'a> Graph<'a> {
    fn create_matrix(x_len: usize, y_len: usize) -> Vec<Vec<CordinateValue>> {
        vec![vec![CordinateValue::Empty; x_len]; y_len]
    }
    pub fn new(def_start: i64, def_end: i64, problem: &'a Problem) -> Self {
        let mut rows = HashMap::new();

        for x in def_start..def_end + 1 {
            let y = problem.clone().solve(Some(x as f64));
            rows.insert(y.to_string(), x);
        }

        let mut y_values = rows
            .keys()
            .map(|y| y.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        y_values.sort();

        let max_value = (def_start..def_end + 1)
            .map(|x| problem.clone().solve(Some(x as f64)) as i64)
            .max()
            .unwrap() as f64;

        let min_value = (def_start..def_end + 1)
            .map(|x| problem.clone().solve(Some(x as f64)) as i64)
            .min()
            .unwrap() as f64;

        Self {
            origo: None,
            matrix: Graph::create_matrix((def_end + 1 - def_start) as usize, rows.len()),

            problem,
            def_end,
            def_start,
            min_value,
            max_value,
            average_value: None,
            median: (y_values.len() - 1) as i64 / 2,
        }
    }

    pub fn average(&mut self) {
        let sum: f64 = (self.def_start..self.def_end + 1)
            .map(|x| self.problem.clone().solve(Some(x as f64)))
            .sum();
        self.average_value = Some(sum / (self.def_end - self.def_start) as f64);

        let min_original_value = self.min_value;
        let max_original_value = self.max_value;
        let min_scaled = 0.0;
        let max_scaled = self.matrix.len() as f64 - 1.0;
        let y = self.matrix.len()
            - 1
            - scale_value(
                self.average_value.unwrap(),
                min_original_value,
                max_original_value,
                min_scaled,
                max_scaled,
            )
            .round()
            .abs() as usize;

        if y != 0 {
            for x_index in 0..self.matrix[0].len() {
                if self.matrix[y][x_index] == CordinateValue::Empty {
                    self.matrix[y][x_index] = CordinateValue::Average;
                }
            }
        }
    }

    pub fn median(&mut self) {
        let matrix_len = self.matrix.len() - 1;

        let median_line: &Vec<CordinateValue> = &self.matrix[self.median as usize]
            .iter()
            .map(|x| {
                if *x == CordinateValue::Empty {
                    CordinateValue::Median
                } else {
                    *x
                }
            })
            .collect();
        self.matrix[matrix_len - self.median as usize] = median_line.clone();
    }

    pub fn set_origo(&mut self) -> Option<Cordinate> {
        let matrix = self.matrix.clone();

        let mut origo: Option<(usize, usize)> = None;

        if self.def_start < 0 && self.def_end > 0 {
            let origo_x = self.def_start.unsigned_abs() as usize;

            let y_float = (self.max_value / (self.max_value - self.min_value))
                * (self.matrix.len() as f64 - 1.0);

            let y_index = y_float.floor() as usize;
            origo = Some((origo_x, y_index));
            self.matrix[origo.unwrap().1][origo.unwrap().0] = CordinateValue::Origo;
        } else if self.def_start == 0 && self.def_end > 0 {
            self.matrix[matrix.len() - 1][0] = CordinateValue::Origo;
            origo = Some((0, matrix.len() - 1));
        } else if self.def_end == 0 && self.def_start < 0 {
            let x = self.def_start.unsigned_abs() as usize;
            let y = 0;
            self.matrix[y][x] = CordinateValue::Origo;
            origo = Some((x, y));
        }

        self.origo = origo.map(|(x, y)| Cordinate(x, y));
        self.origo
    }

    pub fn write(&mut self) -> Self {
        (self.def_start..self.def_end + 1).for_each(|equation_x| {
            let equation_y = self.problem.clone().solve(Some(equation_x as f64));

            let original_value = equation_y;

            let x = (equation_x - self.def_start) as usize;
            let min_original_value = self.min_value;
            let max_original_value = self.max_value;
            let min_scaled = 0.0;
            let max_scaled = self.matrix.len() as f64 - 1.0;

            let y = self.matrix.len()
                - 1
                - scale_value(
                    original_value,
                    min_original_value,
                    max_original_value,
                    min_scaled,
                    max_scaled,
                )
                .round()
                .abs() as usize;

            if self.matrix[y as usize][x] != CordinateValue::Origo {
                self.matrix[y as usize][x] = CordinateValue::Value;
            }
        });

        println!();

        self.matrix = self
            .matrix
            .iter()
            .enumerate()
            .map(|(index, y)| {
                if self.origo.is_some_and(|c| c.1 == index) {
                    y.iter()
                        .map(|x| match x {
                            CordinateValue::Empty => CordinateValue::HorizontalLine,
                            _ => *x,
                        })
                        .collect::<Vec<CordinateValue>>()
                } else {
                    y.clone()
                }
            })
            .collect::<Vec<Vec<CordinateValue>>>();
        self.clone()
    }
    pub fn graph(&self) {
        self.matrix.iter().enumerate().for_each(|(i, y)| {
            y.iter()
                .map(|x| match x {
                    CordinateValue::Empty => " ".to_string(),
                    CordinateValue::Value => "*".green().to_string(),
                    CordinateValue::Origo => "o".blue().to_string(),
                    CordinateValue::HorizontalLine => "-".blue().to_string(),
                    CordinateValue::VerticalLine => "|".to_string(),
                    CordinateValue::Average => "-".red().to_string(),
                    CordinateValue::Median => "-".dimmed().white().to_string(),
                })
                .for_each(|y| print!("{y}"));
            let number = scale_value(
                i as f64,
                self.matrix.len() as f64 - 1.0,
                0.0,
                self.min_value,
                self.max_value,
            )
            .round() as i64;
            print!(" {}", number);

            println!();
        });

        let footer = f!(
            "

values = {values_clr},
median = {median_clr},
middle_value = {medelvarde}

origo and origo line = {origo}, {origo_line}

",
            values_clr = "*".green(),
            median_clr = "-".dimmed().white(),
            origo = "o".blue(),
            origo_line = "-".blue(),
            medelvarde = "-".red()
        );
        println!("{footer}");
    }
}

fn scale_value(
    original_value: f64,
    min_original: f64,
    max_original: f64,
    min_scaled: f64,
    max_scaled: f64,
) -> f64 {
    ((original_value - min_original) / (max_original - min_original)) * (max_scaled - min_scaled)
        + min_scaled
}
