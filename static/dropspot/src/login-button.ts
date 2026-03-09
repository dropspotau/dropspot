import { create_user_js, login_js, type User } from "dropspot-core";
import { html, css, LitElement } from "lit";
import { customElement, state } from "lit/decorators.js";

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
    let user: User | null = null;

    try {
      user = await login_js("alec@dropspot.au", "Password");
    } catch (e) {
      user = await create_user_js(
        "alec@dropspot.au",
        "Password",
        "Alec",
        "Bassingthwaighte",
      );
    } finally {
      this.isSubmitting = false;
    }

    console.debug("Logged in as user", user);
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
