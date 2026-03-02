import htmx from "htmx.org";
import "@material/web/button/filled-button.js";
import init, {
  upload_js,
  download_js,
  create_link_js,
  parse_link_js,
  get_file_js,
  type UploadResult,
  type Link,
} from "dropspot-core";

import "./index.css";
import "./theme.css";
import "./upload-circle.css";
import "./uploads.css";
import "./utils.css";

import "./copy-button";
import "./file-preview";
import "./my-element";
import { download } from "./download";

console.debug(htmx);

init().then(() => {
  console.log("DropSpot initialised");
});

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
      <copy-button value="${url}"></copy-button>
  `;
  recentUploads.appendChild(div);
};

const tryDetectIdentifier = (): Link | null => {
  const url = new URL(window.location.href);
  const file = url.searchParams.get("file");

  if (file) {
    return parse_link_js(file);
  }

  return null;
};

const initialiseDownload = async (): Promise<void> => {
  const identifier = tryDetectIdentifier();
  const linkedFileElement = document.querySelector("#linked-file");

  if (!identifier || !linkedFileElement) {
    return;
  }

  const { file_id: fileId, encryption } = identifier;
  const file = await get_file_js(fileId);

  linkedFileElement.innerHTML = `
      <span class="text-white">You've been sent</span>
      <h3 class="text-white no-margin">${file.name}</h3>
      <md-filled-button class="button-white">Download</md-filled-button>
  `;

  const button = linkedFileElement.querySelector("md-filled-button");

  if (button) {
    button.addEventListener("click", async () => {
      const buffer = (await download_js(
        file.id,
        encryption,
      )) as Uint8Array<ArrayBuffer>;

      if (false) {
        download(file.name, buffer);
      }

      const filePreviewElement = document.createElement("file-preview");
      filePreviewElement.setAttribute("name", file.name);
      filePreviewElement.setBuffer(buffer);
      linkedFileElement.appendChild(filePreviewElement);
    });
  }
};

setTimeout(() => {
  initialiseDownload();
}, 500);

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

    const result = await upload_js(file.name, fileContents);
    addRecentUpload(result);
  });
}
