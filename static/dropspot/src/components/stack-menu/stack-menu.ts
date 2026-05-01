import { html, css, LitElement } from "lit";
import { customElement, state } from "lit/decorators.js";

@customElement("stack-menu")
export class StackMenuElement extends LitElement {
  static styles = css`
    :host {
      display: contents;
    }

    .popover-back-button {
      position: absolute;
      top: 1rem;
      left: 1rem;
    }
  `;

  connectedCallback(): void {
    super.connectedCallback();
    this.addEventListener("menu-navigation", this.handleMenuChange);
  }

  @state()
  private openIndex: number = 0;

  private handleMenuChange = (e: MenuNavigationEvent): void => {
    // @ts-ignore
    const { name, index } = e.detail;
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
  name: string;
  index: number;
}>;

declare global {
  interface HTMLElementTagNameMap {
    "stack-menu": StackMenuElement;
  }

  interface DocumentEventMap {
    "menu-navigation": MenuNavigationEvent;
  }

  interface HTMLElementEventMap {
    "menu-navigation": MenuNavigationEvent;
  }
}
