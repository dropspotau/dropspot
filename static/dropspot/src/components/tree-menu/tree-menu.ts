import { html, css, LitElement } from "lit";
import { customElement, property, state } from "lit/decorators.js";

@customElement("tree-menu")
export class TreeMenuElement extends LitElement {
  static styles = css`
    :host {
      display: contents;
    }

    slot::slotted([menu-key]:not([menu-open])) {
      display: none !important;
    }

    .popover-back-button {
      position: absolute;
      top: 1rem;
      left: 1rem;
    }
  `;

  @property({ attribute: "default-key" })
  private defaultKey?: string;

  connectedCallback(): void {
    super.connectedCallback();
    this.addEventListener("menu-navigation", this.handleMenuChange);
    this.addEventListener("click", this.handleClick);

    if (this.defaultKey) {
      this.showElement(this.defaultKey);
    }
  }

  @state()
  private openIndex: number = 0;

  private showElement = (key: string): void => {
    const element = this.querySelector(`[menu-key="${key}"]`);

    if (element instanceof HTMLElement) {
      element.setAttribute("menu-open", "");
    }

    const otherElements = [
      ...this.querySelectorAll(`[menu-key]:not([menu-key="${key}"])`),
    ].filter((element) => element instanceof HTMLElement);

    for (const element of otherElements) {
      element.removeAttribute("menu-open");
    }
    console.debug(element);
    console.debug(otherElements);
  };

  private handleClick = (e: MouseEvent): void => {
    const { target } = e;

    if (!(target instanceof HTMLElement)) {
      return;
    }

    const key = target.getAttribute("menu-navigate-to");

    if (key) {
      this.showElement(key);
    }
  };

  private handleMenuChange = (e: MenuNavigationEvent): void => {
    const { key } = e.detail;
    this.showElement(key);
  };

  render() {
    const isSubPage = this.openIndex > 0;

    return html`
      ${isSubPage
        ? html`
            <slot name="back-button">
              <md-icon-button class="popover-back-button">
                <md-icon>arrow_back</md-icon>
              </md-icon-button>
            </slot>
          `
        : ""}
      <slot></slot>
    `;
  }
}

type MenuNavigationEvent = CustomEvent<{
  key: string;
}>;

declare global {
  interface HTMLElementTagNameMap {
    "tree-menu": TreeMenuElement;
  }

  interface DocumentEventMap {
    "menu-navigation": MenuNavigationEvent;
  }

  interface HTMLElementEventMap {
    "menu-navigation": MenuNavigationEvent;
  }
}
