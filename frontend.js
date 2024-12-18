function removeAllSelectOptions(selector) {
    while (selector.options.length > 0) {
        selector.remove(0);
    }
}

function addSelectOption(selector, text, enabled, value) {
    let option = new Option(text, value);
    option.disabled = !enabled;
    selector.add(option, undefined);
}

function getElementById(id) {
    const elem = document.getElementById(id);
    if (elem == null) {
        throw new Error("Expected to find element with id: " + id);
    }
    return elem;
}

function readString(id) {
    const elem = getElementById(id);
    return elem.value;
}

// Update the "Statistical test" options based on the "Test family" setting.
function familyChanged() {
    const family = readString("family");
    const testSelector = document.getElementById("test");
    removeAllSelectOptions(testSelector);
    if (family == "exact") {
        addSelectOption(testSelector, "Correlation: Bivariate normal model", true, 1);
        addSelectOption(testSelector, "Linear multiple regression: Random model", false, 2);
        addSelectOption(testSelector, "Proportion: Difference from constant (binomial test, one sample case)", false, 3);
        addSelectOption(testSelector, "Proportions: Inequality, two dependent groups (McNemar)", false, 4);
        addSelectOption(testSelector, "Proportions: Inequality, two independent groups (Fisher's exact test)", false, 5);
        addSelectOption(testSelector, "Proportions: Inequality, two independent groups (unconditional)", false, 6);
        addSelectOption(testSelector, "Proportions: Inequality (offset), two independent groups (unconditional)", false, 7);
        addSelectOption(testSelector, "Proportions: Sign test (binomial test)", false, 8);
    } else if (family == "f") {
        addSelectOption(testSelector, "ANCOVA: Fixed effects, main effects, and interactions", true, 'ANCOVA');
        addSelectOption(testSelector, "ANOVA: Fixed effects, omnibus, one-way", true, 'oneWayANOVA');
        addSelectOption(testSelector, "ANOVA: Fixed effects, special, main effects, and interactions", true , 'twoWayANOVA');
        addSelectOption(testSelector, "ANOVA: Repeated measures, between factors",true, 'betweenRepeatedANOVA');
        addSelectOption(testSelector, "ANOVA: Repeated measures, within factors", true, 'withinRepeatedANOVA');
        addSelectOption(testSelector, "ANOVA: Repeated measures, within-between interaction", true, 'withinBetweenRepeatedANOVA');
        addSelectOption(testSelector, "Hotellings T²: One group mean vector", false, 7);
        addSelectOption(testSelector, "Hotellings T²: Two group mean vector", false, 8);
        addSelectOption(testSelector, "MANOVA: Global effects", false, 9);
        addSelectOption(testSelector, "MANOVA: Special effects and interactions", false, 10);
        addSelectOption(testSelector, "MANOVA: Repeated measures, between factors", false, 11);
        addSelectOption(testSelector, "MANOVA: Repeated measures, within factors", false, 12);
        addSelectOption(testSelector, "MANOVA: Repeated measures, within-between interaction", false, 13);
        addSelectOption(testSelector, "Linear multiple regression: Fixed model, R² deviation from zero", true, 'deviationFromZeroMultipleRegression');
        addSelectOption(testSelector, "Linear multiple regression: Fixed model, R² increase", true, 'increaseMultipleRegression');
        addSelectOption(testSelector, "Variance: Test of equality (two sample case)", false, 16);
        addSelectOption(testSelector, "Generic F test", false, 17);
    } else if (family == "t") {
        addSelectOption(testSelector, "Correlation: Point biseral model", false, 1);
        addSelectOption(testSelector, "Linear bivariate regression: One group, size of slope", false, 2);
        addSelectOption(testSelector, "Linear bivariate regression: Two groups, difference between intercepts", false, 3);
        addSelectOption(testSelector, "Linear bivariate regression: Two groups, difference between slopes", false, 4);
        addSelectOption(testSelector, "Linear multiple regression: Fixed model, single regression coefficient", false, 5);
        addSelectOption(testSelector, "Means: Difference between two dependent means (matched pairs)", false, 'dependentSamplesTTest');
        addSelectOption(testSelector, "Means: Difference between two independent means (two groups)", true, 'independentSamplesTTest');
        addSelectOption(testSelector, "Means: Difference from constant (one sample case)", true, 'oneSampleTTest');
        addSelectOption(testSelector, "Means: Wilcoxon signed-rank test (matched pairs)", false, 9);
        addSelectOption(testSelector, "Means: Wilcoxon signed-rank test (one sample case)", false, 10);
        addSelectOption(testSelector, "Means: Wilcoxon-Mann-Whitney test (two groups)", false, 11);
        addSelectOption(testSelector, "Generic t test", false, 12);
    } else if (family == "chi") {
        addSelectOption(testSelector, "Goodness-of-fit tests: Contingency tables", true, 1);
        addSelectOption(testSelector, "Variance: Difference from constant (one sample case)", false, 2);
        addSelectOption(testSelector, "Generic χ² test", false, 3);
    } else if (family == "z") {
        addSelectOption(testSelector, "Correlation: Tetrachoric model", false, 1);
        addSelectOption(testSelector, "Correlations: Two dependent Pearson r's (common index)", true, 2);
        addSelectOption(testSelector, "Correlations: Two dependent Pearson r's (no common index)", true, 3);
        addSelectOption(testSelector, "Correlations: Two independent Pearson r's", true, 4);
        addSelectOption(testSelector, "Logistic regression", true, 5);
        addSelectOption(testSelector, "Poisson regression", false, 6);
        addSelectOption(testSelector, "Proportions: Difference between two independent proportions", false, 7);
        addSelectOption(testSelector, "Generic z test", false, 8);
    }
    updateNumberOutputAreas();
    return;
}

function removeAllTableRows(table) {
    while (table.rows.length > 0) {
        table.deleteRow(0);
    }
}

/** Add an option to a table. */
function addTableOption(table, description, element) {
    var row = table.insertRow(table.rows.length);
    var left = row.insertCell(0);
    // Using innerHTML over textContent to allow formatting such as italic.
    left.innerHTML = description.concat(":");
    var right = row.insertCell(1);
    right.innerHTML = element;
    return null;
}

/** Return an input element for floats with element `id`. */
function floatInputElement(id, defaultValue, step) {
    return `<input id="${id}" type="number" value="${defaultValue}" onchange="updateOutput()" min="0" max="999999" step="${step}">`;
}

function floatOutputElement(id, value) {
    return `<span id="${id}">${value}</span>`;
}

function disableOutputElement(id) {
    const elem = getElementById(id);
    elem.disabled = true;
    return null;
}

function enableOutputElement(id) {
    const elem = getElementById(id);
    elem.disabled = false;
    return null;
}

function getInputTable() {
    return document.getElementById("input");
}

/** Update the input and output area based on the "Type of power analysis" setting. */
function updateNumberOutputAreas() {
    const inputTable = getInputTable();
    removeAllTableRows(inputTable);
    const family = readString("family");
    const test = readString("test");
    if (family == "exact") {
    } else if (family == "f") {
        if (test == "deviationFromZeroMultipleRegression") {
            addTableOption(inputTable, "Number of predictors", "<input onchange='updateOutput()' id='nPredictors' value='2' min='0' max='1000' step='5'>");
        } else if (test == "increaseMultipleRegression") {
            addTableOption(inputTable, "Number of tested predictors", "<input onchange='updateOutput()' id='q' value='2' min='0' max='1000' step='1'>");
            addTableOption(inputTable, "Total number of predictors", "<input onchange='updateOutput()' id='p' value='5' min='0' max='1000' step='1'>");
        } else if (test == "oneWayANOVA") {
            addTableOption(inputTable, "Number of groups", "<input onchange='updateOutput()' id='k' value='5' min='1' max='1000' step='1'>");
        } else if (test == "twoWayANOVA") {
            addTableOption(inputTable, "Numerator df", "<input onchange='updateOutput()' id='q' value='10' min='1' max='1000' step='1'>");
            addTableOption(inputTable, "Number of groups", "<input onchange='updateOutput()' id='k' value='5' min='1' max='1000' step='1'>");
        } else if (test == "ANCOVA") {
            addTableOption(inputTable, "Numerator df", "<input onchange='updateOutput()' id='q' value='10' min='1' max='1000' step='1'>");
            addTableOption(inputTable, "Number of groups", "<input onchange='updateOutput()' id='k' value='5' min='1' max='1000' step='1'>");
            addTableOption(inputTable, "Number of covariates", "<input onchange='updateOutput()' id='p' value='2' min='2' max='1000' step='1'>");
        } else if (test == "betweenRepeatedANOVA") {
            addTableOption(inputTable, "Number of groups", "<input onchange='updateOutput()' id='k' value='2' min='0' max='1000' step='1'>");
            addTableOption(inputTable, "Number of measurement", "<input onchange='updateOutput()' id='m' value='2' min='2' max='1000' step='1'>");
            addTableOption(inputTable, "Corr among rep measures", "<input onchange='updateOutput()' id='rho' value='0.5' min='0' max='1' step='0.1'>");
        } else if (test == "withinRepeatedANOVA" || test == "withinBetweenRepeatedANOVA") {
            addTableOption(inputTable, "Number of groups", "<input onchange='updateOutput()' id='k' value='2' min='0' max='1000' step='1'>");
            addTableOption(inputTable, "Number of measurement", "<input onchange='updateOutput()' id='m' value='2' min='2' max='1000' step='1'>");
            addTableOption(inputTable, "Corr among rep measures", "<input onchange='updateOutput()' id='rho' value='0.5' min='0' max='1' step='0.1'>");
            // FIXME: the lower bound of epsilon corresponds to 1 / (number of measurements - 1)
            addTableOption(inputTable, "Nonsphericity correction ε", "<input onchange='updateOutput()' id='epsilon' value='1' min='0' max='1' step='0.1'>");
        }
    } else if (family == "t") {
        addTableOption(inputTable, "Tail(s)", "<select onchange='updateOutput()' id='tail'><option value=1>One tail</option><option value=2>Two tails</option></select>");
    } else if (family == "chi") {
        addTableOption(inputTable, "Df", "<input onchange='updateOutput()' id='df' value='5' min='1' max=1000' step='1'>");
    } else if (family == "z") {
    }

    enableOutputElement("n");
    enableOutputElement("alpha");
    enableOutputElement("power");
    enableOutputElement("es");

    const analysis = readString("analysis");
    if (analysis == "n") {
        disableOutputElement("n");
    } else if (analysis == "alpha") {
        disableOutputElement("alpha");
    } else if (analysis == "power") {
        disableOutputElement("power");
    } else if (analysis == "es") {
        disableOutputElement("es");
    }

    updateOutput();

    return null;
}

function readFloat(id) {
    const elem = getElementById(id);
    return parseFloat(elem.value);
}

const highlightBorder = [
    { border: '1px var(--favicon-red) solid' }
];

const highlightTiming = {
    duration: 400,
    iterations: 1,
}

function setFloat(id, value) {
    const elem = getElementById(id);
    elem.animate(highlightBorder, highlightTiming);
    elem.value = value;
    return null;
}

function readInt(id) {
    const elem = getElementById(id);
    return parseInt(elem.value);
}

function tail() {
    return readInt("tail");
}

function alpha() {
    return readFloat("alpha");
}

function power() {
    return readFloat("power");
}

function es() {
    return readFloat("es");
}

function n() {
    return readFloat("n");
}

function restrictFloat(id) {
    const elem = getElementById(id);
    const value = elem.value;
    if (elem.max < elem.value) {
        elem.value = elem.max;
    }
}

/** Enforce that input numbers are within the HTML specified values. */
function restrictInput() {
    restrictFloat("alpha");
}

function setError(text) {
    const elem = getElementById("error");
    elem.innerText = text;
    return null;
}

function handleError(value) {
    if (value == -111 || value == -111.0) {
        setError("Unable to find a solution for given input.");
    }
    return null;
}

function setOutput(id, out) {
    handleError(out);
    setFloat(id, out);
    return null;
}

function frontEndState() {
    const inputTable = getInputTable();
    const inputElements = inputTable.querySelectorAll("input, select");

    // Ignoring family because the back end will infer it from the test.
    const analysis = readString("analysis");
    const test = readString("test");

    const state = {
        analysis: analysis,
        test: test,
        n: n(),
        alpha: alpha(),
        power: power(),
        es: es()
    };

    for (let i = 0; i < inputElements.length; i++) {
        const elem = inputElements[i];
        state[elem.id] = elem.value;
    }

    return state;
}

function writeToPtr(ptr, text) {
    const buffer = Module.HEAPU8.buffer;
    const view = new Uint8Array(buffer, ptr, 1024);
    const encoder = new TextEncoder();
    const with_stop = text + "<END>";
    view.set(encoder.encode(with_stop));
}

function readFromPtr(ptr) {
    const buffer = Module.HEAPU8.buffer;
    const view = new Uint8Array(buffer, ptr, 1024);
    const length = view.findIndex(byte => byte === 0);
    const decoder = new TextDecoder();

    return decoder.decode(new Uint8Array(buffer, ptr, length));
}

/** Update the output area by calculating the numbers via WebAssembly. */
function updateOutput() {
    setError("");
    restrictInput();

    const family = readString("family");
    const analysis = readString("analysis");
    const test = readString("test");

    const state = frontEndState();
    const json = JSON.stringify(state);
    console.log(`Sending the following json to the back end: ${json}`);

    const ptr = Module._alloc();
    writeToPtr(ptr, json);
    Module._calculatePower(ptr);
    const returned = readFromPtr(ptr);
    Module._dealloc(ptr);
    console.log(`Received the following json from the back end: ${returned}`);
    const result = JSON.parse(returned);
    const id = Object.keys(result)[0];
    setOutput(id, result[id]);

    return null;
}

/** Reset the numbers in the output area. */
function resetOutput() {
    setFloat("n", 50);
    setFloat("alpha", 0.05);
    setFloat("power", 0.95);
    setFloat("es", 0.5);
    updateOutput();
}

function webAssemblySupport() {
    try {
        if (typeof WebAssembly === "object" && typeof WebAssembly.instantiate === "function") {
            const module = new WebAssembly.Module(Uint8Array.of(0x0, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00));
            if (module instanceof WebAssembly.Module) {
                return new WebAssembly.Instance(module) instanceof WebAssembly.Instance;
            }
        }
    } catch (e) {
    }
    return false;
}

if (!webAssemblySupport()) {
    document.body.innerHTML = `
        <br>
        <center>
        This site only works with WebAssembly. Enable WebAssembly in your browser to continue.
        </center>
        `;
} else {
    Module['onRuntimeInitialized'] = function() {
        console.log("Loading of the poweranalyses.wasm library succeeded.");
        familyChanged();
        updateNumberOutputAreas();
        updateOutput();
    }
}
