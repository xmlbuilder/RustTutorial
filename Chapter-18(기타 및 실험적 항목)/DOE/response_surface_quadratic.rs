use std::fs::File;
use std::io::Write;
use nalgebra::{DMatrix, DVector};

/// Quadratic response surface:
/// y ≈ b0 + sum_i b_i x_i + sum_i b_ii x_i^2 + sum_{i<j} b_ij x_i x_j
#[derive(Clone, Debug)]
pub struct ResponseSurfaceQuadratic {
    k: usize,
    fitted: bool,
    beta: DVector<f64>,
}

impl ResponseSurfaceQuadratic {
    pub fn new() -> Self {
        Self {
            k: 0,
            fitted: false,
            beta: DVector::zeros(0),
        }
    }

    /// Fit using linear least squares via QR decomposition.
    /// design: N x k points, y: length N responses.
    pub fn fit(&mut self, design: &[Vec<f64>], y: &[f64]) -> bool {
        let n = design.len();
        if n == 0 || y.len() != n {
            return false;
        }
        let k = design[0].len();
        for row in design.iter() {
            if row.len() != k {
                return false;
            }
        }

        let num_cross = k * (k - 1) / 2;
        let m = 1 + k + k + num_cross; // constant + linear + squared + interactions

        let mut phi = DMatrix::<f64>::zeros(n, m);
        let mut yy = DVector::<f64>::zeros(n);

        for (r, x) in design.iter().enumerate() {
            let mut col = 0;

            // constant term
            phi[(r, col)] = 1.0;
            col += 1;

            // linear terms
            for i in 0..k {
                phi[(r, col)] = x[i];
                col += 1;
            }

            // squared terms
            for i in 0..k {
                phi[(r, col)] = x[i] * x[i];
                col += 1;
            }

            // interaction terms (i < j)
            for i in 0..k {
                for j in (i + 1)..k {
                    phi[(r, col)] = x[i] * x[j];
                    col += 1;
                }
            }

            yy[r] = y[r];
        }

        /*
        // QR least squares: min ||phi*beta - yy||
        let qr = phi.clone().qr();
        let beta = qr.solve(&yy).expect("solve failed");
        */
        let svd = phi.svd(true, true);
        let beta = svd.solve(&yy, 1.0e-12).expect("solve failed");
        if beta.len() != m {
            return false; // some error
        }

        self.k = k;
        self.beta = beta;
        self.fitted = true;
        true
    }

    /// Predict at a single point x.
    pub fn predict(&self, x: &[f64]) -> f64 {
        if !self.fitted {
            panic!("ResponseSurfaceQuadratic::predict: model not fitted yet");
        }
        if x.len() != self.k {
            panic!("ResponseSurfaceQuadratic::predict: dimension mismatch");
        }

        let k = self.k;
        let num_cross = k * (k - 1) / 2;
        let m = 1 + k + k + num_cross;
        let mut phi = DVector::<f64>::zeros(m);
        let mut col = 0;

        phi[col] = 1.0;
        col += 1;

        for i in 0..k {
            phi[col] = x[i];
            col += 1;
        }

        for i in 0..k {
            phi[col] = x[i] * x[i];
            col += 1;
        }

        for i in 0..k {
            for j in (i + 1)..k {
                phi[col] = x[i] * x[j];
                col += 1;
            }
        }

        self.beta.dot(&phi)
    }

    pub fn num_factors(&self) -> usize {
        self.k
    }

    pub fn coefficients(&self) -> &DVector<f64> {
        &self.beta
    }
}

pub fn on_export_response_surface_csv(
    rs: &ResponseSurfaceQuadratic,
    x1_min: f64,
    x1_max: f64,
    x2_min: f64,
    x2_max: f64,
    nx: usize,
    ny: usize,
    path: &str,
) {
    let mut f = File::create(path).expect("cannot create csv");
    writeln!(f, "x1,x2,y").unwrap();

    for i in 0..nx {
        let t1 = i as f64 / (nx - 1) as f64;
        let x1 = x1_min + t1 * (x1_max - x1_min);
        for j in 0..ny {
            let t2 = j as f64 / (ny - 1) as f64;
            let x2 = x2_min + t2 * (x2_max - x2_min);
            let y = rs.predict(&[x1, x2]);
            writeln!(f, "{},{},{}", x1, x2, y).unwrap();
        }
    }
}