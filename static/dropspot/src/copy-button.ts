import { html, css, LitElement } from "lit";
import { customElement, property, state } from "lit/decorators.js";
import "./copy-button.css";

@customElement("copy-button")
export class CopyButtonElement extends LitElement {
  static styles = css`
    :host {
      display: contents;
    }
  `;

  @property()
  private value: string = "placeholder";

  @state()
  private hasCopied: boolean = false;

  connectedCallback() {
    super.connectedCallback();

    this.addEventListener("click", this.handleClick);
  }

  disconnectedCallback() {
    super.disconnectedCallback();

    this.removeEventListener("click", this.handleClick);
  }

  private handleClick = async (): Promise<void> => {
    await navigator.clipboard.writeText(this.value);
    this.hasCopied = true;

    setTimeout(() => {
      this.hasCopied = false;
    }, 3000);
  };

  render() {
    return html`<md-filled-button
      >${this.hasCopied ? "Copied!" : "Copy"}</md-filled-button
    >`;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "copy-button": CopyButtonElement;
  }
}
