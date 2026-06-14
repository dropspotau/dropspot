import { html, css, LitElement } from "lit";
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
  `;

  private welcomePopoverRef: Ref<PopoverElement> = createRef();
  private settingsPopoverRef: Ref<PopoverElement> = createRef();
  private filesPopoverRef: Ref<PopoverElement> = createRef();

  // 0 === welcome, 1 === settings, 2 === files, 3 === finished
  private step: 0 | 1 | 2 | 3 = 0;

  connectedCallback(): void {
    super.connectedCallback();

    if (this.shadowRoot) {
      applyGlobalStyles(this.shadowRoot);
    }
  }

  /** Advances through the onboarding by one step */
  private advanceStep = (): void => {
    const { value: welcomePopover } = this.welcomePopoverRef;
    const { value: settingsPopover } = this.settingsPopoverRef;
    const { value: filesPopover } = this.filesPopoverRef;

    if (!welcomePopover || !settingsPopover || !filesPopover) {
      return;
    }

    const uploadCircle = document.querySelector("#upload");
    const settingsDialogButton = document.querySelector(
      "#settings-popover-toggle",
    );
    const filesDialogButton = document.querySelector("#files-popover-toggle");

    if (this.step === 0 && uploadCircle instanceof HTMLElement) {
      welcomePopover.toggle(uploadCircle);
      settingsPopover.close();
      filesPopover.close();
    }

    if (this.step === 1 && settingsDialogButton instanceof HTMLElement) {
      welcomePopover.close();
      settingsPopover.toggle(settingsDialogButton);
      filesPopover.close();
    }

    if (this.step === 2 && filesDialogButton instanceof HTMLElement) {
      welcomePopover.close();
      settingsPopover.close();
      filesPopover.toggle(filesDialogButton);
    }

    if (this.step === 3) {
      welcomePopover.close();
      settingsPopover.close();
      filesPopover.close();
    }

    this.step = Math.min(this.step + 1, 3) as typeof this.step;
  };

  render() {
    return html`
      <dropspot-popover alignment="center" ${ref(this.welcomePopoverRef)}>
        <section class="settings">Welcome!</section>
      </dropspot-popover>
      <dropspot-popover alignment="left" ${ref(this.settingsPopoverRef)}>
        <section class="settings">Settings onboarding</section>
      </dropspot-popover>
      <dropspot-popover alignment="right" ${ref(this.filesPopoverRef)}>
        <section class="settings">Files onboarding</section>
      </dropspot-popover>
      <md-filled-button class="button-white" @click="${this.advanceStep}"
        >Toggle</md-filled-button
      >
    `;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "dropspot-onboarding": OnboardingElement;
  }
}
