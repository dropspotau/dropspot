import { html, css, LitElement } from "lit";
import { customElement } from "lit/decorators.js";

@customElement("dropspot-modal")
export class ModalElement extends LitElement {
  static styles = css`
    :host {
      position: absolute;
      display: contents;
      top: -100dvh;
      left: 0;
      width: 100%;
      height: 100%;
      background-color: rgba(0, 0, 0, 0.5);
      display: flex;
      justify-content: center;
      align-items: center;
      z-index: 10;
      transition: top 0.3s ease-in-out;
    }

    :host([open='true']) {
      top: 0;
    }

    .container {
      display: flex;
      flex-direction: column;
      background-color: white;
      padding: 1rem;
      border-radius: 0.5rem;
      box-shadow: 0 0 0.5rem rgba(0, 0, 0, 0.2);
      max-height: 80%;
      max-width: 64%;
      padding: 2rem;
    }
  `;

  render() {
    return html`<div class="container">
      <slot></slot>
    </div>`;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "dropspot-modal": ModalElement;
  }
}
