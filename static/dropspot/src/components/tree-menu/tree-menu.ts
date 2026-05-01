import { html, css, LitElement, type PropertyValues } from "lit";
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
    }
  }

  protected updated(changedProperties: PropertyValues): void {
    if (changedProperties.has("openKey") && this.openKey) {
      this.showElement(this.openKey);

      // Save the previous key in case of a back button press
      const previousKey = changedProperties.get("openKey") ?? null;
      this.keyHistory.push(previousKey);
    }
  }
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

    if (!(target instanceof Element)) {
      return;
    }

    const keyElement = target.closest("[menu-navigate-to]");
    const key = keyElement?.getAttribute("menu-navigate-to");

    if (key) {
      this.openKey = key;
    }
  };

  /**
   * Handles a programmatic menu change
   * @param e The event
   */
  private handleMenuChange = (e: MenuNavigationEvent): void => {
    const { key } = e.detail;
    this.openKey = key;
  };

  private handleBackClick = (): void => {
    const previousKey = this.keyHistory.pop();

    if (previousKey) {
      this.openKey = previousKey;
    }
  };

  render() {
    console.debug(this.openKey, this.defaultKey);
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
