// Because this file gets included on every slide we need to guard the
// function invocation here
setTimeout(() => {
    if (!window.added) {
        window.added = true;
        addButtons();
    }
}, 0);

// This function adds buttons to any Rust code-block in the slide-deck
function addButtons() {
    const result = '<div class="result"></div>';
    const buttons = `
        <button class="exec btn">run</button>
        <button class="open-in-playground btn">open</button>
    `.replace(/\n( )+/g, "").trim();
    
    for (let block of document.querySelectorAll('pre.src-rust code')) {
        block.insertAdjacentHTML('beforebegin', buttons);
        block.insertAdjacentHTML('afterend', result);
    }

    for (let button of document.querySelectorAll('.exec')) {
        button.onclick = async () => {
            const target = button.parentNode.querySelector('.result');
            const code = button.parentNode.querySelector('code').textContent;
            target.innerHTML = '<img src="imgs/rust.gif" style="border:none; box-shadow:none; margin: 0; background: none;">';

            const payload = { optimize: "0", version: "stable", code };
            console.log(code);
            console.log(payload);
            
            const res = await fetch('https://play.rust-lang.org/evaluate.json', {
                method: 'POST',
                body: JSON.stringify(payload),
                headers: {
                    "Content-Type": "application/json"
                },
            });

            const { result, error } = await res.json();
            target.innerHTML = result;
        };
    }
    
    for (let button of document.querySelectorAll('.open-in-playground')) {
        button.onclick = () => {
            const code = extendCode(button.parentNode.querySelector('code').textContent);
            const payload = encodeURIComponent(code);
            const url = `https://play.rust-lang.org/?version=stable&code=${payload}`;

            window.open(url, '_blank');
        };
    }
}

// Wrap code in a way that Rust playground will like it
function extendCode(code) {
    if (!code.match(/^fn \w+/m)) { // No functions, wrap all in main
        code = "fn main() {\n" + code + "\n}"
    } else if (!code.match(/^fn main/m)) { // some functions, no main, add an empty one
        code = code + "\n\nfn main() {}";
    }
    return code;
}
