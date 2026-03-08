import { html, css, LitElement, type TemplateResult } from "lit";
import { customElement, property, state } from "lit/decorators.js";
import { download } from "./download";

@customElement("file-preview")
export class FilePreviewElement extends LitElement {
  static styles = css`
    :host {
      display: flex;
    }

    .modal-content {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 2rem;
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
  `;

  @property()
  private name: string = "";

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
  };

  private handleDownload = (): void => {
    if (this.blobUrl) {
      download(this.name, this.blobUrl);
    }
  };

  render() {
    const previewType = getFilePreviewType(this.name);
    let previewHtml: TemplateResult<1>;

    switch (previewType) {
      case "image":
        previewHtml = html`<img src="${this.blobUrl}" />`;
        break;
      case "video":
        previewHtml = html`<video src="${this.blobUrl}" controls />`;
        break;
      case "audio":
        previewHtml = html`<audio src="${this.blobUrl}" />`;
        break;
      case "text":
        previewHtml = html`<pre class="text">${this.blobUrl}</pre>`;
        break;
      case null:
        previewHtml = html`<pre class="text">Unknown file type</pre>`;
        break;
    }

    return html`
      <dropspot-modal
        open="${this.isModalOpen}"
        .onClose="${() => {
          this.isModalOpen = false;
        }}"
      >
        <div class="modal-content">
          <h3 class="title">${this.name}</h3>
          ${this.blobUrl && previewHtml}
          <div class="buttons">
            <md-outlined-button @click="${this.handleClose}">
              <div class="button-items">
                <span>Close</span>
                <md-icon>close</md-icon>
              </div>
            </md-outlined-button>
            <md-filled-button @click="${this.handleDownload}">
              <div class="button-items">
                <span>Download</span>
                <md-icon>download</md-icon>
              </div>
            </md-filled-button>
          </div>
        </div>
      </dropspot-modal>
    `;
  }
}

type PreviewType = "image" | "video" | "audio" | "text";

const getFilePreviewType = (fileName: string): PreviewType | null => {
  const extension = fileName.split(".").pop();

  if (!extension) {
    return null;
  }

  if (["png", "jpg", "jpeg", "gif", "svg", "webp"].includes(extension)) {
    return "image";
  }

  if (["mp4", "webm", "ogg", "mp3", "wav", "flac"].includes(extension)) {
    return "video";
  }

  if (["mp3", "wav", "flac"].includes(extension)) {
    return "audio";
  }

  if (["txt", "md", "html", "css", "js"].includes(extension)) {
    return "text";
  }

  return null;
};

declare global {
  interface HTMLElementTagNameMap {
    "file-preview": FilePreviewElement;
  }
}
