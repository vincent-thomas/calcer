use crate::problem::Problem;

#[derive(Debug, Clone)]
pub struct Cordinate(usize, usize);

#[derive(Debug, Clone)]
pub enum CordinateValue {
    Empty,
    Value(Cordinate),
    Origo,
}

#[derive(Debug)]
pub struct Graph {
    pub matrix: Vec<Vec<CordinateValue>>,
    origo: Option<Cordinate>,
    problem: Problem,
    def_end: i64,
    def_start: i64,
    incr: i64,
}

impl Graph {
    pub fn new(def_start: i64, def_end: i64, problem: Problem, incr: i64) -> Self {
        let max_value = problem.clone().solve(Some(def_end as f64));
        let min_value = problem.clone().solve(Some(def_start as f64));
        let minus_origo_y = (min_value / incr as f64).round();
        let plus_origo_y = (max_value / incr as f64).ceil();

        println!("Thing: {}", &(plus_origo_y.abs()));

        Self {
            origo: None,
            matrix: vec![
                vec![CordinateValue::Empty; (def_end - def_start + 1) as usize];
                (plus_origo_y - minus_origo_y).abs() as usize + 1 // + 1 for the origo
            ],
            problem,
            def_end,
            def_start,
            incr,
        }
    }

    pub fn set_origo(&mut self) -> Cordinate {
        dbg!(self.def_end);
        dbg!(self.def_start);

        // Räkna ut värdemängden
        let max_value = self.problem.clone().solve(Some(self.def_end as f64));

        let origo_y = (max_value / self.incr as f64).ceil() as u64;

        let origo_x = self.def_start / 10;
        dbg!(origo_x, origo_y);

        self.matrix[origo_y as usize][origo_x as usize] = CordinateValue::Origo;

        Cordinate(origo_x as usize, origo_y as usize)
    }
}
