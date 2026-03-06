import htmx from "htmx.org";
import "@material/web/button/filled-button.js";
import "@material/web/button/outlined-button.js";
import "@material/web/icon/icon.js";
import "@material/web/iconbutton/icon-button.js";
import "@material/web/progress/circular-progress.js";
import init from "dropspot-core";

import "./index.css";
import "./theme.css";
import "./upload-circle.css";
import "./uploads.css";
import "./utils.css";

import "./copy-button";
import "./file-preview";
import "./modal";
import "./my-element";
import "./recent-upload";

console.debug(htmx);

init().then(() => {
  console.log("DropSpot initialised");
});
