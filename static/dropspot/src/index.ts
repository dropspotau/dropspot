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
import "./popover";
import "./recent-upload";

console.debug(htmx);

init().then(() => {
  console.log("DropSpot initialised");
});

const settingsDialogButton = document.querySelector("#settings-popover-toggle");
const filesDialogButton = document.querySelector("#files-popover-toggle");

if (settingsDialogButton) {
  settingsDialogButton.addEventListener("click", () => {
    document.dispatchEvent(
      new CustomEvent("popover-toggle", {
        detail: {
          selector: "#settings-popover",
          srcElement: settingsDialogButton,
        },
      }),
    );
  });
}

if (filesDialogButton) {
  filesDialogButton.addEventListener("click", () => {
    document.dispatchEvent(
      new CustomEvent("popover-toggle", {
        detail: { selector: "#files-popover", srcElement: filesDialogButton },
      }),
    );
  });
}
