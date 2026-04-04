import type { GcsIntegration, LocalIntegration } from "dropspot-core";
import { html, css, LitElement } from "lit";
import { customElement, property } from "lit/decorators.js";

@customElement("dropspot-integration")
export class IntegrationElement extends LitElement {
  static styles = css`
    :host {
    }
  `;

  @property({ type: Object })
  private integration: LocalIntegration | GcsIntegration | null = null;

  connectedCallback() {
    super.connectedCallback();
  }

  disconnectedCallback() {
    super.disconnectedCallback();
  }

  render() {
    if (!this.integration) {
      return html`No integration`;
    }

    return html` ${this.integration.slug} `;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "dropspot-integration": IntegrationElement;
  }
}
