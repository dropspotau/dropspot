import { create_user_js, login_js, type LoginResult } from "dropspot-core";
import { html, css, LitElement } from "lit";
import { customElement, state } from "lit/decorators.js";
import { setTokens, type LoginEvent } from "./auth";
import { ToastElement } from "./toast";

@customElement("login-button")
export class LoginButtonElement extends LitElement {
  static styles = css`
    :host {
      display: contents;
      --md-filled-button-container-color: #ffffff;
      --md-filled-button-label-text-color: var(--dropspot-primary);
      --md-filled-button-hover-label-text-color: var(--dropspot-primary);
      --md-filled-button-focus-label-text-color: var(--dropspot-primary);
      --md-filled-button-pressed-label-text-color: var(--dropspot-primary);
    }

    .button-contents {
      display: flex;
      flex-flow: row;
      place-items: center;
      gap: 0.5rem;
    }
  `;

  @state()
  private isSubmitting: boolean = false;

  private handleClick = async (): Promise<void> => {
    this.isSubmitting = true;
    let result: LoginResult | null = null;

    try {
      result = await login_js("alec@dropspot.au", "Password");
    } catch (e) {
      try {
        result = await create_user_js(
          "alec@dropspot.au",
          "Alec",
          "Bassingthwaighte",
          "Password",
        );
      } catch (e) {
        ToastElement.create(
          "Sorry, there was an issue logging in. Please try again",
          "danger",
        );
      }
    } finally {
      this.isSubmitting = false;
    }

    if (result) {
      setTokens(result.tokens);
      const event: LoginEvent = new CustomEvent("login", {
        detail: { user: result.user },
        bubbles: true,
      });
      this.dispatchEvent(event);
    }
  };

  render() {
    return html`
      <md-filled-button class="button-white" @click="${this.handleClick}">
        ${this.isSubmitting
          ? html`<md-circular-progress indeterminate></md-circular-progress>`
          : html`
              <div class="button-contents">
                <span>Login</span>
                <md-icon>login</md-icon>
              </div>
            `}
      </md-filled-button>
    `;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "login-button": LoginButtonElement;
  }
}
