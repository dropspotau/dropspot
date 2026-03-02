import { html, css, LitElement } from "lit";
import { customElement, property, state } from "lit/decorators.js";

@customElement("file-preview")
export class FilePreviewElement extends LitElement {
  static styles = css`
    :host {
      display: flex;
    }
  `;

  @property()
  private name: string = "placeholder";

  @state()
  private blobUrl: string | null = null;

  connectedCallback() {
    super.connectedCallback();
  }

  disconnectedCallback() {
    super.disconnectedCallback();
  }

  public setBuffer(buffer: Uint8Array<ArrayBuffer>): void {
    this.blobUrl = URL.createObjectURL(new Blob([buffer]));
  }

  render() {
    return html`
      <h3>${this.name}</h3>
      ${this.blobUrl ? html`<img src="${this.blobUrl}" />` : null}
    `;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "file-preview": FilePreviewElement;
  }
}
