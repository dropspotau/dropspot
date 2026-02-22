import htmx from "htmx.org";
import "./index.css";
import "./my-element";
import { upload_js } from "dropspot-core";

console.debug("Hello", htmx);

const button = document.querySelector("button");

if (button) {
  button.addEventListener("click", async () => {
    console.log("Clicked");
    const result = await upload_js("file.txt", new Uint8Array([1, 2, 3, 4, 5]));
    console.debug(result);
  });
}
