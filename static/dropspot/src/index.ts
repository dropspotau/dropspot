import htmx from "htmx.org";
import init, {
  upload_js,
  download_js,
  create_link_js,
  type UploadResult,
} from "dropspot-core";

import "./index.css";
import "./upload-circle.css";
import "./uploads.css";
import "./utils.css";

import "./copy-button";
import "./my-element";
import "@material/web/button/filled-button.js";

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
  console.debug(recentUploads);

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

    const link = create_link_js(result.file.id, result.encryption);
    console.debug(link);
    const buffer = (await download_js(
      result.file.id,
      result.encryption,
    )) as Uint8Array<ArrayBuffer>;
    console.debug(buffer.length);

    // download(result.file.name, buffer);
  });
}
