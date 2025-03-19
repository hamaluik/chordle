(function () {
    const abort_controller = new AbortController();
    const abort_signal = abort_controller.signal;
    async function can_parse_span(span) {
        abort_controller.abort();
        return fetch(
            "/api/parse_span?span=" + encodeURIComponent(span), {
            signal: abort_signal
        })
            .then((res) => {
                return res.ok;
            })
            .catch((_) => {
                return false;
            });
    }

    async function check_validity(el) {
        let okay = true;
        if (el.classList.contains('name-field')) {
            if (el.validity.valueMissing) {
                // el.setCustomValidity("Chore names cannot be blank.");
                okay = false;
            }
            else if (el.value.trim().length === 0) {
                // el.setCustomValidity("Chore names cannot be just whitespace.");
                okay = false;
            }
            else if (el.value.trim().length > 160) {
                // el.setCustomValidity("Chore name is too long, max 160 characters.");
                okay = false;
            }
        }
        else if (el.classList.contains('interval-field')) {
            if (el.validity.valueMissing) {
                // el.setCustomValidity("Intervals cannot be blank.");
                okay = false;
            }
            else if (el.value.trim().length < 2) {
                // el.setCustomValidity("Intervals must have an amount and unit.");
                okay = false;
            }
            else if (el.value.trim().length > 160) {
                // el.setCustomValidity("Intervals cannot be longer than 160 characters.");
                okay = false;
            }
            else if (!(await can_parse_span(el.value))) {
                el.setCustomValidity("Invalid span, try something like '1w'");
                okay = false;
            }
        }
        if (okay) {
            el.setCustomValidity("");
        }
        el.reportValidity();
        return okay;
    }

    let inputs = document.querySelectorAll('input[type=text]');
    for (let i = 0; i < inputs.length; i++) {
        inputs[i].addEventListener('input', function () {
            this.classList.toggle('is-invalid', !check_validity(this));
        });
        inputs[i].addEventListener('change', function () {
            this.classList.toggle('is-invalid', !check_validity(this));
        });
    }
})();
