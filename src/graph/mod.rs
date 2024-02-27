use crate::prelude::*;
use crate::problem::Problem;
use colored::Colorize;
use std::collections::HashMap;
mod utils;

// Utilstruct som används för att skapa en kordinat, där första usize är x och andra är y
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cordinate(usize, usize);

// Detta är en enum som används för att rendera graphen. Den används för att veta vilken symbol för att använda.
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
// Graph är en struct som används för att skapa en graf av ett "Problem" struct.
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
    // new skapar en Graph struct. Den tar in ett definitionsvärde och problemet som ska lösas.
    // Den skapar sedan ett kordinatsystem som är en matris av CordinateValue.
    // NOTERA: Bredden och höjden på matrisen är inte lika med definitionen, eller värdena som ska lösas.
    // Detta är för att det ska vara lättare att skala värdena till matrisen, och för att graphen ska kunna hantera väldigt stora värden.
    // Den måste göra så här för att indexerna måste inte vara i synk med x-defintionen.

    // Den räknar ut hur lång matrixen ska vara genom att ta hur många x-värden som har ett unikt y-värde på y-axeln. Det gör att så lite rader av graphen som möjligt är tomma.
    // EXEMPEL: Problem: x^2, definition: -4..4, då kommer det att finnas 5 unika y-värden, där med 5 x-värden.
    // Unika: 16, 9, 4, 1, 0
    pub fn new(def_start: i64, def_end: i64, problem: &'a Problem) -> Self {
        // Här och några rader nedanför används en HashMap vilken är en datastruktur som sparar data i key, value
        // HashMap används pga alla keys måste vara unika, vilket gör att vi kan räkna ut hur många unika y-värden det finns. med HashMap.len()
        let mut rows = HashMap::new();

        for x in def_start..def_end + 1 {
            let y_before: f64 = problem.clone().solve(Some(x as f64 - 1.0));

            let mut y = problem.clone().solve(Some(x as f64));

            // Här beräknas derivatan av y-värdena för att se om det är en asymptot
            // Om det är det så sätts y till 10000 eller -10000
            // Detta måste göras på grund av hur algoritment av origin funkar.
            // Den algoritmen är beroende av att det finns ett rimligt maximi/minimi y värde
            if y / y_before == f64::INFINITY && y_before != 0.0 {
                y = 10000.0
            }
            if y / y_before == f64::NEG_INFINITY && y_before != 0.0 {
                y = -10000.0
            }

            rows.insert(y.to_string(), x);
        }

        let iter = rows
            .keys()
            .map(|y| y.parse::<f64>().unwrap().round() as i64);

        let max_value = iter.clone().max().unwrap() as f64;
        let min_value = iter.clone().min().unwrap() as f64;

        Self {
            origo: None,
            matrix: utils::create_matrix(
                (def_end + 1 - def_start) as usize,
                (def_end - def_start + 1) as usize,
            ),
            problem,
            def_end,
            def_start,
            min_value,
            max_value,
            average_value: None,
            median: (iter.len() - 1) as i64 / 2,
        }
    }

    // Funktionen tar in en mutable pointer till en initierad struct, och modifierar self.matrix för att rita ut average värdet i en hel rad (y rad). Den returnar ingenting utan bara modifierar variabler som är tillgängligt utanför sitt scope.
    pub fn average(&mut self) {
        let sum: f64 = (self.def_start..self.def_end + 1)
            .map(|x| self.problem.clone().solve(Some(x as f64)))
            .sum();

        self.average_value = Some(sum / (self.def_end - self.def_start) as f64);
        let min_original_value = self.min_value;
        let max_original_value = self.max_value;
        let min_scaled = 0.0;
        let max_scaled = self.matrix.len() as f64 - 1.0;
        let y = self.matrix.len() as f64
            - 1.0
            - utils::scale_value(
                self.average_value.unwrap(),
                min_original_value,
                max_original_value,
                min_scaled,
                max_scaled,
            )
            .round()
            .abs();

        if y != 0.0 && y >= 0.0 {
            for x_index in 0..self.matrix[0].len() {
                if self.matrix[y as usize][x_index] == CordinateValue::Empty {
                    self.matrix[y as usize][x_index] = CordinateValue::Average;
                }
            }
        }
    }

    // Funktionen tar in en mutable pointer till en initierad struct, och modifierar self.matrix för att rita ut median värdet i en hel rad (y rad). Den returnar ingenting utan bara modifierar variabler som är tillgängligt utanför sitt scope.
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

    // Origo är punkten där x och y axeln är 0.
    // På grund av att matrixens båda indexar inte har någonting att göra med x och y axeln så måste origo räknas ut.
    // På så sätt kan y värdena ritas till grafen med hjälp av origo kordinaterna plus y-värdet.
    pub fn set_origo(&mut self) -> Option<Cordinate> {
        let matrix = self.matrix.clone();

        let mut origo: Option<(usize, usize)> = None;

        if self.def_start < 0 && self.def_end > 0 {
            let origo_kanske_finns = utils::pick_origo_when_middle(
                self.def_start.unsigned_abs() as usize,
                self.max_value,
                self.min_value,
                matrix.len() - 1,
            );
            // Om origo_kanske_finns är None så finns det ingen origo, och None returnas av funkionen
            match origo_kanske_finns {
                None => return None,
                Some((x, y)) => {
                    origo = Some((x, y));
                    self.matrix[y][x] = CordinateValue::Origo;
                }
            }
        } else if self.def_start == 0 && self.def_end > 0 {
            // Origo x kommer alltid vara index 0 (längst till vänster på x axeln)
            // Vi kan räkna ut vad y-värdet är med hjälp av problemet och origo x
            // Detta går dock inte att använda för att rita ut y-värdena på grafen, pga att de inte är i synk med x-värdena
            // Hjälp funktionen pick_origo_when_x räknar ut origo kordinaterna och skalar till graphen
            let (origo_x, origo_y) = utils::pick_origo_when_x(
                self.problem.clone(),
                0.0,
                self.min_value,
                self.max_value,
                self.matrix.len() as f64 - 1.0,
                0.0,
            );

            self.matrix[origo_y][origo_x] = CordinateValue::Origo;
            origo = Some((origo_x, origo_y));
        } else if self.def_end == 0 && self.def_start < 0 {
            // Origo x kommer alltid vara index self.def_start (längst till höger på x axeln)
            // Vi kan räkna ut vad y-värdet är med hjälp av problemet och origo x
            // Detta går dock inte att använda för att rita ut y-värdena på grafen, pga att de inte är i synk med x-värdena
            // Hjälp funktionen pick_origo_when_x räknar ut origo kordinaterna och skalar till graphen
            let (origo_x, origo_y) = utils::pick_origo_when_x(
                self.problem.clone(),
                self.def_start.abs() as f64,
                self.min_value,
                self.max_value,
                0.0,
                self.matrix.len() as f64 - 1.0,
            );

            self.matrix[origo_y][origo_x] = CordinateValue::Origo;
            origo = Some((origo_x, origo_y));
        }

        self.origo = origo.map(|(x, y)| Cordinate(x, y));
        self.origo
    }

    // Funktionen tar in en mutable pointer till en initierad struct, och modifierar self.matrix för att rita ut y-värdena (i form av CordinateValue::Value) på rätt index och subindex grafen. Den returnar ingenting utan bara modifierar variabler som är tillgängligt utanför sitt scope.
    pub fn write(&mut self) -> Self {
        (self.def_start..self.def_end + 1).for_each(|equation_x| {
            let equation_y = self.problem.clone().solve(Some(equation_x as f64));

            let original_value = equation_y;

            let x = (equation_x - self.def_start) as usize;

            let scaled = utils::scale_value(
                original_value,
                self.min_value,
                self.max_value,
                0.0,
                self.matrix.len() as f64 - 1.0,
            )
            .round()
            .abs() as usize;

            // y index är egentligen matrix.len() - 1 - y_index fått från scale_value
            let y = self.matrix.len() - 1 - scaled.min(self.matrix.len() - 1);

            if self.matrix[y as usize][x] != CordinateValue::Origo {
                self.matrix[y as usize][x] = CordinateValue::Value;
            }
        });

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

    // Denna funktionen tar emot en pointer till en initierad struct och bara renderar matrixen till en graph. Inget superkompliceat.
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
            let number = utils::scale_value(
                i as f64,
                self.matrix.len() as f64,
                0.0,
                self.min_value,
                self.max_value,
            )
            .round() as i64;
            print!(" {}", number);

            if i == 0 {
                print!(" = Y");
            }
            let is_average = !y
                .iter()
                .filter(|x| x == &&CordinateValue::Average)
                .collect::<Vec<&CordinateValue>>()
                .is_empty();
            let is_median = !y
                .iter()
                .filter(|x| x == &&CordinateValue::Median)
                .collect::<Vec<&CordinateValue>>()
                .is_empty();

            if is_average && !is_median {
                print!(" {} average", "<--".red());
            }

            if !is_average && is_median {
                print!(" {} median", "<--".dimmed().white());
            }
            println!();
        });
        println!(
            "{} {} {} = X",
            self.def_start,
            if self.matrix[0].len() > 5 {
                "-".repeat(self.matrix[0].len() - self.def_end.to_string().len() - 4)
            } else {
                "".to_string()
            },
            self.def_end
        );

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
