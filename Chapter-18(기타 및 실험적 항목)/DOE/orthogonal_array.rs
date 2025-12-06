use std::fmt;

/// Orthogonal array structure (Taguchi OA)
#[derive(Clone, Debug)]
pub struct OrthogonalArray {
    pub runs: usize,
    pub factors: usize,
    pub levels: usize,
    /// Row-major: data[run * factors + factor] = level index (0-based).
    pub data: Vec<usize>,
}

impl OrthogonalArray {
    #[inline]
    pub fn at(&self, run: usize, factor: usize) -> usize {
        self.data[run * self.factors + factor]
    }
}

/// Physical levels for a factor (maps OA level index -> numeric value).
#[derive(Clone, Debug)]
pub struct FactorLevels {
    pub levels: Vec<f64>,
}

impl fmt::Display for OrthogonalArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "OrthogonalArray(runs={}, factors={}, levels={})",
            self.runs, self.factors, self.levels
        )?;
        for r in 0..self.runs {
            write!(f, "run {:>2}: ", r)?;
            for c in 0..self.factors {
                write!(f, "{} ", self.at(r, c))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

/// Build numeric design matrix from OA and factor levels.
/// design[run][factor] = physical numeric value.
pub fn build_design_from_orthogonal_array(
    oa: &OrthogonalArray,
    factors: &[FactorLevels],
) -> Vec<Vec<f64>> {
    assert_eq!(factors.len(), oa.factors);
    let mut design = vec![vec![0.0_f64; oa.factors]; oa.runs];
    for r in 0..oa.runs {
        for c in 0..oa.factors {
            let lev = oa.at(r, c);
            design[r][c] = factors[c].levels[lev];
        }
    }
    design
}

/// Build numeric design matrix only for selected factor indices.
pub fn build_design_from_orthogonal_array_for_factors(
    oa: &OrthogonalArray,
    all_levels: &[FactorLevels],
    factor_indices: &[usize],
) -> Vec<Vec<f64>> {
    for &idx in factor_indices {
        assert!(idx < oa.factors);
    }
    let mut design = vec![vec![0.0_f64; factor_indices.len()]; oa.runs];
    for r in 0..oa.runs {
        for (j, &f_idx) in factor_indices.iter().enumerate() {
            let lev = oa.at(r, f_idx);
            design[r][j] = all_levels[f_idx].levels[lev];
        }
    }
    design
}

/// L4(2^3): 4 runs, 3 factors, 2 levels
pub fn oa_l4_2_3() -> OrthogonalArray {
    OrthogonalArray {
        runs: 4,
        factors: 3,
        levels: 2,
        data: vec![
            // F1 F2 F3
            0, 0, 0,
            0, 1, 1,
            1, 0, 1,
            1, 1, 0,
        ],
    }
}

/// L8(2^7): 8 runs, 7 factors, 2 levels
pub fn oa_l8_2_7() -> OrthogonalArray {
    OrthogonalArray {
        runs: 8,
        factors: 7,
        levels: 2,
        data: vec![
            // F1 F2 F3 F4 F5 F6 F7
            0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 1, 1, 1,
            0, 1, 1, 0, 0, 1, 1,
            0, 1, 1, 1, 1, 0, 0,
            1, 0, 1, 0, 1, 0, 1,
            1, 0, 1, 1, 0, 1, 0,
            1, 1, 0, 0, 1, 1, 0,
            1, 1, 0, 1, 0, 0, 1,
        ],
    }
}

/// L9(3^4): 9 runs, 4 factors, 3 levels
pub fn oa_l9_3_4() -> OrthogonalArray {
    OrthogonalArray {
        runs: 9,
        factors: 4,
        levels: 3,
        data: vec![
            // F1 F2 F3 F4 (1..3 mapped to 0..2)
            0, 0, 0, 0,
            0, 1, 1, 1,
            0, 2, 2, 2,
            1, 0, 1, 2,
            1, 1, 2, 0,
            1, 2, 0, 1,
            2, 0, 2, 1,
            2, 1, 0, 2,
            2, 2, 1, 0,
        ],
    }
}

/// L18(2^1 * 3^7): 18 runs, 8 factors (F1:2-level, others:3-level)
pub fn oa_l18_2_1_3_7() -> OrthogonalArray {
    OrthogonalArray {
        runs: 18,
        factors: 8,
        levels: 3,
        data: vec![
            // F1 F2 F3 F4 F5 F6 F7 F8
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1, 1, 1, 1, 1, 1,
            0, 0, 2, 2, 2, 2, 2, 2,
            0, 1, 0, 0, 1, 1, 2, 2,
            0, 1, 1, 1, 2, 2, 0, 0,
            0, 1, 2, 2, 0, 0, 1, 1,
            0, 2, 0, 1, 0, 2, 1, 2,
            0, 2, 1, 2, 1, 0, 2, 0,
            0, 2, 2, 0, 2, 1, 0, 1,
            1, 0, 0, 2, 2, 1, 1, 0,
            1, 0, 1, 0, 0, 2, 2, 1,
            1, 0, 2, 1, 1, 0, 0, 2,
            1, 1, 0, 1, 2, 0, 2, 1,
            1, 1, 1, 2, 0, 1, 0, 2,
            1, 1, 2, 0, 1, 2, 1, 0,
            1, 2, 0, 2, 1, 2, 0, 1,
            1, 2, 1, 0, 2, 0, 1, 2,
            1, 2, 2, 1, 0, 1, 2, 0,
        ],
    }
}
