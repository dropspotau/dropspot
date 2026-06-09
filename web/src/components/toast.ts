import { html, css, LitElement } from "lit";
import { customElement, property } from "lit/decorators.js";
import type { AlertVariant } from "./alert";

import "./toast.css";

@customElement("dropspot-toast")
export class ToastElement extends LitElement {
  static styles = css`
    :host([fading]) {
      opacity: 0;
    }
  `;

  @property()
  private variant: AlertVariant = "success";

  static create(message: string, variant: AlertVariant) {
    const toast = document.createElement("dropspot-toast");
    toast.setAttribute("variant", variant);
    toast.textContent = message;

    document.body.appendChild(toast);

    return toast;
  }

  connectedCallback(): void {
    super.connectedCallback();
    const host = document.querySelector("#toast-container");
    const hasNotMounted = host && !host.contains(this);

    if (hasNotMounted) {
      // Move this element to the correct location
      host.appendChild(this);
    }

    // Fade, then delete
    setTimeout(() => {
      this.setAttribute("fading", "");

      setTimeout(() => {
        this.remove();
      }, 3000);
    }, 3000);
  }

  disconnectedCallback(): void {
    super.disconnectedCallback();
  }

  render() {
    return html`
      <dropspot-alert .variant="${this.variant}">
        <slot></slot>
      </dropspot-alert>
    `;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "dropspot-toast": ToastElement;
  }
}
