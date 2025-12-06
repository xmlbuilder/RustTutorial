
use crate::doe::anom::{Anom, AnomOptions};
use crate::doe::orthogonal_array::OrthogonalArray;

/// Factor + ANOM result pairing.
#[derive(Clone, Debug)]
pub struct FactorAnomResult {
    pub factor_name: String,
    pub anom: Anom,
}

/// Build ANOM for a single factor from OA and response y.
/// Each level of the factor becomes a group.
pub fn build_anom_for_factor(
    oa: &OrthogonalArray,
    y: &[f64],
    factor_idx: usize,
    factor_name: &str,
    opt: &AnomOptions,
) -> Anom {
    assert_eq!(oa.runs, y.len());
    assert!(factor_idx < oa.factors);

    // Group responses by level index
    let mut level_values: Vec<Vec<f64>> = vec![Vec::new(); oa.levels];
    for r in 0..oa.runs {
        let lev = oa.at(r, factor_idx);
        level_values[lev].push(y[r]);
    }

    let mut anom = Anom::new(*opt);
    for (lev, vals) in level_values.iter().enumerate() {
        if vals.is_empty() {
            continue;
        }
        let name = format!("{}_L{}", factor_name, lev + 1);
        anom.add_group(&name, vals);
    }
    anom.fit();
    anom
}

/// Build ANOM for all factors.
/// If factor_names is empty, generate "A","B","C",... automatically.
pub fn build_anom_for_all_factors(
    oa: &OrthogonalArray,
    y: &[f64],
    factor_names: &[String],
    opt: &AnomOptions,
) -> Vec<FactorAnomResult> {
    assert_eq!(oa.runs, y.len());

    let names: Vec<String> = if factor_names.is_empty() {
        (0..oa.factors)
            .map(|i| ((b'A' + (i as u8)) as char).to_string())
            .collect()
    } else {
        assert_eq!(factor_names.len(), oa.factors);
        factor_names.to_vec()
    };

    let mut result = Vec::with_capacity(oa.factors);
    for f_idx in 0..oa.factors {
        let anom = build_anom_for_factor(oa, y, f_idx, &names[f_idx], opt);
        result.push(FactorAnomResult {
            factor_name: names[f_idx].clone(),
            anom,
        });
    }
    result
}
