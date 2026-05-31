import { create_user_js, login_js, type LoginResult } from "dropspot-core";
import { html, css, LitElement, type TemplateResult } from "lit";
import { customElement, state } from "lit/decorators.js";
import { setTokens, type LoginEvent } from "./auth";
import { ToastElement } from "./toast";
import { applyGlobalStyles } from "./style";

@customElement("login-controller")
export class LoginControllerElement extends LitElement {
  static styles = css`
    :host {
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
      place-items: center;
      gap: 1rem;
    }

    .form-row {
      display: flex;
      place-items: center;
      gap: 1rem;
    }
  `;

  @state()
  private isSubmitting: boolean = false;

  @state()
  private isOpen: boolean = false;

  @state()
  private isSigningUp: boolean = false;

  connectedCallback(): void {
    super.connectedCallback();

    if (this.shadowRoot) {
      applyGlobalStyles(this.shadowRoot);
    }
  }

  private handleLogin = async (event: SubmitEvent): Promise<false> => {
    event.preventDefault();
    event.stopPropagation();

    const form = event.target;

    if (!(form instanceof HTMLFormElement)) {
      return false;
    }

    const formData = new FormData(form);
    const email = formData.get("email");
    const firstName = formData.get("first_name");
    const lastName = formData.get("last_name");
    const password = formData.get("password");

    console.debug(email, firstName, lastName, password);

    const isValid =
      typeof email === "string" &&
      (!this.isSigningUp || typeof firstName === "string") &&
      (!this.isSigningUp || typeof lastName === "string") &&
      typeof password === "string";

    // Date testing
    if (!isValid) {
      return false;
    }

    this.isSubmitting = true;

    let result: LoginResult | null = null;
    try {
      if (
        this.isSigningUp &&
        typeof firstName === "string" &&
        typeof lastName === "string"
      ) {
        result = await create_user_js(email, firstName, lastName, password);
      } else {
        result = await login_js(email, password);
      }
    } catch (e) {
      ToastElement.create(
        "Sorry, there was an issue logging in. Please try again",
        "danger",
      );
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

    return false;
  };

  private handleClick = (): void => {
    this.isOpen = true;
  };

  private handleModalClose = (): void => {
    this.isOpen = false;
  };

  private handleToggleSignup = (): void => {
    this.isSigningUp = !this.isSigningUp;
  };

  private renderSignin = (): TemplateResult<1> => html`
    <md-filled-text-field type="email" name="email" label="Email">
    </md-filled-text-field>
    <md-filled-text-field
      type="password"
      name="password"
      label="Password"
      pattern="[A-Za-z0-9]{8,}"
    >
    </md-filled-text-field>
  `;

  private renderSignup = (): TemplateResult<1> => html`
    <md-filled-text-field type="email" name="email" label="Email" required>
    </md-filled-text-field>
    <md-filled-text-field
      type="text"
      name="first_name"
      label="First name"
      pattern="[A-Za-z]{1,32}"
      required
    >
    </md-filled-text-field>
    <md-filled-text-field
      type="text"
      name="last_name"
      label="Last name"
      pattern="[A-Za-z]{1,32}"
      required
    >
    </md-filled-text-field>
    <md-filled-text-field
      type="password"
      name="password"
      label="Password"
      pattern="[A-Za-z0-9]{8,}"
    >
    </md-filled-text-field>
  `;

  render() {
    const subtitleText = this.isSigningUp
      ? "Already have an account?"
      : "No account?";
    const actionText = this.isSigningUp ? "Sign in" : "Sign up";

    return html`
      <md-filled-button class="button-white" @click="${this.handleClick}">
        Login
      </md-filled-button>
      <dropspot-modal .open="${this.isOpen}" @close="${this.handleModalClose}">
        <form class="container" @submit="${this.handleLogin}">
          <section class="form">
            ${this.isSigningUp
              ? html`
                  <h3>Sign up</h3>
                  ${this.renderSignup()}
                `
              : html`
                  <h3>Sign in</h3>
                  ${this.renderSignin()}
                `}
          </section>
          <hr />
          <section class="form-row">
            <p class="no-margin">
              <span>${subtitleText}</span>
              <span class="microlink" @click="${this.handleToggleSignup}"
                >${actionText}</span
              >
            </p>
          </section>
          <md-filled-button class="button-primary" type="submit">
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
        </form>
      </dropspot-modal>
    `;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "login-controller": LoginControllerElement;
  }
}
