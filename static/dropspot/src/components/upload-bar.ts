import {
  upload_js,
  create_link_js,
  type UploadResult,
  type Integration,
} from "dropspot-core";
import { html, css, LitElement } from "lit";
import { customElement, property, state } from "lit/decorators.js";

import { getAuth } from "../auth";
import { ToastElement } from "../toast";

const createDownloadUrl = (identifier: string): URL => {
  const url = new URL(window.location.href);
  url.searchParams.set("file", identifier);

  return url;
};

/**
 * A component which shows upload progress of a file, as well as options about which provider to upload with when multiple are available
 */
@customElement("upload-bar")
export class UploadBarElement extends LitElement {
  static styles = css`
    :host {
      display: flex;
      flex-flow: row;
      place-items: center;
      place-content: space-between;
      flex: 0 0 4rem;
      padding: 1rem 2rem;
      border-radius: 1rem;
      gap: 1rem;
      align-items: center;
      background-color: var(--dropspot-dark);
    }
  `;

  @property()
  private file: File = null!;

  @property()
  private integrations: Integration[] = [];

  @state()
  private uploadResult: UploadResult | null = null;

  connectedCallback() {
    super.connectedCallback();
    this.startUpload();
  }

  disconnectedCallback() {
    super.disconnectedCallback();
  }

  public static create = (
    file: File,
    integrations: Integration[],
  ): UploadBarElement => {
    const container = document.querySelector("#recent-uploads");

    if (!container) {
      throw new Error(
        "Cannot create upload bar. No #recent-uploads container.",
      );
    }

    const element = document.createElement("upload-bar");
    element.setAttribute("file", JSON.stringify(file));
    element.setAttribute("integrations", JSON.stringify(integrations));
    container.appendChild(element);

    return element;
  };

  private startUpload = async (): Promise<void> => {
    const fileContents = new Uint8Array(await this.file.arrayBuffer());
    const auth = getAuth();

    let result: UploadResult;

    try {
      result = await upload_js(this.file.name, fileContents, auth, "local");
    } catch (e) {
      ToastElement.create(
        "Sorry, there was an issue uploading the file. Please try again.",
        "danger",
      );
      return;
    }

    this.uploadResult = result;

    const event: FileUploadEvent = new CustomEvent("file-upload", {
      detail: { upload: result },
      bubbles: true,
    });
    this.dispatchEvent(event);
  };

  render() {
    if (this.uploadResult) {
      const link = create_link_js(
        this.uploadResult.file.id,
        this.uploadResult.encryption,
      );
      const url = createDownloadUrl(link);

      return html`
        <h3 class="text-white no-margin">${this.uploadResult.file.name}</h3>
        <copy-button value="${url}" class="button-white"></copy-button>
      `;
    }

    console.debug(this.integrations);
    return html`
      <h3 class="text-white no-margin">Uploading ${this.file.name}...</h3>
      <div>
        <!-- Filler for space-between -->
      </div>
    `;
  }
}

export type FileUploadEvent = CustomEvent<{
  upload: UploadResult;
}>;

export type FileDownloadEvent = CustomEvent<{
  file: File;
}>;

declare global {
  interface DocumentEventMap {
    "file-upload": FileUploadEvent;
    "file-download": FileDownloadEvent;
  }
  interface HTMLElementTagNameMap {
    "upload-bar": UploadBarElement;
  }
}
