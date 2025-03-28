(function () {
    const chore_forms = document.querySelectorAll('form.chore-form');
    for (const form of chore_forms) {
        form.addEventListener('submit', function (event) {
            // remove all children from the form that don't have the spinner class
            let nonSpinners = [];
            let spinners = [];
            for (const child of form.children) {
                if (!child.classList.contains('spinner')) {
                    nonSpinners.push(child);
                }
                else {
                    spinners.push(child);
                }
            }

            for (const child of nonSpinners) {
                form.removeChild(child);
            }
            for (const spinner of spinners) {
                spinner.classList.remove('hidden');
            }
        });
    }
})();
