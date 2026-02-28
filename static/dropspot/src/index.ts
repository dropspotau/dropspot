import htmx from "htmx.org";
import init, { upload_js, download_js } from "dropspot-core";

import "./index.css";
import "./upload-circle.css";
import "./utils.css";
import "./my-element";

console.debug(htmx);

init().then(() => {
  console.log("DropSpot initialised");
});

const upload = document.querySelector("#upload");

if (upload) {
  upload.addEventListener("click", async () => {
    const result = await upload_js("test.txt", new Uint8Array([1, 2, 3, 4, 5]));
    console.debug(result);

    const buffer = await download_js(result.file.id, result.encryption);
    console.debug(buffer);
  });
}
