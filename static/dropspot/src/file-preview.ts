import { html, css, LitElement, type TemplateResult } from "lit";
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
        previewHtml = html`<pre>${this.blobUrl}</pre>`;
        break;
      case null:
        previewHtml = html`<pre>Unknown file type</pre>`;
        break;
    }

    return html`
      <h3>${this.name}</h3>
      ${previewHtml}
    `;
  }
}

type PreviewType = "image" | "video" | "audio" | "text";

export const getFilePreviewType = (fileName: string): PreviewType | null => {
  const extension = fileName.split(".").pop();

  if (!extension) {
    return null;
  }

  if (["png", "jpg", "jpeg", "gif", "svg"].includes(extension)) {
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
