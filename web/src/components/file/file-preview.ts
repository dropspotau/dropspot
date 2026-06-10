import { html, css, LitElement, type TemplateResult } from "lit";
import { customElement, property, state } from "lit/decorators.js";
import { download } from "../../download";

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

export type PreviewType = "image" | "video" | "audio" | "text";

export const getFilePreviewType = (fileName: string): PreviewType | null => {
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
