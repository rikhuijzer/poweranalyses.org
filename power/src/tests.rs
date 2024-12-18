use crate::interface::handle_received;
use serde_json::json;
#[cfg(test)]
use serde_json::Value;

const ES: f64 = 0.5;
const ALPHA: f64 = 0.05;
const POWER: f64 = 0.95;
const N: f64 = 50.0;

/// Join two JSON objects; note that the second object takes precedence.
fn join_json(a: &Value, b: &Value) -> Value {
    let mut result = a.to_owned();
    for (key, value) in b.as_object().unwrap() {
        result[key] = value.to_owned();
    }
    result
}

#[test]
fn json_joining() {
    let a = json!({"a": 1});
    let b = json!({"b": 2});
    let c = json!({"a": 1, "b": 2});
    assert_eq!(join_json(&a, &b), c);
}

fn with_base(input: &Value) -> Value {
    let base = default_input();
    join_json(&base, input)
}

fn test_interface(input: &Value, output: f64) {
    let analysis = input["analysis"].as_str().unwrap();
    let text = input.to_owned().to_string();
    let returned = handle_received(&text);
    let result = returned[analysis].as_f64().unwrap();
    assert_eq!(result, output);
}

fn default_input() -> Value {
    json!({
        "n": N,
        "alpha": ALPHA,
        "power": POWER,
        "es": ES,
    })
}

fn with_rest(test: &str) -> impl Fn(&Value) -> Value {
    let extra = json!({"test": test});
    move |input| join_json(&with_base(input), &extra)
}

#[test]
fn one_sample_t_test() {
    let join = with_rest("oneSampleTTest");
    let extra = json!({"tail": "1", "analysis": "alpha"});
    test_interface(&join(&extra), 0.034);
    let extra = json!({"tail": "2", "analysis": "alpha"});
    test_interface(&join(&extra), 0.067);
    let extra = json!({"tail": "1", "analysis": "power"});
    test_interface(&join(&extra), 0.967);
    let extra = json!({"tail": "2", "analysis": "power"});
    test_interface(&join(&extra), 0.934);
    let extra = json!({"tail": "1", "analysis": "es"});
    test_interface(&join(&extra), 0.472);
    let extra = json!({"tail": "2", "analysis": "es"});
    test_interface(&join(&extra), 0.520);
    let extra = json!({"tail": "1", "analysis": "n"});
    test_interface(&join(&extra), 45.0);
    let extra = json!({"tail": "2", "analysis": "n"});
    test_interface(&join(&extra), 54.0);
}

#[test]
fn independent_samples_t_test() {
    // G*Power only gives 0.392 if you put sample size group 1 and 2 both on n=50.
    // pwr.t.test(n=50, d=0.5, sig.level=NULL, power=0.95, type="two.sample", alternative="two.sided")
    let join = with_rest("independentSamplesTTest");
    let extra = json!({"tail": "2", "analysis": "alpha"});
    test_interface(&join(&extra), 0.398);
    let extra = json!({"tail": "1", "analysis": "n"});
    test_interface(&join(&extra), 88.0);
}

#[test]
fn goodness_of_fit_chisq() {
    let df = "5";
    let join = with_rest("goodnessOfFitChisqTest");
    let extra = json!({"df": df, "analysis": "alpha"});
    test_interface(&join(&extra), 0.254);
    let extra = json!({"df": df, "es": 0.628, "analysis": "alpha"});
    test_interface(&join(&extra), 0.051);
    let extra = json!({"df": df, "analysis": "power"});
    test_interface(&join(&extra), 0.788);
    let extra = json!({"df": df, "analysis": "es"});
    test_interface(&join(&extra), 0.629);
    let extra = json!({"df": df, "analysis": "n"});
    test_interface(&join(&extra), 80.0);
}

#[test]
fn deviation_from_zero_multiple_regression() {
    let join = with_rest("deviationFromZeroMultipleRegression");
    let f_squared = ES.sqrt();
    let extra = json!({"nPredictors": "2", "es": f_squared, "analysis": "alpha"});
    test_interface(&join(&extra), 0.006);
    let extra = json!({"nPredictors": "2", "es": f_squared, "analysis": "power"});
    test_interface(&join(&extra), 0.994);
    let extra = json!({"nPredictors": "2", "es": f_squared, "analysis": "es"});
    test_interface(&join(&extra), 0.574); // take the sqrt of G*Power.
    let extra = json!({"nPredictors": "2", "es": f_squared, "analysis": "n"});
    test_interface(&join(&extra), 35.0);
}

#[test]
fn increase_multiple_regression() {
    let rho = "5";
    let q = "2";
    let f_squared = ES.sqrt();
    let join = with_rest("increaseMultipleRegression");
    let extra = json!({"rho": rho, "q": q, "es": f_squared, "analysis": "alpha"});
    test_interface(&join(&extra), 0.006);
    let extra = json!({"rho": rho, "q": q, "es": f_squared, "analysis": "power"});
    test_interface(&join(&extra), 0.994);
    let extra = json!({"rho": rho, "q": q, "es": f_squared, "analysis": "es"});
    test_interface(&join(&extra), 0.575); // take the sqrt of G*Power.
    let extra = json!({"rho": rho, "q": q, "es": f_squared, "analysis": "n"});
    test_interface(&join(&extra), 35.0);
}

#[test]
fn one_way_anova_test() {
    let k = "5";
    let join = with_rest("oneWayANOVA");
    let extra = json!({"k": k, "analysis": "alpha"});
    test_interface(&join(&extra), 0.247);
    let extra = json!({"k": k, "analysis": "power"});
    test_interface(&join(&extra), 0.773);
    let extra = json!({"k": k, "analysis": "es"});
    test_interface(&join(&extra), 0.643);
    let extra = json!({"k": k, "analysis": "n"});
    test_interface(&join(&extra), 80.0);
}

#[test]
fn two_way_anova_test() {
    let k = "5";
    let q = "10";
    let join = with_rest("twoWayANOVA");
    let extra = json!({"k": k, "q": q, "analysis": "alpha"});
    test_interface(&join(&extra), 0.465);
    let extra = json!({"k": k, "q": q, "analysis": "power"});
    test_interface(&join(&extra), 0.560);
    let extra = json!({"k": k, "q": q, "analysis": "es"});
    test_interface(&join(&extra), 0.770);
    let extra = json!({"k": k, "q": q, "analysis": "n"});
    test_interface(&join(&extra), 107.0);
}

#[test]
fn ancova_test() {
    let k = "5";
    let q = "10";
    let p = "2";
    let join = with_rest("ANCOVA");
    let extra = json!({"k": k, "q": q, "p": p, "analysis": "alpha"});
    test_interface(&join(&extra), 0.469); // G*Power gives 0.467
    let extra = json!({"k": k, "q": q, "p": p, "analysis": "power"});
    test_interface(&join(&extra), 0.553); // G*Power gives 0.555
    let extra = json!({"k": k, "q": q, "p": p, "analysis": "es"});
    test_interface(&join(&extra), 0.775); // G*Power gives 0.773
    let extra = json!({"k": k, "q": q, "p": p, "analysis": "n"});
    test_interface(&join(&extra), 107.0);

    let k = "10";
    let q = "50";
    let p = "10";
    let join = with_rest("ANCOVA");
    let extra = json!({"k": k, "q": q, "p": p, "analysis": "alpha"});
    test_interface(&join(&extra), 0.826); // G*Power gives 0.825
    let extra = json!({"k": k, "q": q, "p": p, "analysis": "power"});
    test_interface(&join(&extra), 0.156); // G*Power gives 0.158
    let extra = json!({"k": k, "q": q, "p": p, "analysis": "es"});
    test_interface(&join(&extra), 1.357); // G*Power gives 1.344
    let extra = json!({"k": k, "q": q, "p": p, "analysis": "n"});
    test_interface(&join(&extra), 204.0);
}

#[test]
fn between_repeated_anova_test() {
    let k = "2";
    let m = "4";
    let rho = "0.5";
    let join = with_rest("betweenRepeatedANOVA");
    let extra = json!({"k": k, "m": m, "rho": rho, "analysis": "alpha"});
    test_interface(&join(&extra), 0.008);
    let extra = json!({"k": k, "m": m, "rho": rho, "analysis": "power"});
    test_interface(&join(&extra), 0.992);
    let extra = json!({"k": k, "m": m, "rho": rho, "analysis": "es"});
    test_interface(&join(&extra), 0.411);
    let extra = json!({"k": k, "m": m, "rho": rho, "analysis": "n"});
    test_interface(&join(&extra), 35.0); // G*Power gives 36

    let k = "5";
    let m = "10";
    let rho = "0.75";
    let join = with_rest("betweenRepeatedANOVA");
    let extra = json!({"k": k, "m": m, "rho": rho, "analysis": "alpha"});
    test_interface(&join(&extra), 0.125);
    let extra = json!({"k": k, "m": m, "rho": rho, "analysis": "power"});
    test_interface(&join(&extra), 0.880);
    let extra = json!({"k": k, "m": m, "rho": rho, "analysis": "es"});
    test_interface(&join(&extra), 0.566);
    let extra = json!({"k": k, "m": m, "rho": rho, "analysis": "n"});
    test_interface(&join(&extra), 63.0); // G*Power gives 65
}

#[test]
fn within_repeated_anova_test() {
    let k = "4";
    let m = "2";
    let rho = "0.5";
    let epsilon = "1.0";
    let n = 12.0;
    let join =
        json!({"n": n, "alpha": ALPHA, "power": POWER, "es": ES, "test": "withinRepeatedANOVA"});
    let extra = json!({"k": k, "m": m, "rho": rho, "epsilon": epsilon, "analysis": "alpha"});
    test_interface(&join_json(&join, &extra), 0.123);
    let extra = json!({"k": k, "m": m, "rho": rho, "epsilon": epsilon, "analysis": "power"});
    test_interface(&join_json(&join, &extra), 0.857);
    let extra = json!({"k": k, "m": m, "rho": rho, "epsilon": epsilon, "analysis": "es"});
    test_interface(&join_json(&join, &extra), 0.597);
    let join = with_rest("withinRepeatedANOVA");
    let extra = json!({"k": k, "m": m, "rho": rho, "epsilon": epsilon, "analysis": "n"});
    test_interface(&join(&extra), 16.0);

    let k = "4";
    let m = "3";
    let rho = "0.75";
    let epsilon = "0.7";
    let n = 10.0;
    let join =
        json!({"n": n, "alpha": ALPHA, "power": POWER, "es": ES, "test": "withinRepeatedANOVA"});
    let extra = json!({"k": k, "m": m, "rho": rho, "epsilon": epsilon, "analysis": "alpha"});
    test_interface(&join_json(&join, &extra), 0.040);
    let extra = json!({"k": k, "m": m, "rho": rho, "epsilon": epsilon, "analysis": "power"});
    test_interface(&join_json(&join, &extra), 0.963);
    let extra = json!({"k": k, "m": m, "rho": rho, "epsilon": epsilon, "analysis": "es"});
    test_interface(&join_json(&join, &extra), 0.481);
    let join = with_rest("withinRepeatedANOVA");
    let extra = json!({"k": k, "m": m, "rho": rho, "epsilon": epsilon, "analysis": "n"});
    test_interface(&join(&extra), 10.0);
}

#[test]
#[should_panic(expected = "lower bound of ε corresponds to 1 / (number of measurements - 1)")]
fn within_repeated_anova_epsilon_error() {
    let k = "4";
    let m = "2";
    let rho = "0.5";
    let epsilon = "0.2"; // lower bound is 1/3.
    let n = 12.0;
    let join =
        json!({"n": n, "alpha": ALPHA, "power": POWER, "es": ES, "test": "withinRepeatedANOVA"});
    let extra = json!({"k": k, "m": m, "rho": rho, "epsilon": epsilon, "analysis": "alpha"});
    test_interface(&join_json(&join, &extra), 0.123);
}

#[test]
fn within_between_repeated_anova_test() {
    let k = "4";
    let m = "2";
    let rho = "0.5";
    let epsilon = "1.0";
    let n = 15.0;
    let join = json!({"n": n, "alpha": ALPHA, "power": POWER, "es": ES, "test": "withinBetweenRepeatedANOVA"});
    let extra = json!({"k": k, "m": m, "rho": rho, "epsilon": epsilon, "analysis": "alpha"});
    test_interface(&join_json(&join, &extra), 0.187);
    let extra = json!({"k": k, "m": m, "rho": rho, "epsilon": epsilon, "analysis": "power"});
    test_interface(&join_json(&join, &extra), 0.782);
    let extra = json!({"k": k, "m": m, "rho": rho, "epsilon": epsilon, "analysis": "es"});
    test_interface(&join_json(&join, &extra), 0.644);
    let join = with_rest("withinBetweenRepeatedANOVA");
    let extra = json!({"k": k, "m": m, "rho": rho, "epsilon": epsilon, "analysis": "n"});
    test_interface(&join(&extra), 22.0); // G*Power gives 24

    let k = "4";
    let m = "3";
    let rho = "0.75";
    let epsilon = "0.7";
    let n = 12.0;
    let join = json!({"n": n, "alpha": ALPHA, "power": POWER, "es": ES, "test": "withinBetweenRepeatedANOVA"});
    let extra = json!({"k": k, "m": m, "rho": rho, "epsilon": epsilon, "analysis": "alpha"});
    test_interface(&join_json(&join, &extra), 0.078);
    let extra = json!({"k": k, "m": m, "rho": rho, "epsilon": epsilon, "analysis": "power"});
    test_interface(&join_json(&join, &extra), 0.913);
    let extra = json!({"k": k, "m": m, "rho": rho, "epsilon": epsilon, "analysis": "es"});
    test_interface(&join_json(&join, &extra), 0.539);
    let join = with_rest("withinBetweenRepeatedANOVA");
    let extra = json!({"k": k, "m": m, "rho": rho, "epsilon": epsilon, "analysis": "n"});
    test_interface(&join(&extra), 14.0); // G*Power gives 16
}

#[test]
#[should_panic(expected = "lower bound of ε corresponds to 1 / (number of measurements - 1)")]
fn within_between_repeated_anova_epsilon_error() {
    let k = "4";
    let m = "2";
    let rho = "0.5";
    let epsilon = "0.2"; // lower bound is 1/3.
    let n = 12.0;
    let join = json!({"n": n, "alpha": ALPHA, "power": POWER, "es": ES, "test": "withinBetweenRepeatedANOVA"});
    let extra = json!({"k": k, "m": m, "rho": rho, "epsilon": epsilon, "analysis": "alpha"});
    test_interface(&join_json(&join, &extra), 0.123);
}
