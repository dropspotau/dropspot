import { html, css, LitElement } from "lit";
import { customElement, property } from "lit/decorators.js";

export type AlertVariant = "success" | "info" | "danger";

@customElement("dropspot-alert")
export class AlertElement extends LitElement {
  static styles = css`
    :host {
      display: flex;
      place-items: center;
      gap: 0.75rem;
      color: #ffffff;
      padding: 1rem 2rem;
      border-radius: 0.5rem;
      box-shadow: var(--dropspot-box-shadow-minor);
      background-color: var(--dropspot-success);
      opacity: 1;

      transition: opacity 3s linear;
    }

    :host([variant="success"]) {
      background-color: var(--dropspot-success);
    }

    :host([variant="info"]) {
      background-color: var(--dropspot-grey-dark);
    }

    :host([variant="danger"]) {
      background-color: var(--dropspot-danger);
    }
  `;

  @property()
  private variant: AlertVariant = "success";

  render() {
    let icon: string;

    switch (this.variant) {
      case "success":
        icon = "check";
        break;
      case "info":
        icon = "info";
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
    "dropspot-alert": AlertElement;
  }
}
