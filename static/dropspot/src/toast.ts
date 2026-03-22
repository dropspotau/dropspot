import { html, css, LitElement } from "lit";
import { customElement, property } from "lit/decorators.js";

import "./toast.css";

type ToastVariant = "success" | "danger";

@customElement("dropspot-toast")
export class ToastElement extends LitElement {
  static styles = css`
    :host {
      display: flex;
      place-items: center;
      gap: 0.75rem;
      color: #ffffff;
      padding: 0.5rem 1rem;
      border-radius: 0.5rem;
      box-shadow: var(--dropspot-box-shadow-minor);
      background-color: var(--dropspot-success);
      opacity: 1;

      transition: opacity 3s linear;
    }

    :host([variant="success"]) {
      background-color: var(--dropspot-success);
    }

    :host([variant="danger"]) {
      background-color: var(--dropspot-danger);
    }

    :host([fading]) {
      opacity: 0;
    }
  `;

  @property()
  private variant: ToastVariant = "success";

  static create(message: string, variant: ToastVariant) {
    const toast = document.createElement("dropspot-toast");
    toast.setAttribute("variant", variant);
    toast.textContent = message;

    document.appendChild(toast);

    return toast;
  }

  connectedCallback() {
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

  disconnectedCallback() {
    super.disconnectedCallback();
  }

  render() {
    let icon: string;

    switch (this.variant) {
      case "success":
        icon = "check";
        break;
      case "danger":
        icon = "error";
        break;
      default:
        icon = "check";
        break;
    }

    return html`
      <md-icon>${icon}</md-icon>
      <slot></slot>
    `;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "dropspot-toast": ToastElement;
  }
}
