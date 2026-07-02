import "@material/web/button/filled-button.js";
import "@material/web/button/outlined-button.js";
import "@material/web/icon/icon.js";
import "@material/web/iconbutton/icon-button.js";
import "@material/web/progress/circular-progress.js";
import "@material/web/menu/menu.js";
import "@material/web/menu/menu-item.js";
import "@material/web/textfield/filled-text-field.js";
import "@material/web/select/outlined-select.js";
import "@material/web/select/select-option.js";
import "@material/web/switch/switch.js";
import init from "@dropspot/dropspot-js";

import "./download.css";
import "./header.css";
import "./file.css";
import "./form.css";
import "./index.css";
import "./material.css";
import "./menu.css";
import "./settings.css";
import "./theme.css";
import "./uploads.css";
import "./utils.css";

import "./background";

import { loginAtStartup } from "./auth";

// Import all components
export * from "./components";

init().then(() => {
  console.log("DropSpot initialised");
  loginAtStartup();
});

const main = document.querySelector("main");
const settingsDialogButton = document.querySelector("#settings-popover-toggle");
const filesDialogButton = document.querySelector("#files-popover-toggle");
const onboardingPreviewTrigger = document.querySelector(
  "#onboarding-preview-trigger",
);

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

if (main && onboardingPreviewTrigger) {
  onboardingPreviewTrigger.addEventListener("click", () => {
    // Make onboarding appear if the unauthorised user has clicked the "See how it works." microlink
    const hasExistingOnboarding = !!document.querySelector(
      "dropspot-onboarding",
    );

    if (hasExistingOnboarding) {
      return;
    }

    const onboarding = document.createElement("dropspot-onboarding");
    main.appendChild(onboarding);
  });

  document.addEventListener("login", () => {
    // Remove the section when logging in
    const previewSection = onboardingPreviewTrigger.closest("section");

    if (previewSection) {
      previewSection.remove();
    }
  });
}
