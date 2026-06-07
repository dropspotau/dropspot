import {
  createUser,
  login,
  validatePassword,
  type LoginResult,
} from "dropspot-core";
import { html, css, LitElement, type TemplateResult } from "lit";
import { customElement, state } from "lit/decorators.js";
import { createRef, ref, type Ref } from "lit/directives/ref.js";
import type { MdFilledButton } from "@material/web/button/filled-button";

import { setTokens, type LoginEvent } from "../auth";
import { applyGlobalStyles } from "../style";
import { ToastElement } from "./toast";

// Characters, numbers and symbols within a length of 8 and 64
const PASSWORD_PATTERN = "(?=.*[0-9])(?=.*[a-z])(?=.*[A-Z])(?=.*[!@#]).{8,64}";

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
      width: 24rem;
      align-items: center;

      & > .form-row {
        width: 100%;
      }
    }

    .password-errors {
      display: flex;
      flex-flow: column;
      gap: 0.75rem;
      margin: 0;
      padding: 1rem 2rem;
      background-color: var(--dropspot-danger-light);
      border-left: 0.25rem solid var(--dropspot-danger);
      border-radius: 0.5rem;
      width: 100%;
      box-sizing: border-box;
    }

    .password-error-icon {
      color: var(--dropspot-danger);
      font-size: 1.25rem;
      --md-icon-size: 1.25rem;
      flex-shrink: 0;
      margin-top: 0.125rem;
    }

    .password-error {
      color: var(--dropspot-danger);
      font-size: 0.875rem;
      line-height: 1.25;
    }
  `;

  @state()
  private isSubmitting: boolean = false;

  @state()
  private isOpen: boolean = false;

  @state()
  private isSigningUp: boolean = false;

  @state()
  private passwordErrors: string[] = [];

  private submitButtonRef: Ref<MdFilledButton> = createRef();

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

    const isValid =
      typeof email === "string" &&
      (!this.isSigningUp || typeof firstName === "string") &&
      (!this.isSigningUp || typeof lastName === "string") &&
      typeof password === "string";

    if (!isValid) {
      return false;
    }

    // Validate whether the password is valid on creation
    if (this.isSigningUp && typeof password === "string") {
      const { ok: isPasswordValid, errors: passwordErrors } =
        validatePassword(password);

      if (!isPasswordValid) {
        // Show password errors and return
        this.passwordErrors = passwordErrors;
        return false;
      }
    }

    this.isSubmitting = true;

    let result: LoginResult | null = null;
    try {
      if (
        this.isSigningUp &&
        typeof firstName === "string" &&
        typeof lastName === "string"
      ) {
        result = await createUser(email, firstName, lastName, password);
      } else {
        result = await login(email, password);
      }
    } catch (e) {
      ToastElement.create(
        "Sorry, there was an issue logging in. Please try again",
        "danger",
      );
    } finally {
      this.isSubmitting = false;
      this.passwordErrors = [];
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

  /** Submits the form if "Enter" was pressed
   * @param e The key press event
   */
  private handleKeyUp = (e: KeyboardEvent): void => {
    e.preventDefault();

    if (e.key !== "Enter") {
      return;
    }

    const { value: submitButton } = this.submitButtonRef;

    if (submitButton) {
      // NOTE(alec): We can't call requestSubmit on a form ref because submitButton isn't an HTMLButtonElement
      // Manually clicking works fine
      submitButton.click();
    }
  };

  private handleModalClose = (): void => {
    this.isOpen = false;
  };

  private handleToggleSignup = (): void => {
    this.isSigningUp = !this.isSigningUp;
  };

  private renderPasswordErrors = (errors: string[]): TemplateResult<1> => html`
    <ul class="password-errors">
      ${errors.map((error) => html`<li class="password-error">${error}</li>`)}
    </ul>
  `;

  private renderSignin = (): TemplateResult<1> => html`
    <div class="form-row">
      <md-filled-text-field
        type="email"
        name="email"
        label="Email"
        required
        @keyup=${this.handleKeyUp}
      ></md-filled-text-field>
    </div>
    <div class="form-row">
      <md-filled-text-field
        type="password"
        name="password"
        label="Password"
        required
        @keyup=${this.handleKeyUp}
      >
      </md-filled-text-field>
    </div>
  `;

  private renderSignup = (): TemplateResult<1> => html`
    <div class="form-row">
      <md-filled-text-field
        type="email"
        name="email"
        label="Email"
        required
        @keyup=${this.handleKeyUp}
      ></md-filled-text-field>
    </div>
    <div class="form-row">
      <md-filled-text-field
        type="text"
        name="first_name"
        label="First name"
        pattern="[A-Za-z]{1,32}"
        required
        @keyup=${this.handleKeyUp}
      >
      </md-filled-text-field>
    </div>
    <div class="form-row">
      <md-filled-text-field
        type="text"
        name="last_name"
        label="Last name"
        pattern="[A-Za-z]{1,32}"
        required
        @keyup=${this.handleKeyUp}
      >
      </md-filled-text-field>
    </div>
    <div class="form-row">
      <md-filled-text-field
        type="password"
        name="password"
        label="Password"
        minlength="8"
        pattern="${PASSWORD_PATTERN}"
        autocomplete="current-password"
        required
        @keyup=${this.handleKeyUp}
      >
      </md-filled-text-field>
    </div>
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
        <form class="form container text-primary" @submit="${this.handleLogin}">
          ${this.isSigningUp
            ? html`
                <h3>Sign up</h3>
                ${this.renderSignup()}
              `
            : html`
                <h3>Sign in</h3>
                ${this.renderSignin()}
              `}
          <hr />
          <section class="form-row">
            <p class="no-margin text-primary">
              <span>${subtitleText}</span>
              <span class="microlink" @click="${this.handleToggleSignup}"
                >${actionText}</span
              >
            </p>
          </section>
          <md-filled-button
            class="button-primary"
            ${ref(this.submitButtonRef)}
            type="submit"
          >
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
          ${this.passwordErrors.length > 0
            ? this.renderPasswordErrors(this.passwordErrors)
            : ""}
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
