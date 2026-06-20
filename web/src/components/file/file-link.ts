import { html, css, LitElement } from "lit";
import { customElement, property } from "lit/decorators.js";

import { getFileLink } from "../../storage";

/** Utility element which will render a button to copy a file's URL if it's been stored locally */
@customElement("file-link")
export class FileLinkElement extends LitElement {
  static styles = css`
    :host {
      display: contents;
    }
  `;

  @property({ attribute: "file-id" })
  private fileId: string | null = null;

  connectedCallback(): void {
    super.connectedCallback();

    if (!this.fileId) {
      return;
    }

    const link = getFileLink(this.fileId);

    if (!link) {
      this.remove();
    }

    this.addEventListener("click", this.handleClick);
  }

  disconnectedCallback(): void {
    this.removeEventListener("click", this.handleClick);
  }

  private handleClick = (): void => {
    console.debug(this.fileId);
  };

  render() {
    return html`<slot></slot>`;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "file-link": FileLinkElement;
  }
}
