import { html, css, LitElement } from "lit";
import { customElement, property } from "lit/decorators.js";
import { deleteFile } from "dropspot-core";
import { getAuth } from "../auth";
import { ToastElement } from "../toast";

@customElement("delete-file")
export class DeleteFileElement extends LitElement {
  static styles = css`
    :host {
      display: contents;
    }
  `;

  @property({ attribute: "file-id" })
  private fileId: string | null = null;

  connectedCallback() {
    super.connectedCallback();

    if (!this.fileId) {
      throw new Error(
        "file-id must be provided with the <delete-file> element",
      );
    }

    this.addEventListener("click", this.handleClick);
  }

  disconnectedCallback() {
    super.disconnectedCallback();
    this.removeEventListener("click", this.handleClick);
  }

  private handleClick = async (): Promise<void> => {
    const auth = getAuth();

    if (!this.fileId || !auth) {
      return;
    }

    try {
      await deleteFile(this.fileId, auth);
      ToastElement.create("Deleted!", "success");
    } catch (e) {
      ToastElement.create("Failed to delete file", "danger");
    }

    const fileDeleteEvent: FileDeleteEvent = new CustomEvent("file-delete", {
      detail: { id: this.fileId },
      bubbles: true,
    });
    this.dispatchEvent(fileDeleteEvent);
  };

  render() {
    return html`<slot></slot>`;
  }
}

export type FileDeleteEvent = CustomEvent<{ id: string }>;

declare global {
  interface HTMLElementTagNameMap {
    "delete-file": DeleteFileElement;
  }
  interface DocumentEventMap {
    "file-delete": FileDeleteEvent;
  }
}
