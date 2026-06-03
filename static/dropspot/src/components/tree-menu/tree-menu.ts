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

  /** The current open key */
  @state()
  private openKey: string | null = null;
  private keyHistory: string[] = [];

  connectedCallback(): void {
    super.connectedCallback();
    this.addEventListener("menu-navigation", this.handleMenuChange);
    this.addEventListener("click", this.handleClick);

    if (this.defaultKey) {
      // Open the default key by default
      this.openKey = this.defaultKey;
      this.showElement(this.openKey);
    }
  }

  /**
   * Shows the page with the given key and hides all other pages
   * @param key The page key to show
   */
  private showElement = (key: string): void => {
    const menuKeyElements = [...this.querySelectorAll("[menu-key]")].filter(
      (element) => element instanceof HTMLElement,
    );

    for (const element of menuKeyElements) {
      const isTarget = element.getAttribute("menu-key") === key;

      if (isTarget) {
        element.setAttribute("menu-open", "");
      } else {
        element.removeAttribute("menu-open");
      }
    }
  };

  /**
   * Handles a menu change via child element click
   * @param e The event
   */
  private handleClick = (e: MouseEvent): void => {
    const { target } = e;

    if (
      !(target instanceof Element) ||
      // Some elements (usually buttons) deliberately don't want to trigger a menu navigation
      target.hasAttribute("ignore-menu-navigation")
    ) {
      return;
    }

    const keyElement = target.closest("[menu-navigate-to]");
    const key = keyElement?.getAttribute("menu-navigate-to");

    if (key && this.openKey && key !== this.openKey) {
      // Record this key in the history, in case we go back
      this.keyHistory.push(this.openKey);
    }

    if (key) {
      this.openKey = key;
      this.showElement(key);
    }
  };

  /**
   * Handles a programmatic menu change
   * @param e The event
   */
  private handleMenuChange = (e: MenuNavigationEvent): void => {
    const { key } = e.detail;

    if (this.openKey) {
      // Record this key in the history, in case we go back
      this.keyHistory.push(this.openKey);
    }

    this.openKey = key;
    this.showElement(key);
  };

  /**
   * Goes back to the previous page
   * @param e The event
   */
  private handleBackClick = (): void => {
    const previousKey = this.keyHistory.pop();

    if (previousKey) {
      this.openKey = previousKey;
      this.showElement(this.openKey);
    }
  };

  render() {
    const isSubPage = this.openKey !== this.defaultKey;

    return html`
      ${isSubPage
        ? html`
            <slot name="back-button">
              <md-icon-button
                class="popover-back-button"
                @click="${this.handleBackClick}"
              >
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
