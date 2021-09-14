"use strict";
// Fix back button cache problem
window.onunload = function () {};

// Global variable, shared between modules
function playpen_text(playpen) {
    if (!window.ace) {
        return "";
    }
    let code_block = playpen.querySelector("div.playpen > div.editable");
    let editor = window.ace.edit(code_block);
    return editor.getValue();
}

// Set ACE editor configuration options here
function configureEditor(editor) {
    editor.setOptions({
        highlightActiveLine: false,
        showPrintMargin: false,
        enableBasicAutocompletion: true,
        showLineNumbers: true,
        showGutter: true,
        maxLines: 40,
        minLines: 8,
        // fontSize: "14pt" // please adjust the font size of the code in general.css
    });

    editor.$blockScrolling = Infinity;

    editor.getSession().setMode("ace/mode/rust");
    editor.setTheme("ace/theme/dreamweaver");
}

function codeSnippets() {
    const PLAYPEN_URL = "https://rustpen.tari.com";

    function fetch_with_timeout(url, options, timeout = 6000) {
        return Promise.race([
            fetch(url, options),
            new Promise((_, reject) => setTimeout(() => reject(new Error('timeout')), timeout))
        ]).then(res => {
            if (res.status >= 200 && res.status < 300) {
                return res;
            }
            return Promise.reject(new Error(`${res.status}: ${res.statusText}`));
        });
    }

    function run_rust_code(code_block) {
        let result_block = code_block.querySelector(".result");
        if (!result_block) {
            result_block = document.createElement('code');
            result_block.className = 'result language-bash';

            code_block.append(result_block);
        }

        let text = playpen_text(code_block);

        const params = {
            channel: "nightly",
            mode: "debug",
            crateType: "bin",
            tests: false,
            code: text,
            backtrace: false,
            edition: "2018"
        };

        result_block.innerText = "Running...";

        fetch_with_timeout(PLAYPEN_URL + '/execute', {
            headers: {
                'Content-Type': "application/json",
            },
            method: 'POST',
            mode: 'cors',
            body: JSON.stringify(params)
        }, 60000)
            .then(response => response.json())
            .then(response => {
                if (response.success) {
                    result_block.innerText = response.stdout;
                } else {
                    result_block.innerText = response.stderr;
                }
            })
            .catch(error => result_block.innerText = error.message);
    }

    function insertPlaypenButtons(block) {
        // Add play button
        let buttons = block.querySelector(".buttons");
        if (!buttons) {
            buttons = document.createElement('div');
            buttons.className = 'buttons';
            block.insertBefore(buttons, block.firstChild);
        }

        const runCodeButton = document.createElement('button');
        runCodeButton.className = 'fa fa-play play-button';
        runCodeButton.hidden = false;
        runCodeButton.title = 'Run this code';
        runCodeButton.setAttribute('aria-label', runCodeButton.title);

        const copyCodeClipboardButton = document.createElement('button');
        copyCodeClipboardButton.className = 'fa fa-copy clip-button';
        copyCodeClipboardButton.innerHTML = '<i class="tooltiptext"></i>';
        copyCodeClipboardButton.title = 'Copy to clipboard';
        copyCodeClipboardButton.setAttribute('aria-label', copyCodeClipboardButton.title);

        buttons.insertBefore(runCodeButton, buttons.firstChild);
        buttons.insertBefore(copyCodeClipboardButton, buttons.firstChild);

        runCodeButton.addEventListener('click', function (e) {
            run_rust_code(block);
        });

        let code_block = block.querySelector("div.playpen > div.editable");
        if (window.ace && code_block.classList.contains("editable")) {
            const undoChangesButton = document.createElement('button');
            undoChangesButton.className = 'fa fa-history reset-button';
            undoChangesButton.title = 'Undo changes';
            undoChangesButton.setAttribute('aria-label', undoChangesButton.title);

            buttons.insertBefore(undoChangesButton, buttons.firstChild);

            undoChangesButton.addEventListener('click', function () {
                let editor = window.ace.edit(code_block);
                editor.setValue(editor.originalCode);
                editor.clearSelection();
            });
        }
    }

    Array.from(document.querySelectorAll(".playpen")).forEach(insertPlaypenButtons);
    console.log("Playpen snippets processed.");
}

function clipboard() {
    const clipButtons = document.querySelectorAll('.clip-button');

    function hideTooltip(elem) {
        elem.firstChild.innerText = "";
        elem.className = 'fa fa-copy clip-button';
    }

    function showTooltip(elem, msg) {
        elem.firstChild.innerText = msg;
        elem.className = 'fa fa-copy tooltipped';
    }

    const clipboardSnippets = new Clipboard('.clip-button', {
        text: function (trigger) {
            hideTooltip(trigger);
            let playpen = trigger.closest(".playpen");
            return playpen_text(playpen);
        }
    });

    Array.from(clipButtons).forEach(function (clipButton) {
        clipButton.addEventListener('mouseout', function (e) {
            hideTooltip(e.currentTarget);
        });
    });

    clipboardSnippets.on('success', function (e) {
        e.clearSelection();
        showTooltip(e.trigger, "Copied!");
    });

    clipboardSnippets.on('error', function (e) {
        showTooltip(e.trigger, "Clipboard error!");
    });
}

function setupEditors() {
    window.editors = [];
    if (typeof (ace) === 'undefined' || !ace) {
        console.error("Cannot set up rustpen - Ace is not loaded");
        return;
    }

    Array.from(document.querySelectorAll('.editable')).forEach(function (editable) {
        let editor = ace.edit(editable);
        editor.originalCode = editor.getValue();
        configureEditor(editor);
        window.editors.push(editor);
    });
}
