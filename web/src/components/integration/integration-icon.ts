import type { StorageType } from "dropspot-core";
import { html, css, LitElement } from "lit";
import { customElement, property } from "lit/decorators.js";

@customElement("integration-icon")
export class IntegrationIconElement extends LitElement {
  static styles = css`
    :host {
      display: block;
      height: 1.5rem;
      width: 1.5rem;
      object-fit: contain;
      color: var(--dropspot-grey-dark);
    }

    .integration-icon {
      height: 100%;
      width: 100%;
      object-fit: contain;
    }
  `;

  @property()
  private slug: StorageType = null!;

  render() {
    switch (this.slug) {
      case "local":
        return html`<md-icon class="integration-icon">storage</md-icon>`;
      case "s3":
        return html`<md-icon class="integration-icon">storage</md-icon>`;
      case "gcs":
        return html`<img
          src="https://image.similarpng.com/file/similarpng/very-thumbnail/2020/06/Logo-google-icon-PNG.png"
          alt="Google"
          class="integration-icon"
        />`;
      default:
        throw new Error(
          `Expected storage type for <integration-icon>, got ${this.slug}`,
        );
    }
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "integration-icon": IntegrationIconElement;
  }
}
