import { html, css, LitElement, type TemplateResult } from "lit";
import { customElement, property, state } from "lit/decorators.js";
import { download } from "../../download";

export type PreviewType = "image" | "video" | "audio" | "text" | "file";

export const getFilePreviewType = (mimeType: string): PreviewType | null => {
  const previewType = mimeType.split("/").at(0);

  if (!previewType) {
    return null;
  }

  if (["font", "application"].includes(previewType)) {
    return "file";
  }

  // The remaining MIME types map to preview types
  return previewType as PreviewType;
};

@customElement("file-preview")
export class FilePreviewElement extends LitElement {
  static styles = css`
    :host {
      display: flex;
    }

    .title {
      color: var(--dropspot-primary);
      margin: 0;
    }

    .text {
      color: var(--dropspot-primary);
    }

    .buttons {
      display: flex;
      gap: 2rem;
    }

    img,
    video,
    audio,
    pre {
      max-height: 100%;
      max-width: 100%;
    }

    .button-items {
      --md-circule-progress-size: 2rem;
      display: flex;
      place-items: center;
      gap: 0.5rem;
    }

    .download {
      display: flex;
      place-items: center;
      place-content: center;
    }
  `;

  @property()
  private name: string = "";

  @property({ attribute: "mime-type" })
  private mimeType: string = "";

  @state()
  private blobUrl: string | null = null;

  @state()
  private isModalOpen: boolean = false;

  public setBuffer(buffer: Uint8Array<ArrayBuffer>): void {
    this.blobUrl = URL.createObjectURL(new Blob([buffer]));
    this.isModalOpen = true;
  }

  private handleClose = (): void => {
    this.isModalOpen = false;
    this.stopMedia();
  };

  private handleDownload = (): void => {
    if (this.blobUrl) {
      download(this.name, this.blobUrl);
    }
  };

  /** Stops audio and videos from playing when the modal is closed */
  private stopMedia = (): void => {
    const video = this.shadowRoot?.querySelector("video");
    const audio = this.shadowRoot?.querySelector("audio");

    if (video && !video.paused) {
      video.pause();
    }

    if (audio && !audio.paused) {
      audio.pause();
    }
  };

  render() {
    const previewType = getFilePreviewType(this.mimeType);
    let previewHtml: TemplateResult<1>;

    switch (previewType) {
      case "image":
        previewHtml = html`<img src="${this.blobUrl}" />`;
        break;
      case "video":
        previewHtml = html`
          <video controls>
            <source src="${this.blobUrl}" type="${this.mimeType}" />
          </video>
        `;
        break;
      case "audio":
        previewHtml = html`<audio src="${this.blobUrl}" />`;
        break;
      case "file":
        previewHtml = html`
          <div class="download" @click="${this.handleDownload}">
            <md-icon>download</md-icon>
          </div>
        `;
        break;
      case "text":
        previewHtml = html`<pre class="text">${this.blobUrl}</pre>`;
        break;
      case null:
        previewHtml = html`<pre class="text">Unknown file type</pre>`;
        break;
    }

    return html`
      <dropspot-modal .open="${this.isModalOpen}" @close="${this.handleClose}">
        <h3 slot="title" class="title">${this.name}</h3>
        ${this.blobUrl && previewHtml}
        <div slot="footer" class="buttons">
          <md-filled-button @click="${this.handleDownload}">
            <div class="button-items">
              <span>Download</span>
              <md-icon>download</md-icon>
            </div>
          </md-filled-button>
        </div>
      </dropspot-modal>
    `;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "file-preview": FilePreviewElement;
  }
}
