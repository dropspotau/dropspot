import { upload_js, create_link_js, type UploadResult } from "dropspot-core";
import { getAuth } from "./auth";

const createDownloadUrl = (identifier: string): URL => {
  const url = new URL(window.location.href);
  url.searchParams.set("file", identifier);

  return url;
};

const addRecentUpload = (result: UploadResult): void => {
  const recentUploads = document.querySelector("#recent-uploads");

  if (!recentUploads) {
    return;
  }

  const link = create_link_js(result.file.id, result.encryption);
  const url = createDownloadUrl(link);

  const div = document.createElement("div");
  div.classList.add("recent-upload");
  div.innerHTML = `
      <h3 class="text-white no-margin">${result.file.name}</h3>
      <copy-button value="${url}" class="button-white"></copy-button>
  `;
  recentUploads.appendChild(div);
};

const upload = document.querySelector("#upload");
const fileInput = document.querySelector("#file-input");

if (upload instanceof HTMLElement && fileInput instanceof HTMLInputElement) {
  upload.addEventListener("click", () => {
    if (fileInput) {
      fileInput.click();
    }
  });

  fileInput.addEventListener("change", async () => {
    if (!fileInput.files) {
      return;
    }

    const [file] = Array.from(fileInput.files);

    if (!file) {
      return;
    }

    const fileContents = new Uint8Array(await file.arrayBuffer());
    const auth = getAuth();
    const result = await upload_js(file.name, fileContents, auth, "gcs");
    addRecentUpload(result);

    const event: FileUploadEvent = new CustomEvent("file-upload", {
      detail: { upload: result },
      bubbles: true,
    });
    fileInput.dispatchEvent(event);
  });
}

export type FileUploadEvent = CustomEvent<{
  upload: UploadResult;
}>;

export type FileDownloadEvent = CustomEvent<{
  file: File;
}>;

declare global {
  interface DocumentEventMap {
    "file-upload": FileUploadEvent;
    "file-download": FileDownloadEvent;
  }
}
