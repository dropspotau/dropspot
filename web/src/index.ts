import "@material/web/button/filled-button.js";
import "@material/web/button/outlined-button.js";
import "@material/web/icon/icon.js";
import "@material/web/iconbutton/icon-button.js";
import "@material/web/progress/circular-progress.js";
import "@material/web/menu/menu.js";
import "@material/web/menu/menu-item.js";
import "@material/web/textfield/filled-text-field.js";
import "@material/web/switch/switch.js";
import init from "dropspot-core";

import "./header.css";
import "./index.css";
import "./file.css";
import "./form.css";
import "./menu.css";
import "./settings.css";
import "./theme.css";
import "./uploads.css";
import "./utils.css";

import { loginAtStartup } from "./auth";

// Import all components
export * from "./components";

init().then(() => {
  console.log("DropSpot initialised");
  loginAtStartup();
});

const settingsDialogButton = document.querySelector("#settings-popover-toggle");
const filesDialogButton = document.querySelector("#files-popover-toggle");

if (settingsDialogButton) {
  settingsDialogButton.addEventListener("click", () => {
    settingsDialogButton.dispatchEvent(
      new CustomEvent("popover-toggle", {
        detail: {
          selector: "#settings-popover",
          srcElement: settingsDialogButton,
        },
        bubbles: true,
      }),
    );
  });
}

if (filesDialogButton) {
  filesDialogButton.addEventListener("click", () => {
    filesDialogButton.dispatchEvent(
      new CustomEvent("popover-toggle", {
        detail: { selector: "#files-popover", srcElement: filesDialogButton },
        bubbles: true,
      }),
    );
  });
}
