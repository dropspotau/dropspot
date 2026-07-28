import { css, html, LitElement } from "lit";
import { customElement, property } from "lit/decorators.js";

import githubIcon from "../assets/github-icon.png";

@customElement("dropspot-icon")
export class IconElement extends LitElement {
  static styles = css`
    :host {
      display: block;
      height: 1.5rem;
      width: 1.5rem;
      object-fit: contain;
      color: var(--dropspot-grey-dark);
    }

    .icon {
      height: 100%;
      width: 100%;
      object-fit: contain;
    }
  `;

  @property({ attribute: "icon" })
  private iconName: string | null = null;

  render() {
    if (!this.iconName) {
      return null;
    }

    let icon = this.iconName;

    if (icon === "github") {
      icon = githubIcon;
    }

    return html`<img src="${icon}" class="icon" />`;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "dropspot-icon": IconElement;
  }
}
