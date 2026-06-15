import { html, css, LitElement, type TemplateResult } from "lit";
import { customElement } from "lit/decorators.js";

import { applyGlobalStyles } from "../../style";
import type { PopoverElement } from "../popover";
import { createRef, ref, type Ref } from "lit/directives/ref.js";

@customElement("dropspot-onboarding")
export class OnboardingElement extends LitElement {
  static styles = css`
    :host {
      display: contents;
    }

    .title {
      align-self: center;
    }

    .container {
      display: flex;
      flex-flow: column;
      gap: 1rem;
    }

    .contents {
      display: flex;
      flex-flow: column;
      gap: 1rem;
    }

    .welcome {
      /* Overwrites the .settings class's sizes */
      --menu-padding: 2rem !important;
      --menu-height: fit-content !important;
      --menu-width: calc(48rem - var(--menu-padding)) !important;
      overflow: auto !important;
    }

    .onboarding-popover {
      height: fit-content !important;
    }

    .button-container {
      align-self: center;
    }

    .integration-list {
      display: flex;
      flex-flow: column;
      gap: 1rem;
      margin-top: 2rem;
      padding: 0 2rem;
    }

    .integration {
      display: flex;
      align-items: center;
      gap: 0.5rem;
    }
  `;

  private welcomePopoverRef: Ref<PopoverElement> = createRef();
  private uploadPopoverRef: Ref<PopoverElement> = createRef();
  private settingsPopoverRef: Ref<PopoverElement> = createRef();
  private filesPopoverRef: Ref<PopoverElement> = createRef();

  // 0 === welcome, 1 === upload, 2 === settings, 3 === files, 4 === finished
  private step: 0 | 1 | 2 | 3 | 4 = 0;

  connectedCallback(): void {
    super.connectedCallback();

    if (this.shadowRoot) {
      applyGlobalStyles(this.shadowRoot);
    }

    setTimeout(() => {
      this.advanceStep();
    }, 0);
  }

  /** Advances through the onboarding by one step */
  private advanceStep = (): void => {
    const { value: welcomePopover } = this.welcomePopoverRef;
    const { value: uploadPopover } = this.uploadPopoverRef;
    const { value: settingsPopover } = this.settingsPopoverRef;
    const { value: filesPopover } = this.filesPopoverRef;

    if (
      !welcomePopover ||
      !uploadPopover ||
      !settingsPopover ||
      !filesPopover
    ) {
      return;
    }

    const uploadCircle = document.querySelector("#upload");
    const settingsDialogButton = document.querySelector(
      "#settings-popover-toggle",
    );
    const filesDialogButton = document.querySelector("#files-popover-toggle");

    if (this.step === 0 && uploadCircle instanceof HTMLElement) {
      welcomePopover.toggle(uploadCircle);
    }

    if (this.step === 1 && uploadCircle instanceof HTMLElement) {
      welcomePopover.close();
      uploadPopover.toggle(uploadCircle);
    }

    if (this.step === 2 && settingsDialogButton instanceof HTMLElement) {
      uploadPopover.close();
      settingsPopover.toggle(settingsDialogButton);
    }

    if (this.step === 3 && filesDialogButton instanceof HTMLElement) {
      settingsPopover.close();
      filesPopover.toggle(filesDialogButton);
    }

    if (this.step === 4) {
      filesPopover.close();
      this.remove();
    }

    this.step = Math.min(this.step + 1, 4) as typeof this.step;
  };

  private renderWelcome = (): TemplateResult<1> => html`
    <div class="contents">
      <h1 class="no-margin title">🚀 Welcome to DropSpot! 🚀</h1>
      <p class="no-margin">
        DropSpot is a file sharing tool which handles file expiry and download
        limits. Upload, send the link and trust that the file will be cleaned up
        on its own.
      </p>
      <p class="no-margin">
        You are able to customise where files upload to, how long they are
        downloadable for and how many times they can be downloaded.
      </p>
      <div class="integration-list">
        <h3 class="no-margin">Available Integrations</h3>
        <div class="integration">
          <integration-icon slug="local"></integration-icon>
          <span>Local file storage</span>
        </div>
        <div class="integration">
          <integration-icon slug="gcs"></integration-icon>
          <span>Google Cloud Storage</span>
        </div>
        <div class="integration">
          <integration-icon slug="s3"></integration-icon>
          <span>AWS S3</span>
        </div>
      </div>
    </div>
    <div class="button-container">
      <md-filled-button class="button-primary" @click="${this.advanceStep}"
        >Got it</md-filled-button
      >
    </div>
  `;

  private renderUpload = (): TemplateResult<1> => html`
    <div class="contents">
      <h1 class="no-margin title">⬆️ Uploads ⬆️</h1>
      <p class="no-margin">
        Here you can drag a file or select one to upload it.
      </p>
      <p class="no-margin">
        When it uploads, a link will appear which you can copy and send to
        someone else.
      </p>
      <p class="no-margin">
        After a certain amount of time has passed or downloads, the file will
        expire and become unavailable to be deleted.
      </p>
      <div class="button-container">
        <md-filled-button class="button-primary" @click="${this.advanceStep}"
          >Got it</md-filled-button
        >
      </div>
    </div>
  `;

  private renderSettings = (): TemplateResult<1> => html`
    <div class="contents">
      <h1 class="no-margin title">⚙️ Settings ⚙️</h1>
      <p class="no-margin">
        Here you can change default file expiry and download limits,
        integrations and see your organisation's users.
      </p>
    </div>
    <div class="button-container">
      <md-filled-button class="button-primary" @click="${this.advanceStep}"
        >Got it</md-filled-button
      >
    </div>
  `;

  private renderFiles = (): TemplateResult<1> => html`
    <div class="contents">
      <h3 class="no-margin title">💾 Files 💾</h3>
      <p class="no-margin">Here you can find all your active files.</p>
    </div>
    <div class="button-container">
      <md-filled-button class="button-primary" @click="${this.advanceStep}"
        >Got it</md-filled-button
      >
    </div>
  `;

  render() {
    return html`
      <dropspot-popover alignment="center" ${ref(this.welcomePopoverRef)}>
        <section class="onboarding-popover welcome settings container">
          ${this.renderWelcome()}
        </section>
      </dropspot-popover>
      <dropspot-popover alignment="left" ${ref(this.uploadPopoverRef)}>
        <section class="onboarding-popover welcome settings container">
          ${this.renderUpload()}
        </section>
      </dropspot-popover>
      <dropspot-popover alignment="left" ${ref(this.settingsPopoverRef)}>
        <section class="onboarding-popover settings container">
          ${this.renderSettings()}
        </section>
      </dropspot-popover>
      <dropspot-popover alignment="right" ${ref(this.filesPopoverRef)}>
        <section class="onboarding-popover settings container">
          ${this.renderFiles()}
        </section>
      </dropspot-popover>
    `;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "dropspot-onboarding": OnboardingElement;
  }
}
