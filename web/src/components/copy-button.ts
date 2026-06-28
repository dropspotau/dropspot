import { html, css, LitElement } from "lit";
import { customElement, property, state } from "lit/decorators.js";

import { ToastElement } from "./toast";
import { applyGlobalStyles } from "../style";

@customElement("copy-button")
export class CopyButtonElement extends LitElement {
  static styles = css`
    :host {
      display: contents;
    }
  `;

  @property()
  private value: string | null = null;

  @state()
  private hasCopied: boolean = false;

  connectedCallback() {
    super.connectedCallback();

    if (this.shadowRoot) {
      applyGlobalStyles(this.shadowRoot);
    }

    this.addEventListener("click", this.handleClick);
  }

  disconnectedCallback() {
    super.disconnectedCallback();

    this.removeEventListener("click", this.handleClick);
  }

  private handleClick = async (): Promise<void> => {
    if (!this.value) {
      throw new Error(
        "<copy-button> was provided with no `value` attribute to copy",
      );
    }

    try {
      await navigator.clipboard.writeText(this.value);
    } catch (e) {
      ToastElement.create(
        "Sorry, there was an error copying to the clipboard. Please try again.",
        "danger",
      );
    }
    this.hasCopied = true;

    setTimeout(() => {
      this.hasCopied = false;
    }, 3000);
  };

  render() {
    return html`
      <md-filled-button class="button-white">
        ${this.hasCopied ? "Copied!" : "Copy"}
      </md-filled-button>
    `;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "copy-button": CopyButtonElement;
  }
}
