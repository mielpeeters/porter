const { invoke } = window.__TAURI__.tauri;
const { open, save, ask } = window.__TAURI__.dialog;
const { homeDir } = window.__TAURI__.path;
const { open: openFile } = window.__TAURI__.shell;

let responseEl;
let input;
let declare;
let output;
let home = "";

async function select_input() {
    if (home === "") {
        home = await homeDir()
    }

    input = await open({
        multiple: false,
        filters: [{
            name: 'Html',
            extensions: ['html']
        }],
        title: "Select Template File",
        defaultPath: home
    });
    responseEl.classList.remove("error");
    responseEl.textContent = "Template selected";
}

async function select_decl() {
    if (home === "") {
        home = await homeDir()
    }
    
    declare = await open({
        multiple: false,
        filters: [{
            name: 'TOML',
            extensions: ['toml']
        }],
        defaultPath: home 
    });
    responseEl.classList.remove("error");
    responseEl.textContent = "Declaration selected";
}

async function select_output() {
    if (home === "") {
        home = await homeDir()
    }

    output = await save({
        filters: [{
            name: 'Html',
            extensions: ['html']
        }],
        defaultPath: home
    });
    responseEl.classList.remove("error");
    responseEl.textContent = "Output selected";
}

async function create_site() {
    let cmd_output = "";
    try {
        cmd_output = await invoke("create_site", { 
            inputFile: input,
            declarationFile: declare,
            outputFile: output
        });
        responseEl.textContent = cmd_output;
        responseEl.classList.remove("error");
    } catch (e) {
        responseEl.classList.add("error");
        if (e.includes("inputFile")) {
            responseEl.textContent = "Select a valid template file."
        } else if (e.includes("declaration")) {
            responseEl.textContent = "Select a valid declaration file."
        } else if (e.includes("output")) {
            // responseEl.textContent = "Select a valid output file."
            responseEl.textContent = e;
        } else {
            responseEl.textContent = e;    
        }
        return;
    }

    if (cmd_output.includes("Error")) {
        responseEl.classList.add("error");
        return;
    }

    if (await ask("Open the resulting file?", {title: "Result", cancelLabell: "no, thanks", okLabel: "yes, please!"})) {
        await openFile(output);
    }
}

window.addEventListener("DOMContentLoaded", () => {
    responseEl = document.querySelector(".response");

    document.querySelector("#create-form").addEventListener("submit", (e) => {
        e.preventDefault();
        create_site();
    });
    document.querySelector("#input-form").addEventListener("submit", (e) => {
        e.preventDefault();
        select_input();
    });
    document.querySelector("#decl-form").addEventListener("submit", (e) => {
        e.preventDefault();
        select_decl();
    });
    document.querySelector("#output-form").addEventListener("submit", (e) => {
        e.preventDefault();
        select_output();
    });
});
