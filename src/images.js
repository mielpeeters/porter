const { invoke } = window.__TAURI__.tauri;
const { open } = window.__TAURI__.dialog;
const { homeDir } = window.__TAURI__.path;
const { listen } = window.__TAURI__.event;

let responseEl;
let input;
let output;
let home = "";

async function select_input() {
    if (home === "") {
        home = await homeDir()
    }

    input = await open({
        directory: true,
        multiple: false,
        title: "Select Images Directory",
        defaultPath: home,
        recursive: true
    });
    responseEl.classList.remove("error");
    responseEl.textContent = "Images folder selected";
}

async function select_output() {
    if (home === "") {
        home = await homeDir()
    }

    output = await open({
        directory: true,
        multiple: false,
        title: "Select output directory",
        defaultPath: home
    });
    responseEl.classList.remove("error");
    responseEl.textContent = "Output selected";
}

async function convert() {
    responseEl.classList.remove("error");
    responseEl.textContent = "0 images have been produced.";

    let cmd_output = "";
    try {
        cmd_output = await invoke("convert_images", { 
            inputDir: input,
            outputDir: output,
        });
        responseEl.textContent = cmd_output;
        responseEl.classList.remove("error");
    } catch (e) {
        responseEl.classList.add("error");
        responseEl.textContent = "Errooor"
    }

}


window.addEventListener("DOMContentLoaded", () => {
    responseEl = document.querySelector(".response");

    document.querySelector("#input-form").addEventListener("submit", (e) => {
        e.preventDefault();
        select_input();
    });
    document.querySelector("#output-form").addEventListener("submit", (e) => {
        e.preventDefault();
        select_output();
    });
    document.querySelector("#create-form").addEventListener("submit", (e) => {
        e.preventDefault();
        convert();
    });
});

const unlisten = await listen("work", (event) => {
    let payload = event.payload;
    let done = payload[0];
    let todo = payload[1];

    responseEl.classList.remove("error");
    responseEl.textContent = done + " out of " + todo + " images have been produced.";
})

const unlisten2 = await listen("skip", (event) => {
    let img = event.payload;

    responseEl.classList.remove("error");
    responseEl.textContent = "image " + img + " was skipped (already exists).";
})

