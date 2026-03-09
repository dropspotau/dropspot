import { html, css, LitElement } from "lit";
import { customElement, property } from "lit/decorators.js";
import type { PreviewType } from "./file-preview";

const getIcon = (previewType: PreviewType | string): string => {
  switch (previewType) {
    case "image":
      return "image";
    case "video":
      return "videocam";
    case "audio":
      return "music_note";
    case "text":
      return "description";
    default:
      return "description";
  }
};

@customElement("file-icon")
export class FileIconElement extends LitElement {
  static styles = css`
    :host {
      display: contents;
    }
  `;

  @property()
  private extension: string = "txt";

  render() {
    return html`<md-icon>${getIcon(this.extension)}</md-icon>`;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "file-icon": FileIconElement;
  }
}
