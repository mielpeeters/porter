const { invoke } = window.__TAURI__.tauri;
const { open, save } = window.__TAURI__.dialog;
// import { open } from '@tauri-apps/api/dialog';

let responseEl;
let input;
let declare;
let output;

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//   greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
// }

async function select_input() {
    input = await open({
        multiple: false,
        filters: [{
            name: 'Html',
            extensions: ['html']
        }]
    });
    responseEl.textContent = "Template selected";
}

async function select_decl() {
    declare = await open({
        multiple: false,
        filters: [{
            name: 'TOML',
            extensions: ['toml']
        }]
    });
    responseEl.textContent = "Declaration selected";
}

async function select_output() {
    output = await save({
        filters: [{
            name: 'Html',
            extensions: ['html']
        }]
    });
    responseEl.textContent = "Output selected";
}

async function create_site() {
    try {
        let cmd_output = await invoke("create_site", { 
            inputFile: input,
            declarationFile: declare,
            outputFile: output
        });
        responseEl.textContent = cmd_output;
    } catch (e) {
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
        
    }

}

window.addEventListener("DOMContentLoaded", () => {
    responseEl = document.querySelector("#response");

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
