import init, { convert_oud2_to_kdl } from "./pkg/oudia_to_kdl.js";

const wasmReady = init().catch((err) => {
    console.error("Failed to load WASM module", err);
    setStatus("Failed to load WebAssembly module.", true);
    throw err;
});

const inputField = document.getElementById("input");
const outputField = document.getElementById("output");
const statusEl = document.getElementById("status");
const convertButton = document.getElementById("convert");
const copyButton = document.getElementById("copy");
const downloadButton = document.getElementById("download");
const openFileButton = document.getElementById("open-file");

const hiddenFileInput = createHiddenFileInput();

function createHiddenFileInput() {
    const input = document.createElement("input");
    input.type = "file";
    input.accept = ".oud2,.oud,.txt,.text";
    input.style.display = "none";
    document.body.appendChild(input);
    return input;
}

function setStatus(message, isError = false) {
    if (!statusEl) {
        return;
    }
    statusEl.textContent = message;
    if (isError) {
        statusEl.classList.add("error");
    } else {
        statusEl.classList.remove("error");
    }
}

function ensureReady() {
    return wasmReady;
}

convertButton?.addEventListener("click", async () => {
    setStatus("Converting...");
    await ensureReady();
    try {
        const input = inputField?.value ?? "";
        const result = convert_oud2_to_kdl(input);
        if (outputField) {
            outputField.value = result;
            outputField.scrollTop = 0;
        }
        setStatus("Conversion complete.");
    } catch (err) {
        console.error("Conversion failed", err);
        setStatus(formatError(err), true);
    }
});

copyButton?.addEventListener("click", async () => {
    if (!outputField) {
        return;
    }
    const text = outputField.value;
    if (!text) {
        setStatus("No output to copy.", true);
        return;
    }
    try {
        await navigator.clipboard.writeText(text);
        setStatus("Copied to clipboard.");
    } catch (err) {
        console.error("Copy failed", err);
        setStatus("Unable to copy to clipboard.", true);
    }
});

downloadButton?.addEventListener("click", () => {
    if (!outputField) {
        return;
    }
    const text = outputField.value;
    if (!text) {
        setStatus("No output to download.", true);
        return;
    }
    const blob = new Blob([text], { type: "text/plain" });
    const url = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.href = url;
    link.download = "output.kdl";
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);
    setStatus("Download ready.");
});

openFileButton?.addEventListener("click", () => {
    hiddenFileInput.click();
});

hiddenFileInput.addEventListener("change", () => {
    const file = hiddenFileInput.files?.[0];
    if (!file) {
        return;
    }
    setStatus(`Loading ${file.name}...`);
    const reader = new FileReader();
    reader.onload = () => {
        const contents = reader.result;
        if (typeof contents === "string") {
            if (inputField) {
                inputField.value = contents;
                inputField.focus();
                inputField.selectionStart = 0;
                inputField.selectionEnd = 0;
            }
            setStatus(`Loaded ${file.name}.`);
        } else {
            setStatus("Could not read file contents.", true);
        }
        hiddenFileInput.value = "";
    };
    reader.onerror = () => {
        console.error("Failed to read file", reader.error);
        setStatus("Failed to read file.", true);
        hiddenFileInput.value = "";
    };
    reader.readAsText(file);
});

function formatError(err) {
    if (!err) {
        return "Unknown error.";
    }
    if (typeof err === "string") {
        return err;
    }
    if (err instanceof Error && err.message) {
        return err.message;
    }
    try {
        return JSON.stringify(err);
    } catch (_) {
        return "Unexpected error.";
    }
}
