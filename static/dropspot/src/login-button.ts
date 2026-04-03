import { create_user_js, login_js, type LoginResult } from "dropspot-core";
import { html, css, LitElement, type TemplateResult } from "lit";
import { customElement, state } from "lit/decorators.js";
import { setTokens, type LoginEvent } from "./auth";
import { ToastElement } from "./toast";
import { applyGlobalStyles } from "./style";

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

    .container {
      display: flex;
      flex-flow: column;
      place-items: center;
      gap: 1rem;
      min-width: 32dvh;
      color: var(--dropspot-primary);
    }

    .form {
      display: flex;
      flex-flow: column;
      gap: 1rem;
    }

    .form-row {
      display: flex;
      gap: 1rem;
    }
  `;

  @state()
  private isSubmitting: boolean = false;

  @state()
  private isOpen: boolean = true;

  connectedCallback(): void {
    super.connectedCallback();

    if (this.shadowRoot) {
      applyGlobalStyles(this.shadowRoot);
    }
  }

  private login = async (): Promise<void> => {
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

  private handleClick = (): void => {
    this.isOpen = true;
  };

  private handleModalClose = (): void => {
    this.isOpen = false;
  };

  private renderSignin = (): TemplateResult<1> => html`
    <md-filled-text-field
      type="email"
      name="email"
      label="Email"
      pattern="W+"
      style="flex: 1;"
    >
    </md-filled-text-field>
    <md-filled-text-field
      type="password"
      name="password"
      label="Password"
      pattern="W+"
    >
    </md-filled-text-field>
  `;

  private renderSignup = (): TemplateResult<1> => html`
    <md-filled-text-field
      type="text"
      name="first_name"
      label="First name"
      pattern="W+"
      style="flex: 1;"
    >
    </md-filled-text-field>
    <md-filled-text-field
      type="text"
      name="first_name"
      label="Last name"
      pattern="W+"
    >
    </md-filled-text-field>
  `;

  render() {
    return html`
      <md-filled-button class="button-white" @click="${this.handleClick}">
        Login
      </md-filled-button>
      <dropspot-modal
        class="container"
        .open="${this.isOpen}"
        @close="${this.handleModalClose}"
      >
        <section class="container">
          <h3>Sign in</h3>
          <section class="form">${this.renderSignin()}</section>
          <hr />
          <h3>Or, sign up</h3>
          <section class="form">${this.renderSignup()}</section>
          <md-filled-button class="button-primary" @click="${this.login}">
            ${this.isSubmitting
              ? html`<md-circular-progress
                  indeterminate
                ></md-circular-progress>`
              : html`
                  <div class="button-contents">
                    <span>Login</span>
                    <md-icon>login</md-icon>
                  </div>
                `}
          </md-filled-button>
        </section>
      </dropspot-modal>
    `;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "login-button": LoginButtonElement;
  }
}
