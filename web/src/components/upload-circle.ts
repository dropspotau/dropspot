import { UploadBarElement } from "./upload-bar";

import "./upload-circle.css";

const upload = document.querySelector("#upload");
const fileInput = document.querySelector("#file-input");

/** Triggers a file to be uploaded */
const triggerUpload = (file: File): void => {
  UploadBarElement.create(file);
};

if (upload instanceof HTMLElement && fileInput instanceof HTMLInputElement) {
  upload.addEventListener("click", () => {
    if (fileInput) {
      // Open the file dialog
      fileInput.click();
    }
  });

  upload.addEventListener("dragover", (e) => {
    // Allows the drop event to fire
    e.preventDefault();
  });

  upload.addEventListener("drop", (e) => {
    const { dataTransfer } = e;

    if (!dataTransfer) {
      return;
    }

    e.preventDefault();
    e.stopPropagation();

    const files = [...dataTransfer.files];

    for (const file of files) {
      triggerUpload(file);
    }
  });

  fileInput.addEventListener("change", () => {
    if (!fileInput.files) {
      return;
    }

    const files = [...fileInput.files];

    for (const file of files) {
      triggerUpload(file);
    }

    if (fileInput) {
      fileInput.value = ""; // Clear files
    }
  });
}
