import {
  upload_js,
  create_link_js,
  preview_upload_js,
  type UploadResult,
  type Integration,
} from "dropspot-core";
import { html, css, LitElement } from "lit";
import { customElement, state } from "lit/decorators.js";

import { getAuth } from "../auth";
import { ToastElement } from "../toast";
import { applyGlobalStyles } from "../style";

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

  // NOTE(alec): Can't use a Lit property as this is set calling setAttribute
  private file: File = null!;

  @state()
  private uploadResult: UploadResult | null = null;

  @state()
  private integrations: Integration[] = [];

  connectedCallback() {
    super.connectedCallback();

    if (this.shadowRoot) {
      applyGlobalStyles(this.shadowRoot);
    }

    this.verifyUpload();
  }

  disconnectedCallback() {
    super.disconnectedCallback();
  }

  public static create = (file: File): UploadBarElement => {
    const container = document.querySelector("#recent-uploads");

    if (!container) {
      throw new Error(
        "Cannot create upload bar. No #recent-uploads container.",
      );
    }

    const element = document.createElement("upload-bar");
    element.setFile(file); // Not a plain object, so JSON.stringify results in {}
    container.appendChild(element);

    return element;
  };

  public setFile = (file: File): void => {
    this.file = file;
  };

  private verifyUpload = async (): Promise<boolean> => {
    const auth = getAuth();
    const uploadPreview = await preview_upload_js(auth, {
      size: this.file.size,
    });
    const { can_upload: canUpload, integrations } = uploadPreview;
    this.integrations = integrations;

    return canUpload;
  };

  // @ts-ignore
  private startUpload = async (doUpload: boolean): Promise<void> => {
    const fileContents = new Uint8Array(await this.file.arrayBuffer());
    const auth = getAuth();

    let result: UploadResult;
    const integration = this.integrations
      .filter((integration) => integration.is_active)
      .at(0);

    if (!integration) {
      ToastElement.create("No integrations to use for file upload", "danger");
      return;
    }

    if (doUpload) {
      try {
        result = await upload_js(
          this.file.name,
          fileContents,
          auth,
          integration.slug,
        );
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
    }
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

    const isSelectingIntegrations =
      this.integrations.length > 1 && !this.uploadResult;
    if (isSelectingIntegrations) {
      return html`
        <h3 class="text-white no-margin">
          How should ${this.file.name} be uploaded?
        </h3>
        <div>Integrations</div>
      `;
    }

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
