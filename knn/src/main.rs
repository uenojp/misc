use std::path::Path;

use csv::{self, Error, ReaderBuilder};
use serde::Deserialize;

// Cell nucleus features and their diagnostic results.
// 1) ID number
// 2) Diagnosis (M = malignant, B = benign)
// 3-32) Features
// ref. http://archive.ics.uci.edu/ml/datasets/Breast+Cancer+Wisconsin+%28Diagnostic%29
#[derive(Debug, Deserialize)]
pub struct CellNucleus {
    id: String,
    diagnosis: Diagnosis,
    features: Vec<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Diagnosis {
    M, // Malignant
    B, // Benign
}

// Classify whether cell nucleus x is Malignant or Bengin.
pub fn classify(trainings: &[CellNucleus], x: &[f64], k: usize) -> Diagnosis {
    // Each trainings data, compute the distance to x.
    let mut diagnosis_distances = Vec::new();
    for training in trainings {
        if let Some(distance) = euclid_distance(&training.features, x) {
            diagnosis_distances.push((training.diagnosis, distance));
        }
    }

    diagnosis_distances.sort_by(|(_, dist1), (_, dist2)| dist1.partial_cmp(dist2).unwrap());

    // Get the nearest k cell nuclei, and count the nuclei diagnosed as malignant.
    let count = diagnosis_distances
        .iter()
        .take(k)
        .filter(|(diag, _)| Diagnosis::M == *diag)
        .count();

    if k / 2 < count {
        Diagnosis::M
    } else {
        Diagnosis::B
    }
}

pub fn read_data_set<P: AsRef<Path>>(path: P) -> Result<Vec<CellNucleus>, Error> {
    let mut records = vec![];
    let mut reader = ReaderBuilder::new().has_headers(false).from_path(path)?;
    for record in reader.deserialize() {
        let record: CellNucleus = record?;
        records.push(record);
    }
    Ok(records)
}

fn main() -> Result<(), Error> {
    let trainings = read_data_set("data/training.data")?;
    let tests = read_data_set("data/test.data")?;

    for test in tests {
        let result = classify(&trainings, &test.features, 3);
        println!("Expected {:?}, Result {:?}", test.diagnosis, result);
    }

    Ok(())
}

fn euclid_distance(a: &[f64], b: &[f64]) -> Option<f64> {
    if a.len() != b.len() || a.is_empty() {
        return None;
    }
    Some(
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y) * (x - y))
            .sum::<f64>()
            .sqrt(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_euclid() {
        let a = vec![3.0, 5.0, 9.0];
        let b = vec![3.0, 3.0, 6.0];
        assert!(euclid_distance(&a, &b).unwrap() - 3.60555 <= 0.0001);

        let a = vec![];
        let b = vec![1.0, 2.0];
        assert_eq!(None, euclid_distance(&a, &b));

        let a = vec![1.0, 2.0];
        let b = vec![];
        assert_eq!(None, euclid_distance(&a, &b));

        let a = vec![];
        let b = vec![];
        assert_eq!(None, euclid_distance(&a, &b));
    }
}
