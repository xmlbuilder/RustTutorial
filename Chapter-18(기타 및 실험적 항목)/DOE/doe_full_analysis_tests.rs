
#[cfg(test)]
mod doe_full_analysis_tests {

    use std::f64;
    use nurbslib::doe::anom::{Anom, AnomOptions};
    use nurbslib::doe::doe_anom_response::{build_anom_for_all_factors, build_anom_for_factor};
    use nurbslib::doe::doe_full_analysis::{run_doe_full_analysis, DoeFullAnalysis};
    use nurbslib::doe::orthogonal_array::{build_design_from_orthogonal_array, oa_l4_2_3, oa_l8_2_7, FactorLevels};
    use nurbslib::doe::response_surface_quadratic::ResponseSurfaceQuadratic;
    use rand_distr::{Normal, Distribution};
    use rand::Rng;

    fn approx_equal(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() <= tol
    }

    #[test]
    fn test_anom_equal_n_basic() {
        let mut opt = AnomOptions::default();
        opt.alpha = 0.05;
        opt.assume_equal_n = true;
        opt.bonferroni = true;

        let mut anom = Anom::new(opt);
        anom.add_group("G1", &[10.0, 10.1, 9.9, 10.0]);
        anom.add_group("G2", &[10.5, 10.6, 10.4, 10.5]);
        anom.add_group("G3", &[9.7, 9.8, 9.6, 9.7]);
        anom.fit();

        let gm = anom.grand_mean();
        let s = anom.s_within();
        println!("grand_mean = {}, s_within = {}", gm, s);

        let res = anom.results();
        assert_eq!(res.len(), 3);
        assert_eq!(res[0].n, 4);
        assert_eq!(res[1].n, 4);
        assert_eq!(res[2].n, 4);

        println!(
            "G1 mean={}, G2 mean={}, G3 mean={}",
            res[0].mean, res[1].mean, res[2].mean
        );
        assert!(res[2].mean < res[0].mean);
        assert!(res[0].mean < res[1].mean);
    }

    #[test]
    fn test_anom_unequal_n() {
        let mut opt = AnomOptions::default();
        opt.alpha = 0.05;
        opt.assume_equal_n = true; // will detect unequal n inside
        opt.bonferroni = true;

        let mut anom = Anom::new(opt);
        anom.add_group("A", &[9.9, 10.1, 10.0, 9.8]); // n=4
        anom.add_group("B", &[10.5, 10.6, 10.4, 10.7, 10.3, 10.6]); // n=6
        anom.add_group("C", &[9.7, 9.6, 9.9]); // n=3
        anom.fit();

        let res = anom.results();
        assert_eq!(res.len(), 3);
        assert_eq!(res[0].n, 4);
        assert_eq!(res[1].n, 6);
        assert_eq!(res[2].n, 3);

        println!("grand_mean = {}, s_within = {}", anom.grand_mean(), anom.s_within());
        for r in res {
            println!(
                "{} n={} mean={} margin={} UDL={} LDL={} high={} low={}",
                r.name, r.n, r.mean, r.margin, r.udl, r.ldl, r.significant_high, r.significant_low
            );
        }
    }

    #[test]
    fn test_orthogonal_array_design() {
        let oa = oa_l4_2_3();
        assert_eq!(oa.runs, 4);
        assert_eq!(oa.factors, 3);

        let mut fl = Vec::new();
        for _ in 0..oa.factors {
            fl.push(FactorLevels {
                levels: vec![-1.0, 1.0],
            });
        }

        let design = build_design_from_orthogonal_array(&oa, &fl);
        assert_eq!(design.len(), oa.runs);
        assert_eq!(design[0].len(), oa.factors);

        for (r, row) in design.iter().enumerate() {
            print!("run {}: ", r);
            for val in row {
                print!("{} ", val);
            }
            println!();
        }

        // first row (0,0,0) -> (-1,-1,-1)
        assert!(approx_equal(design[0][0], -1.0, 1e-12));
        assert!(approx_equal(design[0][1], -1.0, 1e-12));
        assert!(approx_equal(design[0][2], -1.0, 1e-12));

        // last row (1,1,0) -> (1,1,-1)
        assert!(approx_equal(design[3][0], 1.0, 1e-12));
        assert!(approx_equal(design[3][1], 1.0, 1e-12));
        assert!(approx_equal(design[3][2], -1.0, 1e-12));
    }

    #[test]
    fn test_build_anom_for_factor() {
        let oa = oa_l8_2_7();
        let runs = oa.runs;

        // Response y depends only on factor A (index 0).
        let mut y = vec![0.0_f64; runs];
        let mut rng = rand::rng();
        let normal = Normal::new(0.0, 0.2).unwrap();

        for r in 0..runs {
            let lev_a = oa.at(r, 0);
            let mean_a = if lev_a == 0 { 10.0 } else { 12.0 };
            let noise: f64 = normal.sample(&mut rng);
            y[r] = mean_a + noise;
        }

        let mut opt = AnomOptions::default();
        opt.alpha = 0.05;
        opt.assume_equal_n = true;
        opt.bonferroni = true;

        let anom_a = build_anom_for_factor(&oa, &y, 0, "A", &opt);
        let res_a = anom_a.results();
        assert_eq!(res_a.len(), 2);
        println!("Factor A ANOM:");
        for r in res_a {
            println!(
                "{} n={} mean={} UDL={} LDL={} high={} low={}",
                r.name, r.n, r.mean, r.udl, r.ldl, r.significant_high, r.significant_low
            );
        }

        // also build ANOM for all factors
        let factor_names = vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
            "E".to_string(),
            "F".to_string(),
            "G".to_string(),
        ];
        let all_anoms = build_anom_for_all_factors(&oa, &y, &factor_names, &opt);
        assert_eq!(all_anoms.len(), 7);
    }

    #[test]
    fn test_response_surface_quadratic_fit() {
        // true model in 2 variables
        let b0 = 10.0;
        let b1 = 2.0;
        let b2 = -1.0;
        let b11 = 0.5;
        let b22 = -0.3;
        let b12 = 1.2;

        let true_model = |x1: f64, x2: f64| -> f64 {
            b0 + b1 * x1 + b2 * x2 + b11 * x1 * x1 + b22 * x2 * x2 + b12 * x1 * x2
        };

        let mut design: Vec<Vec<f64>> = Vec::new();
        let mut y: Vec<f64> = Vec::new();
        for i in -2..=2 {
            for j in -2..=2 {
                let x1 = (i as f64) * 0.5;
                let x2 = (j as f64) * 0.5;
                design.push(vec![x1, x2]);
                y.push(true_model(x1, x2));
            }
        }

        let mut rs = ResponseSurfaceQuadratic::new();
        let ok = rs.fit(&design, &y);
        assert!(ok);

        let beta = rs.coefficients();
        assert!(approx_equal(beta[0], b0, 1e-6));
        assert!(approx_equal(beta[1], b1, 1e-6));
        assert!(approx_equal(beta[2], b2, 1e-6));
        assert!(approx_equal(beta[3], b11, 1e-6));
        assert!(approx_equal(beta[4], b22, 1e-6));
        assert!(approx_equal(beta[5], b12, 1e-6));

        let x_test = [0.7, -0.4];
        let y_true = true_model(x_test[0], x_test[1]);
        let y_pred = rs.predict(&x_test);
        println!("y_true = {}, y_pred = {}", y_true, y_pred);
        assert!(approx_equal(y_true, y_pred, 1e-6));
    }

    #[test]
    fn test_doe_full_analysis() {
        let oa = oa_l8_2_7();
        let runs = oa.runs;

        // all factors are 2-level {-1,1}
        let mut all_levels = Vec::new();
        for _ in 0..oa.factors {
            all_levels.push(FactorLevels {
                levels: vec![-1.0, 1.0],
            });
        }

        let true_model = |x1: f64, x2: f64| -> f64 {
            10.0 + 2.0 * x1 + 3.0 * x2 + 1.0 * x1 * x1 - 0.5 * x2 * x2
        };

        let mut y = vec![0.0_f64; runs];
        let mut rng = rand::thread_rng();
        let normal = rand_distr::Normal::new(0.0, 0.2).unwrap();
        for r in 0..runs {
            let lev_a = oa.at(r, 0);
            let lev_b = oa.at(r, 1);
            let x1 = all_levels[0].levels[lev_a];
            let x2 = all_levels[1].levels[lev_b];
            let noise: f64 = normal.std_dev();
            y[r] = true_model(x1, x2) + noise;
        }

        let rs_factors = vec![0_usize, 1_usize]; // A,B for RS
        let factor_names = vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
            "E".to_string(),
            "F".to_string(),
            "G".to_string(),
        ];

        let mut opt = AnomOptions::default();
        opt.alpha = 0.05;
        opt.assume_equal_n = true;
        opt.bonferroni = true;

        let analysis: DoeFullAnalysis = run_doe_full_analysis(
            &oa,
            &all_levels,
            &rs_factors,
            &y,
            &factor_names,
            &opt,
        )
            .expect("run_doe_full_analysis failed");

        let beta = analysis.rs_model.coefficients();
        println!("RS beta = {}", beta);

        for fa in analysis.factor_anoms.iter() {
            println!(
                "ANOM factor {}: grand_mean={}, s_within={}",
                fa.factor_name,
                fa.anom.grand_mean(),
                fa.anom.s_within()
            );
            for r in fa.anom.results() {
                println!(
                    "  {} n={} mean={} UDL={} LDL={} high={} low={}",
                    r.name, r.n, r.mean, r.udl, r.ldl, r.significant_high, r.significant_low
                );
            }
        }
    }


}
