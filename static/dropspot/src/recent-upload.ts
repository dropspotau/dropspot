import { get_integrations_js } from "dropspot-core";
import { getAuth } from "./auth";
import { UploadBarElement } from "./components";

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

    const auth = getAuth();

    if (!auth) {
      return;
    }

    const integrations = await get_integrations_js(auth);
    UploadBarElement.create(file, integrations);
  });
}
