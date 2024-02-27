use crate::problem::Problem;

// Denna funktionen används för att skala om ett värde från ett intervall till ett annat
// Inputs:
// original_value: Värdet som ska skalas om
// min_original: Lägsta värdet i originalintervallet
// max_original: Högsta värdet i originalintervallet
// min_scaled: Lägsta värdet i det nya intervallet
// max_scaled: Högsta värdet i det nya intervallet
pub fn scale_value(
    original_value: f64,
    min_original: f64,
    max_original: f64,
    min_scaled: f64,
    max_scaled: f64,
) -> f64 {
    ((original_value - min_original) / (max_original - min_original)) * (max_scaled - min_scaled)
        + min_scaled
}

// Simpel funktion för att skapa en tom matris i x-längd och y-längd
pub fn create_matrix(x_len: usize, y_len: usize) -> Vec<Vec<super::CordinateValue>> {
    vec![vec![super::CordinateValue::Empty; x_len]; y_len]
}

// Denna funktionen används för att hitta origo kordinaten när origo finns mellan definitionsvärdet.
// Inputs:
// def_start: startvärdet för definitionen
// max_value: maxvärdet för definitionen
// min_value: minvärdet för definitionen
// matrix_len: längden på matrisen (y)
pub fn pick_origo_when_middle(
    def_start: usize,
    max_value: f64,
    min_value: f64,
    matrix_len: usize,
) -> Option<(usize, usize)> {
    let y_float = (max_value / (max_value - min_value)) * (matrix_len as f64 - 1.0);

    let y_index = y_float.floor() as usize;
    if y_index >= matrix_len {
        return None;
    }
    Some((def_start, y_index))
}

// Denna funktionen används för att hitta origo kordinaten när vi vet x-värdet.
// Inputs:
// problem: Matteproblemet i form av structen Problem
// x_value: x-värdet för origo
// min_value: minvärdet för definitionen
// max_value: maxvärdet för definitionen
// min_scaled: Lägsta indexvärdet för y i matrixen
// max_scaled: Högsta indexvärdet för y i matrixen
pub fn pick_origo_when_x(
    problem: Problem,
    x_value: f64,
    min_value: f64,
    max_value: f64,
    min_scaled: f64,
    max_scaled: f64,
) -> (usize, usize) {
    // Origo x kommer alltid vara index 0 (längst till vänster på x axeln)
    // Vi kan räkna ut vad y-värdet är med hjälp av problemet och origo x
    // Detta går dock inte att använda för att rita ut y-värdena på grafen, pga att de inte är i synk med x-värdena
    let raw_y = problem.solve(Some(x_value));

    // Här skalar vi om y-värdet så att det passar in i matrixen med indexer
    let y = scale_value(raw_y, min_value, max_value, min_scaled, max_scaled) as usize;

    (x_value as usize, y)
}
