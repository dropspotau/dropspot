import htmx from "htmx.org";
import "./index.css";
import "./my-element";
import { upload_js } from "dropspot-core";

console.debug("Hello", htmx);

upload_js("file.txt", new Uint8Array([1, 2, 3, 4, 5]));
