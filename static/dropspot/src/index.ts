import htmx from "htmx.org";
import init, { upload_js } from "dropspot-core";
import "./index.css";
import "./upload-circle.css";
import "./utils.css";
import "./my-element";

console.debug(htmx);

await init(); // How good are top-level awaits?

const upload = document.querySelector("#upload");

if (upload) {
  upload.addEventListener("click", async () => {
    console.log("Clicked");
    const result = await upload_js("test.txt", new Uint8Array([1, 2, 3, 4, 5]));
    console.debug(result);
  });
}
