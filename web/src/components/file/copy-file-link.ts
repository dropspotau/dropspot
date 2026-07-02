import { html, css, LitElement } from "lit";
import { customElement, property } from "lit/decorators.js";

import { createDownloadUrl, getFileLink } from "../../storage";
import { ToastElement } from "../toast";

/** Utility element which will render a button to copy a file's URL if it's been stored locally */
@customElement("copy-file-link")
export class CopyFileLinkElement extends LitElement {
  static styles = css`
    :host {
      display: contents;
    }
  `;

  @property({ attribute: "file-id" })
  private fileId: string | null = null;

  private link: string | null = null;

  connectedCallback(): void {
    super.connectedCallback();

    if (!this.fileId) {
      return;
    }

    const link = getFileLink(this.fileId);

    if (!link) {
      this.setAttribute("disabled", "");

      // This is a very niche use case to make menus as children of this element disabled. I will likely regret this
      const menu = this.querySelector("md-menu-item");

      if (menu) {
        menu.setAttribute("disabled", "");
      }
    }

    this.link = link;
    this.addEventListener("click", this.handleClick);
  }

  disconnectedCallback(): void {
    this.removeEventListener("click", this.handleClick);
  }

  private handleClick = (e: MouseEvent): void => {
    e.stopPropagation();

    if (!this.link) {
      return;
    }

    const downloadUrl = createDownloadUrl(this.link);

    navigator.clipboard
      .writeText(downloadUrl.toString())
      .then(() => {
        ToastElement.create("Copied!", "success");
      })
      .catch(() => {
        ToastElement.create(
          "Sorry, there was an error copying the link. Please try again.",
          "danger",
        );
      });
  };

  render() {
    return html`<slot></slot>`;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "copy-file-link": CopyFileLinkElement;
  }
}
