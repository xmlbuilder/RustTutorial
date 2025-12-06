use std::f64;
use std::fs::File;
use std::io::Write;

/// Normal quantile approximation (inverse CDF of N(0,1)).
/// Based on a well-known rational approximation (Moro/Wichura style).
pub fn normal_quantile_approx(p: f64) -> f64 {

    if p <= 0.0 || p >= 1.0 {
        panic!("normal_quantile_approx: p must be in (0,1)");
    }

    // Coefficients from a common approximation
    const A1: f64 = -3.969_683_028_665_376e+01;
    const A2: f64 = 2.209_460_984_245_205e+02;
    const A3: f64 = -2.759_285_104_469_687e+02;
    const A4: f64 = 1.383_577_518_672_690e+02;
    const A5: f64 = -3.066_479_806_614_716e+01;
    const A6: f64 = 2.506_628_277_459_239e+00;

    const B1: f64 = -5.447_609_879_822_406e+01;
    const B2: f64 = 1.615_858_368_580_409e+02;
    const B3: f64 = -1.556_989_798_598_866e+02;
    const B4: f64 = 6.680_131_188_771_972e+01;
    const B5: f64 = -1.328_068_155_288_572e+01;

    const C1: f64 = -7.784_894_002_430_293e-03;
    const C2: f64 = -3.223_964_580_411_365e-01;
    const C3: f64 = -2.400_758_277_161_838e+00;
    const C4: f64 = -2.549_732_539_343_734e+00;
    const C5: f64 = 4.374_664_141_464_968e+00;
    const C6: f64 = 2.938_163_982_698_783e+00;

    const D1: f64 = 7.784_695_709_041_462e-03;
    const D2: f64 = 3.224_671_290_700_398e-01;
    const D3: f64 = 2.445_134_137_142_996e+00;
    const D4: f64 = 3.754_408_661_907_416e+00;

    let q: f64;
    let r: f64;

    if p < 0.024_25 {
        // lower region
        q = (-2.0 * p.ln()).sqrt();
        (((((C1 * q + C2) * q + C3) * q + C4) * q + C5) * q + C6)
            / ((((D1 * q + D2) * q + D3) * q + D4) * q + 1.0)
    } else if p > 1.0 - 0.024_25 {
        // upper region
        q = (-2.0 * (1.0 - p).ln()).sqrt();
        -(((((C1 * q + C2) * q + C3) * q + C4) * q + C5) * q + C6)
            / ((((D1 * q + D2) * q + D3) * q + D4) * q + 1.0)
    } else {
        // central region
        q = p - 0.5;
        r = q * q;
        (((((A1 * r + A2) * r + A3) * r + A4) * r + A5) * r + A6) * q
            / (((((B1 * r + B2) * r + B3) * r + B4) * r + B5) * r + 1.0)
    }
}

/// Student t quantile approximation using normal quantile + small df correction.
pub fn student_t_quantile_approx(p: f64, df: f64) -> f64 {
    if df <= 0.0 {
        panic!("student_t_quantile_approx: df must be > 0");
    }
    let z = normal_quantile_approx(p);
    if df > 30.0 {
        return z;
    }
    let z3 = z * z * z;
    z + (z3 + z) / (4.0 * df)
}

/// Bonferroni-based ANOM h for equal n.
pub fn anom_h_bonferroni_equal_n(alpha: f64, a: usize, _n: usize, df: usize) -> f64 {
    if !(0.0 < alpha && alpha < 1.0) {
        panic!("alpha must be in (0,1)");
    }
    if a <= 1 || df == 0 {
        panic!("invalid a or df");
    }
    let alpha_per_group = alpha / (a as f64);
    let p = 1.0 - alpha_per_group / 2.0;
    let tcrit = student_t_quantile_approx(p, df as f64);
    tcrit * (((a - 1) as f64) / (a as f64)).sqrt()
}

/// Bonferroni-based t critical for unequal n (per-group margin uses sqrt(1/n_i)).
pub fn anom_tcrit_bonferroni(alpha: f64, a: usize, df: usize) -> f64 {
    let alpha_per_group = alpha / (a as f64);
    let p = 1.0 - alpha_per_group / 2.0;
    student_t_quantile_approx(p, df as f64)
}

/// Options for ANOM computation and SVG rendering.
#[derive(Clone, Copy, Debug)]
pub struct AnomOptions {
    pub alpha: f64,
    pub assume_equal_n: bool,
    pub bonferroni: bool,
    pub svg_width: f64,
    pub svg_height: f64,
    pub svg_margin: f64,
}

impl Default for AnomOptions {
    fn default() -> Self {
        Self {
            alpha: 0.05,
            assume_equal_n: true,
            bonferroni: true,
            svg_width: 900.0,
            svg_height: 500.0,
            svg_margin: 60.0,
        }
    }
}

/// Per-group ANOM result.
#[derive(Clone, Debug)]
pub struct AnomGroupResult {
    pub name: String,
    pub n: usize,
    pub mean: f64,
    pub margin: f64,
    pub udl: f64,
    pub ldl: f64,
    pub significant_high: bool,
    pub significant_low: bool,
}

#[derive(Clone, Debug)]
struct GroupData {
    name: String,
    values: Vec<f64>,
}

/// ANOM engine: groups + ANOM decision limits + SVG rendering.
#[derive(Clone, Debug)]
pub struct Anom {
    opt: AnomOptions,
    groups: Vec<GroupData>,
    computed: bool,
    grand_mean: f64,
    mse: f64,
    s_within: f64,
    results: Vec<AnomGroupResult>,
}

impl Anom {
    pub fn new(opt: AnomOptions) -> Self {
        Self {
            opt,
            groups: Vec::new(),
            computed: false,
            grand_mean: f64::NAN,
            mse: f64::NAN,
            s_within: f64::NAN,
            results: Vec::new(),
        }
    }

    pub fn add_group(&mut self, name: &str, values: &[f64]) {
        if values.is_empty() {
            panic!("group has no values: {}", name);
        }
        self.groups.push(GroupData {
            name: name.to_string(),
            values: values.to_vec(),
        });
        self.computed = false;
    }

    pub fn clear(&mut self) {
        self.groups.clear();
        self.results.clear();
        self.computed = false;
    }

    pub fn fit(&mut self) {
        if self.groups.is_empty() {
            panic!("no groups to fit");
        }
        let a = self.groups.len();

        // compute per-group mean and n
        let mut means = vec![0.0_f64; a];
        let mut ns = vec![0_usize; a];
        let mut total_n = 0_usize;
        for (i, g) in self.groups.iter().enumerate() {
            let n = g.values.len();
            ns[i] = n;
            total_n += n;
            let sum: f64 = g.values.iter().sum();
            means[i] = sum / (n as f64);
        }

        // grand mean (weighted)
        let mut grand_sum = 0.0_f64;
        for i in 0..a {
            grand_sum += means[i] * (ns[i] as f64);
        }
        self.grand_mean = grand_sum / (total_n as f64);

        // pooled within-group variance
        let mut ss_within = 0.0_f64;
        let mut df_within = 0_i32;
        for i in 0..a {
            let m_i = means[i];
            let n_i = ns[i];
            for x in &self.groups[i].values {
                let d = *x - m_i;
                ss_within += d * d;
            }
            df_within += (n_i as i32) - 1;
        }
        if df_within <= 0 {
            panic!("insufficient degrees of freedom");
        }
        self.mse = ss_within / (df_within as f64);
        self.s_within = self.mse.sqrt();

        // equal-n check
        let equal_n = if self.opt.assume_equal_n {
            ns.iter().all(|&n| n == ns[0])
        } else {
            false
        };

        let mut h = f64::NAN;
        let mut tcrit = f64::NAN;

        if self.opt.bonferroni {
            if equal_n {
                h = anom_h_bonferroni_equal_n(self.opt.alpha, a, ns[0], df_within as usize);
            } else {
                tcrit = anom_tcrit_bonferroni(self.opt.alpha, a, df_within as usize);
            }
        } else {
            // no Bonferroni: one two-sided tcrit
            let p = 1.0 - self.opt.alpha / 2.0;
            tcrit = student_t_quantile_approx(p, df_within as f64);
        }

        self.results.clear();
        for i in 0..a {
            let mean_i = means[i];
            let n_i = ns[i];
            let margin = if equal_n && !h.is_nan() {
                // equal-n ANOM margin; basic form
                h * self.s_within * (1.0 / (n_i as f64)).sqrt()
            } else {
                // general unequal-n case
                tcrit * self.s_within * (1.0 / (n_i as f64)).sqrt()
            };

            let udl = self.grand_mean + margin;
            let ldl = self.grand_mean - margin;
            let significant_high = mean_i > udl;
            let significant_low = mean_i < ldl;

            self.results.push(AnomGroupResult {
                name: self.groups[i].name.clone(),
                n: n_i,
                mean: mean_i,
                margin,
                udl,
                ldl,
                significant_high,
                significant_low,
            });
        }

        self.computed = true;
    }

    fn ensure_computed(&self) {
        if !self.computed {
            panic!("Anom::fit() has not been called");
        }
    }

    pub fn grand_mean(&self) -> f64 {
        self.ensure_computed();
        self.grand_mean
    }

    pub fn s_within(&self) -> f64 {
        self.ensure_computed();
        self.s_within
    }

    pub fn results(&self) -> &Vec<AnomGroupResult> {
        self.ensure_computed();
        &self.results
    }

    pub fn save_csv(&self, path: &str) {
        self.ensure_computed();
        let mut f = File::create(path)
            .unwrap_or_else(|e| panic!("cannot open file {}: {}", path, e));
        writeln!(
            f,
            "group,n,mean,margin,UDL,LDL,significant_high,significant_low"
        )
            .unwrap();
        for r in &self.results {
            writeln!(
                f,
                "{},{},{},{},{},{},{},{}",
                r.name,
                r.n,
                r.mean,
                r.margin,
                r.udl,
                r.ldl,
                if r.significant_high { 1 } else { 0 },
                if r.significant_low { 1 } else { 0 },
            )
                .unwrap();
        }
    }

    fn round2(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    /// Render a simple ANOM chart as SVG string.
    pub fn render_svg(&self) -> String {
        self.ensure_computed();

        let w = self.opt.svg_width;
        let h = self.opt.svg_height;
        let m = self.opt.svg_margin;
        let plot_w = w - 2.0 * m;
        let plot_h = h - 2.0 * m;

        // y-range from LDL/UDL/means
        let mut ymin = self.grand_mean;
        let mut ymax = self.grand_mean;
        for r in &self.results {
            ymin = ymin.min(r.ldl.min(r.mean));
            ymax = ymax.max(r.udl.max(r.mean));
        }
        let dy = if ymax == ymin { 1.0 } else { ymax - ymin };
        let pad = 0.05 * dy;
        ymin -= pad;
        ymax += pad;

        let y_to_px = |y: f64| -> f64 {
            let t = (y - ymin) / (ymax - ymin);
            h - m - t * plot_h
        };

        let a = self.results.len();
        let x_for_i = |i: usize| -> f64 {
            if a == 1 {
                m + plot_w * 0.5
            } else {
                let t = i as f64 / (a as f64 - 1.0);
                m + t * plot_w
            }
        };

        let mut svg = String::new();
        svg.push_str(&format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\">\n",
            w, h
        ));

        // background
        svg.push_str(&format!(
            "<rect x=\"0\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"#ffffff\"/>\n",
            w, h
        ));

        // axes
        svg.push_str(&format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#000\"/>\n",
            m,
            h - m,
            w - m,
            h - m
        ));
        svg.push_str(&format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#000\"/>\n",
            m,
            m,
            m,
            h - m
        ));

        // grand mean line
        svg.push_str(&format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#1f77b4\" stroke-dasharray=\"6,4\"/>\n",
            m,
            y_to_px(self.grand_mean),
            w - m,
            y_to_px(self.grand_mean),
        ));

        // global UDL/LDL
        let mut min_ldl = f64::INFINITY;
        let mut max_udl = f64::NEG_INFINITY;
        for r in &self.results {
            min_ldl = min_ldl.min(r.ldl);
            max_udl = max_udl.max(r.udl);
        }
        svg.push_str(&format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#d62728\" stroke-width=\"1.5\"/>\n",
            m,
            y_to_px(max_udl),
            w - m,
            y_to_px(max_udl)
        ));
        svg.push_str(&format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#2ca02c\" stroke-width=\"1.5\"/>\n",
            m,
            y_to_px(min_ldl),
            w - m,
            y_to_px(min_ldl)
        ));

        // group points and labels
        for (i, r) in self.results.iter().enumerate() {
            let x = x_for_i(i);
            let y = y_to_px(r.mean);

            let color = if r.significant_high {
                "#d62728"
            } else if r.significant_low {
                "#2ca02c"
            } else {
                "#555555"
            };

            svg.push_str(&format!(
                "<circle cx=\"{}\" cy=\"{}\" r=\"5\" fill=\"{}\"/>\n",
                x, y, color
            ));

            // per-group UDL/LDL ticks
            let y_udl = y_to_px(r.udl);
            let y_ldl = y_to_px(r.ldl);
            svg.push_str(&format!(
                "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#d62728\"/>\n",
                x - 12.0,
                y_udl,
                x + 12.0,
                y_udl
            ));
            svg.push_str(&format!(
                "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#2ca02c\"/>\n",
                x - 12.0,
                y_ldl,
                x + 12.0,
                y_ldl
            ));

            // group label
            svg.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" font-size=\"12\" text-anchor=\"middle\" fill=\"#000\">{}</text>\n",
                x,
                h - m + 18.0,
                r.name
            ));
        }

        // y-axis ticks: ymin, grand_mean, ymax
        svg.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" font-size=\"11\" text-anchor=\"end\">{}</text>\n",
            m - 8.0,
            y_to_px(ymax),
            Self::round2(ymax)
        ));
        svg.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" font-size=\"11\" text-anchor=\"end\">{}</text>\n",
            m - 8.0,
            y_to_px(self.grand_mean),
            Self::round2(self.grand_mean)
        ));
        svg.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" font-size=\"11\" text-anchor=\"end\">{}</text>\n",
            m - 8.0,
            y_to_px(ymin),
            Self::round2(ymin)
        ));

        svg.push_str("</svg>\n");
        svg
    }
}
