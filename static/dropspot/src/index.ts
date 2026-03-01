import htmx from "htmx.org";
import init, {
  upload_js,
  download_js,
  create_link_js,
  parse_link_js,
} from "dropspot-core";

import "./index.css";
import "./upload-circle.css";
import "./utils.css";
import "./my-element";

console.debug(htmx);

init().then(() => {
  console.log("DropSpot initialised");
});

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

    const link = create_link_js(result.file.id, result.encryption);
    console.debug(link);
    const parsed = parse_link_js(link);
    console.debug(parsed);
    console.debug(result);
    const buffer = (await download_js(
      result.file.id,
      result.encryption,
    )) as Uint8Array<ArrayBuffer>;
    console.debug(buffer.length);

    // download(result.file.name, buffer);
  });
}
