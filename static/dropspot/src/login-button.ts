import {
  create_user_js,
  login_js,
  type LoginResult,
  type User,
} from "dropspot-core";
import { html, css, LitElement } from "lit";
import { customElement, state } from "lit/decorators.js";
import { setTokens } from "./auth";

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
      result = await create_user_js(
        "alec@dropspot.au",
        "Alec",
        "Bassingthwaighte",
        "Password",
      );
    } finally {
      this.isSubmitting = false;
    }

    if (result) {
      setTokens(result.tokens);
      this.dispatchEvent(
        new CustomEvent("login", {
          detail: { user: result.user },
          bubbles: true,
        }),
      );
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

type LoginEvent = CustomEvent<{ user: User }>;

declare global {
  interface HTMLElementTagNameMap {
    "login-button": LoginButtonElement;
  }

  interface DocumentEventMap {
    login: LoginEvent;
  }
}
