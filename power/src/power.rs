#![allow(clippy::upper_case_acronyms)]
use dist::Dist;
use dist::NoncentralChisq;
use dist::NoncentralF;
use dist::NoncentralT;
use roots::find_root_regula_falsi;
use roots::SimpleConvergency;
use serde_json::Value;

/// Supertype for all test types.
///
/// See the G*Power 3 paper for the equations for the distribution parameters
/// (https://doi.org/10.3758/BF03193146).
pub enum TestKind {
    /// Means: Difference from constant (one sample case).
    OneSampleTTest,
    /// Means: Difference between two independent means (two groups).
    IndependentSamplesTTest,
    /// Goodness-of-fit tests: Contingency tables.
    GoodnessOfFitChisqTest {
        /// Degrees of freedom.
        df: i64,
    },
    /// Linear multiple regression: Fixed model, R^2 deviation from zero.
    DeviationFromZeroMultipleRegression {
        /// Number of predictors (#A).
        n_predictors: i64,
    },
    /// Multiple regression: increase of R^2.
    IncreaseMultipleRegression {
        /// Total number of predictors (#A + #B).
        rho: i64,
        /// Number of tested predictors (#B).
        q: i64,
    },
    /// ANCOVA: Fixed effects, main effects and interattions.
    ANCOVA {
        /// Number of groups.
        /// In factorial ANCOVA is A*B*C.
        k: i64,
        /// Degrees of freedom of the tested effect.
        /// (number of factor levels - 1).
        /// In ANCOVA it depends on what factor you are interested,
        /// e.g. A, B, or C.
        q: i64,
        /// Number of covariates.
        p: i64,
    },
    /// ANOVA: Fixed effects, omnibus, one-way.
    OneWayANOVA {
        /// Number of groups.
        k: i64,
    },
    /// ANOVA: Fixed effects, special, main effects and interactions.
    TwoWayANOVA {
        /// Total number of cells in the design.
        k: i64,
        /// Degrees of freedom of the tested effect.
        /// (number of factor levels - 1).
        q: i64,
    },
    /// ANOVA: Repeated measures, between factors.
    BetweenRepeatedANOVA {
        /// Levels of between factor.
        k: i64,
        /// Levels of repeated measures.
        m: i64,
        /// Correlation among repeated measures.
        rho: f64,
    },
    /// ANOVA: Repeated measures, within factors.
    WithinRepeatedANOVA {
        /// Levels of between factor.
        k: i64,
        /// Levels of repeated measures.
        m: i64,
        /// Correlation among repeated measures.
        rho: f64,
        /// Nonsphericity correction.
        epsilon: f64,
    },
    /// ANOVA: Repeated measures, within-between interactions.
    WithinBetweenRepeatedANOVA {
        /// Levels of between factor.
        k: i64,
        /// Levels of repeated measures.
        m: i64,
        /// Correlation among repeated measures.
        rho: f64,
        /// Nonsphericity correction.
        epsilon: f64,
    },
}

#[derive(Clone, Debug)]
pub enum Tail {
    OneSided,
    TwoSided,
}

fn parse_i64(data: &Value, field: &str) -> Result<i64, String> {
    let value = match data.get(field) {
        Some(value) => value,
        None => return Err(format!("Missing field: {}", field)),
    };
    let value: &str = value
        .as_str()
        .expect("{field} could not be converted to a str");
    let value: i64 = value
        .parse()
        .expect("{field} could not be converted to an integer");
    Ok(value)
}

fn parse_f64(data: &Value, field: &str) -> Result<f64, String> {
    let value = match data.get(field) {
        Some(value) => value,
        None => return Err(format!("Missing field: {}", field)),
    };
    let value: &str = value
        .as_str()
        .expect("{field} could not be converted to a str");
    let value: f64 = value
        .parse()
        .expect("{field} could not be converted to a floating number");
    Ok(value)
}

impl Tail {
    pub fn from_json(data: &Value) -> Option<Tail> {
        let tail: i64 = parse_i64(data, "tail").unwrap();
        match tail {
            1 => Some(Tail::OneSided),
            2 => Some(Tail::TwoSided),
            _ => None,
        }
    }
}

impl TestKind {
    pub fn from_str(text: &str, data: &Value) -> Result<TestKind, String> {
        match text {
            "oneSampleTTest" => Ok(TestKind::OneSampleTTest),
            "independentSamplesTTest" => Ok(TestKind::IndependentSamplesTTest),
            "goodnessOfFitChisqTest" => {
                let df = parse_i64(data, "df").unwrap();
                Ok(TestKind::GoodnessOfFitChisqTest { df })
            }
            "deviationFromZeroMultipleRegression" => {
                let n_predictors = parse_i64(data, "nPredictors").unwrap();
                Ok(TestKind::DeviationFromZeroMultipleRegression { n_predictors })
            }
            "increaseMultipleRegression" => {
                let rho = parse_i64(data, "rho").unwrap();
                let q = parse_i64(data, "q").unwrap();
                Ok(TestKind::IncreaseMultipleRegression { rho, q })
            }
            "ANCOVA" => {
                let k = parse_i64(data, "k").unwrap();
                let q = parse_i64(data, "q").unwrap();
                let p = parse_i64(data, "p").unwrap();
                Ok(TestKind::ANCOVA { k, q, p })
            }
            "oneWayANOVA" => {
                let k = parse_i64(data, "k").unwrap();
                Ok(TestKind::OneWayANOVA { k })
            }
            "twoWayANOVA" => {
                let k = parse_i64(data, "k").unwrap();
                let q = parse_i64(data, "q").unwrap();
                Ok(TestKind::TwoWayANOVA { k, q })
            }
            "betweenRepeatedANOVA" => {
                let k = parse_i64(data, "k").unwrap();
                let m = parse_i64(data, "m").unwrap();
                let rho = parse_f64(data, "rho").unwrap();
                Ok(TestKind::BetweenRepeatedANOVA { k, m, rho })
            }
            "withinRepeatedANOVA" => {
                let k = parse_i64(data, "k").unwrap();
                let m = parse_i64(data, "m").unwrap();
                let rho = parse_f64(data, "rho").unwrap();
                let epsilon = parse_f64(data, "epsilon").unwrap();
                if epsilon < (1.0 / (m as f64 - 1.0)) {
                    Err(
                        "lower bound of ε corresponds to 1 / (number of measurements - 1)"
                            .to_string(),
                    )
                } else {
                    Ok(TestKind::WithinRepeatedANOVA { k, m, rho, epsilon })
                }
            }
            "withinBetweenRepeatedANOVA" => {
                let k = parse_i64(data, "k").unwrap();
                let m = parse_i64(data, "m").unwrap();
                let rho = parse_f64(data, "rho").unwrap();
                let epsilon = parse_f64(data, "epsilon").unwrap();
                if epsilon < (1.0 / (m as f64 - 1.0)) {
                    Err(
                        "lower bound of ε corresponds to 1 / (number of measurements - 1)"
                            .to_string(),
                    )
                } else {
                    Ok(TestKind::WithinBetweenRepeatedANOVA { k, m, rho, epsilon })
                }
            }
            _ => Err(format!("Unknown test: {}", text)),
        }
    }

    fn alternative_distribution(&self, n: f64, es: f64) -> Dist {
        match self {
            TestKind::OneSampleTTest => Box::new(NoncentralT::new(n - 1.0, n.sqrt() * es)),
            TestKind::IndependentSamplesTTest => {
                let v = n - 2.0; // n1 + n2 - 2
                Box::new(NoncentralT::new(v, (n / 2.0).sqrt() * es))
            }
            TestKind::DeviationFromZeroMultipleRegression { n_predictors } => {
                Box::new(NoncentralF::new(
                    *n_predictors as f64,
                    n - (*n_predictors as f64) - 1.0,
                    es.powi(2) * n,
                ))
            }
            TestKind::GoodnessOfFitChisqTest { df } => {
                Box::new(NoncentralChisq::new(*df as f64, es.powi(2) * n))
            }
            TestKind::IncreaseMultipleRegression { rho, q } => Box::new(NoncentralF::new(
                *q as f64,
                n - (*rho as f64) - 1.0,
                es.powi(2) * n,
            )),
            TestKind::ANCOVA { k, q, p } => Box::new(NoncentralF::new(
                *q as f64,
                n - *k as f64 - *p as f64 - 1.0,
                es.powi(2) * n,
            )),
            TestKind::OneWayANOVA { k } => Box::new(NoncentralF::new(
                *k as f64 - 1.0,
                n - *k as f64,
                es.powi(2) * n,
            )),
            TestKind::TwoWayANOVA { k, q } => {
                Box::new(NoncentralF::new(*q as f64, n - *k as f64, es.powi(2) * n))
            }

            TestKind::BetweenRepeatedANOVA { k, m, rho } => {
                let u = *m as f64 / (1.0 + ((*m as f64 - 1.0) * *rho));
                Box::new(NoncentralF::new(
                    *k as f64 - 1.0,
                    n - *k as f64,
                    es.powi(2) * u * n,
                ))
            }
            TestKind::WithinRepeatedANOVA { k, m, rho, epsilon } => {
                let u = *m as f64 / (1.0 - *rho);
                Box::new(NoncentralF::new(
                    (*m as f64 - 1.0) * *epsilon,
                    (n - *k as f64) * (*m as f64 - 1.0) * *epsilon,
                    es.powi(2) * u * n * *epsilon, // G*Power paper is missing the epsilon.
                ))
            }
            TestKind::WithinBetweenRepeatedANOVA { k, m, rho, epsilon } => {
                let u = *m as f64 / (1.0 - *rho);
                Box::new(NoncentralF::new(
                    (*k as f64 - 1.0) * (*m as f64 - 1.0) * *epsilon,
                    (n - *k as f64) * (*m as f64 - 1.0) * *epsilon,
                    es.powi(2) * u * n * *epsilon,
                ))
            }
        }
    }

    fn null_distribution(&self, n: f64, es: f64) -> Dist {
        self.alternative_distribution(n, es).central_distribution()
    }

    pub fn n(&self, tail: Tail, alpha: f64, power: f64, es: f64) -> i64 {
        let f = |n| self.alpha(tail.clone(), n, power, es) - alpha;
        let mut conv = SimpleConvergency {
            eps: 0.0001f64,
            max_iter: 500,
        };
        let step_size = 20;
        // There is probably a better way to do this, but it works.
        for lower in (0..1000).step_by(step_size) {
            let upper = lower + step_size;
            let root = find_root_regula_falsi(lower as f64, upper as f64, f, &mut conv);
            let n = root.unwrap_or(-111.0);
            if n == -111.0 || n.is_nan() {
                continue;
            }
            return n.ceil() as i64;
        }
        -111
    }

    pub fn alpha(&self, tail: Tail, n: f64, power: f64, es: f64) -> f64 {
        let d0 = self.null_distribution(n, es);
        let d1 = self.alternative_distribution(n, es);
        let critical_value = d1.quantile(power, false);
        let right_tail = d0.cdf(critical_value, false);
        match tail {
            Tail::OneSided => right_tail,
            Tail::TwoSided => 2.0 * right_tail,
        }
    }

    pub fn power(&self, tail: Tail, n: f64, alpha: f64, es: f64) -> f64 {
        let d0 = self.null_distribution(n, es);
        let d1 = self.alternative_distribution(n, es);
        let right_tail = match tail {
            Tail::OneSided => alpha,
            Tail::TwoSided => alpha / 2.0,
        };
        let critical_value = d0.quantile(right_tail, false);
        d1.cdf(critical_value, false)
    }

    pub fn es(&self, tail: Tail, n: f64, alpha: f64, power: f64) -> f64 {
        let f = |es| self.alpha(tail.clone(), n, power, es) - alpha;
        let mut conv = SimpleConvergency {
            eps: 0.0001f64,
            max_iter: 500,
        };
        let root = find_root_regula_falsi(0.001f64, 8f64, f, &mut conv);
        root.unwrap_or(-111.0)
    }
}
